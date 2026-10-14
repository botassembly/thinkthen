-- Generated native session ownership and projection helpers.
with Ada.Unchecked_Conversion;
with Thinkthen.Requests.Native;
with Thinkthen.Native_Values;
with Thinkthen_Kinds;
with Interfaces.C.Strings;
with System;
with Thinkthen.Sessions; use Thinkthen.Sessions;
with Thinkthen.Session_C;
package body Thinkthen.Native_Sessions is
   use Interfaces.C;
   use Interfaces.C.Strings;
   use Interfaces;
   use Thinkthen.Session_C;
   Default_Client : Engine;
   use type System.Address;
   type View_Access is access constant thinkthen_complete_session_packet_v1;
   procedure Check (Code : int) renames Thinkthen_Kinds.Check;
   procedure Start (Owner : in out Session; Request : Thinkthen.Requests.T_RequestCall; Function_Name : String) is
   begin
      Start (Default_Client, Owner, Request, Function_Name);
   end Start;
   procedure Start (Client : Engine; Owner : in out Session; Request : Thinkthen.Requests.T_RequestCall; Function_Name : String) is
      Storage : Thinkthen.Requests.Native.Arena;
      type Arguments_Access is access constant thinkthen_request_arguments_v1;
      function Pointer is new Ada.Unchecked_Conversion (System.Address, Arguments_Access);
      Arguments : constant Arguments_Access := Pointer (Thinkthen.Requests.Native.Convert (Request, Storage));
      Bytes : chars_ptr := New_String (Function_Name);
      Surface : chars_ptr := New_String ("ada");
      Code : int;
      New_Handle : aliased Thinkthen.Session_C.Session_Access := null;
   begin
      Code := thinkthen_session_new_typed_with_surface
        (Engine_Pointer (Client.Handle), (data => Bytes, len => Function_Name'Length), Arguments,
         (data => Surface, len => 3), New_Handle'Address);
      Free (Bytes);
      Free (Surface);
      Check (Code);
      Close (Owner);
      Owner.Handle := Session_Address (New_Handle);
   exception
      when others =>
         Free (Bytes);
         Free (Surface);
         if New_Handle /= null then thinkthen_session_free (New_Handle); end if;
         raise;
   end Start;
   procedure Push (Owner : in out Session; Item : Thinkthen.Requests.T_RequestSessionDescriptor;
                   Status : out Push_Status) is
      Storage : Thinkthen.Requests.Native.Arena;
      type Descriptor_Access is access constant thinkthen_request_session_descriptor_v1;
      function Pointer is new Ada.Unchecked_Conversion (System.Address, Descriptor_Access);
      Descriptor : constant Descriptor_Access := Pointer (Thinkthen.Requests.Native.Convert (Item, Storage));
      Native_Status : aliased Unsigned_32;
   begin
      Check (thinkthen_session_push_typed (Session_Pointer (Owner.Handle), Descriptor, Native_Status'Access));
      Status := Push_Status'Val (Native_Status);
   end Push;
   procedure Finish (Owner : in out Session) is
   begin
      Check (thinkthen_session_finish (Session_Pointer (Owner.Handle), Null_Ptr, 0));
   end Finish;
   procedure Finish (Owner : in out Session; Failure : Thinkthen.Requests.T_RequestReaderFailure) is
      Storage : Thinkthen.Requests.Native.Arena;
      type Failure_Access is access constant thinkthen_request_reader_failure_v1;
      function Pointer is new Ada.Unchecked_Conversion (System.Address, Failure_Access);
      Descriptor : constant Failure_Access := Pointer (Thinkthen.Requests.Native.Convert (Failure, Storage));
   begin
      Check (thinkthen_session_finish_typed (Session_Pointer (Owner.Handle), Descriptor));
   end Finish;
   procedure Cancel (Owner : in out Session) is
   begin
      thinkthen_session_cancel (Session_Pointer (Owner.Handle));
   end Cancel;
   procedure Close (Owner : in out Session) is
   begin
      thinkthen_session_free (Session_Pointer (Owner.Handle));
      Owner.Handle := System.Null_Address;
   end Close;
   procedure Close (Value : in out Packet) is
   begin
      thinkthen_session_result_free (Packet_Pointer (Value.Handle));
      Value.Handle := System.Null_Address;
   end Close;
   procedure Wait (Owner : Session'Class; Interest : Unsigned_32;
                   Timeout_Ms : Unsigned_32; Ready : out Unsigned_32) is
      Native_Ready : aliased Unsigned_32;
   begin
      Check (thinkthen_session_wait (Session_Pointer (Owner.Handle), Interest, Timeout_Ms, Native_Ready'Access));
      Ready := Native_Ready;
   end Wait;
   procedure Next_Packet (Owner : in out Session'Class; Value : in out Packet; Status : out Read_Status;
                          Interest : Unsigned_32 := K_THINKTHEN_SESSION_WAIT_SETTLEMENT_V1) is
      Native_Status : aliased Unsigned_32;
      New_Handle : aliased Thinkthen.Session_C.Packet_Access := null;
   begin
      Check (thinkthen_session_next_packet (Session_Pointer (Owner.Handle), Interest, Native_Status'Access, New_Handle'Address));
      Close (Value);
      Value.Handle := Packet_Address (New_Handle);
      Status := Read_Status'Val (Native_Status);
   end Next_Packet;
   procedure Try_Read (Owner : in out Session'Class; Value : in out Packet; Status : out Read_Status) is
      Native_Status : aliased Unsigned_32;
      New_Handle : aliased Thinkthen.Session_C.Packet_Access := null;
   begin
      Check (thinkthen_session_try_read (Session_Pointer (Owner.Handle), Native_Status'Access, New_Handle'Address));
      Close (Value);
      Value.Handle := Packet_Address (New_Handle);
      Status := Read_Status'Val (Native_Status);
   end Try_Read;
   function View (Value : Packet) return access constant thinkthen_complete_session_packet_v1 is
      Result : aliased View_Access := null;
   begin
      Check (thinkthen_session_result_view (Packet_Pointer (Value.Handle), Result'Address));
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
      return Error_Status'Val (Thinkthen_Kinds.Error_Status'Pos
        (Thinkthen_Kinds.Failure_Kind_For (Packet_View.data.terminal.failure.value.error.kind.kind)));
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
      if Value.data = Null_Ptr then raise Dereference_Error; end if;
      declare
         function Address_Of is new Ada.Unchecked_Conversion (chars_ptr, System.Address);
         Bytes : char_array (0 .. size_t (Length) - 1)
           with Import, Address => Address_Of (Value.data);
      begin
         return To_Ada (Bytes, Trim_Nul => False);
      end;
   end Text;
function Plan (Client : Engine; Request : Thinkthen.Requests.T_RequestCall; Function_Name : String) return Thinkthen.Values.Value is
use Thinkthen.Session_C; use Interfaces.C; use Interfaces.C.Strings;
Storage : Thinkthen.Requests.Native.Arena;
type Arguments_Access is access constant thinkthen_request_arguments_v1;
function Pointer is new Ada.Unchecked_Conversion (System.Address, Arguments_Access);
Arguments : constant Arguments_Access := Pointer (Thinkthen.Requests.Native.Convert (Request, Storage));
Function_Bytes : chars_ptr := Null_Ptr; Surface : chars_ptr := Null_Ptr;
type Value_Access is access all thinkthen_value; Output : aliased Value_Access := null; View : aliased access constant thinkthen_complete_json_v1; Code : int;
begin
Function_Bytes := New_String (Function_Name); Surface := New_String ("ada");
Code := thinkthen_request_plan_typed_value (Engine_Pointer (Client.Handle), (data => Function_Bytes, len => Function_Name'Length), Arguments, (data => Surface, len => 3), Output'Address);
Free (Function_Bytes); Free (Surface);
Thinkthen_Kinds.Check (Code);
Thinkthen_Kinds.Check (thinkthen_value_view (Output, View'Address));
declare Result : constant Thinkthen.Values.Value := Thinkthen.Native_Values.Read (View); begin thinkthen_value_free (Output); Output := null; return Result; end;
exception when others => Free (Function_Bytes); Free (Surface); thinkthen_value_free (Output); raise;
end Plan;
end Thinkthen.Native_Sessions;
