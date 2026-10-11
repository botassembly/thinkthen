"""PHP owned objects and FFI declarations from the shared native contracts."""
import json
import re

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


def outputs(definitions, version, header):
    lines = ['<?php', '// Generated from the shared Rust result graph; do not edit.',
             'declare(strict_types=1);', 'namespace ThinkThen\\Results;']
    graph = {}
    for key, source in definitions.items():
        graph[key] = source
        if 'properties' not in source:
            continue
        graph[key] = {**source, 'class': name(key)}
        lines += [f'final class {name(key)} extends Node {{']
        for member, field in source['properties'].items():
            # Every accessor returns presence independently of the field value.
            lines += [f'    /** @return Presence<{hint(field, definitions)}> */ public function {member}(): Presence {{ return $this->field({member!r}); }}']
        lines += ['}']
    declarations = re.sub(r'/\*.*?\*/', '', header, flags=re.S)
    declarations = re.sub(r'//[^\n]*', '', declarations)
    declarations = re.sub(r'^\s*#.*$', '', declarations, flags=re.M)
    declarations = declarations.replace('extern "C" {', '').replace('} // extern "C"', '')
    # cbindgen closes the C++ linkage block with a bare brace.
    declarations = re.sub(r'^}\s*$', '', declarations, flags=re.M)
    declarations = '\n'.join(line.rstrip() for line in declarations.splitlines())
    constants = ['<?php', '// Generated from the native C header; do not edit.',
                 'declare(strict_types=1);', 'namespace ThinkThen\\Session;',
                 'const REQUEST_VERSION = ' + repr(version) + ';']
    for key, value in re.findall(r'^#define (THINKTHEN_(?:SESSION_\w+|E\w+|COMPLETE_USAGE_PERSISTENCE_\w+)) (\d+)\s*$', header, flags=re.M):
        constants += [f'const {key} = {value};']
    constants += ['namespace ThinkThen;', 'enum UsagePersistenceState: int {']
    constants += ['case ' + word.title() + ' = ' + value + ';' for word, value in re.findall(r'^#define THINKTHEN_COMPLETE_USAGE_PERSISTENCE_(\w+)_V1 (\d+)\s*$', header, flags=re.M)]
    constants += ['}']
    return {'results_generated.php': '\n'.join(lines) + '\n',
            'graph_generated.json': json.dumps(graph, indent=2) + '\n',
            'ffi_generated.h': declarations.strip() + '\n',
            'constants_generated.php': '\n'.join(constants) + '\n'}


INPUT_ROOTS = ('RequestInput', 'RequestQuestion', 'RequestReaderFailure',
               'Request', 'RequestCall', 'RequestItem', 'RequestSource', 'RequestReader',
               'RequestOptions', 'RequestSessionDescriptor')


def inputs(definitions):
    """Declare the input documents consumed by the PHP public adapter."""
    selected = set(INPUT_ROOTS)
    for key in INPUT_ROOTS:
        selected.update(child for child, _ in definitions[key].get('variants', []))
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
    return '\n'.join(lines) + '\n'
