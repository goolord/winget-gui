{-# LANGUAGE RecordWildCards #-}
-- | The main window: a virtualized, sortable list of installed apps with
-- multi-select, bulk uninstall and upgrade, rule-based selection, disk and
-- kind filters, and a live operation queue.
module Gui.View
  ( ViewCache
  , newViewCache
  , appView
  )
where

import Control.Exception (SomeException, displayException, try)
import Control.Monad (forM, forM_, join, unless, void, when)
import Data.IORef (IORef, newIORef, readIORef, writeIORef)
import Data.List (findIndex)
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
import NanoUI.Context (Context (..), getPrevRect, getScrollOffset2D, setScrollOffset2D)
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
  , vcPopupOpen :: !(IORef Bool)
  -- ^ Whether a drop-down or popup was on screen at the end of the last frame.
  }

newViewCache :: IO ViewCache
newViewCache = do
  vcVisible <- newIORef Nothing
  vcDisks <- newIORef Nothing
  vcListWid <- newIORef Nothing
  vcDialog <- newIORef Nothing
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
sizeColW = 104
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
  T.intercalate "  ·  " $
    [ if T.null (dsDrive stat) then "Unknown disk" else dsDrive stat
    , tshow (dsCount stat) <> (if dsCount stat == 1 then " app" else " apps")
    ]
      <> [formatSize (dsSize stat) | dsSize stat > 0]

