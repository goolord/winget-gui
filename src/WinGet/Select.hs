-- | Choosing packages by rule, for automated bulk operations.
--
-- A selection file has one selector per line. Blank lines and lines starting
-- with @#@ are ignored.
--
-- > id:Mozilla.Firefox        exact WinGet id
-- > name:Google Chrome        exact display name
-- > publisher:Contoso Ltd.    exact publisher
-- > disk:D:                   installed on drive D: (also disk:Network, disk:Unknown)
-- > match:toolbar             substring of name, id, or publisher
-- > Mozilla.Firefox           bare text: exact id or exact name
--
-- Every comparison ignores case. A package matches a list of selectors when
-- it matches any one of them.
module WinGet.Select
  ( Selector (..)
  , parseSelector
  , parseSelectors
  , renderSelector
  , matchSelector
  , selectPackages
  , normalizeDrive
  )
where

import Data.Text (Text)
import Data.Text qualified as T
import Data.Vector (Vector)
import Data.Vector qualified as V
import WinGet.Bridge (Package (..))

data Selector
  = ById !Text
  | ByName !Text
  | ByPublisher !Text
  | ByDisk !Text
  | Contains !Text
  | IdOrName !Text
  deriving (Eq, Show)

parseSelector :: Text -> Maybe Selector
parseSelector raw
  | T.null line || "#" `T.isPrefixOf` line = Nothing
  | Just v <- field "id:" = ById <$> nonEmpty v
  | Just v <- field "name:" = ByName <$> nonEmpty v
  | Just v <- field "publisher:" = ByPublisher <$> nonEmpty v
  | Just v <- field "disk:" = ByDisk . normalizeDrive <$> nonEmpty v
  | Just v <- field "match:" = Contains <$> nonEmpty v
  | otherwise = Just (IdOrName line)
  where
    line = T.strip raw
    field prefix
      | T.toLower prefix `T.isPrefixOf` T.toLower line = Just (T.strip (T.drop (T.length prefix) line))
      | otherwise = Nothing
    nonEmpty v = if T.null v then Nothing else Just v

parseSelectors :: Text -> [Selector]
parseSelectors = foldr (\l acc -> maybe acc (: acc) (parseSelector l)) [] . T.lines

renderSelector :: Selector -> Text
renderSelector = \case
  ById v -> "id:" <> v
  ByName v -> "name:" <> v
  ByPublisher v -> "publisher:" <> v
  ByDisk v -> "disk:" <> v
  Contains v -> "match:" <> v
  IdOrName v -> v

-- | Canonical drive text as the bridge reports it: @d@, @d:@ and @D:\\@ all
-- become @D:@; @network@ becomes @Network@; empty or @unknown@ becomes empty.
normalizeDrive :: Text -> Text
normalizeDrive raw = case T.unpack (T.strip raw) of
  [c] | c `elem` letters -> T.singleton (toUpperAscii c) <> ":"
  c : ':' : _ | c `elem` letters -> T.singleton (toUpperAscii c) <> ":"
  s
    | T.toCaseFold (T.pack s) == "network" -> "Network"
    | T.toCaseFold (T.pack s) `elem` ["", "unknown"] -> ""
    | otherwise -> T.pack s
  where
    letters = ['a' .. 'z'] <> ['A' .. 'Z']
    toUpperAscii c = if c >= 'a' && c <= 'z' then toEnum (fromEnum c - 32) else c

matchSelector :: Selector -> Package -> Bool
matchSelector sel p = case sel of
  ById v -> same v (pkgId p)
  ByName v -> same v (pkgName p)
  ByPublisher v -> same v (pkgPublisher p)
  ByDisk v -> normalizeDrive v == pkgDrive p
  Contains v -> T.toCaseFold v `T.isInfixOf` pkgHaystack p
  IdOrName v -> same v (pkgId p) || same v (pkgName p)
  where
    same a b = T.toCaseFold a == T.toCaseFold b

-- | Packages matched by any selector, in their original order.
selectPackages :: [Selector] -> Vector Package -> Vector Package
selectPackages sels = V.filter (\p -> any (`matchSelector` p) sels)
