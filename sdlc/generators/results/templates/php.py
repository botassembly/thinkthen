"""PHP owned objects and FFI declarations from the shared native contracts."""
import json
import re
from native import selected_header
from c import normalized, nullable, variant_member, container_key
from host_answers import scalar_answers, aggregate_answers, aggregate_projection, judgment_route, named_object_fields

ROOTS = ('completesessionPacket', 'completeImmediateFacts', 'completeplan')


def name(key):
    key = key.removeprefix('complete')
    semantic = {'Atomic_DecideValue': 'DecisionResult', 'Atomic_Nullable_string': 'ChoiceResult', 'Atomic_Array_of_string': 'TagResult', 'Atomic_double': 'ScoreResult', 'Atomic_boolean': 'FilterResult', 'Atomic_NonZeroUsize': 'RankResult', 'ReadableQuestion2': 'FindQuestionInfo', 'ReadableQuestion3': 'RecognitionQuestionInfo', 'ReadableQuestion4': 'RelationQuestionInfo', 'ImmediateFacts': 'FailureFacts'}
    return semantic.get(key, ''.join(part[:1].upper() + part[1:] for part in key.split('_')))


def input_name(key):
    semantic = {'RecognizeQuestion': 'RecognitionSpec', 'RecognizeDefinition': 'RecognitionDefinition', 'RequestDefinition_RecognizeDefinition': 'RecognizeQuestion', 'RequestDefinition_Authored_relate': 'RelateQuestion', 'RelateQuestion': 'RelationSpec', 'Authored_relation': 'RelationRule', 'RequestDefinition_Authored_choose': 'ChooseQuestion', 'RequestOptions': 'Options', 'EngineSettings': 'Settings'}
    return semantic.get(key, ''.join(part[:1].upper() + part[1:] for part in key.removeprefix('Request').split('_')))


def hint(source, definitions):
    if not isinstance(source, dict):
        return 'mixed'
    if '$ref' in source:
        key = source['$ref'].removeprefix('#/$defs/')
        return name(key) if 'properties' in definitions[key] else hint(definitions[key], definitions)
    if 'variants' in source:
        return '|'.join(name(child) for child, _ in source['variants'])
    if 'anyOf' in source:
        return '|'.join(dict.fromkeys(hint(child, definitions) for child in source['anyOf']))
    kind = source.get('type')
    if isinstance(kind, list):
        types = [hint({**source, 'type': value}, definitions) for value in kind]
        return '|'.join(dict.fromkeys(types))
    if kind == 'array':
        return 'list<' + hint(source['items'], definitions) + '>'
    return {'integer': 'int|string', 'number': 'int|float', 'boolean': 'bool',
            'string': 'string', 'null': 'null', 'object': '\\stdClass'}.get(kind, 'mixed') if isinstance(kind, str) else 'mixed'


