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
