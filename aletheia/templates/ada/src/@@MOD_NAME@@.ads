-- SPDX-License-Identifier: MPL-2.0
-- Core library for @@PROJECT_NAME@@.
--
-- This package is in the SPARK subset: `SPARK_Mode (On)` below means
-- `gnatprove` analyses it, and every contract here is *statically proved*,
-- not merely checked at run time. See README.adoc for the proof recipe.
--
-- Replace these sample subprograms with your own. Adding a subprogram here
-- adds it to gnatprove's remit automatically.

pragma SPARK_Mode (On);

with Ada.Numerics.Big_Numbers.Big_Integers;
use Ada.Numerics.Big_Numbers.Big_Integers;

package @@MOD_ADA@@ is

   --  Unsigned 32-bit. Modular arithmetic wraps rather than raising, which
   --  makes the shift/divide forms below total.
   type U32 is mod 2 ** 32;

   --  Exact-integer view of U32. Contracts must reason about the
   --  *mathematical* half-sum; U32 arithmetic would wrap. This is the
   --  SPARK idiom for that, and the generated functions are ghost.
   package U32_Big is new Unsigned_Conversions (U32);

   --  Shorthand so the contracts below read like specifications.
   function BI (X : U32) return Big_Integer is (U32_Big.To_Big_Integer (X))
     with Ghost;

   --  Clamp Value into the inclusive range Lo .. Hi.
   --
   --  Proved: given Lo <= Hi, the result is always inside Lo .. Hi.
   function Clamp (Value, Lo, Hi : U32) return U32
     with Pre  => Lo <= Hi,
          Post => Clamp'Result in Lo .. Hi;

   --  Overflow-free midpoint of A and B, rounded towards zero.
   --
   --  The naive (A + B) / 2 wraps for large inputs; this form cannot,
   --  because it never materialises the sum.
   --
   --  Proved: given A <= B, the result lies in A .. B and equals the exact
   --  half-sum (A + B) / 2 as a mathematical integer.
   function Midpoint (A, B : U32) return U32
     with Pre  => A <= B,
          Post => Midpoint'Result in A .. B
                  and then BI (Midpoint'Result) = (BI (A) + BI (B)) / 2;

end @@MOD_ADA@@;
