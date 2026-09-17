module Main (main) where

import Gui.Demo (demo)
import Gui.SelfTest (screenshot, selfTest)
import Gui.State (Env, newDryRunEnv, newEnv, startupLoad)
import Gui.Style (appTheme)
import Gui.View (ViewCache, appView, newViewCache)
import NanoUI (Size (..))
import NanoUI.Backend.Sdl (NanoUIFont (..), SdlOptions (..), defaultSdlOptions, runSdlApp)
import System.Environment (getArgs)
import System.IO (BufferMode (..), hSetBuffering, stdout)
import Text.Read (readMaybe)

windowOptions :: SdlOptions
windowOptions =
  defaultSdlOptions
    { sdlWindowTitle = "Installed apps - winget-gui"
    , sdlWindowSize = Size 1560 960
    , sdlAppTheme = Just appTheme
    , -- Windows' own faces, as its Installed apps page uses: Segoe UI Variable
      -- (plain Segoe UI before Windows 11), and Cascadia Mono for paths and rules.
      sdlAppFont = FontSearch ["SegUIVar", "Segoe UI"]
    , sdlAppMonoFont = FontSearch ["Cascadia Mono", "Consolas"]
    , sdlAppFontSize = 18
    }

main :: IO ()
main = do
  -- Keep progress lines even if a headless run dies before flushing.
  hSetBuffering stdout LineBuffering
  args <- getArgs
  cache <- newViewCache
  case args of
    -- The demo video: operations are only queued, never run, and it checks
    -- for updates so rows offer upgrades.
    ["--demo", dir] -> do
      env <- newDryRunEnv
      startupLoad env True
      demo windowOptions env cache dir
    _ -> newEnv >>= \env -> run env cache args

run :: Env -> ViewCache -> [String] -> IO ()
run env cache args =
  case args of
    -- Headless modes skip the update check so the list stays put while they run.
    ["--screenshot", path] -> startupLoad env False >> screenshot windowOptions env cache path 0
    ["--screenshot", path, "--select", n]
      | Just k <- readMaybe n -> startupLoad env False >> screenshot windowOptions env cache path k
    ["--selftest", dir] -> startupLoad env False >> selfTest windowOptions env cache dir
    _ -> startupLoad env True >> runSdlApp windowOptions (appView env cache)
