"""C# immutable result views. Conversion adds no schema validation or policy."""
import json
import re


def name(value):
    value = value.removeprefix('complete')
    if value.startswith('ReadableQuestion'):
        value = value.removeprefix('Readable')
    words = re.findall(r'[A-Z]?[a-z]+|[A-Z]+(?![a-z])|[0-9]+',
                       value.removeprefix('complete'))
    return ''.join(word[0].upper() + word[1:] for word in words)


def quote(value):
    return json.dumps(value, ensure_ascii=True)


def enum_values(schema):
    if 'enum' in schema and all(isinstance(value, str) for value in schema['enum']):
        return schema['enum']
    variants = schema.get('oneOf', schema.get('anyOf', []))
    if variants and all(item.get('type') == 'string' and 'const' in item
                        for item in variants):
        return [item['const'] for item in variants]
    return None


def nullable(schema, definitions=None):
    if isinstance(schema, bool):
        return schema
    if definitions and '$ref' in schema:
        return nullable(definitions[schema['$ref'].removeprefix('#/$defs/')], definitions)
    if schema.get('nullable_json'):
        return True
    kinds = schema.get('type', [])
    return (kinds == 'null' or isinstance(kinds, list) and 'null' in kinds or
            any(item.get('type') == 'null' for item in schema.get('anyOf', [])))


def shape(schema):
    if schema is True:
        return {}
    schema = dict(schema)
    if 'variants' in schema or 'primitive_variants' in schema:
        return schema
    if not enum_values(schema):
        if 'oneOf' in schema:
            raise ValueError(f'C# target needs a typed union for {schema}')
        if 'anyOf' in schema:
            if any(item is True or item == {} for item in schema['anyOf']):
                return {}
            variants = [item for item in schema['anyOf'] if item.get('type') != 'null']
            kinds = [variant.get('type') for variant in variants]
            # Native judgments have disjoint JSON kinds, with null repeated by
            # unresolved decide and choose readings. Missing stays at the member edge.
            alternatives = []
            for variant in variants:
                source = dict(variant)
                kinds_here = source.get('type')
                if isinstance(kinds_here, list):
                    kinds_here = [kind for kind in kinds_here if kind != 'null']
                    if len(kinds_here) == 1:
                        source['type'] = kinds_here[0]
                alternatives.append(source)
            native_kinds = [variant.get('type') for variant in alternatives]
            if (len(native_kinds) == 4 and
                    set(native_kinds) == {'boolean', 'string', 'number', 'array'}):
                return {**schema, 'primitive_variants': native_kinds,
                        'primitive_schemas': dict(zip(native_kinds, alternatives))}
            if len(kinds) == 2 and set(kinds) == {'integer', 'string'}:
                return {**schema, 'primitive_variants': kinds,
                        'primitive_schemas': dict(zip(kinds, variants))}
            if len(variants) != 1:
                raise ValueError(f'C# target needs a typed union for {schema}')
            return shape(variants[0])
    if isinstance(schema.get('type'), list):
        kinds = [kind for kind in schema['type'] if kind != 'null']
        if sorted(kinds) == ['number', 'string']:
            return {**schema, 'primitive_variants': ['number', 'string']}
        if len(kinds) != 1:
            raise ValueError(f'C# target needs a typed union for {schema}')
        schema['type'] = kinds[0]
    return schema


