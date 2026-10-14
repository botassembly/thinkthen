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
    from host_answers import scalar_input_selectors
    scalar = scalar_input_selectors(definitions)
    lines = ['// Generated named calls from the canonical request schema; do not edit.', 'const tt = @import("thinkthen.zig");', 'const inputs = @import("request_generated.zig");', 'const values = @import("results_generated.zig");',
             'pub fn Arguments(comptime Input: type) type {',
             '    _ = inputTag(Input);',
             '    return struct { question: inputs.RequestQuestion, input: Input, options: ?inputs.RequestOptions = null };', '}',
             'pub fn inputTag(comptime Input: type) inputs.RequestInput {']
    for child, _ in definitions['RequestInput']['variants']:
        payload = definitions[child]
        tag = payload['properties']['kind']['const']
        lines += [f'    if (Input == inputs.{name(child)}) return .{{ .{tag} = undefined }};']
    lines += ['    @compileError("use a concrete generated input payload");', '}',
              'pub fn listInput(comptime Input: type) bool {',
              '    _ = inputTag(Input);', '    return ' + ' and '.join('Input != inputs.' + name(child) for child in scalar) + ';', '}',
              'pub fn nativeInput(input: anytype) inputs.RequestInput {',
              '    return @unionInit(inputs.RequestInput, @tagName(inputTag(@TypeOf(input))), input);', '}']
    for function in definitions['RequestFunction']['oneOf']:
        value = function['const']
        lines += [f'pub fn {value}(engine: *tt.Engine, arguments: anytype) !tt.Result(values.NamedCallResult(.{value}, @TypeOf(arguments.input))) {{',
                  f'    return tt.session.completeNamed(engine, .{value}, arguments);', '}']
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
            declarations.append('pub const ' + typ + ' = union(enum) {\n' + '\n'.join(members) + '\n};')
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
    return RESULT_SUPPORT + '\n\n'.join(declarations + readers) + types + answer_helpers(definitions) + call_helpers(definitions) + public_call_types(definitions) + public_value_helpers(definitions)


