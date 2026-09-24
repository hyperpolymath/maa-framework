-- SPDX-License-Identifier: MPL-2.0
-- @@PROJECT_NAME@@ — command-line entry point.
--
-- SPARK_Mode is Off here on purpose: this unit is the I/O boundary, and
-- Ada.Text_IO, exceptions and 'Value/'Image are all outside the SPARK
-- subset. The logic it calls lives in @@MOD_NAME@@.ads, which is SPARK and
-- fully proved. gnatprove skips this file rather than pretending to verify
-- it — see README.adoc.
pragma SPARK_Mode (Off);

with Ada.Command_Line;
with Ada.Text_IO;
with @@MOD_ADA@@;

procedure Main is
   use Ada.Command_Line;
   use Ada.Text_IO;
   use @@MOD_ADA@@;

   Usage : constant String :=
     "@@PROJECT_NAME@@ @@VERSION@@" & ASCII.LF &
     "" & ASCII.LF &
     "usage:" & ASCII.LF &
     "  @@PROJECT_NAME@@ clamp <value> <lo> <hi>   clamp a value into a range" & ASCII.LF &
     "  @@PROJECT_NAME@@ mid   <a> <b>             midpoint, rounded down (a <= b)" & ASCII.LF &
     "  @@PROJECT_NAME@@ --help                    show this message" & ASCII.LF &
     "" & ASCII.LF &
     "All arguments are unsigned 32-bit integers.";

   procedure Unrecognised is
   begin
      Put_Line (Standard_Error, "error: unrecognised arguments");
      New_Line (Standard_Error);
      Put_Line (Standard_Error, Usage);
      Set_Exit_Status (Failure);
   end Unrecognised;

begin
   if Argument_Count = 0 or else Argument (1) = "--help" or else Argument (1) = "-h" then
      Put_Line (Usage);
      return;
   end if;

   if Argument_Count = 4 and then Argument (1) = "clamp" then
      declare
         Value : constant U32 := U32'Value (Argument (2));
         Lo    : constant U32 := U32'Value (Argument (3));
         Hi    : constant U32 := U32'Value (Argument (4));
      begin
         if Lo > Hi then
            Unrecognised;
            return;
         end if;
         Put_Line (U32'Image (Clamp (Value, Lo, Hi)));
         return;
      exception
         when Constraint_Error =>
            Unrecognised;
            return;
      end;
   end if;

   if Argument_Count = 3 and then Argument (1) = "mid" then
      declare
         A : constant U32 := U32'Value (Argument (2));
         B : constant U32 := U32'Value (Argument (3));
      begin
         if A > B then
            Unrecognised;
            return;
         end if;
         Put_Line (U32'Image (Midpoint (A, B)));
         return;
      exception
         when Constraint_Error =>
            Unrecognised;
            return;
      end;
   end if;

   Unrecognised;
end Main;
