-- | The README's demo video, rendered headlessly frame by frame.
--
-- 'demo' drives the real view with scripted pointer and keyboard input and
-- saves every frame as a BMP with the pointer drawn in, ready for ffmpeg. It
-- runs against a dry-run 'Env': confirming the uninstall only queues the
-- jobs, and the script advances them itself, so nothing is uninstalled.
module Gui.Demo
  ( demo
  )
where

import Control.Concurrent (threadDelay)
import Control.Exception (SomeException, fromException, handle, throwIO)
import Control.Monad (forM_, replicateM_, unless, when)
import Data.Bits (shiftL, (.|.))
import Data.ByteString qualified as BS
import Data.ByteString.Unsafe qualified as BSU
import Data.IORef (IORef, newIORef, readIORef, writeIORef)
import Data.List (sortOn)
import Data.Maybe (listToMaybe)
import Data.Text (Text)
import Data.Text qualified as T
import Data.Vector qualified as V
import Data.Word (Word8)
import Foreign.Marshal.Alloc (allocaBytes)
import Foreign.Marshal.Utils (copyBytes)
import Foreign.Ptr (Ptr, castPtr, plusPtr)
import Foreign.Storable (pokeByteOff)
import Gui.SelfTest (Headless (..), headless, uiSpans, waitForList)
import Gui.State
import Gui.View (ViewCache)
import NanoUI (Input (..), Key (..), Rect (..), V2 (..), inputKeysFromList)
import NanoUI.Backend.Sdl (SdlOptions (..), saveScreenshot)
import System.Exit (ExitCode, exitFailure, exitSuccess)
import System.IO (IOMode (..), hPutBuf, hPutStrLn, stderr, withBinaryFile)
import Text.Printf (printf)
import WinGet.Bridge
import WinGet.Queue

fps :: Int
fps = 30

data Rec = Rec
  { recH :: Headless
  , recDir :: FilePath
  , recFrame :: IORef Int
  , recMouse :: IORef V2
  }

demo :: SdlOptions -> Env -> ViewCache -> FilePath -> IO ()
demo opts env cache dir = do
  st0 <- waitForList env
  when (stRevision st0 == 0) $ do
    hPutStrLn stderr "demo: nothing listed"
    exitFailure
  -- Let the update check land too, so rows offer upgrades.
  let awaitUpdates tries = do
        st <- readState env
        unless (stHasUpdateInfo st || tries <= (0 :: Int)) $ threadDelay 250000 >> awaitUpdates (tries - 1)
  awaitUpdates 480
  headless opts env cache $ \h -> handle report $ do
    rec <- Rec h dir <$> newIORef 0 <*> newIORef (V2 1180 560)
    script env rec
    n <- readIORef (recFrame rec)
    putStrLn ("demo: " <> show n <> " frames in " <> dir)
    exitSuccess

-- | Say why a recording stopped; exits pass through.
report :: SomeException -> IO ()
report e = case fromException e of
  Just (code :: ExitCode) -> throwIO code
  Nothing -> hPutStrLn stderr ("demo: FAILED: " <> show e) >> exitFailure

