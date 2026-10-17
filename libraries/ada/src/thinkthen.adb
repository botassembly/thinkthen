with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Interfaces.C; use Interfaces.C;
with Interfaces.C.Strings; use Interfaces.C.Strings;
with Thinkthen.Session_C; use Thinkthen.Session_C;
with Thinkthen.Native_Sessions;
package body Thinkthen is
   use type System.Address;
   function Message (Error : Failure) return String is (To_String (Error.Text));
   overriding procedure Initialize (Client : in out Engine) is
   begin
      Client.Handle := Engine_Address (Engine_Access (thinkthen_engine_new));
      if Client.Handle = System.Null_Address then
         raise Program_Error with Value (thinkthen_error_message (null));
      end if;
   end Initialize;
   overriding procedure Finalize (Client : in out Engine) is
   begin
      thinkthen_engine_free (Engine_Pointer (Client.Handle));
      Client.Handle := System.Null_Address;
   end Finalize;
   overriding procedure Finalize (Owner : in out Session_Owner) is
   begin Thinkthen.Native_Sessions.Close (Owner); end Finalize;
   overriding procedure Finalize (Value : in out Packet_Owner) is
   begin Thinkthen.Native_Sessions.Close (Value); end Finalize;
end Thinkthen;
