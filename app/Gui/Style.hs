-- | The app's palette and nano-ui theme, the button variants and surfaces the
-- view is built from, a few icons, and the widgets nano-ui has no style for:
-- flat column headers and storage bars.
module Gui.Style
  ( Palette (..)
  , palette
  , appTheme
    -- * Buttons
  , quiet
  , tintedFill
  , actionButton
  , compactButton
  , rowButton
    -- * Surfaces
  , surface
    -- * Icons
  , icon
  , iconCheck
  , iconCross
  , iconAlert
  , iconBox
    -- * Custom widgets
  , HeaderAlign (..)
  , headerCell
  , sizeCell
    -- * Layout
  , cellInset
  , cellPad
  , cellPadEnd
  , padLR
  )
where

import Control.Monad (unless, void, when)
import Data.Char (ord)
import Data.Maybe (isJust)
import Data.Text (Text)
import Data.Text qualified as T
import NanoUI
import NanoUI.Testing (UiCursorKind (..))

data Palette = Palette
  { palBackground :: !Color
  -- ^ Window and list background.
  , palStripe :: !Color
  -- ^ Every other list row.
  , palRowHover :: !Color
  -- ^ The row under the pointer.
  , palSurface :: !Color
  -- ^ Raised chrome: top bar, header, queue, dialogs.
  , palElevated :: !Color
  -- ^ Buttons.
  , palHover :: !Color
  , palPressed :: !Color
  , palBorder :: !Color
  , palSeparator :: !Color
  , palTrack :: !Color
  -- ^ The empty part of a storage bar.
  , palText :: !Color
  , palTextMuted :: !Color
  , palTextFaint :: !Color
  , palOnAccent :: !Color
  -- ^ Labels on accent, green and red fills.
  , palAccent :: !Color
  , palAccentSoft :: !Color
  -- ^ Selected rows.
  , palGreen :: !Color
  , palRed :: !Color
  , palYellow :: !Color
  , palOrange :: !Color
  }

-- | Graphite greys with a mineral teal. The greys are neutral, with no blue
-- tint. Three surface levels, each a clear step apart, say what a region is:
-- the well holds content (the list, inputs, inset boxes), the shelf holds
-- chrome (the top bar, the queue, dialogs), and controls sit a step above
-- both with visible edges. Colour marks consequence only: teal for what is
-- selected and where storage goes, red for removing, green for updating.
palette :: Palette
palette =
  Palette
    { palBackground = rgb 25 26 25 -- well
    , palStripe = rgb 31 32 31
    , palRowHover = rgb 40 41 39
    , palSurface = rgb 47 48 46 -- shelf
    , palElevated = rgb 66 67 64 -- controls
    , palHover = rgb 80 81 78
    , palPressed = rgb 54 55 52
    , palBorder = rgb 94 95 91
    , palSeparator = rgb 72 73 70 -- rule
    , palTrack = rgb 52 53 50
    , palText = rgb 240 239 234 -- chalk
    , palTextMuted = rgb 198 197 191
    , palTextFaint = rgb 162 162 156
    , palOnAccent = rgb 14 18 17
    , palAccent = rgb 104 194 180 -- mineral teal
    , palAccentSoft = rgb 36 72 66
    , palGreen = rgb 164 208 122
    , palRed = rgb 238 118 108
    , palYellow = rgb 232 196 112
    , palOrange = rgb 236 152 92
    }
  where
    rgb r g b = colorRGBA r g b 255

