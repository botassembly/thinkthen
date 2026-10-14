"""Rust-owned Ruby results derived from the prepared shared graph."""
import json
from host_answers import call_results
ROOTS = ('completesessionPacket', 'completeplan')

def name(key):
    return 'Native' + ''.join(part[:1].upper() + part[1:] for part in key.removeprefix('complete').split('_'))

def render(definitions):
    lines = ['# Generated from the shared Rust result graph; do not edit.', 'module ThinkThen', '  module Results']
    for key, source in definitions.items():
        if 'properties' not in source: continue
        lines += [f'    class {name(key)} < Native::Result']
        if key == 'completeplan':
            lines += ['      FIELDS = ' + repr(list(source['properties'])) + '.freeze']
        for member in source['properties']:
            lines += [f'      define_method({member!r}) {{ self[{member!r}] }}']
        lines += ['    end']
    metadata = {verb: {key: value for key, value in spec.items() if key in ('value_projection', 'probability_mode', 'probability_path', 'pick_path')} | {'collection': spec['receipt_payload'].get('type') == 'array' and spec['scalar_kind'] is None} for verb, spec in call_results(definitions).items()}
    lines += ['    CALL_RESULTS = JSON.parse(' + repr(json.dumps(metadata)) + ').freeze']
    lines += ['  end', '  class Client', *['    def self.choose_question(text, options:)', '      {choose: text, options: options}.freeze', '    end', '    def self.recognize_question(kinds:, **fields)', '      {version: 1, recognize: {kinds: kinds, **fields}}.freeze', '    end', '    def self.relation(name, **fields)', '      {name: name, **fields}.freeze', '    end', '    def self.relate_question(relations:, fields: nil)', '      body = {relations: relations}', '      body[:fields] = fields unless fields.nil?', '      {version: 1, relate: body}.freeze', '    end'], '  end', 'end']
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


def rbs(definitions):
    from templates import ruby_rbs
    return ruby_rbs.render(definitions, name)
