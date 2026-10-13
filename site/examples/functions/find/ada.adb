with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Interfaces; use Interfaces;
with Thinkthen.Requests; use Thinkthen.Requests;
with Thinkthen.Sessions; use Thinkthen.Sessions;
with Thinkthen.Sessions.Calls;
use Thinkthen.Sessions.Calls;
with Thinkthen_Session_C; use Thinkthen_Session_C;

procedure Sample is
   Question : constant String :=
     "Which line gives the refund deadline?";
   function Line (Text : String) return T_RequestItem is
     (T_original =>
        (True,
         (T_RequestOriginal_arm_text,
          (T_kind => <>,
           T_text => To_Unbounded_String (Text)))),
      others => <>);
   Policy : T_RequestItem_List;
   Request : T_RequestCall;
   Owner : Session;
   Answer : Packet;
   Read : Read_Status;
   Ready : Unsigned_32;
   Refund_Deadline : Unbounded_String;
   Settlement : constant :=
     K_THINKTHEN_SESSION_WAIT_SETTLEMENT_V1;
   Found : constant :=
     K_THINKTHEN_COMPLETE_SESSION_PACKET_FIND_AGGREGATE_V1;
begin
   Policy.Append
     (Line ("Returns need the original receipt."));
   Policy.Append
     (Line
        ("Refunds are issued within 30 days of purchase."));
   Policy.Append
     (Line ("Shipping is free on orders over $50."));
   Policy.Append
     (Line ("Gift cards cannot be exchanged for cash."));
   Request.T_question.V_0.T_text :=
     To_Unbounded_String (Question);
   Request.T_input :=
     (T_RequestInput_arm_records,
      (T_items => Policy, T_kind => <>));
   Find (Owner, Request);
   Finish (Owner);
   loop
      Try_Read (Owner, Answer, Read);
      exit when Read = Finished;
      if Read = Pending then
         Wait (Owner, Settlement, 50, Ready);
      elsif View (Answer).kind = Found then
         declare
            Unit : thinkthen_complete_find_v1 renames
              View (Answer).data.find_aggregate.value.all;
         begin
            if State (Unit.value.presence) = Present then
               Refund_Deadline := To_Unbounded_String
                 (Text (Unit.value.value.data.string));
            end if;
         end;
      end if;
   end loop;
   pragma Assert
     (Refund_Deadline =
        "Refunds are issued within 30 days of purchase.");
end Sample;
