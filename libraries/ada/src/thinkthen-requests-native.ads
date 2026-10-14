-- Generated native carrier adaptation; do not edit.
with Ada.Finalization;
with System;
package Thinkthen.Requests.Native is
   type Arena is new Ada.Finalization.Limited_Controlled with private;
function Convert (Value : T_RequestCall; Storage : in out Arena) return System.Address;
function Convert (Value : T_RequestSessionDescriptor; Storage : in out Arena) return System.Address;
function Convert (Value : T_RequestReaderFailure; Storage : in out Arena) return System.Address;
function Convert (Value : T_EngineSettings; Storage : in out Arena) return System.Address;
private
   type Allocation;
   type Allocation_Access is access Allocation;
   type Allocation is record
      Address : System.Address;
      Next : Allocation_Access;
   end record;
   type Arena is new Ada.Finalization.Limited_Controlled with record
      Head : Allocation_Access := null;
   end record;
   overriding procedure Finalize (Storage : in out Arena);

end Thinkthen.Requests.Native;