appTheme :: Theme
appTheme =
  Theme
    { themeWindow = palBackground p
    , themePanel = style (palSurface p) (palBorder p) (palHover p) (palPressed p) 8
    , -- Dialogs and drop-downs use the panel surface too: nano-ui fills padded
      -- containers with the panel colour, which would otherwise show as a box
      -- inside the dialog.
      themeFloatingWindow = style (palSurface p) (palBorder p) (palSurface p) (palSurface p) 12
    , themeButton = style (palElevated p) (palBorder p) (palHover p) (palPressed p) 6
    , themeInput = style (palBackground p) (palBorder p) (palStripe p) (palBackground p) 6
    , themeSeparator = palSeparator p
    , themeAccent = palAccent p
    , themeMuted = palTextMuted p
    , themeRed = palRed p
    , themeOrange = palOrange p
    , themeYellow = palYellow p
    , themeGreen = palGreen p
    , -- The app draws nothing in purple; keep nano-ui's purple slot neutral.
      themePurple = palTextMuted p
    , themeOverlayDim = colorRGBA 0 0 0 160
    , themeOnAccent = palOnAccent p
    , themeSelection = let c = palAccent p in colorRGBA (colorR c) (colorG c) (colorB c) 84
    , themeFocusRing = palAccent p
    , themeLink = lerpColor (palAccent p) (palText p) 0.3
    , themeShadow = colorRGBA 0 0 0 110
    , themeDisabledFade = 0.55
    }
  where
    p = palette
    style bg border hover active radius =
      Style
        { styleBg = bg
        , styleFg = palText p
        , styleBorder = border
        , styleBorderWidth = 1
        , styleCornerRadius = radius
        , styleHoverBg = hover
        , styleActiveBg = active
        }

-- | Buttons with no fill or border until hovered, in the muted text colour:
-- secondary actions that repeat on every row, and the ones beside a filled
-- button.
quiet :: Theme -> Theme
quiet = subtle . buttonStyle (foreground (palTextMuted palette))

-- | A button washed with @c@ over the usual button fill, labelled and edged in
-- @c@: a coloured action that stays quieter than a filled 'success' button.
tintedFill :: Color -> Style -> Style
tintedFill c =
  -- Over the darker pressed tone, so the label keeps 4.5:1 on the wash.
  fillColor (lerpColor (palPressed palette) c 0.16)
    . foreground c
    . borderColor (lerpColor (palPressed palette) c 0.45)

-- | A button with room around its label, as tall as the search field and the
-- drop-downs beside it. nano-ui sizes a button from its text alone, so the
-- size is set here.
actionButton :: Text -> NanoUI Bool
actionButton = sizedButton 32 34

-- | A row's button: shorter, so it sits inside a list row.
compactButton :: Text -> NanoUI Bool
compactButton = sizedButton 24 30

sizedButton :: Float -> Float -> Text -> NanoUI Bool
sizedButton sidePad h txt = do
  fm <- uiFontMetrics
  buttonWith (fixedWH (fromIntegral (ceiling (lineWidth fm txt + sidePad) :: Int)) h . alignMid) txt

-- | A list row's action button, in the button style of the enclosing theme
-- scope (so 'tintedFill' and friends apply), with its label centred on the
-- ink. nano-ui centres a button label from the pen to the last glyph's ink
-- and on the capital height; Segoe UI's left bearings and small x-height then
-- set lowercase labels visibly right of and below centre. Drawn, so it takes
-- no keyboard focus: a row's actions are also in its details dialog, and
-- Delete uninstalls the selection.
rowButton :: Text -> NanoUI Bool
rowButton txt = do
  fm <- uiFontMetrics
  theme <- uiTheme
  let s = themeButton theme
      (inkStart, inkEnd) = inkSpan fm txt
  (resp, ()) <-
    customWidget
      defaultCustomWidgetSpec
        { widgetLayout = (fixedWH (fromIntegral (ceiling (inkEnd - inkStart + 28) :: Int)) 30 . alignMid) defaultLayout
        , widgetContent =
            drawKey txt [styleBg s, styleFg s, styleBorder s, styleHoverBg s, styleActiveBg s] [styleCornerRadius s, styleBorderWidth s]
        , widgetDraw = \dc r -> runCanvas $ do
            let f = cdcFont dc
                st = themeButton (cdcTheme dc)
                fill
                  | cdcPressed dc = styleActiveBg st
                  | cdcHovered dc = styleHoverBg st
                  | otherwise = styleBg st
                bw = styleBorderWidth st
                radius = styleCornerRadius st
                snap v = let k = max 1 (fmSnapScale f) in fromIntegral (round (v * k) :: Int) / k
                (i0, i1) = inkSpan f txt
                -- Optical middle: halfway between the capital and x-height
                -- middles, measured from the top of the line box.
                (capTop, xTop, baseline) = verticalInk f
                inkMid = ((capTop + baseline) / 2 + (xTop + baseline) / 2) / 2
                penX = snap (rectX r + (rectW r - (i1 - i0)) / 2 - i0)
                lineTop = snap (rectY r + rectH r / 2 - inkMid)
            when (bw > 0) $ drawRoundedRect r radius (styleBorder st)
            drawRoundedRect (Rect (rectX r + bw) (rectY r + bw) (rectW r - 2 * bw) (rectH r - 2 * bw)) (max 0 (radius - bw)) fill
            drawText (V2 penX lineTop) AlignStart AlignTop txt (styleFg st)
        , widgetCursor = Just (const UiCursorPointer)
        }
  pure (respClicked resp)

