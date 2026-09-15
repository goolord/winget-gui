-- | Headless automation for the same bridge the GUI uses.
module Main (main) where

import Control.Concurrent.MVar (newMVar, withMVar)
import Control.Monad (unless, when)
import Data.IORef (modifyIORef', newIORef, readIORef, writeIORef)
import Data.Text (Text)
import Data.Text qualified as T
import Data.Text.IO qualified as T
import Data.Vector qualified as V
import System.Environment (getArgs)
import System.Exit (ExitCode (..), exitWith)
import System.IO (BufferMode (..), hFlush, hSetBuffering, hSetEncoding, stderr, stdout, utf8)
import WinGet.Bridge
import WinGet.Queue
import WinGet.Select

data Command = CmdList | CmdUninstall | CmdUpgrade | CmdHelp
  deriving (Eq, Show)

data Options = Options
  { optCommand :: !Command
  , optUpdates :: !Bool
  , optSelectors :: ![Selector]
  , optFiles :: ![FilePath]
  , optAll :: !Bool
  , optYes :: !Bool
  , optDryRun :: !Bool
  , optFlags :: !OpFlags
  , optParallel :: !Int
  }

defaultOptions :: Options
defaultOptions =
  Options
    { optCommand = CmdHelp
    , optUpdates = False
    , optSelectors = []
    , optFiles = []
    , optAll = False
    , optYes = False
    , optDryRun = False
    , optFlags = defaultOpFlags
    , optParallel = 1
    }

usage :: Text
usage =
  T.unlines
    [ "winget-gui-cli: list, uninstall, and upgrade installed programs in bulk."
    , ""
    , "  winget-gui-cli list [--updates] [SELECTOR...]"
    , "  winget-gui-cli uninstall [OPTIONS] SELECTOR... | --from FILE"
    , "  winget-gui-cli upgrade [OPTIONS] (SELECTOR... | --from FILE | --all)"
    , ""
    , "Selectors (case-insensitive):"
    , "  --id ID  --name NAME  --publisher NAME  --disk D:  --match TEXT  or a bare id/name"
    , "  --from FILE   one selector per line (id:, name:, publisher:, disk:, match:, or bare)"
    , "  A package matches when it matches any selector."
    , ""
    , "Options:"
    , "  --updates       correlate with sources to show available upgrades"
    , "  --all           (upgrade) every package with an available upgrade"
    , "  --interactive   show installer UI instead of running silently"
    , "  --force         skip WinGet safety checks"
    , "  --accept-agreements  (upgrade) accept package license agreements"
    , "  --parallel N    run N operations at once (default 1)"
    , "  --dry-run       print what would run"
    , "  --yes           do not ask for confirmation"
    ]

parseArgs :: [String] -> Either Text Options
parseArgs = go defaultOptions
  where
    go o = \case
      [] -> Right o {optSelectors = reverse (optSelectors o)}
      ("list" : rest) -> go o {optCommand = CmdList} rest
      ("uninstall" : rest) -> go o {optCommand = CmdUninstall} rest
      ("upgrade" : rest) -> go o {optCommand = CmdUpgrade, optUpdates = True} rest
      ("--help" : rest) -> go o {optCommand = CmdHelp} rest
      ("--updates" : rest) -> go o {optUpdates = True} rest
      ("--all" : rest) -> go o {optAll = True} rest
      ("--yes" : rest) -> go o {optYes = True} rest
      ("-y" : rest) -> go o {optYes = True} rest
      ("--dry-run" : rest) -> go o {optDryRun = True} rest
      ("--interactive" : rest) -> go o {optFlags = (optFlags o) {opSilent = False, opInteractive = True}} rest
      ("--force" : rest) -> go o {optFlags = (optFlags o) {opForce = True}} rest
      ("--accept-agreements" : rest) -> go o {optFlags = (optFlags o) {opAcceptAgreements = True}} rest
      ("--parallel" : n : rest) | [(k, "")] <- reads n, k > 0 -> go o {optParallel = k} rest
      ("--from" : f : rest) -> go o {optFiles = f : optFiles o} rest
      ("--id" : v : rest) -> sel o (ById (T.pack v)) rest
      ("--name" : v : rest) -> sel o (ByName (T.pack v)) rest
      ("--publisher" : v : rest) -> sel o (ByPublisher (T.pack v)) rest
      ("--match" : v : rest) -> sel o (Contains (T.pack v)) rest
      ("--disk" : v : rest) -> sel o (ByDisk (normalizeDrive (T.pack v))) rest
      (a : _) | "--" `T.isPrefixOf` T.pack a -> Left ("unknown or incomplete option: " <> T.pack a)
      (a : rest) -> sel o (IdOrName (T.pack a)) rest
    sel o s = go o {optSelectors = s : optSelectors o}

showSize :: Package -> Text
showSize p
  | pkgSize p == 0 = ""
  | otherwise = T.pack (show (pkgSize p `div` (1024 * 1024))) <> " MB"

printPackages :: V.Vector Package -> IO ()
printPackages pkgs = do
  T.putStrLn (T.intercalate "\t" ["Name", "Id", "Version", "Available", "Publisher", "Size", "Installed", "Disk"])
  V.forM_ pkgs $ \p ->
    T.putStrLn (T.intercalate "\t" [pkgName p, pkgId p, pkgVersion p, pkgAvailable p, pkgPublisher p, showSize p, pkgDate p, pkgDrive p])

confirm :: Text -> IO Bool
confirm prompt = do
  T.putStr (prompt <> " [y/N] ")
  hFlush stdout
  answer <- T.strip . T.toLower <$> T.getLine
  pure (answer `elem` ["y", "yes"])

main :: IO ()
main = do
  -- App names are arbitrary Unicode; never let the console code page decide.
  mapM_ (`hSetEncoding` utf8) [stdout, stderr]
  hSetBuffering stdout LineBuffering
  args <- getArgs
  opts <- case parseArgs args of
    Left err -> T.hPutStrLn stderr err >> T.putStr usage >> exitWith (ExitFailure 2)
    Right o -> pure o
  when (optCommand opts == CmdHelp) $ T.putStr usage >> exitWith ExitSuccess
  fileSelectors <- concat <$> mapM (fmap parseSelectors . T.readFile) (optFiles opts)
  let selectors = optSelectors opts <> fileSelectors

  _via <- either (\e -> die' ("Could not connect to WinGet: " <> e)) pure =<< initBridge
  listing <- listInstalled (optUpdates opts)
  (snap, pkgs) <- either (\e -> die' ("Could not list installed packages: " <> e)) pure listing

  case optCommand opts of
    CmdList -> printPackages (if null selectors then pkgs else selectPackages selectors pkgs)
    cmd -> do
      let kind = if cmd == CmdUpgrade then OpUpgrade else OpUninstall
          chosen
            | kind == OpUpgrade && optAll opts = V.filter (not . T.null . pkgAvailable) pkgs
            | kind == OpUpgrade = V.filter (not . T.null . pkgAvailable) (selectPackages selectors pkgs)
            | otherwise = selectPackages selectors pkgs
      when (null selectors && not (optAll opts)) $ die' "No packages selected. Pass selectors, --from FILE, or --all."
      when (V.null chosen) $ die' "No installed packages matched."
      let verb = if kind == OpUpgrade then "Upgrade" else "Uninstall"
      T.putStrLn (verb <> " " <> T.pack (show (V.length chosen)) <> " package(s):")
      V.forM_ chosen $ \p -> T.putStrLn ("  " <> pkgName p <> "  [" <> pkgId p <> "]  " <> pkgVersion p)
      when (optDryRun opts) $ exitWith ExitSuccess
      ok <- if optYes opts then pure True else confirm "Proceed?"
      unless ok $ exitWith (ExitFailure 1)

      jobs <- mapM (newJob kind (optFlags opts)) (V.toList chosen)
      failures <- newIORef (0 :: Int)
      lastPct <- newIORef ([] :: [(Integer, Int)])
      out <- newMVar ()
      let say t = withMVar out (\_ -> T.putStrLn t)
          report job st = case st of
            JobRunning StateRunning frac -> do
              let pct = floor (frac * 100) `div` 25 * 25 :: Int
                  key = toInteger (jobToken job)
              seen <- lookup key <$> readIORef lastPct
              when (seen /= Just pct) $ do
                modifyIORef' lastPct ((key, pct) :)
                say ("  " <> pkgName (jobPackage job) <> ": " <> T.pack (show pct) <> "%")
            JobRunning StateQueued _ -> say ("> " <> pkgName (jobPackage job))
            JobSucceeded msg -> say ("OK   " <> pkgName (jobPackage job) <> ": " <> msg)
            JobFailed msg -> modifyIORef' failures (+ 1) >> say ("FAIL " <> pkgName (jobPackage job) <> ": " <> msg)
            JobCancelled -> modifyIORef' failures (+ 1) >> say ("SKIP " <> pkgName (jobPackage job))
            _ -> pure ()
      stop <- newIORef False
      runJobs (optParallel opts) snap (const (readIORef stop)) report jobs
      writeIORef stop True
      n <- readIORef failures
      releaseSnapshot snap
      say (T.pack (show (length jobs - n)) <> " succeeded, " <> T.pack (show n) <> " failed")
      exitWith (if n == 0 then ExitSuccess else ExitFailure (min 255 n))
  where
    die' :: Text -> IO a
    die' msg = T.hPutStrLn stderr msg >> exitWith (ExitFailure 1)
