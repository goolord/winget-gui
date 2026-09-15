-- | Haskell side of the WinGet bridge (@bridge/include/winget_bridge.h@).
--
-- Every call here blocks on WinGet's out-of-process COM server, so run them
-- off the UI thread. They are @safe@ imports: the RTS keeps other Haskell
-- threads running while one waits.
module WinGet.Bridge
  ( SnapshotId (..)
  , Package (..)
  , PackageKind (..)
  , UninstallInfo (..)
  , OpKind (..)
  , OpFlags (..)
  , defaultOpFlags
  , OpState (..)
  , OpResult (..)
  , opSucceeded
  , initBridge
  , listInstalled
  , releaseSnapshot
  , runOperation
  , cancelOperation
  , errorMessage
  , describeResult
  , parsePackages
  )
where

import Control.Exception (bracket)
import Data.Bits ((.|.))
import Data.ByteString qualified as BS
import Data.Int (Int32)
import Data.Text (Text)
import Data.Text qualified as T
import Data.Text.Encoding qualified as TE
import Data.Text.Read qualified as TR
import Data.Vector (Vector)
import Data.Vector qualified as V
import Data.Word (Word32, Word64, Word8)
import Foreign.C.Types (CSize (..))
import Foreign.Marshal.Alloc (alloca)
import Foreign.Ptr (FunPtr, Ptr, castPtr, freeHaskellFunPtr, nullPtr)
import Foreign.Storable (peek)
import Numeric (showHex)

newtype SnapshotId = SnapshotId Word64
  deriving (Eq, Ord, Show)

data PackageKind
  = -- | A classic installer registered under Add/Remove Programs.
    KindWin32
  | -- | An MSIX / Store package.
    KindMsix
  | -- | Correlated with a WinGet catalog entry.
    KindCatalog
  deriving (Eq, Ord, Show, Enum, Bounded)

data UninstallInfo = HasSilentUninstall | HasUninstall | NoUninstall
  deriving (Eq, Ord, Show)

data Package = Package
  { pkgIndex :: !Int
  , pkgId :: !Text
  , pkgName :: !Text
  , pkgVersion :: !Text
  , pkgPublisher :: !Text
  , pkgSource :: !Text
  , pkgAvailable :: !Text
  , pkgScope :: !Text
  , pkgInstaller :: !Text
  , pkgLocation :: !Text
  , pkgDate :: !Text
  , pkgSize :: !Word64
  , pkgProductCodes :: ![Text]
  , pkgFamilies :: ![Text]
  , pkgUninstall :: !UninstallInfo
  , pkgArch :: !Text
  , pkgDrive :: !Text
  -- ^ Volume the app is installed on (@C:@, @Network@), or empty when unknown.
  , pkgKind :: !PackageKind
  , pkgHaystack :: !Text
  -- ^ Case-folded name, id, and publisher, precomputed for filtering.
  }
  deriving (Eq, Show)

data OpKind = OpUninstall | OpUpgrade
  deriving (Eq, Ord, Show)

data OpFlags = OpFlags
  { opSilent :: !Bool
  , opInteractive :: !Bool
  , opForce :: !Bool
  , opAcceptAgreements :: !Bool
  -- ^ Upgrades only: accept package license agreements. Off unless the user opts in.
  }
  deriving (Eq, Show)

defaultOpFlags :: OpFlags
defaultOpFlags = OpFlags {opSilent = True, opInteractive = False, opForce = False, opAcceptAgreements = False}

data OpState = StateQueued | StateRunning | StatePost | StateFinished
  deriving (Eq, Ord, Show, Enum, Bounded)

data OpResult = OpResult
  { resHresult :: !Int32
  , resStatus :: !Word32
  , resInstallerCode :: !Word32
  , resReboot :: !Bool
  }
  deriving (Eq, Show)

opSucceeded :: OpResult -> Bool
opSucceeded r = resStatus r == 0 && resHresult r >= 0

type ProgressCb = Ptr () -> Word32 -> Double -> IO ()

