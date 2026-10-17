with Ada.Finalization;
with Ada.Strings.Unbounded;
with System;
with Thinkthen_Kinds;
package Thinkthen is
   type Error_Kind is new Thinkthen_Kinds.Error_Kind;
   type Failure is record
      Kind : Error_Kind := None;
      Retryable : Boolean := False;
      Text : Ada.Strings.Unbounded.Unbounded_String;
   end record;
   function Message (Error : Failure) return String;
   type Engine is new Ada.Finalization.Limited_Controlled with private;
   -- Native Rust admits settings. Failed replacement preserves the current engine.
   type Session_Owner is new Ada.Finalization.Limited_Controlled with private;
   type Packet_Owner is new Ada.Finalization.Limited_Controlled with private;
private
   type Engine is new Ada.Finalization.Limited_Controlled with record
      Handle : System.Address := System.Null_Address;
   end record;
   type Session_Owner is new Ada.Finalization.Limited_Controlled with record
      Handle : System.Address := System.Null_Address;
   end record;
   type Packet_Owner is new Ada.Finalization.Limited_Controlled with record
      Handle : System.Address := System.Null_Address;
   end record;
   overriding procedure Finalize (Owner : in out Session_Owner);
   overriding procedure Finalize (Value : in out Packet_Owner);
   overriding procedure Initialize (Client : in out Engine);
   overriding procedure Finalize (Client : in out Engine);
end Thinkthen;
