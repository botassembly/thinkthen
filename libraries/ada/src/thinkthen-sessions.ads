with Ada.Finalization;
with Interfaces;
with Interfaces.C;
with Thinkthen.Requests;
with Thinkthen_Session_C;
package Thinkthen.Sessions is
   type Session is new Ada.Finalization.Limited_Controlled with private;
   type Packet is new Ada.Finalization.Limited_Controlled with private;
   type Read_Status is (Result, Pending, Finished);
   type Push_Status is (Accepted, Full, Closed);
   type Presence is (Missing, Null_Value, Present);
   type Error_Status is (Success, Usage, Backend, Deadline, Local, Cancelled, Defect);
   for Error_Status use (Success => 0, Usage => 1, Backend => 2, Deadline => 3,
                        Local => 4, Cancelled => 5, Defect => 6);
   Native_Error : exception;
   Representation_Overflow : exception;
   procedure Start (Owner : in out Session; Request : Thinkthen.Requests.T_Request);
   procedure Push (Owner : in out Session; Item : Thinkthen.Requests.T_RequestSessionDescriptor;
                   Status : out Push_Status);
   procedure Finish (Owner : in out Session);
   procedure Finish (Owner : in out Session; Failure : Thinkthen.Requests.T_RequestReaderFailure);
   procedure Cancel (Owner : in out Session);
   procedure Close (Owner : in out Session);
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
