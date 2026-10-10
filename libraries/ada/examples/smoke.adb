with Ada.Environment_Variables; use Ada.Environment_Variables;
with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Ada.Text_IO; use Ada.Text_IO;
with Thinkthen.Requests; use Thinkthen.Requests;
with Thinkthen.Sessions; use Thinkthen.Sessions;
with Thinkthen.Sessions.Calls;
with Thinkthen_Session_C; use Thinkthen_Session_C;
procedure Smoke is
   Owner : Session;
   Answer : Packet;
   Read : Read_Status;
   Request : T_RequestCall_decide;
begin
   Request.T_question.V_0.T_text := T_RequestQuestion_text_field_text
     (Ada.Strings.Unbounded.To_Unbounded_String (Value ("THINKTHEN_TEST_SMOKE_QUESTION")));
   Request.T_input.V_0.T_text := T_RequestInput_text_field_text
     (Ada.Strings.Unbounded.To_Unbounded_String (Value ("THINKTHEN_TEST_SMOKE_TEXT")));
   Thinkthen.Sessions.Calls.Decide (Owner, Request);
   Finish (Owner);
   loop
      Try_Read (Owner, Answer, Read);
      case Read is
         when Pending => delay 0.001;
         when Finished => exit;
         when Result =>
            if Status (Answer) /= Success then raise Program_Error with "smoke judgment failed"; end if;
            declare
               P : constant access constant thinkthen_complete_session_packet_v1 := View (Answer);
            begin
               if P.kind = K_THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_ROW_V1 then
                  case State (P.data.decide_row.value.value.presence) is
                     when Null_Value => Put_Line ("smoke: null");
                     when Missing => raise Program_Error with "smoke answer missing";
                     when Present =>
                        if P.data.decide_row.value.value.value.value.data.boolean = 1 then
                           Put_Line ("smoke: true");
                        else Put_Line ("smoke: false");
                        end if;
                  end case;
               end if;
            end;
      end case;
   end loop;
end Smoke;
