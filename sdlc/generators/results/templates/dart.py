"""Dart host types from the shared prepared Rust graphs; no admission rules."""
import json
import re

ROOTS = ('completesessionPacket', 'completeImmediateFacts', 'completeplan')
PREFIX = ''
INPUT_ROOTS = ('RequestFunction', 'RequestQuestion', 'RequestInput', 'RequestOptions', 'RequestSessionDescriptor', 'RequestReaderFailure', 'EngineSettings')


def name(key):
    key = key.removeprefix('complete')
    result_names = {'Atomic_DecideValue': 'DecisionResult', 'Atomic_Nullable_string': 'ChoiceResult', 'Atomic_Array_of_string': 'TagResult', 'Atomic_double': 'ScoreResult', 'Atomic_boolean': 'FilterResult', 'Atomic_NonZeroUsize': 'RankResult', 'ReadableQuestion2': 'FindQuestionInfo', 'ReadableQuestion3': 'RecognitionQuestionInfo', 'ReadableQuestion4': 'RelationQuestionInfo', 'ImmediateFacts': 'FailureFacts'}
    if not PREFIX:
        return result_names.get(key, ''.join(part[:1].upper() + part[1:] for part in key.split('_')))
    if key.startswith('RequestDefinition_Authored_'):
        return key.removeprefix('RequestDefinition_Authored_').title() + 'Question'
    if key == 'RequestDefinition_RecognizeDefinition': return 'RecognizeQuestion'
    if key == 'RequestDefinition_AnnotateDefinition': return 'AnnotateQuestion'
    replacements = {'Authored_options': 'ChoiceOptions', 'Authored_threshold': 'DecisionThreshold', 'Authored_find': 'FindQuestionDefinition', 'Authored_inputDeclaration': 'QuestionInputDeclaration', 'RequestDefinition': 'Definition', 'RequestQuestion_definition': 'DefinedQuestion', 'RequestBatch': 'BatchOptions', 'RequestImage': 'ImageAttachment', 'RecognitionStageContext': 'RecognitionContext', 'RelateFields': 'EntityFields', 'AnnotateQuestion': 'AnnotationSpec', 'RequestSessionDescriptor': 'FeedItem', 'RequestOptions': 'Options', 'Authored_relation': 'RelationDefinition', 'Authored_questionText': 'Wording', 'Authored_criterion': 'Reading', 'RecognizeQuestion': 'RecognitionSpec', 'RelateQuestion': 'RelationSpec'}
    if key in replacements: return replacements[key]
    if key.startswith('Request'): key = key.removeprefix('Request')
    if key.startswith('Authored_'): key = key.removeprefix('Authored_')
    return ''.join(part[:1].upper() + part[1:] for part in key.split('_'))


def arm(source, definitions):
    source = resolved(source, definitions)
    kind = source.get('type')
    if isinstance(kind, list): kind = next((item for item in kind if item != 'null'), 'null')
    if isinstance(kind, str): return {'string': 'text', 'array': 'list', 'object': 'fields'}.get(kind, kind)
    return 'reading'


def member(key):
    value = re.sub(r'_([a-z])', lambda m: m[1].upper(), key)
    return value + 'Value' if value in ('null', 'true', 'false') else value


def resolved(source, definitions):
    if '$ref' in source:
        return definitions[source['$ref'].removeprefix('#/$defs/')]
    return source


def scalar(source, definitions):
    source = resolved(source, definitions)
    if 'properties' in source or 'variants' in source or 'primitive_variants' in source:
        return None
    alternatives = source.get('anyOf', source.get('oneOf', [source]))
    kinds = set()
    for item in alternatives:
        item = resolved(item, definitions)
        kind = item.get('type')
        if kind is None and 'const' in item:
            kind = 'string' if isinstance(item['const'], str) else 'boolean' if isinstance(item['const'], bool) else 'integer' if isinstance(item['const'], int) else 'number'
        if kind is None:
            return None
        kinds.update(kind if isinstance(kind, list) else [kind])
    nullable = 'null' in kinds
    kinds.discard('null')
    if len(kinds) != 1:
        return None
    return next(iter(kinds)), nullable


def union(source, definitions):
    return isinstance(source, dict) and not any(k in source for k in ('properties', 'variants', 'primitive_variants')) and len(source.get('anyOf', source.get('oneOf', []))) > 1 and scalar(source, definitions) is None


