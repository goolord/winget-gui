-- | Headless checks of the real view against the real installed-app list.
--
-- 'screenshot' renders the main window to a BMP. 'selfTest' drives the view
-- with scripted input through search, the details dialog, the confirmation
-- dialog, rule selection, and the disk filter, saving a screenshot of each.
--
-- Nothing here starts an operation: the confirmation dialog is always
-- cancelled, Delete is never pressed, and the queue screenshot uses jobs
-- written straight into the state, which are never run.
module Gui.SelfTest
  ( screenshot
  , selfTest
  )
where

import Control.Concurrent (threadDelay)
import Control.Exception (SomeException, catch, displayException, fromException)
import Control.Monad (replicateM_, unless, void, when)
import Data.List (sortOn)
import Data.Maybe (isNothing, listToMaybe)
import Data.Set qualified as Set
import Data.Text (Text)
import Data.Text qualified as T
import Data.Vector qualified as V
import GHC.Clock (getMonotonicTime)
import Gui.State
import Gui.View (ViewCache, appView)
import NanoUI (Input (..), Key (..), Rect (..), V2 (..), emptyInput, inputKeysFromList)
import NanoUI.Backend.Sdl (SdlEnv, SdlOptions (..), newSdlContext, saveScreenshot, sdlDrawFrame, withSdl)
import NanoUI.Context (Context)
import NanoUI.Testing (collectOverlayTextSpans, collectTextSpans, withTheme)
import NanoUI.Testing.Harness (clickPos, findExact, hasText, requireSpan)
import System.Exit (ExitCode (..), exitSuccess, exitWith)
import System.IO (hFlush, hPutStrLn, stderr)
import WinGet.Bridge
import WinGet.Queue

-- | Wait (up to two minutes) for the first listing or a connection failure.
waitForList :: Env -> IO AppState
waitForList env = getMonotonicTime >>= go
  where
    go start = do
      st <- readState env
      now <- getMonotonicTime
      let failed = not (stConnected st) && isNothing (stLoading st)
      if stRevision st > 0 || failed || now - start > 120
        then pure st
        else threadDelay 50000 >> go start

data Headless = Headless
  { hCtx :: Context
  , hEnv :: SdlEnv
  , hFrame :: Input -> IO ()
  , hBase :: Input
  , hSave :: FilePath -> IO ()
  }

headless :: SdlOptions -> Env -> ViewCache -> (Headless -> IO a) -> IO a
headless opts env cache k = do
  -- Apply the app theme as runSdlApp does, so screenshots match the real window.
  ctx0 <- newSdlContext >>= \c -> maybe (pure c) (withTheme c) (sdlAppTheme opts)
  withSdl opts {sdlWindowHidden = True, sdlWindowResizable = False} ctx0 $ \ctx sdlEnv -> do
    let base = emptyInput {inputWindowSize = sdlWindowSize opts, inputMousePos = V2 (-1) (-1)}
        frame forceFull inp = void (sdlDrawFrame ctx (appView env cache) sdlEnv inp forceFull)
        save path = do
          replicateM_ 2 (frame False base)
          frame True base
          saved <- saveScreenshot sdlEnv path
          unless saved $ fail ("could not save " <> path)
    k Headless {hCtx = ctx, hEnv = sdlEnv, hFrame = frame False, hBase = base, hSave = save}

selectFirst :: Env -> Int -> IO ()
selectFirst env n =
  modifyState env $ \s ->
    let firstByName = V.toList (V.take n (visiblePackages s "" FilterAll Nothing SortName False))
     in s {stSelected = Set.fromList (map pkgId firstByName)}

-- | Render the main window once apps are listed, optionally with the first
-- @selectCount@ apps selected, save it to @path@, and exit.
screenshot :: SdlOptions -> Env -> ViewCache -> FilePath -> Int -> IO ()
screenshot opts env cache path selectCount = do
  _ <- waitForList env
  when (selectCount > 0) $ selectFirst env selectCount
  headless opts env cache $ \h -> do
    replicateM_ 2 (hFrame h (hBase h))
    hSave h path
    exitSuccess

selfTest :: SdlOptions -> Env -> ViewCache -> FilePath -> IO ()
selfTest opts env cache dir =
  selfTestSteps opts env cache dir `catch` \(e :: SomeException) ->
    case fromException e of
      Just (code :: ExitCode) -> exitWith code
      Nothing -> do
        hPutStrLn stderr ("selftest: FAILED: " <> displayException e)
        hFlush stderr
        exitWith (ExitFailure 2)

