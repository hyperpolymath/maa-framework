-- SPDX-License-Identifier: MPL-2.0
pragma SPARK_Mode (On);

package body Proofada is

   function Clamp (Value, Lo, Hi : U32) return U32 is
      Result : U32 := Value;
   begin
      if Result < Lo then
         Result := Lo;
      end if;
      if Result > Hi then
         Result := Hi;
      end if;
      return Result;
   end Clamp;

   function Midpoint (A, B : U32) return U32 is
      D : constant U32 := B - A;
   begin
      --  Stepping stones for the provers: modular subtraction and division
      --  agree with their exact-integer counterparts on this domain, and
      --  the final addition cannot wrap because the result is at most B.
      pragma Assert (BI (D) = BI (B) - BI (A));
      pragma Assert (BI (D / 2) = BI (D) / 2);
      pragma Assert (BI (A + D / 2) = BI (A) + BI (D) / 2);
      pragma Assert (BI (A) + (BI (B) - BI (A)) / 2 = (BI (A) + BI (B)) / 2);
      return A + D / 2;
   end Midpoint;

end Proofada;