def hint(source, definitions):
    if not isinstance(source, dict):
        return 'Object?'
    alternatives = source.get('anyOf', source.get('oneOf', []))
    if len(alternatives) == 1:
        return hint(alternatives[0], definitions)
    if len(alternatives) == 2 and any(item.get('type') == 'null' for item in alternatives):
        inner = next(item for item in alternatives if item.get('type') != 'null')
        return hint(inner, definitions).removesuffix('?') + '?'
    if '$ref' in source:
        key = source['$ref'].removeprefix('#/$defs/')
        target = definitions[key]
        if 'properties' in target or 'variants' in target or 'primitive_variants' in target or union(target, definitions):
            return name(key)
        return hint(target, definitions)
    simple = scalar(source, definitions)
    if simple:
        kind, nullable = simple
        if kind == 'array':
            item = source.get('items', {})
            value = f'List<{hint(item, definitions)}>'
        elif kind == 'object':
            value = f'Map<String, {hint(source.get("additionalProperties", {}), definitions)}>'
        else:
            value = {'string': 'String', 'boolean': 'bool', 'number': 'num', 'integer': 'BigInt'}.get(kind, 'Object')
        return value + ('?' if nullable else '')
    return 'Object?'


def read(source, value, definitions):
    if not isinstance(source, dict):
        return value
    alternatives = source.get('anyOf', source.get('oneOf', []))
    if len(alternatives) == 1:
        return read(alternatives[0], value, definitions)
    if len(alternatives) == 2 and any(item.get('type') == 'null' for item in alternatives):
        inner = next(item for item in alternatives if item.get('type') != 'null')
        return f'{value} == null ? null : {read(inner, value, definitions)}'
    if '$ref' in source:
        key = source['$ref'].removeprefix('#/$defs/')
        if any(item in definitions[key] for item in ('properties', 'variants', 'primitive_variants')) or union(definitions[key], definitions):
            return f'{name(key)}.read({value})'
        return read(definitions[key], value, definitions)
    simple = scalar(source, definitions)
    if not simple:
        return value
    kind, nullable = simple
    if kind == 'array':
        expr = f'({value} as List).map((v) => {read(source.get("items", {}), "v", definitions)}).toList(growable: false)'
    elif kind == 'object':
        expr = f'({value} as Map<String, Object?>).map((k, v) => MapEntry(k, {read(source.get("additionalProperties", {}), "v", definitions)}))'
    elif kind == 'integer':
        expr = f'_readInteger({value})'
    else:
        expr = f'{value} as {hint(source, definitions).removesuffix("?")}'
    return f'{value} == null ? null : {expr}' if nullable else expr


def condition(tag, variable='map'):
    mode, field, value = tag
    if mode == 'literal':
        return f'{variable}[{json.dumps(field)}] == {json.dumps(value)}'
    if mode == 'literals':
        return ' && '.join(f'{variable}[{json.dumps(k)}] == {json.dumps(v)}' for k, v in value.items())
    if mode == 'member':
        return f'{variable}.containsKey({json.dumps(field)})'
    return ' && '.join([f'{variable}.containsKey({json.dumps(k)})' for k in value['required']] + [f'!{variable}.containsKey({json.dumps(k)})' for k in value['excluded']])