def conversion(schema, definitions, expression='member', depth=0):
    schema = shape(schema)
    if schema is True or schema == {}:
        # An explicitly unrestricted authored value owns arbitrary JSON content.
        return 'JsonElement', f'{expression}.Clone()'
    if '$ref' in schema:
        key = schema['$ref'].removeprefix('#/$defs/')
        target = shape(definitions[key])
        if ('properties' in target or enum_values(target) or 'variants' in target
                or 'primitive_variants' in target):
            kind = name(key)
            argument = f'{expression}.GetString()!' if enum_values(target) else expression
            return kind, (f'global::ThinkThen.Results.{kind}.Read({expression})' if
                          'variants' in target or 'primitive_variants' in target else
                          f'new {kind}({argument})')
        return conversion(target, definitions, expression, depth)
    if isinstance(schema.get('const'), str):
        return 'string', f'{expression}.GetString()!'
    kind = schema.get('type')
    if kind == 'object' and isinstance(schema.get('additionalProperties'), dict):
        value = schema['additionalProperties']
        if nullable(value):
            raise ValueError('C# target needs a nullable map value alternative')
        entry = f'entry{depth}'
        element, decode = conversion(value, definitions, entry + '.Value', depth + 1)
        return f'IReadOnlyDictionary<string, {element}>', (
            f'new ReadOnlyDictionary<string, {element}>({expression}.EnumerateObject()'
            f'.ToDictionary({entry} => {entry}.Name, {entry} => {decode}, StringComparer.Ordinal))')
    if kind == 'array':
        if nullable(schema['items']) and shape(schema['items']) != {}:
            raise ValueError('C# target needs a nullable array item alternative')
        item = f'item{depth}'
        element, decode = conversion(schema['items'], definitions, item, depth + 1)
        return f'IReadOnlyList<{element}>', (
            f'Array.AsReadOnly({expression}.EnumerateArray().Select({item} => {decode}).ToArray())')
    if kind == 'null':
        return 'JsonElement', f'{expression}.Clone()'
    if kind == 'integer':
        integers = {'uint16': ('ushort', 'GetUInt16'),
                    'uint32': ('uint', 'GetUInt32'),
                    'uint64': ('ulong', 'GetUInt64'),
                    'uint': ('ulong', 'GetUInt64'),
                    'int32': ('int', 'GetInt32'),
                    'int64': ('long', 'GetInt64')}
        if schema.get('format') in integers:
            native, read = integers[schema['format']]
            return native, f'{expression}.{read}()'
        unsigned = schema.get('minimum', -1) >= 0
        return ('ulong', f'{expression}.GetUInt64()') if unsigned else ('long', f'{expression}.GetInt64()')
    if kind in ('string', 'number', 'boolean'):
        return {'string': ('string', f'{expression}.GetString()!'),
                'number': ('double', f'{expression}.GetDouble()'),
                'boolean': ('bool', f'{expression}.GetBoolean()')}[kind]
    raise ValueError(f'C# target needs a typed alternative for {schema}')


SUPPORT = '''// Generated from Rust result types; do not edit.
#nullable enable
using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Linq;
using System.Text.Json;
using System.Text.Json.Nodes;

namespace ThinkThen.Results;

public enum PresenceState { Missing, Null, Value }

public readonly struct Presence<T>
{
    private readonly T? value;
    public PresenceState State { get; }
    private Presence(PresenceState state, T? value) { State = state; this.value = value; }
    public T Value => State == PresenceState.Value ? value! :
        throw new InvalidOperationException($"Member is {State}.");
    internal static Presence<T> Null => new(PresenceState.Null, default);
    internal static Presence<T> Present(T value) => new(PresenceState.Value, value);
}

public abstract class ResultObject
{
    private readonly JsonElement document;
    protected ResultObject(JsonElement document) { this.document = document.Clone(); }
    protected JsonElement RequiredElement(string name) => document.GetProperty(name);
    protected Presence<T> Optional<T>(string name, Func<JsonElement, T> read)
    {
        if (!document.TryGetProperty(name, out var member)) return default;
        return member.ValueKind == JsonValueKind.Null ? Presence<T>.Null : Presence<T>.Present(read(member));
    }
    // Cloning and plain conversion retain every JSON member, including unknown nested data.
    public JsonElement ToJson() => document.Clone();
    public JsonObject ToPlain() => JsonNode.Parse(document.GetRawText())!.AsObject();
    public string ToJsonString() => document.GetRawText();
}
'''


