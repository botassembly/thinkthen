"""Python result carriers and native conversion from the shared result graph."""
import json

ROOTS = ('completesessionPacket',)


def name(key):
    return 'Native' + ''.join(part[:1].upper() + part[1:] for part in
                            key.removeprefix('complete').split('_'))


def expression(source, value='value'):
    if not isinstance(source, dict):
        return f'plain(py, {value})'
    if '$ref' in source:
        return f'convert(py, {json.dumps(source["$ref"].removeprefix("#/$defs/"))}, {value})'
    alternatives = source.get('anyOf', source.get('oneOf', []))
    refs = [item for item in alternatives if '$ref' in item]
    if len(refs) == 1:
        return f'if {value}.is_null() {{ plain(py, {value}) }} else {{ {expression(refs[0], value)} }}'
    kinds = source.get('type', [])
    if not isinstance(kinds, list):
        kinds = [kinds]
    if 'array' in kinds:
        return f'array(py, {value}, |value| {expression(source.get("items", {}))})'
    if 'object' in kinds and isinstance(source.get('additionalProperties'), dict):
        return f'mapping(py, {value}, |value| {expression(source["additionalProperties"])})'
    return f'plain(py, {value})'


def predicate(tag):
    mode, member, value = tag
    if mode == 'literal':
        return f'value.get({json.dumps(member)}).and_then(Value::as_str) == Some({json.dumps(value)})'
    if mode == 'member':
        return f'value.get({json.dumps(member)}).is_some()'
    if mode == 'literals':
        return ' && '.join(predicate(('literal', member, item)) for member, item in value.items())
    if mode == 'structure':
        return ' && '.join([f'value.get({json.dumps(key)}).is_some()' for key in value['required']] +
                           [f'value.get({json.dumps(key)}).is_none()' for key in value['excluded']])
    raise ValueError(mode)


def render(definitions):
    lines = ['// Generated from the shared Rust result graph; do not edit.',
             'use pyo3::prelude::*;', 'use serde_json::Value;',
             'use super::native_result::{array, mapping, object, plain};',
             '#[expect(clippy::too_many_lines, reason = "dispatch generated from the shared result graph")]',
             'pub(crate) fn convert(py: Python<\'_>, kind: &str, value: &Value) -> PyResult<Py<PyAny>> {',
             'match kind {']
    for key, source in definitions.items():
        lines.append(json.dumps(key) + ' => {')
        if 'variants' in source:
            for child, tag in source['variants']:
                lines.append(f'if {predicate(tag)} {{ return convert(py, {json.dumps(child)}, value); }}')
            lines.append('Err(crate::defect(py, "native result has no generated alternative"))')
        elif 'properties' in source:
            lines.append(f'object(py, {json.dumps(name(key))}, {json.dumps(key)}, value)')
        elif 'primitive_schemas' in source:
            item = source['primitive_schemas'].get('object')
            lines.append(f'if value.is_object() {{ {expression(item)} }} else {{ plain(py, value) }}'
                         if item else 'plain(py, value)')
        else:
            lines.append(expression(source))
        lines.append('},')
    lines += ['_ => Err(crate::defect(py, "unknown generated Python result type")),', '}', '}',
              '#[expect(clippy::too_many_lines, clippy::cognitive_complexity, reason = "fields generated from the shared result graph")]',
              'pub(crate) fn field(py: Python<\'_>, kind: &str, member: &str, value: &Value) -> PyResult<Py<PyAny>> {',
              'match (kind, member) {']
    for key, source in definitions.items():
        for member, field in source.get('properties', {}).items():
            lines.append(f'({json.dumps(key)}, {json.dumps(member)}) => {expression(field)},')
    lines += ['_ => plain(py, value),', '}', '}',
              'pub(crate) fn has_value(kind: &str) -> bool {', 'matches!(kind,']
    lines.append(' | '.join(json.dumps(key) for key, source in definitions.items()
                           if 'value' in source.get('properties', {})))
    lines += [')', '}']
    return '\n'.join(lines) + '\n'


def annotation(source):
    if not isinstance(source, dict):
        return 'Any'
    if '$ref' in source:
        return name(source['$ref'].removeprefix('#/$defs/'))
    if 'const' in source:
        return 'Literal[' + repr(source['const']) + ']'
    if 'enum' in source:
        return 'Literal[' + ', '.join(repr(v) for v in source['enum']) + ']'
    alternatives = source.get('anyOf', source.get('oneOf'))
    if alternatives:
        return 'Union[' + ', '.join(annotation(v) for v in alternatives) + ']'
    kind = source.get('type')
    if isinstance(kind, list):
        return 'Union[' + ', '.join(annotation({**source, 'type': v}) for v in kind) + ']'
    if kind == 'array':
        return 'list[' + annotation(source.get('items', {})) + ']'
    if kind == 'object':
        return 'dict[str, ' + annotation(source.get('additionalProperties', {})) + ']'
    return {'null': 'None', 'boolean': 'bool', 'string': 'str', 'integer': 'int', 'number': 'float'}.get(kind, 'Any')


def types(definitions, stubs=False):
    lines = ['# Generated from the shared Rust result graph; do not edit.',
             'from ._thinkthen import _NativeResult',
             'from typing import Any, Literal, Union']
    if stubs:
        lines = [lines[0], 'from typing import Any, Literal, Union',
                 'class _NativeResult:',
                 '    def __getitem__(self, key: str) -> Any: ...',
                 '    def __contains__(self, key: str) -> bool: ...',
                 '    def __len__(self) -> int: ...',
                 '    def __bool__(self) -> bool: ...',
                 '    def keys(self) -> list[str]: ...',
                 '    def to_dict(self) -> dict[str, Any]: ...']
    for key, source in definitions.items():
        if 'properties' in source:
            lines += ['', f'class {name(key)}(_NativeResult):', '    __slots__ = ()']
            if stubs:
                for member, field in source['properties'].items():
                    if member.isidentifier():
                        lines += ['    @property', f'    def {member}(self) -> {annotation(field)}: ...']
    for key, source in definitions.items():
        if 'properties' not in source:
            if 'variants' in source:
                value = 'Union[' + ', '.join(name(child) for child, _ in source['variants']) + ']'
            else:
                value = ('Union[' + ', '.join(annotation(v) for v in source['primitive_schemas'].values()) + ']'
                         if 'primitive_schemas' in source else annotation(source))
            lines.append(f'{name(key)} = {value}')
    return '\n'.join(lines) + '\n'
