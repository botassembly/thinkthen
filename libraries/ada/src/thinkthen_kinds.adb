-- Generated native kinds and diagnostics; do not edit.
with Interfaces.C.Strings;
with Thinkthen_Session_C; use Thinkthen_Session_C;
package body Thinkthen_Kinds is
use type Interfaces.C.int;
procedure Check (Code : Interfaces.C.int) is
begin
if Code = 0 then return; end if;
declare Message : constant String := Interfaces.C.Strings.Value (thinkthen_session_error_message);
begin case Code is
when K_THINKTHEN_EUSAGE => raise Usage_Error with Message;
when K_THINKTHEN_EBACKEND => raise Backend_Error with Message;
when K_THINKTHEN_EDEADLINE => raise Deadline_Error with Message;
when K_THINKTHEN_ELOCAL => raise Local_Error with Message;
when K_THINKTHEN_ECANCELLED => raise Cancelled_Error with Message;
when K_THINKTHEN_EDEFECT => raise Defect_Error with Message;
when others => raise Native_Error with Message;
end case; end; end Check;
function Failure_Kind_For (Kind : Interfaces.Unsigned_32) return Error_Status is
(case Kind is
when K_THINKTHEN_COMPLETE_FAILURE_KIND_USAGE_V1 => Usage,
when K_THINKTHEN_COMPLETE_FAILURE_KIND_BACKEND_V1 => Backend,
when K_THINKTHEN_COMPLETE_FAILURE_KIND_DEADLINE_V1 => Deadline,
when K_THINKTHEN_COMPLETE_FAILURE_KIND_LOCAL_V1 => Local,
when K_THINKTHEN_COMPLETE_FAILURE_KIND_CANCELLED_V1 => Cancelled,
when K_THINKTHEN_COMPLETE_FAILURE_KIND_DEFECT_V1 => Defect,
when others => Defect);
function Persistence_State_For (Kind : Interfaces.Unsigned_32) return Persistence_State is
(case Kind is
when K_THINKTHEN_COMPLETE_USAGE_PERSISTENCE_DISABLED_V1 => Disabled,
when K_THINKTHEN_COMPLETE_USAGE_PERSISTENCE_PENDING_V1 => Pending,
when K_THINKTHEN_COMPLETE_USAGE_PERSISTENCE_WRITTEN_V1 => Written,
when K_THINKTHEN_COMPLETE_USAGE_PERSISTENCE_FAILED_V1 => Failed,
when others => raise Program_Error with "unknown native persistence state");
end Thinkthen_Kinds;
