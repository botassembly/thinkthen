"""Swift values derived from the shared Rust schema graph."""
import json
import re
from host_answers import aggregate_answers, named_object_fields, usage_operations, alternative_name, aggregate_projection

ROOTS = ('completesessionPacket', 'completeplan', 'completequestionDocument')
INPUT_ROOTS = ('Request', 'RequestSessionDescriptor', 'RequestReaderFailure', 'EngineSettings')


def name(key):
    return ''.join(part[:1].upper() + part[1:] for part in key.removeprefix('complete').removeprefix('Request' if key not in ('Request',) else '').split('_'))


def member(key):
    return '`' + re.sub(r'_([a-z])', lambda m: m[1].upper(), key) + '`'


def literal_member(value):
    token = re.sub(r'[^A-Za-z0-9_]', '_', value)
    if token[:1].isdigit():
        token = 'value_' + token
    return member(token[:1].lower() + token[1:])


def arm_name(source):
    if isinstance(source.get('type'), list):
        return 'Or'.join(alternative_name({'type': kind}) for kind in source['type'])
    return alternative_name(source)


def render(definitions, inputs=False):
    if not inputs:
        from generate import ROOT, SCHEMA, graph, immediate_schema, prepare
        definitions = {**prepare(graph(immediate_schema(json.loads(SCHEMA.read_text())), ('completeImmediateFacts',))), **definitions}
    from generate import ROOT, graph, prepare
    request_schema = json.loads((ROOT / 'specification/request.schema.json').read_text())
    request_schema['$defs']['Request'] = {key: value for key, value in request_schema.items() if key != '$defs'}
    input_names = {name(key) for key in prepare(graph(request_schema, INPUT_ROOTS))}
    aliases = {key: key.removeprefix('RequestDefinition_') for key in definitions if key.startswith('RequestDefinition_') and definitions[key] == definitions.get(key.removeprefix('RequestDefinition_'))}
    nodes = {key: source for key, source in definitions.items() if key not in aliases}
    def named(key):
        key = aliases.get(key, key)
        if key == 'RequestCall': return 'CallRequest'
        if key == 'completeError': return 'FailureDetail'
        result = name(key)
        return 'Receipt' + result if not inputs and result in input_names else result
    def typ(source, path):
        if not isinstance(source, dict) or not source:
            return 'DataValue'
        if '$ref' in source:
            return named(source['$ref'].removeprefix('#/$defs/'))
        kinds = source.get('type')
        if isinstance(kinds, list) and len(kinds) == 2 and 'null' in kinds:
            return typ({**source, 'type': next(kind for kind in kinds if kind != 'null')}, path) + '?'
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
        if kind == 'object' and source.get('x-preserve-order'):
            return 'OrderedMap<' + typ(source.get('additionalProperties', {}), path + '_Entry') + '>'
        if kind == 'object':
            return '[String: ' + typ(source.get('additionalProperties', {}), path + '_Entry') + ']'
        return {'string': 'String', 'boolean': 'Bool', 'number': 'Double', 'integer': 'UInt64' if source.get('format', '').startswith('uint') else 'Int64', 'null': 'DataValue'}.get(kind, 'DataValue')
    def case(tag, index):
        mode, field, value = tag
        token = value if mode == 'literal' else '_'.join(value.values()) if mode == 'literals' else field if mode == 'member' else '_'.join(field for field in value['required'] if field != 'version')
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
    if inputs:
        lines += [ordered_maps()]
    pending = set()
    while nodes.keys() - pending:
        key = sorted(nodes.keys() - pending)[0]
        pending.add(key)
        source, cls = nodes[key], named(key)
        literals = source.get('enum', []) or [alt.get('const') for alt in source.get('oneOf', source.get('anyOf', []))]
        if literals and all(isinstance(value, str) for value in literals):
            lines += [f'public enum {cls}: ValueRepresentable {{']
            lines += [f'    case {literal_member(value)}' for value in literals]
            lines += ([] if inputs else ['    case unknown(String)'])
            lines += [f'    static func read(_ json: DataValue) throws -> {cls} {{', '        let value = try String.read(json)', '        switch value {']
            lines += [f'        case {json.dumps(value)}: return .{literal_member(value)}' for value in literals]
            lines += [('        default: throw ValueConversionError("Unknown ' + cls + ' value")') if inputs else '        default: return .unknown(value)', '        }', '    }', '    var json: DataValue {', '        switch self {']
            lines += [f'        case .{literal_member(value)}: return .string({json.dumps(value)})' for value in literals]
            lines += ([] if inputs else ['        case .unknown(let value): return .string(value)']) + ['        }', '    }', '}']
        elif 'properties' in source:
            fields = source['properties']
            required = set(source.get('required', []))
            lines += [f'public struct {cls}: ValueRepresentable {{', '    public let extensions: [String: DataValue]', '    private var _sourceJSON: DataValue? = nil']
            args, assignments, reads, writes = ['extensions: [String: DataValue] = [:]'], ['self.extensions = extensions'], [], []
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
            lines += ['    public init(' + ', '.join(args) + ') { ' + '; '.join(assignments) + ' }', f'    static func read(_ json: DataValue) throws -> {cls} {{', '        let object = try jsonObject(json)', f'        let known: Set<String> = [{keys}]', '        var value = Self(extensions: object.filter { !known.contains($0.key) }, ' + ', '.join(reads) + ')', '        value._sourceJSON = json', '        return value', '    }', '    var json: DataValue {', '        if let _sourceJSON { return _sourceJSON }', f'        let known: Set<String> = [{keys}]', '        var object = extensions.filter { !known.contains($0.key) }', *('        ' + line for line in writes), '        return .object(object)', '    }', '}']
            if not inputs:
                lines.pop()
                for prop in named_object_fields(source, definitions):
                    accessor = member(prop).strip('`') + 'Name'
                    lines += [f'    public func {accessor}() throws -> Presence<String> {{', f'        switch try readPresence(jsonObject(self.json), {json.dumps(prop)}, DataValue.self) {{', '        case .absent: return .absent', '        case .null: return .null', '        case .value(let object): return try readPresence(jsonObject(object), "name", String.self)', '        }', '    }']
                lines += ['}']
        elif 'variants' in source:
            variants = source['variants']
            lines += [f'public indirect enum {cls}: ValueRepresentable {{']
            lines += [f'    case {case(tag, i)}({named(child)})' for i, (child, tag) in enumerate(variants)]
            lines += [f'    static func read(_ json: DataValue) throws -> {cls} {{', '        let object = try jsonObject(json)']
            for i, (child, tag) in enumerate(variants):
                lines += [f'        if {cond(tag)} {{ return .{case(tag, i)}(try {named(child)}.read(json)) }}']
            lines += [f'        throw ValueConversionError("Unknown {cls} alternative")', '    }', '    var json: DataValue {', '        switch self {']
            lines += [f'        case .{case(tag, i)}(let value): return value.json' for i, (_, tag) in enumerate(variants)]
            lines += ['        }', '    }', '}']
        elif 'primitive_variants' in source or source.get('anyOf', source.get('oneOf')) or isinstance(source.get('type'), list):
            alts = list(source.get('primitive_schemas', {}).values()) or source.get('anyOf', source.get('oneOf', [])) or [{**source, 'type': kind} for kind in source['type']]
            lines += [f'public indirect enum {cls}: ValueRepresentable {{']
            arms = [literal_member(arm_name(alt)) for alt in alts]
            for i, alt in enumerate(alts):
                lines += [f'    case {arms[i]}' if alt.get('type') == 'null' else f'    case {arms[i]}({typ(alt, key + "_" + arm_name(alt))})']
            lines += [f'    static func read(_ json: DataValue) throws -> {cls} {{']
            for i, alt in enumerate(alts):
                if alt.get('type') == 'null':
                    lines += [f'        if json == .null {{ return .{arms[i]} }}']
                else:
                    lines += [f'        if let value = try? {typ(alt, key + "_" + arm_name(alt))}.read(json) {{ return .{arms[i]}(value) }}']
            lines += [f'        throw ValueConversionError("Unknown {cls} value")', '    }', '    var json: DataValue {', '        switch self {']
            lines += [f'        case .{arms[i]}: return .null' if alt.get('type') == 'null' else f'        case .{arms[i]}(let value): return value.json' for i, alt in enumerate(alts)]
            lines += ['        }', '    }', '}']
        else:
            lines += [f'public typealias {cls} = {typ(source, key + "_Value")}']
    if inputs:
        lines += [named_calls(definitions)]
    else:
        lines += ['extension SessionPacket {']
        variants = definitions['completesessionPacket']['variants']
        for function, child, payload in aggregate_answers(definitions):
            index = next(i for i, (candidate, _) in enumerate(variants) if candidate == child)
            tag = variants[index][1]
            lines += [f'    public func {function}Answer() throws -> {typ(payload, child + "_value")} {{', f'        if case .{case(tag, index)}(let packet) = self {{ return packet.value }}', f'        throw ValueConversionError("Expected {function} aggregate packet")', '    }']
        lines += ['}', answer_helpers(definitions)]
        from swift_native import render as native_graph
        lines += [native_graph(definitions)]
    return ('\n'.join(lines) + '\n')


