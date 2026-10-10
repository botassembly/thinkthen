"""Generate Ada transport records from the shared graph and C layouts from its header."""
import json
import re
import subprocess
import tempfile
from pathlib import Path


def identifier(value):
    return 'T_' + re.sub(r'[^a-zA-Z0-9]+', '_', value).strip('_')


def render(definitions):
    specs, bodies, done = [], [], set()
    def emit(key, schema):
        name = identifier(key)
        if key in done:
            return name
        done.add(key)
        if not isinstance(schema, dict):
            schema = {}
        if '$ref' in schema:
            original = schema['$ref'].removeprefix('#/$defs/')
            base = emit(original, definitions[original])
            specs.append(f'subtype {name} is {base};')
            return name
        kinds = schema.get('type')
        alternatives = schema.get('anyOf', schema.get('oneOf'))
        if isinstance(kinds, list):
            alternatives = [{**schema, 'type': k} for k in kinds]
        variants = schema.get('variants')
        if variants or alternatives and len(alternatives) > 1:
            members = [(n, {'$ref': '#/$defs/' + n}) for n, _ in variants] if variants else [(str(i), v) for i, v in enumerate(alternatives)]
            arms = [(identifier(key + '_arm_' + tag), emit(key + '_value_' + tag, value)) for tag, value in members]
            specs.append(f'type {name}_Kind is (' + ', '.join(tag for tag, _ in arms) + ');')
            specs.append(f'type {name} (Kind : {name}_Kind := {arms[0][0]}) is record\ncase Kind is\n' + '\n'.join(f'when {tag} => V_{i} : {typ};' for i, (tag, typ) in enumerate(arms)) + '\nend case;\nend record;')
            body = f'case Value.Kind is\n' + '\n'.join(f'when {tag} => return Encode (Value.V_{i});' for i, (tag, _) in enumerate(arms)) + '\nend case;'
        elif alternatives:
            return emit(key, alternatives[0]) if False else scalar(key, name, alternatives[0])
        elif 'properties' in schema:
            fields, entries = [], []
            for member, value in schema['properties'].items():
                field = identifier(member)
                typ = emit(key + '_field_' + member, value)
                required = member in schema.get('required', [])
                if not required:
                    opt = name + '_Optional_' + field
                    specs.append(f'type {opt} (Present : Boolean := False) is record\ncase Present is\nwhen True => Value : {typ};\nwhen False => null;\nend case;\nend record;')
                    fields.append(f'{field} : {opt};')
                    entries.append(f'if Value.{field}.Present then Add (Result, {ada_string(member)}, Encode (Value.{field}.Value)); end if;')
                else:
                    fields.append(f'{field} : {typ};')
                    entries.append(f'Add (Result, {ada_string(member)}, Encode (Value.{field}));')
            specs.append(f'type {name} is ' + ('record\n' + '\n'.join(fields) + '\nend record;' if fields else 'null record;'))
            body = 'Result : Unbounded_String := To_Unbounded_String ("{");\nbegin\n' + '\n'.join(entries) + '\nAppend (Result, "}");\nreturn To_String (Result);'
        elif kinds == 'array' or kinds == 'object' and isinstance(schema.get('additionalProperties'), dict):
            mapping = kinds == 'object'
            typ = emit(key + '_element', schema['additionalProperties'] if mapping else schema.get('items', {}))
            if mapping:
                entry = name + '_Entry'
                specs.append(f'type {entry} is record\nKey : Unbounded_String;\nValue : {typ};\nend record;')
                typ = entry
            specs.append(f'package {name}_Vectors is new Ada.Containers.Vectors (Positive, {typ});\nsubtype {name} is {name}_Vectors.Vector;')
            start, end = ('{', '}') if mapping else ('[', ']')
            item = 'Quote (To_String (Item.Key)) & ":" & Encode (Item.Value)' if mapping else 'Encode (Item)'
            body = f'Result : Unbounded_String := To_Unbounded_String ("{start}");\nbegin\nfor Item of Value loop\nif Length (Result) > 1 then Append (Result, ","); end if;\nAppend (Result, {item});\nend loop;\nAppend (Result, "{end}");\nreturn To_String (Result);'
        else:
            return scalar(key, name, schema)
        specs.append(f'function Encode (Value : {name}) return String;')
        if not body.startswith('Result :'):
            body = 'begin\n' + body
        bodies.append(f'function Encode (Value : {name}) return String is\n{body}\nend Encode;')
        return name
    def scalar(key, name, schema):
        kind = schema.get('type')
        const = schema.get('const', ...)
        alternatives = schema.get('oneOf', [])
        if len(alternatives) == 1:
            schema = alternatives[0]
            const = schema.get('const', ...)
            kind = schema.get('type')
        if const is not ...:
            specs.append(f'type {name} is null record;')
            expression = ada_string(json.dumps(const, ensure_ascii=True))
        elif kind == 'string':
            specs.append(f'type {name} is new Unbounded_String;')
            expression = 'Quote (To_String (Unbounded_String (Value)))'
        elif kind == 'boolean':
            specs.append(f'type {name} is new Boolean;')
            expression = '(if Value then "true" else "false")'
        elif kind == 'integer':
            base = 'Interfaces.Unsigned_64' if schema.get('format') in ('uint', 'uint64', 'uint32') else 'Interfaces.Integer_64'
            specs.append(f'type {name} is new {base};')
            expression = f'Ada.Strings.Fixed.Trim ({name}\'Image (Value), Ada.Strings.Both)'
        elif kind == 'number':
            specs.append(f'type {name} is new Long_Float;')
            expression = f'Ada.Strings.Fixed.Trim ({name}\'Image (Value), Ada.Strings.Both)'
        elif kind == 'null':
            specs.append(f'type {name} is null record;')
            expression = '"null"'
        else:
            # Only schema-declared arbitrary JSON is transported verbatim.
            specs.append(f'type {name} is new JSON_Value;')
            expression = 'To_String (Unbounded_String (Value))'
        specs.append(f'function Encode (Value : {name}) return String;')
        bodies.append(f'function Encode (Value : {name}) return String is\nbegin\nreturn {expression};\nend Encode;')
        return name
    for key, value in definitions.items():
        emit(key, value)
    spec = '''-- Generated from the canonical Request graph. Do not edit.
with Ada.Containers.Vectors;
with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Interfaces;
package Thinkthen.Requests is
-- Arbitrary JSON content only; native admission owns its syntax and meaning.
type JSON_Value is new Unbounded_String;
''' + '\n'.join(specs) + '\nend Thinkthen.Requests;\n'
    body = '''-- Generated transport only. Native admission owns all semantic rules.
with Ada.Strings.Fixed;
package body Thinkthen.Requests is
function Quote (Value : String) return String is
Result : Unbounded_String := To_Unbounded_String ("""");
Hex : constant String := "0123456789abcdef";
begin
for C of Value loop
case C is
when '"' => Append (Result, "\\""");
when '\\' => Append (Result, "\\\\");
when Character'Val (0) .. Character'Val (31) =>
Append (Result, "\\u00" & Hex (Character'Pos (C) / 16 + 1) & Hex (Character'Pos (C) mod 16 + 1));
when others => Append (Result, C);
end case;
end loop;
Append (Result, '"');
return To_String (Result);
end Quote;
procedure Add (Result : in out Unbounded_String; Key, Value : String) is
begin
if Length (Result) > 1 then Append (Result, ","); end if;
Append (Result, Quote (Key) & ":" & Value);
end Add;
''' + '\n'.join(bodies) + '\nend Thinkthen.Requests;\n'
    return spec, body