def render(definitions, inputs=False):
    global PREFIX
    PREFIX = 'Input' if inputs else ''
    lines = ['// Generated from the shared Rust graph. Do not edit.', 'part of thinkthen_session;']
    parents = {child: name(key) for key, source in definitions.items() for child, _ in source.get('variants', [])}
    for key, source in definitions.items():
        cls = name(key)
        if 'variants' in source:
            lines += [f'sealed class {cls} extends _NativeValue {{', f'  {cls}(super.fields);', *(['  String get name;'] if all(definitions[child].get('properties', {}).get('name', {}).get('type') == 'string' and 'name' in definitions[child].get('required', []) for child, _ in source['variants']) else []), f'  factory {cls}.read(Object? value) {{', '    final map = _readObject(value);']
            for child, tag in source['variants']:
                lines += [f'    if ({condition(tag)}) return {name(child)}.read(map);']
            lines += [f'    throw FormatException("Unknown {cls} alternative");', '  }', '}']
        elif 'primitive_variants' in source:
            lines += [f'final class {cls} implements _EncodedValue {{', '  final Object? value;', f'  const {cls}(this.value);', f'  factory {cls}.read(Object? value) => value is {cls} ? value : {cls}(value);', '  Object? _toValue() => value;']
            for kind, field in source['primitive_schemas'].items():
                if kind != 'null':
                    lines += [f'  {hint(field, definitions)} get as{kind.title()} => {read(field, "value", definitions)};']
            lines += ['}']
        elif union(source, definitions):
            lines += [f'final class {cls} implements _EncodedValue {{', '  final Object? value;', f'  const {cls}._(this.value);', f'  factory {cls}.read(Object? value) => value is {cls} ? value : {cls}._(value);', '  Object? _toValue() => value;']
            for index, field in enumerate(source.get('anyOf', source.get('oneOf', []))):
                if field.get('type') == 'null':
                    lines += [f'  const {cls}.nullValue() : value = null;']
                else:
                    lines += [f'  const {cls}.{arm(field, definitions)}({hint(field, definitions)} this.value);', f'  {hint(field, definitions)} get as{arm(field, definitions).title()} => {read(field, "value", definitions)};']
            lines += ['}']
        elif 'properties' in source:
            parent = parents.get(key, '_NativeValue')
            lines += [f'final class {cls} extends {parent} {{', f'  {cls}._(super.fields);', f'  factory {cls}.read(Object? value) => {cls}._(_readObject(value));']
            fields = source['properties']
            required = set(source.get('required', []))
            if inputs:
                args, values = [], []
                for prop, field in fields.items():
                    var, typ = member(prop), hint(field, definitions)
                    if isinstance(field, dict) and 'const' in field:
                        values += [f'{json.dumps(prop)}: {json.dumps(field["const"])}']
                    elif prop in required:
                        args += [f'required {typ} {var}']
                        values += [f'{json.dumps(prop)}: {var}']
                    else:
                        args += [f'Presence<{typ}> {var} = const Presence.absent()']
                        values += [f'if ({var}.isPresent) {json.dumps(prop)}: {var}.value']
                signature = '{' + ', '.join(args) + '}' if args else ''
                lines += [f'  {cls}({signature}) : super({{{", ".join(values)}}});']
            for prop, field in fields.items():
                typ, var = hint(field, definitions), member(prop)
                expr = read(field, f'_fields[{json.dumps(prop)}]', definitions)
                if prop in required:
                    lines += [f'  {typ} get {var} => {expr};']
                else:
                    lines += [f'  Presence<{typ}> get {var} => _fields.containsKey({json.dumps(prop)}) ? Presence.present({expr}) : const Presence.absent();']
            if not inputs:
                from host_answers import named_object_fields
                for prop in named_object_fields(source, definitions):
                    if prop in required:
                        lines += [f'  String get {member(prop)}Name => {member(prop)}.name;']
            lines += ['  Map<String, Object?> get extensions => Map.unmodifiable({for (final entry in _fields.entries) if (!' + json.dumps(list(fields)) + '.contains(entry.key)) entry.key: entry.value});', '}']
    return '\n'.join(lines) + '\n'


