-- Generated native kinds and diagnostics; do not edit.
with Interfaces.C.Strings;
package body Thinkthen_Kinds is
function thinkthen_session_error_message return Interfaces.C.Strings.chars_ptr with Import, Convention => C, External_Name => "thinkthen_session_error_message";
use type Interfaces.C.int;
procedure Check (Code : Interfaces.C.int) is
begin
if Code = 0 then return; end if;
declare Message : constant String := Interfaces.C.Strings.Value (thinkthen_session_error_message);
begin case Code is
when 1 => raise Usage_Error with Message;
when 2 => raise Backend_Error with Message;
when 3 => raise Deadline_Error with Message;
when 4 => raise Local_Error with Message;
when 5 => raise Cancelled_Error with Message;
when 6 => raise Defect_Error with Message;
when others => raise Native_Error with Message;
end case; end; end Check;
function Failure_Kind_For (Kind : Interfaces.Unsigned_32) return Error_Status is
(case Kind is
when 1 => Usage,
when 2 => Backend,
when 5 => Deadline,
when 3 => Local,
when 4 => Cancelled,
when 6 => Defect,
when others => Defect);
function Persistence_State_For (Kind : Interfaces.Unsigned_32) return Persistence_State is
(case Kind is
when 1 => Disabled,
when 2 => Pending,
when 3 => Written,
when 4 => Failed,
when others => raise Program_Error with "unknown native persistence state");
end Thinkthen_Kinds;