foreign import ccall safe "wg_init" c_wg_init :: IO Int32
foreign import ccall safe "wg_list_installed" c_wg_list_installed :: Word32 -> Ptr CSize -> Ptr Word64 -> Ptr Int32 -> IO (Ptr Word8)
foreign import ccall unsafe "wg_release_snapshot" c_wg_release_snapshot :: Word64 -> IO ()
foreign import ccall "wrapper" mkProgressCb :: ProgressCb -> IO (FunPtr ProgressCb)
foreign import ccall safe "wg_uninstall"
  c_wg_uninstall :: Word64 -> Word32 -> Word32 -> Word64 -> FunPtr ProgressCb -> Ptr () -> Ptr Word32 -> Ptr Word32 -> Ptr Word8 -> IO Int32
foreign import ccall safe "wg_upgrade"
  c_wg_upgrade :: Word64 -> Word32 -> Word32 -> Word64 -> FunPtr ProgressCb -> Ptr () -> Ptr Word32 -> Ptr Word32 -> Ptr Word8 -> IO Int32
foreign import ccall safe "wg_cancel" c_wg_cancel :: Word64 -> IO Word8
foreign import ccall unsafe "wg_error_message" c_wg_error_message :: Int32 -> Ptr CSize -> IO (Ptr Word8)
foreign import ccall unsafe "wg_free" c_wg_free :: Ptr Word8 -> CSize -> IO ()
foreign import ccall unsafe "wg_last_error" c_wg_last_error :: Ptr CSize -> IO (Ptr Word8)
foreign import ccall unsafe "wg_activation_mode" c_wg_activation_mode :: IO Word32

-- | Connect to WinGet. On success, says how: through WinGet's COM server, or
-- with WinGet loaded into this process.
initBridge :: IO (Either Text Text)
initBridge = do
  hr <- c_wg_init
  if hr < 0
    then do
      detail <- alloca $ \lenPtr -> do
        buf <- c_wg_last_error lenPtr
        T.strip . TE.decodeUtf8Lenient <$> (takeBuffer buf =<< peek lenPtr)
      Left <$> if T.null detail then errorMessage hr else pure detail
    else
      Right <$> do
        mode <- c_wg_activation_mode
        pure $ case mode of
          1 -> "WinGet COM server"
          2 -> "WinGet in-process"
          _ -> "WinGet"

-- | Copy a bridge-owned buffer into a 'BS.ByteString' and free it.
--
-- The copy must happen before the free: 'BS.packCStringLen' copies right
-- away, where a lazy @BS.copy@ would read the buffer after it was released.
takeBuffer :: Ptr Word8 -> CSize -> IO BS.ByteString
takeBuffer ptr len
  | ptr == nullPtr = pure BS.empty
  | otherwise = do
      bytes <- BS.packCStringLen (castPtr ptr, fromIntegral len)
      c_wg_free ptr len
      pure bytes

-- | Snapshot installed packages. Checking updates correlates every package
-- with the configured sources, which is slower and may touch the network.
listInstalled :: Bool -> IO (Either Text (SnapshotId, Vector Package))
listInstalled checkUpdates =
  alloca $ \lenPtr -> alloca $ \snapPtr -> alloca $ \hrPtr -> do
    buf <- c_wg_list_installed (if checkUpdates then 1 else 0) lenPtr snapPtr hrPtr
    if buf == nullPtr
      then Left <$> (errorMessage =<< peek hrPtr)
      else do
        bytes <- takeBuffer buf =<< peek lenPtr
        snap <- peek snapPtr
        pure (Right (SnapshotId snap, parsePackages (TE.decodeUtf8Lenient bytes)))

releaseSnapshot :: SnapshotId -> IO ()
releaseSnapshot (SnapshotId s) = c_wg_release_snapshot s