def bridge(abi, definitions):
    def types(value):
        carriers = {'const thinkthen_named_request_v1 *': '_NativeNamedRequest', 'thinkthen_complete_usage_persistence_v1 *': '_NativeUsageState', 'thinkthen_complete_utf8_v1 *': '_NativeUsageAdvice'}
        if value == 'thinkthen_complete_utf8_v1':
            return ('_NativeUsageAdvice',) * 2
        if value in carriers:
            return (f'Pointer<{carriers[value]}>',) * 2
        if value.endswith('**'):
            return ('Pointer<Pointer<Void>>',) * 2
        if value.endswith('*'):
            if value.removeprefix('const ').startswith('char'):
                return ('Pointer<Uint8>',) * 2
            if value.removeprefix('const ').startswith('uint32_t'):
                return ('Pointer<Uint32>',) * 2
            if value.removeprefix('const ').startswith('size_t'):
                return ('Pointer<Size>',) * 2
            return ('Pointer<Void>',) * 2
        return {'uint32_t': ('Uint32', 'int'), 'int': ('Int32', 'int'), 'size_t': ('Size', 'int'), 'void': ('Void', 'void')}[value]
    symbols = ('thinkthen_engine_new_with', 'thinkthen_engine_free', 'thinkthen_error_code',
               'thinkthen_error_view', 'thinkthen_request_plan_named_value',
               'thinkthen_value_view', 'thinkthen_value_free',
               'thinkthen_session_new_named_with_surface', 'thinkthen_session_push',
               'thinkthen_session_next_packet', 'thinkthen_session_finish', 'thinkthen_session_cancel',
               'thinkthen_session_free', 'thinkthen_session_result_free', 'thinkthen_session_result_view',
               'thinkthen_session_error_view', 'thinkthen_engine_usage_persistence_v1',
               'thinkthen_engine_finish_usage_status_v1')
    lines = ['// Generated from the canonical C header ABI. Do not edit.', 'part of thinkthen_session;']
    for carrier, host in [('thinkthen_complete_usage_persistence_v1', '_NativeUsageState'), ('thinkthen_complete_utf8_v1', '_NativeUsageAdvice')]:
        lines += [f'final class {host} extends Struct {{']
        for name, field in abi['records'][carrier]['fields'].items():
            annotation, typ = {'uint32_t': ('@Uint32()', 'int'), 'size_t': ('@Size()', 'int'), 'const char *': ('', 'Pointer<Uint8>')}[field['type']]
            lines += ([f'  {annotation}'] if annotation else []) + [f'  external {typ} {name};']
        lines += ['}']
    lines += ['final class _NativeNamedRequest extends Struct {']
    lines += ['  external _NativeUsageAdvice ' + member + ';' for member in abi['records']['thinkthen_named_request_v1']['fields']]
    lines += ['}']
    states = [(key, value) for key, value in abi['constants'].items() if key.startswith('THINKTHEN_COMPLETE_USAGE_PERSISTENCE_')]
    lines += ['enum UsagePersistenceState { ' + ', '.join(key.removeprefix('THINKTHEN_COMPLETE_USAGE_PERSISTENCE_').removesuffix('_V1').lower() + '(' + str(value) + ')' for key, value in states) + '; final int code; const UsagePersistenceState(this.code); }']
    members = []
    for symbol in symbols:
        fn = abi['functions'][symbol]
        ret = types(fn['return'])
        args = [types(value) for value in fn['arguments']]
        native = f'{ret[0]} Function({", ".join(arg[0] for arg in args)})'
        parameters = ', '.join(f'{arg[1]} arg{i}' for i, arg in enumerate(args))
        lines += [f"@Native<{native}>(symbol: '{symbol}', assetId: 'package:thinkthen_dart/thinkthen')",
                  f'external {ret[1]} _{symbol}({parameters});']
        members += [f'  final {symbol} = _{symbol};']
        if symbol in ('thinkthen_engine_free', 'thinkthen_session_free'):
            members += [f'  final {symbol}Pointer = Native.addressOf<NativeFunction<{native}>>(_{symbol});']
    lines += ['final class _NativeAbi {', *members, *abi_helpers(abi), '}']
    errors = [(key, value) for key, value in abi['constants'].items() if key.startswith('THINKTHEN_E') and not key.endswith('_V1')]
    lines += ['enum ErrorKind { ' + ', '.join(key.removeprefix('THINKTHEN_E').lower() + '(' + str(value) + ')' for key, value in errors) + '; final int code; const ErrorKind(this.code); }']
    constants = ('THINKTHEN_SESSION_ACCEPTED_V1', 'THINKTHEN_SESSION_CLOSED_V1',
                 'THINKTHEN_SESSION_END_V1', 'THINKTHEN_SESSION_FULL_V1',
                 'THINKTHEN_SESSION_PENDING_V1', 'THINKTHEN_SESSION_RESULT_V1',
                 'THINKTHEN_SESSION_WAIT_INPUT_V1', 'THINKTHEN_SESSION_WAIT_SETTLEMENT_V1')
    lines += [f'const {key} = {abi["constants"][key]};' for key in constants]
    return '\n'.join(lines) + '\n' + native_readers(abi, definitions)