RESULT_SUPPORT = '''// Generated owned values from the canonical native packet graph; do not edit.
const std = @import("std");
const c = @import("native.zig").c;
pub fn Presence(comptime T: type) type {
    return union(enum) {
        missing, null_value, value: T,
        pub fn optional(self: @This()) ?T { return switch (self) { .value => |v| v, else => null }; }
    };
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
};
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


def answer_helpers(definitions):
    """Project the shared native observations; Rust owns all answer semantics."""
    from host_answers import scalar_answers, aggregate_answers, aggregate_projection
    lines = ['\npub const AnswerView = struct {',
             '    packet: *const SessionPacket,', '    allocator: std.mem.Allocator,',
             '    fn row(self: @This()) !SessionObservedRow {',
             '        return switch (self.packet.*) { .observation => |v| switch (v.value) { .row => |r| r.value, else => error.WrongAnswer }, else => error.WrongAnswer };', '    }',
             '    fn judgment(self: @This()) !SessionJudgment {',
             '        return switch (try self.row()) { .judgment => |v| v.value, else => error.WrongAnswer };', '    }']
    scalar_types = {'decide': '?bool', 'choose': '?[]const u8', 'score': 'f64', 'tag': '[]const []const u8'}
    for function, tag, payload in scalar_answers(definitions):
        expression = 'v.value.optional()' if function in ('decide', 'choose') else 'v.value'
        lines += [f'    pub fn {function}(self: @This()) !{scalar_types[function]} {{',
                  f'        return switch (try self.judgment()) {{ .{tag} => |v| {expression}, else => error.WrongAnswer }};', '    }']
    for function, tag, typ in [('find', 'find', '?u64'), ('annotate', 'annotated', '[]const SessionAnnotation'), ('recognize', 'recognized', 'SessionRecognition'), ('relate', 'relations', '[]const SessionRelationEdge')]:
        expr = 'v.value.optional()' if function == 'find' else 'v.value'
        lines += [f'    pub fn {function}(self: @This()) !{typ} {{',
                  f'        return switch (try self.row()) {{ .{tag} => |v| {expr}, else => error.WrongAnswer }};', '    }']
    for function, child, payload in aggregate_answers(definitions):
        projected, member = aggregate_projection(function, payload, definitions)
        if member is None: continue
        typ = 'u64' if function == 'rank' else 'Value'
        expr = 'switch (receipt.' + member + ') { .value => |v| v, .null_value => ' + ('.null_value' if function == 'filter' else 'return error.WrongAnswer') + ', .missing => return error.WrongAnswer }'
        receipt_type = name(payload['items']['$ref'].split('/')[-1])
        row = ' .filter_row => |v| &.{v.value},' if function == 'filter' else ''
        lines += [f'    pub fn {function}(self: @This()) ![]const {typ} {{',
                  f'        const receipts: []const {receipt_type} = switch (self.packet.*) {{{row} .{function}_aggregate => |v| v.value, else => return error.WrongAnswer }};',
                  f'        const answer = try self.allocator.alloc({typ}, receipts.len);',
                  f'        for (answer, receipts) |*out, receipt| out.* = {expr};',
                  '        return answer;', '    }']
    return '\n'.join(lines + ['};', ''])


def call_helpers(definitions):
    """Private typed receipt projections, independent of the named call API."""
    from host_answers import call_results
    from c import variant_member, nullable
    rules = call_results(definitions)
    lines = ['\npub fn callReceiptType(comptime function: anytype) type {']
    for function, rule in rules.items():
        payload = rule['receipt_payload']
        receipt = payload.get('items', payload)['$ref'].split('/')[-1]
        lines.append(f'    if (comptime std.mem.eql(u8, @tagName(function), "{function}")) return {name(receipt)};')
    lines += ['    @compileError("unknown named function");', '}',
              'pub fn callReceipts(comptime function: anytype, packet: *const SessionPacket, a: std.mem.Allocator) !?[]const callReceiptType(function) {']
    for function, rule in rules.items():
        carrier = variant_member('completesessionPacket', rule['carrier'])
        lines += [f'    if (comptime std.mem.eql(u8, @tagName(function), "{function}")) {{',
                  f'        if (packet.* != .{carrier}) return null;']
        if rule['receipt_payload'].get('type') == 'array':
            lines += [f'        return packet.{carrier}.value;']
        else:
            lines += ['        const result = try a.alloc(callReceiptType(function), 1);',
                      f'        result[0] = packet.{carrier}.value;', '        return result;']
        lines += ['    }']
    lines += ['    unreachable;', '}',
              'pub fn callValueType(comptime function: anytype) type {']
    for function, rule in rules.items():
        payload = rule['receipt_payload']
        receipt = definitions[payload.get('items', payload)['$ref'].split('/')[-1]]
        field = rule['value_projection']
        schema = receipt['properties'][field]
        presence = field not in receipt.get('required', []) or nullable(schema, definitions)
        raw = f'@TypeOf(@field(@as(callReceiptType(function), undefined), "{field}"))'
        typ = f'@typeInfo(@TypeOf(@as({raw}, undefined).optional())).optional.child' if presence else raw
        if presence and function not in ('filter', 'rank'):
            typ = '?' + typ
        lines += [f'    if (comptime std.mem.eql(u8, @tagName(function), "{function}")) return {typ};']
    lines += ['    @compileError("unknown named function");', '}',
              'pub fn callValue(comptime function: anytype, receipt: callReceiptType(function)) !callValueType(function) {']
    for function, rule in rules.items():
        payload = rule['receipt_payload']
        receipt = definitions[payload.get('items', payload)['$ref'].split('/')[-1]]
        field = rule['value_projection']
        presence = field not in receipt.get('required', []) or nullable(receipt['properties'][field], definitions)
        value = f'receipt.{field}'
        if presence:
            if function in ('filter', 'rank'):
                null = '.null_value' if function == 'filter' else 'return error.MissingAnswer'
                value = f'switch ({value}) {{ .value => |v| v, .null_value => {null}, .missing => return error.MissingAnswer }}'
            else:
                value += '.optional()'
        lines += [f'    if (comptime std.mem.eql(u8, @tagName(function), "{function}")) return {value};']
    lines += ['    unreachable;', '}',
              'pub fn callProbabilityType(comptime function: anytype) type {']
    for function, rule in rules.items():
        typ = '[]const ?f64' if function in ('recognize', 'relate') else '?f64'
        lines += [f'    if (comptime std.mem.eql(u8, @tagName(function), "{function}")) return {typ};']
    lines += ['    @compileError("unknown named function");', '}',
              'pub fn callProbability(comptime function: anytype, receipt: callReceiptType(function), ordinal: usize, packets: []const *const SessionPacket, a: std.mem.Allocator) !callProbabilityType(function) {']
    for function, rule in rules.items():
        lines += [f'    if (comptime std.mem.eql(u8, @tagName(function), "{function}")) {{']
        mode = rule['probability_mode']
        if 'resolution' in rule:
            tag = rule['scalar_kind']
            lines += ['        const index = receipt.index.optional() orelse ordinal;',
                      '        var resolved = false;',
                      '        for (packets) |packet| {',
                      '            if (packet.* != .observation or packet.observation.value != .row) continue;',
                      '            const row = packet.observation.value.row;',
                      '            if (row.index != index or row.value != .judgment) continue;',
                      f'            if (row.value.judgment.value != .{tag}) continue;',
                      f'            resolved = row.value.judgment.value.{tag}.value == .value;',
                      '        }', '        if (!resolved) return null;']
        if 'selection_path' in rule:
            lines += ['        if (receipt.index != .value) return null;']
        if mode == 'null':
            lines += ['        return null;']
        elif mode in ('yes', 'picked') or function == 'rank':
            answer = 'receipt.answer'
            schema = definitions[definitions[rule['receipt_payload'].get('items', rule['receipt_payload'])['$ref'].split('/')[-1]]['properties']['answer']['$ref'].split('/')[-1]]
            if 'variants' in schema:
                tag = 'yes_no' if mode == 'yes' or function == 'rank' else 'choice'
                lines += [f'        if (receipt.answer != .{tag}) return ' + ('null;' if function == 'rank' else 'error.WrongAnswer;')]
                answer += '.' + tag
            if mode == 'picked':
                lines += [f'        for ({answer}.probabilities.entries) |entry| {{',
                          f'            if (std.mem.eql(u8, entry.key, {answer}.pick)) return entry.value;',
                          '        }', '        return null;']
            else:
                lines += [f'        return {answer}.probability;']
        elif function == 'relate':
            lines += ['        const result = try a.alloc(?f64, receipt.value.len);',
                      '        for (result, receipt.value) |*out, edge| out.* = edge.probability;', '        return result;']
        elif function == 'recognize':
            lines += RECOGNITION_CHANCES.strip().splitlines()
        lines += ['    }']
    lines += ['    unreachable;', '}',
              'pub fn callRowReceipts(comptime function: anytype, packet: *const SessionPacket, a: std.mem.Allocator) !?[]const callReceiptType(function) {']
    for child, _ in definitions['completesessionPacket']['variants']:
        fields = definitions[child].get('properties', {})
        function = fields.get('function', {}).get('const')
        if fields.get('kind', {}).get('const') != 'row' or function not in rules:
            continue
        carrier = variant_member('completesessionPacket', child)
        lines += [f'    if (comptime std.mem.eql(u8, @tagName(function), "{function}")) {{',
                  f'        if (packet.* != .{carrier}) return null;',
                  '        const result = try a.alloc(callReceiptType(function), 1);',
                  f'        result[0] = packet.{carrier}.value;', '        return result;', '    }']
    return '\n'.join(lines + ['    return null;', '}', ''])


RECOGNITION_CHANCES = '''
        switch (receipt.value) {
            .mode_proposals => |value| {
                const result = try a.alloc(?f64, value.proposals.len);
                for (result, value.proposals) |*out, proposal| out.* = proposal.probability;
                return result;
            },
            .entities => |value| {
                if (receipt.answer != .recognize_answer) return error.WrongAnswer;
                const result = try a.alloc(?f64, value.entities.len);
                for (result, value.entities) |*out, entity| {
                    out.* = null;
                    for (receipt.answer.recognize_answer.proposals) |proposal| {
                        if (!proposal.kept) continue;
                        const selected = proposal.selected.optional() orelse continue;
                        const kind = proposal.kind.optional() orelse continue;
                        if (selected.start == entity.start and selected.end == entity.end and std.mem.eql(u8, kind, entity.kind)) {
                            out.* = proposal.span_probability;
                            break;
                        }
                    }
                }
                return result;
            },
        }
