with Ada.Finalization;
with Ada.Strings.Unbounded;
with Thinkthen_Session_C;
package Thinkthen is
   type Error_Kind is (None, Usage, Backend, Deadline, Local, Cancelled, Defect);
   type Failure is record
      Kind : Error_Kind := None;
      Retryable : Boolean := False;
      Text : Ada.Strings.Unbounded.Unbounded_String;
   end record;
   function Message (Error : Failure) return String;
   type Engine is new Ada.Finalization.Limited_Controlled with private;
   -- Native Rust admits settings. Failed replacement preserves the current engine.
   procedure Configure (Client : in out Engine; Settings_JSON : String; Error : out Failure);
private
   type Engine_Access is access all Thinkthen_Session_C.thinkthen_engine;
   type Engine is new Ada.Finalization.Limited_Controlled with record
      Handle : Engine_Access := null;
   end record;
   overriding procedure Initialize (Client : in out Engine);
   overriding procedure Finalize (Client : in out Engine);
end Thinkthen;
