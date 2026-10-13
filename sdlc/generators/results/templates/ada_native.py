"""Convert idiomatic generated Ada values to the canonical counted C carriers."""
from templates.native_request import render as native_render, name
from templates.ada import identifier
from templates.csharp import enum_values


def render(definitions, names, schemas):
    metadata = {}
    native_render(definitions, metadata)
    pending = [('RequestCall', 'RequestArguments'), ('RequestSessionDescriptor', 'RequestSessionDescriptor'), ('RequestReaderFailure', 'RequestReaderFailure')]
    done, bodies, pointers = set(), [], set()
    def call(host, native):
        if (host, native) not in done and (host, native) not in pending:
            pending.append((host, native))
        return 'Convert_' + host + '_' + native
    while pending:
        host, native = pending.pop(0)
        if (host, native) in done:
            continue
        done.add((host, native))
        source, typ = schemas[host], name(native)
        pointers.add(typ)
        body, locals = [], []
        def child(value, path, host_path):
            host_key = value['$ref'].removeprefix('#/$defs/') if isinstance(value, dict) and '$ref' in value else host_path
            native_key = metadata['children'][(native, path)]
            pointers.add(name(native_key))
            return call(host_key, native_key), name(native_key)
        if '$ref' in source:
            target = source['$ref'].removeprefix('#/$defs/')
            body = ['return ' + call(target, target) + ' (Value, Storage);']
        elif 'properties' in source:
            if not source['properties']:
                body.append('Result.reserved := 0;')
            for member, value in source['properties'].items():
                if host == 'RequestCall' and member == 'function':
                    continue
                resolved = definitions[value['$ref'].removeprefix('#/$defs/')] if isinstance(value, dict) and '$ref' in value else value
                if isinstance(resolved, dict) and 'const' in resolved:
                    continue
                function, child_type = child(value, native + '_' + member, host + '_field_' + member)
                field = identifier(member)
                if member in source.get('required', []):
                    body.append(f'Result.m_{member} := P_{child_type} ({function} (Value.{field}, Storage));')
                else:
                    body.append(f'if Value.{field}.Present then Result.m_{member} := P_{child_type} ({function} (Value.{field}.Value, Storage)); else Result.m_{member} := null; end if;')
        elif enum_values(source):
            values = enum_values(source)
            body = [f"Result.kind := {names[host]}'Pos (Value) + 1;"]
            if not metadata['nodes'][native].get('enum'):
                body.append('case Value is')
                for index, value in enumerate(values):
                    child_type = name(metadata['children'][(native, native + '_arm_' + str(index + 1))])
                    pointers.add(child_type)
                    body += [f'when {identifier(host + "_" + value)} =>', f'Result.data := (discr => {index}, arm_{index + 1} => P_{child_type} (Store_{child_type} ((reserved => 0), Storage)));']
                body.append('end case;')
        elif source.get('variants') or source.get('oneOf') or source.get('anyOf') or isinstance(source.get('type'), list):
            alternatives = source.get('oneOf', source.get('anyOf', []))
            tags = [str(i) for i in range(len(alternatives))]
            if source.get('variants'):
                alternatives = [{'$ref':'#/$defs/' + target} for target, _ in source['variants']]
                tags = [target.removeprefix(host + '_') for target, _ in source['variants']]
            elif isinstance(source.get('type'), list):
                alternatives = [{**source, 'type':kind} for kind in source['type']]
                tags = [str(i) for i in range(len(alternatives))]
            body = ['case Value.Kind is']
            for index, (tag, alternative) in enumerate(zip(tags, alternatives)):
                function, child_type = child(alternative, native + '_arm_' + str(index + 1), host + '_value_' + tag)
                body += [f'when {identifier(host + "_arm_" + tag)} =>', f'Result.kind := {index + 1};', f'Result.data := (discr => {index}, arm_{index + 1} => P_{child_type} ({function} (Value.V_{index}, Storage)));']
            body.append('end case;')
        elif source.get('type') == 'array' or source.get('type') == 'object':
            mapping = source['type'] == 'object'
            value = (source.get('additionalProperties') if isinstance(source.get('additionalProperties'), dict) else {}) if mapping else source.get('items', {})
            function, child_type = child(value, native + ('_value' if mapping else '_item'), host + '_element')
            array_type = typ.removesuffix('_v1') + '_entry_v1' if mapping else 'System.Address'
            pointers.add(array_type) if mapping else None
            locals += [f'type Items is array (size_t range <>) of aliased {array_type} with Convention => C;', 'Length : constant size_t := size_t (Value.Length);', f'Address : constant System.Address := Allocate (Storage, Length * ({array_type}\'Size / 8));', "Values : Items (1 .. Length) with Import, Address => Address;", 'Position : size_t := 1;']
            body = ['for Item of Value loop']
            body += [f'Values (Position) := (key => UTF8 (To_String (Item.Key), Storage), value => P_{child_type} ({function} (Item.Value, Storage)));'] if mapping else [f'Values (Position) := {function} (Item, Storage);']
            body += ['Position := Position + 1;', 'end loop;', 'Result.len := Length;']
            body += [f'Result.data := P_{array_type} (Address);'] if mapping else ['Result.data := Address;']
        elif source.get('enum'):
            body = [f'Result.kind := {names[host]}\'Pos (Value) + 1;']
        elif 'const' in source or source.get('type') == 'null':
            locals.append('pragma Unreferenced (Value);')
            body = ['Result.reserved := 0;']
        elif source.get('type') in ('integer','number','boolean'):
            kind = source['type']
            native_type = 'double' if kind == 'number' else 'Interfaces.Unsigned_32' if kind == 'boolean' else 'Interfaces.Unsigned_64' if source.get('format','').startswith('u') else 'Interfaces.Integer_64'
            expression = f'{native_type} (Value)' if kind != 'boolean' else '(if Value then 1 else 0)'
            body = ['Result.value := ' + expression + ';']
        else:
            body = ['Result.text := UTF8 (To_String (Unbounded_String (Value)), Storage);']
        if not body:
            locals.append('pragma Unreferenced (Value);')
            body = ['Result.reserved := 0;']
        function = 'Convert_' + host + '_' + native
        bodies.append((f'function {function} (Value : {names[host]}; Storage : in out Arena) return System.Address', '\n'.join([f'function {function} (Value : {names[host]}; Storage : in out Arena) return System.Address is', f'Result : {typ};', *locals, 'begin', *body, f'return Store_{typ} (Result, Storage);', 'end ' + function + ';'])))
    declarations = []
    for typ in sorted(pointers):
        declarations += [f'type A_{typ} is access all {typ};', f'function P_{typ} is new Ada.Unchecked_Conversion (System.Address, A_{typ});', f'function Store_{typ} is new Store ({typ});']
    entrypoints = []
    for host, native in [('RequestCall','RequestArguments'), ('RequestSessionDescriptor','RequestSessionDescriptor'), ('RequestReaderFailure','RequestReaderFailure')]:
        entrypoints.append(f'function Convert (Value : {names[host]}; Storage : in out Arena) return System.Address is (Convert_{host}_{native} (Value, Storage));')
    spec = SPEC.replace('-- ENTRIES', '\n'.join(f'function Convert (Value : {names[host]}; Storage : in out Arena) return System.Address;' for host in ('RequestCall','RequestSessionDescriptor','RequestReaderFailure'))) + '\nend Thinkthen.Requests.Native;\n'
    body = BODY + '\n'.join(declarations + [signature + ';' for signature, _ in bodies] + [body for _, body in bodies] + entrypoints) + '\nend Thinkthen.Requests.Native;\n'
    return spec, body


