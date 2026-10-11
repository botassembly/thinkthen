with Ada.Exceptions;
with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Ada.Text_IO; use Ada.Text_IO;
with Interfaces; use type Interfaces.Unsigned_32;
with Interfaces.C; use Interfaces.C;
with Interfaces.C.Strings; use Interfaces.C.Strings;
with Thinkthen.Requests; use Thinkthen.Requests;
with Thinkthen.Sessions.Calls;
with Thinkthen_Session_C; use Thinkthen_Session_C;
procedure Thinkthen.Sessions.Parity_Case is
   Client : Engine;
   Owner : Session;
   Value : Packet;
   Read : Read_Status;
   Error : Failure;
   Packets : Unbounded_String := Ada.Strings.Unbounded.To_Unbounded_String ("{""packets"": [");
   First : Boolean := True;
   function Quoted (Text : String) return String is
   begin
      return Encode (T_RequestQuestion_text_field_text (Ada.Strings.Unbounded.To_Unbounded_String (Text)));
   end Quoted;
   procedure Admission (Code : Natural; Message : String) is
   begin
      Put_Line ("{""admission"":{""code"":" & Natural'Image (Code) & ",""message"":" & Quoted (Message) & "}}");
   end Admission;
   procedure Drain_One is
   begin
      Try_Read (Owner, Value, Read);
      case Read is
         when Pending => delay 0.001;
         when Finished => null;
         when Result =>
            declare
               Bytes : aliased chars_ptr := Null_Ptr;
               Count : aliased size_t := 0;
               Code : constant int := thinkthen_session_result_json (Value.Handle, Bytes'Address, Count'Access);
            begin
               if Code /= 0 then raise Program_Error with "packet serialization refused"; end if;
               if not First then Append (Packets, ','); end if;
               First := False;
               Append (Packets, Text ((data => Bytes, len => Count)));
               if View (Value).kind = K_THINKTHEN_COMPLETE_SESSION_PACKET_TERMINAL_V1 then
                  Close (Owner);
               end if;
            end;
      end case;
   end Drain_One;
   -- REQUEST
begin
   Configure (Client, -- SETTINGS
              Error);
   if Error.Kind /= None then Admission (Error_Kind'Pos (Error.Kind), Message (Error)); return; end if;
   Thinkthen.Sessions.Calls.CALL_NAME (Client, Owner, Request);
   -- CANCEL
   -- FEED
   Finish (Owner);
   declare
      task Canceller;
      task body Canceller is
      begin
         if HELD_CANCEL then
            declare Signal : constant String := Get_Line; begin null; end;
            Cancel (Owner); Put_Line ("cancel-fired"); Flush;
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
