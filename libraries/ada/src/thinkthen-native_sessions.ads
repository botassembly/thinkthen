-- Generated native session ownership and projection helpers.
with Thinkthen.Sessions;
with Thinkthen.Values;
with Thinkthen.Requests;
with Thinkthen.Session_C;
with Interfaces.C;
private package Thinkthen.Native_Sessions is
   procedure Start (Client : Engine; Owner : in out Session_Owner; Request : Thinkthen.Requests.T_RequestCall; Function_Name : String);
   procedure Start (Owner : in out Session_Owner; Request : Thinkthen.Requests.T_RequestCall; Function_Name : String);
   procedure Push (Owner : in out Session_Owner; Item : Thinkthen.Requests.T_RequestSessionDescriptor;
                   Status : out Thinkthen.Sessions.Push_Status);
   procedure Finish (Owner : in out Session_Owner);
   procedure Finish (Owner : in out Session_Owner; Failure : Thinkthen.Requests.T_RequestReaderFailure);
   procedure Cancel (Owner : in out Session_Owner);
   procedure Close (Owner : in out Session_Owner);
   -- Wait consumes no packet. Join every waiting task before Close/Finalize.
   -- Generated native constants name Result/Input/Settlement interests and readiness.
   procedure Wait (Owner : Session_Owner'Class; Interest : Interfaces.Unsigned_32;
                   Timeout_Ms : Interfaces.Unsigned_32; Ready : out Interfaces.Unsigned_32);
   -- Block until one packet arrives or settlement completes.
   procedure Next_Packet (Owner : in out Session_Owner'Class; Value : in out Packet_Owner; Status : out Thinkthen.Sessions.Read_Status;
                          Interest : Interfaces.Unsigned_32 := Thinkthen.Session_C.K_THINKTHEN_SESSION_WAIT_SETTLEMENT_V1);
   procedure Try_Read (Owner : in out Session_Owner'Class; Value : in out Packet_Owner; Status : out Thinkthen.Sessions.Read_Status);
   procedure Close (Value : in out Packet_Owner);
   -- Every nested pointer shares Value's lifetime. Join tasks before closing owners.
   function View (Value : Packet_Owner) return access constant Thinkthen.Session_C.thinkthen_complete_session_packet_v1;
   function Status (Value : Packet_Owner) return Thinkthen.Sessions.Error_Status;
   function State (Value : Interfaces.Unsigned_32) return Thinkthen.Sessions.Presence;
   function Text (Value : Thinkthen.Session_C.thinkthen_complete_utf8_v1) return String;
   -- Bound native extents before converting them to Ada indexing or String bounds.
   function Index (Value : Interfaces.C.size_t) return Natural;
function Plan (Client : Engine; Request : Thinkthen.Requests.T_RequestCall; Function_Name : String) return Thinkthen.Values.Value;
end Thinkthen.Native_Sessions;
