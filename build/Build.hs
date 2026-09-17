-- | The winget-gui build: the Rust bridge, the Haskell executables, the
-- vendored SDL3, and the @dist\\@ folder a release ships.
--
-- > cabal run build                 -- stage dist\
-- > cabal run build -- run          -- ...and start the GUI
-- > cabal run build -- selftest     -- ...and run the scripted self-test
-- > cabal run build -- package      -- ...and zip it as a release asset
-- > cabal run build -- clean
--
-- Everything Haskell and Rust is linked into the executables; at run time they
-- need only SDL3, SDL3_ttf and Windows' own DLLs. The staged folder is checked
-- against that on every build, so a release cannot quietly pick up a DLL that
-- only exists on the machine it was built on.
module Main (main) where

import Codec.Archive.Zip
  ( ZipOption (OptDestination)
  , addEntryToArchive
  , emptyArchive
  , extractFilesFromArchive
  , fromArchive
  , toArchive
  , toEntry
  )
import Control.Monad (filterM, forM, forM_, unless, when)
import Data.ByteString.Lazy qualified as LBS
import Data.Char (isSpace, toLower)
import Data.Digest.Pure.SHA (sha256, showDigest)
import Data.List (intercalate, isPrefixOf, isSuffixOf, nub, sort)
import Data.Time.Clock.POSIX (utcTimeToPOSIXSeconds)
import Development.Shake
import Development.Shake.FilePath
import System.Directory qualified as Dir
import System.Exit (ExitCode (ExitSuccess))

-- | The Rust target whose ABI matches the toolchain GHC links with: clang,
-- lld, compiler-rt and the UCRT. Its static library drops straight into a GHC
-- link, where the default MSVC target's does not.
rustTarget :: String
rustTarget = "x86_64-pc-windows-gnullvm"

bridgeLib :: FilePath
bridgeLib = "bridge/target" </> rustTarget </> "release/libwinget_bridge.a"

distDir, vendorDir, releaseDir, shakeDir, readme :: FilePath
distDir = "dist"
vendorDir = "vendor"
releaseDir = "release"
shakeDir = "dist-newstyle/shake"
readme = "README.md"

executables :: [String]
executables = ["winget-gui", "winget-gui-cli"]

-- | An SDL development package from upstream. Their MinGW builds are
-- self-contained -- FreeType and HarfBuzz are linked into SDL3_ttf.dll -- so a
-- release ships two DLLs. MSYS2's builds keep those as separate DLLs and drag
-- in glib, PCRE2, brotli, zlib and the rest of their tail.
data SdlPackage = SdlPackage
  { sdlName :: String   -- ^ Unpacked directory name, e.g. @SDL3-3.4.14@.
  , sdlDll :: FilePath  -- ^ The one DLL a release needs from it.
  , sdlPc :: FilePath   -- ^ Its pkg-config file, under @lib/pkgconfig@.
  , sdlUrl :: String
  , sdlSha256 :: String -- ^ Of the downloaded .zip.
  }

-- The SDL3 ABI is baked into sdl3-bindgen-sys, which asserts it against the
-- headers it compiles against, so these pins are not free to float: SDL 3.4.16
-- grew SDL_PenProximityEvent and fails that check. Bump them along with the
-- bindings, and read the failing assertion in cabal's build log if one does not
-- take.
sdlPackages :: [SdlPackage]
sdlPackages =
  [ SdlPackage
      { sdlName = "SDL3-3.4.14"
      , sdlDll = "SDL3.dll"
      , sdlPc = "sdl3.pc"
      , sdlUrl = "https://github.com/libsdl-org/SDL/releases/download/release-3.4.14/SDL3-devel-3.4.14-mingw.zip"
      , sdlSha256 = "5f43d6dfadaf13c63c8f52a9f8a31bcfd497faeb569cae9ba6165126d1ebb960"
      }
  , SdlPackage
      { sdlName = "SDL3_ttf-3.2.2"
      , sdlDll = "SDL3_ttf.dll"
      , sdlPc = "sdl3-ttf.pc"
      , sdlUrl = "https://github.com/libsdl-org/SDL_ttf/releases/download/release-3.2.2/SDL3_ttf-devel-3.2.2-mingw.zip"
      , sdlSha256 = "e44e78d6dec080f1d96bd8ac1bac98ddc2574eff980d777c223c799c5a03b1ba"
      }
  ]