def render(definitions):
    chunks = [SUPPORT]
    declared = [name(key) for key, schema in definitions.items()
                if 'properties' in schema or 'variants' in schema or enum_values(schema)
                or 'primitive_variants' in shape(schema)]
    declared.extend(name(key) + name(variant) for key, schema in definitions.items()
                    for variant in shape(schema).get('primitive_variants', []))
    if any(not kind for kind in declared) or len(set(declared)) != len(declared):
        raise ValueError('colliding or empty C# result type names')
    parents = {variant: name(key) for key, schema in definitions.items()
               for variant, _ in schema.get('variants', [])}
    for key, schema in definitions.items():
        schema = shape(schema)
        kind = name(key)
        values = enum_values(schema)
        if 'variants' in schema:
            chunks.append(render_union(kind, schema['variants'], definitions))
        elif 'primitive_variants' in schema:
            chunks.append(render_primitive_union(kind, schema['primitive_variants'], definitions, schema.get('primitive_schemas', {})))
        elif values:
            chunks.append(f'public readonly record struct {kind}(string Value)\n{{\n')
            for value in values:
                chunks.append(f'    public static {kind} {name(value)} => new({quote(value)});\n')
            chunks.append('}\n\n')
        elif 'properties' in schema:
            chunks.append(f'public sealed class {kind} : {parents.get(key, "ResultObject")}\n{{\n'
                          f'    public {kind}(JsonElement document) : base(document) {{ }}\n')
            for member, field in schema['properties'].items():
                optional = member not in schema.get('required', []) or nullable(field, definitions)
                field_type, decode = conversion(field, definitions)
                prop = name(member)
                if prop == kind:
                    prop += "Value"
                if optional:
                    chunks.append(f'    public Presence<{field_type}> {prop} => '
                                  f'Optional<{field_type}>({quote(member)}, member => {decode});\n')
                else:
                    _, required_decode = conversion(field, definitions, f'RequiredElement({quote(member)})')
                    chunks.append(f'    public {field_type} {prop} => {required_decode};\n')
            chunks.append('}\n\n')
        else:
            # Primitive references use their native C# types at the member edge.
            conversion(schema, definitions)
    return ''.join(chunks)


def render_primitive_union(kind, variants, definitions, schemas):
    chunks = [f'public abstract class {kind}\n{{\n'
              '    private readonly JsonElement document;\n'
              f'    protected {kind}(JsonElement document) {{ this.document = document.Clone(); }}\n'
              '    protected JsonElement Document => document;\n'
              '    public JsonElement ToJson() => document.Clone();\n'
              '    public JsonNode ToPlain() => JsonNode.Parse(document.GetRawText())!;\n'
              '    public string ToJsonString() => document.GetRawText();\n'
              f'    public static {kind} Read(JsonElement document) => document.ValueKind switch\n    {{\n']
    for variant in variants:
        tokens = {'number': ('Number',), 'integer': ('Number',), 'string': ('String',),
                  'boolean': ('True', 'False'), 'array': ('Array',),
                  'object': ('Object',), 'null': ('Null',)}[variant]
        for token in tokens:
            chunks.append(f'        JsonValueKind.{token} => new {kind}{name(variant)}(document),\n')
    chunks.append('        _ => throw new JsonException("Unknown primitive result variant.")\n'
                  '    };\n}\n\n')
    for variant in variants:
        native, decode = conversion(schemas.get(variant, {'type': variant}), definitions, 'Document')
        chunks.append(f'public sealed class {kind}{name(variant)} : {kind}\n{{\n'
                      f'    public {kind}{name(variant)}(JsonElement document) : base(document) {{ }}\n'
                      f'    public {native} Value => {decode};\n}}\n\n')
    return ''.join(chunks)


def kind_test(schema, definitions, expression):
    schema = shape(schema)
    if schema == {}:
        return 'true'
    if '$ref' in schema:
        return kind_test(definitions[schema['$ref'].removeprefix('#/$defs/')],
                         definitions, expression)
    kinds = {'string': ('String',), 'number': ('Number',), 'integer': ('Number',),
             'object': ('Object',), 'array': ('Array',), 'boolean': ('True', 'False')}
    if schema.get('type') not in kinds:
        raise ValueError(f'C# target needs a JSON kind for discriminator {schema}')
    return ' || '.join(f'{expression}.ValueKind == JsonValueKind.{kind}'
                       for kind in kinds[schema['type']])


