"""Swift values derived from the shared Rust schema graph."""
import json
import re
from host_answers import aggregate_answers, named_object_fields, usage_operations, alternative_name, aggregate_projection, call_results, scalar_input_selectors

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
        lines += [answer_helpers(definitions)]
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


def result_types(info, single):
    def typ(source):
        if not isinstance(source, dict): return 'DataValue'
        if '$ref' in source: return name(source['$ref'].split('/')[-1])
        kind = source.get('type')
        if isinstance(kind, list): return typ({**source, 'type': next(k for k in kind if k != 'null')}) + '?'
        if kind == 'array': return '[' + typ(source['items']) + ']'
        return {'string': 'String', 'boolean': 'Bool', 'number': 'Double', 'integer': 'UInt64'}.get(kind, 'DataValue')
    value = info['aggregate_value']
    if info['call_shape'] == 'input': value = value['items']
    target = 'DataValue?' if 'resolution' in info and info['probability_mode'] == 'yes' else typ(value)
    if 'selection_path' in info: target = 'DataValue?'
    mode = info['probability_mode']
    probability = '[Double?]' if mode == 'items' else 'Double?'
    if mode == 'items' and info['call_shape'] != 'input': probability = '[Double?]'
    if info['call_shape'] == 'collection': probability = '[Double?]'
    if info['call_shape'] == 'input' and not single:
        target = '[' + target + ']'
        probability = '[' + probability + ']' if mode != 'null' else 'Double?'
    return target, probability, typ(info['receipt_payload'])


def named_calls(definitions):
    from generate import SCHEMA, graph, prepare
    results = call_results(prepare(graph(json.loads(SCHEMA.read_text()), ROOTS)))
    single_inputs = set(scalar_input_selectors(definitions))
    selectors = [(child, definitions[child]['properties']['kind']['const'])
                 for child, _ in definitions['RequestInput']['variants']]
    lines = ['extension Client {']
    version = literal_member(definitions['RequestVersion']['oneOf'][0]['const'])
    for function, info in results.items():
        inputs = selectors if info['call_shape'] == 'input' else [(None, None)]
        for child, kind in inputs:
            single = child in single_inputs
            target, probability, receipts = result_types(info, single)
            argument = name(child) if child else 'Input'
            expression = '.' + literal_member(kind) + '(input)' if child else 'input'
            suffix = 'Scalar' if single else 'List'
            if info['call_shape'] != 'input': suffix = ''
            signature = f'    public func {function}(_ question: Question, input: {argument}, options: Presence<Options> = .absent'
            feed = ', feed: AsyncThrowingStream<SessionDescriptor, any Error>' if kind == 'feed' else ''
            passed_feed = ', feed: feed' if kind == 'feed' else ''
            lines += [signature + feed + f') async throws -> CallResult<{target}, {probability}, {receipts}> {{',
                      f'        let call = try await execute(Request(call: CallRequest(function: .{literal_member(function)}, input: {expression}, options: options, question: question), schema: .{version}){passed_feed})',
                      f'        return try call.{function}Result{suffix}()', '    }']
            if single:
                payload = 'String' if kind == 'text' else 'DataValue'
                member_name = 'text' if kind == 'text' else 'value'
                lines += [f'    public func {function}(_ question: Question, input: {payload}, options: Presence<Options> = .absent) async throws -> CallResult<{target}, {probability}, {receipts}> {{',
                          f'        try await {function}(question, input: {argument}({member_name}: input), options: options)', '    }']
    return '\n'.join(lines + ['}'])


def answer_helpers(definitions):
    lines = ['extension Call {']
    def path(values): return '[' + ', '.join(json.dumps(v) for v in values) + ']'
    for function, info in call_results(definitions).items():
        variants = [True, False] if info['call_shape'] == 'input' else [False]
        for single in variants:
            target, probability, receipts = result_types(info, single)
            suffix = ('Scalar' if single else 'List') if info['call_shape'] == 'input' else ''
            resolution = info.get('resolution', {})
            guards = '[' + ', '.join('(' + path(p) + ', ' + json.dumps(v) + ')' for p, v in resolution.get('guards', [])) + ']'
            collection = str(info['receipt_payload'].get('type') == 'array').lower()
            lines += [f'    func {function}Result{suffix}() throws -> CallResult<{target}, {probability}, {receipts}> {{',
                      f'        let projection = try project({json.dumps(function)}, collection: {collection}, resolution: {path(resolution.get("path", []))}, guards: {guards}, indexPath: {path(resolution.get("index_path", []))})']
            source = '.array(projection.receipts)' if collection == 'true' else 'projection.receipts[0]'
            if single or collection == 'false': lines += ['        guard projection.receipts.count == 1 else { throw ValueConversionError("Expected one result") }']
            lines += [f'        let results = try {receipts}.read({source})']
            elem = target[1:-1] if target.startswith('[') and (info['call_shape'] == 'collection' or info['call_shape'] == 'input' and not single) else target
            projected = info['value_projection']
            read = f'projection.value(index, projection: {json.dumps(projected)}, resolved: {str(bool(resolution)).lower()})'
            if elem != 'DataValue?': read = f'{elem}.read({read} ?? .null)'
            lines += [f'        let values = try projection.receipts.indices.map {{ index in try {read} }}']
            if 'selection_path' in info:
                lines += [f'        let value: {target} = try callPath(projection.receipts[0], {path(info["selection_path"])}) == .null ? nil : values[0]']
            elif info['call_shape'] == 'input':
                lines += ['        let value = values[0]' if single else '        let value = values']
            elif info['call_shape'] == 'collection': lines += ['        let value = values']
            else: lines += ['        let value = values[0]']
            mode = info['probability_mode']
            if mode == 'null': lines += ['        let probability: Double? = nil']
            elif mode in ('yes', 'picked'):
                settled = 'values[index] != nil' if resolution else 'value != nil' if 'selection_path' in info else 'true'
                pick = path(info['pick_path']) if 'pick_path' in info else 'nil'
                lines += [f'        let probabilities = try projection.receipts.indices.map {{ index in try projection.probability(index, path: {path(info["probability_path"])}, pick: {pick}, settled: {settled}) }}',
                          '        let probability = probabilities' if probability.startswith('[') else '        let probability = probabilities[0]']
            elif info['call_shape'] == 'collection':
                lines += ['        let probability: [Double?] = results.map { if case .yesNo(let answer) = $0.answer { return answer.probability }; return nil }']
            elif info['call_shape'] == 'input':
                lines += ['        let probabilities = try results.map(recognitionProbabilities)', '        let probability = probabilities[0]' if single else '        let probability = probabilities']
            else: lines += ['        let probability: [Double?] = value.map { $0.probability }']
            lines += ['        guard let facts = terminal.facts.value else { throw ValueConversionError("Missing result facts") }',
                      '        return CallResult(value: value, probability: probability, results: results, facts: facts, call: self)', '    }']
    return '\n'.join(lines + ['}'])
