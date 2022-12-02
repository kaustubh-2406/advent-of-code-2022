import Data.List
import Data.Maybe

data Result a = Win a | Lose a | Tie a

opponent = ["A","B","C"]
strategy   = ["X", "Y", "Z"]

get_point :: Result Int -> Int 
get_point (Win  x) = x + 6
get_point (Tie  x) = x + 3
get_point (Lose x) = x

-- First part: The second column is considered to be my stategy to win..
check_state :: [String] -> Result Int
check_state [x, y]
  | i == 0 && j == 1 = Win  p
  | i == 1 && j == 2 = Win  p
  | i == 2 && j == 0 = Win  p
  | i == j           = Tie  p
  | otherwise        = Lose p
  where
    i = fromJust $ elemIndex x opponent
    j = fromJust $ elemIndex y strategy
    p = j + 1

-- Second part: suggests the stategy and we need to play as per strategy.
-- X = Lose
-- Y = Tie
-- Z = Win
part2 :: [String] -> Result Int
part2 [x, y]
  | y == "X" = Lose (l + 1)
  | y == "Y" = Tie  (i + 1)
  | y == "Z" = Win  (w + 1)
  where 
    i = fromJust $ elemIndex x opponent
    w = (i + 1) `mod` 3
    l = (i + 3 - 1) `mod` 3
    t = i


main = interact $ show . sum . map (get_point . part2 . words) . lines 
