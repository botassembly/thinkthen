with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Interfaces; use Interfaces;
with Thinkthen.Requests; use Thinkthen.Requests;
with Thinkthen.Sessions; use Thinkthen.Sessions;
with Thinkthen.Sessions.Calls;
use Thinkthen.Sessions.Calls;
with Thinkthen_Session_C; use Thinkthen_Session_C;

procedure Sample is
   function Name (Text, Kind : String)
     return T_RequestItem is
     (T_original =>
        (True,
         (T_RequestOriginal_arm_RequestOriginal_json,
          (T_kind => <>,
           T_value => To_Unbounded_String
             ("{""name"": """ & Text
              & """, ""kind"": """ & Kind & """}")))),
      others => <>);
   Names : T_RequestInput_entities_field_items;
   Request : T_RequestCall_relate;
   Owner : Session;
   Answer : Packet;
   Read : Read_Status;
   Ready : Unsigned_32;
   Who_Sings : Unbounded_String;
   Settlement : constant :=
     K_THINKTHEN_SESSION_WAIT_SETTLEMENT_V1;
   Settled : constant :=
     K_THINKTHEN_COMPLETE_SESSION_PACKET_TERMINAL_V1;
   Observed : constant :=
     K_THINKTHEN_COMPLETE_SESSION_PACKET_OBSERVATION_V1;
   type Edge is access constant
     thinkthen_complete_related_entity_edge_v1;
begin
   Names.Append (Name ("Paul McCartney", "singer"));
   Names.Append (Name ("Ringo Starr", "singer"));
   Names.Append (Name ("Yesterday", "song"));
   Names.Append (Name ("Octopus's Garden", "song"));
   Request.T_question :=
     (T_RequestQuestion_arm_RequestQuestion_file,
      (T_kind => <>,
       T_path => To_Unbounded_String ("sings.json")));
   Request.T_input :=
     (T_RequestInput_arm_RequestInput_entities,
      (T_items => Names, T_kind => <>));
   Relate (Owner, Request);
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
              (View (Answer).data.relate_aggregate
                 .value.value.len);
            Edges : array (1 .. Count) of Edge
              with Import,
                   Address =>
                     View (Answer).data.relate_aggregate
                       .value.value.data;
         begin
            for One of Edges loop
               Append
                 (Who_Sings,
                  Text (One.source.data.fields_kind_name
                          .name)
                  & " sings "
                  & Text (One.target.data.fields_kind_name
                            .name)
                  & ". ");
            end loop;
         end;
      end if;
   end loop;
   pragma Assert
     (Who_Sings = "Paul McCartney sings Yesterday. "
        & "Ringo Starr sings Octopus's Garden. ");
end Sample;
