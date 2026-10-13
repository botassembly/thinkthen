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
                    tag = field.get('const') if isinstance(field.get('const'), str) else kind if isinstance(kind, str) and kind else re.sub(r'(?<!^)(?=[A-Z])', '_', field.get('$ref', '').split('/')[-1]).lower() or 'value' + str(index)
                    variants.append((tag, field))
                if len({tag for tag, _ in variants}) != len(variants):
                    variants = [(tag + '_' + str(i), f) for i, (tag, f) in enumerate(variants)]
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


def calls(definitions):
    fields = definitions['RequestCall']['properties']
    lines = ['// Generated named calls from the canonical request schema; do not edit.', 'const tt = @import("thinkthen.zig");', 'const inputs = @import("request_generated.zig");', 'pub const Arguments = struct {', '    question: inputs.RequestQuestion,', '    input: inputs.RequestInput,', '    options: ?inputs.RequestOptions = null,', '};']
    for function in definitions['RequestFunction']['oneOf']:
        value = function['const']
        lines += [f'pub fn {value}(engine: *tt.Engine, arguments: Arguments) !tt.session.Session {{', f'    return tt.session.startNamed(engine, .{value}, arguments);', '}']
    return '\n'.join(lines) + '\n'

def render_results(definitions):
    """Owned Zig values copied from the canonical decoded native graph."""
    from c import normalized, nullable, ident, variant_member, container_key
    def result_name(key):
        return 'JudgmentValue' if key == 'completevalue' else name(key)
    nodes, pending = {}, ['completesessionPacket', 'completeImmediateFacts']
    def references(value):
        if isinstance(value, dict):
            if '$ref' in value: yield value['$ref'].split('/')[-1]
            for child, _ in value.get('variants', []): yield child
            for item in value.values(): yield from references(item)
        elif isinstance(value, list):
            for item in value: yield from references(item)
    while pending:
        key = pending.pop()
        if key in nodes: continue
        nodes[key] = definitions[key]
        pending.extend(references(definitions[key]))
    declarations, readers = [], []
    def native_type(source, path):
        source = normalized(source)
        if '$ref' in source: return result_name(source['$ref'].split('/')[-1])
        if source == {} or source.get('type') == 'null': return 'Value'
        if any(key in source for key in ('properties', 'enum', 'variants', 'primitive_variants')):
            nodes.setdefault(path, source)
            return result_name(path)
        kind = source.get('type')
        if isinstance(source.get('const'), str): return '[]const u8'
        if kind == 'array': return '[]const ' + native_type(source['items'], container_key(source, path) + '_item')
        if kind == 'object': return 'Map(' + native_type(source['additionalProperties'], container_key(source, path) + '_entry_value') + ')'
        if kind == 'integer': return 'u64' if source.get('minimum', -1) >= 0 or source.get('format', '').startswith('uint') else 'i64'
        return {'string': '[]const u8', 'number': 'f64', 'boolean': 'bool'}[kind]
    def decode(source, path, value):
        source = normalized(source)
        if '$ref' in source: return 'try read' + result_name(source['$ref'].split('/')[-1]) + '(a, ' + value + '[0])'
        if source == {} or source.get('type') == 'null': return 'try readValue(a, ' + value + '[0])'
        if any(key in source for key in ('properties', 'enum', 'variants', 'primitive_variants')):
            nodes.setdefault(path, source)
            return 'try read' + result_name(path) + '(a, ' + value + '[0])'
        kind = source.get('type')
        if kind == 'string' or isinstance(source.get('const'), str): return 'try a.dupe(u8, text(' + value + '))'
        if kind == 'boolean': return value + ' != 0'
        if kind in ('number', 'integer'): return value
        typ = native_type(source, path)
        element = source['items'] if kind == 'array' else source['additionalProperties']
        element_path = container_key(source, path) + ('_item' if kind == 'array' else '_entry_value')
        item = 'entry' if kind == 'array' else 'entry.value'
        expression = decode(element, element_path, item)
        if kind == 'array':
            return 'blk: { const items = try a.alloc(' + native_type(element, element_path) + ', ' + value + '.len); for (items, 0..) |*out, i| { const entry = ' + value + '.data[i]; out.* = ' + expression + '; } break :blk items; }'
        return 'blk: { const entries = try a.alloc(' + typ + '.Entry, ' + value + '.len); for (entries, 0..) |*out, i| { const entry = ' + value + '.data[i]; out.* = .{ .key = try a.dupe(u8, text(entry.name)), .value = ' + expression + ' }; } break :blk .{ .entries = entries }; }'
    emitted = set()
    while len(emitted) < len(nodes):
        key = next(key for key in nodes if key not in emitted)
        emitted.add(key)
        source, typ = normalized(nodes[key]), result_name(key)
        body = []
        if 'properties' in source:
            members = []
            for field, schema in source['properties'].items():
                child_path = key + '_field_' + field
                child = native_type(schema, child_path)
                is_optional = field not in source.get('required', [])
                native_presence = is_optional or nullable(schema, definitions)
                if native_presence: child = 'Presence(' + child + ')'
                members.append('    ' + identifier(field) + ': ' + child + ',')
                value = 'v.' + identifier({'true': 'true_', 'false': 'false_'}.get(field, field))
                expression = decode(schema, child_path, value + ('.value' if native_presence else ''))
                if native_presence:
                    expression = 'switch (' + value + '.presence) { 0 => .missing, 1 => .null_value, 2 => .{ .value = ' + expression + ' }, else => return error.InvalidNativeDiscriminator }'
                body.append('        .' + identifier(field) + ' = ' + expression + ',')
            members.append('    extensions: Map(Value),')
            body.append('        .extensions = try readObject(a, v.extensions),')
            declarations.append('pub const ' + typ + ' = struct {\n' + '\n'.join(members) + '\n};')
            body = ['    return .{', *body, '    };']
        elif 'variants' in source:
            members = []
            body = ['    return switch (v.kind) {']
            for index, (child, _) in enumerate(source['variants'], 1):
                arm = variant_member(key, child)
                members.append('    ' + identifier(arm) + ': ' + result_name(child) + ',')
                body.append('        ' + str(index) + ' => .{ .' + identifier(arm) + ' = try read' + result_name(child) + '(a, v.data.' + identifier(arm) + '[0]) },')
            body += ['        else => error.InvalidNativeDiscriminator,', '    };']
            declarations.append('pub const ' + typ + ' = union(enum) {\n' + '\n'.join(members) + ('\n    pub fn jsonStringify(self: @This(), writer: anytype) !void { try writeNative(self, writer); }' if key == 'completesessionPacket' else '') + '\n};')
        elif 'enum' in source:
            declarations.append('pub const ' + typ + ' = enum { ' + ', '.join(identifier(value) for value in source['enum']) + ' };')
            body = ['    _ = a;', '    return switch (v.kind) {', *('        ' + str(i) + ' => .' + identifier(value) + ',' for i, value in enumerate(source['enum'], 1)), '        else => error.InvalidNativeDiscriminator,', '    };']
        elif 'primitive_variants' in source:
            members = []
            body = ['    return switch (v.kind) {']
            for index, kind in enumerate(source['primitive_variants'], 1):
                if kind == 'null':
                    members.append('    null_value,')
                    body.append('        ' + str(index) + ' => .null_value,')
                else:
                    schema = source['primitive_schemas'][kind]
                    path = key + '_' + kind
                    members.append('    ' + identifier(kind) + ': ' + native_type(schema, path) + ',')
                    body.append('        ' + str(index) + ' => .{ .' + identifier(kind) + ' = ' + decode(schema, path, 'v.data.' + identifier(kind)) + ' },')
            declarations.append('pub const ' + typ + ' = union(enum) {\n' + '\n'.join(members) + '\n};')
            body += ['        else => error.InvalidNativeDiscriminator,', '    };']
        else:
            declarations.append('pub const ' + typ + ' = ' + native_type(source, key + '_value') + ';')
            expression = decode(source, key + '_value', 'v.value')
            if 'a,' not in expression and 'a.' not in expression: body.append('    _ = a;')
            body.append('    return ' + expression + ';')
        readers.append('pub fn read' + typ + '(a: std.mem.Allocator, v: c.' + ident(key) + ') anyerror!' + typ + ' {\n' + '\n'.join(body) + '\n}')
    aliases = ['Presence', 'Map', 'Number', 'Value', *(result_name(key) for key in emitted)]
    types = '\npub const Types = struct {\n' + '\n'.join('    pub const ' + typ + ' = @import("results_generated.zig").' + typ + ';' for typ in sorted(set(aliases))) + '\n};\n'
    return RESULT_SUPPORT + '\n\n'.join(declarations + readers) + types


