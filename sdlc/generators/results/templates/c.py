"""Complete immutable C packet views from the shared Rust-derived schema IR."""
import json
import re


def ident(value):
    words = re.findall(r'[A-Z]?[a-z]+|[A-Z]+(?![a-z])|[0-9]+', value.removeprefix('complete'))
    return 'thinkthen_complete_' + '_'.join(word.lower() for word in words) + '_v1'


def field(value):
    if value in ('true', 'false'):
        return value + '_'
    return 'r#' + value if value in ('type', 'match', 'ref', 'in', 'loop', 'mod', 'move', 'true', 'false') else value


def empty(typ):
    if typ.startswith('*const '):
        return 'std::ptr::null()'
    if typ == 'f64':
        return '0.0'
    if typ in ('u32', 'u64', 'i64', 'usize'):
        return '0'
    return typ + ' { data: std::ptr::null(), len: 0 }'


def literal(value):
    return json.dumps(value, ensure_ascii=False)


def normalized(schema):
    if schema is True:
        return {}
    schema = dict(schema)
    if 'variants' in schema or 'primitive_variants' in schema:
        return schema
    if 'enum' in schema:
        return {**schema, 'enum': schema['enum']}
    variants = schema.get('oneOf', schema.get('anyOf', []))
    if variants and all(item.get('type') == 'string' and 'const' in item for item in variants):
        return {'enum': [item['const'] for item in variants]}
    if variants:
        if any(item is True or item == {} for item in variants):
            return {}
        nonnull = [item for item in variants if item.get('type') != 'null']
        if len(nonnull) == 1:
            return normalized(nonnull[0])
        kinds = [item.get('type') for item in nonnull]
        if len(set(kinds)) != len(kinds):
            raise ValueError(f'overlapping C primitive alternatives: {schema}')
        return {'primitive_variants': kinds, 'primitive_schemas': dict(zip(kinds, nonnull))}
    if isinstance(schema.get('type'), list):
        kinds = [kind for kind in schema['type'] if kind != 'null']
        if len(kinds) == 1:
            schema['type'] = kinds[0]
        else:
            return {'primitive_variants': kinds,
                    'primitive_schemas': {kind: {'type': kind} for kind in kinds}}
    return schema


def nullable(schema, definitions):
    if schema is True or schema == {}:
        return True
    if '$ref' in schema:
        return nullable(definitions[schema['$ref'].removeprefix('#/$defs/')], definitions)
    kinds = schema.get('type', [])
    return (schema.get('nullable_json', False) or kinds == 'null' or
            isinstance(kinds, list) and 'null' in kinds or
            any(item is True or item == {} or item.get('type') == 'null'
                for item in schema.get('anyOf', [])))


