{-# LANGUAGE RecordWildCards #-}
-- | The main window: a virtualized, sortable list of installed apps with
-- multi-select, bulk uninstall and upgrade, rule-based selection, disk and
-- kind filters, and a live operation queue.
--
-- Each frame gathers what its widgets read into a 'Frame', then builds the
-- window from top to bottom: the header, the package list, the queue, the
-- dialogs, and last the keyboard shortcuts, which need every widget to have run.
module Gui.View
  ( ViewCache
  , newViewCache
  , appView
  , rowH
  )
where

import Control.Exception (SomeException, displayException, try)
import Control.Monad (forM, forM_, join, mfilter, unless, void, when)
import Data.IORef (IORef, newIORef, readIORef, writeIORef)
import Data.List (find, findIndex, intersperse)
import Data.Map.Strict qualified as Map
import Data.Maybe (isJust, listToMaybe)
import Data.Set (Set)
import Data.Set qualified as Set
import Data.Text (Text)
import Data.Text qualified as T
import Data.Text.IO qualified as T
import Data.Vector (Vector)
import Data.Vector qualified as V
import Data.Word (Word64)
import Gui.State
import Gui.Style
import NanoUI
import NanoUI.Backend.Sdl (FileDialogId, FileDialogResult (..), askOpenFileDialog, askSaveFileDialog, defaultFileDialogOptions, pollFileDialogUi)
import NanoUI.Context (Context (..))
import NanoUI.Monad (askContext, askInput)
import NanoUI.Store (anySelectOpen)
import NanoUI.Testing (floatingPanelActive, getStore, textFieldActive)
import System.Process (spawnProcess)
import WinGet.Bridge
import WinGet.Queue
import WinGet.Select

--------------------------------------------------------------------------------
-- Caches kept between frames

data ViewCache = ViewCache
  { vcVisible :: !(IORef (Maybe (ViewKey, Vector Package)))
  , vcDisks :: !(IORef (Maybe (Int, [DiskStat])))
  , vcListWid :: !(IORef (Maybe WidgetId))
  , vcDialog :: !(IORef (Maybe (DialogPurpose, FileDialogId)))
  -- ^ A rules file dialog waiting for the user's answer.
  , vcTuned :: !(IORef Bool)
  -- ^ Whether the context has been given this app's scroll tuning yet.
  , vcPopupOpen :: !(IORef Bool)
  -- ^ Whether a drop-down or popup was on screen at the end of the last frame.
  }

data DialogPurpose = LoadRules | SaveRules

newViewCache :: IO ViewCache
newViewCache = do
  vcVisible <- newIORef Nothing
  vcDisks <- newIORef Nothing
  vcListWid <- newIORef Nothing
  vcDialog <- newIORef Nothing
  vcTuned <- newIORef False
  vcPopupOpen <- newIORef False
  pure ViewCache {..}

-- | A value cached under a key, computed again only when the key changes.
cached :: Eq k => IORef (Maybe (k, v)) -> k -> v -> IO v
cached ref key value =
  readIORef ref >>= \case
    Just (k, v) | k == key -> pure v
    _ -> writeIORef ref (Just (key, value)) >> pure value

--------------------------------------------------------------------------------
-- One frame's inputs

-- | A piece of the view's own state: its value this frame, and its setter.
data Var a = Var
  { val :: !a
  , put :: a -> NanoUI ()
  }

var :: NanoUI (a, a -> NanoUI ()) -> NanoUI (Var a)
var hook = uncurry Var <$> hook

-- | Show a control for a variable and keep what it returns. Setting the value
-- a variable already has does nothing.
bind :: Var a -> (a -> NanoUI a) -> NanoUI ()
bind v control = control (val v) >>= put v

-- | The view's own state, which nano-ui keeps between frames.
data Vars = Vars
  { query :: !(Var Text)
  , kindFilter :: !(Var Filter)
  , sortKey :: !(Var SortKey)
  , sortDesc :: !(Var Bool)
  , anchor :: !(Var (Maybe Text))
  -- ^ Id of the row a shift-click range extends from.
  , pending :: !(Var (Maybe PendingBatch))
  -- ^ The operation the confirmation dialog asks about.
  , details :: !(Var (Maybe Text))
  -- ^ Id of the app the details dialog shows.
  , rulesOpen :: !(Var Bool)
  , rulesText :: !(Var Text)
  , installerUi :: !(Var InstallerUi)
  , force :: !(Var Bool)
  , acceptAgreements :: !(Var Bool)
  , parallelIx :: !(Var Int)
  -- ^ Index into 'parallelism'.
  , notice :: !(Var (Maybe Text))
  -- ^ A warning shown in the status line until dismissed.
  , pressedRow :: !(Var (Maybe Text))
  -- ^ Id of the row the mouse went down on, to tell a click from a drag.
  , diskChoice :: !(Var (Maybe Text))
  -- ^ The disk picked in the menu, which may since have lost all its apps.
  }

useVars :: NanoUI Vars
useVars = do
  query <- var (useText "")
  kindFilter <- var (useState FilterAll)
  sortKey <- var (useState SortName)
  sortDesc <- var (useFlag False)
  anchor <- var (useState Nothing)
  pending <- var (useState Nothing)
  details <- var (useState Nothing)
  rulesOpen <- var (useFlag False)
  rulesText <- var (useText "")
  installerUi <- var (useEnum InstallerSilent)
  force <- var (useFlag False)
  acceptAgreements <- var (useFlag False)
  parallelIx <- var (useInt 0)
  notice <- var (useState Nothing)
  pressedRow <- var (useState Nothing)
  diskChoice <- var (useState Nothing)
  pure Vars {..}

-- | Everything the parts of one frame read.
data Frame = Frame
  { env :: Env
  , cache :: ViewCache
  , ctx :: Context
  , inp :: Input
  , st :: AppState
  , vars :: Vars
  , disks :: [DiskStat]
  , disk :: Maybe Text
  -- ^ The disk filter in effect.
  , visible :: Vector Package
  -- ^ The apps the list shows, filtered and sorted.
  , visibleIds :: Set Text
  , selectedPackages :: [Package]
  , busy :: Set Text
  -- ^ Ids of apps with a job queued or running.
  , anyModal :: Bool
  , geom :: ListGeometry
  }

prepareFrame :: Env -> ViewCache -> Context -> Input -> AppState -> Vars -> IO Frame
prepareFrame env cache ctx inp st vars@Vars {..} = do
  disks <- cached (vcDisks cache) (stRevision st) (diskStats (stPackages st))
  -- A chosen disk that no longer has apps (after a refresh) falls back to all.
  let disk = mfilter (\d -> any ((== d) . dsDrive) disks) (val diskChoice)
      (q, filt, key, desc) = (val query, val kindFilter, val sortKey, val sortDesc)
  visible <-
    cached (vcVisible cache) (ViewKey (stRevision st) q filt disk key desc) $
      visiblePackages st q filt disk key desc
  listWid <- readIORef (vcListWid cache)
  metrics <- maybe (pure Nothing) (getScrollMetrics ctx) listWid
  pure
    Frame
      { visibleIds = Set.fromList (V.toList (V.map pkgId visible))
      , selectedPackages = V.toList (V.filter ((`Set.member` stSelected st) . pkgId) (stPackages st))
      , busy = Set.fromList [pkgId (jobPackage (jvJob jv)) | jv <- V.toList (stJobs st), not (jobIsFinished (jvStatus jv))]
      , anyModal = isJust (val pending) || isJust (val details) || val rulesOpen
      , geom = listGeometry listWid metrics (V.length visible)
      , ..
      }

setSelection :: Frame -> (Set Text -> Set Text) -> NanoUI ()
setSelection Frame {..} f = uiIO (modifyState env (\s -> s {stSelected = f (stSelected s)}))

-- | Ask the user to confirm an operation.
askConfirm :: Frame -> OpKind -> [Package] -> NanoUI ()
askConfirm Frame {vars = Vars {..}} kind pkgs = put pending (Just (PendingBatch kind pkgs))

-- | Run an action, showing the exception as the notice if it throws.
attempt :: Frame -> IO a -> NanoUI (Maybe a)
attempt Frame {vars = Vars {..}} action =
  uiIO (try action) >>= \case
    Left (e :: SomeException) -> Nothing <$ put notice (Just (T.pack (displayException e)))
    Right a -> pure (Just a)

--------------------------------------------------------------------------------
-- The window

appView :: Env -> ViewCache -> NanoUI ()
appView env cache = do
  ctx <- askContext
  inp <- askInput
  uiIO $ do
    readIORef (ctxWakeLoop ctx) >>= mapM_ (writeIORef (envWake env))
    tuneScrollingOnce ctx cache
  st <- uiIO (readState env)
  vars <- useVars
  frame <- uiIO (prepareFrame env cache ctx inp st vars)

  columnWith (fillW . fillH . gap 0 . tight) $ do
    surface (palSurface palette) (palSurface palette) 0 (fillW . tight) $
      columnWith (fillW . gap 0 . tight) $ do
        topBar frame
        statusLine frame
        separator
        columnHeaders frame
    separator
    rowActions <- packageList frame
    trackRowClicks frame
    mapM_ (applyRowAction frame) rowActions
    unless (V.null (stJobs st)) $ do
      separator
      queuePanel frame

  detailsDialog frame
  confirmDialog frame
  rulesDialog frame
  pollRulesFile frame
  keyboardShortcuts frame

-- | Scrolling glides onto its target instead of jumping, everywhere in the
-- window. Set once: the context keeps it.
tuneScrollingOnce :: Context -> ViewCache -> IO ()
tuneScrollingOnce ctx cache = do
  tuned <- readIORef (vcTuned cache)
  unless tuned $ do
    setScrollTuning ctx defaultScrollTuning {scrollSmoothTime = 0.12}
    writeIORef (vcTuned cache) True

--------------------------------------------------------------------------------
-- Header

-- | Title, search, filters, and list actions.
topBar :: Frame -> NanoUI ()
topBar Frame {vars = Vars {..}, ..} =
  rowWith (fillW . padXY 16 12 . gap 10 . alignMid . tight) $ do
    labelWith (tight . alignMid . fontSemiBold . fontSize 24 . inkColor palText) "Installed apps"
    spacer (Fixed 12) Fit
    bind query (rowWith (tight . fixedW 320) . searchField "Search name, id, or publisher")
    fm <- uiFontMetrics
    -- Size each drop-down to its longest option (plus insets and the arrow),
    -- so no option runs past the menu at any font size.
    let fitted options = fixedW (max 160 (maximum (0 : map (lineWidth fm) options) + 56))
        kindOptions = enumLabels filterLabel
        diskOptions = "All disks" : map diskLabel disks
        diskIx = diskIndex disks disk
    enumMenu (fitted kindOptions) kindOptions kindFilter
    -- Compared with the menu, not stored outright: a choice that has lost its
    -- apps shows as all disks, and comes back if they do.
    d <- selectWith (fitted diskOptions) diskOptions diskIx
    when (d /= diskIx) $ put diskChoice (diskAt disks d)
    flex
    whenM (actionButton "Select by rule…") (put rulesOpen True)
    -- A listing is already on its way while one is loading.
    disabledWhen (isJust (stLoading st)) $ do
      whenM (actionButton "Refresh") (uiIO (refresh env (stHasUpdateInfo st)))
      whenM (actionButton "Check updates") (uiIO (refresh env True))

-- | Counts, load status, any notice, and what can be done with the selection.
--
-- alignMid on the row places the row itself; centring the contents in the
-- 48px band takes one on each child, the way a package row centres its cells.
statusLine :: Frame -> NanoUI ()
statusLine frame =
  rowWith (fillW . fixedH 48 . statusRowPad . padXY 16 0 . gap 12 . alignMid . tight) $ do
    listSummary frame
    loadStatus frame
    noticeMessage frame
    flex
    selectionActions frame

listSummary :: Frame -> NanoUI ()
listSummary Frame {..} = do
  statusLabel (fontMedium . inkColor palText) counts
  unless (shownSize == 0) $
    statusLabel (inkColor palTextMuted) ("using " <> formatSize shownSize)
  when (stHasUpdateInfo st && updates > 0) $
    statusLabel (inkColor palGreen) (countOf updates "update")
  where
    total = V.length (stPackages st)
    shown = V.length visible
    updates = V.length (V.filter hasUpdate (stPackages st))
    shownSize = V.sum (V.map pkgSize visible)
    counts
      | shown == total = countOf total "app"
      | otherwise = thousands shown <> " of " <> countOf total "app"

-- | A load in progress, or else the last load's error, or else how long the
-- listing took.
loadStatus :: Frame -> NanoUI ()
loadStatus Frame {..}
  | Just msg <- stLoading st = do
      spinnerWith alignMid 16
      statusLabel (inkColor palAccent) msg
  | Just err <- stError st = do
      icon 16 (palRed palette) iconAlert
      statusLabel (inkColor palRed) (ellipsize 110 err)
  | V.null (stPackages st) = pure ()
  | otherwise = statusLabel (inkColor palTextFaint) ("listed in " <> tshow ms <> " ms")
  where
    ms = round (stLoadSeconds st * 1000) :: Int

noticeMessage :: Frame -> NanoUI ()
noticeMessage Frame {vars = Vars {..}} =
  forM_ (val notice) $ \msg -> do
    icon 16 (palYellow palette) iconAlert
    statusLabel (inkColor palYellow) (ellipsize 80 msg)
    styled quiet $ whenM (compactButton "Dismiss") (put notice Nothing)

-- | Select shown, and once anything is selected, clear it or act on it.
selectionActions :: Frame -> NanoUI ()
selectionActions frame@Frame {..} = do
  styled quiet $ disabledWhen (V.null visible) $
    whenM (actionButton "Select shown") (setSelection frame (Set.union visibleIds))
  unless (null selectedPackages) $ do
    styled quiet $ whenM (actionButton "Clear") (setSelection frame (const Set.empty))
    spacer (Fixed 4) Fit
    statusLabel (fontSemiBold . inkColor palAccent) (thousands (length selectedPackages) <> " selected" <> selectedSize)
    unless (null upgradable) $
      styled success $ whenM (actionButton ("Upgrade " <> thousands (length upgradable))) $
        askConfirm frame OpUpgrade upgradable
    styled destructive $ whenM (actionButton "Uninstall selected") $
      askConfirm frame OpUninstall selectedPackages
  where
    upgradable = filter hasUpdate selectedPackages
    selectedBytes = sum (map pkgSize selectedPackages)
    selectedSize = if selectedBytes == 0 then "" else ", " <> formatSize selectedBytes

-- | Column headers, laid out exactly like a row. When the list overflows it
-- reserves room for its scrollbar, and the header gives up the same width so
-- its cells stay over their columns.
columnHeaders :: Frame -> NanoUI ()
columnHeaders frame@Frame {vars = Vars {..}, ..} =
  rowWith (fillW . fixedH 46 . gap 10 . alignMid . padLR (14 + listInset) (14 + lgRightInset geom) . tight) $ do
    toggled <- rowWith (alignMid . tight) (checkbox "" allShown)
    when (toggled /= allShown) $
      setSelection frame (if toggled then Set.union visibleIds else (`Set.difference` visibleIds))
    sortable SortName (grow . minW 220) HeaderStart "Name"
    sortable SortPublisher (fixedW publisherW) HeaderStart "Publisher"
    void $ headerCell (fixedW versionW) HeaderStart "Version" Nothing False
    sortable SortSize (fixedW sizeColW) HeaderEnd "Size"
    sortable SortDate (fixedW dateW) HeaderStart "Installed"
    spacer (Fixed actionW) Fit
  where
    allShown = not (V.null visible) && Set.isSubsetOf visibleIds (stSelected st)
    -- Clicking the sorted column flips its direction. Another column starts
    -- largest or newest first for sizes and dates, A to Z otherwise.
    sortable key sizing align title = do
      let isCurrent = val sortKey == key
      clicked <- headerCell sizing align title (if isCurrent then Just (val sortDesc) else Nothing) True
      when clicked $
        if isCurrent
          then put sortDesc (not (val sortDesc))
          else put sortKey key >> put sortDesc (key `elem` [SortSize, SortDate])

--------------------------------------------------------------------------------
-- Package list

-- Column geometry, shared by the header and the rows so they line up.
rowH, checkW, publisherW, versionW, sizeColW, dateW, actionW :: Float
rowH = 54
checkW = 56
publisherW = 220
versionW = 160
sizeColW = 132
dateW = 126
actionW = 196

-- | Narrowest a row lays out (padding, checkbox, the name's minimum, fixed
-- columns, gaps). Below that the list scrolls sideways.
rowMinW :: Float
rowMinW = 2 * 14 + 22 + 220 + publisherW + versionW + sizeColW + dateW + actionW + 6 * 10

-- | The scroller's side gap, which insets the list's content.
listInset :: Float
listInset = 3

-- | The package list as the scroller last laid it out.
data ListGeometry = ListGeometry
  { lgScroller :: !(Maybe WidgetId)
  , lgMeasured :: !Bool
  -- ^ Whether the list has been laid out yet.
  , lgViewport :: !Rect
  -- ^ What the rows show through: inside the padding, clear of the scrollbars.
  , lgOffset :: !V2
  -- ^ The scroll offset the scroller reports.
  , lgTop :: !Float
  -- ^ The vertical offset, clamped to the list's range. A filter that just
  -- shrank the list knows the new row count a frame before the scroller
  -- measures the shorter content.
  , lgFirst :: !Int
  , lgLast :: !Int
  -- ^ The rows to build: those in the viewport, and two either side.
  , lgRightInset :: !Float
  -- ^ The content's inset on the right: the whole scrollbar gutter once the
  -- list overflows.
  }

listGeometry :: Maybe WidgetId -> Maybe ScrollMetrics -> Int -> ListGeometry
listGeometry lgScroller metrics n = ListGeometry {..}
  where
    lgMeasured = isJust metrics
    -- A stand-in until the list has been laid out once.
    lgViewport = maybe (Rect 0 0 600 600) scrollViewport metrics
    lgOffset = maybe (V2 0 0) scrollOffset metrics
    viewH = rectH lgViewport
    contentH = fromIntegral n * rowH
    lgTop = min (max 0 (contentH - viewH)) (max 0 (v2Y lgOffset))
    lgFirst = max 0 (floor (lgTop / rowH) - 2)
    lgLast = min (n - 1) (ceiling ((lgTop + viewH) / rowH) + 2)
    lgRightInset = if contentH > viewH then scrollBarGutter ScrollBarList 0 else listInset

-- | The row under a point, anywhere across its width. Rows have a fixed
-- height, so this is arithmetic on the viewport and the scroll offset.
rowUnder :: Frame -> V2 -> Maybe Int
rowUnder Frame {..} pos
  | anyModal || not (lgMeasured geom) || not (rectContains viewport pos) = Nothing
  | i < 0 || i >= V.length visible = Nothing
  | otherwise = Just i
  where
    viewport = lgViewport geom
    i = floor ((v2Y pos - rectY viewport + lgTop geom) / rowH)

-- | The app whose row text is under a point: not its checkbox or buttons.
rowTextUnder :: Frame -> V2 -> Maybe Text
rowTextUnder frame@Frame {..} pos = do
  i <- rowUnder frame pos
  -- Content x: follows horizontal scroll in a narrow window.
  let x = v2X pos - rectX (lgViewport geom) + v2X (lgOffset geom)
  if x > checkW && x < max (rectW (lgViewport geom)) rowMinW - actionW - 24
    then Just (pkgId (visible V.! i))
    else Nothing

data RowAction
  = RowToggle !Int !Bool
  | RowUninstall !Package
  | RowUpgrade !Package

-- | The virtualized package list: only rows in and near the viewport are built.
packageList :: Frame -> NanoUI [RowAction]
packageList frame@Frame {..} = do
  -- A filter that shortened the list can leave the offset past the new end.
  -- Pull it back before the rows are built, and without a glide, so the list
  -- does not slide away under the first frame of a new search.
  forM_ (lgScroller geom) $ \w ->
    when (abs (lgTop geom - v2Y (lgOffset geom)) > 0.5) $
      uiIO (scrollTo ctx w (V2 (v2X (lgOffset geom)) (lgTop geom)) ScrollInstant)
  (wid, actions) <- withKey ("package-list" :: Text) $ scrollArea2D (fillW . fillH) $
    columnWith (fillW . gap 0 . tight) $
      if V.null visible
        then [] <$ emptyList frame
        else listRows frame
  uiIO $ do
    writeIORef (vcListWid cache) (Just wid)
    -- A notch moves the list by exactly one row, so rows always sit at the
    -- same place in the viewport rather than half-cut. Rows are 54px, close
    -- enough to the 60px default that this costs no speed. Everything else
    -- in the window keeps the default step.
    setScrollStep ctx wid rowH
  pure actions

-- | What the list says with no rows to show.
emptyList :: Frame -> NanoUI ()
emptyList Frame {..} =
  rowWith (fillW . padXY 24 40 . gap 12 . alignMid . tight) $
    case (stLoading st, V.null (stPackages st)) of
      (Just msg, True) -> spinnerWith alignMid 20 >> message msg
      (_, True) -> emptyIcon >> message "No apps listed."
      _ -> emptyIcon >> message "No apps match the search or filters."
  where
    emptyIcon = icon 22 (palTextFaint palette) iconBox
    message = labelWith (tight . alignMid . inkColor palTextMuted)

-- | The rows to build, with spacers standing in for the rows above and below.
listRows :: Frame -> NanoUI [RowAction]
listRows frame@Frame {..} = do
  when (lo > 0) $ spacer Fit (Fixed (fromIntegral lo * rowH))
  actions <- forM [lo .. hi] $ \i -> do
    let p = visible V.! i
        look =
          RowLook
            { rowIndex = i
            , rowShare = sizeShare largest (pkgSize p)
            , rowHovered = hovered == Just i
            , rowSelected = Set.member (pkgId p) (stSelected st)
            , rowBusy = Set.member (pkgId p) busy
            }
    withKey (pkgId p) (packageRow look p)
  when (hi < n - 1) $ spacer Fit (Fixed (fromIntegral (n - 1 - hi) * rowH))
  pure (concat actions)
  where
    n = V.length visible
    lo = lgFirst geom
    hi = lgLast geom
    largest = V.foldl' (\m q -> max m (pkgSize q)) 0 visible
    hovered = rowUnder frame (inputMousePos inp)

-- | A size bar's fill, as a share of the largest app shown, so a filter
-- rescales the bars. App sizes run from KB to hundreds of GB, so the share is
-- on a log scale from 1 MB: on a linear one, all but a few bars would be empty.
sizeShare :: Word64 -> Word64 -> Float
sizeShare largest bytes
  | bytes == 0 || logMB largest <= 0 = 0
  | otherwise = realToFrac (logMB bytes / logMB largest)
  where
    logMB b = logBase 10 (max 1 (fromIntegral b / 1048576)) :: Double

-- | How a row is drawn, beyond its app.
data RowLook = RowLook
  { rowIndex :: !Int
  , rowShare :: !Float
  -- ^ The size bar's fill.
  , rowHovered :: !Bool
  , rowSelected :: !Bool
  , rowBusy :: !Bool
  -- ^ Whether the app has a job queued or running.
  }

packageRow :: RowLook -> Package -> NanoUI [RowAction]
packageRow RowLook {..} p =
  surface rowFill rowFill 0 (fillW . fixedH rowH . tight) $
    rowWith (fillW . fixedH rowH . gap 10 . alignMid . padXY 14 0 . tight) $ do
      toggled <- rowWith (alignMid . tight) (checkbox "" rowSelected)
      nameCell
      textCell publisherW (ellipsize 24 (pkgPublisher p))
      versionCell
      sizeCell sizeColW (formatSize (pkgSize p)) rowShare
      textCell dateW (pkgDate p)
      actions <-
        rowWith (fixedW actionW . gap 4 . alignMid . alignEnd . tight) $
          if rowBusy then queued else buttons
      pure ([RowToggle rowIndex toggled | toggled /= rowSelected] <> actions)
  where
    rowFill
      | rowSelected && rowHovered = lerpColor (palAccentSoft palette) (palAccent palette) 0.08
      | rowSelected = palAccentSoft palette
      | rowHovered = palRowHover palette
      | odd rowIndex = palStripe palette
      | otherwise = palBackground palette
    nameCell =
      columnWith (grow . minW 220 . gap 3 . alignMid . nameBlockNudge . cellPad . tight) $ do
        labelWith (tight . fontMedium . inkColor palText) (ellipsize 64 (pkgName p))
        labelWith (tight . fontSize 14 . inkColor (if rowSelected then palTextMuted else palTextFaint)) (ellipsize 72 (pkgId p))
    textCell width text =
      rowWith (fixedW width . alignMid . cellPad . tight) $
        labelWith (tight . inkColor palTextMuted) text
    versionCell =
      columnWith (fixedW versionW . gap 3 . alignMid . cellPad . tight) $ do
        labelWith (tight . fontSize 15 . inkColor palTextMuted) (ellipsize 18 (displayVersion p))
        when (hasUpdate p) $
          labelWith (tight . fontSize 14 . inkColor palGreen) ("↑ " <> ellipsize 16 (pkgAvailable p))
    queued = do
      spinnerWith alignMid 16
      labelWith (tight . alignMid . inkColor palTextMuted) "Queued"
      pure []
    -- Filled, so a row's actions are easy to find on the stripes. An upgrade
    -- carries a wash of green as well as green text.
    buttons = do
      up <- shownIf (hasUpdate p) $ styled (buttonStyle (tintedFill (palGreen palette))) (rowButton "Upgrade")
      un <- rowButton "Uninstall"
      pure ([RowUpgrade p | up] <> [RowUninstall p | un])

-- | Clicking a row's text, pressing and releasing on the same row, opens its
-- details.
trackRowClicks :: Frame -> NanoUI ()
trackRowClicks frame@Frame {vars = Vars {..}, ..} = do
  when (inputMousePressed inp) $ put pressedRow underMouse
  when (inputMouseReleased inp) $ do
    when (isJust underMouse && underMouse == val pressedRow) $ put details underMouse
    put pressedRow Nothing
  where
    underMouse = rowTextUnder frame (inputMousePos inp)

applyRowAction :: Frame -> RowAction -> NanoUI ()
applyRowAction frame@Frame {vars = Vars {..}, ..} = \case
  RowToggle i on -> do
    let pid = pkgId (visible V.! i)
        anchorIx = val anchor >>= \a -> V.findIndex ((== a) . pkgId) visible
        toggledIds = case anchorIx of
          -- Shift-click toggles the range from the last row toggled.
          Just a | modShift (inputModifiers inp) -> Set.fromList [pkgId (visible V.! j) | j <- [min a i .. max a i]]
          _ -> Set.singleton pid
    setSelection frame (if on then Set.union toggledIds else (`Set.difference` toggledIds))
    put anchor (Just pid)
  RowUninstall p -> askConfirm frame OpUninstall [p]
  RowUpgrade p -> askConfirm frame OpUpgrade [p]

--------------------------------------------------------------------------------
-- Queue

queuePanel :: Frame -> NanoUI ()
queuePanel frame@Frame {..} =
  surface (palSurface palette) (palSurface palette) 0 (fillW . fixedH 260 . tight) $
    columnWith (fillW . fillH . padXY 16 10 . gap 8 . tight) $ do
      rowWith (fillW . fixedH 34 . gap 10 . alignMid . tight) $ do
        labelWith (tight . alignMid . fontSemiBold . inkColor palText) "Queue"
        labelWith (tight . alignMid . inkColor palTextMuted) (tshow finished <> " of " <> tshow (V.length jobs) <> " finished")
        when (failed > 0) $ labelWith (tight . alignMid . inkColor palRed) (tshow failed <> " failed")
        flex
        when (finished > 0) $ styled quiet $ whenM (actionButton "Clear finished") (uiIO (clearFinished env))
        when (failed > 0) $ whenM (actionButton "Retry failed") (uiIO (retryFailed env))
        when (stActiveBatches st > 0) $ styled destructive $ whenM (actionButton "Stop all") (uiIO (stopBatches env))
      progressBarWith fillW 4 (realToFrac overall)
      scrollWith (fillW . fillH) $ columnWith (fillW . gap 0 . tight) $
        V.forM_ jobs $ \jv -> withKey (jobToken (jvJob jv)) (jobRow frame jv)
  where
    jobs = stJobs st
    finished = V.length (V.filter (jobIsFinished . jvStatus) jobs)
    failed = V.length (V.filter (isFailed . jvStatus) jobs)
    overall
      | V.null jobs = 0
      | otherwise = V.sum (V.map (jobFraction . jvStatus) jobs) / fromIntegral (V.length jobs)

jobRow :: Frame -> JobView -> NanoUI ()
jobRow Frame {..} JobView {jvJob = job, jvStatus = status} =
  rowWith (fillW . fixedH 36 . gap 10 . alignMid . tight) $ do
    -- An inset from the list's edges. A spacer, since nano-ui fills a
    -- container padded 8px or more with the panel colour.
    spacer (Fixed 2) Fit
    -- Every status leads with a mark in the same 20px slot, so the names
    -- after it line up.
    rowWith (fixedW 20 . alignMid . tight) (jobMark status)
    labelWith (tight . fixedW 90 . alignMid . inkColor palTextFaint) (opVerb (jobKind job))
    labelWith (tight . fixedW 330 . alignMid . inkColor palText) (ellipsize 36 (pkgName (jobPackage job)))
    -- Status grows; the action button keeps its own column so Skip and
    -- Cancel line up across rows.
    rowWith (grow . gap 10 . alignMid . tight) (jobProgress skipped status)
    rowWith (fixedW 96 . alignMid . tight) $ styled quiet $ case status of
      JobPending -> whenM (compactButton "Skip") cancel
      JobRunning {} -> whenM (compactButton "Cancel") cancel
      _ -> pure ()
    spacer (Fixed 2) Fit
  where
    skipped = Set.member (jobToken job) (stSkipped st)
    cancel = uiIO (cancelJob env job)

jobMark :: JobStatus -> NanoUI ()
jobMark = \case
  JobPending -> pure ()
  JobRunning {} -> spinnerWith alignMid 16
  JobSucceeded _ -> icon 20 (palGreen palette) iconCheck
  JobFailed _ -> icon 20 (palRed palette) iconCross
  JobCancelled -> icon 20 (palTextFaint palette) iconCross

-- | Where a job has got to. A waiting job the user skipped says so until the
-- queue reaches it.
jobProgress :: Bool -> JobStatus -> NanoUI ()
jobProgress skipped = \case
  JobPending -> note palTextMuted (if skipped then "Skipping" else "Waiting")
  JobRunning state frac -> do
    progressBarWith (fixedW 180 . alignMid) 4 (realToFrac frac)
    note palTextMuted $ case state of
      StateQueued -> "Starting"
      StateRunning -> tshow (round (frac * 100) :: Int) <> "%"
      StatePost -> "Finishing"
      StateFinished -> "Done"
  JobSucceeded msg -> note palGreen msg
  JobFailed msg -> note palRed (ellipsize 120 msg)
  JobCancelled -> note palTextMuted "Cancelled"
  where
    note colour = labelWith (tight . alignMid . inkColor colour)

-- | How far through a job is, from 0 to 1.
jobFraction :: JobStatus -> Double
jobFraction = \case
  JobRunning _ frac -> frac
  status | jobIsFinished status -> 1
  _ -> 0

isFailed :: JobStatus -> Bool
isFailed = \case
  JobFailed _ -> True
  _ -> False

--------------------------------------------------------------------------------
-- Dialogs

-- | A modal dialog, open while it has a subject. The title bar's close button
-- runs @close@. Returns the subject and what the body chose, if it chose
-- anything this frame.
dialogFor :: Maybe s -> (s -> Text) -> NanoUI () -> (s -> NanoUI (Maybe c)) -> NanoUI (Maybe (s, c))
dialogFor subject title close body = do
  (resp, chosen) <- modal (isJust subject) (maybe "" title subject) $
    forM subject $ \s -> fmap (s,) <$> body s
  when (respClicked resp) close
  pure (join (join chosen))

-- | Layout for a dialog's body. nano-ui pads a modal's body equally on both
-- sides and keeps its scrollbar out in that padding, so the body adds none.
dialogBody :: Layout -> Layout
dialogBody l = l {layoutPadding = Padding 0 0 0 0}

-- | A gap at the ends of a row in a dialog. Spacers rather than padding:
-- nano-ui fills padded containers. Fixed sizes in dialogs are multiples of 4,
-- so they land on whole device pixels at every 25% display scale.
inset :: NanoUI ()
inset = spacer (Fixed 16) Fit

-- Details ---------------------------------------------------------------------

data DetailAction = DetailCopyId | DetailOpenLocation | DetailUninstall | DetailUpgrade

detailsDialog :: Frame -> NanoUI ()
detailsDialog frame@Frame {vars = Vars {..}, ..} = do
  chosen <- dialogFor shown pkgName (put details Nothing) detailsBody
  forM_ chosen $ \(p, action) -> case action of
    DetailCopyId -> uiIO (void (ctxClipboardSet ctx (pkgId p)))
    DetailOpenLocation -> void (attempt frame (spawnProcess "explorer.exe" [T.unpack (pkgLocation p)]))
    DetailUninstall -> put details Nothing >> askConfirm frame OpUninstall [p]
    DetailUpgrade -> put details Nothing >> askConfirm frame OpUpgrade [p]
  where
    -- Closes by itself if the app leaves the list.
    shown = val details >>= \pid -> V.find ((== pid) . pkgId) (stPackages st)

detailsBody :: Package -> NanoUI (Maybe DetailAction)
detailsBody p = columnWith (gap 12 . minW 720 . dialogBody) $ do
  section "App" $ do
    field "Id" (pkgId p)
    field "Version" (orDash (pkgVersion p))
    when (hasUpdate p) $ field "Available" (pkgAvailable p)
    field "Publisher" (orDash (pkgPublisher p))
    field "Source" (orDash (pkgSource p))
    field "Kind" (kindLabel (pkgKind p))
    field "Scope" (orDash (pkgScope p))
    field "Installer" (orDash (pkgInstaller p))
    field "Architecture" (orDash (pkgArch p))
  section "Storage" $ do
    field "Disk" (if T.null (pkgDrive p) then "Unknown" else pkgDrive p)
    field "Size" (orDash (formatSize (pkgSize p)))
    field "Installed" (orDash (pkgDate p))
    fieldRow "Location" $
      selectableTextWith (tight . alignMid . fontMono . inkColor palText) (orDash (pkgLocation p))
  section "Uninstall" $ do
    fieldRow "Support" $ rowWith (gap 8 . alignMid . tight) (uninstallSupport (pkgUninstall p))
    field "Product codes" (joined (pkgProductCodes p))
    field "Package family" (joined (pkgFamilies p))
  separator
  rowWith (fillW . gap 8 . alignMid . tight) $ do
    (copy, open) <- styled quiet $
      (,) <$> actionButton "Copy id" <*> disabledWhen (T.null (pkgLocation p)) (actionButton "Open location")
    flex
    up <- shownIf (hasUpdate p) $ styled success (actionButton "Upgrade")
    un <- styled destructive (actionButton "Uninstall")
    pure $ firstChosen [(copy, DetailCopyId), (open, DetailOpenLocation), (up, DetailUpgrade), (un, DetailUninstall)]
  where
    orDash t = if T.null t then "—" else t
    joined = orDash . T.intercalate ", "
    section title fields = columnWith (fillW . gap 8 . tight) $ do
      labelWith (tight . fontSize 15 . fontSemiBold . inkColor palTextFaint) title
      surface (palBackground palette) (palSeparator palette) 8 (fillW . tight) $
        columnWith (fillW . gap 0 . tight) $ do
          spacer Fit (Fixed 4)
          void fields
          spacer Fit (Fixed 4)
    fieldRow :: Text -> NanoUI () -> NanoUI ()
    fieldRow title content = rowWith (fillW . fixedH 32 . gap 0 . alignMid . tight) $ do
      inset
      labelWith (tight . fixedW 150 . alignMid . inkColor palTextMuted) title
      content
      inset
    field title value = fieldRow title $ labelWith (tight . alignMid . inkColor palText) (ellipsize 58 value)

kindLabel :: PackageKind -> Text
kindLabel = \case
  KindWin32 -> "Desktop installer"
  KindMsix -> "MSIX / Store package"
  KindCatalog -> "WinGet catalog package"

uninstallSupport :: UninstallInfo -> NanoUI ()
uninstallSupport = \case
  HasSilentUninstall -> do
    icon 20 (palGreen palette) iconCheck
    labelWith (tight . alignMid . inkColor palText) "Silent uninstall supported"
  HasUninstall -> do
    icon 20 (palTextMuted palette) iconCheck
    labelWith (tight . alignMid . inkColor palText) "Uninstaller registered"
  NoUninstall -> do
    icon 16 (palYellow palette) iconAlert
    labelWith (tight . alignMid . inkColor palYellow) "No uninstall command registered"

-- Confirmation ----------------------------------------------------------------

-- | How much of an installer's own UI to show while it runs.
data InstallerUi = InstallerSilent | InstallerDefault | InstallerInteractive
  deriving (Eq, Enum, Bounded)

installerUiLabel :: InstallerUi -> Text
installerUiLabel = \case
  InstallerSilent -> "Silent (unattended)"
  InstallerDefault -> "Installer default"
  InstallerInteractive -> "Interactive"

-- | The "Run at once" options: how many operations run side by side.
parallelism :: [(Text, Int)]
parallelism = [("1 (recommended)", 1), ("2", 2), ("4", 4)]

data ConfirmChoice = ConfirmCancel | ConfirmRun

confirmDialog :: Frame -> NanoUI ()
confirmDialog frame@Frame {vars = Vars {..}, ..} = do
  chosen <- dialogFor (val pending) batchTitle (put pending Nothing) (confirmBody frame)
  forM_ chosen $ \(batch, choice) -> do
    case choice of
      ConfirmCancel -> pure ()
      ConfirmRun -> uiIO (startBatch env (pbKind batch) flags workers (pbPackages batch))
    put pending Nothing
  where
    flags =
      OpFlags
        { opSilent = val installerUi == InstallerSilent
        , opInteractive = val installerUi == InstallerInteractive
        , opForce = val force
        , opAcceptAgreements = val acceptAgreements
        }
    workers = snd (parallelism !! max 0 (min (length parallelism - 1) (val parallelIx)))

batchTitle :: PendingBatch -> Text
batchTitle batch = opVerb (pbKind batch) <> " " <> countOf (length (pbPackages batch)) "app"

confirmBody :: Frame -> PendingBatch -> NanoUI (Maybe ConfirmChoice)
confirmBody Frame {vars = Vars {..}} batch =
  columnWith (gap 16 . minW 660 . dialogBody) $ do
    void . richTextWith (fillW . inkColor palTextMuted) $
      if upgrading
        then ["These apps will be ", emphasised "updated", " to their newest available versions."]
        else ["These apps will be ", emphasised "removed", " from this PC."]
    batchApps pkgs
    when (not upgrading && missingUninstaller > 0) $ uninstallWarning missingUninstaller
    columnWith (fillW . gap 12 . tight) $ do
      optionRow "Installer UI" $
        enumMenu (fixedW 260) (enumLabels installerUiLabel) installerUi
      optionRow "Run at once" $
        bind parallelIx (selectWith (fixedW 260) (map fst parallelism))
      optionRow "" $ columnWith (gap 4 . tight) $ do
        bind force (checkbox "Force")
        labelWith (tight . fontSize 15 . inkColor palTextFaint) "Skip WinGet's safety checks, such as for apps that are still running."
      when upgrading $ optionRow "" $
        bind acceptAgreements (checkbox "Accept package license agreements")
    separator
    rowWith (fillW . gap 8 . alignMid . tight) $ do
      flex
      cancel <- styled quiet (actionButton "Cancel")
      run <- styled (if upgrading then success else destructive) (actionButton (batchTitle batch))
      pure (firstChosen [(cancel, ConfirmCancel), (run, ConfirmRun)])
  where
    pkgs = pbPackages batch
    upgrading = pbKind batch == OpUpgrade
    missingUninstaller = length (filter ((== NoUninstall) . pkgUninstall) pkgs)
    emphasised = inlineWith (fontSemiBold . inkColor palText)
    optionRow title control = rowWith (fillW . gap 16 . alignMid . tight) $ do
      labelWith (tight . fixedW 130 . alignMid . inkColor palTextMuted) title
      control

-- | The apps a batch acts on. Only a long list scrolls (about six and a half
-- rows show), so a short one has no scrollbar.
batchApps :: [Package] -> NanoUI ()
batchApps pkgs =
  surface (palBackground palette) (palBorder palette) 8 (fillW . tight) $
    if length pkgs > 6 then scrollWith (fillW . fixedH 244 . tight) appRows else appRows
  where
    appRows = columnWith (fillW . gap 0 . tight) $ do
      spacer Fit (Fixed 8)
      forM_ (zip [0 :: Int ..] pkgs) $ \(i, p) ->
        withKey i $ rowWith (fillW . fixedH 36 . gap 0 . alignMid . tight) $ do
          inset
          labelWith (tight . alignMid . inkColor palText) (ellipsize 46 (pkgName p))
          flex
          spacer (Fixed 16) Fit
          labelWith (tight . alignMid . fontSize 15 . inkColor palTextMuted) (ellipsize 22 (displayVersion p))
          inset
      spacer Fit (Fixed 8)

-- | A warning that some apps in an uninstall have no uninstall command.
uninstallWarning :: Int -> NanoUI ()
uninstallWarning k =
  surface (yellowWash 0.08) (yellowWash 0.35) 8 (fillW . tight) $
    rowWith (fillW . fixedH 44 . gap 12 . alignMid . tight) $ do
      spacer (Fixed 4) Fit
      icon 18 (palYellow palette) iconAlert
      void . richTextWith (grow . alignMid . inkColor palYellow) $
        [ inlineWith fontSemiBold (countOf k "app" <> (if k == 1 then " has" else " have"))
        , " no registered uninstall command and may fail."
        ]
  where
    yellowWash = lerpColor (palSurface palette) (palYellow palette)

-- Rule selection --------------------------------------------------------------

data RulesAction = RulesFile DialogPurpose | RulesAdd | RulesReplace

-- | Rule-based selection, shared with winget-gui-cli --from FILE.
rulesDialog :: Frame -> NanoUI ()
rulesDialog frame@Frame {vars = Vars {..}, ..} = do
  chosen <- dialogFor matches (const "Select by rule") (put rulesOpen False) (rulesBody frame)
  forM_ chosen $ \(found, action) -> case action of
    RulesFile purpose -> askRulesFile frame purpose
    RulesAdd -> selectMatches (Set.union (idsOf found))
    RulesReplace -> selectMatches (const (idsOf found))
  where
    -- The apps the rules match, while the dialog is open.
    matches
      | val rulesOpen = Just (selectPackages (parseSelectors (val rulesText)) (stPackages st))
      | otherwise = Nothing
    idsOf = Set.fromList . V.toList . V.map pkgId
    selectMatches f = setSelection frame f >> put rulesOpen False

rulesBody :: Frame -> Vector Package -> NanoUI (Maybe RulesAction)
rulesBody Frame {vars = Vars {..}} matches =
  columnWith (gap 16 . minW 860 . dialogBody) $ do
    columnWith (fillW . gap 6 . tight) $ do
      void . richTextWith (fillW . inkColor palTextMuted) $
        ["One rule per line: "]
          <> intersperse ", " (map code ["id:ID", "name:NAME", "publisher:NAME", "disk:D:", "match:TEXT"])
          <> [", or a bare id or name."]
      void . richTextWith (fillW . fontSize 15 . inkColor palTextFaint) $
        ["Lines starting with ", code "#", " are ignored. The same files work with ", code "winget-gui-cli --from", "."]
    bind rulesText (textAreaWith (fillW . fixedH 240 . fontMono))
    separator
    rowWith (fillW . gap 8 . alignMid . tight) $ do
      (load, save) <- styled quiet $ (,) <$> actionButton "Load file…" <*> actionButton "Save rules…"
      flex
      labelWith (tight . alignMid . inkColor (if noMatches then palTextMuted else palText)) (thousands (V.length matches) <> " installed apps match")
      spacer (Fixed 8) Fit
      -- Nothing to select until a rule matches.
      (add, replace) <- disabledWhen noMatches $
        (,) <$> actionButton "Add to selection" <*> styled primary (actionButton "Select matches")
      pure $
        firstChosen
          [(load, RulesFile LoadRules), (save, RulesFile SaveRules), (add, RulesAdd), (replace, RulesReplace)]
  where
    noMatches = V.null matches
    code = inlineWith (fontMono . inkColor palText)

-- | Open a file dialog for rules. The answer arrives in a later frame, to
-- 'pollRulesFile'.
askRulesFile :: Frame -> DialogPurpose -> NanoUI ()
askRulesFile Frame {..} purpose = do
  asked <- case purpose of
    LoadRules -> askOpenFileDialog defaultFileDialogOptions
    SaveRules -> askSaveFileDialog defaultFileDialogOptions
  forM_ asked $ \did -> uiIO (writeIORef (vcDialog cache) (Just (purpose, did)))

-- | Load or save the rules once the file dialog has been answered.
pollRulesFile :: Frame -> NanoUI ()
pollRulesFile frame@Frame {vars = Vars {..}, ..} = do
  asked <- uiIO (readIORef (vcDialog cache))
  forM_ asked $ \(purpose, did) ->
    pollFileDialogUi did >>= \case
      FileDialogPending -> pure ()
      FileDialogSelected (path : _) -> finish >> useFile purpose path
      -- Cancelled, failed, or answered with no file.
      _ -> finish
  where
    finish = uiIO (writeIORef (vcDialog cache) Nothing)
    useFile LoadRules path = attempt frame (T.readFile path) >>= mapM_ (put rulesText)
    useFile SaveRules path = do
      saved <- attempt frame (T.writeFile path rulesToSave)
      when (isJust saved) $ put notice (Just ("Saved " <> T.pack path))
    -- The typed rules, or the current selection as exact ids.
    rulesToSave
      | T.null (T.strip (val rulesText)) = T.unlines (map (renderSelector . ById . pkgId) selectedPackages)
      | otherwise = val rulesText

--------------------------------------------------------------------------------
-- Keyboard

-- | Ctrl+A selects what is shown, Delete uninstalls the selection, and Escape
-- closes a dialog or else clears the selection.
--
-- These run after every widget. Whether a text field holds focus, or a popup
-- is open, can only be asked once this frame's nodes exist: the node arena is
-- reset before the view is built, so asking earlier would always answer "no"
-- and hand the list every keystroke meant for the search box.
keyboardShortcuts :: Frame -> NanoUI ()
keyboardShortcuts frame@Frame {vars = Vars {..}, ..} = do
  editing <- uiIO (textFieldActive ctx)
  popupWasOpen <- uiIO (readIORef (vcPopupOpen cache))
  popupOpen <- uiIO (floatingPanelActive ctx)
  -- A select's drop-down is not a floating panel: its open flag lives in the
  -- store, and is still set here because input is resolved after the view.
  selectOpen <- uiIO (anySelectOpen <$> getStore ctx)
  uiIO (writeIORef (vcPopupOpen cache) popupOpen)

  -- Ctrl+A and Delete belong to whatever is being edited before they belong
  -- to the list.
  let listHasKeys = not editing && not anyModal
  when (ctrlA && listHasKeys) $
    setSelection frame (Set.union visibleIds)
  when (pressed KeyDelete && listHasKeys && not (null selectedPackages)) $
    askConfirm frame OpUninstall selectedPackages

  -- Escape is deliberately not guarded on `editing`. nano-ui's text input
  -- ignores Escape, so no field consumes it and none gives up focus for it;
  -- guarding on focus would mean that once the search box had been clicked
  -- (which is most of the time) Escape could never clear the selection again.
  let escape
        | anyModal = put pending Nothing >> put details Nothing >> put rulesOpen False
        -- A drop-down or popup on screen this frame or the last took it.
        | selectOpen || popupWasOpen || popupOpen = pure ()
        | otherwise = setSelection frame (const Set.empty)
  when (pressed KeyEscape) escape
  where
    pressed key = inputKeysElem key (inputKeys inp)
    ctrlA = modCtrl (inputModifiers inp) && T.any (`T.elem` "aA\x01") (inputChars inp)

--------------------------------------------------------------------------------
-- Small helpers

-- | Apps and total estimated size on one drive (@""@ = unknown drive).
data DiskStat = DiskStat
  { dsDrive :: !Text
  , dsCount :: !Int
  , dsSize :: !Word64
  }

-- | Known drives in order, then apps whose drive could not be determined.
diskStats :: Vector Package -> [DiskStat]
diskStats pkgs = known <> unknown
  where
    totals = V.foldl' (\acc p -> Map.insertWith add (pkgDrive p) (1, pkgSize p) acc) Map.empty pkgs
    add (n1, s1) (n2, s2) = (n1 + n2, s1 + s2)
    known = [DiskStat d n s | (d, (n, s)) <- Map.toList totals, not (T.null d)]
    unknown = [DiskStat "" n s | Just (n, s) <- [Map.lookup "" totals]]

diskLabel :: DiskStat -> Text
diskLabel DiskStat {..} = name <> " (" <> T.intercalate ", " parts <> ")"
  where
    name = if T.null dsDrive then "Unknown disk" else dsDrive
    parts = countOf dsCount "app" : [formatSize dsSize | dsSize > 0]

-- | A disk's place in the disk menu, which lists all disks first.
diskIndex :: [DiskStat] -> Maybe Text -> Int
diskIndex stats = maybe 0 (\d -> maybe 0 (+ 1) (findIndex ((== d) . dsDrive) stats))

-- | The disk at a place in the disk menu; 'Nothing' for all disks.
diskAt :: [DiskStat] -> Int -> Maybe Text
diskAt stats i
  | i <= 0 = Nothing
  | otherwise = dsDrive <$> listToMaybe (drop (i - 1) stats)

-- | A count with thousands separated, as Windows writes them: 1,331.
thousands :: Int -> Text
thousands k
  | k < 0 = "-" <> thousands (negate k)
  | otherwise = T.reverse (T.intercalate "," (T.chunksOf 3 (T.reverse (tshow k))))

-- | A count and a noun, in the plural unless the count is one: "1,331 apps".
countOf :: Int -> Text -> Text
countOf k noun = thousands k <> " " <> noun <> (if k == 1 then "" else "s")

opVerb :: OpKind -> Text
opVerb = \case
  OpUpgrade -> "Upgrade"
  OpUninstall -> "Uninstall"

hasUpdate :: Package -> Bool
hasUpdate = not . T.null . pkgAvailable

-- | WinGet reports a missing version as "Unknown".
displayVersion :: Package -> Text
displayVersion p = if pkgVersion p == "Unknown" then "—" else pkgVersion p

inkColor :: (Palette -> Color) -> Layout -> Layout
inkColor colour = fontColor (colour palette)

statusLabel :: (Layout -> Layout) -> Text -> NanoUI ()
statusLabel style = labelWith (alignMid . tight . style)

-- | Top padding for the two-line name block. Its line boxes leave more room
-- under the smaller id line than above the name, so a box-centred block reads
-- about 4px high next to single-line cells (measured at the 18px UI font).
-- Top padding shifts the block's text down by the full amount.
nameBlockNudge :: Layout -> Layout
nameBlockNudge layout = layout {layoutPadding = (layoutPadding layout) {padT = 4}}

-- | The status line's slot. The row above it ends with 12px of padding inside
-- its own band, while this row has none, so centring in the raw 48px band
-- reads low: the eye measures the gap from the row above's text, not from its
-- box edge. Matching that padding at the bottom centres the contents in the
-- gap that is actually visible, between the row above and the separator below.
statusRowPad :: Layout -> Layout
statusRowPad layout = layout {layoutPadding = (layoutPadding layout) {padB = 12}}

-- | Every value of an enumeration's label, in order.
enumLabels :: (Enum a, Bounded a) => (a -> Text) -> [Text]
enumLabels labelOf = map labelOf [minBound .. maxBound]

-- | A drop-down over an enumeration, its options in 'fromEnum' order.
enumMenu :: Enum a => (Layout -> Layout) -> [Text] -> Var a -> NanoUI ()
enumMenu sizing options v = bind v (fmap toEnum . selectWith sizing options . fromEnum)

-- | A button shown only when a condition holds; a hidden one is never clicked.
shownIf :: Bool -> NanoUI Bool -> NanoUI Bool
shownIf shown clicked = if shown then clicked else pure False

-- | The action of the first clicked button, in order of priority.
firstChosen :: [(Bool, a)] -> Maybe a
firstChosen = fmap snd . find fst
