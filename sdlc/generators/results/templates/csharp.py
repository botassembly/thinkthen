"""C# immutable result views. Conversion adds no schema validation or policy."""
import json
import re
from host_answers import alternative_name


def name(value):
    value = value.removeprefix('complete')
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
    private protected ResultObject(JsonElement document) { this.document = document.Clone(); }
    private protected JsonElement RequiredElement(string name) => document.GetProperty(name);
    private protected Presence<T> Optional<T>(string name, Func<JsonElement, T> read)
    {
        if (!document.TryGetProperty(name, out var member)) return default;
        return member.ValueKind == JsonValueKind.Null ? Presence<T>.Null : Presence<T>.Present(read(member));
    }
    private protected Presence<T> RequiredNullable<T>(string name, Func<JsonElement, T> read)
    {
        var member = RequiredElement(name);
        return member.ValueKind == JsonValueKind.Null ? Presence<T>.Null : Presence<T>.Present(read(member));
    }
    // Cloning and plain conversion retain every JSON member, including unknown nested data.
    internal JsonElement ToJson() => document.Clone();
    internal JsonObject ToPlain() => JsonNode.Parse(document.GetRawText(), documentOptions: new JsonDocumentOptions { MaxDepth = int.MaxValue })!.AsObject();
    internal string ToJsonString() => document.GetRawText();
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
                          f'    internal {kind}(JsonElement document) : base(document) {{ }}\n')
            for member, field in schema['properties'].items():
                optional = member not in schema.get('required', []) or nullable(field, definitions)
                field_type, decode = conversion(field, definitions)
                prop = name(member)
                if prop == kind:
                    prop += "Value"
                if optional:
                    chunks.append(f'    public Presence<{field_type}> {prop} => '
                                  f'{"RequiredNullable" if member in schema.get("required", []) else "Optional"}<{field_type}>({quote(member)}, member => {decode});\n')
                else:
                    _, required_decode = conversion(field, definitions, f'RequiredElement({quote(member)})')
                    chunks.append(f'    public {field_type} {prop} => {required_decode};\n')
            from host_answers import named_object_fields
            for field in named_object_fields(schema, definitions):
                value = schema['properties'][field]
                target = definitions[value['$ref'].split('/')[-1]] if '$ref' in value else value
                if 'name' in target.get('properties', {}):
                    expression = name(field) + '.Name'
                elif 'variants' in target:
                    arms = ', '.join(name(child) + ' value => value.Name' for child, _ in target['variants'])
                    expression = name(field) + ' switch { ' + arms + ', _ => throw new JsonException("Unknown named endpoint.") }'
                else: continue
                chunks.append('    public string ' + name(field) + 'Name => ' + expression + ';\n')
            chunks.append('}\n\n')
        else:
            # Primitive references use their native C# types at the member edge.
            conversion(schema, definitions)
    return ''.join(chunks)


def render_primitive_union(kind, variants, definitions, schemas):
    chunks = [f'public abstract class {kind}\n{{\n'
              '    private readonly JsonElement document;\n'
              f'    private protected {kind}(JsonElement document) {{ this.document = document.Clone(); }}\n'
              '    private protected JsonElement Document => document;\n'
              '    internal JsonElement ToJson() => document.Clone();\n'
              '    internal JsonNode ToPlain() => JsonNode.Parse(document.GetRawText(), documentOptions: new JsonDocumentOptions { MaxDepth = int.MaxValue })!;\n'
              '    internal string ToJsonString() => document.GetRawText();\n'
              f'    internal static {kind} Read(JsonElement document) => document.ValueKind switch\n    {{\n']
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
                      f'    internal {kind}{name(variant)}(JsonElement document) : base(document) {{ }}\n'
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
              f'    private protected {kind}(JsonElement document) : base(document) {{ }}\n'
              f'    internal static {kind} Read(JsonElement document)\n    {{\n'
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


