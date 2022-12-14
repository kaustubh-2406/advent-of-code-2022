import Prelude hiding (Left, Right)
import Data.List

data Position = Position Int Int deriving(Show, Eq)
data Move = Up | Down | Left | Right deriving(Show, Eq)

data State = State { 
    rope  :: [Position],
    trail :: [Position] 
  } deriving(Show)

initialState :: State
initialState = State {
  rope  = [Position 0 0, Position 0 0],
  trail = []
}

moveHead :: Position -> Move -> Position
moveHead (Position x y) Up     = Position x (y - 1)
moveHead (Position x y) Down   = Position x (y + 1)
moveHead (Position x y) Left   = Position (x - 1) y
moveHead (Position x y) Right  = Position (x + 1) y

-- TailPos -> HeadPos -> TailPos
moveTail :: Position -> Position -> Position
moveTail (Position tx ty) (Position hx hy)
  -- if head is right next to tail.. no need to move it :)
  | abs (tx - hx) < 2 && abs (ty - hy) < 2 = Position tx ty
  -- diagonal
  | tx /= hx && ty /= hy = Position (tx + signx) (ty + signy) 
  -- vertical
  | tx == hx && ty /= hy = Position tx (ty + signy)      
  -- horizontal
  | otherwise            = Position (tx + signx) ty                
  where dx = hx - tx
        dy = hy - ty
        signx = dx `div` abs dx
        signy = dy `div` abs dy

-- need to handle move head and move tail toooo...
move :: State -> Move -> State
move state m = newstate 
  where 
    (headpos:tailpos:[]) = rope state
    newhead = moveHead headpos m
    newtail = moveTail tailpos newhead
    newtrail = trail state ++ [newtail]
    newstate = State { rope = [newhead, newtail], trail = newtrail }

parseMoves :: [Move] -> String -> [Move]
parseMoves moves xs 
  | dir == "U" = moves <> (map (\_ -> Up) [1..amount])
  | dir == "D" = moves <> (map (\_ -> Down) [1..amount])
  | dir == "L" = moves <> (map (\_ -> Left) [1..amount])
  | dir == "R" = moves <> (map (\_ -> Right) [1..amount])
  where
    dir:amountStr:[] = words xs
    amount = read amountStr :: Int

main :: IO()
main = interact $ show . length . nub . trail . foldl move initialState . foldl parseMoves [] . lines