-- | The 64-bit tree inside an unpacked package.
sdlTree :: SdlPackage -> FilePath
sdlTree pkg = vendorDir </> sdlName pkg </> "x86_64-w64-mingw32"

sdlDllPath, sdlPcPath :: SdlPackage -> FilePath
sdlDllPath pkg = sdlTree pkg </> "bin" </> sdlDll pkg
sdlPcPath pkg = sdlTree pkg </> "lib/pkgconfig" </> sdlPc pkg

-- | DLLs that ship with Windows. Anything else has to travel with the program.
windowsDlls :: [String]
windowsDlls =
  [ "advapi32.dll", "avrt.dll", "bcrypt.dll", "bcryptprimitives.dll"
  , "cabinet.dll", "cfgmgr32.dll", "comctl32.dll", "comdlg32.dll", "crypt32.dll"
  , "d3d11.dll", "d3d12.dll", "d3d9.dll", "dbghelp.dll", "dinput8.dll"
  , "dwmapi.dll", "dwrite.dll", "dxgi.dll", "dxva2.dll", "gdi32.dll", "hid.dll"
  , "imm32.dll", "iphlpapi.dll", "kernel32.dll", "kernelbase.dll", "ktmw32.dll"
  , "msimg32.dll", "msvcrt.dll", "ncrypt.dll", "netapi32.dll", "normaliz.dll"
  , "ntdll.dll", "ole32.dll", "oleaut32.dll", "opengl32.dll", "powrprof.dll"
  , "propsys.dll", "psapi.dll", "rpcrt4.dll", "secur32.dll", "setupapi.dll"
  , "shcore.dll", "shell32.dll", "shlwapi.dll", "user32.dll", "userenv.dll"
  , "usp10.dll", "uxtheme.dll", "version.dll", "wer.dll", "windowscodecs.dll"
  , "winmm.dll", "winspool.drv", "wintrust.dll", "ws2_32.dll", "wsock32.dll"
  , "xinput1_4.dll"
  ]

-- | API sets, which Windows resolves for itself.
windowsPrefixes :: [String]
windowsPrefixes = ["api-ms-win-", "ext-ms-win-"]