def outputs(definitions, header):
    lines = ['<?php', '// Generated from the shared Rust result graph; do not edit.',
             'declare(strict_types=1);', 'namespace ThinkThen\\Results;']
    lines += answer_helpers(definitions)
    for key, source in definitions.items():
        if 'properties' not in source:
            continue
        lines += [f'final class {name(key)} extends Node {{']
        for member, field in source['properties'].items():
            # Every accessor returns presence independently of the field value.
            lines += [f'    /** @return Presence<{hint(field, definitions)}> */ public function {member}(): Presence {{ return $this->field({member!r}); }}']
        for member in named_object_fields(source, definitions):
            lines += [f'    public function {member}Name(): string {{ return $this->{member}->name; }}']
        lines += ['}']
    lines += native_readers(definitions)
    declarations = re.sub(r'^#.*$', '', selected_header(header, 'php'), flags=re.M)
    constants = ['<?php', '// Generated from the native C header; do not edit.',
                 'declare(strict_types=1);', 'namespace ThinkThen\\Internal\\Session;']
    for key, value in re.findall(r'^#define (THINKTHEN_(?:SESSION_\w+|E\w+|COMPLETE_USAGE_PERSISTENCE_\w+)) (\d+)\s*$', header, flags=re.M):
        constants += [f'const {key} = {value};']
    constants += ['namespace ThinkThen;', 'enum UsagePersistenceState: int {']
    constants += ['case ' + word.title() + ' = ' + value + ';' for word, value in re.findall(r'^#define THINKTHEN_COMPLETE_USAGE_PERSISTENCE_(\w+)_V1 (\d+)\s*$', header, flags=re.M)]
    errors = re.findall(r'^#define (THINKTHEN_E\w+) (\d+)\s*$', header, flags=re.M)
    constants += ['}', 'enum ErrorKind: int {']
    constants += ['    case ' + symbol.removeprefix('THINKTHEN_E').title() + ' = ' + code + ';' for symbol, code in errors]
    constants += ['}']
    constants += ['final class ' + symbol.removeprefix('THINKTHEN_E').title() + 'Failure extends CallFailure { public const KIND = ErrorKind::' + symbol.removeprefix('THINKTHEN_E').title() + '; }' for symbol, _ in errors]
    constants += ['namespace ThinkThen\\Internal;', 'final class ErrorFactory', '{', '    public static function classFor(string $kind): string', '    {', '        return match ($kind) {']
    constants += ["            '" + symbol.removeprefix('THINKTHEN_E').lower() + "' => \\ThinkThen\\" + symbol.removeprefix('THINKTHEN_E').title() + 'Failure::class,' for symbol, _ in errors]
    constants += ['            default => \\ThinkThen\\DefectFailure::class,', '        };', '    }', '    public static function raise(\\FFI $ffi, int $code, bool $session): never', '    {', '        $view = $session ? $ffi->thinkthen_session_error_view() : $ffi->thinkthen_error_view(null);', '        $retryable = false; $facts = null;', '        if ($view !== null && !\\FFI::isNull($view)) {', '            $code = $view->code;', '            $retryable = (bool)$view->retryable;', "            $message = $view->message->len === 0 ? '' : \\FFI::string($view->message->data, $view->message->len);", '            if (!\\FFI::isNull($view->facts)) $facts = Decoder::failureFacts($view->facts);', '        } else {', '            $text = $session ? $ffi->thinkthen_session_error_message() : $ffi->thinkthen_error_message(null);', '            $message = is_string($text) ? $text : \\FFI::string($text);', '        }', '        $class = match ($code) {']
    constants += ['            ' + code + ' => \\ThinkThen\\' + symbol.removeprefix('THINKTHEN_E').title() + 'Failure::class,' for symbol, code in errors]
    constants += ['            default => \\ThinkThen\\DefectFailure::class,', '        };', '        throw new $class($message, retryable: $retryable, immediateFacts: $facts);', '    }', '}']
    return {'results_generated.php': '\n'.join(lines) + '\n',
            'ffi_generated.h': declarations.strip() + '\n',
            'constants_generated.php': '\n'.join(constants) + '\n'}


INPUT_ROOTS = ('EngineSettings', 'RequestFunction', 'RequestInput', 'RequestQuestion', 'RequestReaderFailure',
               'RequestItem', 'RequestSource', 'RequestReader',
               'RequestOptions', 'RequestSessionDescriptor')


