-- SPDX-License-Identifier: MPL-2.0
-- Test runner for @@PROJECT_NAME@@.
--
-- Dependency-free by design: no AUnit, just a counter and a non-zero exit
-- status on failure, so the suite needs nothing beyond the GNAT runtime.
--
-- These are *runtime* tests. They complement, and do not replace, the
-- static proof in @@MOD_NAME@@.ads — see README.adoc.
with Ada.Command_Line;
with Ada.Text_IO;
with @@MOD_ADA@@;

procedure Run_Tests is
   use Ada.Text_IO;
   use @@MOD_ADA@@;

   Failures : Natural := 0;

   procedure Check (Label : String; Condition : Boolean) is
   begin
      if not Condition then
         Put_Line ("FAIL: " & Label);
         Failures := Failures + 1;
      end if;
   end Check;

begin
   --  Clamp: below, inside and above the range
   Check ("clamp keeps an in-range value", Clamp (5, 0, 10) = 5);
   Check ("clamp lifts a low value", Clamp (0, 1, 10) = 1);
   Check ("clamp drops a high value", Clamp (99, 0, 10) = 10);
   Check ("clamp handles a degenerate range", Clamp (7, 7, 7) = 7);

   --  Clamp: idempotent, and always inside the range
   for Value in U32 range 0 .. 128 loop
      declare
         Once : constant U32 := Clamp (Value, 10, 100);
      begin
         Check ("clamp is idempotent", Clamp (Once, 10, 100) = Once);
         Check ("clamp result is in range", Once >= 10 and then Once <= 100);
      end;
   end loop;

   --  Midpoint: agrees with the naive form wherever that form is safe
   for A in U32 range 0 .. 63 loop
      for B in U32 range A .. 63 loop
         Check ("midpoint matches naive", Midpoint (A, B) = (A + B) / 2);
      end loop;
   end loop;

   --  Midpoint: stays inside the inputs, and cannot overflow
   Check ("midpoint bounded (0, max)", Midpoint (0, U32'Last) = U32'Last / 2);
   Check ("midpoint bounded (max, max)", Midpoint (U32'Last, U32'Last) = U32'Last);
   Check ("midpoint (8, 11)", Midpoint (8, 11) = 9);
   Check ("midpoint (7, 9)", Midpoint (7, 9) = 8);
   Check ("midpoint (0, 0)", Midpoint (0, 0) = 0);

   if Failures = 0 then
      Put_Line ("All tests passed.");
   else
      Put_Line (Natural'Image (Failures) & " test(s) failed.");
      Ada.Command_Line.Set_Exit_Status (Ada.Command_Line.Failure);
   end if;
end Run_Tests;
