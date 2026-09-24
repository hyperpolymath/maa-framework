-- SPDX-License-Identifier: MPL-2.0
-- | @@PROJECT_NAME@@ — command-line entry point.
module Main (main) where

import Core (clamp, meanFloor)
import System.IO (hPutStrLn, stderr)
import System.Environment (getArgs)
import System.Exit (exitFailure, exitSuccess)
import Text.Read (readMaybe)

usage :: String
usage =
  unlines
    [ "@@PROJECT_NAME@@ 0.1.0"
    , ""
    , "usage:"
    , "  @@PROJECT_NAME@@ clamp <value> <lo> <hi>   clamp a value into a range"
    , "  @@PROJECT_NAME@@ mean  <a> <b>             integer mean, rounded down"
    , "  @@PROJECT_NAME@@ --help                    show this message"
    , ""
    , "All arguments are non-negative integers."
    ]

parseWord :: String -> Maybe Word
parseWord raw = case readMaybe raw of
  Just n | n >= 0 -> Just n
  _ -> Nothing

main :: IO ()
main = do
  args <- getArgs
  case args of
    [] -> putStr usage >> exitSuccess
    ["--help"] -> putStr usage >> exitSuccess
    ["-h"] -> putStr usage >> exitSuccess
    ["clamp", value, lo, hi] ->
      case (parseWord value, parseWord lo, parseWord hi) of
        (Just v, Just l, Just h) | l <= h -> print (clamp v l h) >> exitSuccess
        _ -> unrecognised
    ["mean", a, b] ->
      case (parseWord a, parseWord b) of
        (Just x, Just y) -> print (meanFloor x y) >> exitSuccess
        _ -> unrecognised
    _ -> unrecognised

unrecognised :: IO a
unrecognised = do
  hPutStrLn stderr "error: unrecognised arguments"
  hPutStrLn stderr ""
  hPutStrLn stderr usage
  exitFailure
