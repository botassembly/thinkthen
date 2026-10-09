"""C# immutable result views. Conversion adds no schema validation or policy."""
import json
import re


def name(value):
    words = re.findall(r'[A-Z]?[a-z]+|[A-Z]+(?![a-z])|[0-9]+',
                       value.removeprefix('complete'))
    return ''.join(word[0].upper() + word[1:] for word in words)


def quote(value):
    return json.dumps(value, ensure_ascii=True)


def enum_values(schema):
    variants = schema.get('oneOf', schema.get('anyOf', []))
    if variants and all(item.get('type') == 'string' and 'const' in item
                        for item in variants):
        return [item['const'] for item in variants]
    return None


def nullable(schema):
    kinds = schema.get('type', [])
    return (kinds == 'null' or isinstance(kinds, list) and 'null' in kinds or
            any(item.get('type') == 'null' for item in schema.get('anyOf', [])))


def shape(schema):
    schema = dict(schema)
    if isinstance(schema.get('type'), list):
        kinds = [kind for kind in schema['type'] if kind != 'null']
        if len(kinds) != 1:
            raise ValueError(f'C# target needs a typed union for {schema}')
        schema['type'] = kinds[0]
    if 'anyOf' in schema:
        variants = [item for item in schema['anyOf'] if item.get('type') != 'null']
        if len(variants) == 1:
            return variants[0]
    return schema


def conversion(schema, definitions, expression='member'):
    schema = shape(schema)
    if '$ref' in schema:
        key = schema['$ref'].removeprefix('#/$defs/')
        target = definitions[key]
        if 'properties' in target or enum_values(target):
            kind = name(key)
            argument = f'{expression}.GetString()!' if enum_values(target) else expression
            return kind, f'new {kind}({argument})'
        return conversion(target, definitions, expression)
    kind = schema.get('type')
    if kind == 'array':
        element, decode = conversion(schema['items'], definitions, 'item')
        return f'IReadOnlyList<{element}>', (
            f'Array.AsReadOnly({expression}.EnumerateArray().Select(item => {decode}).ToArray())')
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
    protected JsonElement Required(string name) => document.GetProperty(name);
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
    for key, schema in definitions.items():
        kind = name(key)
        values = enum_values(schema)
        if values:
            chunks.append(f'public readonly record struct {kind}(string Value)\n{{\n')
            for value in values:
                chunks.append(f'    public static {kind} {name(value)} => new({quote(value)});\n')
            chunks.append('}\n\n')
        elif 'properties' in schema:
            chunks.append(f'public sealed class {kind} : ResultObject\n{{\n'
                          f'    public {kind}(JsonElement document) : base(document) {{ }}\n')
            for member, field in schema['properties'].items():
                optional = member not in schema.get('required', []) or nullable(field)
                field_type, decode = conversion(field, definitions)
                prop = name(member)
                if optional:
                    chunks.append(f'    public Presence<{field_type}> {prop} => '
                                  f'Optional<{field_type}>({quote(member)}, member => {decode});\n')
                else:
                    _, required_decode = conversion(field, definitions, f'Required({quote(member)})')
                    chunks.append(f'    public {field_type} {prop} => {required_decode};\n')
            chunks.append('}\n\n')
        else:
            # Primitive references use their native C# types at the member edge.
            conversion(schema, definitions)
    return ''.join(chunks)