def inputs(definitions):
    """Declare the input documents consumed by the PHP public adapter."""
    selected = set(INPUT_ROOTS)
    # The adapter accepts common records and preserves selector descriptors.
    # Emit only the tagged constructors it instantiates.
    selected.update(('RecognizeQuestion', 'RelateQuestion', 'Authored_relation', 'RequestDefinition_RecognizeDefinition', 'RequestDefinition_Authored_choose', 'RequestDefinition_Authored_relate', 'RequestInput_records', 'RequestInput_units', 'RequestInput_entities', 'RequestInput_source', 'RequestInput_feed',
                     'RequestQuestion_text', 'RequestQuestion_definition', 'RequestQuestion_file', 'RequestQuestion_name', 'RequestQuestion_reference',
                     'RequestReaderFailure_producer', 'RecognitionStageContext', 'RecognitionExample', 'RecognitionSeedSpan'))
    lines = ['<?php', '// Generated from the shared Rust request graph; do not edit.',
             'declare(strict_types=1);', 'namespace ThinkThen\\Inputs;', 'enum Omitted { case Value; }', 'interface QuestionSource {}', 'interface QuestionDefinition {}']
    for key, source in definitions.items():
        if key not in selected:
            continue
        if 'properties' not in source:
            continue
        fields = source['properties']
        defaults = {member: field['const'] for member, field in fields.items() if isinstance(field, dict) and 'const' in field}
        required = [member for member in source.get('required', []) if member not in defaults]
        def native_type(source):
            if not isinstance(source, dict): return 'mixed'
            if '$ref' in source:
                key = source['$ref'].removeprefix('#/$defs/')
                return input_name(key) if key in selected and 'properties' in definitions[key] else native_type(definitions[key])
            alternatives = source.get('anyOf', source.get('oneOf', []))
            if alternatives:
                types = [native_type(child) for child in alternatives]
                return 'mixed' if 'mixed' in types else '|'.join(dict.fromkeys('|'.join(types).split('|')))
            kind = source.get('type')
            if isinstance(kind, list):
                return '|'.join(dict.fromkeys(native_type({**source, 'type': child}) for child in kind))
            if 'const' in source:
                kind = {str: 'string', bool: 'boolean', int: 'integer', type(None): 'null'}.get(type(source['const']))
            return {'string': 'string', 'array': 'array', 'object': 'array|\\stdClass', 'boolean': 'bool',
                    'integer': 'int', 'number': 'int|float', 'null': 'null'}.get(kind, 'mixed')
        def parameter(member):
            return native_type(fields[member])
        parameters = ', '.join(parameter(member) + ' $' + member for member in required)
        values = ', '.join(repr(member) + ' => $' + member for member in required)
        tags = ', '.join(repr(k) + ' => ' + repr(v) for k, v in defaults.items())
        public = key in {'RequestQuestion_file', 'RequestQuestion_name', 'RequestQuestion_reference', 'RequestOptions', 'EngineSettings', 'RecognitionStageContext', 'RecognitionExample', 'RecognitionSeedSpan', 'RecognizeQuestion', 'RecognizeDefinition', 'RequestDefinition_RecognizeDefinition', 'RequestDefinition_Authored_choose', 'RequestDefinition_Authored_relate', 'RelateQuestion', 'Authored_relation'}
        namespace = 'ThinkThen\\Inputs' if public else 'ThinkThen\\Internal\\Requests'
        lines += ['namespace ' + namespace + ';', 'final class ' + input_name(key) + ' extends \\ThinkThen\\Internal\\Document' + (' implements QuestionSource' if key in {'RequestQuestion_file', 'RequestQuestion_name', 'RequestQuestion_reference'} else (' implements \\ThinkThen\\Inputs\\QuestionDefinition' if key.startswith('RequestDefinition_') else '')), '{', *([('    private const TAGS = [' if public else '    public const TAGS = [') + tags + '];'] if tags else [])]
        if public:
            args, entries = [], []
            for member, field in fields.items():
                if member in defaults: continue
                typ = parameter(member)
                reference = field.get('$ref', '').split('/')[-1] if isinstance(field, dict) else ''
                if reference in selected and reference in {'RecognizeQuestion', 'RelateQuestion'}: typ = input_name(reference)
                optional = member not in required
                args.append((optional, 'public readonly ' + typ + ('|Omitted' if optional and typ != 'mixed' else '') + ' $' + member + (' = Omitted::Value' if optional else '')))
                entries.append(('...($' + member + ' === Omitted::Value ? [] : [' + repr(member) + ' => $' + member + '])') if optional else repr(member) + ' => $' + member)
            lines += ['    public function __construct(' + ', '.join(argument for _, argument in sorted(args, key=lambda item: item[0])) + ')', '    {', '        parent::__construct([' + ', '.join((['...self::TAGS'] if tags else []) + entries) + ']);', '    }', '}']
        else:
            lines += ['    public function __construct(' + parameters + (', ' if parameters else '') + 'array $fields = [])',
                      '    {', '        parent::__construct([' + ', '.join(part for part in ('...self::TAGS' if tags else '', values) if part) + ', ...$fields]);' if tags or values else '        parent::__construct($fields);',
                      '    }', '}']
    lines += ['namespace ThinkThen;', 'trait NamedCalls', '{']
    for function in definitions['RequestFunction']['oneOf']:
        value = function['const']
        lines += [f"    public function {value}(string|array|\\stdClass|\\ThinkThen\\Inputs\\QuestionDefinition|\\ThinkThen\\Inputs\\QuestionSource $question, mixed $input, \\ThinkThen\\Inputs\\Options $options = new \\ThinkThen\\Inputs\\Options(), ?Cancellation $cancel = null): \\ThinkThen\\Results\\Completed {{ return $this->call('{value}', $question, $input, $options, $cancel); }}"]
    lines += ['}']
    return '\n'.join(lines) + '\n'