def bridge(abi):
    lines = ['// Generated from the compiler-derived C header. Do not edit.', 'import CThinkThen']
    for cls, prefix, suffix in [('QuestionGrammar', 'THINKTHEN_LOAD_', '_V1'), ('UsagePersistenceState', 'THINKTHEN_COMPLETE_USAGE_PERSISTENCE_', '_V1')]:
        lines += [f'{"public " if cls != "QuestionGrammar" else ""}enum {cls}: UInt32, Sendable {{']
        lines += ['    case ' + literal_member(key.removeprefix(prefix).removesuffix(suffix).lower()) + ' = ' + str(value)
                  for key, value in abi['constants'].items() if key.startswith(prefix) and key.endswith(suffix)]
        lines += ['}']
    lines += ['func nativeFailureKind(_ code: Int32) -> FailureKind {', '    switch code {']
    lines += ['    case ' + str(value) + ': return .' + literal_member(key.removeprefix('THINKTHEN_E').lower())
              for key, value in abi['constants'].items() if re.fullmatch(r'THINKTHEN_E[A-Z]+', key)]
    lines += ['    default: return .unknown(String(code))', '    }', '}']
    lines += ['extension Client {']
    for symbol, public in usage_operations(abi['functions']):
        public = member(public).strip('`')
        lines += [f'    public func {public}() throws -> UsagePersistenceStatus {{ try usage({symbol}) }}']
    lines += ['}']
    return '\n'.join(lines) + '\n'