def abi_helpers(abi):
    """Generate allocations and counted-field copies from the selected native ABI."""
    from host_answers import usage_operations
    fields = abi['records']['thinkthen_named_request_v1']['fields']
    arguments = {'function': 'function', 'question_json': '_encodeNative(question)',
                 'input_json': '_encodeNative(input)', 'options_json': '_encodeNative(options ?? Options())'}
    if set(fields) != set(arguments): raise ValueError('named request ABI changed')
    lines = [
        '  Pointer<Void> startNamed(Pointer<Void> engine, String function, Question question, Input input, Options? options, String surface, {bool preview = false}) {',
        '    final output = calloc<Pointer<Void>>();',
        '    final request = calloc<_NativeNamedRequest>();',
        '    final bytes = <Pointer<Uint8>>[];',
        '    final views = <Pointer<_NativeUsageAdvice>>[];',
        '    _NativeUsageAdvice counted(String text) {',
        '      final encoded = utf8.encode(text);',
        '      final data = calloc<Uint8>(encoded.length + 1);',
        '      bytes.add(data);',
        '      data.asTypedList(encoded.length).setAll(0, encoded);',
        '      final view = calloc<_NativeUsageAdvice>();',
        '      views.add(view);',
    ]
    for field in abi['records']['thinkthen_complete_utf8_v1']['fields']:
        value = {'data': 'data', 'len': 'encoded.length'}[field]
        lines += [f'      view.ref.{field} = {value};']
    lines += ['      return view.ref;', '    }', '    try {']
    lines += [f'      request.ref.{field} = counted({arguments[field]});' for field in fields]
    lines += [
        '      final code = preview',
        '          ? thinkthen_request_plan_named_value(engine, request, output)',
        '          : thinkthen_session_new_named_with_surface(engine, request, counted(surface), output);',
        '      _checkSession(this, code);',
        '      return output.value;',
        '    } finally {',
        '      for (final value in bytes) calloc.free(value);',
        '      for (final value in views) calloc.free(value);',
        '      calloc.free(request); calloc.free(output);', '    }', '  }',
        '  SessionPacket takePacket(Pointer<Void> packet) {',
        '    final view = calloc<Pointer<Void>>();',
        '    try {',
        '      _checkSession(this, thinkthen_session_result_view(packet, view));',
        '      return SessionPacket.read(_NativePacketReader.readSessionPacket(view.value));',
        '    } finally { thinkthen_session_result_free(packet); calloc.free(view); }', '  }',
    ]
    lines += ['  Plan takePlan(Pointer<Void> value) {',
              '    final view = calloc<Pointer<Void>>();', '    try {',
              '      _checkSession(this, thinkthen_value_view(value, view));',
              '      return Plan.read(_NativePacketReader._value(view.value));',
              '    } finally { thinkthen_value_free(value); calloc.free(view); }', '  }']
    for symbol, operation in usage_operations(abi['functions']):
        host = {'usage_persistence': 'usagePersistence', 'finish_usage_status': 'finishUsageStatus'}[operation]
        lines += [f'  UsagePersistenceStatus {host}(Pointer<Void> engine) {{',
                  '    final state = calloc<_NativeUsageState>();', '    final advice = calloc<_NativeUsageAdvice>();',
                  '    try {', f'      _checkSession(this, {symbol}(engine, state, advice));',
                  '      return UsagePersistenceStatus(UsagePersistenceState.values.firstWhere((value) => value.code == state.ref.kind), advice.ref.data == nullptr ? null : utf8.decode(advice.ref.data.asTypedList(advice.ref.len)));',
                  '    } finally { calloc.free(state); calloc.free(advice); }', '  }']
    operations = [
        ('waitPacket', 'thinkthen_session_next_packet', 'THINKTHEN_SESSION_END_V1', 'THINKTHEN_SESSION_RESULT_V1', True),
        ('pushItem', 'thinkthen_session_push', 'THINKTHEN_SESSION_CLOSED_V1', 'THINKTHEN_SESSION_ACCEPTED_V1', False),
    ]
    for host, native, terminal, accepted, output in operations:
        assert native in abi['functions'] and terminal in abi['constants'] and accepted in abi['constants']
        signature = 'Pointer<Void> session' + ('' if output else ', String item')
        lines += [f'  int {host}({signature}) {{', '    final status = calloc<Uint32>();']
        if output: lines += ['    final value = calloc<Pointer<Void>>();']
        lines += ['    try {']
        if output:
            lines += [f'      _checkSession(this, {native}(session, THINKTHEN_SESSION_WAIT_SETTLEMENT_V1, status, value));']
        else:
            lines += [f'      _withText(item, (bytes, length) => _checkSession(this, {native}(session, bytes, length, status)));']
        lines += [f'      if (status.value == {terminal}) return ' + ('0;' if output else 'status.value;'),
                  f'      if (status.value != {accepted}) throw StateError("Invalid native {host} status");',
                  '      return ' + ('value.value.address;' if output else 'status.value;'),
                  '    } finally { calloc.free(status);' + (' calloc.free(value);' if output else '') + ' }', '  }']
    return lines


