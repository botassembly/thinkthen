with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Interfaces.C; use Interfaces.C;
with Interfaces.C.Strings; use Interfaces.C.Strings;
with Thinkthen_Session_C; use Thinkthen_Session_C;
package body Thinkthen is
   function Message (Error : Failure) return String is (To_String (Error.Text));
   overriding procedure Initialize (Client : in out Engine) is
   begin
      Client.Handle := Engine_Access (thinkthen_engine_new);
      if Client.Handle = null then
         raise Program_Error with Value (thinkthen_error_message (null));
      end if;
   end Initialize;
   procedure Configure (Client : in out Engine; Settings_JSON : String; Error : out Failure) is
      Settings : chars_ptr := Null_Ptr;
      Replacement : Engine_Access;
      Code : int;
   begin
      for C of Settings_JSON loop
         if C = Character'Val (0) then raise Constraint_Error with "NUL in C string"; end if;
      end loop;
      Settings := New_String (Settings_JSON);
      Replacement := Engine_Access (thinkthen_engine_new_with (Settings));
      Free (Settings);
      if Replacement = null then
         Code := thinkthen_error_code (null);
         if Code not in 1 .. 6 then raise Program_Error with "unknown native error kind"; end if;
         Error := (Kind => Error_Kind'Val (Code),
                   Retryable => thinkthen_error_retryable (null) /= 0,
                   Text => To_Unbounded_String (Value (thinkthen_error_message (null))));
      else
         thinkthen_engine_free (Client.Handle);
         Client.Handle := Replacement;
         Error := (others => <>);
      end if;
   exception
      when others => Free (Settings); raise;
   end Configure;
   overriding procedure Finalize (Client : in out Engine) is
   begin
      thinkthen_engine_free (Client.Handle);
      Client.Handle := null;
   end Finalize;
end Thinkthen;
