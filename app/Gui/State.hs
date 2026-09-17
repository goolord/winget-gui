-- | Application state shared between the UI thread and WinGet worker threads.
--
-- Workers change the state with 'modifyState', which writes atomically and then
-- wakes the SDL event loop so the next frame shows the change. The view only
-- reads the state and issues actions; it never blocks on WinGet.
module Gui.State
  ( Env (..)
  , AppState (..)
  , JobView (..)
  , Filter (..)
  , SortKey (..)
  , PendingBatch (..)
  , ViewKey (..)
  , newEnv
  , newDryRunEnv
  , readState
  , modifyState
  , startupLoad
  , refresh
  , startBatch
  , reportJob
  , endDryRunBatch
  , stopBatches
  , cancelJob
  , retryFailed
  , clearFinished
  , visiblePackages
  , filterLabel
  , formatSize
  , ellipsize
  , tshow
  )
where

import Control.Concurrent (forkIO)
import Control.Monad (forM_, join, unless, void, when)
import Data.IORef (IORef, atomicModifyIORef', newIORef, readIORef)
import Data.List (sortOn)
import Data.Maybe (isJust, mapMaybe)
import Data.Map.Strict qualified as Map
import Data.Set (Set)
import Data.Set qualified as Set
import Data.Text (Text)
import Data.Text qualified as T
import Data.Vector (Vector)
import Data.Vector qualified as V
import Data.Word (Word64)
import GHC.Clock (getMonotonicTime)
import Numeric (showFFloat)
import WinGet.Bridge
import WinGet.Queue

data Filter
  = FilterAll
  | FilterUpdates
  | FilterWin32
  | FilterMsix
  | FilterSilent
  deriving (Eq, Show, Enum, Bounded)

filterLabel :: Filter -> Text
filterLabel = \case
  FilterAll -> "All apps"
  FilterUpdates -> "Updates available"
  FilterWin32 -> "Desktop installers"
  FilterMsix -> "Store / MSIX"
  FilterSilent -> "Silent uninstall"

data SortKey = SortName | SortPublisher | SortSize | SortDate
  deriving (Eq, Show, Enum, Bounded)

data JobView = JobView
  { jvJob :: !Job
  , jvStatus :: !JobStatus
  }
  deriving (Eq, Show)

-- | An operation waiting for the user to confirm it.
data PendingBatch = PendingBatch
  { pbKind :: !OpKind
  , pbPackages :: ![Package]
  }
  deriving (Eq, Show)

data AppState = AppState
  { stConnected :: !Bool
  , stPackages :: !(Vector Package)
  , stRevision :: !Int
  -- ^ Bumped whenever 'stPackages' changes, to key cached views.
  , stSnapshot :: !(Maybe SnapshotId)
  -- ^ The bridge snapshot 'stPackages' indexes into.
  , stRetired :: ![SnapshotId]
  -- ^ Replaced snapshots that running batches may still use.
  , stLoading :: !(Maybe Text)
  , stError :: !(Maybe Text)
  , stLoadSeconds :: !Double
  , stHasUpdateInfo :: !Bool
  , stSelected :: !(Set Text)
  -- ^ Selected package ids; ids survive refreshes, indices do not.
  , stJobs :: !(Vector JobView)
  , stActiveBatches :: !Int
  , stSkipped :: !(Set Word64)
  -- ^ Tokens of jobs the user skipped, cancelled, or stopped. A job is
  -- checked against this set as it is about to start.
  }

data Env = Env
  { envState :: !(IORef AppState)
  , envWake :: !(IORef (IO ()))
  -- ^ Wakes the event loop; installed by the view on its first frame.
  , envDryRun :: !Bool
  -- ^ Queue operations without running them: the demo recording advances
  -- the jobs itself, so nothing is ever uninstalled or upgraded.
  }

newEnv :: IO Env
newEnv = newEnvWith False

-- | An environment whose operations are only queued, never run. See 'envDryRun'.
newDryRunEnv :: IO Env
newDryRunEnv = newEnvWith True

newEnvWith :: Bool -> IO Env
newEnvWith dryRun =
  Env
    <$> newIORef
      AppState
        { stConnected = False
        , stPackages = V.empty
        , stRevision = 0
        , stSnapshot = Nothing
        , stRetired = []
        , stLoading = Just "Connecting to WinGet…"
        , stError = Nothing
        , stLoadSeconds = 0
        , stHasUpdateInfo = False
        , stSelected = Set.empty
        , stJobs = V.empty
        , stActiveBatches = 0
        , stSkipped = Set.empty
        }
    <*> newIORef (pure ())
    <*> pure dryRun

readState :: Env -> IO AppState
readState = readIORef . envState

notify :: Env -> IO ()
notify env = join (readIORef (envWake env))

modifyState :: Env -> (AppState -> AppState) -> IO ()
modifyState env f = atomicModifyIORef' (envState env) (\s -> (f s, ())) >> notify env

-- | Connect and show installed apps as fast as the local catalog allows; then,
-- if asked, correlate with the configured sources to fill in available upgrades.
startupLoad :: Env -> Bool -> IO ()
startupLoad env checkUpdates = void . forkIO $ connectAndLoad env checkUpdates

connectAndLoad :: Env -> Bool -> IO ()
connectAndLoad env checkUpdates = do
  modifyState env (\s -> s {stLoading = Just "Connecting to WinGet…", stError = Nothing})
  connected <- initBridge
  case connected of
    Left err ->
      modifyState env (\s -> s {stLoading = Nothing, stError = Just ("Could not connect to WinGet: " <> err)})
    Right _ -> do
      modifyState env (\s -> s {stConnected = True})
      loadOnce env False
      when checkUpdates (loadOnce env True)

-- | Reload in the background unless a load is already running. If WinGet
-- could not be reached before, connect again first.
refresh :: Env -> Bool -> IO ()
refresh env checkUpdates = do
  claimed <- atomicModifyIORef' (envState env) $ \s ->
    if isJust (stLoading s) then (s, Nothing) else (s {stLoading = Just "Refreshing…"}, Just (stConnected s))
  forM_ claimed $ \connected ->
    void . forkIO $ if connected then loadOnce env checkUpdates else connectAndLoad env checkUpdates

loadOnce :: Env -> Bool -> IO ()
loadOnce env checkUpdates = do
  modifyState env $ \s ->
    s
      { stLoading = Just (if checkUpdates then "Checking sources for updates…" else "Reading installed apps…")
      , stError = Nothing
      }
  t0 <- getMonotonicTime
  result <- listInstalled checkUpdates
  t1 <- getMonotonicTime
  case result of
    Left err -> modifyState env (\s -> s {stLoading = Nothing, stError = Just err})
    Right (snap, pkgs) -> do
      -- Parse every record here rather than in the first frames that touch them.
      V.foldM' (\() p -> p `seq` pure ()) () pkgs
      release <- atomicModifyIORef' (envState env) $ \s ->
        let ids = Set.fromList (V.toList (V.map pkgId pkgs))
            old = maybe [] pure (stSnapshot s)
            s' =
              s
                { stPackages = pkgs
                , stRevision = stRevision s + 1
                , stSnapshot = Just snap
                , stLoading = Nothing
                , stLoadSeconds = t1 - t0
                , stHasUpdateInfo = checkUpdates
                , stSelected = Set.intersection (stSelected s) ids
                }
         in if stActiveBatches s > 0 then (s' {stRetired = old <> stRetired s}, []) else (s', old)
      mapM_ releaseSnapshot release
      notify env

-- | Queue an uninstall or upgrade of the given packages.
--
-- Packages are matched by id against the snapshot current at this moment, so a
-- refresh that landed after the user clicked can never shift an index onto a
-- different app. Packages no longer installed are dropped.
startBatch :: Env -> OpKind -> OpFlags -> Int -> [Package] -> IO ()
startBatch env kind flags workers requested = do
  claimed <- atomicModifyIORef' (envState env) $ \s ->
    case stSnapshot s of
      Nothing -> (s, Nothing)
      Just snap ->
        let current = Map.fromList [(pkgId p, p) | p <- V.toList (stPackages s)]
            resolved = mapMaybe (\p -> Map.lookup (pkgId p) current) requested
         in if null resolved
              then (s, Nothing)
              else (s {stActiveBatches = stActiveBatches s + 1}, Just (snap, resolved))
  forM_ claimed $ \(snap, pkgs) -> do
    jobs <- mapM (newJob kind flags) pkgs
    modifyState env $ \s ->
      s
        { stJobs = stJobs s <> V.fromList [JobView j JobPending | j <- jobs]
        , stSelected = foldr (Set.delete . pkgId) (stSelected s) pkgs
        }
    -- A dry run stops here: the jobs stay queued for whoever drives them.
    unless (envDryRun env) . void . forkIO $ do
      let skip job = Set.member (jobToken job) . stSkipped <$> readState env
      runJobs workers snap skip (report env) jobs
      (release, recheck) <- atomicModifyIORef' (envState env) $ \s ->
        let n = stActiveBatches s - 1
         in if n <= 0
              then (s {stActiveBatches = 0, stRetired = []}, (stRetired s, stHasUpdateInfo s))
              else (s {stActiveBatches = n}, ([], stHasUpdateInfo s))
      mapM_ releaseSnapshot release
      notify env
      -- Pick up anything the installers changed beyond what the results say.
      refresh env recheck

-- | Record a job's new status, as the queue reports it. A dry run's driver
-- calls this too.
reportJob :: Env -> Job -> JobStatus -> IO ()
reportJob = report

-- | Mark a dry-run batch finished once its driver has settled every job.
endDryRunBatch :: Env -> IO ()
endDryRunBatch env = modifyState env (\s -> s {stActiveBatches = max 0 (stActiveBatches s - 1)})

report :: Env -> Job -> JobStatus -> IO ()
report env job status = modifyState env $ \s ->
  let token = jobToken job
      p = jobPackage job
      s1 = s {stJobs = V.map (\jv -> if jobToken (jvJob jv) == token then jv {jvStatus = status} else jv) (stJobs s)}
   in case (status, jobKind job) of
        (JobSucceeded _, OpUninstall) ->
          s1 {stPackages = V.filter ((/= pkgId p) . pkgId) (stPackages s1), stRevision = stRevision s1 + 1}
        (JobSucceeded _, OpUpgrade) ->
          let upgraded q
                | pkgId q == pkgId p && not (T.null (pkgAvailable q)) = q {pkgVersion = pkgAvailable q, pkgAvailable = ""}
                | otherwise = q
           in s1 {stPackages = V.map upgraded (stPackages s1), stRevision = stRevision s1 + 1}
        _ -> s1

-- | Stop every job currently queued or running. Each job is marked skipped
-- individually, so batches queued afterwards run normally while these stay
-- stopped. Cancellation is requested for all of them: the bridge cancels a
-- running operation now and one that is just starting as soon as it registers.
stopBatches :: Env -> IO ()
stopBatches env = do
  open <- atomicModifyIORef' (envState env) $ \s ->
    let jobs = [jvJob jv | jv <- V.toList (stJobs s), not (jobIsFinished (jvStatus jv))]
     in (s {stSkipped = foldr (Set.insert . jobToken) (stSkipped s) jobs}, jobs)
  notify env
  unless (envDryRun env) . void . forkIO $ forM_ open (void . cancelOperation . jobToken)

-- | Cancel one job: skip it if waiting, or ask WinGet to cancel it if running.
cancelJob :: Env -> Job -> IO ()
cancelJob env job = do
  modifyState env (\s -> s {stSkipped = Set.insert (jobToken job) (stSkipped s)})
  unless (envDryRun env) . void . forkIO . void $ cancelOperation (jobToken job)

-- | Re-queue failed jobs with their original options. Jobs the user skipped or
-- cancelled stay as they are.
retryFailed :: Env -> IO ()
retryFailed env = do
  failed <- atomicModifyIORef' (envState env) $ \s ->
    let isFailed jv = case jvStatus jv of
          JobFailed _ -> True
          _ -> False
        (retry, keep) = V.partition isFailed (stJobs s)
     in (s {stJobs = keep}, map jvJob (V.toList retry))
  let byOp = Map.fromListWith (flip (<>)) [((jobKind j == OpUpgrade, show (jobFlags j)), [j]) | j <- failed]
  forM_ (Map.elems byOp) $ \case
    [] -> pure ()
    jobs@(j : _) -> startBatch env (jobKind j) (jobFlags j) 1 (map jobPackage jobs)

clearFinished :: Env -> IO ()
clearFinished env = modifyState env $ \s ->
  s {stJobs = V.filter (not . jobIsFinished . jvStatus) (stJobs s)}

data ViewKey = ViewKey !Int !Text !Filter !(Maybe Text) !SortKey !Bool
  deriving (Eq)

-- | Packages matching the search, filter, and disk, sorted. Every search word
-- must appear in the name, id, or publisher. The disk is a drive as the
-- bridge reports it (@C:@, @Network@); @Just ""@ selects apps on an unknown disk.
visiblePackages :: AppState -> Text -> Filter -> Maybe Text -> SortKey -> Bool -> Vector Package
visiblePackages st query filt disk key desc =
  V.fromList (direction (order (V.toList (V.filter keep (stPackages st)))))
  where
    terms = T.words (T.toCaseFold query)
    keep p = all (`T.isInfixOf` pkgHaystack p) terms && passes p && maybe True (== pkgDrive p) disk
    passes p = case filt of
      FilterAll -> True
      FilterUpdates -> not (T.null (pkgAvailable p))
      FilterWin32 -> pkgKind p == KindWin32
      FilterMsix -> pkgKind p == KindMsix
      FilterSilent -> pkgUninstall p == HasSilentUninstall
    name = T.toCaseFold . pkgName
    order = case key of
      SortName -> sortOn name
      SortPublisher -> sortOn (\p -> (T.null (pkgPublisher p), T.toCaseFold (pkgPublisher p), name p))
      SortSize -> sortOn (\p -> (pkgSize p, name p))
      SortDate -> sortOn (\p -> (pkgDate p, name p))
    direction = if desc then reverse else id

formatSize :: Word64 -> Text
formatSize bytes
  | bytes == 0 = ""
  | b < mb = T.pack (show (max 1 (round (b / kb) :: Int))) <> " KB"
  | b < gb = T.pack (showFFloat (Just 1) (b / mb) "") <> " MB"
  | otherwise = T.pack (showFFloat (Just 2) (b / gb) "") <> " GB"
  where
    b = fromIntegral bytes :: Double
    kb = 1024
    mb = 1024 * kb
    gb = 1024 * mb

ellipsize :: Int -> Text -> Text
ellipsize n t
  | T.length t > n = T.stripEnd (T.take (n - 1) t) <> "…"
  | otherwise = t

tshow :: Show a => a -> Text
tshow = T.pack . show
