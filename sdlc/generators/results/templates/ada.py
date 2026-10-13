from native import selected_header
from registry import output_path
from templates.csharp import enum_values
"""Generate Ada transport records from the shared graph and C layouts from its header."""
import json
import re
import subprocess
import tempfile
from pathlib import Path


def identifier(value):
    return 'T_' + re.sub(r'[^a-zA-Z0-9]+', '_', value).strip('_')


def render(definitions, names=None):
    specs, bodies, done = [], [], set()
    optionals = {}
    scalar_names = {}
    aliases = {}
    containers = {}
    def optional(typ):
        if typ not in optionals:
            package = typ + '_Options'
            name = typ + '_Optional'
            specs.append(f'package {package} is new Optional_Values ({typ});\nsubtype {name} is {package}.Optional;')
            optionals[typ] = name
        return optionals[typ]
    def emit(key, schema):
        name = aliases.get(key, identifier(key))
        if key in done:
            return name
        done.add(key)
        if not isinstance(schema, dict):
            schema = {}
        if '$ref' in schema:
            original = schema['$ref'].removeprefix('#/$defs/')
            base = emit(original, definitions[original])
            aliases[key] = base
            return base
        values = enum_values(schema)
        if values:
            return scalar(key, name, {'type': 'string', 'enum': values})
        kinds = schema.get('type')
        alternatives = schema.get('anyOf', schema.get('oneOf'))
        if isinstance(kinds, list):
            alternatives = [{**schema, 'type': k} for k in kinds]
        variants = schema.get('variants')
        if variants or alternatives and len(alternatives) > 1:
            members = [(n, {'$ref': '#/$defs/' + n}) for n, _ in variants] if variants else [(str(i), v) for i, v in enumerate(alternatives)]
            arms = [(identifier(key + '_arm_' + tag.removeprefix(key + '_')), emit(key + '_value_' + tag.removeprefix(key + '_'), value)) for tag, value in members]
            specs.append(f'type {name}_Kind is (' + ',\n'.join(tag for tag, _ in arms) + ');')
            specs.append(f'type {name} (Kind : {name}_Kind := {arms[0][0]}) is record\ncase Kind is\n' + '\n'.join(f'when {tag} => V_{i} : {typ};' for i, (tag, typ) in enumerate(arms)) + '\nend case;\nend record;')
            body = f'case Value.Kind is\n' + '\n'.join(f'when {tag} => return Encode (Value.V_{i});' for i, (tag, _) in enumerate(arms)) + '\nend case;'
        elif alternatives:
            base = emit(key + '_value', alternatives[0])
            aliases[key] = base
            return base
        elif 'properties' in schema:
            fields, entries = [], []
            for member, value in schema['properties'].items():
                field = identifier(member)
                typ = emit(key + '_field_' + member, value)
                required = member in schema.get('required', [])
                if not required:
                    opt = optional(typ)
                    fields.append(f'{field} : {opt};')
                    entries.append(f'if Value.{field}.Present then Add (Result, {ada_string(member)}, Encode (Value.{field}.Value)); end if;')
                else:
                    qualified = 'Thinkthen.Requests.' + typ if field.lower() == typ.lower() else typ
                    fields.append(f'{field} : {qualified};')
                    entries.append(f'Add (Result, {ada_string(member)}, Encode (Value.{field}));')
            specs.append(f'type {name} is ' + ('record\n' + '\n'.join(fields) + '\nend record;' if fields else 'null record;'))
            body = 'Result : Unbounded_String := To_Unbounded_String ("{");\nbegin\n' + '\n'.join(entries) + '\nAppend (Result, "}");\nreturn To_String (Result);'
        elif kinds == 'array' or kinds == 'object' and isinstance(schema.get('additionalProperties'), dict):
            mapping = kinds == 'object'
            typ = emit(key + '_element', schema['additionalProperties'] if mapping else schema.get('items', {}))
            if key not in definitions:
                identity = (mapping, typ)
                if identity in containers:
                    aliases[key] = containers[identity]
                    return containers[identity]
                name = typ + ('_Map' if mapping else '_List')
                aliases[key] = name
                containers[identity] = name
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
        if key not in definitions:
            identity = json.dumps(schema, sort_keys=True)
            if identity in scalar_names:
                aliases[key] = scalar_names[identity]
                return scalar_names[identity]
            kind = schema.get('type')
            if 'const' in schema:
                semantic = 'Literal_' + str(schema['const'])
            else:
                semantic = {'string':'Text', 'boolean':'Boolean', 'integer':('Unsigned' if schema.get('format', '').startswith('uint') else 'Integer'), 'number':'Number', 'null':'Null'}.get(kind, 'JSON')
            candidate = identifier(semantic)
            # Different value constraints share transport scalars; native Rust checks them.
            transport = ('literal', json.dumps(schema['const'])) if 'const' in schema else ('value', semantic)
            if transport in scalar_names:
                aliases[key] = scalar_names[transport]
                return scalar_names[transport]
            name = candidate
            scalar_names[identity] = name
            scalar_names[transport] = name
            aliases[key] = name
        kind = schema.get('type')
        const = schema.get('const', ...)
        alternatives = schema.get('oneOf', [])
        if len(alternatives) == 1:
            schema = alternatives[0]
            const = schema.get('const', ...)
            kind = schema.get('type')
        values = schema.get('enum')
        if values and all(isinstance(value, str) for value in values):
            literals = [identifier(key + '_' + value) for value in values]
            specs.append(f'type {name} is (' + ',\n'.join(literals) + ');')
            expression = '(case Value is ' + ',\n'.join('when ' + literal + ' => ' + ada_string(json.dumps(value)) for literal, value in zip(literals, values)) + ')'
        elif const is not ...:
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
        unused = 'pragma Unreferenced (Value);\n' if const is not ... or kind == 'null' else ''
        bodies.append(f'function Encode (Value : {name}) return String is\n{unused}begin\nreturn {expression};\nend Encode;')
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
generic
   type Element is private;
