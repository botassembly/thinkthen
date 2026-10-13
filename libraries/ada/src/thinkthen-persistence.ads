-- Generated native usage status wrappers; do not edit.
with Ada.Strings.Unbounded;
with Thinkthen_Kinds;
package Thinkthen.Persistence is
type Persistence_State is new Thinkthen_Kinds.Persistence_State;
type Observation is record
State : Persistence_State := Disabled;
Advice : Ada.Strings.Unbounded.Unbounded_String;
end record;
function Usage_Persistence (Client : Engine) return Observation;
function Finish_Usage_Status (Client : Engine) return Observation;
end Thinkthen.Persistence;
