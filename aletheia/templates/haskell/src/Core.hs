-- SPDX-License-Identifier: MPL-2.0
-- | Core library for @@PROJECT_NAME@@.
--
-- Replace these sample functions with your own.
module Core
  ( clamp
  , meanFloor
  ) where

import Data.Bits (shiftR, xor, (.&.))

-- | Clamp @value@ into the inclusive range @[lo, hi]@.
--
-- If @lo > hi@ the range is empty; the upper bound wins, so the result is
-- @hi@. Callers that care should check their range first.
clamp :: Ord a => a -> a -> a -> a
clamp value lo hi = min hi (max lo value)

-- | Integer mean of two 'Word's, rounded towards zero.
--
-- The naive @(a + b) \`div\` 2@ overflows for large inputs; this form
-- cannot, because it never materialises the sum.
meanFloor :: Word -> Word -> Word
meanFloor a b = (a .&. b) + ((a `xor` b) `shiftR` 1)
