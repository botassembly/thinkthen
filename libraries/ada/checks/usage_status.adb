with Ada.Command_Line;
with Ada.Exceptions;
with Ada.Finalization;
with Ada.Text_IO; use Ada.Text_IO;
with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Interfaces; use type Interfaces.Unsigned_32; use type Interfaces.Unsigned_64;
with Interfaces.C; use Interfaces.C;
with Thinkthen; use Thinkthen;
with Thinkthen.Persistence; use Thinkthen.Persistence;
with Thinkthen.Sessions;
with Thinkthen.Typed; use Thinkthen.Typed;
with Thinkthen.Typed.Complete; use Thinkthen.Typed.Complete;
with Thinkthen_C_Inputs; use Thinkthen_C_Inputs;
with Thinkthen_C_Rows; use Thinkthen_C_Rows;
with Thinkthen_C_Events; use Thinkthen_C_Events;
procedure Usage_Status is
   Mode : constant String := Ada.Command_Line.Argument (1);
   Output : Result;
   Row : Decide_View_V1;
   Facts : Summary_V1;
   Saved : Thinkthen.Persistence.Observation;
   Code : int;
   procedure Require (Condition : Boolean) is
   begin
      if not Condition then raise Program_Error with "owned persistence behavior"; end if;
   end Require;
begin
   declare
      Client : Engine;
      Error : Failure;
      Q : Question;
      Input : Source;
      Asked : aliased String := "Is it?";
      Evidence : aliased String := "usage-ada-" & Mode;
      Spec : Question_Spec_V1 :=
        (Kind => C_FUNCTION_DECIDE_V1,
         Text => (C_CONTENT_TEXT_V1, (Asked'Address, Asked'Length)), others => <>);
      Records_Input : Record_Array (1 .. 1);
   begin
      Configure (Client, "{""cache"":false,""max_retries"":0}", Error);
      Require (Error.Kind = None);
      if Mode = "disabled" then
         Require (Usage_Persistence (Client).State = Disabled);
         Require (Finish_Usage_Status (Client).State = Disabled);
         Put_Line ("DISABLED_PASS");
         return;
      end if;
      Require (Usage_Persistence (Client).State = Written);
      New_Question (Client, Spec, Q, Code); Require (Code = 0);
      Records_Input (1).Original := (1, (C_CONTENT_TEXT_V1, (Evidence'Address, Evidence'Length)));
      Records (Client, Records_Input, Input, Code); Require (Code = 0);
      Decide (Client, Q, Input, Controls, Output, Code); Require (Code = 0);
      Summary (Output, Facts, Code); Require (Code = 0 and Facts.Facts.Value.Requests_Sent = 1);
      Require (Usage_Persistence (Client).State = Pending);
      Put_Line ("PENDING"); Flush;
      declare Continue : constant String := Get_Line; begin Require (Continue = "continue"); end;
      Saved := Finish_Usage_Status (Client);
      Require (Saved.State = (if Mode = "written" then Written else Failed));
      Require (Usage_Persistence (Client) = Saved);
      -- Seed the separate failed-build slot before closing through the existing
      -- controlled lifecycle. A status failure must read the session slot.
      Configure (Client, "{""unknown_setting"":true}", Error);
      Require (Error.Kind = Usage);
      Ada.Finalization.Finalize (Ada.Finalization.Limited_Controlled'Class (Client));
      for Finish in Boolean loop
         declare
            Refused : Boolean := False;
            Unused : Thinkthen.Persistence.Observation;
         begin
            begin
               Unused := (if Finish then Finish_Usage_Status (Client) else Usage_Persistence (Client));
            exception
               when Failure : Thinkthen.Sessions.Usage_Error =>
                  Require (Ada.Exceptions.Exception_Message (Failure) = "invalid session arguments or input");
                  Refused := True;
            end;
            Require (Refused);
         end;
      end loop;
   end;
   Decide (Output, 0, Row, Code); Require (Code = 0);
   Require (Row.Value.Kind = C_DECIDE_BOOLEAN_V1 and Row.Value.Data.Boolean = 1);
   Summary (Output, Facts, Code); Require (Code = 0 and Facts.Facts.Value.Requests_Sent = 1);
   Require (To_String (Saved.Advice) =
     (if Mode = "written" then "" else "check the usage folder permissions and free space"));
   Put_Line ("ADA_USAGE_PASS");
end Usage_Status;
