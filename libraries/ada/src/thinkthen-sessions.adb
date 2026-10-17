with Thinkthen.Native_Sessions;
package body Thinkthen.Sessions is
   procedure Finish (Owner : in out Session) renames Thinkthen.Native_Sessions.Finish;
   procedure Cancel (Owner : in out Session) renames Thinkthen.Native_Sessions.Cancel;
   procedure Close (Owner : in out Session) renames Thinkthen.Native_Sessions.Close;
   procedure Next_Packet (Owner : in out Session'Class; Value : in out Packet; Status : out Read_Status) is
   begin Thinkthen.Native_Sessions.Next_Packet (Owner, Value, Status); end Next_Packet;
   procedure Try_Read (Owner : in out Session'Class; Value : in out Packet; Status : out Read_Status) renames Thinkthen.Native_Sessions.Try_Read;
   procedure Close (Value : in out Packet) renames Thinkthen.Native_Sessions.Close;
   function Status (Value : Packet) return Error_Status renames Thinkthen.Native_Sessions.Status;
end Thinkthen.Sessions;
