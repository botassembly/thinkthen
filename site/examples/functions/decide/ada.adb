with Interfaces; use Interfaces;
with Thinkthen.Requests; use Thinkthen.Requests;
with Thinkthen.Sessions; use Thinkthen.Sessions;
with Thinkthen.Sessions.Calls;
use Thinkthen.Sessions.Calls;
with Thinkthen_Session_C; use Thinkthen_Session_C;

procedure Sample is
   Question : constant String :=
     "Does the customer ask for a refund?";
   Broken : constant String :=
     "Please refund my order. It arrived broken.";
   Request : T_RequestCall_decide;
   Owner : Session;
   Answer : Packet;
   Read : Read_Status;
   Ready : Unsigned_32;
   Is_Refund : Boolean := False;
   Settlement : constant :=
     K_THINKTHEN_SESSION_WAIT_SETTLEMENT_V1;
   Decide_Row : constant :=
     K_THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_ROW_V1;
   subtype Decision is
     thinkthen_complete_atomic_decide_value_v1;
begin
   Request.T_question.V_0.T_text :=
     To_Unbounded_String (Question);
   Request.T_input.V_0.T_text :=
     To_Unbounded_String (Broken);
   Decide (Owner, Request);
   Finish (Owner);
   loop
      Try_Read (Owner, Answer, Read);
      exit when Read = Finished;
      if Read = Pending then
         Wait (Owner, Settlement, 50, Ready);
      elsif View (Answer).kind = Decide_Row then
         declare
            Row : Decision renames
              View (Answer).data.decide_row.value.all;
         begin
            Is_Refund :=
              State (Row.value.presence) = Present
              and then
                Row.value.value.value.data.boolean = 1;
         end;
      end if;
   end loop;
   pragma Assert (Is_Refund);
end Sample;
