module Main (main) where

import Gui.SelfTest (screenshot, selfTest)
import Gui.State (newEnv, startupLoad)
import Gui.Style (appTheme)
import Gui.View (appView, newViewCache)
import NanoUI (Size (..))
import NanoUI.Backend.Sdl (SdlOptions (..), defaultSdlOptions, runSdlApp)
import System.Environment (getArgs)
import System.IO (BufferMode (..), hSetBuffering, stdout)
import Text.Read (readMaybe)

windowOptions :: SdlOptions
windowOptions =
  defaultSdlOptions
    { sdlWindowTitle = "Installed apps - winget-gui"
    , sdlWindowSize = Size 1440 900
    , sdlAppTheme = Just appTheme
    }

main :: IO ()
main = do
  -- Keep progress lines even if a headless run dies before flushing.
  hSetBuffering stdout LineBuffering
  args <- getArgs
  env <- newEnv
  cache <- newViewCache
  case args of
    -- Headless modes skip the update check so the list stays put while they run.
    ["--screenshot", path] -> startupLoad env False >> screenshot windowOptions env cache path 0
    ["--screenshot", path, "--select", n]
      | Just k <- readMaybe n -> startupLoad env False >> screenshot windowOptions env cache path k
    ["--selftest", dir] -> startupLoad env False >> selfTest windowOptions env cache dir
    _ -> startupLoad env True >> runSdlApp windowOptions (appView env cache)
