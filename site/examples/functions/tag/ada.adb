with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Interfaces; use Interfaces;
with Thinkthen.Requests; use Thinkthen.Requests;
with Thinkthen.Sessions; use Thinkthen.Sessions;
with Thinkthen.Sessions.Calls;
use Thinkthen.Sessions.Calls;
with Thinkthen_Session_C; use Thinkthen_Session_C;

procedure Sample is
   Question : constant String :=
     "Which labels fit this message?";
   Message : constant String :=
     "Love the new dashboard, but export crashes the app,"
     & ASCII.LF & "and I was charged twice." & ASCII.LF;
   Labels : T_Authored_name_List;
   Which : T_RequestDefinition_Authored_tag;
   Request : T_RequestCall;
   Owner : Session;
   Answer : Packet;
   Read : Read_Status;
   Ready : Unsigned_32;
   Fitting_Labels : Unbounded_String;
   Settlement : constant :=
     K_THINKTHEN_SESSION_WAIT_SETTLEMENT_V1;
   Tag_Row : constant :=
     K_THINKTHEN_COMPLETE_SESSION_PACKET_TAG_ROW_V1;
   Tag_Arm : constant T_RequestDefinition_Kind :=
     T_RequestDefinition_arm_Authored_tag;
   subtype Tagged_Text is
     thinkthen_complete_atomic_array_of_string_v1;
begin
   Labels.Append
     (T_Authored_name (To_Unbounded_String ("praise")));
   Labels.Append
     (T_Authored_name (To_Unbounded_String ("bug")));
   Labels.Append
     (T_Authored_name (To_Unbounded_String ("billing")));
   Which.T_tag :=
     (T_Authored_questionText_arm_0,
      To_Unbounded_String (Question));
   Which.T_labels :=
     (True, (T_Authored_labels_arm_0, Labels));
   Request.T_question :=
     (T_RequestQuestion_arm_definition,
      (T_kind => <>, T_value => (Tag_Arm, Which)));
   Request.T_input.V_0.T_text :=
     To_Unbounded_String (Message);
   Tag (Owner, Request);
   Finish (Owner);
   loop
      Try_Read (Owner, Answer, Read);
      exit when Read = Finished;
      if Read = Pending then
         Wait (Owner, Settlement, 50, Ready);
      elsif View (Answer).kind = Tag_Row then
         declare
            Row : Tagged_Text renames
              View (Answer).data.tag_row.value.all;
            Found : array (1 .. Index (Row.value.len))
              of thinkthen_complete_utf8_v1
              with Import,
                   Address => Row.value.data.all'Address;
         begin
            for Label of Found loop
               Append (Fitting_Labels, Text (Label) & " ");
            end loop;
         end;
      end if;
   end loop;
   pragma Assert (Fitting_Labels = "praise bug billing ");
end Sample;
