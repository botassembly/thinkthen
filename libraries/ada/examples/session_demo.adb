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
   Request : constant T_RequestCall :=
     (T_question => Text_Question ("Does this ask for a refund?"),
      T_input => Items_Input ([Text_Item ("Please return my payment.")]), others => <>);
begin
   Thinkthen.Sessions.Calls.Decide (Owner, Request);
   Finish (Owner);
   loop
      Next_Packet (Owner, Answer, State);
      case State is
         when Pending => raise Program_Error with "blocking read returned pending";
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