'''


def public_call_types(definitions):
    from host_answers import call_results
    lines = ['\npub fn callList(comptime function: anytype, comptime Input: type) bool {']
    for function, rule in call_results(definitions).items():
        expression = 'true' if rule['call_shape'] == 'collection' else 'false' if rule['call_shape'] == 'aggregate' else '@import("calls_generated.zig").listInput(Input)'
        lines += [f'    if (comptime std.mem.eql(u8, @tagName(function), "{function}")) return {expression};']
    lines += ['    @compileError("unknown named function");', '}',
              'pub fn NamedCallResult(comptime function: anytype, comptime Input: type) type {',
              '    const V = callPublicValueType(function);', '    const P = callProbabilityType(function);',
              '    return @import("blocking.zig").CallResult(if (callList(function, Input)) []const V else V, if (callList(function, Input)) []const P else P, callReceiptType(function));', '}']
    return '\n'.join(lines) + '\n'


def public_value_helpers(definitions):
    """Use normalized native rows where they remove endpoint and mode wrappers."""
    from host_answers import call_results, named_object_fields
    from c import variant_member
    def resolve(schema):
        return definitions[schema['$ref'].split('/')[-1]] if isinstance(schema, dict) and '$ref' in schema else schema if isinstance(schema, dict) else {}
    normalized = {}
    for function, rule in call_results(definitions).items():
        if rule['value_projection'] != 'value':
            continue
        receipt = resolve(rule['receipt_payload'].get('items', rule['receipt_payload']))
        source = resolve(receipt['properties']['value'])
        for child, _ in definitions['completesessionObservedRow']['variants']:
            payload = definitions[child]['properties']['value']
            target = resolve(payload)
            if source.get('type') == target.get('type') == 'array':
                original, projected = resolve(source['items']), resolve(target['items'])
                matches = bool(named_object_fields(original, definitions)) and original.get('properties', {}).keys() == projected.get('properties', {}).keys()
            elif 'variants' in source and 'properties' in target:
                fields = {key: value for variant, _ in source['variants'] for key, value in definitions[variant]['properties'].items() if key != 'mode'}
                matches = bool(fields) and fields.keys() <= target['properties'].keys() and sum(target['properties'][key] == value for key, value in fields.items()) >= 2
            else:
                matches = False
            if matches:
                typ = '[]const ' + name(target['items']['$ref'].split('/')[-1]) if target.get('type') == 'array' else name(payload['$ref'].split('/')[-1])
                normalized[function] = (variant_member('completesessionObservedRow', child), typ)
                break
    lines = ['\npub fn callPublicValueType(comptime function: anytype) type {']
    for function, (_, typ) in normalized.items():
        lines += [f'    if (comptime std.mem.eql(u8, @tagName(function), "{function}")) return {typ};']
    lines += ['    return callValueType(function);', '}',
              'pub fn callPublicValue(comptime function: anytype, receipt: callReceiptType(function), ordinal: usize, packets: []const *const SessionPacket) !callPublicValueType(function) {']
    for function, (tag, _) in normalized.items():
        lines += [f'    if (comptime std.mem.eql(u8, @tagName(function), "{function}")) {{',
                  '        const index = receipt.index.optional() orelse ordinal;',
                  '        for (packets) |packet| {',
                  '            if (packet.* != .observation or packet.observation.value != .row) continue;',
                  '            const row = packet.observation.value.row;',
                  f'            if (row.index == index and row.value == .{tag}) return row.value.{tag}.value;',
                  '        }', '        return error.MissingAnswer;', '    }']
    lines += ['    return callValue(function, receipt);', '}', '']
    return '\n'.join(lines)
