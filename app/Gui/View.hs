{-# LANGUAGE RecordWildCards #-}
-- | The main window: a virtualized, sortable list of installed apps with
-- multi-select, bulk uninstall and upgrade, rule-based selection, disk and
-- kind filters, and a live operation queue.
module Gui.View
  ( ViewCache
  , newViewCache
  , appView
  , rowH
  )
where

import Control.Exception (SomeException, displayException, try)
import Control.Monad (forM, forM_, join, unless, void, when)
import Data.IORef (IORef, newIORef, readIORef, writeIORef)
import Data.List (findIndex, intersperse)
import Data.Map.Strict qualified as Map
import Data.Maybe (isJust, listToMaybe)
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

data DialogPurpose = LoadRules | SaveRules

-- | Apps and total estimated size on one drive (@""@ = unknown drive).
data DiskStat = DiskStat
  { dsDrive :: !Text
  , dsCount :: !Int
  , dsSize :: !Word64
  }

data ViewCache = ViewCache
  { vcVisible :: !(IORef (Maybe (ViewKey, Vector Package)))
  , vcDisks :: !(IORef (Maybe (Int, [DiskStat])))
  , vcListWid :: !(IORef (Maybe WidgetId))
  , vcDialog :: !(IORef (Maybe (DialogPurpose, FileDialogId)))
  , vcTuned :: !(IORef Bool)
  -- ^ Whether the context has been given this app's scroll tuning yet.
  , vcPopupOpen :: !(IORef Bool)
  -- ^ Whether a drop-down or popup was on screen at the end of the last frame.
  }

newViewCache :: IO ViewCache
newViewCache = do
  vcVisible <- newIORef Nothing
  vcDisks <- newIORef Nothing
  vcListWid <- newIORef Nothing
  vcDialog <- newIORef Nothing
  vcTuned <- newIORef False
  vcPopupOpen <- newIORef False
  pure ViewCache {..}

data RowAction
  = RowToggle !Int !Bool
  | RowUninstall !Package
  | RowUpgrade !Package

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

-- | Known drives in order, then apps whose drive could not be determined.
diskStats :: Vector Package -> [DiskStat]
diskStats pkgs =
  let totals = V.foldl' (\acc p -> Map.insertWith add (pkgDrive p) (1, pkgSize p) acc) Map.empty pkgs
      add (n1, s1) (n2, s2) = (n1 + n2, s1 + s2)
      known = [DiskStat d n s | (d, (n, s)) <- Map.toList totals, not (T.null d)]
      unknown = [DiskStat "" n s | Just (n, s) <- [Map.lookup "" totals]]
   in known <> unknown

diskLabel :: DiskStat -> Text
diskLabel stat =
  (if T.null (dsDrive stat) then "Unknown disk" else dsDrive stat)
    <> " ("
    <> T.intercalate ", " ([thousands (dsCount stat) <> (if dsCount stat == 1 then " app" else " apps")] <> [formatSize (dsSize stat) | dsSize stat > 0])
    <> ")"

-- | A count with thousands separated, as Windows writes them: 1,331.
thousands :: Int -> Text
thousands k
  | k < 0 = "-" <> thousands (negate k)
  | otherwise = T.reverse (T.intercalate "," (T.chunksOf 3 (T.reverse (tshow k))))

appView :: Env -> ViewCache -> NanoUI ()
appView env cache = do
  ctx <- askContext
  inp <- askInput
  uiIO $ readIORef (ctxWakeLoop ctx) >>= mapM_ (writeIORef (envWake env))
  -- Scrolling glides onto its target instead of jumping, everywhere in the
  -- window. Set once: the context keeps it.
  uiIO $ do
    tuned <- readIORef (vcTuned cache)
    unless tuned $ do
      setScrollTuning ctx defaultScrollTuning {scrollSmoothTime = 0.12}
      writeIORef (vcTuned cache) True
  st <- uiIO (readState env)
  let pal = palette

  (query, setQuery) <- useText ""
  (filt, setFilt) <- useState FilterAll
  (sortKey, setSortKey) <- useState SortName
  (sortDesc, setSortDesc) <- useFlag False
  (anchor, setAnchor) <- useState (Nothing :: Maybe Text)
  (pending, setPending) <- useState (Nothing :: Maybe PendingBatch)
  (details, setDetails) <- useState (Nothing :: Maybe Text)
  (rulesOpen, setRulesOpen) <- useFlag False
  (rulesText, setRulesText) <- useText ""
  (modeIx, setModeIx) <- useInt 0
  (force, setForce) <- useFlag False
  (acceptAgreements, setAcceptAgreements) <- useFlag False
  (parallelIx, setParallelIx) <- useInt 0
  (notice, setNotice) <- useState (Nothing :: Maybe Text)
  (pressedRow, setPressedRow) <- useState (Nothing :: Maybe Text)
  (diskChoice, setDiskChoice) <- useState (Nothing :: Maybe Text)

  disks <- uiIO $ do
    cached <- readIORef (vcDisks cache)
    case cached of
      Just (rev, ds) | rev == stRevision st -> pure ds
      _ -> do
        let ds = diskStats (stPackages st)
        writeIORef (vcDisks cache) (Just (stRevision st, ds))
        pure ds
  -- A chosen disk that no longer has apps (after a refresh) falls back to all.
  let disk = diskChoice >>= \d -> if any ((== d) . dsDrive) disks then Just d else Nothing

  visible <- uiIO $ do
    let key = ViewKey (stRevision st) query filt disk sortKey sortDesc
    cached <- readIORef (vcVisible cache)
    case cached of
      Just (k, v) | k == key -> pure v
      _ -> do
        let v = visiblePackages st query filt disk sortKey sortDesc
        writeIORef (vcVisible cache) (Just (key, v))
        pure v

  let selected = stSelected st
      selectedPackages = V.toList (V.filter ((`Set.member` selected) . pkgId) (stPackages st))
      busy =
        Set.fromList
          [pkgId (jobPackage (jvJob jv)) | jv <- V.toList (stJobs st), not (jobIsFinished (jvStatus jv))]
      setSelection f = uiIO (modifyState env (\s -> s {stSelected = f (stSelected s)}))
      visibleIds = Set.fromList (V.toList (V.map pkgId visible))
      anyModal = isJust pending || isJust details || rulesOpen

  -- Keyboard: Ctrl+A selects what is shown, Delete uninstalls the selection,
  -- Escape clears it. All three are applied at the end of the frame, after
  -- every widget has run. Whether a text field holds focus can only be asked
  -- once this frame's nodes exist -- the arena is reset before the view is
  -- built, so asking here would always answer "no" and hand the list every
  -- keystroke meant for the search box.
  let keys = inputKeys inp
      ctrlA = modCtrl (inputModifiers inp) && T.any (`T.elem` "aA\x01") (inputChars inp)

  -- List geometry as the scroller last laid it out, used by the header too.
  -- The viewport is what the rows actually show through: inside the padding
  -- and clear of the scrollbars.
  mWid <- uiIO (readIORef (vcListWid cache))
  mMetrics <- maybe (pure Nothing) (uiIO . getScrollMetrics ctx) mWid
  let n = V.length visible
      -- Size bars are shares of the largest app shown, so a filter rescales
      -- them. App sizes run from KB to hundreds of GB, so the share is on a log
      -- scale from 1 MB: on a linear one, all but a few bars would be empty.
      largest = V.foldl' (\m p -> max m (pkgSize p)) 0 visible
      logMB bytes = logBase 10 (max 1 (fromIntegral bytes / 1048576)) :: Double
      share bytes
        | bytes == 0 || logMB largest <= 0 = 0
        | otherwise = realToFrac (logMB bytes / logMB largest) :: Float
      -- A stand-in until the list has been laid out once.
      viewport = maybe (Rect 0 0 600 600) scrollViewport mMetrics
      off = maybe (V2 0 0) scrollOffset mMetrics
      viewH = rectH viewport
      -- The list's own range: a filter that just shrank it knows the new row
      -- count a frame before the scroller measures the shorter content.
      maxOff = max 0 (fromIntegral n * rowH - viewH)
      y = min maxOff (max 0 (v2Y off))
      lo = max 0 (floor (y / rowH) - 2)
      hi = min (n - 1) (ceiling ((y + viewH) / rowH) + 2)
      -- The list reserves room for its scrollbar when it overflows; the
      -- header gives up the same width so its cells stay over their columns.
      -- The list's content is inset by the scroller's side gap, and on the
      -- right by the whole scrollbar gutter once it overflows.
      listInset = 3
      rightInset = if fromIntegral n * rowH > viewH then scrollBarGutter ScrollBarList 0 else listInset
      -- The row under a point, if the point is on a row's text rather than
      -- its checkbox or buttons. Rows have a fixed height, so this is
      -- arithmetic on the viewport and the scroll offset.
      rowAt pos =
        rowIndexAt pos >>= \i ->
          -- Content x: follows horizontal scroll in a narrow window.
          let localX = v2X pos - rectX viewport + v2X off
           in if localX > checkW && localX < max (rectW viewport) rowMinW - actionW - 24
                then Just (pkgId (visible V.! i))
                else Nothing
      -- The row under a point anywhere across its width.
      rowIndexAt pos
        | not anyModal && isJust mMetrics && rectContains viewport pos =
            let i = floor ((v2Y pos - rectY viewport + y) / rowH)
             in if i >= 0 && i < n then Just i else Nothing
        | otherwise = Nothing
      hoveredRow = rowIndexAt (inputMousePos inp)

  columnWith (fillW . fillH . gap 0 . tight) $ do
    surface (palSurface pal) (palSurface pal) 0 (fillW . tight) $
      columnWith (fillW . gap 0 . tight) $ do
        -- Title, search, filters, and list actions
        rowWith (fillW . padXY 16 12 . gap 10 . alignMid . tight) $ do
          labelWith (tight . alignMid . fontSemiBold . fontSize 24 . fontColor (palText pal)) "Installed apps"
          spacer (Fixed 12) Fit
          q <- rowWith (tight . fixedW 320) (searchField "Search name, id, or publisher" query)
          when (q /= query) (setQuery q)
          fm <- uiFontMetrics
          -- Size each drop-down to its longest option (plus insets and the arrow),
          -- so no option runs past the menu at any font size.
          let fitOptions options = fixedW (max 160 (maximum (0 : map (lineWidth fm) options) + 56))
              kindOptions = map filterLabel [minBound .. maxBound]
              diskOptions = "All disks" : map diskLabel disks
          f <- selectWith (fitOptions kindOptions) kindOptions (fromEnum filt)
          when (f /= fromEnum filt) (setFilt (toEnum f))
          let diskIx = maybe 0 (\d -> maybe 0 (+ 1) (findIndex ((== d) . dsDrive) disks)) disk
          k <- selectWith (fitOptions diskOptions) diskOptions diskIx
          when (k /= diskIx) $
            setDiskChoice (if k == 0 then Nothing else dsDrive <$> listToMaybe (drop (k - 1) disks))
          flex
          whenM (actionButton "Select by rule…") (setRulesOpen True)
          -- A listing is already on its way while one is loading.
          disabledWhen (isJust (stLoading st)) $ do
            whenM (actionButton "Refresh") (uiIO (refresh env (stHasUpdateInfo st)))
            whenM (actionButton "Check updates") (uiIO (refresh env True))

        -- Status and selection actions. alignMid on the row places the row
        -- itself; centring the contents in the 48px band takes one on each
        -- child, the way a package row centres its cells.
        rowWith (fillW . fixedH 48 . statusRowPad . padXY 16 0 . gap 12 . alignMid . tight) $ do
          let total = V.length (stPackages st)
              updates = V.length (V.filter (not . T.null . pkgAvailable) (stPackages st))
              shownSize = V.sum (V.map pkgSize visible)
              selectedBytes = sum (map pkgSize selectedPackages)
              selectedSize = if selectedBytes == 0 then "" else ", " <> formatSize selectedBytes
              counts
                | n == total = thousands total <> " apps"
                | otherwise = thousands n <> " of " <> thousands total <> " apps"
              statusLabel f = labelWith (alignMid . tight . f)
          statusLabel (fontMedium . fontColor (palText pal)) counts
          unless (shownSize == 0) $ statusLabel (fontColor (palTextMuted pal)) ("using " <> formatSize shownSize)
          when (stHasUpdateInfo st && updates > 0) $
            statusLabel (fontColor (palGreen pal)) (thousands updates <> (if updates == 1 then " update" else " updates"))
          case (stLoading st, stError st) of
            (Just msg, _) -> do
              spinnerWith alignMid 16
              statusLabel (fontColor (palAccent pal)) msg
            (_, Just err) -> do
              icon 16 (palRed pal) iconAlert
              statusLabel (fontColor (palRed pal)) (ellipsize 110 err)
            _ | total > 0 -> statusLabel (fontColor (palTextFaint pal)) ("listed in " <> tshow (round (stLoadSeconds st * 1000) :: Int) <> " ms")
            _ -> pure ()
          forM_ notice $ \msg -> do
            icon 16 (palYellow pal) iconAlert
            statusLabel (fontColor (palYellow pal)) (ellipsize 80 msg)
            styled quiet $ whenM (compactButton "Dismiss") (setNotice Nothing)
          flex
          styled quiet $ disabledWhen (V.null visible) $
            whenM (actionButton "Select shown") (setSelection (Set.union visibleIds))
          unless (null selectedPackages) $ do
            styled quiet $ whenM (actionButton "Clear") (setSelection (const Set.empty))
            spacer (Fixed 4) Fit
            statusLabel (fontSemiBold . fontColor (palAccent pal)) (thousands (length selectedPackages) <> " selected" <> selectedSize)
            let upgradable = filter (not . T.null . pkgAvailable) selectedPackages
            unless (null upgradable) $
              styled success $ whenM (actionButton ("Upgrade " <> tshow (length upgradable))) $
                setPending (Just (PendingBatch OpUpgrade upgradable))
            styled destructive $ whenM (actionButton "Uninstall selected") $
              setPending (Just (PendingBatch OpUninstall selectedPackages))
        separator

        -- Column headers, laid out exactly like a row
        rowWith (fillW . fixedH 46 . gap 10 . alignMid . padLR (14 + listInset) (14 + rightInset) . tight) $ do
          let allShown = not (V.null visible) && Set.isSubsetOf visibleIds selected
          toggled <- rowWith (alignMid . tight) (checkbox "" allShown)
          when (toggled /= allShown) $
            setSelection (if toggled then Set.union visibleIds else (`Set.difference` visibleIds))
          let sortable key sizing align title = do
                clicked <- headerCell sizing align title (if sortKey == key then Just sortDesc else Nothing) True
                when clicked $
                  if sortKey == key
                    then setSortDesc (not sortDesc)
                    else setSortKey key >> setSortDesc (key `elem` [SortSize, SortDate])
          sortable SortName (grow . minW 220) HeaderStart "Name"
          sortable SortPublisher (fixedW publisherW) HeaderStart "Publisher"
          void $ headerCell (fixedW versionW) HeaderStart "Version" Nothing False
          sortable SortSize (fixedW sizeColW) HeaderEnd "Size"
          sortable SortDate (fixedW dateW) HeaderStart "Installed"
          spacer (Fixed actionW) Fit
    separator

    -- Virtualized package list: only rows inside the viewport are built.
    -- A filter that shortened the list can leave the offset past the new end;
    -- pull it back before the rows are built, and without a glide, so the
    -- list does not slide away under the first frame of a new search.
    forM_ mWid $ \w ->
      when (abs (y - v2Y off) > 0.5) $
        uiIO (scrollTo ctx w (V2 (v2X off) y) ScrollInstant)
    (wid, actions) <- withKey ("package-list" :: Text) $ scrollArea2D (fillW . fillH) $
      columnWith (fillW . gap 0 . tight) $
        if n == 0
          then do
            rowWith (fillW . padXY 24 40 . gap 12 . alignMid . tight) $
              case (stLoading st, V.null (stPackages st)) of
                (Just msg, True) -> do
                  spinnerWith alignMid 20
                  labelWith (tight . alignMid . fontColor (palTextMuted pal)) msg
                (_, True) -> do
                  icon 22 (palTextFaint pal) iconBox
                  labelWith (tight . alignMid . fontColor (palTextMuted pal)) "No apps listed."
                _ -> do
                  icon 22 (palTextFaint pal) iconBox
                  labelWith (tight . alignMid . fontColor (palTextMuted pal)) "No apps match the search or filters."
            pure []
          else do
            when (lo > 0) $ spacer Fit (Fixed (fromIntegral lo * rowH))
            acts <- forM [lo .. hi] $ \i ->
              let p = visible V.! i
               in withKey (pkgId p) (packageRow i (share (pkgSize p)) (hoveredRow == Just i) (Set.member (pkgId p) selected) (Set.member (pkgId p) busy) p)
            when (hi < n - 1) $ spacer Fit (Fixed (fromIntegral (n - 1 - hi) * rowH))
            pure (concat acts)
    uiIO $ do
      writeIORef (vcListWid cache) (Just wid)
      -- A notch moves the list by exactly one row, so rows always sit at the
      -- same place in the viewport rather than half-cut. Rows are 54px, close
      -- enough to the 60px default that this costs no speed. Everything else
      -- in the window keeps the default step.
      setScrollStep ctx wid rowH

    -- Clicking a row's text (press and release on the same row) opens details.
    when (inputMousePressed inp) $ setPressedRow (rowAt (inputMousePos inp))
    when (inputMouseReleased inp) $ do
      let released = rowAt (inputMousePos inp)
      when (isJust released && released == pressedRow) $ setDetails released
      setPressedRow Nothing

    forM_ actions $ \case
      RowToggle i on -> do
        let p = visible V.! i
            shift = modShift (inputModifiers inp)
            anchorIx = anchor >>= \a -> V.findIndex ((== a) . pkgId) visible
        case anchorIx of
          Just a | shift -> do
            let range = Set.fromList [pkgId (visible V.! j) | j <- [min a i .. max a i]]
            setSelection (if on then Set.union range else (`Set.difference` range))
          _ -> setSelection (if on then Set.insert (pkgId p) else Set.delete (pkgId p))
        setAnchor (Just (pkgId p))
      RowUninstall p -> setPending (Just (PendingBatch OpUninstall [p]))
      RowUpgrade p -> setPending (Just (PendingBatch OpUpgrade [p]))

    unless (V.null (stJobs st)) $ do
      separator
      queuePanel env st

  -- Details dialog
  let detailPkg = details >>= \pid -> V.find ((== pid) . pkgId) (stPackages st)
  (detailResp, detailAct) <- modal (isJust detailPkg) (maybe "" pkgName detailPkg) $
    forM detailPkg detailsBody
  when (respClicked detailResp) (setDetails Nothing)
  forM_ ((,) <$> detailPkg <*> join (join detailAct)) $ \(p, act) -> case act of
    DetailCopyId -> uiIO (void (ctxClipboardSet ctx (pkgId p)))
    DetailOpenLocation -> do
      r <- uiIO (try @SomeException (spawnProcess "explorer.exe" [T.unpack (pkgLocation p)]))
      either (setNotice . Just . T.pack . displayException) (const (pure ())) r
    DetailUninstall -> setDetails Nothing >> setPending (Just (PendingBatch OpUninstall [p]))
    DetailUpgrade -> setDetails Nothing >> setPending (Just (PendingBatch OpUpgrade [p]))

  -- Confirmation for single and bulk operations
  let verb kind = if kind == OpUpgrade then "Upgrade" else "Uninstall"
      countApps count = tshow count <> (if count == 1 then " app" else " apps")
      confirmTitle pb = verb (pbKind pb) <> " " <> countApps (length (pbPackages pb))
  (confirmResp, confirmAct) <- modal (isJust pending) (maybe "" confirmTitle pending) $
    forM pending $ \pb -> columnWith (gap 16 . minW 660 . dialogBody) $ do
      let pkgs = pbPackages pb
          upgrading = pbKind pb == OpUpgrade
          noUninstaller = length (filter ((== NoUninstall) . pkgUninstall) pkgs)
          shownVersion p = if pkgVersion p == "Unknown" then "—" else pkgVersion p
          -- Spacers rather than padding: nano-ui fills padded containers.
          -- Sizes are multiples of 4, as in 'detailsBody'.
          inset = spacer (Fixed 16) Fit
          optionRow title control = rowWith (fillW . gap 16 . alignMid . tight) $ do
            labelWith (tight . fixedW 130 . alignMid . fontColor (palTextMuted pal)) title
            control
          appRows = columnWith (fillW . gap 0 . tight) $ do
            spacer Fit (Fixed 8)
            forM_ (zip [0 :: Int ..] pkgs) $ \(i, p) ->
              withKey i $ rowWith (fillW . fixedH 36 . gap 0 . alignMid . tight) $ do
                inset
                labelWith (tight . alignMid . fontColor (palText pal)) (ellipsize 46 (pkgName p))
                flex
                spacer (Fixed 16) Fit
                labelWith (tight . alignMid . fontSize 15 . fontColor (palTextMuted pal)) (ellipsize 22 (shownVersion p))
                inset
            spacer Fit (Fixed 8)
      let emphasised = inlineWith (fontSemiBold . fontColor (palText pal))
      void . richTextWith (fillW . fontColor (palTextMuted pal)) $
        if upgrading
          then ["These apps will be ", emphasised "updated", " to their newest available versions."]
          else ["These apps will be ", emphasised "removed", " from this PC."]
      -- Only a long list scrolls (about six and a half rows show), so a short
      -- one has no scrollbar.
      surface (palBackground pal) (palBorder pal) 8 (fillW . tight) $
        if length pkgs > 6 then scrollWith (fillW . fixedH 244 . tight) appRows else appRows
      when (not upgrading && noUninstaller > 0) $
        surface (lerpColor (palSurface pal) (palYellow pal) 0.08) (lerpColor (palSurface pal) (palYellow pal) 0.35) 8 (fillW . tight) $
          rowWith (fillW . fixedH 44 . gap 12 . alignMid . tight) $ do
            spacer (Fixed 4) Fit
            icon 18 (palYellow pal) iconAlert
            void . richTextWith (grow . alignMid . fontColor (palYellow pal)) $
              [ inlineWith fontSemiBold (tshow noUninstaller <> (if noUninstaller == 1 then " app has" else " apps have"))
              , " no registered uninstall command and may fail."
              ]
      columnWith (fillW . gap 12 . tight) $ do
        optionRow "Installer UI" $ do
          m <- selectWith (fixedW 260) ["Silent (unattended)", "Installer default", "Interactive"] modeIx
          when (m /= modeIx) (setModeIx m)
        optionRow "Run at once" $ do
          k <- selectWith (fixedW 260) ["1 (recommended)", "2", "4"] parallelIx
          when (k /= parallelIx) (setParallelIx k)
        optionRow "" $ columnWith (gap 4 . tight) $ do
          f <- checkbox "Force" force
          when (f /= force) (setForce f)
          labelWith (tight . fontSize 15 . fontColor (palTextFaint pal)) "Skip WinGet's safety checks, such as for apps that are still running."
        when upgrading $ optionRow "" $ do
          a <- checkbox "Accept package license agreements" acceptAgreements
          when (a /= acceptAgreements) (setAcceptAgreements a)
      separator
      rowWith (fillW . gap 8 . alignMid . tight) $ do
        flex
        cancel <- styled quiet (actionButton "Cancel")
        go <- styled (if upgrading then success else destructive) (actionButton (confirmTitle pb))
        pure (cancel, go)
  when (respClicked confirmResp) (setPending Nothing)
  case (pending, join confirmAct) of
    (_, Just (True, _)) -> setPending Nothing
    (Just pb, Just (_, True)) -> do
      let flags =
            OpFlags
              { opSilent = modeIx == 0
              , opInteractive = modeIx == 2
              , opForce = force
              , opAcceptAgreements = acceptAgreements
              }
          workers = [1, 2, 4] !! max 0 (min 2 parallelIx)
      uiIO (startBatch env (pbKind pb) flags workers (pbPackages pb))
      setPending Nothing
    _ -> pure ()

  -- Rule-based selection, shared with winget-gui-cli --from FILE
  let matches = if rulesOpen then selectPackages (parseSelectors rulesText) (stPackages st) else V.empty
  (rulesResp, rulesAct) <- modal rulesOpen "Select by rule" $ columnWith (gap 16 . minW 860 . dialogBody) $ do
    columnWith (fillW . gap 6 . tight) $ do
      let code = inlineWith (fontMono . fontColor (palText pal))
      void . richTextWith (fillW . fontColor (palTextMuted pal)) $
        ["One rule per line: "]
          <> intersperse ", " (map code ["id:ID", "name:NAME", "publisher:NAME", "disk:D:", "match:TEXT"])
          <> [", or a bare id or name."]
      void . richTextWith (fillW . fontSize 15 . fontColor (palTextFaint pal)) $
        ["Lines starting with ", code "#", " are ignored. The same files work with ", code "winget-gui-cli --from", "."]
    t <- textAreaWith (fillW . fixedH 240 . fontMono) rulesText
    when (t /= rulesText) (setRulesText t)
    separator
    rowWith (fillW . gap 8 . alignMid . tight) $ do
      (load, save) <- styled quiet $ (,) <$> actionButton "Load file…" <*> actionButton "Save rules…"
      flex
      labelWith (tight . alignMid . fontColor (if V.null matches then palTextMuted pal else palText pal)) (tshow (V.length matches) <> " installed apps match")
      spacer (Fixed 8) Fit
      -- Nothing to select until a rule matches.
      disabledWhen (V.null matches) $ do
        add <- actionButton "Add to selection"
        replace <- styled primary (actionButton "Select matches")
        pure (load, save, add, replace)
  when (respClicked rulesResp) (setRulesOpen False)
  forM_ rulesAct $ \(load, save, add, replace) -> do
    let openDialog purpose launch = launch defaultFileDialogOptions >>= mapM_ (\d -> uiIO (writeIORef (vcDialog cache) (Just (purpose, d))))
    when load $ openDialog LoadRules askOpenFileDialog
    when save $ openDialog SaveRules askSaveFileDialog
    when (add || replace) $ do
      let ids = Set.fromList (V.toList (V.map pkgId matches))
      setSelection (if replace then const ids else Set.union ids)
      setRulesOpen False

  mDialog <- uiIO (readIORef (vcDialog cache))
  forM_ mDialog $ \(purpose, did) -> do
    result <- pollFileDialogUi did
    case result of
      FileDialogPending -> pure ()
      FileDialogSelected (path : _) -> do
        uiIO (writeIORef (vcDialog cache) Nothing)
        case purpose of
          LoadRules -> do
            contents <- uiIO (try @SomeException (T.readFile path))
            either (setNotice . Just . T.pack . displayException) setRulesText contents
          SaveRules -> do
            -- Save the typed rules, or the current selection as exact ids.
            let body
                  | not (T.null (T.strip rulesText)) = rulesText
                  | otherwise = T.unlines (map (renderSelector . ById . pkgId) selectedPackages)
            r <- uiIO (try @SomeException (T.writeFile path body))
            either (setNotice . Just . T.pack . displayException) (const (setNotice (Just ("Saved " <> T.pack path)))) r
      _ -> uiIO (writeIORef (vcDialog cache) Nothing)

  -- Whether a text field owns the keyboard right now. Read here, not at the
  -- top of the frame: the node arena is reset before the view is built, so
  -- the focused field only exists to be found once its widget has run.
  editing <- uiIO (textFieldActive ctx)

  -- Ctrl+A and Delete belong to whatever is being edited before they belong
  -- to the list.
  when (ctrlA && not editing && not anyModal) $ setSelection (Set.union visibleIds)
  when (inputKeysElem KeyDelete keys && not editing && not anyModal && not (null selectedPackages)) $
    setPending (Just (PendingBatch OpUninstall selectedPackages))

  -- Escape closes a dialog, or else clears the selection. If a drop-down or
  -- popup was on screen this frame or the last, the Escape was for it and the
  -- selection stays. This runs after every widget, when this frame's popups exist.
  popupWasOpen <- uiIO (readIORef (vcPopupOpen cache))
  popupOpen <- uiIO (floatingPanelActive ctx)
  -- A select's drop-down is not a floating panel: its open flag lives in the
  -- store, and is still set here because input is resolved after the view.
  selectOpen <- uiIO (anySelectOpen <$> getStore ctx)
  uiIO (writeIORef (vcPopupOpen cache) popupOpen)
  -- Escape is deliberately not guarded on `editing`, unlike Ctrl+A and Delete
  -- above. nano-ui's text input ignores Escape, so no field consumes it and
  -- none gives up focus for it; guarding on focus would mean that once the
  -- search box had been clicked -- which is most of the time -- Escape could
  -- never clear the selection again.
  when (inputKeysElem KeyEscape keys) $
    if anyModal
      then setPending Nothing >> setDetails Nothing >> setRulesOpen False
      else unless (selectOpen || popupWasOpen || popupOpen) (setSelection (const Set.empty))

packageRow :: Int -> Float -> Bool -> Bool -> Bool -> Package -> NanoUI [RowAction]
packageRow ix sizeShare isHovered isSelected isBusy p =
  surface rowFill rowFill 0 (fillW . fixedH rowH . tight) $
    rowWith (fillW . fixedH rowH . gap 10 . alignMid . padXY 14 0 . tight) $ do
      toggled <- rowWith (alignMid . tight) (checkbox "" isSelected)
      columnWith (grow . minW 220 . gap 3 . alignMid . nameBlockNudge . cellPad . tight) $ do
        labelWith (tight . fontMedium . fontColor (palText pal)) (ellipsize 64 (pkgName p))
        labelWith (tight . fontSize 14 . fontColor (if isSelected then palTextMuted pal else palTextFaint pal)) (ellipsize 72 (pkgId p))
      rowWith (fixedW publisherW . alignMid . cellPad . tight) $
        labelWith (tight . fontColor (palTextMuted pal)) (ellipsize 24 (pkgPublisher p))
      columnWith (fixedW versionW . gap 3 . alignMid . cellPad . tight) $ do
        labelWith (tight . fontSize 15 . fontColor (palTextMuted pal)) (ellipsize 18 version)
        unless (T.null (pkgAvailable p)) $
          labelWith (tight . fontSize 14 . fontColor (palGreen pal)) ("↑ " <> ellipsize 16 (pkgAvailable p))
      sizeCell sizeColW (formatSize (pkgSize p)) sizeShare
      rowWith (fixedW dateW . alignMid . cellPad . tight) $
        labelWith (tight . fontColor (palTextMuted pal)) (pkgDate p)
      acts <- rowWith (fixedW actionW . gap 4 . alignMid . alignEnd . tight) $
        if isBusy
          then do
            spinnerWith alignMid 16
            labelWith (tight . alignMid . fontColor (palTextMuted pal)) "Queued"
            pure []
          else do
            -- Filled, so a row's actions are easy to find on the stripes. An
            -- upgrade carries a wash of green as well as green text.
            up <-
              if T.null (pkgAvailable p)
                then pure False
                else styled (buttonStyle (tintedFill (palGreen pal))) (rowButton "Upgrade")
            un <- rowButton "Uninstall"
            pure ([RowUpgrade p | up] <> [RowUninstall p | un])
      pure ([RowToggle ix toggled | toggled /= isSelected] <> acts)
  where
    pal = palette
    rowFill
      | isSelected = if isHovered then lerpColor (palAccentSoft pal) (palAccent pal) 0.08 else palAccentSoft pal
      | isHovered = palRowHover pal
      | odd ix = palStripe pal
      | otherwise = palBackground pal
    -- WinGet reports a missing version as "Unknown".
    version = if pkgVersion p == "Unknown" then "—" else pkgVersion p

data DetailAction = DetailCopyId | DetailOpenLocation | DetailUninstall | DetailUpgrade

detailsBody :: Package -> NanoUI (Maybe DetailAction)
detailsBody p = columnWith (gap 12 . minW 720 . dialogBody) $ do
  let pal = palette
      orDash t = if T.null t then "—" else t
      joined = orDash . T.intercalate ", "
      -- Spacers rather than padding: nano-ui fills padded containers.
      -- Fixed sizes here are multiples of 4, so they land on whole device
      -- pixels at every 25% display scale.
      inset = spacer (Fixed 16) Fit
      sectionTitle title = labelWith (tight . fontSize 15 . fontSemiBold . fontColor (palTextFaint pal)) title
      section title fields = columnWith (fillW . gap 8 . tight) $ do
        sectionTitle title
        surface (palBackground pal) (palSeparator pal) 8 (fillW . tight) $
          columnWith (fillW . gap 0 . tight) $ do
            spacer Fit (Fixed 4)
            void fields
            spacer Fit (Fixed 4)
      field title value = rowWith (fillW . fixedH 32 . gap 0 . alignMid . tight) $ do
        inset
        labelWith (tight . fixedW 150 . alignMid . fontColor (palTextMuted pal)) title
        labelWith (tight . alignMid . fontColor (palText pal)) (ellipsize 58 value)
        inset
  section "App" $ do
    field "Id" (pkgId p)
    field "Version" (orDash (pkgVersion p))
    unless (T.null (pkgAvailable p)) $ field "Available" (pkgAvailable p)
    field "Publisher" (orDash (pkgPublisher p))
    field "Source" (orDash (pkgSource p))
    field "Kind" $ case pkgKind p of
      KindWin32 -> "Desktop installer"
      KindMsix -> "MSIX / Store package"
      KindCatalog -> "WinGet catalog package"
    field "Scope" (orDash (pkgScope p))
    field "Installer" (orDash (pkgInstaller p))
    field "Architecture" (orDash (pkgArch p))
  section "Storage" $ do
    field "Disk" (if T.null (pkgDrive p) then "Unknown" else pkgDrive p)
    field "Size" (orDash (formatSize (pkgSize p)))
    field "Installed" (orDash (pkgDate p))
    rowWith (fillW . fixedH 32 . gap 0 . alignMid . tight) $ do
      inset
      labelWith (tight . fixedW 150 . alignMid . fontColor (palTextMuted pal)) "Location"
      selectableTextWith (tight . alignMid . fontMono . fontColor (palText pal)) (orDash (pkgLocation p))
      inset
  section "Uninstall" $ do
    rowWith (fillW . fixedH 32 . gap 0 . alignMid . tight) $ do
      inset
      labelWith (tight . fixedW 150 . alignMid . fontColor (palTextMuted pal)) "Support"
      rowWith (gap 8 . alignMid . tight) $ case pkgUninstall p of
        HasSilentUninstall -> do
          icon 20 (palGreen pal) iconCheck
          labelWith (tight . alignMid . fontColor (palText pal)) "Silent uninstall supported"
        HasUninstall -> do
          icon 20 (palTextMuted pal) iconCheck
          labelWith (tight . alignMid . fontColor (palText pal)) "Uninstaller registered"
        NoUninstall -> do
          icon 16 (palYellow pal) iconAlert
          labelWith (tight . alignMid . fontColor (palYellow pal)) "No uninstall command registered"
      inset
    field "Product codes" (joined (pkgProductCodes p))
    field "Package family" (joined (pkgFamilies p))
  separator
  rowWith (fillW . gap 8 . alignMid . tight) $ do
    (copy, open) <- styled quiet $ do
      copy <- actionButton "Copy id"
      open <- disabledWhen (T.null (pkgLocation p)) (actionButton "Open location")
      pure (copy, open)
    flex
    up <- if T.null (pkgAvailable p) then pure False else styled success (actionButton "Upgrade")
    un <- styled destructive (actionButton "Uninstall")
    pure $ case () of
      _
        | copy -> Just DetailCopyId
        | open -> Just DetailOpenLocation
        | up -> Just DetailUpgrade
        | un -> Just DetailUninstall
        | otherwise -> Nothing

queuePanel :: Env -> AppState -> NanoUI ()
queuePanel env st =
  surface (palSurface pal) (palSurface pal) 0 (fillW . fixedH 260 . tight) $
    columnWith (fillW . fillH . padXY 16 10 . gap 8 . tight) $ do
      let jobs = stJobs st
          total = V.length jobs
          finished = V.length (V.filter (jobIsFinished . jvStatus) jobs)
          failed = V.length (V.filter (isFailed . jvStatus) jobs)
          progress jv = case jvStatus jv of
            JobRunning _ f -> f
            s | jobIsFinished s -> 1
            _ -> 0
          overall = if total == 0 then 0 else V.sum (V.map progress jobs) / fromIntegral total
      rowWith (fillW . fixedH 34 . gap 10 . alignMid . tight) $ do
        labelWith (tight . alignMid . fontSemiBold . fontColor (palText pal)) "Queue"
        labelWith (tight . alignMid . fontColor (palTextMuted pal)) (tshow finished <> " of " <> tshow total <> " finished")
        when (failed > 0) $ labelWith (tight . alignMid . fontColor (palRed pal)) (tshow failed <> " failed")
        flex
        when (finished > 0) $ styled quiet $ whenM (actionButton "Clear finished") (uiIO (clearFinished env))
        when (failed > 0) $ whenM (actionButton "Retry failed") (uiIO (retryFailed env))
        when (stActiveBatches st > 0) $ styled destructive $ whenM (actionButton "Stop all") (uiIO (stopBatches env))
      progressBarWith fillW 4 (realToFrac overall)
      scrollWith (fillW . fillH) $ columnWith (fillW . gap 0 . tight) $
        V.forM_ jobs $ \jv -> withKey (jobToken (jvJob jv)) $
          rowWith (fillW . fixedH 36 . gap 10 . alignMid . tight) $ do
            let job = jvJob jv
            -- An inset from the list's edges. A spacer, since nano-ui fills
            -- a container padded 8px or more with the panel colour.
            spacer (Fixed 2) Fit
            -- Every status leads with a mark in the same 20px slot, so the
            -- names after it line up.
            rowWith (fixedW 20 . alignMid . tight) $ case jvStatus jv of
              JobPending -> pure ()
              JobRunning {} -> spinnerWith alignMid 16
              JobSucceeded _ -> icon 20 (palGreen pal) iconCheck
              JobFailed _ -> icon 20 (palRed pal) iconCross
              JobCancelled -> icon 20 (palTextFaint pal) iconCross
            labelWith (tight . fixedW 90 . alignMid . fontColor (palTextFaint pal)) (if jobKind job == OpUpgrade then "Upgrade" else "Uninstall")
            labelWith (tight . fixedW 330 . alignMid . fontColor (palText pal)) (ellipsize 36 (pkgName (jobPackage job)))
            -- Status grows; the action button keeps its own column so Skip and
            -- Cancel line up across rows.
            rowWith (grow . gap 10 . alignMid . tight) $ case jvStatus jv of
              JobPending ->
                labelWith (tight . alignMid . fontColor (palTextMuted pal)) (if Set.member (jobToken job) (stSkipped st) then "Skipping" else "Waiting")
              JobRunning state frac -> do
                progressBarWith (fixedW 180 . alignMid) 4 (realToFrac frac)
                labelWith (tight . alignMid . fontColor (palTextMuted pal)) $ case state of
                  StateQueued -> "Starting"
                  StateRunning -> tshow (round (frac * 100) :: Int) <> "%"
                  StatePost -> "Finishing"
                  StateFinished -> "Done"
              JobSucceeded msg -> labelWith (tight . alignMid . fontColor (palGreen pal)) msg
              JobFailed msg -> labelWith (tight . alignMid . fontColor (palRed pal)) (ellipsize 120 msg)
              JobCancelled -> labelWith (tight . alignMid . fontColor (palTextMuted pal)) "Cancelled"
            rowWith (fixedW 96 . alignMid . tight) $ styled quiet $ case jvStatus jv of
              JobPending -> whenM (compactButton "Skip") (uiIO (cancelJob env job))
              JobRunning {} -> whenM (compactButton "Cancel") (uiIO (cancelJob env job))
              _ -> pure ()
            spacer (Fixed 2) Fit
  where
    pal = palette
    isFailed = \case
      JobFailed _ -> True
      _ -> False

-- | Layout for a dialog's body. nano-ui pads a modal's body equally on both
-- sides and keeps its scrollbar out in that padding, so the body adds none.
dialogBody :: Layout -> Layout
dialogBody l = l {layoutPadding = Padding 0 0 0 0}