def render_union(kind, variants, definitions):
    chunks = [f'public abstract class {kind} : ResultObject\n{{\n'
              f'    protected {kind}(JsonElement document) : base(document) {{ }}\n'
              f'    public static {kind} Read(JsonElement document)\n    {{\n'
              f'        {kind}? result = null;\n']
    for variant, (mode, member, value) in variants:
        test = f'document.TryGetProperty({quote(member)}, out var tag{name(variant)})'
        if mode == 'literals':
            tests = []
            for index, (tag, literal) in enumerate(value.items()):
                variable = f'tag{name(variant)}{index}'
                tests.append(f'document.TryGetProperty({quote(tag)}, out var {variable})'
                             f' && {variable}.ValueKind == JsonValueKind.String'
                             f' && {variable}.GetString() == {quote(literal)}')
            test = ' && '.join(tests)
        if mode == 'structure':
            tests = []
            for index, tag in enumerate(value['required']):
                variable = f'tag{name(variant)}{index}'
                expected = kind_test(definitions[variant]['properties'][tag], definitions, variable)
                tests.append(f'document.TryGetProperty({quote(tag)}, out var {variable})'
                             f' && ({expected})')
            tests.extend(f'!document.TryGetProperty({quote(tag)}, out _)'
                         for tag in value['excluded'])
            test = ' && '.join(tests)
        if mode == 'literal':
            test += (f' && tag{name(variant)}.ValueKind == JsonValueKind.String'
                     f' && tag{name(variant)}.GetString() == {quote(value)}')
        chunks.append(f'        if ({test})\n        {{\n')
        if mode == 'member':
            expected = kind_test(definitions[variant]['properties'][member], definitions,
                                 f'tag{name(variant)}')
            chunks.append(f'            if (!({expected})) throw new JsonException("Invalid result identity kind.");\n')
        chunks.append('            if (result is not null) throw new JsonException("Ambiguous result variant.");\n'
                      f'            result = new {name(variant)}(document);\n        }}\n')
    chunks.append('        return result ?? throw new JsonException("Unknown result variant.");\n'
                  '    }\n}\n\n')
    return ''.join(chunks)


