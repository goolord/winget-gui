-- | The app's palette and nano-ui theme, plus the few widgets nano-ui has no
-- style for: flat column headers, pill badges, and filled action buttons.
module Gui.Style
  ( Palette (..)
  , palette
  , appTheme
  , HeaderAlign (..)
  , headerCell
  , badge
  , filledButton
  , cellInset
  , cellPad
  , cellPadEnd
  , padLR
  )
where

import Control.Monad (void, when)
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
  , palSurface :: !Color
  -- ^ Raised chrome: top bar, header, queue, dialogs.
  , palElevated :: !Color
  -- ^ Buttons.
  , palHover :: !Color
  , palPressed :: !Color
  , palBorder :: !Color
  , palSeparator :: !Color
  , palText :: !Color
  , palTextMuted :: !Color
  , palTextFaint :: !Color
  , palAccent :: !Color
  , palAccentSoft :: !Color
  -- ^ Selected rows.
  , palGreen :: !Color
  , palRed :: !Color
  , palYellow :: !Color
  , palOrange :: !Color
  }

-- | Neutral charcoal greys with a teal accent. The greys carry no blue tint,
-- and updates use a yellow-green so they read apart from the accent.
palette :: Palette
palette =
  Palette
    { palBackground = rgb 17 18 19
    , palStripe = rgb 21 22 23
    , palSurface = rgb 25 26 28
    , palElevated = rgb 35 37 39
    , palHover = rgb 46 48 51
    , palPressed = rgb 29 30 32
    , palBorder = rgb 54 56 60
    , palSeparator = rgb 38 40 43
    , palText = rgb 240 240 237
    , palTextMuted = rgb 180 182 184
    , palTextFaint = rgb 142 145 149
    , palAccent = rgb 56 196 178
    , palAccentSoft = rgb 22 52 50
    , palGreen = rgb 150 210 96
    , palRed = rgb 240 110 100
    , palYellow = rgb 236 196 106
    , palOrange = rgb 240 152 86
    }
  where
    rgb r g b = colorRGBA r g b 255

appTheme :: Theme
appTheme =
  Theme
    { themeWindow = palBackground p
    , themePanel = style (palSurface p) (palBorder p) (palHover p) (palPressed p) 4
    , -- Dialogs and drop-downs use the panel surface too: nano-ui fills padded
      -- containers with the panel colour, which would otherwise show as a box
      -- inside the dialog.
      themeFloatingWindow = style (palSurface p) (palBorder p) (palSurface p) (palSurface p) 12
    , themeButton = style (palElevated p) (palBorder p) (palHover p) (palPressed p) 7
    , themeInput = style (palBackground p) (palBorder p) (palStripe p) (palBackground p) 7
    , themeSeparator = palSeparator p
    , themeAccent = palAccent p
    , themeMuted = palTextMuted p
    , themeRed = palRed p
    , themeOrange = palOrange p
    , themeYellow = palYellow p
    , themeGreen = palGreen p
    , -- The app draws nothing in purple; keep nano-ui's purple slot neutral.
      themePurple = palTextMuted p
    , themeOverlayDim = colorRGBA 0 0 0 170
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
              drawRoundedRect r 6 (if pressed then palPressed palette else palHover palette)
            case align of
              HeaderStart -> drawText (V2 (rectX r + cellInset) y) AlignStart AlignMiddle caption fg
              HeaderEnd -> drawText (V2 (rectX r + rectW r - cellInset) y) AlignEnd AlignMiddle caption fg
        , widgetCursor = if sortable then Just (const UiCursorPointer) else Nothing
        }
  pure (sortable && respClicked resp)

-- | A rounded, tinted label such as "106 updates".
badge :: Color -> Text -> NanoUI ()
badge color txt =
  void $
    customWidget
      defaultCustomWidgetSpec
        { widgetLayout = fixedH 28 defaultLayout
        , widgetContent = drawKey txt [color] []
        , widgetMeasure = Just (\fm _ -> (lineWidth fm txt + 22, 28))
        , widgetDraw = \_ r -> runCanvas $ do
            drawRoundedRect r 14 (lerpColor (palSurface palette) color 0.16)
            drawText (V2 (rectX r + rectW r / 2) (rectY r + rectH r / 2)) AlignCenter AlignMiddle txt color
        }

-- | A dialog's main action: filled with @color@, dark text, lighter on hover
-- and darker while pressed. As tall as a nano-ui button (1.3 line heights;
-- nano-ui's button padding is a total, not per side) so it lines up with the
-- buttons beside it, with a little more room on the sides.
filledButton :: Color -> Text -> NanoUI Bool
filledButton color txt = do
  (resp, ()) <-
    customWidget
      defaultCustomWidgetSpec
        { widgetLayout = alignMid defaultLayout
        , widgetContent = drawKey txt [color] []
        , widgetMeasure = Just (\fm _ -> (lineWidth fm txt + 4 * fmAdvance fm ' ', fmLineHeight fm * 1.3))
        , widgetDraw = \dc r -> runCanvas $ do
            let fill
                  | cdcPressed dc = lerpColor color (palBackground palette) 0.25
                  | cdcHovered dc = lerpColor color (palText palette) 0.18
                  | otherwise = color
            drawRoundedRect r 7 fill
            drawText (V2 (rectX r + rectW r / 2) (rectY r + rectH r / 2)) AlignCenter AlignMiddle txt (palBackground palette)
        , widgetCursor = Just (const UiCursorPointer)
        }
  pure (respClicked resp)