def render_inputs(schema, prepare, graph):
    """Generate writable values from the native Request graph, including authored unions."""
    schema = {**schema, '$defs': dict(schema['$defs'])}
    schema['$defs']['Request'] = {k: v for k, v in schema.items() if k != '$defs'}
    definitions = prepare(graph(schema, ('Request', 'RequestSessionDescriptor', 'RequestReaderFailure', 'EngineSettings')))
    for key, source in list(definitions.items()):
        if 'variants' in source:
            names = [child for child, _ in source['variants']]
            definitions[key] = {'oneOf': [definitions[child] for child in names], '_variant_names': names}
    nodes = {}
    def cname(key):
        return name(key)
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
            nodes.setdefault(path, {'anyOf': [{**value, 'type': k} for k in kind]})
            return cname(path)
        if kind == 'array':
            return 'IReadOnlyList<' + cs_type(value.get('items', {}), path + '_Item') + '>'
        if kind == 'object' and 'properties' in value:
            nodes.setdefault(path, value)
            return cname(path)
        if kind == 'object' and isinstance(value.get('additionalProperties'), dict):
            return 'IReadOnlyList<KeyValuePair<string, ' + cs_type(value['additionalProperties'], path + '_Entry') + '>>'
        return {'string': 'string', 'boolean': 'bool', 'integer': 'long',
                'number': 'double'}.get(kind, 'JsonElement') if kind != 'integer' else {'uint64':'ulong', 'uint32':'uint', 'uint':'ulong'}.get(value.get('format'), 'long')
    def type_of_literal(value):
        return 'string' if isinstance(value, str) else 'bool' if isinstance(value, bool) else 'long'
    def emit(value, expr, path):
        typ = cs_type(value, path)
        if typ in {cname(key) for key in nodes}:
            return 'if (' + expr + ' is null) writer.WriteNullValue(); else ' + expr + '.Write(writer);'
        if typ.startswith('IReadOnlyList<KeyValuePair'):
            return 'writer.WriteStartObject(); foreach (var entry in ' + expr + ') { WriteName(writer, entry.Key); ' + emit(value['additionalProperties'], 'entry.Value', path + '_Entry') + ' } writer.WriteEndObject();'
        if typ.startswith('IReadOnlyList'):
            return 'writer.WriteStartArray(); foreach (var item in ' + expr + ') { ' + emit(value.get('items', {}), 'item', path + '_Item') + ' } writer.WriteEndArray();'
        if typ.startswith('IReadOnlyDictionary'):
            return 'writer.WriteStartObject(); foreach (var entry in ' + expr + ') { WriteName(writer, entry.Key); ' + emit(value['additionalProperties'], 'entry.Value', path + '_Entry') + ' } writer.WriteEndObject();'
        return {'string': 'WriteText(writer, ' + expr + ');',
                'bool': 'writer.WriteBooleanValue(' + expr + ');',
                'long': 'writer.WriteNumberValue(' + expr + ');',
                'ulong': 'writer.WriteNumberValue(' + expr + ');',
                'uint': 'writer.WriteNumberValue(' + expr + ');',
                'double': 'writer.WriteNumberValue(' + expr + ');'}.get(typ, expr + '.WriteTo(writer);')
    reading = set()
    def child_key(key, index, alt):
        names = definitions.get(key, {}).get('_variant_names')
        if names:
            return names[index]
        return key + '_' + alternative_name(alt)
    def mark_read(value, path, wrapper=False):
        if isinstance(value, bool):
            return
        if '$ref' in value:
            key = value['$ref'].removeprefix('#/$defs/')
            if key not in reading:
                reading.add(key)
                mark_read(definitions[key], key, True)
            return
        if isinstance(value.get('type'), list):
            mark_read({'anyOf': [{**value, 'type': kind} for kind in value['type']]}, path + ('_Value' if wrapper else ''))
            return
        alternatives = value.get('oneOf', value.get('anyOf'))
        if alternatives:
            reading.add(path)
            for index, alt in enumerate(alternatives):
                child = child_key(path, index, alt)
                reading.add(child)
                mark_read(alt, child, True)
        elif 'properties' in value:
            reading.add(path)
            for member, prop in value['properties'].items():
                mark_read(prop, path + '_' + member)
        elif value.get('type') == 'array':
            mark_read(value.get('items', {}), path + ('_Value_Item' if wrapper else '_Item'))
        elif isinstance(value.get('additionalProperties'), dict):
            mark_read(value['additionalProperties'], path + ('_Value_Entry' if wrapper else '_Entry'))
    mark_read({'$ref': '#/$defs/RequestDefinition'}, 'RequestDefinition')
    def read(value, expr, path, depth=0):
        typ = cs_type(value, path)
        if typ in {cname(key) for key in nodes}:
            return typ + '.Read(' + expr + ')'
        if typ.startswith('IReadOnlyList<KeyValuePair'):
            item = 'entry' + str(depth)
            element = cs_type(value['additionalProperties'], path + '_Entry')
            return 'Array.AsReadOnly(' + expr + '.EnumerateObject().Select(' + item + ' => new KeyValuePair<string, ' + element + '>(' + item + '.Name, ' + read(value['additionalProperties'], item + '.Value', path + '_Entry', depth + 1) + ')).ToArray())'
        if typ.startswith('IReadOnlyList'):
            item = 'item' + str(depth)
            return 'Array.AsReadOnly(' + expr + '.EnumerateArray().Select(' + item + ' => ' + read(value.get('items', {}), item, path + '_Item', depth + 1) + ').ToArray())'
        if typ.startswith('IReadOnlyDictionary'):
            item = 'entry' + str(depth)
            return 'new System.Collections.ObjectModel.' + typ.replace('IReadOnlyDictionary', 'ReadOnlyDictionary') + '(' + expr + '.EnumerateObject().ToDictionary(' + item + ' => ' + item + '.Name, ' + item + ' => ' + read(value['additionalProperties'], item + '.Value', path + '_Entry', depth + 1) + '))'
        return expr + {'string': '.GetString()!', 'bool': '.GetBoolean()', 'long': '.GetInt64()', 'ulong': '.GetUInt64()', 'uint': '.GetUInt32()', 'double': '.GetDouble()'}.get(typ, '.Clone()')
    def constant_matches(value, expr):
        # Match admitted values, never their authored JSON escape spelling.
        if isinstance(value, str):
            return '(' + expr + '.ValueKind == JsonValueKind.String && ' + expr + '.GetString() == ' + quote(value) + ')'
        if isinstance(value, bool):
            return expr + '.ValueKind == JsonValueKind.' + ('True' if value else 'False')
        if isinstance(value, int):
            return '(' + expr + '.ValueKind == JsonValueKind.Number && ' + expr + '.GetInt64() == ' + str(value) + ')'
        raise ValueError('unsupported Request constant type: ' + type(value).__name__)
    def matches(value, expr):
        if isinstance(value, bool):
            return 'true'
        if '$ref' in value:
            return matches(definitions[value['$ref'].removeprefix('#/$defs/')], expr)
        kinds = {'string': 'String', 'boolean': 'True || ' + expr + '.ValueKind == JsonValueKind.False', 'integer': 'Number', 'number': 'Number', 'object': 'Object', 'array': 'Array', 'null': 'Null'}
        checks = []
        if value.get('type') in kinds:
            checks.append('(' + expr + '.ValueKind == JsonValueKind.' + kinds[value['type']] + ')')
        if 'const' in value:
            checks.append(constant_matches(value['const'], expr))
        if 'properties' in value:
            checks.append(expr + '.ValueKind == JsonValueKind.Object')
            for member in value.get('required', []):
                checks.append(expr + '.TryGetProperty(' + quote(member) + ', out _)')
            for member, prop in value['properties'].items():
                if isinstance(prop, dict) and 'const' in prop:
                    checks.append(constant_matches(prop['const'], expr + '.GetProperty(' + quote(member) + ')'))
        return ' && '.join(checks) if checks else 'true'
    header = '''// Generated from the Rust-derived Request schema; do not edit.
#nullable enable
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.Json;
namespace ThinkThen.Inputs;
public readonly struct AuthoredPresence<T> {
    public bool IsPresent { get; }
    public T Value { get; }
    private AuthoredPresence(T value) { Value = value; IsPresent = true; }
    public static implicit operator AuthoredPresence<T>(T value) => new(value);
}
public abstract class AuthoredDocument {
    private protected AuthoredDocument() {}
    private static readonly System.Text.Encoding StrictUtf8 = new System.Text.UTF8Encoding(false, true);
    private protected static void WriteText(Utf8JsonWriter writer, string? value) { if (value is not null) _ = StrictUtf8.GetByteCount(value); writer.WriteStringValue(value); }
    private protected static void WriteName(Utf8JsonWriter writer, string value) { _ = StrictUtf8.GetByteCount(value); writer.WritePropertyName(value); }
    // Only internal readers retain native-admitted JSON; raw writes bypass host depth validation.
    private protected JsonElement? ParsedDocument { get; init; }
    internal abstract void Write(Utf8JsonWriter writer);
    internal byte[] ToBytes() { using var stream = new MemoryStream(); using (var writer = new Utf8JsonWriter(stream, new JsonWriterOptions { MaxDepth = int.MaxValue })) Write(writer); return stream.ToArray(); }
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
            declaration = 'public abstract class ' + typ + ' : AuthoredDocument { private protected ' + typ + '() {}'
            if key in reading:
                declaration += ' internal static ' + typ + ' Read(JsonElement value) { '
                for index, alt in enumerate(alternatives):
                    declaration += 'if (' + matches(alt, 'value') + ') return ' + cname(child_key(key, index, alt)) + '.Read(value); '
                declaration += 'throw new JsonException("Authored data does not match the native Request schema."); }'
            output.append(declaration + ' }')
            for index, alt in enumerate(alternatives):
                child = child_key(key, index, alt)
                nodes[child] = {**alt, '_base': typ}
            continue
        base = value.get('_base', 'AuthoredDocument')
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
                lines.append('public ' + ('required ' + ptype if mandatory else 'AuthoredPresence<' + ptype + '>') + ' ' + pname + ' { get; init; }')
                body = 'writer.WritePropertyName(' + quote(member) + '); ' + emit(prop, pname if mandatory else pname + '.Value', key + '_' + member)
                writes.append(body if mandatory else 'if (' + pname + '.IsPresent) { ' + body + ' }')
            writes.append('writer.WriteEndObject();')
        elif value.get('type') == 'null':
            writes = ['writer.WriteNullValue();']
        elif 'const' in value:
            literal = value['const']
            writes = [emit(value, quote(literal) if isinstance(literal, str) else str(literal).lower(), key + '_Value')]
        else:
            stripped = {k: v for k, v in value.items() if k != '_base'}
            ptype = cs_type(stripped, key + '_Value')
            lines.append('public required ' + ptype + ' Value { get; init; }')
            writes = [emit(stripped, 'Value', key + '_Value')]
        if key in reading:
            assignments = ['ParsedDocument = value.Clone()']
            if properties is not None:
                for member, prop in properties.items():
                    if isinstance(prop, dict) and 'const' in prop:
                        continue
                    ptype = cs_type(prop, key + '_' + member)
                    pname = name(member)
                    expr = read(prop, 'value.GetProperty(' + quote(member) + ')', key + '_' + member)
                    if member not in required:
                        expr = 'value.TryGetProperty(' + quote(member) + ', out _) ? (AuthoredPresence<' + ptype + '>)(' + expr + ') : default'
                    assignments.append(pname + ' = ' + expr)
            elif value.get('type') != 'null' and 'const' not in value:
                assignments.append('Value = ' + read(stripped, 'value', key + '_Value'))
            lines.append('internal ' + ('new ' if '_base' in value else '') + 'static ' + typ + ' Read(JsonElement value) => new() { ' + ', '.join(assignments) + ' };')
        lines.append('internal override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true); return; } ' + ' '.join(writes) + ' }')
        lines.append('}')
        output.append('\n'.join(lines))
    builders = ['public static class QuestionBuilders {']
    for child in definitions['RequestDefinition']['_variant_names']:
        source = definitions[child]
        for field, value in source.get('properties', {}).items():
            if '$ref' not in value or field not in source.get('required', []): continue
            key = value['$ref'].split('/')[-1]
            target = definitions[key]
            if not target.get('title', '').endswith('Question') or 'properties' not in target: continue
            builders += ['public static RequestQuestionDefinition ' + name(field) + '(' + cname(key) + ' question) {',
                         'ArgumentNullException.ThrowIfNull(question);',
                         'return new RequestQuestionDefinition { Value = new ' + cname(child) + ' { ' + name(field) + ' = question } };', '}']
    return header + '\n\n'.join(output) + '\n' + '\n'.join(builders + ['}']) + '\n'


def named_calls(functions):
    """The result graph emits named calls and their typed input overloads."""
    return "// Generated from canonical RequestFunction; do not edit.\n"


def answer_helpers(definitions):
    """Project the canonical call metadata into the C# scalar/list idiom."""
    from host_answers import call_results
    def carriers(key, seen=()):
        if key == 'completesessionJudgment': return []
        if key in seen: return None
        source = definitions[key]
        for child, _ in source.get('variants', []):
            result = carriers(child, seen + (key,))
            if result is not None: return result
        for field, value in source.get('properties', {}).items():
            if isinstance(value, dict) and '$ref' in value:
                result = carriers(value['$ref'].split('/')[-1], seen + (key,))
                if result is not None: return [(key, field)] + result
        return None
    route = carriers('completesessionPacket')
    if not route: raise ValueError('C# needs a normalized judgment carrier route')
    lines = ['#nullable enable', 'using ThinkThen.Inputs;', 'using ThinkThen.Results;',
             'using System.Text.Json;', 'namespace ThinkThen;',
             'public sealed record CallResult<TValue, TProbability, TResults> : OwnedCall {',
             'public TValue Value { get; }', 'public TProbability Probability { get; }',
             'public TResults Results { get; }', 'public Facts Facts { get; }',
             'internal CallResult(OwnedCall call, TValue value, TProbability probability, TResults results) : base(call.Packets, call.Terminal) {',
             'Value = value; Probability = probability; Results = results;',
             'Facts = call.Terminal.Facts.State == PresenceState.Value ? call.Terminal.Facts.Value : throw new JsonException("Successful call has no facts.");',
             '}', '}', 'public sealed partial class Engine {']
    for function, spec in call_results(definitions).items():
        payload = spec['receipt_payload']
        array = payload.get('type') == 'array'
        receipt_type, _ = conversion(payload['items'] if array else payload, definitions)
        results_type = 'IReadOnlyList<' + receipt_type + '>'
        carrier = name(spec['carrier'])
        projection = name(spec['value_projection'])
        scalar = spec['scalar_value']
        aggregate_value = spec['aggregate_value']
        item_schema = aggregate_value['items'] if array else aggregate_value
        item_type, _ = conversion(scalar if scalar is not None else item_schema, definitions)
        if nullable(scalar if scalar is not None else item_schema, definitions): item_type += '?'
        mode = spec['probability_mode']
        probability_type = 'IReadOnlyList<double>' if mode == 'items' and function != 'rank' else 'double?'
        per_input = spec['call_shape'] == 'input'
        for scalar_input, input_type in [(True, 'RequestInputText'), (True, 'RequestInputJson'),
                                       (False, 'RequestInputRecords'), (False, 'RequestInputSource'),
                                       (False, 'RequestInputUnits'), (False, 'RequestInputEntities'),
                                       (False, 'RequestInputFeed'), (False, 'IAsyncEnumerable<RequestSessionDescriptor>')]:
            single = scalar_input and per_input
            value_type = item_type if single or spec['call_shape'] == 'aggregate' else 'IReadOnlyList<' + item_type + '>'
            odds_type = probability_type if single or spec['call_shape'] == 'aggregate' or mode == 'null' else 'IReadOnlyList<' + probability_type + '>'
            result_type = 'CallResult<' + ', '.join((value_type, odds_type, results_type)) + '>'
            feed = input_type.startswith('IAsyncEnumerable')
            suffix = 'Scalar' if single else ''
            lines += ['public async Task<' + result_type + '> ' + function.title() + 'Async(RequestQuestion question, ' + input_type + ' input,',
                      'RequestOptions? options = null, CancellationToken cancellation = default) {',
                      'ArgumentNullException.ThrowIfNull(input);',
                      'var call = await ExecuteSessionAsync(() => StartNamedSession(' + quote(function) + ', question, ' + ('FeedInput' if feed else 'input') + ', options), ' + ('input' if feed else 'null') + ', cancellation).ConfigureAwait(false);',
                      'return Project' + function.title() + suffix + '(call);', '}']
        values_expr = 'row.' + projection
        receipt_schema = definitions[(payload['items'] if array else payload)['$ref'].split('/')[-1]]
        optional_projection = spec['value_projection'] not in receipt_schema.get('required', [])
        if nullable(item_schema, definitions):
            values_expr = 'row.' + projection + '.State == PresenceState.Value ? row.' + projection + '.Value : (' + item_type + ')null'
        elif optional_projection:
            values_expr += '.Value'
        if scalar:
            variant = next(child for child, _ in definitions['completesessionJudgment']['variants'] if definitions[child].get('x-request-function') == function)
            pattern = name(variant) + ' judgment'
            for key, field in reversed(route):
                pattern = name(key) + ' { ' + name(field) + ': ' + pattern + ' }'
            judgment_value = 'judgment.Value'
            if nullable(scalar, definitions): judgment_value = 'judgment.Value.State == PresenceState.Value ? judgment.Value.Value : null'
            values_decl = 'var values = new List<' + item_type + '>(); foreach (var packet in call.Packets) if (packet is ' + pattern + ' observation && observation.Function.Value == ' + quote(function) + ') values.Add(' + judgment_value + ');'
        else:
            values_decl = 'var values = results.Select(row => ' + values_expr + ').ToArray();'
        if mode == 'null': probability_expr = '(double?)null'
        elif mode == 'yes': probability_expr = '(double?)((AnswerYesNo)row.Answer).Probability'
        elif mode == 'picked': probability_expr = 'row.Answer is ' + ('FindAnswer' if function == 'find' else 'AnswerChoice') + ' odds ? (double?)odds.Probabilities[odds.Pick] : null'
        elif function == 'rank': probability_expr = '(double?)((AnswerYesNo)row.Answer).Probability'
        elif function == 'relate': probability_expr = 'Array.AsReadOnly(row.Value.Select(edge => edge.Probability).ToArray())'
        else: probability_expr = 'RecognitionProbabilities(row)'
        single_aggregate = spec['call_shape'] == 'aggregate'
        value_type = item_type if single_aggregate else 'IReadOnlyList<' + item_type + '>'
        odds_type = probability_type if single_aggregate or mode == 'null' else 'IReadOnlyList<' + probability_type + '>'
        result_type = 'CallResult<' + ', '.join((value_type, odds_type, results_type)) + '>'
        lines += ['private static ' + result_type + ' Project' + function.title() + '(OwnedCall call) {',
                  'var receipts = new List<' + receipt_type + '>();', 'foreach (var packet in call.Packets) {',
                  'if (packet is ' + carrier + ' aggregate) ' + ('receipts.AddRange(aggregate.Value);' if array else 'receipts.Add(aggregate.Value);')]
        for child, _ in definitions['completesessionPacket']['variants']:
            fields = definitions[child].get('properties', {})
            if array and fields.get('kind', {}).get('const') == 'row' and fields.get('function', {}).get('const') == function and fields.get('value') == payload['items']:
                lines += ['else if (packet is ' + name(child) + ' row) receipts.Add(row.Value);']
        lines += ['}', 'var results = receipts.AsReadOnly();', values_decl]
        if single_aggregate: lines += ['if (results.Count != 1) throw new JsonException("Missing aggregate result.");']
        odds = '(double?)null' if mode == 'null' else ('results.Select(row => ' + probability_expr + ').Single()' if single_aggregate else 'Array.AsReadOnly(results.Select(row => ' + probability_expr + ').ToArray())')
        value = 'values.Single()' if single_aggregate else ('values.AsReadOnly()' if scalar else 'Array.AsReadOnly(values)')
        lines += ['return new(call, ' + value + ', ' + odds + ', results);', '}']
        if per_input:
            scalar_type = 'CallResult<' + ', '.join((item_type, probability_type, results_type)) + '>'
            lines += ['private static ' + scalar_type + ' Project' + function.title() + 'Scalar(OwnedCall call) {',
                      'var result = Project' + function.title() + '(call);',
                      'return new(call, result.Value.Single(), ' + ('result.Probability' if mode == 'null' else 'result.Probability.Single()') + ', result.Results);', '}']
    lines += ['private static IReadOnlyList<double> RecognitionProbabilities(Recognition row) {',
              'if (row.Value is RecognizeModeProposals boundary) return Array.AsReadOnly(boundary.Proposals.Select(proposal => proposal.Probability).ToArray());',
              'var entities = ((RecognizeEntities)row.Value).Entities;',
              'var answer = (RecognitionOddsRecognizeAnswer)row.Answer;',
              'return Array.AsReadOnly(entities.Select(entity => answer.Proposals.Single(proposal => proposal.Kept && proposal.Selected.State == PresenceState.Value && proposal.Selected.Value.Start == entity.Start && proposal.Selected.Value.End == entity.End && proposal.Kind.State == PresenceState.Value && proposal.Kind.Value == entity.Kind).SpanProbability).ToArray());', '}', '}']
    return '\n'.join(lines) + '\n'