def render_inputs(schema):
    """Generate writable values from the native Request graph, including authored unions."""
    definitions = dict(schema['$defs'])
    definitions['Request'] = {k: v for k, v in schema.items() if k != '$defs'}
    nodes = {}
    def cname(key):
        return 'Input' + name(key)
    def cs_type(value, path):
        if isinstance(value, bool):
            return 'JsonElement'
        if '$ref' in value:
            return cname(value['$ref'].removeprefix('#/$defs/'))
        if 'const' in value:
            return type_of_literal(value['const'])
        if 'oneOf' in value or 'anyOf' in value:
            nodes.setdefault(path, value)
            return cname(path)
        kind = value.get('type')
        if isinstance(kind, list):
            nodes.setdefault(path, {'anyOf': [{'type': k} for k in kind]})
            return cname(path)
        if kind == 'array':
            return 'IReadOnlyList<' + cs_type(value.get('items', {}), path + '_Item') + '>'
        if kind == 'object' and 'properties' in value:
            nodes.setdefault(path, value)
            return cname(path)
        if kind == 'object' and isinstance(value.get('additionalProperties'), dict):
            return 'IReadOnlyDictionary<string, ' + cs_type(value['additionalProperties'], path + '_Entry') + '>'
        return {'string': 'string', 'boolean': 'bool', 'integer': 'long',
                'number': 'double'}.get(kind, 'JsonElement')
    def type_of_literal(value):
        return 'string' if isinstance(value, str) else 'bool' if isinstance(value, bool) else 'long'
    def emit(value, expr, path):
        typ = cs_type(value, path)
        if typ.startswith('Input'):
            return expr + '.Write(writer);'
        if typ.startswith('IReadOnlyList'):
            return 'writer.WriteStartArray(); foreach (var item in ' + expr + ') { ' + emit(value.get('items', {}), 'item', path + '_Item') + ' } writer.WriteEndArray();'
        if typ.startswith('IReadOnlyDictionary'):
            return 'writer.WriteStartObject(); foreach (var entry in ' + expr + ') { writer.WritePropertyName(entry.Key); ' + emit(value['additionalProperties'], 'entry.Value', path + '_Entry') + ' } writer.WriteEndObject();'
        return {'string': 'writer.WriteStringValue(' + expr + ');',
                'bool': 'writer.WriteBooleanValue(' + expr + ');',
                'long': 'writer.WriteNumberValue(' + expr + ');',
                'double': 'writer.WriteNumberValue(' + expr + ');'}.get(typ, expr + '.WriteTo(writer);')
    header = '''// Generated from the Rust-derived Request schema; do not edit.
#nullable enable
using System;
using System.Collections.Generic;
using System.IO;
using System.Text.Json;
namespace ThinkThen.Inputs;
public readonly struct InputPresence<T> {
    public bool IsPresent { get; }
    public T Value { get; }
    private InputPresence(T value) { Value = value; IsPresent = true; }
    public static implicit operator InputPresence<T>(T value) => new(value);
}
public abstract class InputDocument {
    public abstract void Write(Utf8JsonWriter writer);
    internal byte[] ToBytes() { using var stream = new MemoryStream(); using (var writer = new Utf8JsonWriter(stream)) Write(writer); return stream.ToArray(); }
}
'''
    nodes.update(definitions)
    output = []
    completed = set()
    while nodes.keys() - completed:
        key = sorted(nodes.keys() - completed)[0]
        completed.add(key)
        value = nodes[key]
        typ = cname(key)
        alternatives = value.get('oneOf', value.get('anyOf'))
        if alternatives and 'properties' not in value:
            output.append('public abstract class ' + typ + ' : InputDocument { private protected ' + typ + '() {} }')
            for index, alt in enumerate(alternatives):
                tag = next((p['const'] for p in alt.get('properties', {}).values() if isinstance(p.get('const'), str)), None)
                child = key + '_' + (tag if tag else 'Alternative' + str(index))
                nodes[child] = {**alt, '_base': typ}
            continue
        base = value.get('_base', 'InputDocument')
        lines = ['public sealed class ' + typ + ' : ' + base + ' {']
        properties = value.get('properties')
        if properties is not None:
            required = value.get('required', [])
            writes = ['writer.WriteStartObject();']
            for member, prop in properties.items():
                if isinstance(prop, bool):
                    prop = {}
                if 'const' in prop:
                    writes += ['writer.WritePropertyName(' + quote(member) + ');', emit(prop, quote(prop['const']) if isinstance(prop['const'], str) else str(prop['const']).lower(), key + '_' + member)]
                    continue
                ptype = cs_type(prop, key + '_' + member)
                pname = name(member)
                mandatory = member in required
                lines.append('public ' + ('required ' + ptype if mandatory else 'InputPresence<' + ptype + '>') + ' ' + pname + ' { get; init; }')
                body = 'writer.WritePropertyName(' + quote(member) + '); ' + emit(prop, pname if mandatory else pname + '.Value', key + '_' + member)
                writes.append(body if mandatory else 'if (' + pname + '.IsPresent) { ' + body + ' }')
            writes.append('writer.WriteEndObject();')
        elif 'const' in value:
            literal = value['const']
            writes = [emit(value, quote(literal) if isinstance(literal, str) else str(literal).lower(), key + '_Value')]
        else:
            stripped = {k: v for k, v in value.items() if k != '_base'}
            ptype = cs_type(stripped, key + '_Value')
            lines.append('public required ' + ptype + ' Value { get; init; }')
            writes = [emit(stripped, 'Value', key + '_Value')]
        lines.append('public override void Write(Utf8JsonWriter writer) { ' + ' '.join(writes) + ' }')
        lines.append('}')
        output.append('\n'.join(lines))
    return header + '\n\n'.join(output) + '\n'
