with Ada.Finalization;
with Interfaces;
with Interfaces.C;
with Thinkthen.Requests;
with Thinkthen_Session_C;
with Thinkthen_Kinds;
package Thinkthen.Sessions is
   type Session is new Ada.Finalization.Limited_Controlled with private;
   type Packet is new Ada.Finalization.Limited_Controlled with private;
   type Read_Status is (Result, Pending, Finished);
   type Push_Status is (Accepted, Full, Closed);
   type Presence is (Missing, Null_Value, Present);
   type Error_Status is new Thinkthen_Kinds.Error_Status;
   Native_Error : exception renames Thinkthen_Kinds.Native_Error;
   Usage_Error : exception renames Thinkthen_Kinds.Usage_Error;
   Backend_Error : exception renames Thinkthen_Kinds.Backend_Error;
   Deadline_Error : exception renames Thinkthen_Kinds.Deadline_Error;
   Local_Error : exception renames Thinkthen_Kinds.Local_Error;
   Cancelled_Error : exception renames Thinkthen_Kinds.Cancelled_Error;
   Defect_Error : exception renames Thinkthen_Kinds.Defect_Error;
   Representation_Overflow : exception;
   procedure Start (Client : Engine; Owner : in out Session; Request : Thinkthen.Requests.T_RequestCall; Function_Name : String);
   procedure Start (Owner : in out Session; Request : Thinkthen.Requests.T_RequestCall; Function_Name : String);
   procedure Push (Owner : in out Session; Item : Thinkthen.Requests.T_RequestSessionDescriptor;
                   Status : out Push_Status);
   procedure Finish (Owner : in out Session);
   procedure Finish (Owner : in out Session; Failure : Thinkthen.Requests.T_RequestReaderFailure);
   procedure Cancel (Owner : in out Session);
   procedure Close (Owner : in out Session);
   -- Wait consumes no packet. Join every waiting task before Close/Finalize.
   -- Generated native constants name Result/Input/Settlement interests and readiness.
   procedure Wait (Owner : Session'Class; Interest : Interfaces.Unsigned_32;
                   Timeout_Ms : Interfaces.Unsigned_32; Ready : out Interfaces.Unsigned_32);
   -- Block until one packet arrives or settlement completes.
   procedure Next_Packet (Owner : in out Session'Class; Value : in out Packet; Status : out Read_Status;
                          Interest : Interfaces.Unsigned_32 := Thinkthen_Session_C.K_THINKTHEN_SESSION_WAIT_SETTLEMENT_V1);
   procedure Try_Read (Owner : in out Session'Class; Value : in out Packet; Status : out Read_Status);
   procedure Close (Value : in out Packet);
   -- Every nested pointer shares Value's lifetime. Join tasks before closing owners.
   function View (Value : Packet) return access constant Thinkthen_Session_C.thinkthen_complete_session_packet_v1;
   function Status (Value : Packet) return Error_Status;
   function State (Value : Interfaces.Unsigned_32) return Presence;
   function Text (Value : Thinkthen_Session_C.thinkthen_complete_utf8_v1) return String;
   -- Bound native extents before converting them to Ada indexing or String bounds.
   function Index (Value : Interfaces.C.size_t) return Natural;
private
   type Session_Access is access all Thinkthen_Session_C.thinkthen_session;
   type Packet_Access is access all Thinkthen_Session_C.thinkthen_session_result;
   type Session is new Ada.Finalization.Limited_Controlled with record
      Handle : aliased Session_Access := null;
   end record;
   type Packet is new Ada.Finalization.Limited_Controlled with record
      Handle : aliased Packet_Access := null;
   end record;
   overriding procedure Finalize (Owner : in out Session);
   overriding procedure Finalize (Value : in out Packet);
end Thinkthen.Sessions;