main :: IO ()
main = do
  -- Every path below is relative to the project root, while `cabal run` leaves
  -- the working directory wherever it was called from. Without this, running
  -- the build from a subdirectory scatters dist\, vendor\ and Shake's own
  -- state through the tree and then fails to find bridge\Cargo.toml.
  projectRoot >>= Dir.setCurrentDirectory
  shakeArgs options $ do
    want ["dist"]

    cabalProgram <- newCache (const findCabal)

    let stagedExes = [distDir </> name <.> "exe" | name <- executables]
        staged = stagedExes <> [distDir </> sdlDll pkg | pkg <- sdlPackages]
        stageDist = need staged >> pruneDist staged >> checkDist staged

    phony "dist" stageDist

    phony "run" $ do
      need ["dist"]
      command_ [] (distDir </> "winget-gui.exe") []

    -- The self-test clicks through every dialog headlessly and never starts an
    -- operation, so it is safe to run on the way to a release.
    phony "selftest" $ do
      need ["dist"]
      let out = shakeDir </> "selftest"
      liftIO $ do
        Dir.removePathForcibly out
        Dir.createDirectoryIfMissing True out
      command_ [] (distDir </> "winget-gui.exe") ["--selftest", out]

    phony "package" $ do
      version <- cabalVersion
      need [releaseDir </> "winget-gui-" <> version <> "-x64.zip"]

    -- Not shakeDir: this build is holding its lock file open. It lives under
    -- dist-newstyle along with cabal's own output, which `cabal clean` removes.
    phony "clean" $ do
      putInfo "Removing dist and release..."
      liftIO $ mapM_ Dir.removePathForcibly [distDir, releaseDir]

    -- Cargo tracks its own inputs, so this only has to fire when a source file
    -- changes; cargo then decides whether anything is actually recompiled.
    bridgeLib %> \_ -> do
      sources <- getDirectoryFiles "" ["bridge/src//*.rs", "bridge/Cargo.toml", "bridge/Cargo.lock"]
      need sources
      ensureRustTarget
      putInfo "Building the WinGet bridge (cargo)..."
      command_ [] "cargo"
        ["build", "--release", "--target", rustTarget, "--manifest-path", "bridge/Cargo.toml"]

    -- One cabal invocation produces both executables.
    stagedExes &%> \outs -> do
      need (bridgeLib : map sdlPcPath sdlPackages)
      sources <- getDirectoryFiles "" ["src//*.hs", "app//*.hs", "*.cabal", "cabal.project"]
      need sources
      pkgConfig <- pkgConfigPath
      cabal <- cabalProgram ()
      let withPkgConfig = [AddEnv "PKG_CONFIG_PATH" pkgConfig]
      binPaths <- forM outs $ \out -> do
        Stdout binPath <- command withPkgConfig cabal ["list-bin", takeBaseName out]
        pure (trim binPath)
      -- cabal tracks Haskell sources, not the libraries it links against, so a
      -- rebuilt bridge on its own leaves the executables alone -- and deleting
      -- one is not enough either, since cabal decides from its own cache rather
      -- than from what is on disk. Drop both for any executable that predates
      -- the bridge; cabal then links it again from the objects it already has.
      liftIO $ forM_ binPaths $ \binPath -> do
        built <- Dir.doesFileExist binPath
        when built $ do
          libStamp <- Dir.getModificationTime bridgeLib
          binStamp <- Dir.getModificationTime binPath
          when (binStamp < libStamp) $ do
            Dir.removePathForcibly binPath
            -- .../<component>/<optimization>/build/<exe>/<exe>.exe, so the
            -- component's build cache is three directories up.
            let component = takeDirectory (takeDirectory (takeDirectory binPath))
            Dir.removePathForcibly (component </> "cache" </> "build")
      putInfo "Building the GUI and the CLI (cabal)..."
      command_ withPkgConfig cabal ("build" : ["exe:" <> name | name <- executables])
      strip <- liftIO $ firstExecutable ["llvm-strip", "strip"]
      upx <- liftIO $ Dir.findExecutable "upx"
      forM_ (zip outs binPaths) $ \(out, binPath) -> do
        liftIO $ Dir.copyFile binPath out
        -- GHC executables carry full symbol tables. Strip the staged copies
        -- only; the ones under dist-newstyle keep their symbols for debugging.
        case strip of
          Just prog -> command_ [] prog ["--strip-all", out]
          Nothing -> putWarn ("Neither llvm-strip nor strip is on PATH; " <> out <> " is unstripped.")
        -- Then pack them, which takes the CLI from about 19 MB to under 4. UPX
        -- keeps a stub import for every DLL the original named, so checkDist
        -- still sees the real dependencies.
        case upx of
          Just prog -> do
            putInfo ("Packing " <> out <> " (upx)...")
            command_ [] prog ["--best", "--no-progress", "-q", out]
          Nothing -> putWarn ("upx is not on PATH; " <> out <> " is unpacked.")

    forM_ sdlPackages $ \pkg -> do
      distDir </> sdlDll pkg %> \out -> do
        need [sdlDllPath pkg]
        liftIO $ Dir.copyFile (sdlDllPath pkg) out

      [sdlDllPath pkg, sdlPcPath pkg] &%> \_ -> fetchSdl pkg

    releaseDir </> "winget-gui-*-x64.zip" %> \out -> do
      stageDist
      -- Everything under one folder, so unzipping anywhere is tidy, with a copy
      -- of the README next to the binaries.
      need [readme]
      entries <- forM (staged <> [readme]) $ \member -> do
        content <- liftIO $ LBS.readFile member
        stamp <- liftIO $ Dir.getModificationTime member
        pure (toEntry ("winget-gui/" <> takeFileName member)
                      (floor (utcTimeToPOSIXSeconds stamp))
                      content)
      liftIO $ LBS.writeFile out (fromArchive (foldr addEntryToArchive emptyArchive entries))
      putInfo ("Packaged " <> out)
  where
    options =
      shakeOptions
        { shakeFiles = shakeDir
        , shakeColor = True
        , shakeThreads = 0
        , shakeChange = ChangeModtimeAndDigest
        }