appView :: Env -> ViewCache -> NanoUI ()
appView env cache = do
  ctx <- askContext
  inp <- askInput
  uiIO $ readIORef (ctxWakeLoop ctx) >>= mapM_ (writeIORef (envWake env))
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
  -- Escape is handled at the end of the frame, after every widget has run.
  editing <- uiIO (textFieldActive ctx)
  let keys = inputKeys inp
      ctrlA = modCtrl (inputModifiers inp) && T.any (`T.elem` "aA\x01") (inputChars inp)
  when (ctrlA && not editing && not anyModal) $ setSelection (Set.union visibleIds)
  when (inputKeysElem KeyDelete keys && not editing && not anyModal && not (null selectedPackages)) $
    setPending (Just (PendingBatch OpUninstall selectedPackages))

  -- List geometry from the previous frame, used by the header too.
  mWid <- uiIO (readIORef (vcListWid cache))
  mRect <- maybe (pure Nothing) (uiIO . getPrevRect ctx) mWid
  off <- maybe (pure (V2 0 0)) (uiIO . getScrollOffset2D ctx) mWid
  let n = V.length visible
      viewH = maybe 600 rectH mRect
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
      -- arithmetic on the list's rectangle and scroll offset.
      rowAt pos = case mRect of
        Just r
          | not anyModal && rectContains r pos ->
              let i = floor ((v2Y pos - rectY r + y) / rowH)
                  -- Content x: follows horizontal scroll in a narrow window.
                  localX = v2X pos - rectX r + v2X off
               in if i >= 0 && i < n && localX > checkW && localX < max (rectW r) rowMinW - actionW - 24
                    then Just (pkgId (visible V.! i))
                    else Nothing
        _ -> Nothing

  columnWith (fillW . fillH . gap 0 . tight) $ do
    panelStyledWith (palSurface pal) (palSurface pal) (fillW . tight) $
      columnWith (fillW . gap 0 . tight) $ do
        -- Title, search, filters, and list actions
        rowWith (fillW . padXY 16 12 . gap 10 . alignMid . tight) $ do
          labelWith (tight . fontSemiBold . fontSize 24 . fontColor (palText pal)) "Installed apps"
          spacer (Fixed 8) Fit
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
          whenM (button "Select by rule…") (setRulesOpen True)
          whenM (button "Refresh") (uiIO (refresh env (stHasUpdateInfo st)))
          whenM (button "Check updates") (uiIO (refresh env True))

        -- Status and selection actions
        rowWith (fillW . fixedH 48 . padXY 16 0 . gap 12 . alignMid . tight) $ do
          let total = V.length (stPackages st)
              updates = V.length (V.filter (not . T.null . pkgAvailable) (stPackages st))
              shownSize = V.sum (V.map pkgSize visible)
              counts
                | n == total = tshow total <> " apps"
                | otherwise = tshow n <> " of " <> tshow total <> " apps"
          labelWith (tight . fontMedium . fontColor (palText pal)) counts
          unless (shownSize == 0) $ labelWith (tight . fontColor (palTextMuted pal)) (formatSize shownSize)
          when (stHasUpdateInfo st && updates > 0) $ badge (palGreen pal) (tshow updates <> " updates")
          case (stLoading st, stError st) of
            (Just msg, _) -> labelWith (tight . fontColor (palAccent pal)) msg
            (_, Just err) -> labelWith (tight . fontColor (palRed pal)) (ellipsize 110 err)
            _ | total > 0 -> labelWith (tight . fontColor (palTextFaint pal)) ("listed in " <> tshow (round (stLoadSeconds st * 1000) :: Int) <> " ms")
            _ -> pure ()
          forM_ notice $ \msg -> do
            labelWith (tight . fontColor (palYellow pal)) (ellipsize 80 msg)
            whenM (buttonWith tight "Dismiss") (setNotice Nothing)
          flex
          unless (null selectedPackages) $ do
            labelWith (tight . fontSemiBold . fontColor (palAccent pal)) (tshow (length selectedPackages) <> " selected")
            whenM (buttonWith (fontColor (palRed pal)) "Uninstall selected") $
              setPending (Just (PendingBatch OpUninstall selectedPackages))
            let upgradable = filter (not . T.null . pkgAvailable) selectedPackages
            unless (null upgradable) $
              whenM (buttonWith (fontColor (palGreen pal)) ("Upgrade " <> tshow (length upgradable))) $
                setPending (Just (PendingBatch OpUpgrade upgradable))
            whenM (button "Clear") (setSelection (const Set.empty))
          unless (V.null visible) $ whenM (button "Select shown") (setSelection (Set.union visibleIds))
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
    forM_ mWid $ \w -> when (abs (y - v2Y off) > 0.5) $ uiIO (setScrollOffset2D ctx w (V2 (v2X off) y))
    (wid, actions) <- withKey ("package-list" :: Text) $ scrollArea2D (fillW . fillH) $
      columnWith (fillW . gap 0 . tight) $
        if n == 0
          then do
            rowWith (fillW . padXY 24 32 . tight) $
              labelWith (tight . fontColor (palTextMuted pal)) $
                case (stLoading st, V.null (stPackages st)) of
                  (Just msg, True) -> msg
                  (_, True) -> "No apps listed."
                  _ -> "No apps match the search or filters."
            pure []
          else do
            when (lo > 0) $ spacer Fit (Fixed (fromIntegral lo * rowH))
            acts <- forM [lo .. hi] $ \i ->
              let p = visible V.! i
               in withKey (pkgId p) (packageRow i (Set.member (pkgId p) selected) (Set.member (pkgId p) busy) p)
            when (hi < n - 1) $ spacer Fit (Fixed (fromIntegral (n - 1 - hi) * rowH))
            pure (concat acts)
    uiIO (writeIORef (vcListWid cache) (Just wid))

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
          tone = if upgrading then palGreen pal else palRed pal
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
      labelWith (tight . fontColor (palTextMuted pal)) $
        if upgrading
          then "These apps will be updated to their newest available versions."
          else "These apps will be removed from this PC."
      -- Only a long list scrolls (about six and a half rows show), so a short
      -- one has no scrollbar.
      panelStyledWith (palBackground pal) (palBorder pal) (fillW . tight) $
        if length pkgs > 6 then scrollWith (fillW . fixedH 244 . tight) appRows else appRows
      when (not upgrading && noUninstaller > 0) $
        panelStyledWith (lerpColor (palSurface pal) (palYellow pal) 0.10) (lerpColor (palSurface pal) (palYellow pal) 0.40) (fillW . tight) $
          rowWith (fillW . fixedH 44 . gap 0 . alignMid . tight) $ do
            inset
            labelWith (tight . alignMid . fontColor (palYellow pal)) $
              tshow noUninstaller <> (if noUninstaller == 1 then " app has" else " apps have") <> " no registered uninstall command and may fail."
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
      rowWith (fillW . gap 12 . alignMid . tight) $ do
        flex
        cancel <- button "Cancel"
        go <- filledButton tone (confirmTitle pb)
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
    columnWith (fillW . gap 4 . tight) $ do
      labelWith (tight . fontColor (palTextMuted pal)) "One rule per line: id:ID, name:NAME, publisher:NAME, disk:D:, match:TEXT, or a bare id or name."
      labelWith (tight . fontSize 15 . fontColor (palTextFaint pal)) "Lines starting with # are ignored. The same files work with winget-gui-cli --from."
    t <- textAreaWith (fillW . fixedH 240) rulesText
    when (t /= rulesText) (setRulesText t)
    separator
    rowWith (fillW . gap 12 . alignMid . tight) $ do
      load <- button "Load file…"
      save <- button "Save rules…"
      flex
      labelWith (tight . alignMid . fontColor (if V.null matches then palTextMuted pal else palText pal)) (tshow (V.length matches) <> " installed apps match")
      spacer (Fixed 8) Fit
      add <- button "Add to selection"
      replace <- filledButton (palAccent pal) "Select matches"
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

  -- Escape closes a dialog, or else clears the selection. If a drop-down or
  -- popup was on screen this frame or the last, the Escape was for it and the
  -- selection stays. This runs after every widget, when this frame's popups exist.
  popupWasOpen <- uiIO (readIORef (vcPopupOpen cache))
  popupOpen <- uiIO (floatingPanelActive ctx)
  -- A select's drop-down is not a floating panel: its open flag lives in the
  -- store, and is still set here because input is resolved after the view.
  selectOpen <- uiIO (anySelectOpen <$> getStore ctx)
  uiIO (writeIORef (vcPopupOpen cache) popupOpen)
  when (inputKeysElem KeyEscape keys) $
    if anyModal
      then setPending Nothing >> setDetails Nothing >> setRulesOpen False
      else unless (editing || selectOpen || popupWasOpen || popupOpen) (setSelection (const Set.empty))

