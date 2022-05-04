module Main where

import System.IO
import Data.Char

main :: IO ()
main =
  -- hSetBuffering stdin LineBuffering
  loop (Just [])
  >> return ()

data RailTerm = RailInt Integer
              | RailOp (Integer -> Integer -> Integer)
              | RailCmd String

railOps = [("+", (+)), ("-", (-)), ("*", (*)), ("/", div)]

parseTerm :: String -> RailTerm
parseTerm s
  | all isDigit s = RailInt (read s)
  | otherwise = case lookup s railOps of
    Just op -> RailOp op
    Nothing -> RailCmd s

loop :: Maybe [Integer] -> IO (Maybe [Integer])
loop Nothing = return Nothing
loop (Just stack) = do
  shouldExit <- isEOF
  if shouldExit
  then
    putStrLn "end of the line" >> return Nothing
  else do
    -- putStr ">> "
    hFlush stdout
    line <- getLine
    case parseTerm line of
      RailInt n -> do
        let stack2 = n : stack
        loop (Just stack2)
      RailOp op -> do
        if length stack < 2
        then
          error $ "Stack underflow"
        else do
          let (a:b:stack2) = stack
          let stack3 = (a `op` b) : stack2
          loop (Just stack3)
      RailCmd cmd -> case cmd of
        "." -> do
          print stack
          loop (Just stack)
        unknown -> do
          error $ "Unknown command: " ++ unknown
