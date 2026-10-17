with Ada.Command_Line;
with Ada.Exceptions;
with Ada.Finalization;
with Ada.Text_IO; use Ada.Text_IO;
with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Interfaces; use type Interfaces.Unsigned_32; use type Interfaces.Unsigned_64;
with Interfaces.C; use Interfaces.C;
with Thinkthen.Inputs;
with Thinkthen.Inputs.Calls;
with Thinkthen; use Thinkthen;
with Thinkthen.Answers;
with Thinkthen.Values;
with Thinkthen.Persistence; use Thinkthen.Persistence;
with Thinkthen.Sessions; use Thinkthen.Sessions;
with Thinkthen.Sessions.Calls;
procedure Usage_Status is
   use type Thinkthen.Answers.Completion_Status;
   Mode : constant String := Ada.Command_Line.Argument (1);
   Completed : Thinkthen.Answers.Completion;
   Answer : Thinkthen.Values.Value;
   Owner : Session;
   Saved : Thinkthen.Persistence.Observation;
   procedure Require (Condition : Boolean) is
   begin
      if not Condition then raise Program_Error with "owned persistence behavior"; end if;
   end Require;
begin
   declare
      Client : Engine;
      Error : Failure;
   begin
      Thinkthen.Inputs.Calls.Configure (Client, Thinkthen.Inputs.With_Max_Retries (Thinkthen.Inputs.With_Cache (Thinkthen.Inputs.Make_Engine_Settings, Thinkthen.Inputs.Cache_Document_From_False), 0), Error);
      Require (Error.Kind = None);
      if Mode = "disabled" then
         Require (Usage_Persistence (Client).State = Disabled);
         Require (Finish_Usage_Status (Client).State = Disabled);
         Put_Line ("DISABLED_PASS");
         return;
      end if;
      Require (Usage_Persistence (Client).State = Written);
      Thinkthen.Sessions.Calls.Decide (Client, Owner, Thinkthen.Inputs.Text_Question ("Is it?"),
        Thinkthen.Inputs.Text_Input ("usage-ada-" & Mode));
      Thinkthen.Sessions.Finish (Owner);
      Completed := Thinkthen.Answers.Collect (Owner);
      declare Call : constant Thinkthen.Answers.Decide_Result := Thinkthen.Answers.Decide (Completed); begin Answer := Call.Value; end;
      Require (Thinkthen.Values.As_Boolean (Answer));
      Thinkthen.Sessions.Close (Owner);
      Require (Thinkthen.Answers.Status (Completed) = Thinkthen.Answers.Succeeded and then
        Thinkthen.Answers.Requests_Sent (Thinkthen.Answers.Call_Facts (Completed)) = 1);
      Require (Usage_Persistence (Client).State = Pending);
      Put_Line ("PENDING"); Flush;
      declare Continue : constant String := Get_Line; begin Require (Continue = "continue"); end;
      Saved := Finish_Usage_Status (Client);
      Require (Saved.State = (if Mode = "written" then Written else Failed));
      Require (Usage_Persistence (Client) = Saved);
      -- Seed the separate failed-build slot before closing through the existing
      -- controlled lifecycle. A status failure must read the session slot.
      Thinkthen.Inputs.Calls.Configure (Client, Thinkthen.Inputs.With_Base_Url (Thinkthen.Inputs.Make_Engine_Settings, "invalid endpoint"), Error);
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
   Require (Thinkthen.Values.As_Boolean (Answer));
   Require (Thinkthen.Answers.Requests_Sent (Thinkthen.Answers.Call_Facts (Completed)) = 1);
   Require (To_String (Saved.Advice) =
     (if Mode = "written" then "" else "check the usage folder permissions and free space"));
   Put_Line ("ADA_USAGE_PASS");
end Usage_Status;
