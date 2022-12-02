import Data.List
import Data.Maybe

data Result a = Win a | Lose a | Tie a

opponent = ["A","B","C"]
mine     = ["X","Y","Z"]

get_point :: Result Int -> Int 
get_point (Win  x) = x + 6
get_point (Tie  x) = x + 3
get_point (Lose x) = x

check_state :: [String] -> Result Int
check_state [x, y]
  | i == 0 && j == 1 = Win  p
  | i == 1 && j == 2 = Win  p
  | i == 2 && j == 0 = Win  p
  | i == j           = Tie  p
  | otherwise        = Lose p
  where
    i = fromJust $ elemIndex x opponent
    j = fromJust $ elemIndex y mine
    p = j + 1

main = interact $ show . sum . map (get_point . check_state . words) . lines 

-- main :: IO ()
-- main = interact $  $ words