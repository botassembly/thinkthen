"""Rust-owned Ruby results derived from the prepared shared graph."""
import json
ROOTS = ('completesessionPacket',)

def name(key):
    return 'Native' + ''.join(part[:1].upper() + part[1:] for part in key.removeprefix('complete').split('_'))

def render(definitions):
    lines = ['# Generated from the shared Rust result graph; do not edit.', 'module ThinkThen', '  module Results']
    for key, source in definitions.items():
        if 'properties' not in source: continue
        lines += [f'    class {name(key)} < Native::Result']
        for member in source['properties']:
            lines += [f'      define_method({member!r}) {{ self[{member!r}] }}']
        lines += ['    end']
    lines += ['  end', 'end']
    return '\n'.join(lines) + '\n'

def rust(definitions):
    # Compact generated metadata is consumed only by the native value converter.
    # Every property and alternative comes from the common prepared graph.
    metadata = {}
    for key, source in definitions.items():
        if 'properties' in source:
            metadata[key] = {'class':name(key), 'fields':source['properties']}
        elif 'variants' in source:
            metadata[key] = {'variants':source['variants']}
        elif 'primitive_schemas' in source:
            metadata[key] = {'primitive_schemas':source['primitive_schemas']}
        else:
            metadata[key] = source
    lines = ['// Generated from the shared Rust result graph; do not edit.', 'pub(super) const GRAPH: &str = concat!(', json.dumps('{') + ',']
    for index, (key, source) in enumerate(metadata.items()):
        prefix = (',' if index else '') + json.dumps(key) + ':'
        if 'fields' in source:
            lines.append(json.dumps(prefix + '{"class":' + json.dumps(source['class']) + ',"fields":{') + ',')
            for member_index, (member, field) in enumerate(source['fields'].items()):
                fragment = (',' if member_index else '') + json.dumps(member) + ':' + json.dumps(field, separators=(',', ':'))
                lines.append(json.dumps(fragment) + ',')
            lines.append(json.dumps('}}') + ',')
        else:
            lines.append(json.dumps(prefix + json.dumps(source, separators=(',', ':'))) + ',')
    lines += [json.dumps('}'), ');']
    return '\n'.join(lines[:2] + ['    ' + line for line in lines[2:-1]] + lines[-1:]) + '\n'
