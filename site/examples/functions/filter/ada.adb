with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Interfaces; use Interfaces;
with Thinkthen.Requests; use Thinkthen.Requests;
with Thinkthen.Sessions; use Thinkthen.Sessions;
with Thinkthen.Sessions.Calls;
use Thinkthen.Sessions.Calls;
with Thinkthen_Session_C; use Thinkthen_Session_C;

procedure Sample is
   Question : constant String := "Is this a complaint?";
   function Review (Text : String) return T_RequestItem is
     (T_original =>
        (True,
         (T_RequestOriginal_arm_text,
          (T_kind => <>,
           T_text => To_Unbounded_String (Text)))),
      others => <>);
   Reviews : T_RequestItem_List;
   Request : T_RequestCall;
   Owner : Session;
   Answer : Packet;
   Read : Read_Status;
   Ready : Unsigned_32;
   Complaints : array (1 .. 4) of Boolean :=
     [others => False];
   Settlement : constant :=
     K_THINKTHEN_SESSION_WAIT_SETTLEMENT_V1;
   Filter_Row : constant :=
     K_THINKTHEN_COMPLETE_SESSION_PACKET_FILTER_ROW_V1;
   subtype Kept is thinkthen_complete_atomic_boolean_v1;
begin
   Reviews.Append
     (Review ("Arrived a day early. Thank you!"));
   Reviews.Append
     (Review
        ("The zipper broke the first time I used it."));
   Reviews.Append
     (Review ("Does this come in blue?"));
   Reviews.Append
     (Review ("The strap snapped on day two."));
   Request.T_question.V_0.T_text :=
     To_Unbounded_String (Question);
   Request.T_input :=
     (T_RequestInput_arm_records,
      (T_items => Reviews, T_kind => <>));
   Filter (Owner, Request);
   Finish (Owner);
   loop
      Try_Read (Owner, Answer, Read);
      exit when Read = Finished;
      if Read = Pending then
         Wait (Owner, Settlement, 50, Ready);
      elsif View (Answer).kind = Filter_Row then
         declare
            Row : Kept renames
              View (Answer).data.filter_row.value.all;
         begin
            Complaints (Natural (Row.index.value) + 1) :=
              Row.value = 1;
         end;
      end if;
   end loop;
   pragma Assert (Complaints = [False, True, False, True]);
end Sample;