packageRow :: Int -> Bool -> Bool -> Package -> NanoUI [RowAction]
packageRow ix isSelected isBusy p =
  panelStyledWith background background (fillW . fixedH rowH . tight) $
    rowWith (fillW . fixedH rowH . gap 10 . alignMid . padXY 14 0 . tight) $ do
      toggled <- rowWith (alignMid . tight) (checkbox "" isSelected)
      columnWith (grow . minW 220 . gap 3 . alignMid . nameBlockNudge . cellPad . tight) $ do
        labelWith (tight . fontMedium . fontColor (palText pal)) (ellipsize 64 (pkgName p))
        labelWith (tight . fontSize 14 . fontColor (palTextFaint pal)) (ellipsize 72 (pkgId p))
      rowWith (fixedW publisherW . alignMid . cellPad . tight) $
        labelWith (tight . fontColor (palTextMuted pal)) (ellipsize 24 (pkgPublisher p))
      columnWith (fixedW versionW . gap 3 . alignMid . cellPad . tight) $ do
        labelWith (tight . fontSize 15 . fontColor (palTextMuted pal)) (ellipsize 18 version)
        unless (T.null (pkgAvailable p)) $
          labelWith (tight . fontSize 14 . fontColor (palGreen pal)) ("↑ " <> ellipsize 16 (pkgAvailable p))
      rowWith (fixedW sizeColW . alignMid . cellPadEnd . tight) $
        labelWith (fillW . alignEnd . tight . fontColor (palText pal)) (formatSize (pkgSize p))
      rowWith (fixedW dateW . alignMid . cellPad . tight) $
        labelWith (tight . fontColor (palTextMuted pal)) (pkgDate p)
      acts <- rowWith (fixedW actionW . gap 6 . alignMid . alignEnd . tight) $
        if isBusy
          then labelWith (tight . fontColor (palYellow pal)) "Queued…" >> pure []
          else do
            up <- if T.null (pkgAvailable p) then pure False else buttonWith (tight . fontColor (palGreen pal)) "Upgrade"
            un <- buttonWith tight "Uninstall"
            pure ([RowUpgrade p | up] <> [RowUninstall p | un])
      pure ([RowToggle ix toggled | toggled /= isSelected] <> acts)
  where
    pal = palette
    background
      | isSelected = palAccentSoft pal
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
        panelStyledWith (palBackground pal) (palBorder pal) (fillW . tight) $
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
      selectableTextWith (tight . alignMid . fontMono . fontSize 15 . fontColor (palText pal)) (orDash (pkgLocation p))
      inset
  section "Uninstall" $ do
    field "Support" $ case pkgUninstall p of
      HasSilentUninstall -> "Silent uninstall supported"
      HasUninstall -> "Uninstaller registered"
      NoUninstall -> "No uninstall command registered"
    field "Product codes" (joined (pkgProductCodes p))
    field "Package family" (joined (pkgFamilies p))
  separator
  rowWith (fillW . gap 12 . alignMid . tight) $ do
    copy <- button "Copy id"
    open <- if T.null (pkgLocation p) then pure False else button "Open location"
    flex
    up <- if T.null (pkgAvailable p) then pure False else filledButton (palGreen pal) "Upgrade"
    un <- filledButton (palRed pal) "Uninstall"
    pure $ case () of
      _
        | copy -> Just DetailCopyId
        | open -> Just DetailOpenLocation
        | up -> Just DetailUpgrade
        | un -> Just DetailUninstall
        | otherwise -> Nothing