RESULT_SUPPORT = '''// Generated owned values from the canonical native packet graph; do not edit.
const std = @import("std");
const c = @import("native.zig").c;
pub fn Presence(comptime T: type) type {
    return union(enum) { missing, null_value, value: T };
}
pub fn Map(comptime T: type) type {
    return struct {
        pub const Entry = struct { key: []const u8, value: T };
        entries: []const Entry,
    };
}
pub const Number = union(enum) { signed: i64, unsigned: u64, decimal: f64 };
pub const Value = union(enum) {
    null_value, boolean: bool, number: Number, string: []const u8, array: []const Value, object: Map(Value),
    pub fn jsonStringify(self: @This(), writer: anytype) !void { try writeNative(self, writer); }
};
fn writeNative(value: anytype, writer: anytype) error{WriteFailed}!void {
    const T = @TypeOf(value);
    switch (@typeInfo(T)) {
        .@"union" => switch (value) { inline else => |member| { if (@TypeOf(member) == void) try writer.write(null) else try writeNative(member, writer); } },
        .@"struct" => {
            try writer.beginObject();
            if (@hasField(T, "entries")) {
                for (value.entries) |entry| { try writer.objectField(entry.key); try writeNative(entry.value, writer); }
            } else inline for (std.meta.fields(T)) |field| {
                const member = @field(value, field.name);
                if (comptime std.mem.eql(u8, field.name, "extensions")) {
                    for (member.entries) |entry| { try writer.objectField(entry.key); try writeNative(entry.value, writer); }
                } else {
                    const omit = if (comptime @typeInfo(field.type) == .@"union" and @hasField(field.type, "missing")) member == .missing else false;
                    if (!omit) { try writer.objectField(field.name); try writeNative(member, writer); }
                }
            }
            try writer.endObject();
        },
        .pointer => |info| {
            if (T == []const u8) return writer.write(value);
            if (info.size == .one) return writeNative(value.*, writer);
            try writer.beginArray(); for (value) |member| try writeNative(member, writer); try writer.endArray();
        },
        else => try writer.write(value),
    }
}
fn text(v: c.thinkthen_complete_utf8_v1) []const u8 {
    return if (v.len == 0) &.{} else v.data[0..v.len];
}
fn readNumber(v: c.thinkthen_complete_utf8_v1) !Number {
    const source = text(v);
    if (std.mem.indexOfAny(u8, source, ".eE") != null) return .{ .decimal = try std.fmt.parseFloat(f64, source) };
    if (source.len != 0 and source[0] == '-') return .{ .signed = try std.fmt.parseInt(i64, source, 10) };
    return .{ .unsigned = try std.fmt.parseInt(u64, source, 10) };
}
/// Project a decoded native value into a generated typed record.
pub fn readAs(comptime T: type, v: Value) anyerror!T {
    switch (@typeInfo(T)) {
        .optional => |info| return if (v == .null_value) null else try readAs(info.child, v),
        .pointer => return if (T == []const u8 and v == .string) v.string else error.InvalidNativeValue,
        .bool => return if (v == .boolean) v.boolean else error.InvalidNativeValue,
        .int => return switch (v.number) { .unsigned => |n| std.math.cast(T, n) orelse error.InvalidNativeValue, .signed => |n| std.math.cast(T, n) orelse error.InvalidNativeValue, else => error.InvalidNativeValue },
        .@"struct" => {
            if (v != .object) return error.InvalidNativeValue;
            var result: T = undefined;
            inline for (std.meta.fields(T)) |field| {
                var found = false;
                for (v.object.entries) |entry| if (std.mem.eql(u8, entry.key, field.name)) {
                    @field(result, field.name) = try readAs(field.type, entry.value);
                    found = true;
                    break;
                };
                if (!found) return error.MissingNativeField;
            }
            return result;
        },
        else => return error.InvalidNativeValue,
    }
}
pub fn readObject(a: std.mem.Allocator, v: anytype) anyerror!Map(Value) {
    const entries = try a.alloc(Map(Value).Entry, v.len);
    for (entries, 0..) |*out, i| out.* = .{ .key = try a.dupe(u8, text(v.data[i].name)), .value = try readValue(a, v.data[i].value[0]) };
    return .{ .entries = entries };
}
pub fn readValue(a: std.mem.Allocator, v: c.thinkthen_complete_json_v1) anyerror!Value {
    return switch (v.kind) {
        1 => .null_value,
        2 => .{ .boolean = v.data.boolean != 0 },
        3 => .{ .number = try readNumber(v.data.number) },
        4 => .{ .string = try a.dupe(u8, text(v.data.string)) },
        5 => blk: {
            const items = try a.alloc(Value, v.data.array.len);
            for (items, 0..) |*out, i| out.* = try readValue(a, v.data.array.data[i][0]);
            break :blk .{ .array = items };
        },
        6 => .{ .object = try readObject(a, v.data.object) },
        else => error.InvalidNativeDiscriminator,
    };
}
'''
