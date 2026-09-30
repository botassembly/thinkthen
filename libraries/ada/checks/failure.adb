with Ada.Strings.Fixed;
with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Ada.Text_IO; use Ada.Text_IO;
with Thinkthen; use Thinkthen;
procedure Failure is
   Client : Engine;
   Result : Unbounded_String;
   First_Error : Thinkthen.Failure;
   Next_Error : Thinkthen.Failure;
   First_Facts : Unbounded_String;
   First_Message : Unbounded_String;
begin
   Call (Client, "{""decide"":""Is it?"",""evidence"":""failure-two""}", Result, First_Error);
   if First_Error.Kind /= Backend or else Failure_Facts (First_Error) = "" then
      raise Program_Error with "missing started failure facts";
   end if;
   First_Facts := To_Unbounded_String (Failure_Facts (First_Error));
   First_Message := To_Unbounded_String (Message (First_Error));
   Call (Client, "{""decide"":""Is it?"",""evidence"":""first""}", Result, Next_Error);
   if Next_Error.Kind /= None or else Ada.Strings.Fixed.Index (To_String (Result), """value"":true") = 0 then
      raise Program_Error with "recovery failed";
   end if;
   if Failure_Facts (First_Error) /= To_String (First_Facts) or else
      Message (First_Error) /= To_String (First_Message) then
      raise Program_Error with "borrowed failure changed after recovery";
   end if;
   Put_Line ("ADA_FAILURE_FACTS_PASS");
end Failure;
