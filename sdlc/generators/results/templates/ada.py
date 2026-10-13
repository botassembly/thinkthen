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


def render(definitions, names=None, schemas=None):
    specs, done = [], set()
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
        if schemas is not None:
            schemas[key] = schema
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
        elif alternatives:
            base = emit(key + '_value', alternatives[0])
            aliases[key] = base
            return base
        elif 'properties' in schema:
            fields = []
            for member, value in schema['properties'].items():
                field = identifier(member)
                typ = emit(key + '_field_' + member, value)
                required = member in schema.get('required', [])
                if not required:
                    opt = optional(typ)
                    fields.append(f'{field} : {opt};')
                else:
                    qualified = 'Thinkthen.Requests.' + typ if field.lower() == typ.lower() else typ
                    fields.append(f'{field} : {qualified};')
            specs.append(f'type {name} is ' + ('record\n' + '\n'.join(fields) + '\nend record;' if fields else 'null record;'))
        elif kinds == 'array' or kinds == 'object':
            mapping = kinds == 'object'
            typ = emit(key + '_element', (schema.get('additionalProperties') if isinstance(schema.get('additionalProperties'), dict) else {}) if mapping else schema.get('items', {}))
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
        else:
            return scalar(key, name, schema)

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
        elif const is not ...:
            specs.append(f'type {name} is null record;')
        elif kind == 'string':
            specs.append(f'type {name} is new Unbounded_String;')
        elif kind == 'boolean':
            specs.append(f'type {name} is new Boolean;')
        elif kind == 'integer':
            base = 'Interfaces.Unsigned_64' if schema.get('format') in ('uint', 'uint64', 'uint32') else 'Interfaces.Integer_64'
            specs.append(f'type {name} is new {base};')
        elif kind == 'number':
            specs.append(f'type {name} is new Long_Float;')
        elif kind == 'null':
            specs.append(f'type {name} is null record;')
        else:
            # Only schema-declared arbitrary JSON is transported verbatim.
            specs.append(f'subtype {name} is JSON_Value;')

        return name
    for key, value in definitions.items():
        emit(key, value)
    spec = '''-- Generated from the canonical Request graph. Do not edit.
with Ada.Containers.Vectors;
with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Interfaces;
with Thinkthen.Values;
package Thinkthen.Requests is
-- Arbitrary values use owned native trees; Rust owns their admission.
subtype JSON_Value is Thinkthen.Values.Value;
use type Thinkthen.Values.Value;
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
    if names is not None:
        names.update({key: aliases.get(key, identifier(key)) for key in done})
    return spec, ''


def request_helpers(definitions, names):
    """Construction-only helpers preserve the schema's typed alternatives."""
    output = ['subtype Question is ' + names['RequestQuestion'] + ';',
              'subtype Input is ' + names['RequestInput'] + ';',
              'subtype Item is ' + names['RequestItem'] + ';',
              'subtype Items is T_RequestItem_List;']
    questions = definitions['RequestQuestion']['variants']
    def selector(kind, field, parameter, expression):
        index, branch = next((i, key) for i, (key, _) in enumerate(questions) if key.endswith('_' + kind))
        aggregate = f"{names['RequestQuestion']}'(Kind => {identifier('RequestQuestion_arm_' + kind)}, V_{index} => ({identifier(field)} => {expression}, others => <>))"
        return f'function {parameter} return {names["RequestQuestion"]} is ({aggregate});'
    for kind, field, public in [('text','text','Text_Question'), ('file','path','Question_File'), ('name','name','Question_Name'), ('reference','reference','Question_Reference')]:
        output.append(selector(kind, field, public + ' (Value : String)', 'T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Value))'))
    for function in ('recognize','relate'):
        index, branch = next((i, key) for i, (key, _) in enumerate(definitions['RequestDefinition']['variants']) if function in definitions[key].get('properties', {}))
        field = definitions[branch]['properties'][function]
        typ = names[field['$ref'].removeprefix('#/$defs/')]
        definition = f"{names['RequestDefinition']}'(Kind => {identifier('RequestDefinition_arm_' + branch.removeprefix('RequestDefinition_'))}, V_{index} => ({identifier(function)} => Value, others => <>))"
        output.append(selector('definition','value',function.title() + '_Question (Value : ' + typ + ')',definition))
        definition = f"{names['RequestDefinition']}'(Kind => {identifier('RequestDefinition_arm_' + branch.removeprefix('RequestDefinition_'))}, V_{index} => Value)"
        output.append(selector('definition','value',function.title() + '_Question (Value : ' + names[branch] + ')',definition))
    output += [
        "function Text_Item (Value : String) return T_RequestItem is (T_RequestItem'(T_original => (Present => True, Value => (Kind => T_RequestOriginal_arm_text, V_0 => (T_text => T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Value)), others => <>))), others => <>));",
        "function Items_Input (Value : T_RequestItem_List) return T_RequestInput is (T_RequestInput'(Kind => T_RequestInput_arm_records, V_2 => (T_items => Value, others => <>)));",
        "function File_Input (Path : String) return T_RequestInput is (T_RequestInput'(Kind => T_RequestInput_arm_source, V_5 => (T_source => (T_paths => T_Text_List'[T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Path))], others => <>), others => <>)));",
    ]
    return '\n'.join(output)


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
    definitions = prepare(graph(schema, ('Request', 'RequestSessionDescriptor', 'RequestReaderFailure')))
    names, schemas = {}, {}
    spec, _ = render(definitions, names, schemas)
    spec = spec.replace('end Thinkthen.Requests;', request_helpers(definitions, names) + '\nend Thinkthen.Requests;')
    from templates.ada_native import render as native_render
    native_spec, native_body = native_render(definitions, names, schemas)
    calls_spec = ['-- Generated named calls from canonical RequestCall variants.',
                  'with Thinkthen.Requests; use Thinkthen.Requests;', 'with Thinkthen.Inputs;',
                  'package Thinkthen.Sessions.Calls is']
    calls_body = ['-- Generated named calls from the canonical request schema; do not edit.', 'with Ada.Unchecked_Conversion;', 'with System;', 'with Thinkthen_Kinds;', 'with Thinkthen.Requests.Native;', 'with Thinkthen.Inputs.Calls;', 'with Thinkthen.Native_Values;', 'with Interfaces.C.Strings;', 'package body Thinkthen.Sessions.Calls is']
    version = identifier('RequestVersion_' + enum_values(schema['$defs']['RequestVersion'])[0])
    for function in enum_values(schema['$defs']['RequestFunction']):
        declaration = f'procedure {function.title()} (Owner : in out Session; Request : T_RequestCall)'
        shared = declaration.replace('(Owner', '(Client : Engine; Owner')
        calls_spec += [declaration + ';', shared + ';']
        for signature, owner in ((declaration, 'Owner'), (shared, 'Client, Owner')):
            calls_body.append(signature + ' is\nbegin\nStart (' + owner + ', Request, "' + function + '");\nend ' + function.title() + ';')
    for function in enum_values(schema['$defs']['RequestFunction']):
        declaration = f'procedure {function.title()} (Owner : in out Session; Question : Thinkthen.Requests.Question; Input : Thinkthen.Requests.Input; Options : T_RequestOptions_Optional := (Present => False))'
        shared = declaration.replace('(Owner', '(Client : Engine; Owner')
        calls_spec += [declaration + ';', shared + ';']
        for signature, owner in ((declaration, 'Owner'), (shared, 'Client, Owner')):
            calls_body.append(signature + ' is\nbegin\nStart (' + owner + ', (T_question => Question, T_input => Input, T_options => Options, others => <>), "' + function + '");\nend ' + function.title() + ';')
    for function in enum_values(schema['$defs']['RequestFunction']):
        declaration = f'procedure {function.title()} (Owner : in out Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Thinkthen.Inputs.Call_Options := Thinkthen.Inputs.No_Options)'
        shared = declaration.replace('(Owner', '(Client : Engine; Owner')
        calls_spec += [declaration+';',shared+';']
        for signature,owner in ((declaration,'Owner'),(shared,'Client, Owner')):
            calls_body.append(signature+' is\nbegin\nThinkthen.Inputs.Calls.'+function.title()+' ('+owner+', Question, Input, Options);\nend '+function.title()+';')
    calls_spec.insert(1, 'with Thinkthen.Values;')
    calls_spec.append('function Plan (Client : Engine; Request : T_RequestCall) return Thinkthen.Values.Value;')
    from templates.ada_native import plan
    calls_body.extend(plan(schema['$defs']['RequestFunction'], (root / 'libraries/c/include/thinkthen.h').read_text()))
    calls_spec.append('end Thinkthen.Sessions.Calls;')
    calls_body.append('end Thinkthen.Sessions.Calls;')
    outputs = {'thinkthen-sessions-calls.ads': '\n'.join(calls_spec) + '\n',
               'thinkthen-sessions-calls.adb': '\n'.join(calls_body) + '\n',
               'thinkthen-requests.ads': spec,
               'thinkthen-requests-native.ads': native_spec, 'thinkthen-requests-native.adb': native_body,
               'thinkthen_session_c.ads': header(root, env)}
    from templates.ada_inputs import outputs as input_outputs
    outputs.update(input_outputs(definitions, names, schemas))
    from templates.ada_value_readers import outputs as value_reader_outputs
    outputs.update(value_reader_outputs())
    from templates.ada_answers import outputs as answer_outputs
    results = json.loads((root / 'crates/thinkthen/src/public/results/complete.schema.json').read_text())
    outputs.update(answer_outputs(prepare(graph(results, ('completesessionPacket',)))))
    from templates.ada_values import outputs as value_outputs
    outputs.update(value_outputs())
    from templates.ada_usage import outputs as usage_outputs
    outputs.update(usage_outputs((root / 'libraries/c/include/thinkthen.h').read_text()))
    for name, content in outputs.items():
        path = output_path(root, 'ada', name)
        if check:
            if not path.exists() or path.read_text() != content:
                raise SystemExit(f'generated Ada declaration differs: {path}')
        else:
            path.write_text(content)
