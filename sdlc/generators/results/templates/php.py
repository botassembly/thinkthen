"""PHP owned objects and FFI declarations from the shared native contracts."""
import json
import re
from native import selected_header
from c import normalized, nullable, variant_member, container_key
from host_answers import scalar_answers, aggregate_answers, judgment_route, named_object_fields

ROOTS = ('completesessionPacket',)


def name(key):
    return 'Native' + ''.join(part[:1].upper() + part[1:] for part in key.removeprefix('complete').split('_'))


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
    graph = {}
    lines += answer_helpers(definitions)
    for key, source in definitions.items():
        graph[key] = source
        if 'properties' not in source:
            continue
        graph[key] = {**source, 'class': name(key)}
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
                 'declare(strict_types=1);', 'namespace ThinkThen\\Session;']
    for key, value in re.findall(r'^#define (THINKTHEN_(?:SESSION_\w+|E\w+|COMPLETE_USAGE_PERSISTENCE_\w+)) (\d+)\s*$', header, flags=re.M):
        constants += [f'const {key} = {value};']
    constants += ['namespace ThinkThen;', 'enum UsagePersistenceState: int {']
    constants += ['case ' + word.title() + ' = ' + value + ';' for word, value in re.findall(r'^#define THINKTHEN_COMPLETE_USAGE_PERSISTENCE_(\w+)_V1 (\d+)\s*$', header, flags=re.M)]
    constants += ['}', 'trait NativeErrors', '{', '    public static function throwImmediate(int $code, string $message): never', '    {', '        $class = match ($code) {']
    for symbol in re.findall(r'^#define (THINKTHEN_E\w+) \d+\s*$', header, flags=re.M):
        constants += ['            \\ThinkThen\\Session\\' + symbol + ' => ' + symbol.removeprefix('THINKTHEN_E').title() + 'Failure::class,']
    constants += ['            default => DefectFailure::class,', '        };', '        throw new $class($message);', '    }', '}']
    return {'results_generated.php': '\n'.join(lines) + '\n',
            'graph_generated.json': json.dumps({'$comment': 'Generated from the shared Rust result graph; do not edit.', **graph}, indent=2) + '\n',
            'ffi_generated.h': declarations.strip() + '\n',
            'constants_generated.php': '\n'.join(constants) + '\n'}


INPUT_ROOTS = ('RequestFunction', 'RequestInput', 'RequestQuestion', 'RequestReaderFailure',
               'RequestItem', 'RequestSource', 'RequestReader',
               'RequestOptions', 'RequestSessionDescriptor')


def inputs(definitions):
    """Declare the input documents consumed by the PHP public adapter."""
    selected = set(INPUT_ROOTS)
    # The adapter accepts common records and preserves selector descriptors.
    # Emit only the tagged constructors it instantiates.
    selected.update(('RecognizeQuestion', 'RelateQuestion', 'Authored_relation', 'RecognizeDefinition', 'RequestDefinition_Authored_relate', 'RequestInput_records', 'RequestInput_units', 'RequestInput_entities', 'RequestInput_source', 'RequestInput_feed',
                     'RequestQuestion_text', 'RequestQuestion_definition',
                     'RequestReaderFailure_producer'))
    lines = ['<?php', '// Generated from the shared Rust request graph; do not edit.',
             'declare(strict_types=1);', 'namespace ThinkThen\\Inputs;']
    for key, source in definitions.items():
        if key not in selected:
            continue
        if 'properties' not in source:
            if key == 'RequestQuestion':
                lines += ['final class InputRequestQuestion extends Document', '{', '}']
            continue
        fields = source['properties']
        defaults = {member: field['const'] for member, field in fields.items() if 'const' in field}
        required = [member for member in source.get('required', []) if member not in defaults]
        def parameter(member):
            field = fields[member]
            source = definitions[field['$ref'].removeprefix('#/$defs/')] if '$ref' in field else field
            alternatives = source.get('oneOf', [])
            if len(alternatives) == 1:
                source = alternatives[0]
            kind = source.get('type')
            native = {'string': 'string', 'array': 'array', 'boolean': 'bool', 'integer': 'int', 'number': 'int|float'}
            return native.get(kind, 'mixed') if isinstance(kind, str) else 'mixed'
        parameters = ', '.join(parameter(member) + ' $' + member for member in required)
        values = ', '.join(repr(member) + ' => $' + member for member in required)
        tags = ', '.join(repr(k) + ' => ' + repr(v) for k, v in defaults.items())
        lines += ['final class Input' + ''.join(part[:1].upper() + part[1:] for part in key.split('_')) + ' extends Document', '{',
                  *(['    public const TAGS = [' + tags + '];'] if tags else []),
                  '    public function __construct(' + parameters + (', ' if parameters else '') + 'array $fields = [])',
                  '    {', '        parent::__construct([' + ', '.join(part for part in ('...self::TAGS' if tags else '', values) if part) + ', ...$fields]);' if tags or values else '        parent::__construct($fields);',
                  '    }', '}']
    lines += ['namespace ThinkThen;', 'trait NamedCalls', '{']
    for function in definitions['RequestFunction']['oneOf']:
        value = function['const']
        lines += [f"    public function {value}(string|array|\\stdClass|\\ThinkThen\\Inputs\\Document|QuestionInput $question, mixed $input, array $options = [], ?Cancellation $cancel = null): \\ThinkThen\\Results\\Completed {{ return $this->call('{value}', $question, $input, $options, $cancel); }}"]
    lines += ['}']
    return '\n'.join(lines) + '\n'