-- | The directory holding cabal.project, searched for upwards from wherever
-- the build was started.
projectRoot :: IO FilePath
projectRoot = Dir.getCurrentDirectory >>= walk
  where
    walk dir = do
      here <- Dir.doesFileExist (dir </> "cabal.project")
      case (here, takeDirectory dir) of
        (True, _) -> pure dir
        (False, up) | up /= dir -> walk up
        _ -> fail "No cabal.project here or in any parent directory."

-- | @PKG_CONFIG_PATH@ for the vendored SDL, which is how the SDL a release
-- ships is also the SDL it was built against. pkg-config splits it on @;@ here.
pkgConfigPath :: Action String
pkgConfigPath = do
  dirs <- liftIO $ mapM (Dir.makeAbsolute . takeDirectory . sdlPcPath) sdlPackages
  pure (intercalate ";" dirs)

fetchSdl :: SdlPackage -> Action ()
fetchSdl pkg = do
  liftIO $ Dir.removePathForcibly (vendorDir </> sdlName pkg)
  withTempFile $ \archive -> do
    putInfo ("Fetching " <> sdlName pkg <> "...")
    command_ [] "curl"
      ["--silent", "--show-error", "--location", "--fail", "--output", archive, sdlUrl pkg]
    bytes <- liftIO $ LBS.readFile archive
    let digest = showDigest (sha256 bytes)
    when (digest /= sdlSha256 pkg) $
      fail (sdlName pkg <> ": SHA-256 is " <> digest <> ", expected " <> sdlSha256 pkg)
    liftIO $ extractFilesFromArchive [OptDestination vendorDir] (toArchive bytes)
  -- Only the 64-bit tree is used; the packages also carry a 32-bit one.
  liftIO $ Dir.removePathForcibly (vendorDir </> sdlName pkg </> "i686-w64-mingw32")
  -- SDL3_ttf ships a .pc holding the prefix of the machine it was built on.
  -- Anchor both of them to the file's own directory instead.
  let pcDir = takeDirectory (sdlPcPath pkg)
  pcFiles <- liftIO $ filter ((== ".pc") . takeExtension) <$> Dir.listDirectory pcDir
  liftIO $ forM_ pcFiles $ \pcFile -> do
    let path = pcDir </> pcFile
    text <- readFile path
    length text `seq` writeFile path (unlines (map anchorPrefix (lines text)))
  where
    anchorPrefix line
      | "prefix=" `isPrefixOf` line = "prefix=${pcfiledir}/../.."
      | otherwise = line

-- | The cabal to build the executables with. @cabal run@ puts its own install
-- directory at the front of PATH, so a nested cabal can resolve to an ancient
-- one someone installed there years ago; take the first on PATH that is new
-- enough to read this project's cabal files.
findCabal :: Action FilePath
findCabal = do
  dirs <- liftIO getSearchPath
  candidates <- filterM doesFileExist [dir </> "cabal" <.> Dir.exeExtension | dir <- dirs]
  usable <- filterM newEnough candidates
  case usable of
    cabal : _ -> pure cabal
    [] -> fail "No cabal 3 or newer on PATH."
  where
    newEnough cabal = do
      (Exit code, Stdout version) <- command [Traced ""] cabal ["--numeric-version"]
      pure $ case (code, reads (takeWhile (/= '.') (trim version))) of
        (ExitSuccess, [(major, "")]) -> major >= (3 :: Int)
        _ -> False

