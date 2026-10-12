with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Interfaces; use Interfaces;
with Thinkthen.Requests; use Thinkthen.Requests;
with Thinkthen.Sessions; use Thinkthen.Sessions;
with Thinkthen.Sessions.Calls;
use Thinkthen.Sessions.Calls;
with Thinkthen_Session_C; use Thinkthen_Session_C;

procedure Sample is
   Text_In : constant String :=
     "Sam Patel says his Kindle arrived broken "
     & "in Denver.";
   Request : T_RequestCall_recognize;
   Owner : Session;
   Answer : Packet;
   Read : Read_Status;
   Ready : Unsigned_32;
   Names : Unbounded_String;
   Settlement : constant :=
     K_THINKTHEN_SESSION_WAIT_SETTLEMENT_V1;
   Settled : constant :=
     K_THINKTHEN_COMPLETE_SESSION_PACKET_TERMINAL_V1;
   Observed : constant :=
     K_THINKTHEN_COMPLETE_SESSION_PACKET_OBSERVATION_V1;
   type Recognized is access constant
     thinkthen_complete_recognition_v1;
   type Entity is access constant
     thinkthen_complete_entity_v1;
begin
   Request.T_question :=
     (T_RequestQuestion_arm_RequestQuestion_file,
      (T_kind => <>,
       T_path => To_Unbounded_String ("kinds.json")));
   Request.T_input.V_0.T_text :=
     To_Unbounded_String (Text_In);
   Recognize (Owner, Request);
   Finish (Owner);
   loop
      Try_Read (Owner, Answer, Read);
      exit when Read = Finished;
      if Read = Pending then
         Wait (Owner, Settlement, 50, Ready);
      elsif View (Answer).kind not in Settled | Observed
      then
         declare
            Count : constant Natural := Index
              (View (Answer).data.recognize_aggregate
                 .value.len);
            Rows : array (1 .. Count) of Recognized
              with Import,
                   Address =>
                     View (Answer).data.recognize_aggregate
                       .value.data;
         begin
            for Row of Rows loop
               declare
                  Spans : constant Natural := Index
                    (Row.value.data.fields_entities
                       .entities.len);
                  Entities : array (1 .. Spans) of Entity
                    with Import,
                         Address =>
                           Row.value.data.fields_entities
                             .entities.data;
               begin
                  for One of Entities loop
                     Append (Names, Text (One.text) & "/"
                               & Text (One.kind) & " ");
                  end loop;
               end;
            end loop;
         end;
      end if;
   end loop;
   pragma Assert
     (Names = "Sam Patel/person "
        & "Kindle/product "
        & "Denver/place ");
end Sample;