def ada_string(value):
    return '"' + value.replace('"', '""') + '"'


def header(root, env):
    with tempfile.TemporaryDirectory(prefix='thinkthen-ada-header-') as temporary:
        work = Path(temporary)
        # Use a relative canonical basename so no checkout path reaches output.
        (work / 'thinkthen.h').write_bytes((root / 'libraries/c/include/thinkthen.h').read_bytes())
        subprocess.run(['gcc', '-c', '-fdump-ada-spec', 'thinkthen.h'], cwd=work,
                       env=env, check=True, capture_output=True)
        source = (work / 'thinkthen_h.ads').read_text()
    source = re.sub(r'--[^\n]*', '', source)
    source = re.sub(r'^with (?:stddef_h|\w+_stdint_\w+_h);\n', '', source, flags=re.M)
    source = source.replace('stddef_h.size_t', 'Interfaces.C.size_t')
    source = re.sub(r'\w+_stdint_uintn_h\.uint(\d+)_t', r'Interfaces.Unsigned_\1', source)
    source = re.sub(r'\w+_stdint_intn_h\.int(\d+)_t', r'Interfaces.Integer_\1', source)
    source = source.replace('thinkthen_h', 'Thinkthen_Session_C')
    source = source.replace('pragma Ada_2012;', '')
    for constant in re.findall(r'(THINKTHEN_\w+) : constant', source):
        source = re.sub(r'\b' + constant + r'\b', 'K_' + constant, source)
    return '-- Generated from the canonical C header by GNAT. Do not edit.\nwith Interfaces;\n' + '\n'.join(line.rstrip() for line in source.splitlines() if line.strip()) + '\n'


def generate(root, graph, prepare, env, check):
    schema = json.loads((root / 'specification/request.schema.json').read_text())
    schema['$defs']['Request'] = {k: v for k, v in schema.items() if k != '$defs'}
    spec, body = render(prepare(graph(schema, ('Request', 'RequestSessionDescriptor', 'RequestReaderFailure'))))
    calls_spec = ['-- Generated named calls from canonical RequestCall variants.',
                  'with Thinkthen.Requests; use Thinkthen.Requests;',
                  'package Thinkthen.Sessions.Calls is']
    calls_body = ['package body Thinkthen.Sessions.Calls is']
    for i, arm in enumerate(schema['$defs']['RequestCall']['oneOf']):
        function = arm['properties']['function']['const']
        typename = identifier('RequestCall_' + function)
        declaration = f'procedure {function.title()} (Owner : in out Session; Request : {typename})'
        calls_spec.append(declaration + ';')
        calls_body.append(declaration + ' is\nbegin\nStart (Owner, (T_schema => (null record), T_call => (Kind => ' + identifier('RequestCall_arm_RequestCall_' + function) + f', V_{i} => Request)));\nend {function.title()};')
    calls_spec.append('end Thinkthen.Sessions.Calls;')
    calls_body.append('end Thinkthen.Sessions.Calls;')
    outputs = {'thinkthen-sessions-calls.ads': '\n'.join(calls_spec) + '\n',
               'thinkthen-sessions-calls.adb': '\n'.join(calls_body) + '\n',
               'thinkthen-requests.ads': spec, 'thinkthen-requests.adb': body,
               'thinkthen_session_c.ads': header(root, env)}
    for name, content in outputs.items():
        path = root / 'libraries/ada/src' / name
        if check:
            if not path.exists() or path.read_text() != content:
                raise SystemExit(f'generated Ada declaration differs: {path}')
        else:
            path.write_text(content)
