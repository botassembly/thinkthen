"""Swift Codable values derived from the shared Rust schema graph."""
import json
import re

ROOTS = ('completesessionPacket', 'completeplan', 'completeReadableQuestion')
INPUT_ROOTS = ('Request', 'RequestSessionDescriptor', 'RequestReaderFailure', 'EngineSettings')


def name(key):
    return ''.join(part[:1].upper() + part[1:] for part in key.removeprefix('complete').split('_'))


def member(key):
    return '`' + re.sub(r'_([a-z])', lambda m: m[1].upper(), key) + '`'


def render(definitions, inputs=False):
    prefix = 'Input' if inputs else 'Owned'
    nodes = dict(definitions)
    def named(key):
        return prefix + name(key)
    def typ(source, path):
        if not isinstance(source, dict) or not source:
            return 'JSONValue'
        if '$ref' in source:
            return named(source['$ref'].removeprefix('#/$defs/'))
        alts = source.get('anyOf', source.get('oneOf', []))
        if len(alts) == 1:
            return typ(alts[0], path)
        if len(alts) == 2 and any(alt.get('type') == 'null' for alt in alts):
            return typ(next(alt for alt in alts if alt.get('type') != 'null'), path) + '?'
        if alts or 'properties' in source or isinstance(source.get('type'), list):
            nodes.setdefault(path, source)
            return named(path)
        kind = source.get('type')
        if kind is None and 'const' in source:
            kind = 'string' if isinstance(source['const'], str) else 'boolean' if isinstance(source['const'], bool) else 'integer'
        if kind == 'array':
            return '[' + typ(source.get('items', {}), path + '_Item') + ']'
        if kind == 'object':
            return '[String: ' + typ(source.get('additionalProperties', {}), path + '_Entry') + ']'
        return {'string': 'String', 'boolean': 'Bool', 'number': 'Double', 'integer': 'UInt64' if source.get('format', '').startswith('uint') else 'Int64', 'null': 'JSONValue'}.get(kind, 'JSONValue')
    def case(tag, index):
        mode, field, value = tag
        token = value if mode == 'literal' else '_'.join(value.values()) if mode == 'literals' else field if mode == 'member' else 'alternative' + str(index)
        token = name(token)
        return '`' + token[:1].lower() + token[1:] + '`'
    def cond(tag):
        mode, field, value = tag
        if mode == 'literal':
            return 'object[' + json.dumps(field) + '] == .string(' + json.dumps(value) + ')'
        if mode == 'literals':
            return ' && '.join('object[' + json.dumps(k) + '] == .string(' + json.dumps(v) + ')' for k, v in value.items())
        if mode == 'member':
            return 'object[' + json.dumps(field) + '] != nil && object[' + json.dumps(field) + '] != .null'
        return ' && '.join(['object[' + json.dumps(k) + '] != nil' for k in value['required']] + ['object[' + json.dumps(k) + '] == nil' for k in value['excluded']])
    lines = ['// Generated from the shared Rust graph. Do not edit.', 'import Foundation']
    pending = set()
    while nodes.keys() - pending:
        key = sorted(nodes.keys() - pending)[0]
        pending.add(key)
        source, cls = nodes[key], named(key)
        if 'properties' in source:
            fields = source['properties']
            required = set(source.get('required', []))
            lines += [f'public struct {cls}: JSONRepresentable {{', '    public let extensions: [String: JSONValue]', '    private var _sourceJSON: JSONValue? = nil']
            args, assignments, reads, writes = ['extensions: [String: JSONValue] = [:]'], ['self.extensions = extensions'], [], []
            for prop, field in fields.items():
                pname, ptype = member(prop), typ(field, key + '_' + prop)
                mandatory = prop in required
                constant = field.get('const') if isinstance(field, dict) and 'const' in field else None
                default = (' = ' + json.dumps(constant)) if mandatory and constant is not None else '' if mandatory else ' = .absent'
                stored = ptype if mandatory else 'Presence<' + ptype + '>'
                lines += [f'    public let {pname}: {stored}']
                args += [f'{pname}: {stored}{default}']
                assignments += [f'self.{pname} = {pname}']
                expression = f'try {ptype}.read(jsonRequired(object, {json.dumps(prop)}))' if mandatory else f'try readPresence(object, {json.dumps(prop)}, {ptype}.self)'
                reads += [f'{pname.strip(chr(96))}: {expression}']
                writes += [f'object[{json.dumps(prop)}] = {pname}.json' if mandatory else f'writePresence({pname}, {json.dumps(prop)}, &object)']
            keys = ', '.join(json.dumps(prop) for prop in fields)
            lines += ['    public init(' + ', '.join(args) + ') { ' + '; '.join(assignments) + ' }', f'    public static func read(_ json: JSONValue) throws -> {cls} {{', '        let object = try jsonObject(json)', f'        let known: Set<String> = [{keys}]', '        var value = Self(extensions: object.filter { !known.contains($0.key) }, ' + ', '.join(reads) + ')', '        value._sourceJSON = json', '        return value', '    }', '    public var json: JSONValue {', '        if let _sourceJSON { return _sourceJSON }', f'        let known: Set<String> = [{keys}]', '        var object = extensions.filter { !known.contains($0.key) }', *('        ' + line for line in writes), '        return .object(object)', '    }', '}']
        elif 'variants' in source:
            variants = source['variants']
            lines += [f'public indirect enum {cls}: JSONRepresentable {{']
            lines += [f'    case {case(tag, i)}({named(child)})' for i, (child, tag) in enumerate(variants)]
            lines += [f'    public static func read(_ json: JSONValue) throws -> {cls} {{', '        let object = try jsonObject(json)']
            for i, (child, tag) in enumerate(variants):
                lines += [f'        if {cond(tag)} {{ return .{case(tag, i)}(try {named(child)}.read(json)) }}']
            lines += [f'        throw JSONConversionError("Unknown {cls} alternative")', '    }', '    public var json: JSONValue {', '        switch self {']
            lines += [f'        case .{case(tag, i)}(let value): return value.json' for i, (_, tag) in enumerate(variants)]
            lines += ['        }', '    }', '}']
        elif 'primitive_variants' in source or source.get('anyOf', source.get('oneOf')) or isinstance(source.get('type'), list):
            alts = list(source.get('primitive_schemas', {}).values()) or source.get('anyOf', source.get('oneOf', [])) or [{**source, 'type': kind} for kind in source['type']]
            lines += [f'public indirect enum {cls}: JSONRepresentable {{']
            for i, alt in enumerate(alts):
                lines += [f'    case alternative{i}({typ(alt, key + "_Alternative" + str(i))})']
            lines += [f'    public static func read(_ json: JSONValue) throws -> {cls} {{']
            for i, alt in enumerate(alts):
                if alt.get('type') == 'null':
                    lines += [f'        if json == .null {{ return .alternative{i}(.null) }}']
                else:
                    lines += [f'        if let value = try? {typ(alt, key + "_Alternative" + str(i))}.read(json) {{ return .alternative{i}(value) }}']
            lines += [f'        throw JSONConversionError("Unknown {cls} value")', '    }', '    public var json: JSONValue {', '        switch self {']
            lines += [f'        case .alternative{i}(let value): return value.json' for i in range(len(alts))]
            lines += ['        }', '    }', '}']
        else:
            lines += [f'public typealias {cls} = {typ(source, key + "_Value")}']
    return '\n'.join(lines) + '\n'