def calls(inputs, definitions):
    global PREFIX
    PREFIX = ''
    from host_answers import call_results, scalar_input_selectors
    rules = call_results(definitions)
    PREFIX = 'Input'
    singles = ' || '.join('input is ' + name(key) for key in scalar_input_selectors(inputs))
    PREFIX = ''
    lines = ['// Generated named calls and results from canonical contracts; do not edit.',
             'part of thinkthen_session;', 'mixin NamedCalls {',
             '  Future<OwnedCall> _call(String function, Question question, Input input, Options? options, Cancellation? cancellation, Stream<FeedItem>? feed);',
             '  OwnedSession _startSession(Question question, Input input, String function, {Options? options});']
    for entry in inputs['RequestFunction']['oneOf']:
        function = entry['const']
        rule = rules[function]
        payload = rule['receipt_payload']
        item = payload.get('items', payload)
        receipt = hint(item, definitions)
        value_type = hint(rule['aggregate_value'], definitions)
        if rule['call_shape'] == 'input': value_type = 'Object?'
        typ = f'CallResult<{value_type}, {receipt}>'
        lines += [f'  OwnedSession {function}Session(Question question, Input input, {{Options? options}}) => _startSession(question, input, "{function}", options: options);',
                  f'  Future<{typ}> {function}(Question question, Input input, {{Options? options, Cancellation? cancellation, Stream<FeedItem>? feed}}) async {{',
                  f'    final call = await _call("{function}", question, input, options, cancellation, feed);',
                  f'    final rows = <{receipt}>[];', '    var aggregated = false;', '    for (final packet in call.packets) {',
                  f'      if (packet is {name(rule["carrier"])}) {{', '        if (!aggregated) rows.clear();', '        aggregated = true;', '        ' + ('rows.addAll(packet.value);' if payload.get('type') == 'array' else 'rows.add(packet.value);'), '      }']
        if payload.get('type') == 'array':
            for key, _ in definitions['completesessionPacket']['variants']:
                fields = definitions[key].get('properties', {})
                if fields.get('kind', {}).get('const') == 'row' and fields.get('function', {}).get('const') == function and fields.get('value') == item:
                    lines += [f'      else if (!aggregated && packet is {name(key)}) rows.add(packet.value);']
        lines += ['    }']
        if function == 'rank': lines += ['    rows.sort((a, b) => a.value.compareTo(b.value));']
        rowdefs = definitions[item['$ref'].split('/')[-1]]
        projection = rule['value_projection']
        prop = member(projection)
        getter = f'row.{prop}'
        if projection not in rowdefs.get('required', []): getter += '.value'
        if function == 'filter':
            value = f'List<Object?>.unmodifiable(rows.where((row) => row.value == true).map((row) => {getter}))'
        elif function == 'rank':
            value = f'List<BigInt>.unmodifiable(rows.map((row) => {getter} ?? (throw StateError("Rank answer has no input index"))))'
        elif rule['call_shape'] == 'aggregate':
            value = 'rows.single.value'
        else:
            value = f'({singles}) ? rows.single.value : List.unmodifiable(rows.map((row) => row.value))'
        schema = rowdefs['properties'].get('value', {})
        target = resolved(schema, definitions) if isinstance(schema, dict) else {}
        if 'primitive_variants' in target or union(target, definitions):
            value = value.replace('row.value', 'row.value.value').replace('rows.single.value', 'rows.single.value.value')
        if function == 'annotate':
            value = value.replace('row.value', "row._fields['value']").replace('rows.single.value', "rows.single._fields['value']")
        mode = rule['probability_mode']
        chances = 'null'
        if mode != 'null':
            resolution = rule.get('resolution')
            if resolution:
                cond = ' && '.join('_atPath(packet._fields, ' + json.dumps(path) + ') == ' + json.dumps(literal) for path, literal in resolution['guards'])
                lines += ['    final resolved = <BigInt, bool>{};', '    for (final packet in call.packets) {',
                          f'      if ({cond}) resolved[_readInteger(_atPath(packet._fields, {json.dumps(resolution["index_path"])}))] = _atPath(packet._fields, {json.dumps(resolution["path"])}) != null;', '    }']
            if function == 'recognize':
                expr = '_recognitionProbability(row)'
            elif function == 'relate': expr = 'row.value.map((edge) => edge.probability.toDouble()).toList(growable: false)'
            else:
                path = rule.get('probability_path', ('answer', 'probability'))
                expr = f'_numberProbability(_atPath(row._fields, {json.dumps(path)}))'
                if mode == 'picked':
                    expr = f'_numberProbability((_atPath(row._fields, {json.dumps(path)}) as Map)[_atPath(row._fields, {json.dumps(rule["pick_path"])})])'
                if resolution:
                    expr = f'resolved[row.index.value ?? BigInt.zero] == true ? {expr} : null'
                elif mode == 'picked':
                    expr = f'_atPath(row._fields, {json.dumps(rule["selection_path"])}) != null ? {expr} : null'
            chance_rows = 'rows.where((row) => row.value == true)' if function == 'filter' else 'rows'
            lines += [f'    final chances = {chance_rows}.map((row) => {expr}).toList(growable: false);']
            if function == 'relate': chances = 'ListProbability(chances.single)'
            elif function == 'recognize': chances = f'({singles}) ? ListProbability(chances.single) : GroupProbability(chances)'
            elif rule['call_shape'] == 'aggregate': chances = '_scalarProbability(chances.single)'
            elif rule['call_shape'] == 'collection': chances = 'ListProbability(chances)'
            else: chances = f'({singles}) ? _scalarProbability(chances.single) : ListProbability(chances)'
        lines += [f'    return {typ}._({value}, {chances}, rows, call);', '  }']
    return '\n'.join(lines + ['}']) + '\n'