def linux_bridge(functions):
    lines = ['// Generated from compiler-derived C declarations. Do not edit.',
             '#include <dlfcn.h>', '#include <pthread.h>', '#include <stdlib.h>',
             '#include <string.h>', '#pragma GCC visibility push(hidden)',
             '#include "loader.h"', 'struct native_functions {']
    for name, prototype in functions.items():
        arguments = ', '.join(prototype['arguments']) or 'void'
        lines.append(f'    {prototype["return"]} (*{name})({arguments});')
    lines += ['};', 'static struct native_functions native;',
              'static pthread_mutex_t loading = PTHREAD_MUTEX_INITIALIZER;',
              'static int state;', 'static void *native_handle;',
              'int thinkthen_swift_load(const char *path) {',
              '    pthread_mutex_lock(&loading);',
              '    if (!state) {', '        state = -1;',
              '        void *handle = path && path[0] == \'/\' ? dlopen(path, RTLD_NOW | RTLD_LOCAL) : NULL;',
              '        if (handle) {', '            struct native_functions candidate;',
              '            void *symbol;']
    for name in functions:
        lines += [f'            symbol = dlsym(handle, "{name}");',
                  '            if (!symbol) goto failed;',
                  f'            _Static_assert(sizeof(candidate.{name}) == sizeof(symbol), "function pointer width");',
                  f'            memcpy(&candidate.{name}, &symbol, sizeof(symbol));']
    lines += ['            native = candidate;',
              '            native_handle = handle; // Workers may outlive Swift owners; retain until process exit.',
              '            state = 1;', '            goto done;',
              'failed:', '            dlclose(handle);', '        }', '    }',
              'done:;', '    int result = state == 1 ? 0 : THINKTHEN_ELOCAL;',
              '    pthread_mutex_unlock(&loading);', '    return result;', '}']
    for name, prototype in functions.items():
        arguments = ', '.join(f'{kind} arg{i}' for i, kind in enumerate(prototype['arguments'])) or 'void'
        passed = ', '.join(f'arg{i}' for i in range(len(prototype['arguments'])))
        returned = '' if prototype['return'] == 'void' else 'return '
        lines += [f'{prototype["return"]} {name}({arguments}) {{',
                  f'    if (!native.{name}) abort();', f'    {returned}native.{name}({passed});', '}']
    return '\n'.join(lines + ['#pragma GCC visibility pop']) + '\n'