def native_readers(definitions):
    "Copy the Rust decoded graph into owned idiomatic PHP values."
    nodes = {key: value for key, value in definitions.items() if key not in plan_types(definitions)}
    def expression(source, value, path):
        source = normalized(source)
        if '$ref' in source:
            key = source['$ref'].removeprefix('#/$defs/')
            return f'self::read{ name(key) }({value}[0])'
        if source == {} or source.get('type') == 'null':
            return f'self::jsonValue({value}[0])'
        if any(k in source for k in ('enum', 'variants', 'primitive_variants', 'properties')):
            nodes.setdefault(path, source)
            return f'self::read{ name(path) }({value}[0])'
        kind = source.get('type')
        if kind == 'string' or isinstance(source.get('const'), str):
            return f'self::text({value})'
        if kind == 'boolean':
            return f'(bool)({value})'
        if kind == 'integer':
            unsigned = source.get('minimum', -1) >= 0 or source.get('format', '').startswith('uint')
            return f'self::unsigned({value})' if unsigned else value
        if kind == 'number':
            return value
        if kind == 'array':
            return f'self::sequence({value}, fn($v) => ' + expression(source['items'], '$v', container_key(source, path) + '_item') + ')'
        if kind == 'object':
            return f'self::mapping({value}, fn($v) => ' + expression(source['additionalProperties'], '$v', container_key(source, path) + '_entry_value') + ')'
        raise ValueError(source)
    lines = ['namespace ThinkThen\\Internal;', 'final class Decoder', '{', NATIVE_SUPPORT]
    lines += plan_readers(definitions)
    emitted = set()
    while len(emitted) < len(nodes):
        key = next(k for k in nodes if k not in emitted)
        emitted.add(key)
        source = normalized(nodes[key])
        lines += [f'    private static function read{name(key)}(\\FFI\\CData $native): mixed', '    {']
        if 'properties' in source:
            lines += ['        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));']
            for member, field in source['properties'].items():
                native = '$native->' + {'true': 'true_', 'false': 'false_'}.get(member, member)
                optional = member not in source.get('required', [])
                presence = optional or nullable(field, definitions)
                read = expression(field, native + ('->value' if presence else ''), key + '_field_' + member)
                if presence:
                    read = f'{native}->presence === 1 ? null : ({read})'
                lines += [f'        ' + (f'if ({native}->presence !== 0) ' if presence else '') + f'$values[{member!r}] = {read};']
            lines += [f'        return new \\ThinkThen\\Results\\{name(key)}($values);']
        elif 'variants' in source:
            lines += ['        return match ($native->kind) {']
            for index, (child, _) in enumerate(source['variants'], 1):
                lines += [f'            {index} => self::read{name(child)}($native->data->{variant_member(key, child)}[0]),']
            lines += ["            default => throw new \\LogicException('invalid native discriminator'),", '        };']
        elif 'enum' in source:
            lines += ['        return match ($native->kind) {'] + [f'            {i} => {value!r},' for i, value in enumerate(source['enum'], 1)] + ["            default => throw new \\LogicException('invalid native enum'),", '        };']
        elif 'primitive_variants' in source:
            lines += ['        return match ($native->kind) {']
            for index, kind in enumerate(source['primitive_variants'], 1):
                read = 'null' if kind == 'null' else expression(source['primitive_schemas'][kind], '$native->data->' + kind, key + '_' + kind)
                lines += [f'            {index} => {read},']
            lines += ["            default => throw new \\LogicException('invalid native value'),", '        };']
        else:
            lines += ['        return ' + expression(source, '$native->value', key + '_value') + ';']
        lines += ['    }']
    return lines + ['}']


NATIVE_SUPPORT = r'''    public static function plan(\FFI\CData $view): \ThinkThen\Results\Plan
    {
        return self::planPlan(self::jsonValue($view[0]));
    }
    public static function failureFacts(\FFI\CData $view): \ThinkThen\Results\FailureFacts
    {
        return self::readFailureFacts($view[0]);
    }
    public static function native(\FFI\CData $view): \ThinkThen\Results\Node
    {
        return self::readSessionPacket($view[0]);
    }
    private static function text(\FFI\CData $view): string
    {
        return $view->len === 0 ? '' : \FFI::string($view->data, $view->len);
    }
    private static function unsigned(int $value): int|string
    {
        return $value < 0 ? sprintf('%u', $value) : $value;
    }
    private static function sequence(\FFI\CData $view, callable $read): array
    {
        $values = [];
        for ($i = 0; $i < $view->len; ++$i) $values[] = $read($view->data[$i]);
        return $values;
    }
    private static function mapping(\FFI\CData $view, callable $read): \stdClass
    {
        $values = new \stdClass();
        for ($i = 0; $i < $view->len; ++$i) {
            $entry = $view->data[$i];
            $values->{self::text($entry->name)} = $read($entry->value);
        }
        return $values;
    }
    private static function number(string $value): int|float|string
    {
        if (strpbrk($value, '.eE') !== false) {
            $number = (float)$value;
            return is_finite($number) && json_encode($number, JSON_PRESERVE_ZERO_FRACTION) === $value ? $number : $value;
        }
        $integer = filter_var($value, FILTER_VALIDATE_INT);
        return $integer === false ? $value : $integer;
    }
    private static function jsonValue(\FFI\CData $value): mixed
    {
        return match ($value->kind) {
            1 => null,
            2 => (bool)$value->data->boolean,
            3 => self::number(self::text($value->data->number)),
            4 => self::text($value->data->string),
            5 => self::sequence($value->data->array, fn($v) => self::jsonValue($v[0])),
            6 => self::mapping($value->data->object, fn($v) => self::jsonValue($v[0])),
            default => throw new \LogicException('invalid native JSON value'),
        };
    }'''