def native_readers(definitions):
    "Copy the Rust decoded graph into owned idiomatic PHP values."
    nodes = dict(definitions)
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
    lines = ['final class Decoder', '{', NATIVE_SUPPORT]
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
            lines += [f'        return new {name(key)}((object)$values, $values);']
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


NATIVE_SUPPORT = r'''    public static function native(\FFI\CData $view): Node
    {
        return self::readNativeSessionPacket($view[0]);
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
        if (strpbrk($value, '.eE') !== false) return (float)$value;
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
    lines = ['trait Answers', '{']
    for function, kind, schema in scalar_answers(definitions):
        typ = hint(schema, definitions)
        lines += [f'    /** @return {typ} */ public function {function}Answer(): mixed', '    {', '        $answers = [];', '        foreach ($this->packets as $packet) {', f'            if ({checks} && {access}->kind === {kind!r}) $answers[] = {access}->value;', '        }', r"        if (count($answers) !== 1) throw new \ThinkThen\UsageFailure('expected one answer');", '        return $answers[0];', '    }']
    for function, packet, schema in aggregate_answers(definitions):
        if function in {f for f, _, _ in scalar_answers(definitions)}: continue
        lines += [f'    /** @return {hint(schema, definitions)} */ public function {function}Answer(): mixed', '    {']
        if schema.get('type') == 'array':
            lines += ['        $answer = [];', '        foreach ($this->packets as $packet) {', f'            if ($packet instanceof {name(packet)}) array_push($answer, ...$packet->value);']
            for child, _ in definitions['completesessionPacket']['variants']:
                fields = definitions[child].get('properties', {})
                if fields.get('kind', {}).get('const') == 'row' and fields.get('function', {}).get('const') == function and fields.get('value') == schema['items']:
                    lines += [f'            elseif ($packet instanceof {name(child)}) $answer[] = $packet->value;']
            lines += ['        }']
        else:
            lines += [f'        $values = array_values(array_filter($this->packets, fn($p) => $p instanceof {name(packet)}));', r"        if (count($values) !== 1) throw new \ThinkThen\UsageFailure('expected one answer');", '        $answer = $values[0]->value;']
        if function == 'filter':
            lines += ['        return array_values(array_map(fn($row) => $row->input, array_filter($answer, fn($row) => $row->value === true)));']
        elif function == 'rank':
            lines += ['        $answer = array_values(array_filter($answer, fn($row) => $row->value !== null));', '        usort($answer, fn($a, $b) => $a->value <=> $b->value);', '        return array_map(fn($row) => $row->input, $answer);']
        else: lines += ['        return $answer;']
        lines += ['    }']
    return lines + ['}']
