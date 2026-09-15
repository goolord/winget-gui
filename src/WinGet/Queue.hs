-- | Running uninstalls and upgrades in bulk.
--
-- Jobs run on a small pool of worker threads. One worker is the safe default:
-- most classic installers take the machine-wide Windows Installer lock, so
-- running them side by side only makes them wait for each other.
module WinGet.Queue
  ( Job (..)
  , JobStatus (..)
  , newJob
  , jobIsActive
  , jobIsFinished
  , runJobs
  )
where

import Control.Concurrent (forkFinally, newEmptyMVar, putMVar, takeMVar)
import Control.Exception (SomeException, displayException, try)
import Control.Monad (forM, forM_)
import Data.IORef (atomicModifyIORef', newIORef)
import Data.Text (Text)
import Data.Text qualified as T
import Data.Unique (hashUnique, newUnique)
import Data.Word (Word64)
import WinGet.Bridge

data JobStatus
  = JobPending
  | JobRunning !OpState !Double
  | JobSucceeded !Text
  | JobFailed !Text
  | JobCancelled
  deriving (Eq, Show)

data Job = Job
  { jobToken :: !Word64
  -- ^ Unique per job; names the operation for 'cancelOperation'.
  , jobKind :: !OpKind
  , jobPackage :: !Package
  , jobFlags :: !OpFlags
  }
  deriving (Eq, Show)

newJob :: OpKind -> OpFlags -> Package -> IO Job
newJob kind flags pkg = do
  u <- newUnique
  pure Job {jobToken = fromIntegral (hashUnique u), jobKind = kind, jobPackage = pkg, jobFlags = flags}

jobIsActive :: JobStatus -> Bool
jobIsActive = \case
  JobRunning {} -> True
  _ -> False

jobIsFinished :: JobStatus -> Bool
jobIsFinished = \case
  JobSucceeded {} -> True
  JobFailed {} -> True
  JobCancelled -> True
  _ -> False

-- | E_ABORT and HRESULT_FROM_WIN32(ERROR_CANCELLED).
isCancelled :: OpResult -> Bool
isCancelled r = resHresult r `elem` [fromIntegral (0x80004004 :: Word64), fromIntegral (0x800704C7 :: Word64)]

-- | Run every job on @workers@ threads and wait for all of them. @shouldSkip@
-- is checked as each job is about to start; skipped jobs are reported
-- cancelled. @report@ may be called concurrently from worker and WinGet
-- callback threads.
runJobs :: Int -> SnapshotId -> (Job -> IO Bool) -> (Job -> JobStatus -> IO ()) -> [Job] -> IO ()
runJobs workers snap shouldSkip report jobs = do
  queue <- newIORef jobs
  let next = atomicModifyIORef' queue $ \case
        [] -> ([], Nothing)
        (j : js) -> (js, Just j)
      worker = next >>= maybe (pure ()) (\j -> runOne j >> worker)
  dones <- forM [1 .. max 1 workers] $ \_ -> do
    done <- newEmptyMVar
    _ <- forkFinally worker (\_ -> putMVar done ())
    pure done
  forM_ dones takeMVar
  where
    runOne job = do
      skip <- shouldSkip job
      if skip
        then report job JobCancelled
        else do
          report job (JobRunning StateQueued 0)
          outcome <-
            try $
              runOperation (jobKind job) snap (pkgIndex (jobPackage job)) (jobFlags job) (jobToken job) $
                \st frac -> report job (JobRunning st frac)
          case outcome of
            Left (e :: SomeException) -> report job (JobFailed (T.pack (displayException e)))
            Right res
              | opSucceeded res -> report job . JobSucceeded =<< describeResult (jobKind job) res
              | isCancelled res -> report job JobCancelled
              | otherwise -> report job . JobFailed =<< describeResult (jobKind job) res