-- | Where a line's ink starts and ends, from the pen.
inkSpan :: FontMetrics -> Text -> (Float, Float)
inkSpan fm txt =
  let start = maybe 0 (\(c, _) -> maybe 0 gqX (fmGlyph fm c)) (T.uncons txt)
      end = maybe (lineWidth fm txt) stInkEnd (fmShape fm txt)
   in (start, end)

-- | The top of a capital, the top of an x-height letter, and the baseline,
-- from the top of the line box.
verticalInk :: FontMetrics -> (Float, Float, Float)
verticalInk fm =
  case (fmGlyph fm 'H', fmGlyph fm 'x') of
    (Just h, Just x) -> (gqY h, gqY x, gqY h + gqH h)
    _ -> let a = fmAscent fm in (a * 0.3, a * 0.45, a)

-- | A panel in its own colours. Full-width chrome passes a border the same
-- colour as the fill and no radius.
surface :: Color -> Color -> Float -> (Layout -> Layout) -> NanoUI a -> NanoUI a
surface bg border radius f =
  styled (panelStyle (background bg . borderColor border . cornerRadius radius)) . panelWith f

-- | A one-colour icon, @size@ px square, centred in its row.
icon :: Float -> Color -> Svg -> NanoUI ()
icon size color = svgIconWith (fixedWH size size . alignMid . fontColor color)

-- Stroked on a 24px grid in currentColor, so one raster serves every tint.
svg :: Text -> Svg
svg body =
  either (error . ("Gui.Style: bad icon: " <>)) id . parseSvg $
    "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\">"
      <> body
      <> "</svg>"

iconCheck, iconCross, iconAlert, iconBox :: Svg
iconCheck = svg "<path d=\"M5 12.5l4.5 4.5L19 7.5\" stroke-width=\"2.5\"/>"
iconCross = svg "<path d=\"M7 7l10 10M17 7L7 17\" stroke-width=\"2.5\"/>"
iconAlert =
  svg
    "<path d=\"M10.3 4.2L2.6 17.6A2 2 0 0 0 4.3 20.6h15.4a2 2 0 0 0 1.7-3L13.7 4.2a2 2 0 0 0-3.4 0z\"/>\
    \<path d=\"M12 9.5v4.5\"/>\
    \<circle cx=\"12\" cy=\"17.2\" r=\"1.2\" fill=\"currentColor\" stroke=\"none\"/>"
iconBox =
  svg
    "<path d=\"M12 2.8l8.5 4.7v9L12 21.2l-8.5-4.7v-9z\"/>\
    \<path d=\"M3.5 7.5L12 12.2l8.5-4.7\"/>\
    \<path d=\"M12 12.2v9\"/>"

-- | Horizontal inset of text inside list cells. Header titles use the same
-- inset, so each title starts exactly where its column's values do.
cellInset :: Float
cellInset = 8

padLR :: Float -> Float -> Layout -> Layout
padLR l r layout = layout {layoutPadding = (layoutPadding layout) {padL = l, padR = r}}

-- | Padding that puts a left-aligned cell's content at 'cellInset'.
cellPad :: Layout -> Layout
cellPad layout = layout {layoutPadding = (layoutPadding layout) {padL = cellInset}}