def native_readers(abi, definitions):
    """Read measured native graph fields, preserving exact integers and presence."""
    from c import normalized, nullable, ident, variant_member, container_key
    nodes, pending = {}, [key for key in ROOTS if key != 'completeplan']
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
    records = abi['records']
    lines = ['final class _NativePacketReader {']
    def slot(base, offset): return f'_at({base}, {offset})'
    def field(base, carrier, member): return slot(base, records[carrier]['fields'][member]['offset'])
    def carrier_type(source, path):
        source = normalized(source)
        if '$ref' in source: return ident(source['$ref'].split('/')[-1]) + ' *'
        if source == {} or source.get('type') == 'null': return 'thinkthen_complete_json_v1 *'
        if any(key in source for key in ('properties', 'enum', 'variants', 'primitive_variants')):
            nodes.setdefault(path, source)
            return ident(path) + ' *'
        kind = source.get('type')
        if kind == 'string' or isinstance(source.get('const'), str): return 'thinkthen_complete_utf8_v1'
        if kind == 'boolean': return 'uint32_t'
        if kind == 'number': return 'double'
        if kind == 'integer': return 'uint64_t' if source.get('minimum', -1) >= 0 or source.get('format', '').startswith('uint') else 'int64_t'
        return ident(container_key(source, path))
    def width(typ):
        if typ.endswith('*'): return abi['platform']['pointer']
        return {'uint32_t': 4, 'uint64_t': 8, 'int64_t': 8, 'double': 8}.get(typ) or records[typ]['size']
    def decode(source, path, pointer):
        source = normalized(source)
        typ = carrier_type(source, path)
        if '$ref' in source: return f'read{name(source["$ref"].split("/")[-1])}(_pointer({pointer}))'
        if source == {} or source.get('type') == 'null': return f'_value(_pointer({pointer}))'
        if any(key in source for key in ('properties', 'enum', 'variants', 'primitive_variants')): return f'read{name(path)}(_pointer({pointer}))'
        if typ == 'thinkthen_complete_utf8_v1': return f'_text({pointer})'
        if typ == 'uint32_t': return f'_u32({pointer}) != 0'
        if typ == 'uint64_t': return f'_unsigned({pointer})'
        if typ == 'int64_t': return f'BigInt.from({pointer}.cast<Int64>().value)'
        if typ == 'double': return f'{pointer}.cast<Double>().value'
        kind = source.get('type')
        item = source['items'] if kind == 'array' else source['additionalProperties']
        path = container_key(source, path) + ('_item' if kind == 'array' else '_entry_value')
        if kind == 'array':
            stride = width(carrier_type(item, path))
            return f'_list({pointer}, {stride}, (p) => {decode(item, path, "p")})'
        entry = typ.removesuffix('_v1') + '_entry_v1'
        return f'_map({pointer}, {records[entry]["size"]}, {records[entry]["fields"]["value"]["offset"]}, (p) => {decode(item, path, "p")})'
    emitted = set()
    while len(emitted) < len(nodes):
        key = next(key for key in nodes if key not in emitted)
        emitted.add(key)
        source, carrier = normalized(nodes[key]), ident(key)
        lines += [f'  static Object? read{name(key)}(Pointer<Void> p) {{']
        if 'properties' in source:
            lines += [f'    final values = _extensions({field("p", carrier, "extensions")});']
            for member, schema in source['properties'].items():
                native_member = {'true': 'true_', 'false': 'false_'}.get(member, member)
                address = field('p', carrier, native_member)
                is_presence = member not in source.get('required', []) or nullable(schema, definitions)
                if is_presence:
                    kind = records[carrier]['fields'][native_member]['type'].removeprefix('const ').removeprefix('struct ')
                    value = decode(schema, key + '_field_' + member, field(address, kind, 'value'))
                    lines += [f'    if (_u32({address}) != 0) values[{json.dumps(member)}] = _u32({address}) == 1 ? null : {value};']
                else:
                    lines += [f'    values[{json.dumps(member)}] = {decode(schema, key + "_field_" + member, address)};']
            lines += ['    return values;']
        elif 'variants' in source:
            lines += [f'    return switch (_u32({field("p", carrier, "kind")})) {{']
            for index, (child, _) in enumerate(source['variants'], 1):
                arm = variant_member(key, child)
                lines += [f'      {index} => read{name(child)}(_pointer({field("p", carrier, "data." + arm)})),']
            lines += ["      _ => throw StateError('Invalid native discriminator'),", '    };']
        elif 'enum' in source:
            lines += [f'    return switch (_u32({field("p", carrier, "kind")})) {{'] + [f'      {i} => {json.dumps(value)},' for i, value in enumerate(source['enum'], 1)] + ["      _ => throw StateError('Invalid native enum'),", '    };']
        elif 'primitive_variants' in source:
            lines += [f'    return switch (_u32({field("p", carrier, "kind")})) {{']
            for index, kind in enumerate(source['primitive_variants'], 1):
                expression = 'null' if kind == 'null' else decode(source['primitive_schemas'][kind], key + '_' + kind, field('p', carrier, 'data.' + kind))
                lines += [f'      {index} => {expression},']
            lines += ["      _ => throw StateError('Invalid native value'),", '    };']
        else:
            lines += [f'    return {decode(source, key + "_value", field("p", carrier, "value"))};']
        lines += ['  }']
    json_view = records['thinkthen_complete_json_v1']
    value_field = lambda arm: slot('p', json_view['fields']['data.' + arm]['offset'])
    extension = records['thinkthen_complete_extension_v1']
    lines += [f'  static Map<String, Object?> _extensions(Pointer<Void> p) => _map(p, {extension["size"]}, {extension["fields"]["value"]["offset"]}, (v) => _value(_pointer(v)));',
              '  static Object? _value(Pointer<Void> p) => switch (_u32(p)) {', '    1 => null,', f'    2 => _u32({value_field("boolean")}) != 0,', f'    3 => _number(_text({value_field("number")})),', f'    4 => _text({value_field("string")}),', f'    5 => _list({value_field("array")}, {abi["platform"]["pointer"]}, (v) => _value(_pointer(v))),', f'    6 => _map({value_field("object")}, {records["thinkthen_complete_json_entry_v1"]["size"]}, {records["thinkthen_complete_json_entry_v1"]["fields"]["value"]["offset"]}, (v) => _value(_pointer(v))),', "    _ => throw StateError('Invalid native value'),", '  };']
    error = records['thinkthen_error_view_v1']['fields']
    lines += ['  static CallFailure failure(Pointer<Void> p, int fallback) {', "    if (p == nullptr) return CallFailure._native(fallback, false, 'Native operation failed', null);", f'    final facts = _pointer(_at(p, {error["facts"]["offset"]}));', f'    return CallFailure._native(_at(p, {error["code"]["offset"]}).cast<Int32>().value, _at(p, {error["retryable"]["offset"]}).cast<Int32>().value != 0, _text(_at(p, {error["message"]["offset"]})), facts == nullptr ? null : FailureFacts.read(readFailureFacts(facts)));', '  }']
    lines += [NATIVE_READER_SUPPORT, '}']
    return '\n'.join(lines) + '\n'


