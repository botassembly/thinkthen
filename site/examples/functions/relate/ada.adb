with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Interfaces; use Interfaces;
with Thinkthen.Values;
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
         (T_RequestOriginal_arm_json,
          (T_kind => <>,
           T_value => Thinkthen.Values.Object_Item
             ([Thinkthen.Values.Named
                ("name",
                 Thinkthen.Values.String_Item (Text)),
               Thinkthen.Values.Named
                ("kind",
                 Thinkthen.Values.String_Item (Kind))])))),
      others => <>);
   Names : T_RequestItem_List;
   Request : T_RequestCall;
   Owner : Session;
   Answer : Packet;
   Read : Read_Status;
   Ready : Unsigned_32;
   Who_Makes : Unbounded_String;
   Settlement : constant :=
     K_THINKTHEN_SESSION_WAIT_SETTLEMENT_V1;
   Settled : constant :=
     K_THINKTHEN_COMPLETE_SESSION_PACKET_TERMINAL_V1;
   Observed : constant :=
     K_THINKTHEN_COMPLETE_SESSION_PACKET_OBSERVATION_V1;
   type Edge is access constant
     thinkthen_complete_related_entity_edge_v1;
begin
   Names.Append (Name ("Apple", "company"));
   Names.Append (Name ("Amazon", "company"));
   Names.Append (Name ("iPhone", "product"));
   Names.Append (Name ("Kindle", "product"));
   Request.T_question :=
     (T_RequestQuestion_arm_file,
      (T_kind => <>,
       T_path => To_Unbounded_String ("makes.json")));
   Request.T_input :=
     (T_RequestInput_arm_entities,
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
                 (Who_Makes,
                  Text (One.source.data.related_entity
                          .name)
                  & " makes "
                  & Text (One.target.data.related_entity
                            .name)
                  & ". ");
            end loop;
         end;
      end if;
   end loop;
   pragma Assert
     (Who_Makes = "Apple makes iPhone. "
        & "Amazon makes Kindle. ");
end Sample;