ensureRustTarget :: Action ()
ensureRustTarget = do
  rustup <- liftIO $ Dir.findExecutable "rustup"
  case rustup of
    Nothing -> fail ("rustup is required to build the bridge for " <> rustTarget <> ".")
    Just prog -> do
      Stdout installed <- command [] prog ["target", "list", "--installed"]
      unless (rustTarget `elem` map trim (lines installed)) $ do
        putInfo ("Adding the Rust target " <> rustTarget <> "...")
        command_ [] prog ["target", "add", rustTarget]

-- | Drop anything left in @dist@ by an earlier build. A stale DLL there is
-- exactly the kind of thing that would otherwise ride along into a release.
pruneDist :: [FilePath] -> Action ()
pruneDist staged = do
  present <- doesDirectoryExist distDir
  when present $ do
    entries <- liftIO $ Dir.listDirectory distDir
    let keep = map takeFileName staged
    forM_ entries $ \entry ->
      unless (entry `elem` keep) $ do
        putWarn ("Removing " <> (distDir </> entry) <> ", left over from an earlier build.")
        liftIO $ Dir.removePathForcibly (distDir </> entry)

-- | Fail unless everything staged imports only Windows components and DLLs
-- staged beside it. This is what catches a build that would run here and
-- nowhere else, such as one that picked up VCRUNTIME140.dll or libgcc.
checkDist :: [FilePath] -> Action ()
checkDist files = do
  reader <- liftIO $ firstExecutable ["llvm-readobj", "objdump"]
  case reader of
    Nothing ->
      fail "Neither llvm-readobj nor objdump is on PATH, so dist cannot be checked for stray dependencies."
    Just prog -> do
      let readobj = "llvm-readobj" `isPrefixOf` takeBaseName prog
          present = map (lower . takeFileName) files
      strays <- forM files $ \file -> do
        Stdout out <- command [] prog (if readobj then ["--coff-imports", file] else ["-p", file])
        let imports = sort (nub (concatMap importedDll (lines out)))
        pure [(file, dll) | dll <- imports, not (acceptable present dll)]
      case concat strays of
        [] -> putInfo "dist imports only Windows components and the DLLs staged beside it."
        problems -> do
          forM_ problems $ \(file, dll) -> putWarn (takeFileName file <> " imports " <> dll)
          fail
            ( show (length problems)
                <> " stray dependencies: neither staged in dist nor part of Windows. Stage them, "
                <> "link them statically, or add them to windowsDlls in build/Build.hs if they "
                <> "really do ship with Windows."
            )
  where
    acceptable present dll =
      lower dll `elem` present
        || lower dll `elem` windowsDlls
        || any (`isPrefixOf` lower dll) windowsPrefixes

-- | The DLL named by a line of @llvm-readobj --coff-imports@ or @objdump -p@
-- output, both of which spell it @Name: foo.dll@.
importedDll :: String -> [String]
importedDll line = case words line of
  ["Name:", name] | isLibrary name -> [name]
  ["DLL", "Name:", name] | isLibrary name -> [name]
  _ -> []
  where
    isLibrary name = any (`isSuffixOf` lower name) [".dll", ".drv"]

cabalVersion :: Action String
cabalVersion = do
  need ["winget-gui.cabal"]
  text <- liftIO $ readFile "winget-gui.cabal"
  case [trim (drop (length field) line) | line <- lines text, field `isPrefixOf` line] of
    version : _ -> pure version
    [] -> fail "winget-gui.cabal has no version field."
  where
    field = "version:"

firstExecutable :: [String] -> IO (Maybe FilePath)
firstExecutable [] = pure Nothing
firstExecutable (name : rest) =
  Dir.findExecutable name >>= maybe (firstExecutable rest) (pure . Just)

lower :: String -> String
lower = map toLower

trim :: String -> String
trim = dropWhile isSpace . reverse . dropWhile isSpace . reverse
