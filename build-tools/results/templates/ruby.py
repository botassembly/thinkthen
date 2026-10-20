"""Rust-owned Ruby results derived from the prepared shared graph."""
import json
ROOTS = ('completesessionPacket', 'completeplan')

def name(key):
    return 'Native' + ''.join(part[:1].upper() + part[1:] for part in key.removeprefix('complete').split('_'))

def literal(value):
    if isinstance(value, dict): return '{' + ', '.join(literal(k) + ' => ' + literal(v) for k, v in value.items()) + '}'
    if isinstance(value, list): return '[' + ', '.join(map(literal, value)) + ']'
    return {True: 'true', False: 'false', None: 'nil'}.get(value, repr(value)) if not isinstance(value, str) else repr(value)

def calls(definitions):
    payloads = {}
    for source in definitions.values():
        fields = source.get('properties', {})
        if fields.get('kind', {}).get('const') == 'aggregate':
            payloads[fields['function']['const']] = fields['value']
    return {definitions[key]['properties']['function']['const']: {
                'value': definitions[key]['properties']['value'],
                'probability': definitions[key]['properties']['probability'],
                'results': payloads[definitions[key]['properties']['function']['const']]}
            for key, _ in definitions['completesettledAnswer']['variants']}

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
    lines += ['    SCHEMAS = ' + literal(definitions) + '.freeze', '    CALL_TYPES = ' + literal(calls(definitions)) + '.freeze']
    lines += ['  end', '  class Client', *['    def self.choose_question(text, options:)', '      {choose: text, options: options}.freeze', '    end', '    def self.recognize_question(kinds:, **fields)', '      {recognize: {kinds: kinds, **fields}}.freeze', '    end', '    def self.relation(name, **fields)', '      {name: name, **fields}.freeze', '    end', '    def self.relate_question(relations:, fields: nil)', '      body = {relations: relations}', '      body[:fields] = fields unless fields.nil?', '      {relate: body}.freeze', '    end'], '  end', 'end']
    return '\n'.join(lines) + '\n'


def rbs(definitions):
    from templates import ruby_rbs
    return ruby_rbs.render(definitions, name)