NATIVE_READER_SUPPORT = '''  static Pointer<Void> _at(Pointer<Void> p, int offset) => (p.cast<Uint8>() + offset).cast<Void>();
  static Pointer<Void> _pointer(Pointer<Void> p) => p.cast<Pointer<Void>>().value;
  static int _u32(Pointer<Void> p) => p.cast<Uint32>().value;
  static BigInt _unsigned(Pointer<Void> p) {
    final value = p.cast<Uint64>().value;
    return value < 0 ? BigInt.from(value) + (BigInt.one << 64) : BigInt.from(value);
  }
  static String _text(Pointer<Void> p) {
    final data = p.cast<Pointer<Uint8>>().value;
    final length = _at(p, sizeOf<Pointer<Void>>()).cast<Size>().value;
    return length == 0 ? '' : utf8.decode(data.asTypedList(length));
  }
  static Object _number(String source) => BigInt.tryParse(source) ?? double.parse(source);
  static List<Object?> _list(Pointer<Void> p, int stride, Object? Function(Pointer<Void>) read) {
    final data = _pointer(p);
    final length = _at(p, sizeOf<Pointer<Void>>()).cast<Size>().value;
    return List.generate(length, (i) => read(_at(data, i * stride)), growable: false);
  }
  static Map<String, Object?> _map(Pointer<Void> p, int stride, int valueOffset, Object? Function(Pointer<Void>) read) {
    final data = _pointer(p);
    final length = _at(p, sizeOf<Pointer<Void>>()).cast<Size>().value;
    return {for (var i = 0; i < length; i++) _text(_at(data, i * stride)): read(_at(data, i * stride + valueOffset))};
  }'''
