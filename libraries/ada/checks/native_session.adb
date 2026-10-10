with Ada.Command_Line;
with Ada.Environment_Variables;
with Thinkthen;
with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Ada.Text_IO; use Ada.Text_IO;
with Interfaces; use Interfaces;
with Interfaces.C; use type Interfaces.C.double;
with System;
with Interfaces.C.Strings;
with Thinkthen.Requests; use Thinkthen.Requests;
with Thinkthen.Sessions; use Thinkthen.Sessions;
with Thinkthen.Sessions.Calls;
with Thinkthen_Session_C; use Thinkthen_Session_C;
procedure Native_Session is
   Client : Thinkthen.Engine;
   use type Thinkthen.Error_Kind;
   procedure Select_Settings (Route, Settings : String) is
      Error : Thinkthen.Failure;
   begin
      Thinkthen.Configure (Client, "{""base_url"":""" &
        Ada.Environment_Variables.Value ("TT_BACKEND_ROOT") & Route & """," & Settings & "}", Error);
      if Error.Kind /= Thinkthen.None then raise Program_Error with Thinkthen.Message (Error); end if;
   end Select_Settings;
   procedure Select_Route (Route : String) is
      Error : Thinkthen.Failure;
   begin
      Thinkthen.Configure (Client, "{""cache"":false,""base_url"":""" &
        Ada.Environment_Variables.Value ("TT_BACKEND_ROOT") & Route & """}", Error);
      if Error.Kind /= Thinkthen.None then raise Program_Error with Thinkthen.Message (Error); end if;
   end Select_Route;
   Owner : Session;
   Value : Packet;
   Read : Read_Status;
   Seen : Boolean;
   Entity_Count : Natural;
   procedure Require (Condition : Boolean; Message : String) is
   begin
      if not Condition then raise Program_Error with Message; end if;
   end Require;
   procedure Metadata (M : access constant thinkthen_complete_meta_v1) is
   begin
      Require (M /= null and then Text (M.model)'Length > 0 and then
        Text (M.tool)'Length > 0 and then Text (M.url)'Length > 0 and then
        Index (M.requests.len) > 0, "metadata missing");
      Require (State (M.usage.presence) in Missing | Present and then
        State (M.question_sha256.presence) in Missing | Present and then
        State (M.attempts.presence) in Missing | Present, "metadata presence invalid");
   end Metadata;
   Terminal : Boolean;
   procedure Drain (Failure : Error_Status := Success; Decision : Integer := -1; Located : Boolean := False; Expected_Sends : Integer := -1; Images : Boolean := False; Cache_Hit : Boolean := False; Odds : Interfaces.C.double := 0.9) is
   begin
      Seen := False;
      Entity_Count := 0;
      Terminal := False;
      loop
         Try_Read (Owner, Value, Read);
         case Read is
            when Pending => delay 0.001;
            when Finished => exit;
            when Result =>
               declare
                  P : constant access constant thinkthen_complete_session_packet_v1 := View (Value);
               begin
                  if P.kind = K_THINKTHEN_COMPLETE_SESSION_PACKET_TERMINAL_V1 then
                     Terminal := True;
                     Close (Owner);
                     if Status (Value) /= Failure then raise Program_Error with "wrong terminal failure"; end if;
                     if Failure = Success and then State (P.data.terminal.facts.presence) /= Present then raise Program_Error with "missing facts"; end if;
                     if State (P.data.terminal.facts.presence) = Present and then Text (P.data.terminal.facts.value.call_id.value)'Length = 0 then raise Program_Error with "missing call identity"; end if;
                     if Failure = Success then
                        if Expected_Sends < 0 then
                           Require (P.data.terminal.facts.value.requests_sent > 0, "missing sends");
                        else
                           Require (P.data.terminal.facts.value.requests_sent = Unsigned_64 (Expected_Sends), "unexpected sends");
                        end if;
                        if Cache_Hit then Require (P.data.terminal.facts.value.cache_answers = 1, "cache miss"); end if;
                        Require (P.data.terminal.facts.value.records > 0 and then
                          Text (P.data.terminal.facts.value.token_estimate_method)'Length > 0, "facts missing");
                     end if;
                     if Failure = Backend then
                        if Text (P.data.terminal.failure.value.error.message)'Length = 0 or else
                           State (P.data.terminal.failure.value.facts.presence) /= Present or else
                           P.data.terminal.failure.value.facts.value.requests_sent /= 1 then
                           raise Program_Error with "missing failure facts";
                        end if;
                     end if;
                     exit;
                  end if;
                  case P.kind is
                     when K_THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_ROW_V1 =>
                        Seen := Text (P.data.decide_row.value.answer_id.value)'Length > 0 and then P.data.decide_row.value.meta /= null;
                        if Decision = 0 then
                           if State (P.data.decide_row.value.value.presence) /= Present or else
                              P.data.decide_row.value.value.value = null or else
                              P.data.decide_row.value.value.value.value = null or else
                              P.data.decide_row.value.value.value.value.kind /= K_THINKTHEN_COMPLETE_JSON_BOOLEAN_V1 or else
                              P.data.decide_row.value.value.value.value.data.boolean /= 0 then
                              raise Program_Error with "false decision lost";
                           end if;
                        elsif Decision = 1 then
                           if State (P.data.decide_row.value.value.presence) /= Null_Value or else
                              P.data.decide_row.value.value.value /= null then
                              raise Program_Error with "null decision lost";
                           end if;
                        end if;
                        Metadata (P.data.decide_row.value.meta);
                        Require (P.data.decide_row.value.answer.kind = K_THINKTHEN_COMPLETE_ANSWER_YES_NO_V1 and then
                          P.data.decide_row.value.answer.data.yes_no.probability = Odds, "decision odds changed");
                        if Located then
                           Require (State (P.data.decide_row.value.source.presence) = Present and then
                             State (P.data.decide_row.value.source.value.first_line.presence) = Present, "source missing");
                        end if;
                        if Images then Require (State (P.data.decide_row.value.images.presence) = Present and then
                          Index (P.data.decide_row.value.images.value.len) = 2, "ordered images missing"); end if;
                     when K_THINKTHEN_COMPLETE_SESSION_PACKET_CHOOSE_ROW_V1 =>
                        Seen := Text (P.data.choose_row.value.answer_id.value)'Length > 0;
                        Metadata (P.data.choose_row.value.meta);
                        Require (State (P.data.choose_row.value.value.presence) = Present and then
                          Text (P.data.choose_row.value.value.value) = "billing" and then
                          P.data.choose_row.value.answer.kind = K_THINKTHEN_COMPLETE_ANSWER_CHOICE_V1 and then
                          Index (P.data.choose_row.value.answer.data.choice.probabilities.len) = 3, "choice changed");
                     when K_THINKTHEN_COMPLETE_SESSION_PACKET_TAG_ROW_V1 =>
                        Seen := Text (P.data.tag_row.value.answer_id.value)'Length > 0;
                        Metadata (P.data.tag_row.value.meta);
                        Require (Index (P.data.tag_row.value.value.len) = 2, "tags changed");
                     when K_THINKTHEN_COMPLETE_SESSION_PACKET_SCORE_ROW_V1 =>
                        Seen := Text (P.data.score_row.value.answer_id.value)'Length > 0;
                        Metadata (P.data.score_row.value.meta);
                        Require (abs (P.data.score_row.value.value - 1.0) < 0.00001, "score changed");
                     when K_THINKTHEN_COMPLETE_SESSION_PACKET_FILTER_ROW_V1 =>
                        Seen := Text (P.data.filter_row.value.answer_id.value)'Length > 0;
                     when K_THINKTHEN_COMPLETE_SESSION_PACKET_RANK_AGGREGATE_V1 =>
                        Seen := Index (P.data.rank_aggregate.value.len) > 0;
                     when K_THINKTHEN_COMPLETE_SESSION_PACKET_FIND_AGGREGATE_V1 =>
                        Seen := Text (P.data.find_aggregate.value.answer_id.value)'Length > 0;
                     when K_THINKTHEN_COMPLETE_SESSION_PACKET_ANNOTATE_ROW_V1 =>
                        Seen := Text (P.data.annotate_row.value.answer_id.value)'Length > 0;
                     when K_THINKTHEN_COMPLETE_SESSION_PACKET_RECOGNIZE_AGGREGATE_V1 =>
                        declare
                           type Recognition_Access is access constant thinkthen_complete_recognition_v1;
                           Rows : array (1 .. Index (P.data.recognize_aggregate.value.len)) of Recognition_Access
                             with Import, Address => P.data.recognize_aggregate.value.data;
                        begin
                           for Row of Rows loop
                              Metadata (Row.meta);
                              Require (Row.value.kind = K_THINKTHEN_COMPLETE_RECOGNIZE_FIELDS_ENTITIES_V1, "recognition kind changed");
                              declare
                                 type Entity_Access is access constant thinkthen_complete_entity_v1;
                                 Entities : array (1 .. Index (Row.value.data.fields_entities.entities.len)) of Entity_Access
                                   with Import, Address => Row.value.data.fields_entities.entities.data;
                              begin
                                 for Entity of Entities loop
                                    Entity_Count := Entity_Count + 1;
                                    Require (Text (Entity.text) = "Maria Chen" and then Text (Entity.kind) = "person" and then
                                      Entity.start = 10 and then Entity.c_end = 20 and then Entity.length = 10 and then
                                      abs (Entity.strength - 0.9877) < 0.00001, "Unicode entity changed");
                                 end loop;
                              end;
                           end loop;
                           Seen := Seen or else Entity_Count > 0;
                        end;
                     when K_THINKTHEN_COMPLETE_SESSION_PACKET_RELATE_AGGREGATE_V1 =>
                        Seen := Text (P.data.relate_aggregate.value.answer_id.value)'Length > 0;
                     when others => null;
                  end case;
               end;
         end case;
      end loop;
      if not Terminal or else (Failure = Success and not Seen) then raise Program_Error with "missing typed packet"; end if;
   end Drain;
   -- Shared corpus typed call expressions are inserted here by the installed check.
   -- DECLARATIONS
begin
   if Ada.Command_Line.Argument_Count = 0 then
      -- CALLS
   else
      Thinkthen.Sessions.Calls.Decide (Owner, Failed); Finish (Owner); Drain (Backend);
   end if;
   declare
      Overflowed : Boolean := False;
   begin
      begin
         declare
            Ignored : constant Natural := Index (Interfaces.C.size_t'Last);
         begin
            Put_Line (Natural'Image (Ignored));
         end;
      exception
         when Representation_Overflow => Overflowed := True;
      end;
      if not Overflowed then raise Program_Error with "extent overflow accepted"; end if;
   end;
   declare
      use Interfaces.C.Strings;
      Expected : constant String := "a" & Character'Val (0) & "b";
      Bytes : chars_ptr := New_String (Expected);
   begin
      if Text ((data => Bytes, len => 3)) /= Expected or else
         Text ((data => Null_Ptr, len => 0)) /= "" then
         raise Program_Error with "counted text changed";
      end if;
      Free (Bytes);
   exception
      when others => Free (Bytes); raise;
   end;
   if State (0) /= Missing or State (1) /= Null_Value or State (2) /= Present then raise Program_Error with "presence conflated"; end if;
   Put_Line ("INSTALLED_ADA_NATIVE_SESSION_PASS");
end Native_Session;