package Optional_Values is
   type Optional (Present : Boolean := False) is record
      case Present is
         when True => Value : Element;
         when False => null;
      end case;
   end record;
end Optional_Values;
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
    if names is not None:
        names.update({key: aliases.get(key, identifier(key)) for key in done})
    return spec, body


def ada_string(value):
    return '"' + value.replace('"', '""') + '"'


def header(root, env):
    with tempfile.TemporaryDirectory(prefix='thinkthen-ada-header-') as temporary:
        work = Path(temporary)
        # Use a relative canonical basename so no checkout path reaches output.
        (work / 'thinkthen.h').write_text(selected_header((root / 'libraries/c/include/thinkthen.h').read_text(), 'ada'))
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
    calls_body = ['-- Generated named calls from the canonical request schema; do not edit.', 'package body Thinkthen.Sessions.Calls is']
    version = identifier('RequestVersion_' + enum_values(schema['$defs']['RequestVersion'])[0])
    for function in enum_values(schema['$defs']['RequestFunction']):
        declaration = f'procedure {function.title()} (Owner : in out Session; Request : T_RequestCall)'
        shared = declaration.replace('(Owner', '(Client : Engine; Owner')
        calls_spec += [declaration + ';', shared + ';']
        for signature, owner in ((declaration, 'Owner'), (shared, 'Client, Owner')):
            calls_body.append(signature + ' is\nCall : T_RequestCall := Request;\nbegin\nCall.T_function := ' + identifier('RequestFunction_' + function) + ';\nStart (' + owner + ', (T_schema => ' + version + ', T_call => Call));\nend ' + function.title() + ';')
    calls_spec.append('end Thinkthen.Sessions.Calls;')
    calls_body.append('end Thinkthen.Sessions.Calls;')
    outputs = {'thinkthen-sessions-calls.ads': '\n'.join(calls_spec) + '\n',
               'thinkthen-sessions-calls.adb': '\n'.join(calls_body) + '\n',
               'thinkthen-requests.ads': spec, 'thinkthen-requests.adb': body,
               'thinkthen_session_c.ads': header(root, env)}
    for name, content in outputs.items():
        path = output_path(root, 'ada', name)
        if check:
            if not path.exists() or path.read_text() != content:
                raise SystemExit(f'generated Ada declaration differs: {path}')
        else:
            path.write_text(content)