script :: Env -> Rec -> IO ()
script env r = do
  -- Warm up: the first frames lay out the list and load glyphs.
  replicateM_ 4 (hDraw (recH r) True (hBase (recH r)))
  hold r 1.4

  -- Hover down the list, then scroll it.
  moveTo r (V2 760 330) 0.7
  moveTo r (V2 760 620) 0.9
  forM_ [1 :: Int .. 4] $ \_ -> wheel r 1 >> hold r 0.25
  hold r 0.5
  forM_ [1 :: Int .. 4] $ \_ -> wheel r (-1) >> hold r 0.2
  hold r 0.4

  -- Sort by size: the storage bars show where the disk went.
  moveTo r (V2 1166 133) 0.8
  click r
  hold r 2.2

  -- Filter to one drive, then back to all of them.
  moveTo r (V2 900 29) 0.7
  click r
  hold r 0.7
  moveToText r (T.isPrefixOf "D: (") 0.5
  click r
  hold r 1.8
  moveTo r (V2 900 29) 0.4
  click r
  hold r 0.5
  moveToText r (== "All disks") 0.4
  click r
  hold r 0.5

  -- Search, select what it finds, and uninstall it (a dry run).
  moveTo r (V2 330 29) 0.7
  click r
  typeText r "2C-Audio"
  hold r 1.0
  moveTo r (V2 28 132) 0.8
  click r
  hold r 1.3
  moveToText r (== "Uninstall selected") 0.8
  click r
  hold r 2.4
  moveToText r (T.isPrefixOf "Uninstall 6 apps") 0.7
  hold r 0.3
  click r
  hold r 0.6
  runQueue env r
  hold r 1.4
  moveToText r (== "Clear finished") 0.8
  click r
  hold r 0.6

  -- Clear the search and open an app's details.
  moveTo r (V2 330 29) 0.7
  click r
  key r KeyEnd
  forM_ [1 :: Int .. T.length "2C-Audio"] $ \_ -> key r KeyBackspace >> hold r 0.03
  hold r 0.9
  st <- readState env
  forM_ (listToMaybe (V.toList (visiblePackages st "" FilterAll Nothing SortSize True))) $ \p ->
    moveToText r (== ellipsize 64 (pkgName p)) 0.8
  click r
  hold r 2.6
  moveTo r (V2 1100 900) 0.5
  key r KeyEscape
  hold r 0.6

  -- Select by rule.
  moveToText r (== "Select by rule…") 0.9
  click r
  hold r 0.5
  moveTo r (V2 780 470) 0.5
  click r
  typeText r "publisher:Microsoft"
  hold r 2.2
  key r KeyEscape
  hold r 1.2

-- | Run the queued dry-run jobs one after another, as the queue would report
-- them, without running anything.
runQueue :: Env -> Rec -> IO ()
runQueue env r = do
  st <- readState env
  let jobs = [jvJob jv | jv <- V.toList (stJobs st), jvStatus jv == JobPending]
      steps = 26 :: Int
  forM_ jobs $ \job -> do
    forM_ [0 .. steps] $ \i -> do
      let frac = fromIntegral i / fromIntegral steps
          state = if i == 0 then StateQueued else StateRunning
      reportJob env job (JobRunning state frac)
      shoot r id
    reportJob env job (JobSucceeded "Done")
    hold r 0.15
  endDryRunBatch env

-- Frames ---------------------------------------------------------------------

-- | Render one frame with the pointer where it is, save it, and draw the
-- pointer into the saved image.
shoot :: Rec -> (Input -> Input) -> IO ()
shoot r f = do
  pos <- readIORef (recMouse r)
  let h = recH r
      inp = f (hBase h) {inputMousePos = pos, inputDeltaTime = 1 / fromIntegral fps}
  hDraw h True inp
  n <- readIORef (recFrame r)
  let path = recDir r <> "\\" <> printf "f%05d.bmp" n
  saved <- saveScreenshot (hEnv h) path
  unless saved $ fail ("demo: could not save " <> path)
  drawPointer path pos
  writeIORef (recFrame r) (n + 1)

hold :: Rec -> Double -> IO ()
hold r secs = replicateM_ (max 1 (round (secs * fromIntegral fps))) (shoot r id)

-- | Glide the pointer to a point, easing in and out.
moveTo :: Rec -> V2 -> Double -> IO ()
moveTo r (V2 tx ty) secs = do
  V2 fx fy <- readIORef (recMouse r)
  let n = max 1 (round (secs * fromIntegral fps)) :: Int
  forM_ [1 .. n] $ \i -> do
    let t = fromIntegral i / fromIntegral n :: Double
        e = realToFrac (if t < 0.5 then 4 * t * t * t else 1 - ((-2 * t + 2) ** 3) / 2)
    writeIORef (recMouse r) (V2 (fx + (tx - fx) * e) (fy + (ty - fy) * e))
    shoot r id