SPEC = '''-- Generated native carrier adaptation; do not edit.
with Ada.Finalization;
with System;
package Thinkthen.Requests.Native is
   type Arena is new Ada.Finalization.Limited_Controlled with private;
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
public
'''
# Ada has no public section after private; generate declarations before private.
SPEC = SPEC.replace('private\n', '-- ENTRIES\nprivate\n', 1).replace('public\n', '')

BODY = '''-- Generated native carrier adaptation; do not edit.
with Ada.Unchecked_Conversion;
with Ada.Unchecked_Deallocation;
with Interfaces.C.Strings;
with System.Address_To_Access_Conversions;
with Thinkthen_Session_C; use Thinkthen_Session_C;
package body Thinkthen.Requests.Native is
use Interfaces.C;
use type Interfaces.Unsigned_32;
use type System.Address;
function Malloc (Size : size_t) return System.Address with Import, Convention => C, External_Name => "malloc";
procedure Free (Address : System.Address) with Import, Convention => C, External_Name => "free";
procedure Dispose is new Ada.Unchecked_Deallocation (Allocation, Allocation_Access);
function Allocate (Storage : in out Arena; Size : size_t) return System.Address is
   Address : constant System.Address := Malloc (size_t'Max (Size, 1));
begin
   if Address = System.Null_Address then raise Storage_Error; end if;
   begin
      Storage.Head := new Allocation'(Address, Storage.Head);
   exception
      when others => Free (Address); raise;
   end;
   return Address;
end Allocate;
overriding procedure Finalize (Storage : in out Arena) is
   Item : Allocation_Access;
begin
   while Storage.Head /= null loop
      Item := Storage.Head;
      Storage.Head := Item.Next;
      Free (Item.Address);
      Dispose (Item);
   end loop;
end Finalize;
generic
   type Element is private;
function Store (Value : Element; Storage : in out Arena) return System.Address;
function Store (Value : Element; Storage : in out Arena) return System.Address is
   package Pointers is new System.Address_To_Access_Conversions (Element);
   Address : constant System.Address := Allocate (Storage, Element'Size / 8);
begin
   Pointers.To_Pointer (Address).all := Value;
   return Address;
end Store;
function UTF8 (Value : String; Storage : in out Arena) return thinkthen_complete_utf8_v1 is
   Address : constant System.Address := Allocate (Storage, size_t (Value'Length));
   Bytes : char_array (1 .. size_t (Value'Length)) with Import, Address => Address;
   function Pointer is new Ada.Unchecked_Conversion (System.Address, Interfaces.C.Strings.chars_ptr);
begin
   for I in Value'Range loop Bytes (size_t (I - Value'First + 1)) := To_C (Value (I)); end loop;
   return (data => Pointer (Address), len => size_t (Value'Length));
end UTF8;
'''


