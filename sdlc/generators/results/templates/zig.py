"""Zig payload declarations; Rust owns admission and semantic validation."""
import json
import re


def name(key):
    return ''.join(word[:1].upper() + word[1:] for word in re.split(r'[_-]', key.removeprefix('complete')))


def identifier(value):
    return '@"' + value + '"'


def render(definitions):
    nodes = dict(definitions)
    def typed(source, path):
        if isinstance(source, bool) or not source:
            return 'std.json.Value'
        if '$ref' in source:
            return name(source['$ref'].removeprefix('#/$defs/'))
        if 'variants' in source or 'anyOf' in source or 'oneOf' in source:
            nodes.setdefault(path, source)
            return name(path)
        kind = source.get('type')
        if isinstance(kind, list):
            if len(kind) == 2 and 'null' in kind:
                return '?' + typed({**source, 'type': next(k for k in kind if k != 'null')}, path)
            nodes.setdefault(path, {'anyOf': [{**source, 'type': k} for k in kind]})
            return name(path)
        alternatives = source.get('oneOf', source.get('anyOf', []))
        if alternatives and all(isinstance(f.get('const'), str) for f in alternatives):
            chunks.append('pub const ' + typename + ' = enum { ' + ', '.join(identifier(f['const']) for f in alternatives) + ' };\n')
        elif 'properties' in source:
            nodes.setdefault(path, source)
            return name(path)
        if kind == 'array':
            return '[]const ' + typed(source.get('items', {}), path + '_item')
        if kind == 'object' and isinstance(source.get('additionalProperties'), dict):
            return 'Map(' + typed(source['additionalProperties'], path + '_entry') + ')'
        if 'const' in source:
            value = source['const']
            if isinstance(value, str):
                return 'enum { ' + identifier(value) + ' }'
            if isinstance(value, bool):
                return 'bool'
            return 'i64'
        if kind == 'integer':
            return {'uint': 'u64', 'uint64': 'u64', 'uint32': 'u32', 'int32': 'i32'}.get(source.get('format'), 'i64')
        return {'string': '[]const u8', 'number': 'f64', 'boolean': 'bool', 'null': 'void'}.get(kind, 'std.json.Value')
    chunks = [SUPPORT]
    emitted = set()
    while len(emitted) < len(nodes):
        key = next(k for k in nodes if k not in emitted)
        emitted.add(key)
        source = nodes[key]
        typename = name(key)
        alternatives = source.get('oneOf', source.get('anyOf', []))
        if alternatives and all(isinstance(f.get('const'), str) for f in alternatives):
            chunks.append('pub const ' + typename + ' = enum { ' + ', '.join(identifier(f['const']) for f in alternatives) + ' };\n')
        elif 'properties' in source:
            chunks.append('pub const ' + typename + ' = struct {\n')
            for member, field in source['properties'].items():
                native = typed(field, key + '_' + member)
                required = member in source.get('required', [])
                default = ''
                resolved = nodes[field['$ref'].removeprefix('#/$defs/')] if isinstance(field, dict) and '$ref' in field else field
                constants = resolved.get('oneOf', resolved.get('anyOf', [])) if isinstance(resolved, dict) else []
                literal = resolved.get('const') if isinstance(resolved, dict) else None
                if len(constants) == 1 and isinstance(constants[0].get('const'), str):
                    literal = constants[0]['const']
                if literal is not None:
                    default = ' = .' + identifier(literal) if isinstance(literal, str) else ' = ' + json.dumps(literal)
                elif not required:
                    native, default = '?' + native, ' = null'
                chunks.append('    ' + identifier(member) + ': ' + native + default + ',\n')
            chunks.append('};\n')
        elif 'variants' in source or 'anyOf' in source or 'oneOf' in source:
            if 'variants' in source:
                variants = [(child.removeprefix(key + '_').removeprefix('fields_'), {'$ref': '#/$defs/' + child}) for child, _ in source['variants']]
            else:
                variants = []
                for index, field in enumerate(source.get('anyOf', source.get('oneOf', []))):
                    kind = field.get('type', '')
                    tag = field.get('const') if isinstance(field.get('const'), str) else kind if isinstance(kind, str) and kind else 'alternative' + str(index)
                    variants.append((tag, field))
                if len({tag for tag, _ in variants}) != len(variants):
                    variants = [('alternative' + str(i), f) for i, (_, f) in enumerate(variants)]
            chunks.append('pub const ' + typename + ' = union(enum) {\n')
            for tag, field in variants:
                chunks.append('    ' + identifier(tag) + ': ' + typed(field, key + '_' + tag) + ',\n')
            chunks.append('    pub fn jsonStringify(self: @This(), writer: anytype) !void {\n        switch (self) {\n')
            for tag, field in variants:
                chunks.append('            .' + identifier(tag) + ' => ' + ('' if field.get('type') == 'null' else '|value| ') + 'try writer.write(' + ('null' if field.get('type') == 'null' else 'value') + '),\n')
            chunks.append('        }\n    }\n};\n')
        else:
            chunks.append('pub const ' + typename + ' = ' + typed(source, key + '_value') + ';\n')
    return '\n'.join(chunks)


SUPPORT = '''// Generated from the canonical Rust schema; do not edit.
const std = @import("std");
/// Ordered authored maps preserve member ordering during transport.
pub fn Map(comptime T: type) type {
    return struct {
        entries: []const struct { key: []const u8, value: T },
        pub fn jsonStringify(self: @This(), writer: anytype) !void {
            try writer.beginObject();
            for (self.entries) |entry| {
                try writer.objectField(entry.key);
                try writer.write(entry.value);
            }
            try writer.endObject();
        }
    };
}
'''