def ordered_maps():
    return """public struct OrderedEntry<Value: Sendable>: Sendable {
    public let key: String
    public let value: Value
    public init(_ key: String, _ value: Value) { self.key = key; self.value = value }
}
public struct OrderedMap<Value: Sendable>: Sendable {
    public let entries: [OrderedEntry<Value>]
    public init(_ entries: [OrderedEntry<Value>]) { self.entries = entries }
}
extension OrderedMap: ValueRepresentable where Value: ValueRepresentable {
    static func read(_ json: DataValue) throws -> Self {
        let members: [ValueMember]
        switch json.kind {
        case .orderedObject(let value): members = value
        case .object(let value): members = value.map { ValueMember($0.key, $0.value) }
        default: throw ValueConversionError("Expected JSON object")
        }
        return Self(try members.map { OrderedEntry($0.key, try Value.read($0.value)) })
    }
    var json: DataValue { .orderedObject(entries.map { ValueMember($0.key, $0.value.json) }) }
}"""


def named_calls(definitions):
    lines = ['extension Client {']
    for item in definitions['RequestFunction'].get('oneOf', []):
        function = item['const']
        version = literal_member(definitions['RequestVersion']['oneOf'][0]['const'])
        lines += [f'    public func {function}(_ question: Question, input: Input, options: Presence<Options> = .absent) async throws -> Call {{',
                  f'        try await execute(Request(call: CallRequest(function: .{literal_member(function)}, input: input, options: options, question: question), schema: .{version}))', '    }']
    return '\n'.join(lines + ['}'])


