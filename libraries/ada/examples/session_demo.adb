with Ada.Strings.Unbounded;
with Ada.Text_IO;
with Interfaces;
with Thinkthen.Requests;
with Thinkthen.Sessions;
with Thinkthen.Sessions.Calls;
with Thinkthen_Session_C;
procedure Session_Demo is
   use Thinkthen.Requests;
   use Thinkthen.Sessions;
   Owner : Session;
   Answer : Packet;
   State : Read_Status;
   Request : T_RequestCall_decide;
begin
   Request.T_question.V_0.T_text := T_RequestQuestion_text_field_text
     (Ada.Strings.Unbounded.To_Unbounded_String ("Does this ask for a refund?"));
   Request.T_input.V_0.T_text := T_RequestInput_text_field_text
     (Ada.Strings.Unbounded.To_Unbounded_String ("Please return my payment."));
   Thinkthen.Sessions.Calls.Decide (Owner, Request);
   Finish (Owner);
   loop
      Try_Read (Owner, Answer, State);
      case State is
         when Pending => delay 0.001;
         when Finished => exit;
         when Result =>
            declare
               View_Value : constant access constant Thinkthen_Session_C.thinkthen_complete_session_packet_v1 := View (Answer);
            begin
               Ada.Text_IO.Put_Line (Interfaces.Unsigned_32'Image (View_Value.kind));
               if Status (Answer) /= Success then raise Program_Error with "judgment failed"; end if;
            end;
      end case;
   end loop;
end Session_Demo;
