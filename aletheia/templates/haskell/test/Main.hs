-- SPDX-License-Identifier: MPL-2.0
-- | Test suite for @@PROJECT_NAME@@.
--
-- Deliberately dependency-free: a plain exitcode-stdio runner rather than
-- a framework, so the package still needs only `base`.
module Main (main) where

import Control.Monad (forM_, unless)
import Core (clamp, meanFloor)
import Data.IORef (modifyIORef', newIORef, readIORef)
import System.Exit (exitFailure, exitSuccess)

main :: IO ()
main = do
  failures <- newIORef (0 :: Int)
  let check label condition =
        unless condition $ do
          putStrLn ("FAIL: " ++ label)
          modifyIORef' failures (+ 1)

  -- clamp: values inside, below and above the range
  check "clamp keeps an in-range value" (clamp (5 :: Int) 0 10 == 5)
  check "clamp lifts a low value" (clamp (0 :: Int) 1 10 == 1)
  check "clamp drops a high value" (clamp (99 :: Int) 0 10 == 10)
  check "clamp handles a degenerate range" (clamp (7 :: Int) 7 7 == 7)

  -- clamp: idempotent and always in range
  forM_ [0, 1, 5, 42, 10000 :: Int] $ \value -> do
    let once = clamp value 10 100
    check "clamp is idempotent" (clamp once 10 100 == once)
    check "clamp result is in range" (once >= 10 && once <= 100)

  -- meanFloor: agrees with the naive form where the naive form is safe
  forM_ [0 .. 63 :: Word] $ \a ->
    forM_ [0 .. 63 :: Word] $ \b ->
      check "meanFloor matches naive" (meanFloor a b == (a + b) `div` 2)

  -- meanFloor: never exceeds either input, and cannot overflow
  forM_ [(0, 0), (1, 2), (7, 9), (maxBound, 0), (maxBound, maxBound)] $
    \(a, b) -> check "meanFloor bounded" (meanFloor a b <= max a b)

  total <- readIORef failures
  if total == 0
    then putStrLn "All tests passed." >> exitSuccess
    else putStrLn (show total ++ " test(s) failed.") >> exitFailure
