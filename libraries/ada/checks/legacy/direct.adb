with Ada.Text_IO; use Ada.Text_IO;
with Interfaces.C; use Interfaces.C;
with Interfaces.C.Strings; use Interfaces.C.Strings;
with Thinkthen_C; use Thinkthen_C;
with System; use System;
procedure Direct is
   E : constant Handle := Engine_New;
   Q : chars_ptr := New_String ("Is it?");
   -- Ada String'Length counts UTF-8 storage bytes, not Unicode code points.
   Text : chars_ptr := New_String ("café");
   Result : aliased Answer := (Outcome => 123, Probability => -1.0);
   Code : int;
begin
   if E = Null_Handle then
      raise Program_Error with Value (Error_Message (E));
   end if;
   if Answer'Size /= 128 or Answer'Alignment /= 8 then
      raise Program_Error with "answer ABI mismatch";
   end if;
   Code := Decide (E, Q, Text, 5, Result'Access);
   if Code /= 0 then
      raise Program_Error with "code" & Code'Image & ": " & Value (Error_Message (E));
   end if;
   if Result.Outcome /= 1 or Result.Probability /= 0.9 then
      raise Program_Error with "incorrect judgment";
   end if;
   Put_Line ("ADA_DIRECT_PASS bytes=5 outcome=" & Result.Outcome'Image);
   Free (Q); Free (Text); Engine_Free (E);
end Direct;
