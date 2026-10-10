with Ada.Command_Line;
with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Ada.Text_IO; use Ada.Text_IO;
with Interfaces; use Interfaces;
with Interfaces.C;
with Interfaces.C.Strings;
with Thinkthen.Requests; use Thinkthen.Requests;
with Thinkthen.Sessions; use Thinkthen.Sessions;
with Thinkthen.Sessions.Calls;
with Thinkthen_Session_C; use Thinkthen_Session_C;
procedure Native_Session is
   Owner : Session;
   Value : Packet;
   Read : Read_Status;
   Seen : Boolean;
   Terminal : Boolean;
   procedure Drain (Failure : Error_Status := Success; Decision : Integer := -1; Located : Boolean := False) is
   begin
      Seen := False;
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
                     if Failure = Success and then P.data.terminal.facts.value.requests_sent = 0 then raise Program_Error with "missing sends"; end if;
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
                        if Located and then State (P.data.decide_row.value.source.presence) /= Present then raise Program_Error with "source missing"; end if;
                     when K_THINKTHEN_COMPLETE_SESSION_PACKET_CHOOSE_ROW_V1 =>
                        Seen := Text (P.data.choose_row.value.answer_id.value)'Length > 0;
                     when K_THINKTHEN_COMPLETE_SESSION_PACKET_TAG_ROW_V1 =>
                        Seen := Text (P.data.tag_row.value.answer_id.value)'Length > 0;
                     when K_THINKTHEN_COMPLETE_SESSION_PACKET_SCORE_ROW_V1 =>
                        Seen := Text (P.data.score_row.value.answer_id.value)'Length > 0;
                     when K_THINKTHEN_COMPLETE_SESSION_PACKET_FILTER_ROW_V1 =>
                        Seen := Text (P.data.filter_row.value.answer_id.value)'Length > 0;
                     when K_THINKTHEN_COMPLETE_SESSION_PACKET_RANK_AGGREGATE_V1 =>
                        Seen := Index (P.data.rank_aggregate.value.len) > 0;
                     when K_THINKTHEN_COMPLETE_SESSION_PACKET_FIND_AGGREGATE_V1 =>
                        Seen := Text (P.data.find_aggregate.value.answer_id.value)'Length > 0;
                     when K_THINKTHEN_COMPLETE_SESSION_PACKET_ANNOTATE_ROW_V1 =>
                        Seen := Text (P.data.annotate_row.value.answer_id.value)'Length > 0;
                     when K_THINKTHEN_COMPLETE_SESSION_PACKET_RECOGNIZE_AGGREGATE_V1 =>
                        Seen := Index (P.data.recognize_aggregate.value.len) > 0;
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