queuePanel :: Env -> AppState -> NanoUI ()
queuePanel env st =
  panelStyledWith (palSurface pal) (palSurface pal) (fillW . fixedH 260 . tight) $
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
      rowWith (fillW . gap 10 . alignMid . tight) $ do
        labelWith (tight . fontSemiBold . fontColor (palText pal)) "Queue"
        labelWith (tight . fontColor (palTextMuted pal)) (tshow finished <> " of " <> tshow total <> " finished")
        when (failed > 0) $ badge (palRed pal) (tshow failed <> " failed")
        flex
        when (stActiveBatches st > 0) $ whenM (button "Stop all") (uiIO (stopBatches env))
        when (failed > 0) $ whenM (button "Retry failed") (uiIO (retryFailed env))
        when (finished > 0) $ whenM (button "Clear finished") (uiIO (clearFinished env))
      progressBarWith fillW 6 (realToFrac overall)
      scrollWith (fillW . fillH) $ columnWith (fillW . gap 2 . tight) $
        V.forM_ jobs $ \jv -> withKey (jobToken (jvJob jv)) $
          rowWith (fillW . fixedH 34 . gap 10 . alignMid . tight) $ do
            let job = jvJob jv
            labelWith (tight . fixedW 90 . fontColor (palTextFaint pal)) (if jobKind job == OpUpgrade then "Upgrade" else "Uninstall")
            labelWith (tight . fixedW 330 . fontColor (palText pal)) (ellipsize 36 (pkgName (jobPackage job)))
            -- Status grows; the action button keeps its own column so Skip and
            -- Cancel line up across rows.
            rowWith (grow . gap 10 . alignMid . tight) $ case jvStatus jv of
              JobPending ->
                labelWith (tight . fontColor (palTextMuted pal)) (if Set.member (jobToken job) (stSkipped st) then "Skipping" else "Waiting")
              JobRunning state frac -> do
                progressBarWith (fixedW 180) 6 (realToFrac frac)
                labelWith (tight . fontColor (palTextMuted pal)) $ case state of
                  StateQueued -> "Starting"
                  StateRunning -> tshow (round (frac * 100) :: Int) <> "%"
                  StatePost -> "Finishing"
                  StateFinished -> "Done"
              JobSucceeded msg -> labelWith (tight . fontColor (palGreen pal)) msg
              JobFailed msg -> labelWith (tight . fontColor (palRed pal)) (ellipsize 120 msg)
              JobCancelled -> labelWith (tight . fontColor (palTextMuted pal)) "Cancelled"
            rowWith (fixedW 96 . alignMid . tight) $ case jvStatus jv of
              JobPending -> whenM (buttonWith tight "Skip") (uiIO (cancelJob env job))
              JobRunning {} -> whenM (buttonWith tight "Cancel") (uiIO (cancelJob env job))
              _ -> pure ()
  where
    pal = palette
    isFailed = \case
      JobFailed _ -> True
      _ -> False

-- | Layout for a dialog's body. A dialog's scroll viewport is slightly
-- narrower than its content, so keep a right margin clear of the clipped edge.
dialogBody :: Layout -> Layout
dialogBody l = l {layoutPadding = Padding 0 16 0 0}
