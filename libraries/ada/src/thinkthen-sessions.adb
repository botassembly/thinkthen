with Interfaces.C.Strings;
with System;
package body Thinkthen.Sessions is
   use Interfaces.C;
   use Interfaces.C.Strings;
   use Interfaces;
   use Thinkthen_Session_C;
   type Engine_Access is access all thinkthen_engine;
   type View_Access is access constant thinkthen_complete_session_packet_v1;
   procedure Check (Code : int) is
   begin
      if Code /= 0 then
         raise Native_Error with int'Image (Code) & ": " & Value (thinkthen_session_error_message);
      end if;
   end Check;
   procedure Start (Owner : in out Session; Request : Thinkthen.Requests.T_Request) is
      JSON : constant String := Thinkthen.Requests.Encode (Request);
      Bytes : chars_ptr := New_String (JSON);
      Surface : chars_ptr := New_String ("ada");
      Client : Engine_Access := Engine_Access (thinkthen_engine_new);
      Code : int;
   begin
      Close (Owner);
      if Client = null then
         raise Native_Error with "native engine construction failed";
      end if;
      Code := thinkthen_session_new_with_surface (Client, Bytes, size_t (JSON'Length),
                                                  Surface, 3, Owner.Handle'Address);
      Free (Bytes);
      Free (Surface);
      thinkthen_engine_free (Client);
      Client := null;
      Check (Code);
   exception
      when others =>
         Free (Bytes);
         Free (Surface);
         if Client /= null then thinkthen_engine_free (Client); end if;
         raise;
   end Start;
   procedure Push (Owner : in out Session; Item : Thinkthen.Requests.T_RequestSessionDescriptor;
                   Status : out Push_Status) is
      JSON : constant String := Thinkthen.Requests.Encode (Item);
      Bytes : chars_ptr := New_String (JSON);
      Native_Status : aliased Unsigned_32;
      Code : int;
   begin
      Code := thinkthen_session_try_push (Owner.Handle, Bytes, size_t (JSON'Length), Native_Status'Access);
      Free (Bytes);
      Check (Code);
      Status := Push_Status'Val (Native_Status);
   exception
      when others => Free (Bytes); raise;
   end Push;
   procedure Finish (Owner : in out Session) is
   begin
      Check (thinkthen_session_finish (Owner.Handle, Null_Ptr, 0));
   end Finish;
   procedure Finish (Owner : in out Session; Failure : Thinkthen.Requests.T_RequestReaderFailure) is
      JSON : constant String := Thinkthen.Requests.Encode (Failure);
      Bytes : chars_ptr := New_String (JSON);
      Code : int;
   begin
      Code := thinkthen_session_finish (Owner.Handle, Bytes, size_t (JSON'Length));
      Free (Bytes);
      Check (Code);
   exception
      when others => Free (Bytes); raise;
   end Finish;
   procedure Cancel (Owner : in out Session) is
   begin
      thinkthen_session_cancel (Owner.Handle);
   end Cancel;
   procedure Close (Owner : in out Session) is
   begin
      thinkthen_session_free (Owner.Handle);
      Owner.Handle := null;
   end Close;
   procedure Close (Value : in out Packet) is
   begin
      thinkthen_session_result_free (Value.Handle);
      Value.Handle := null;
   end Close;
   overriding procedure Finalize (Owner : in out Session) is
   begin
      Close (Owner);
   end Finalize;
   overriding procedure Finalize (Value : in out Packet) is
   begin
      Close (Value);
   end Finalize;
   procedure Try_Read (Owner : in out Session'Class; Value : in out Packet; Status : out Read_Status) is
      Native_Status : aliased Unsigned_32;
   begin
      Close (Value);
      Check (thinkthen_session_try_read (Owner.Handle, Native_Status'Access, Value.Handle'Address));
      Status := Read_Status'Val (Native_Status);
   end Try_Read;
   function View (Value : Packet) return access constant thinkthen_complete_session_packet_v1 is
      Result : aliased View_Access := null;
   begin
      Check (thinkthen_session_result_view (Value.Handle, Result'Address));
      return Result;
   end View;
   function Status (Value : Packet) return Error_Status is
      Packet_View : constant access constant thinkthen_complete_session_packet_v1 := View (Value);
   begin
      if Packet_View.kind /= K_THINKTHEN_COMPLETE_SESSION_PACKET_TERMINAL_V1 then
         return Success;
      end if;
      if State (Packet_View.data.terminal.failure.presence) /= Present then
         return Success;
      end if;
      case Packet_View.data.terminal.failure.value.error.kind.kind is
         when K_THINKTHEN_COMPLETE_FAILURE_KIND_USAGE_V1 => return Usage;
         when K_THINKTHEN_COMPLETE_FAILURE_KIND_BACKEND_V1 => return Backend;
         when K_THINKTHEN_COMPLETE_FAILURE_KIND_LOCAL_V1 => return Local;
         when K_THINKTHEN_COMPLETE_FAILURE_KIND_CANCELLED_V1 => return Cancelled;
         when K_THINKTHEN_COMPLETE_FAILURE_KIND_DEADLINE_V1 => return Deadline;
         when others => return Defect;
      end case;
   end Status;
   function State (Value : Unsigned_32) return Presence is
   begin
      return Presence'Val (Value);
   end State;
   function Index (Value : size_t) return Natural is
   begin
      if Value > size_t (Natural'Last) then
         raise Representation_Overflow with "native extent exceeds Ada Natural";
      end if;
      return Natural (Value);
   end Index;
   function Text (Value : thinkthen_complete_utf8_v1) return String is
      Length : constant Natural := Index (Value.len);

   begin
      if Length = 0 then return ""; end if;
      return Interfaces.C.Strings.Value (Value.data, size_t (Length));
   end Text;
end Thinkthen.Sessions;
