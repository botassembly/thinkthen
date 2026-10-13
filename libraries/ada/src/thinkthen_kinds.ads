-- Generated native kinds and diagnostics; do not edit.
with Interfaces;
with Interfaces.C;
package Thinkthen_Kinds is
type Error_Kind is (None, Usage, Backend, Deadline, Local, Cancelled, Defect);
for Error_Kind use (None => 0, Usage => 1, Backend => 2, Deadline => 3, Local => 4, Cancelled => 5, Defect => 6);
type Error_Status is (Success, Usage, Backend, Deadline, Local, Cancelled, Defect);
type Persistence_State is (Disabled, Pending, Written, Failed);
Native_Error : exception;
Usage_Error : exception;
Backend_Error : exception;
Deadline_Error : exception;
Local_Error : exception;
Cancelled_Error : exception;
Defect_Error : exception;
procedure Check (Code : Interfaces.C.int);
function Failure_Kind_For (Kind : Interfaces.Unsigned_32) return Error_Status;
function Persistence_State_For (Kind : Interfaces.Unsigned_32) return Persistence_State;
end Thinkthen_Kinds;
