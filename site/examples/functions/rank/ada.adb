with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Interfaces; use Interfaces;
with Thinkthen.Requests; use Thinkthen.Requests;
with Thinkthen.Sessions; use Thinkthen.Sessions;
with Thinkthen.Sessions.Calls;
use Thinkthen.Sessions.Calls;
with Thinkthen_Session_C; use Thinkthen_Session_C;

procedure Sample is
   Question : constant String := "Is this urgent?";
   function Message (Text : String) return T_RequestItem is
     (T_original =>
        (True,
         (T_RequestOriginal_arm_RequestOriginal_text,
          (T_kind => <>,
           T_text => To_Unbounded_String (Text)))),
      others => <>);
   Inbox : T_RequestInput_records_field_items;
   Request : T_RequestCall_rank;
   Owner : Session;
   Answer : Packet;
   Read : Read_Status;
   Ready : Unsigned_32;
   By_Urgency : array (1 .. 4) of Natural := [others => 0];
   Settlement : constant :=
     K_THINKTHEN_SESSION_WAIT_SETTLEMENT_V1;
   Rank_Aggregate : constant :=
     K_THINKTHEN_COMPLETE_SESSION_PACKET_RANK_AGGREGATE_V1;
   subtype Ranking is
     thinkthen_complete_session_packet_rank_aggregate_v1;
   type Place is access constant
     thinkthen_complete_atomic_non_zero_usize_v1;
begin
   Inbox.Append
     (Message ("Newsletter: our autumn catalog is here. "
               & "No reply needed."));
   Inbox.Append
     (Message
        ("Our checkout page is down and customers "
         & "cannot pay"));
   Inbox.Append
     (Message ("Reminder: your invoice is due in 30 days"));
   Inbox.Append
     (Message
        ("Please send the signed quote by 5 pm today"));
   Request.T_question.V_0.T_text :=
     To_Unbounded_String (Question);
   Request.T_input :=
     (T_RequestInput_arm_RequestInput_records,
      (T_items => Inbox, T_kind => <>));
   Rank (Owner, Request);
   Finish (Owner);
   loop
      Try_Read (Owner, Answer, Read);
      exit when Read = Finished;
      if Read = Pending then
         Wait (Owner, Settlement, 50, Ready);
      elsif View (Answer).kind = Rank_Aggregate then
         declare
            Ranked : Ranking renames
              View (Answer).data.rank_aggregate.all;
            Places : array (1 .. Index (Ranked.value.len))
              of Place
              with Import, Address => Ranked.value.data;
         begin
            for One of Places loop
               By_Urgency (Natural (One.value)) :=
                 Natural (One.index.value) + 1;
            end loop;
         end;
      end if;
   end loop;
   pragma Assert (By_Urgency = [2, 4, 3, 1]);
end Sample;
