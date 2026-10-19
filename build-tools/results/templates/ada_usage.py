"""Generate native error projections and the Ada usage status wrappers."""
import re


def outputs(header):
    constants = {key: int(value) for key, value in re.findall(r'^#define (THINKTHEN_\w+) (\d+)$', header, re.M)}
    errors = sorted((value, key.removeprefix('THINKTHEN_E').title(), key) for key, value in constants.items() if key.startswith('THINKTHEN_E'))
    persistence = sorted((value, key.removeprefix('THINKTHEN_COMPLETE_USAGE_PERSISTENCE_').removesuffix('_V1').title(), key) for key, value in constants.items() if key.startswith('THINKTHEN_COMPLETE_USAGE_PERSISTENCE_'))
    error_names = ['None'] + [name for _, name, _ in errors]
    kinds_spec = '''-- Generated native kinds and diagnostics; do not edit.
with Interfaces;
with Interfaces.C;
package Thinkthen_Kinds is
''' + 'type Error_Kind is (' + ', '.join(error_names) + ');\n' + 'for Error_Kind use (None => 0, ' + ', '.join(f'{name} => {value}' for value, name, _ in errors) + ');\n'
    kinds_spec += 'type Error_Status is (' + ', '.join(['Success'] + [name for _, name, _ in errors]) + ');\n'
    kinds_spec += 'type Persistence_State is (' + ', '.join(name for _, name, _ in persistence) + ');\n'
    kinds_spec += 'Native_Error : exception;\n' + '\n'.join(name + '_Error : exception;' for _, name, _ in errors) + '\n'
    kinds_spec += '''procedure Check (Code : Interfaces.C.int);
function Failure_Kind_For (Kind : Interfaces.Unsigned_32) return Error_Status;
function Persistence_State_For (Kind : Interfaces.Unsigned_32) return Persistence_State;
end Thinkthen_Kinds;
'''
    kinds_body = '''-- Generated native kinds and diagnostics; do not edit.
with Interfaces.C.Strings;
package body Thinkthen_Kinds is
function thinkthen_session_error_message return Interfaces.C.Strings.chars_ptr with Import, Convention => C, External_Name => "thinkthen_session_error_message";
use type Interfaces.C.int;
procedure Check (Code : Interfaces.C.int) is
begin
if Code = 0 then return; end if;
declare Message : constant String := Interfaces.C.Strings.Value (thinkthen_session_error_message);
begin case Code is
''' + '\n'.join('when K_' + symbol + ' => raise ' + name + '_Error with Message;' for _, name, symbol in errors) + '\nwhen others => raise Native_Error with Message;\nend case; end; end Check;\n'
    kinds_body += 'function Failure_Kind_For (Kind : Interfaces.Unsigned_32) return Error_Status is\n(case Kind is\n' + ',\n'.join('when K_THINKTHEN_COMPLETE_FAILURE_KIND_' + name.upper() + '_V1 => ' + name for _, name, _ in errors) + ',\nwhen others => Defect);\n'
    kinds_body += 'function Persistence_State_For (Kind : Interfaces.Unsigned_32) return Persistence_State is\n(case Kind is\n' + ',\n'.join('when K_' + symbol + ' => ' + name for _, name, symbol in persistence) + ',\nwhen others => raise Program_Error with "unknown native persistence state");\nend Thinkthen_Kinds;\n'
    for symbol, value in constants.items():
        kinds_body = re.sub(r'\bK_' + symbol + r'\b', str(value), kinds_body)
    usage_spec = '''-- Generated native usage status wrappers; do not edit.
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
'''
    usage_body = '''-- Generated native usage status wrappers; do not edit.
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
'''
    return {'thinkthen_kinds.ads': kinds_spec, 'thinkthen_kinds.adb': kinds_body,
            'thinkthen-persistence.ads': usage_spec, 'thinkthen-persistence.adb': usage_body}
