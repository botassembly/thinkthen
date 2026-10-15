with Ada.Exceptions;
with Thinkthen.Sessions; use Thinkthen.Sessions;
with Thinkthen.Inputs.Calls;
with Thinkthen.Inputs.Native;
with Thinkthen.Answers;
with Thinkthen.Values;
with Ada.Streams.Stream_IO;
with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Ada.Text_IO; use Ada.Text_IO;
with Interfaces; use type Interfaces.Unsigned_32;
with Interfaces.C; use Interfaces.C;
with Interfaces.C.Strings; use Interfaces.C.Strings;
with Thinkthen.Requests; use Thinkthen.Requests;
with Thinkthen.Native_Sessions; use Thinkthen.Native_Sessions;
with System;
with Thinkthen.Session_C; use Thinkthen.Session_C;
procedure Thinkthen.Sessions.Parity_Case is
   function thinkthen_session_result_json (Result : access constant thinkthen_session_result;
     Bytes : System.Address; Count : access Interfaces.C.size_t) return Interfaces.C.int
     with Import, Convention => C, External_Name => "thinkthen_session_result_json";
   procedure Configure (Client : in out Engine; Settings_JSON : String; Error : out Failure) is
      Settings : chars_ptr := Null_Ptr;
      Replacement : Engine_Access;
      Code : int;
   begin
      Settings := New_String (Settings_JSON);
      Replacement := Engine_Access (thinkthen_engine_new_with (Settings));
      Free (Settings);
      if Replacement = null then
         Code := thinkthen_error_code (null);
         if Code not in 1 .. 6 then raise Program_Error with "unknown native error kind"; end if;
         Error := (Kind => Error_Kind'Val (Code),
                   Retryable => thinkthen_error_retryable (null) /= 0,
                   Text => To_Unbounded_String (Interfaces.C.Strings.Value (thinkthen_error_message (null))));
      else
         thinkthen_engine_free (Engine_Pointer (Client.Handle));
         Client.Handle := Engine_Address (Replacement);
         Error := (others => <>);
      end if;
   exception
      when others => Free (Settings); raise;
   end Configure;
   Client : Engine;
   Owner : Session;
   Value : Packet;
   Read : Read_Status;
   Error : Failure;
   Packets : Unbounded_String := Ada.Strings.Unbounded.To_Unbounded_String ("{""packets"": [");
   First : Boolean := True;
   function Quoted (Text : String) return String is
      Output : Unbounded_String := To_Unbounded_String ("""");
      Hex : constant String := "0123456789abcdef";
   begin
      for C of Text loop
         case C is
            when '"' => Append (Output, "\""");
            when '\' => Append (Output, "\\");
            when Character'Val (0) .. Character'Val (31) =>
               Append (Output, "\u00" & Hex (Character'Pos (C) / 16 + 1)
                       & Hex (Character'Pos (C) mod 16 + 1));
            when others => Append (Output, C);
         end case;
      end loop;
      Append (Output, '"');
      return To_String (Output);
   end Quoted;
   procedure Admission (Code : Natural; Message : String) is
   begin
      Put_Line ("{""admission"":{""code"":" & Natural'Image (Code) & ",""message"":" & Quoted (Message) & "}}");
   end Admission;
   procedure Drain_One is
   begin
      Thinkthen.Native_Sessions.Next_Packet (Owner, Value, Read);
      case Read is
         when Pending => raise Program_Error with "blocking read returned pending";
         when Finished => null;
         when Result =>
            declare
               Bytes : aliased chars_ptr := Null_Ptr;
               Count : aliased size_t := 0;
               Code : constant int := thinkthen_session_result_json (Packet_Pointer (Value.Handle), Bytes'Address, Count'Access);
            begin
               if Code /= 0 then raise Program_Error with "packet serialization refused"; end if;
               if not First then Append (Packets, ','); end if;
               First := False;
               Append (Packets, Thinkthen.Native_Sessions.Text ((data => Bytes, len => Count)));
            end;
      end case;
   end Drain_One;
   function Fixture_Text (Path : String) return Unbounded_String is
      use Ada.Streams;
      File : Ada.Streams.Stream_IO.File_Type;
      Bytes : Stream_Element_Array (1 .. 8192);
      Last : Stream_Element_Offset;
      Value : Unbounded_String;
   begin
      Ada.Streams.Stream_IO.Open (File, Ada.Streams.Stream_IO.In_File, Path);
      while not Ada.Streams.Stream_IO.End_Of_File (File) loop
         Ada.Streams.Stream_IO.Read (File, Bytes, Last);
         declare
            Chunk : String (1 .. Integer (Last));
         begin
            for Index in Chunk'Range loop
               Chunk (Index) := Character'Val (Bytes (Stream_Element_Offset (Index)));
            end loop;
            Append (Value, Chunk);
         end;
      end loop;
      Ada.Streams.Stream_IO.Close (File);
      return Value;
   end Fixture_Text;
   -- REQUEST
   function JSON (Item : Thinkthen.Values.Value) return String is
      use Thinkthen.Values;
      Output : Unbounded_String;
   begin
      case Kind (Item) is
         when Null_Value => return "null";
         when Thinkthen.Values.Boolean_Value => return (if As_Boolean (Item) then "true" else "false");
         when Number_Value => return As_Number (Item);
         when String_Value => return Quoted (As_String (Item));
         when Array_Value | Object_Value =>
            Append (Output, (if Kind (Item) = Array_Value then '[' else '{'));
            for I in 1 .. Length (Item) loop
               if I > 1 then Append (Output, ','); end if;
               if Kind (Item) = Object_Value then
                  Append (Output, Quoted (Field_Name (Item, I)) & ':' & JSON (Field_Value (Item, Field_Name (Item, I))));
               else
                  Append (Output, JSON (Element (Item, I)));
               end if;
            end loop;
            Append (Output, (if Kind (Item) = Array_Value then ']' else '}'));
            return To_String (Output);
      end case;
   end JSON;
   procedure Public_Call is
      use Thinkthen.Answers;
      use Thinkthen.Values;
      Verb : constant String := Get_Line (Input);
      Request : constant T_RequestCall := -- PUBLIC REQUEST;
      Question : constant Thinkthen.Inputs.Question := Thinkthen.Inputs.Native.Wrap (Request.T_question);
      Original : constant Thinkthen.Inputs.Input_text := Thinkthen.Inputs.As_text (Thinkthen.Inputs.Native.Wrap (Request.T_input));
      Options : constant Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.Native.Wrap (Request.T_options);
      procedure Emit (Value : Thinkthen.Values.Value; Probability : Number_Field; Results : Receipt_List; Facts : Thinkthen.Answers.Facts) is
         Rows : Unbounded_String := To_Unbounded_String ("[");
         Packets : Unbounded_String := To_Unbounded_String ("[");
         Facts_JSON : constant String := "{""requests_sent"":" & Interfaces.Unsigned_64'Image (Requests_Sent (Facts)) & ",""call_id"":" & Quoted (Call_Id (Facts)) & "}";
      begin
         for I in Results'Range loop
            if I > Results'First then Append (Rows, ','); Append (Packets, ','); end if;
            Append (Rows, JSON (Record_Value (Results (I))));
            Append (Packets, "{""kind"":""row"",""value"":" & JSON (Record_Value (Results (I))) & "}");
         end loop;
         Append (Rows, ']');
         if Results'Length > 0 then Append (Packets, ','); end if;
         Append (Packets, "{""kind"":""terminal"",""facts"":" & Facts_JSON & "}]");
         Put_Line ("{""packets"":" & To_String (Packets) & ",""call_result"":{""value"":" & JSON (Value) & ",""probability"":" & (if Probability.State = Present_Field then Long_Float'Image (Probability.Value) else "null") & ",""results"":" & To_String (Rows) & ",""facts"":" & Facts_JSON & "}}");
      end Emit;
   begin
      if Verb = "decide" then
         declare Item : constant Decide_Result := Thinkthen.Inputs.Calls.Decide (Client, Question, Original, Options); begin
            Emit (Item.Value, Item.Probability, Item.Results, Item.Facts);
         end;
      elsif Verb = "choose" then
         declare Item : constant Choose_Result := Thinkthen.Inputs.Calls.Choose (Client, Question, Original, Options); begin
            Emit ((if Item.Value.Known then String_Item (To_String (Item.Value.Value)) else Null_Item), Item.Probability, Item.Results, Item.Facts);
         end;
      else
         raise Program_Error with "unexpected call result function";
      end if;
   end Public_Call;
begin
   Configure (Client, -- SETTINGS
              Error);
   if Error.Kind /= None then Admission (Error_Kind'Pos (Error.Kind), Message (Error)); return; end if;
   Thinkthen.Sessions.Calls.CALL_NAME (Client, Owner, Request);
   -- CANCEL
   declare
      Count : constant Natural := Natural'Value (Get_Line (Input));
      type Feed_List is array (Positive range <>) of T_RequestSessionDescriptor;
      function Read_Feed return Feed_List is
         Items : Feed_List (1 .. Count);
      begin
         for Item of Items loop Item := -- FEED ITEM; end loop;
         return Items;
      end Read_Feed;
      Items : constant Feed_List := Read_Feed;
      Held : constant Boolean := Boolean'Value (Get_Line (Input));
      Producer_Failure : Unbounded_String;
   begin
      declare
         -- Blocking pushes require a concurrent reader when output fills.
         task Producer;
         task body Producer is
            Status : Push_Status;
         begin
            for Item of Items loop
               Thinkthen.Native_Sessions.Push (Owner, Item, Status);
               if Status /= Accepted then raise Program_Error with "blocking feed refused"; end if;
            end loop;
            Thinkthen.Native_Sessions.Finish (Owner);
         exception
            when E : others =>
               Producer_Failure := To_Unbounded_String (Ada.Exceptions.Exception_Information (E));
               Thinkthen.Native_Sessions.Cancel (Owner);
         end Producer;
         task Canceller;
         task body Canceller is
         begin
            if HELD_CANCEL then
               declare Signal : constant String := Get_Line; begin null; end;
               Thinkthen.Native_Sessions.Cancel (Owner); Put_Line ("cancel-fired"); Flush;
            end if;
         end Canceller;
      begin
         loop
            Drain_One;
            exit when Read = Finished or else
              (Read = Result and then View (Value).kind = K_THINKTHEN_COMPLETE_SESSION_PACKET_TERMINAL_V1);
         end loop;
      exception
         when others => Thinkthen.Native_Sessions.Cancel (Owner); raise;
      end;
      -- The task scope joins both workers before releasing the session.
      Thinkthen.Native_Sessions.Close (Owner);
      if Length (Producer_Failure) > 0 then
         raise Program_Error with To_String (Producer_Failure);
      end if;
   end;
   Append (Packets, "]}"); Put_Line (Ada.Strings.Unbounded.To_String (Packets));
exception
   when E : Usage_Error => Admission (1, Ada.Exceptions.Exception_Message (E));
   when E : Backend_Error => Admission (2, Ada.Exceptions.Exception_Message (E));
   when E : Deadline_Error => Admission (3, Ada.Exceptions.Exception_Message (E));
   when E : Local_Error => Admission (4, Ada.Exceptions.Exception_Message (E));
   when E : Cancelled_Error => Admission (5, Ada.Exceptions.Exception_Message (E));
   when E : Defect_Error => Admission (6, Ada.Exceptions.Exception_Message (E));
end Thinkthen.Sessions.Parity_Case;