-- | Padding that puts a right-aligned cell's content 'cellInset' from its end.
cellPadEnd :: Layout -> Layout
cellPadEnd layout = layout {layoutPadding = (layoutPadding layout) {padR = cellInset}}

-- | A 'widgetContent' key for a drawing that reads text and colours as well as
-- numbers. nano-ui skips both the rebuild and the repaint of a custom widget
-- whose key, size and interaction state are unchanged, so a key must cover
-- everything its drawing reads: a stale key draws stale pixels. nano-ui's
-- 'contentKey' hashes the numbers; 'mix64' folds in the colours and the text.
drawKey :: Text -> [Color] -> [Float] -> Int
drawKey txt colors numbers =
  let seed = fromIntegral (contentKey numbers)
      withColors = foldl' (\acc c -> mix64 acc (fromIntegral (colorToWord32 c))) seed colors
      key = fromIntegral (T.foldl' (\acc ch -> mix64 acc (fromIntegral (ord ch))) withColors txt)
   in if key == 0 then 1 else key

data HeaderAlign = HeaderStart | HeaderEnd
  deriving (Eq)

-- | A column title sized like its column. A sortable title highlights on
-- hover, shows a pointer, and is drawn in the accent colour with an arrow
-- while it is the sort column (@Just descending@). Returns whether it was
-- clicked.
headerCell :: (Layout -> Layout) -> HeaderAlign -> Text -> Maybe Bool -> Bool -> NanoUI Bool
headerCell sizing align title sortState sortable = do
  -- Everything the drawing reads besides hover and press state.
  let sortNum = case sortState of {Just True -> 2; Just False -> 1; Nothing -> 0}
      flags = [sortNum, if sortable then 1 else 0, if align == HeaderEnd then 1 else 0]
  (resp, ()) <-
    customWidget
      defaultCustomWidgetSpec
        { widgetLayout = (sizing . fixedH 34 . alignMid) defaultLayout
        , widgetContent = drawKey title [] flags
        , widgetDraw = \dc r -> runCanvas $ do
            let hovered = sortable && cdcHovered dc
                pressed = sortable && cdcPressed dc
                caption = title <> maybe "" (\desc -> if desc then "  ↓" else "  ↑") sortState
                fg
                  | isJust sortState = palAccent palette
                  | hovered = palText palette
                  | otherwise = palTextMuted palette
                y = rectY r + rectH r / 2
            when (hovered || pressed) $
              drawRoundedRect r 6 (if pressed then palPressed palette else palElevated palette)
            case align of
              HeaderStart -> drawText (V2 (rectX r + cellInset) y) AlignStart AlignMiddle caption fg
              HeaderEnd -> drawText (V2 (rectX r + rectW r - cellInset) y) AlignEnd AlignMiddle caption fg
        , widgetCursor = if sortable then Just (const UiCursorPointer) else Nothing
        }
  pure (sortable && respClicked resp)

-- | An app's size over a thin bar of its share of the largest app shown, the
-- way Explorer draws a drive's used space: scanning the column shows where the
-- disk went. Right-aligned like the "Size" title, and blank for an unknown size.
sizeCell :: Float -> Text -> Float -> NanoUI ()
sizeCell width txt share =
  void $
    customWidget
      defaultCustomWidgetSpec
        { widgetLayout = (fixedW width . fillH . alignMid) defaultLayout
        , widgetContent = drawKey txt [] [share]
        , widgetDraw = \_ r -> runCanvas $
            unless (T.null txt) $ do
              let x0 = rectX r + cellInset
                  x1 = rectX r + rectW r - cellInset
                  mid = rectY r + rectH r / 2
                  barY = fromIntegral (round (mid + 12) :: Int)
                  -- A sliver for the smallest apps, so every known size shows.
                  filled = max 3 (min 1 share * (x1 - x0))
              drawText (V2 x1 (mid - 5)) AlignEnd AlignMiddle txt (palText palette)
              drawRoundedRect (Rect x0 barY (x1 - x0) 4) 2 (palTrack palette)
              drawRoundedRect (Rect (x1 - filled) barY filled 4) 2 (palAccent palette)
        }

