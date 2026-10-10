with Ada.Command_Line;
with Ada.Exceptions;
with Ada.Finalization;
with Ada.Text_IO; use Ada.Text_IO;
with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Interfaces; use type Interfaces.Unsigned_32; use type Interfaces.Unsigned_64;
with Interfaces.C; use Interfaces.C;
with Thinkthen; use Thinkthen;
with Thinkthen.Persistence; use Thinkthen.Persistence;
with Thinkthen.Requests; use Thinkthen.Requests;
with Thinkthen.Sessions; use Thinkthen.Sessions;
with Thinkthen.Sessions.Calls;
with Thinkthen_Session_C; use Thinkthen_Session_C;
procedure Usage_Status is
   Mode : constant String := Ada.Command_Line.Argument (1);
   Output : Packet;
   Terminal : Packet;
   Owner : Session;
   Read : Read_Status;
   Saved : Thinkthen.Persistence.Observation;
   procedure Require (Condition : Boolean) is
   begin
      if not Condition then raise Program_Error with "owned persistence behavior"; end if;
   end Require;
begin
   declare
      Client : Engine;
      Error : Failure;
      Request : T_RequestCall_decide;
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
      Request.T_question.V_0.T_text := T_RequestQuestion_text_field_text (Ada.Strings.Unbounded.To_Unbounded_String ("Is it?"));
      Request.T_input.V_0.T_text := T_RequestInput_text_field_text (Ada.Strings.Unbounded.To_Unbounded_String ("usage-ada-" & Mode));
      Thinkthen.Sessions.Calls.Decide (Client, Owner, Request);
      Finish (Owner);
      loop
         Try_Read (Owner, Output, Read);
         if Read = Pending then delay 0.001;
         elsif Read = Result then
            if View (Output).kind = K_THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_ROW_V1 then exit; end if;
         else raise Program_Error with "missing owned answer";
         end if;
      end loop;
      Require (View (Output).kind = K_THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_ROW_V1);
      loop
         Try_Read (Owner, Terminal, Read);
         if Read = Pending then delay 0.001;
         elsif Read = Result then
            if View (Terminal).kind = K_THINKTHEN_COMPLETE_SESSION_PACKET_TERMINAL_V1 then exit; end if;
         else raise Program_Error with "missing terminal facts";
         end if;
      end loop;
      Close (Owner);
      Require (Status (Terminal) = Success and then
        View (Terminal).data.terminal.facts.value.requests_sent = 1);
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
   Require (View (Output).data.decide_row.value.value.value.value.data.boolean = 1);
   Require (View (Terminal).data.terminal.facts.value.requests_sent = 1);
   Require (To_String (Saved.Advice) =
     (if Mode = "written" then "" else "check the usage folder permissions and free space"));
   Put_Line ("ADA_USAGE_PASS");
end Usage_Status;