class Target:
    def __init__(self, definitions):
        self.definitions = definitions
        self.output = []
        self.names = set()
        self.schemas = {}

    def structure(self, name, members):
        if name in self.names:
            raise ValueError(f'duplicate C declaration {name}')
        self.names.add(name)
        self.output.append('#[repr(C)]\n#[derive(Clone, Copy, Debug)]\npub struct ' + name + ' {\n' +
                           '\n'.join('    pub ' + field(key) + ': ' + typ + ',' for key, typ in members) + '\n}')

    def read(self, name, body):
        self.output.append('pub(crate) fn read_' + name + '(node: &Node, ' + ('store' if 'store' in body else '_store') + ': &mut Storage) -> Result<' +
                           name + ', ErrorKind> {\n' + body + '\n}')

    def shape(self, schema, path, node):
        schema = normalized(schema)
        if '$ref' in schema:
            name = ident(schema['$ref'].removeprefix('#/$defs/'))
            return '*const ' + name, 'read_' + name + '(' + node + ', store).map(|value| store.hold(value))?' 
        if schema == {} or schema.get('type') == 'null':
            return '*const thinkthen_complete_json_v1', 'read_json(' + node + ', store)?'
        if 'enum' in schema or 'variants' in schema or 'primitive_variants' in schema or 'properties' in schema:
            name = ident(path)
            self.definition(path, schema)
            return '*const ' + name, 'read_' + name + '(' + node + ', store).map(|value| store.hold(value))?' 
        kind = schema.get('type')
        if kind == 'string' or isinstance(schema.get('const'), str):
            return 'thinkthen_complete_utf8_v1', 'store.text(' + node + '.text()?)'
        if kind == 'boolean':
            return 'u32', node + '.boolean()?'
        if kind in ('integer', 'number'):
            typ = 'f64' if kind == 'number' else ('u64' if schema.get('minimum', -1) >= 0 or schema.get('format', '').startswith('uint') else 'i64')
            return typ, node + '.number()?'
        name = ident(path)
        if kind == 'array':
            typ, read = self.shape(schema['items'], path + '_item', 'item')
            self.structure(name, [('data', '*const ' + typ), ('len', 'usize')])
            self.read(name, '    let mut values = Vec::new();\n    for item in node.array()? {\n        values.push(' + read + ');\n    }\n    let (data, len) = store.slice(values);\n    Ok(' + name + ' { data, len })')
        elif kind == 'object' and 'additionalProperties' in schema:
            typ, read = self.shape(schema['additionalProperties'], path + '_entry_value', 'item')
            entry = ident(path + '_entry')
            self.structure(entry, [('name', 'thinkthen_complete_utf8_v1'), ('value', typ)])
            self.structure(name, [('data', '*const ' + entry), ('len', 'usize')])
            self.read(name, '    let mut values = Vec::new();\n    for (key, item) in node.object()? {\n        values.push(' + entry + ' { name: store.text(key), value: ' + read + ' });\n    }\n    let (data, len) = store.slice(values);\n    Ok(' + name + ' { data, len })')
        else:
            raise ValueError(f'unsupported C transport shape: {schema}')
        return name, 'read_' + name + '(' + node + ', store)?'

    def union(self, key, alternatives):
        name = ident(key)
        union = ident(key + '_data')
        arms = []
        reads = []
        for index, (arm, predicate, typ, read) in enumerate(alternatives, 1):
            tag = name.removesuffix('_v1').upper() + '_' + arm.upper() + '_V1'
            self.output.append('pub const ' + tag + ': u32 = ' + str(index) + ';')
            arms.append('    pub ' + field(arm) + ': ' + typ + ',')
            reads.append('    if ' + predicate + ' {\n        let value = ' + read + ';\n        return Ok(' + name + ' { kind: ' + tag + ', data: ' + union + ' { ' + field(arm) + ': value } });\n    }')
        self.names.add(union)
        self.output.append('#[repr(C)]\n#[derive(Clone, Copy)]\npub union ' + union + ' {\n' + '\n'.join(arms) + '\n}\nimpl std::fmt::Debug for ' + union + ' {\n    fn fmt(&self, f: &mut std::fmt::Formatter<\'_>) -> std::fmt::Result { f.write_str("union payload") }\n}')
        self.structure(name, [('kind', 'u32'), ('data', union)])
        self.read(name, '\n'.join(reads) + '\n    Err(ErrorKind::Defect)')

    def definition(self, key, source):
        name = ident(key)
        source = normalized(source)
        if name in self.schemas:
            if self.schemas[name] != source:
                raise ValueError(f"conflicting C type name {name}")
            return
        if name in self.names:
            raise ValueError(f"conflicting C declaration {name}")
        self.schemas[name] = source
        if 'variants' in source:
            alternatives = []
            for child, (mode, member, value) in source['variants']:
                if mode == 'literal':
                    check = 'node.is_member(' + literal(member) + ', ' + literal(value) + ')'
                elif mode == 'literals':
                    check = ' && '.join('node.is_member(' + literal(k) + ', ' + literal(v) + ')' for k, v in value.items())
                elif mode == 'member':
                    check = 'node.member(' + literal(member) + ').is_some()'
                elif mode == 'structure':
                    check = ' && '.join(['node.member(' + literal(k) + ').is_some()' for k in value['required']] +
                                        ['node.member(' + literal(k) + ').is_none()' for k in value['excluded']])
                else:
                    raise ValueError(mode)
                alternatives.append((child.removeprefix(key + '_'), check, '*const ' + ident(child),
                                     'read_' + ident(child) + '(node, store).map(|value| store.hold(value))?' ))
            self.union(key, alternatives)
        elif 'primitive_variants' in source:
            alternatives = []
            for kind in source['primitive_variants']:
                typ, read = ('u32', '0') if kind == 'null' else self.shape(source.get('primitive_schemas', {}).get(kind, {'type': kind}), key + '_' + kind, 'node')
                alternatives.append((kind, 'node.kind() == ' + literal('number' if kind == 'integer' else kind), typ, read))
            self.union(key, alternatives)
        elif 'enum' in source:
            self.structure(name, [('kind', 'u32')])
            lines = ['    let kind = match node.text()? {']
            for index, value in enumerate(source['enum'], 1):
                tag = name.removesuffix('_v1').upper() + '_' + re.sub(r'[^A-Za-z0-9]+', '_', value).strip('_').upper() + '_V1'
                self.output.append('pub const ' + tag + ': u32 = ' + str(index) + ';')
                lines.append('        ' + literal(value) + ' => ' + tag + ',')
            lines += ['        _ => return Err(ErrorKind::Defect),', '    };', '    Ok(' + name + ' { kind })']
            self.read(name, '\n'.join(lines))
        elif 'properties' in source:
            members, lines = [], []
            required = source.get('required', [])
            for member, schema in source['properties'].items():
                path = key + '_field_' + member
                typ, read = self.shape(schema, path, 'value')
                optional = member not in required
                null = optional or nullable(schema, self.definitions)
                if optional or null:
                    presence = ident(path + '_presence')
                    self.structure(presence, [('presence', 'u32'), ('value', typ)])
                    members.append((member, presence))
                    lines.append('        ' + field(member) + ': match node.member(' + literal(member) + ') {')
                    if optional:
                        lines.append('            None => ' + presence + ' { presence: THINKTHEN_COMPLETE_PRESENCE_MISSING_V1, value: ' + empty(typ) + ' },')
                    else:
                        lines.append('            None => return Err(ErrorKind::Defect),')
                    if null:
                        lines.append('            Some(value) if value.kind() == "null" => ' + presence + ' { presence: THINKTHEN_COMPLETE_PRESENCE_NULL_V1, value: ' + empty(typ) + ' },')
                    lines.append('            Some(value) => ' + presence + ' { presence: THINKTHEN_COMPLETE_PRESENCE_VALUE_V1, value: ' + read + ' },\n        },')
                else:
                    members.append((member, typ))
                    lines.append('        ' + field(member) + ': { let value = node.required(' + literal(member) + ')?; ' + read + ' },')
            members.append(('extensions', 'thinkthen_complete_extensions_v1'))
            self.structure(name, members)
            known = '&[' + ', '.join(literal(k) for k in source['properties']) + ']'
            self.read(name, '    Ok(' + name + ' {\n' + '\n'.join(lines) + '\n        extensions: store.extensions(node, ' + known + ')?,\n    })')
        else:
            typ, read = self.shape(source, key + '_value', 'node')
            self.structure(name, [('value', typ)])
            self.read(name, '    let value = ' + read + ';\n    Ok(' + name + ' { value })')


def render(definitions):
    target = Target(definitions)
    for key, source in definitions.items():
        target.definition(key, source)
    return '''// Generated from Rust-derived complete result schema; do not edit.
#![allow(non_camel_case_types, missing_docs, clippy::too_many_lines, reason = "generated versioned C schema objects and tagged alternatives")]
use super::views::{Node, Storage, read_json};
use super::views::{thinkthen_complete_utf8_v1, thinkthen_complete_json_v1, thinkthen_complete_extensions_v1};
use thinkthen::ErrorKind;
pub const THINKTHEN_COMPLETE_PRESENCE_MISSING_V1: u32 = 0;
pub const THINKTHEN_COMPLETE_PRESENCE_NULL_V1: u32 = 1;
pub const THINKTHEN_COMPLETE_PRESENCE_VALUE_V1: u32 = 2;

''' + '\n\n'.join(target.output) + '\n'