moveToText :: Rec -> (Text -> Bool) -> Double -> IO ()
moveToText r match secs = do
  spans <- uiSpans (recH r)
  -- The lowest match: a dialog's button sits below a title with the same text.
  case sortOn (\(Rect _ y _ _) -> negate y) [Rect x y w hh | (Rect x y w hh, txt, _, _, _) <- spans, match txt] of
    Rect x y w hh : _ -> moveTo r (V2 (x + w / 2) (y + hh / 2)) secs
    [] -> fail ("demo: no matching text on screen: " <> show [t | (_, t, _, _, _) <- take 60 spans])

click :: Rec -> IO ()
click r = do
  shoot r (\i -> i {inputMouseDown = True, inputMousePressed = True})
  replicateM_ 2 (shoot r (\i -> i {inputMouseDown = True}))
  shoot r (\i -> i {inputMouseReleased = True})

wheel :: Rec -> Float -> IO ()
wheel r notches = shoot r (\i -> i {inputScroll = V2 0 notches})

key :: Rec -> Key -> IO ()
key r k = shoot r (\i -> i {inputKeys = inputKeysFromList [k]})

-- | About twelve characters a second.
typeText :: Rec -> Text -> IO ()
typeText r txt =
  forM_ (T.unpack txt) $ \c -> do
    shoot r (\i -> i {inputChars = T.singleton c})
    replicateM_ 1 (shoot r id)

-- Pointer --------------------------------------------------------------------

-- | The classic arrow: B outline, W fill.
arrow :: [String]
arrow =
  [ "B"
  , "BB"
  , "BWB"
  , "BWWB"
  , "BWWWB"
  , "BWWWWB"
  , "BWWWWWB"
  , "BWWWWWWB"
  , "BWWWWWWWB"
  , "BWWWWWWWWB"
  , "BWWWWWWWWWB"
  , "BWWWWWWBBBBB"
  , "BWWWBWWB"
  , "BWWBBWWB"
  , "BWB  BWWB"
  , "BB   BWWB"
  , "B     BWWB"
  , "      BWWB"
  , "       BB"
  ]

-- | Draw the pointer into a saved BMP (uncompressed, 24 or 32 bits a pixel).
drawPointer :: FilePath -> V2 -> IO ()
drawPointer path (V2 px py) = do
  bs <- BS.readFile path
  let u32 o = foldr (\k acc -> acc `shiftL` 8 .|. fromIntegral (BS.index bs (o + k))) 0 [0 .. 3] :: Int
      offset = u32 10
      width = u32 18
      height0 = u32 22
      height = if height0 >= 0x80000000 then 0x100000000 - height0 else height0
      bottomUp = height0 < 0x80000000
      bytesPer = u32 28 `mod` 0x10000 `div` 8
      stride = (width * bytesPer + 3) `div` 4 * 4
      len = BS.length bs
  BSU.unsafeUseAsCStringLen bs $ \(src, _) ->
    allocaBytes len $ \buf -> do
      copyBytes buf (castPtr src) len
      let put :: Int -> Int -> Word8 -> IO ()
          put x y v =
            when (x >= 0 && y >= 0 && x < width && y < height) $ do
              let row = if bottomUp then height - 1 - y else y
                  base = offset + row * stride + x * bytesPer
              forM_ [0 .. 2] $ \c -> pokeByteOff (buf :: Ptr Word8) (base + c) v
          x0 = round px :: Int
          y0 = round py :: Int
      forM_ (zip [0 ..] arrow) $ \(dy, line) ->
        forM_ (zip [0 ..] line) $ \(dx, ch) -> case ch of
          'B' -> put (x0 + dx) (y0 + dy) 0
          'W' -> put (x0 + dx) (y0 + dy) 255
          _ -> pure ()
      withBinaryFile path WriteMode $ \hdl -> hPutBuf hdl (buf `plusPtr` 0) len