-- | Parse the bridge's record format (see the header for field order).
parsePackages :: Text -> Vector Package
parsePackages = V.fromList . concatMap parseRecord . T.split (== '\x1E')
  where
    parseRecord rec
      | T.null rec = []
      | otherwise = case T.split (== '\x1F') rec of
          [ix, pid, name, ver, pub, src, avail, scope, inst, loc, date, size, codes, fams, unin, arch, drive] ->
            [ Package
                { pkgIndex = readInt ix
                , pkgId = pid
                , pkgName = if T.null name then pid else name
                , pkgVersion = ver
                , pkgPublisher = pub
                , pkgSource = src
                , pkgAvailable = avail
                , pkgScope = scope
                , pkgInstaller = inst
                , pkgLocation = loc
                , pkgDate = date
                , pkgSize = readWord size
                , pkgProductCodes = splitList codes
                , pkgFamilies = splitList fams
                , pkgUninstall = case unin of
                    "S" -> HasSilentUninstall
                    "U" -> HasUninstall
                    _ -> NoUninstall
                , pkgArch = arch
                , pkgDrive = drive
                , pkgKind = kindOf pid
                , pkgHaystack = T.toCaseFold (T.intercalate "\n" [name, pid, pub])
                }
            ]
          _ -> []
    readInt :: Text -> Int
    readInt t = either (const 0) fst (TR.decimal t)
    readWord :: Text -> Word64
    readWord t = either (const 0) fst (TR.decimal t)
    splitList t = filter (not . T.null) (T.split (== ';') t)
    kindOf pid
      | "ARP\\" `T.isPrefixOf` pid = KindWin32
      | "MSIX\\" `T.isPrefixOf` pid = KindMsix
      | otherwise = KindCatalog

-- | Run an uninstall or upgrade to completion. The progress action runs on a
-- WinGet callback thread; keep it short.
runOperation :: OpKind -> SnapshotId -> Int -> OpFlags -> Word64 -> (OpState -> Double -> IO ()) -> IO OpResult
runOperation kind (SnapshotId snap) index flags token onProgress =
  bracket (mkProgressCb progress) freeHaskellFunPtr $ \cb ->
    alloca $ \statusPtr -> alloca $ \codePtr -> alloca $ \rebootPtr -> do
      hr <- call snap (fromIntegral index) flagBits token cb nullPtr statusPtr codePtr rebootPtr
      OpResult hr <$> peek statusPtr <*> peek codePtr <*> ((/= 0) <$> peek rebootPtr)
  where
    call = case kind of
      OpUninstall -> c_wg_uninstall
      OpUpgrade -> c_wg_upgrade
    flagBits =
      (if opSilent flags then 1 else 0)
        .|. (if opInteractive flags then 2 else 0)
        .|. (if opForce flags then 4 else 0)
        .|. (if opAcceptAgreements flags then 8 else 0)
    progress _ st frac = onProgress (toEnum (fromIntegral (min 3 st))) frac

-- | Ask WinGet to cancel the operation started with this token.
cancelOperation :: Word64 -> IO Bool
cancelOperation token = (/= 0) <$> c_wg_cancel token

errorMessage :: Int32 -> IO Text
errorMessage hr = alloca $ \lenPtr -> do
  buf <- c_wg_error_message hr lenPtr
  msg <- T.strip . TE.decodeUtf8Lenient <$> (takeBuffer buf =<< peek lenPtr)
  pure (if T.null msg then hex else msg <> " (" <> hex <> ")")
  where
    hex = "0x" <> T.justifyRight 8 '0' (T.pack (showHex (fromIntegral hr :: Word32) ""))

-- | A one-line explanation of a finished operation.
describeResult :: OpKind -> OpResult -> IO Text
describeResult kind r
  | opSucceeded r = pure (if resReboot r then "Done; restart required" else "Done")
  | otherwise = do
      detail <- if resHresult r < 0 then errorMessage (resHresult r) else pure ""
      let status = statusName kind (resStatus r)
          code = if resInstallerCode r /= 0 then " (installer exit code " <> T.pack (show (resInstallerCode r)) <> ")" else ""
      pure (T.intercalate ": " (filter (not . T.null) [status <> code, detail]))

statusName :: OpKind -> Word32 -> Text
statusName OpUninstall = \case
  0 -> "Ok"
  1 -> "Blocked by policy"
  2 -> "Catalog error"
  3 -> "Internal error"
  4 -> "Invalid options"
  5 -> "Uninstall failed"
  6 -> "Manifest error"
  n -> "Status " <> T.pack (show n)
statusName OpUpgrade = \case
  0 -> "Ok"
  1 -> "Blocked by policy"
  2 -> "Catalog error"
  3 -> "Internal error"
  4 -> "Invalid options"
  5 -> "Download failed"
  6 -> "Install failed"
  7 -> "Manifest error"
  8 -> "No applicable installer"
  9 -> "No applicable upgrade"
  10 -> "Package agreements not accepted"
  n -> "Status " <> T.pack (show n)