selfTestSteps :: SdlOptions -> Env -> ViewCache -> FilePath -> IO ()
selfTestSteps opts env cache dir = do
  st0 <- waitForList env
  when (stRevision st0 == 0) $
    fail ("selftest: nothing listed: " <> maybe "timed out" T.unpack (stError st0))
  headless opts env cache $ \h -> do
    let base = hBase h
        frame = hFrame h
        settle = replicateM_ 3 (frame base)
        step name = putStrLn ("selftest: " <> name)
        shot name = hSave h (dir <> "\\" <> name <> ".bmp")
        press key = frame base {inputKeys = inputKeysFromList [key]} >> settle
        -- Dialogs and drop-downs draw into the overlay layer, so look at both layers.
        spans = (<>) <$> collectTextSpans (hCtx h) <*> collectOverlayTextSpans (hCtx h) base
        center (Rect x y w hgt) = V2 (x + w / 2) (y + hgt / 2)
        dumpVisible = do
          visible <- spans
          hPutStrLn stderr ("selftest: visible text: " <> show [txt | (_, txt, _, _, _) <- take 80 visible])
          hSave h (dir <> "\\failure.bmp")
        expect :: Text -> IO ()
        expect needle = do
          present <- hasText needle <$> spans
          unless present $ dumpVisible >> fail ("selftest: expected to see " <> show needle)
        expectGone :: Text -> IO ()
        expectGone needle = do
          present <- hasText needle <$> spans
          when present $ dumpVisible >> fail ("selftest: expected " <> show needle <> " to be gone")
        clickAt what pos = do
          target <- requireSpan ("selftest: no " <> what <> " on screen") pos
          clickPos frame base target
          settle
        click label = clickAt (show label) . findExact label =<< spans
        -- The highest span with exactly this text (the search field sits above
        -- any row showing the same text).
        clickTopmost label = do
          found <- spans
          clickAt (show label) (listToMaybe [center r | (r, txt, _, _, _) <- sortOn (\(r, _, _, _, _) -> rectY r) found, txt == label])
        clickPrefixed prefix = do
          found <- spans
          clickAt (show prefix) (listToMaybe [center r | (r, txt, _, _, _) <- found, prefix `T.isPrefixOf` txt])

    settle
    expect "Installed apps"
    shot "01-list"

    step "search"
    click "Search name, id, or publisher"
    frame base {inputChars = "2C-Audio"}
    settle
    expect " of "
    shot "02-search"

    step "details dialog"
    st <- readState env
    firstMatch <- case V.toList (visiblePackages st "2C-Audio" FilterAll Nothing SortName False) of
      p : _ -> pure p
      [] -> fail "selftest: search matched nothing"
    click (ellipsize 64 (pkgName firstMatch))
    expect "Product codes"
    shot "03-details"
    press KeyEscape
    expectGone "Product codes"

    step "confirmation dialog (cancelled)"
    click "Select shown"
    expect "selected"
    click "Uninstall selected"
    expect "Installer UI"
    shot "04-confirm"
    click "Cancel"
    expectGone "Installer UI"
    stAfter <- readState env
    unless (V.null (stJobs stAfter)) $ fail "selftest: cancelling the dialog queued jobs"
    click "Clear"

    step "rule selection dialog"
    click "Select by rule…"
    expect "installed apps match"
    shot "05-rules"
    press KeyEscape
    expectGone "installed apps match"

    step "disk filter"
    -- Empty the search box: caret to the end, then delete "2C-Audio".
    clickTopmost "2C-Audio"
    press KeyEnd
    replicateM_ (T.length "2C-Audio") (press KeyBackspace)
    expectGone " of "
    -- Escape that closes a drop-down must not also clear the selection.
    click "Select shown"
    expect "selected"
    click "All disks"
    expect "Unknown disk"
    press KeyEscape
    expectGone "Unknown disk"
    expect "selected"
    click "Clear"
    expectGone "selected"
    click "All disks"
    expect "Unknown disk"
    shot "07-disks"
    clickPrefixed "C:  ·"
    expect " of "
    shot "08-disk-c"

    step "queue panel (display only)"
    stQueue <- readState env
    let sample = take 4 (V.toList (visiblePackages stQueue "" FilterAll (Just "C:") SortName False))
        statuses =
          [ JobSucceeded "Done"
          , JobRunning StateRunning 0.42
          , JobFailed "Uninstall failed (installer exit code 1603)"
          , JobPending
          ]
    jobs <- mapM (newJob OpUninstall defaultOpFlags) sample
    modifyState env (\s -> s {stJobs = V.fromList (zipWith JobView jobs statuses)})
    settle
    expect "Queue"
    shot "06-queue"
    modifyState env (\s -> s {stJobs = V.empty})

    putStrLn ("selftest: passed; screenshots in " <> dir)
    exitWith ExitSuccess
