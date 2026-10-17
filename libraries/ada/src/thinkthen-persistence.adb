-- Generated native usage status wrappers; do not edit.
with Interfaces.C;
with Thinkthen.Native_Sessions;
with Thinkthen.Session_C; use Thinkthen.Session_C;
package body Thinkthen.Persistence is
function Observe (Client : Engine; Finish : Boolean) return Observation is
State : aliased thinkthen_complete_usage_persistence_v1;
Advice : aliased thinkthen_complete_utf8_v1;
Code : Interfaces.C.int;
begin
if Finish then
Code := thinkthen_engine_finish_usage_status_v1 (Engine_Pointer (Client.Handle), State'Access, Advice'Access);
else
Code := thinkthen_engine_usage_persistence_v1 (Engine_Pointer (Client.Handle), State'Access, Advice'Access);
end if;
Thinkthen_Kinds.Check (Code);
return (State => Persistence_State (Thinkthen_Kinds.Persistence_State_For (State.kind)),
Advice => Ada.Strings.Unbounded.To_Unbounded_String (Thinkthen.Native_Sessions.Text (Advice)));
end Observe;
function Usage_Persistence (Client : Engine) return Observation is (Observe (Client, False));
function Finish_Usage_Status (Client : Engine) return Observation is (Observe (Client, True));
end Thinkthen.Persistence;