def answer_helpers(definitions):
    from host_answers import scalar_answers, judgment_route
    route, guards = judgment_route(definitions)
    def path(value): return '[' + ', '.join(json.dumps(field) for field in value) + ']'
    checks = ['                guard try answerPath(packet.json, ' + path(fields) + ') == .string(' + json.dumps(value) + ') else { continue }' for fields, value in guards]
    collections = [function for function, _, payload in aggregate_answers(definitions) if payload.get('type') == 'array']
    collection_test = ' || '.join('function == ' + json.dumps(function) for function in collections)
    lines = ['public struct AnswerCardinalityError: Error, Sendable {',
             '    public let function: String', '    public let count: Int', '    public let call: Call', '}',
             'private func answerPath(_ json: DataValue, _ path: [String]) throws -> DataValue? {',
             '    var value: DataValue? = json', '    for key in path { guard let object = value else { return nil }; value = try jsonObject(object)[key] }', '    return value', '}',
             'private func answerProjection(_ value: DataValue, _ member: String) throws -> DataValue {',
             '    guard case .array(let rows) = value.kind else { throw ValueConversionError("Expected answer collection") }',
             '    return .array(try rows.map { try jsonRequired(jsonObject($0), member) })', '}',
             'extension Call {',
             '    private func answerJSON(_ function: String, _ judgment: String) throws -> DataValue {',
             '        guard terminal.kind == "terminal" else { throw AnswerCardinalityError(function: function, count: 0, call: self) }',
             '        if case .null = terminal.failure { throw ValueConversionError("Terminal failure must be an object") }',
             '        if case .value(let failure) = terminal.failure { throw SessionFailure(failure: failure, call: self) }',
             '        let collection = ' + collection_test, '        var values: [DataValue] = []', '        for packet in packets {',
             '            guard try answerPath(packet.json, ["function"]) == .string(function) else { continue }',
             '            if judgment.isEmpty {',
             '                let kind = try answerPath(packet.json, ["kind"])',
             '                let row = collection && kind == .string("row")',
             '                guard row || kind == .string("aggregate"), let value = try answerPath(packet.json, ["value"]) else { continue }',
             '                values.append(row ? .array([value]) : value)', '            } else {',
             *checks, '                guard let carrier = try answerPath(packet.json, ' + path(route) + ') else { continue }',
             '                guard try answerPath(carrier, ["kind"]) == .string(judgment), let value = try answerPath(carrier, ["value"]) else { continue }',
             '                values.append(value)', '            }', '        }',
             '        if judgment.isEmpty && collection { var combined: [DataValue] = []; for value in values { guard case .array(let items) = value.kind else { throw ValueConversionError("Expected aggregate collection") }; combined.append(contentsOf: items) }; return .array(combined) }',
             '        guard values.count == 1 else { throw AnswerCardinalityError(function: function, count: values.count, call: self) }',
             '        return values[0]', '    }']
    def value_type(source):
        if not isinstance(source, dict): return 'DataValue'
        if '$ref' in source: return name(source['$ref'].split('/')[-1])
        if source.get('type') == 'array': return '[' + value_type(source['items']) + ']'
        return {'string': 'String', 'boolean': 'Bool', 'number': 'Double', 'integer': 'UInt64'}.get(source.get('type'), 'DataValue')
    answers = {function: (value_type(aggregate_projection(function, payload, definitions)[0]), value_type(aggregate_projection(function, payload, definitions)[0]), '') for function, _, payload in aggregate_answers(definitions)}
    projections = {function: aggregate_projection(function, payload, definitions)[1] for function, _, payload in aggregate_answers(definitions)}
    for function, kind, payload in scalar_answers(definitions):
        source = dict(payload); kinds = source.get('type'); nullable = isinstance(kinds, list) and 'null' in kinds
        if isinstance(kinds, list): source['type'] = next(value for value in kinds if value != 'null')
        target = value_type(source)
        answers[function] = (target + '?' if nullable else target, 'Optional<' + target + '>' if nullable else target, kind)
    for function, (target, reader, kind) in answers.items():
        projection = projections.get(function)
        expression = f'answerJSON({json.dumps(function)}, {json.dumps(kind)})'
        if projection:
            expression = f'answerProjection({expression}, {json.dumps(projection)})'
        lines += [f'    public func {function}Answer() throws -> {target} {{ try {reader}.read({expression}) }}']
    lines += ['}', 'extension Client {']
    for function, (target, _, _) in answers.items():
        lines += [f'    public func {function}Answer(_ question: Question, input: Input, options: Presence<Options> = .absent) async throws -> {target} {{',
                  f'        let call = try await {function}(question, input: input, options: options)', f'        return try call.{function}Answer()', '    }']
    return '\n'.join(lines + ['}'])