def answer_helpers(definitions):
    path, guards = judgment_route(definitions)
    access = '$packet' + ''.join('->' + field for field in path)
    checks = ' && '.join('$packet' + ''.join('->' + f for f in fields) + ' === ' + repr(value) for fields, value in guards)
    lines = ['trait AnswerAccessors', '{', '    private function requireSuccess(): void', '    {', "        if (!$this->terminal->has('failure')) return;", '        $failure = $this->terminal->failure;', '        $class = \\ThinkThen\\Internal\\ErrorFactory::classFor($failure->error->kind);', '        throw new $class($failure->error->message, $failure, $this->results, $this->terminal, $this->packets);', '    }']
    for function, kind, schema in scalar_answers(definitions):
        typ = hint(schema, definitions)
        lines += [f'    /** @return {typ} */ public function {function}Answer(): ' + ('array' if typ.startswith('list<') else typ), '    {', '        $this->requireSuccess();', '        $answers = [];', '        foreach ($this->packets as $packet) {', f'            if ({checks} && {access}->kind === {kind!r}) $answers[] = {access}->value;', '        }', r"        if (count($answers) !== 1) throw new \ThinkThen\UsageFailure('expected one answer');", '        return $answers[0];', '    }']
    for function, packet, schema in aggregate_answers(definitions):
        if function in {f for f, _, _ in scalar_answers(definitions)}: continue
        projected, member = aggregate_projection(function, schema, definitions)
        typ = hint(projected, definitions)
        lines += [f'    /** @return {typ} */ public function {function}Answer(): ' + ('array' if typ.startswith('list<') else typ), '    {', '        $this->requireSuccess();']
        if schema.get('type') == 'array':
            lines += ['        $answer = []; $matched = false;', '        foreach ($this->packets as $packet) {', f'            if ($packet instanceof {name(packet)}) {{ $matched = true; array_push($answer, ...$packet->value); }}']
            for child, _ in definitions['completesessionPacket']['variants']:
                fields = definitions[child].get('properties', {})
                if fields.get('kind', {}).get('const') == 'row' and fields.get('function', {}).get('const') == function and fields.get('value') == schema['items']:
                    lines += [f'            elseif ($packet instanceof {name(child)}) {{ $matched = true; $answer[] = $packet->value; }}']
            lines += ['        }', r"        if (!$matched) throw new \ThinkThen\UsageFailure('expected an answer for this function');"]
        else:
            lines += [f'        $values = array_values(array_filter($this->packets, fn($p) => $p instanceof {name(packet)}));', r"        if (count($values) !== 1) throw new \ThinkThen\UsageFailure('expected one answer');", '        $answer = $values[0]->value;']
        if member:
            lines += [f'        return array_map(fn($row) => $row->{member}, $answer);']
        else: lines += ['        return $answer;']
        lines += ['    }']
    return lines + ['}']


def plan_readers(definitions):
    """Decode the native canonical value with the public plan schema."""
    lines = []
    for key in sorted(plan_types(definitions)):
        lines += [f'    private static function plan{name(key)}(\\stdClass $value): \\ThinkThen\\Results\\{name(key)}', '    {', '        $values = (array)$value;']
        for member, field in definitions[key]['properties'].items():
            if '$ref' in field:
                child = field['$ref'].split('/')[-1]
                lines += [f'        $values[{member!r}] = self::plan{name(child)}($value->{member});']
        lines += [f'        return new \\ThinkThen\\Results\\{name(key)}($values);', '    }']
    return lines


def plan_types(definitions):
    """Find plan object types read from canonical native values."""
    pending, seen = ['completeplan'], set()
    while pending:
        key = pending.pop()
        if key in seen: continue
        seen.add(key)
        pending.extend(field['$ref'].split('/')[-1] for field in definitions[key]['properties'].values() if '$ref' in field)
    return seen