def plan(function_schema, header):
    import re
    function_names = ', '.join('when ' + identifier('RequestFunction_' + function) + ' => "' + function + '"' for function in enum_values(function_schema))
    return ['function Plan (Client : Engine; Request : T_RequestCall) return String is',
            'use Thinkthen_Session_C; use Interfaces.C; use Interfaces.C.Strings;',
            'Storage : Thinkthen.Requests.Native.Arena;',
            'type Arguments_Access is access constant thinkthen_request_arguments_v1;',
            'function Pointer is new Ada.Unchecked_Conversion (System.Address, Arguments_Access);',
            'Arguments : constant Arguments_Access := Pointer (Thinkthen.Requests.Native.Convert (Request, Storage));',
            'Function_Name : constant String := (case Request.T_function is ' + function_names + ');',
            'Function_Bytes : chars_ptr := Null_Ptr; Surface : chars_ptr := Null_Ptr;',
            'Output : aliased chars_ptr := Null_Ptr; Length : aliased size_t := 0; Code : int;',
            'begin', 'Function_Bytes := New_String (Function_Name); Surface := New_String ("ada");',
            "Code := thinkthen_request_plan_typed_json (Client.Handle, (data => Function_Bytes, len => Function_Name'Length), Arguments, (data => Surface, len => 3), Output'Address, Length'Access);",
            'Free (Function_Bytes); Free (Surface);',
            'Thinkthen_Kinds.Check (Code);',
            'declare Result : constant String := Text ((data => Output, len => Length)); begin thinkthen_free_string (Output); Output := Null_Ptr; return Result; end;',
            'exception when others => Free (Function_Bytes); Free (Surface); thinkthen_free_string (Output); raise;',
            'end Plan;']
