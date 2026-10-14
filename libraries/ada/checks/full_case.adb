with Ada.Exceptions;
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
               Append (Packets, Text ((data => Bytes, len => Count)));
               if View (Value).kind = K_THINKTHEN_COMPLETE_SESSION_PACKET_TERMINAL_V1 then
                  Thinkthen.Native_Sessions.Close (Owner);
               end if;
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
begin
   Configure (Client, -- SETTINGS
              Error);
   if Error.Kind /= None then Admission (Error_Kind'Pos (Error.Kind), Message (Error)); return; end if;
   Thinkthen.Sessions.Calls.CALL_NAME (Client, Owner, Request);
   -- CANCEL
   -- FEED
   Thinkthen.Native_Sessions.Finish (Owner);
   declare
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
