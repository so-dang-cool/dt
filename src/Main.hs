module Main where

import System.IO

main :: IO ()
main =
  hSetBuffering stdin LineBuffering
  >> loop (Just [])
  >> return ()

loop :: Maybe [String] -> IO (Maybe [String])
loop Nothing = return Nothing
loop (Just stack) = do
  shouldExit <- isEOF
  if shouldExit
  then
    putStrLn "end of the line" >> return Nothing
  else do
    putStr ">> "
    term <- getLine
    let terms = term : stack
    print terms
    loop (Just terms)

