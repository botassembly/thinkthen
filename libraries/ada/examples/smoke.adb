--  The replay smoke (ticket 0335): one decide through the environment-reading
--  engine, with the question and text sdlc/scripts/smoke names.
with Ada.Environment_Variables; use Ada.Environment_Variables;
with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Ada.Text_IO; use Ada.Text_IO;
with Thinkthen; use Thinkthen;
procedure Smoke is
   Client : Thinkthen.Engine;
   Answer : Thinkthen.Decision;
   Facts : Unbounded_String;
   Error : Thinkthen.Failure;
begin
   Thinkthen.Decide (Client, Value ("THINKTHEN_SMOKE_QUESTION"), Value ("THINKTHEN_SMOKE_TEXT"), Answer, Facts, Error);
   if Error.Kind /= Thinkthen.None then
      raise Program_Error with "smoke decide failed: " & Thinkthen.Message (Error);
   end if;
   case Answer.Value is
      when Yes => Put_Line ("smoke: true");
      when No => Put_Line ("smoke: false");
      when Not_Sure => Put_Line ("smoke: null");
   end case;
end Smoke;
