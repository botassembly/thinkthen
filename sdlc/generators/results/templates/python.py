"""Python result carriers and native conversion from the shared result graph."""
import json
from host_answers import call_results

ROOTS = ('completesessionPacket', 'completeplan')


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


def column_schema(source):
    """Describe column members once from the same native result graph."""
    if not isinstance(source, dict): return None
    if '$ref' in source:
        key = source['$ref'].removeprefix('#/$defs/')
        return ('ref', key)
    if 'properties' in source:
        keys = dict.fromkeys((*source.get('required', ()), *source['properties']))
        return {key: column_schema(source['properties'][key]) for key in keys}
    alternatives = source.get('anyOf', source.get('oneOf', []))
    choices = [column_schema(value) for value in alternatives if value.get('type') != 'null']
    if choices: return choices[0] if len(choices) == 1 or all(value == choices[0] for value in choices) else None
    kind = source.get('type')
    if isinstance(kind, list):
        kind = next((value for value in kind if value != 'null'), None)
    if kind == 'array': return [column_schema(source.get('items'))]
    return kind if kind in ('boolean', 'string', 'integer', 'number') else None


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
                 '    def _to_dict(self) -> dict[str, Any]: ...']
    if not stubs:
        lines += ['', '_COLUMN_SCHEMAS = ' + repr({key: column_schema(source) for key, source in definitions.items()}),
                  '_CALL_RESULTS = ' + repr(call_results(definitions))]
    for key, source in definitions.items():
        if 'properties' in source:
            lines += ['', f'class {name(key)}(_NativeResult):', '    __slots__ = ()']
            if not stubs:
                lines.append('    _column_schema = _COLUMN_SCHEMAS[' + repr(key) + ']')
                lines.append('    _column_required = ' + repr(tuple(source.get('required', ()))))
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


def option_types(schema):
    """Keep Python control declarations derived from native admission schemas."""
    definitions = schema['$defs']
    emitted = set()
    lines = ['from __future__ import annotations',
             'from os import PathLike',
             'from typing import Literal, TypedDict, TYPE_CHECKING, Union, _SpecialForm',
             'if TYPE_CHECKING:', '    from . import CancelToken',
             'Labels = list[str] | dict[str, str] | dict[str, object] | type[object] | tuple[object, ...] | _SpecialForm']

    def emit(key):
        if key in emitted:
            return
        emitted.add(key)
        source = definitions[key]

        def walk(value):
            if isinstance(value, dict):
                if '$ref' in value:
                    emit(value['$ref'].split('/')[-1])
                for child in value.values():
                    walk(child)
            elif isinstance(value, list):
                for child in value:
                    walk(child)
        walk(source)
        if 'properties' in source:
            required = source.get('required', ())
            if required:
                lines.append(f'class _{key}Required(TypedDict):')
                for field in required:
                    lines.append(f'    {field}: {request_annotation(source["properties"][field])}')
            base = f'_{key}Required' if required else 'TypedDict'
            lines.append(f'class {key}({base}, total=False):')
            optional = {field: value for field, value in source['properties'].items() if field not in required}
            if not optional:
                lines.append('    pass')
            for field, value in optional.items():
                if key == 'RequestOptions' and field == 'details': continue
                typed = request_annotation(value)
                if key == 'EngineSettings':
                    if field in ('record', 'replay', 'profile', 'cache'):
                        typed += ' | PathLike[str]'
                    typed += ' | None'
                lines.append(f'    {field}: {typed}')
        else:
            lines.append(f'{key} = {request_annotation(source)}')

    def request_annotation(source):
        if not isinstance(source, dict):
            return 'object'
        if '$ref' in source:
            return source['$ref'].split('/')[-1]
        typed = annotation(source).replace('Any', 'object')
        for key in definitions:
            typed = typed.replace(name(key), key)
        return typed

    emit('EngineSettings')
    emit('RequestOptions')
    lines += ['class CallOptions(RequestOptions, total=False):',
              '    token: CancelToken', '    on: str | list[str]',
              'class DecideOptions(CallOptions, total=False):',
              '    true: object', '    false: object',
              'class ChooseOptions(CallOptions, total=False):', '    options: Labels',
              'class TagOptions(CallOptions, total=False):', '    labels: Labels',
              'class ScoreOptions(CallOptions, total=False):',
              '    levels: Labels | list[float]',
              'class FilterOptions(DecideOptions): pass',
              'class RankOptions(DecideOptions): pass',
              'class FindOptions(CallOptions): pass',
              'class AnnotateOptions(CallOptions): pass',
              'class RelateOptions(CallOptions, total=False):',
              '    relations: dict[str, tuple[str, str]] | list[tuple[str, str, str]]',
              '    either: list[str]',
              'class RecognizeOptions(RelateOptions, total=False):',
              '    kinds: Labels', '    descriptions: dict[str, object]'
,
              '    instructions: object', '    entity_definition: object',
              'class PlanOptions(DecideOptions, ChooseOptions, TagOptions, ScoreOptions, RecognizeOptions): pass']
    return lines


def named_calls(schema):
    """Derive the named family and native control inventory from RequestFunction."""
    definitions = schema['$defs']
    controls = [field for field in definitions['RequestOptions']['properties'] if field != 'details']
    lines = ['# Generated from the canonical request schema; do not edit.',
             *option_types(schema), 'MISSING = object()', 'class NamedCalls:', '    __slots__ = ()']
    settings = list(definitions['EngineSettings']['properties'])
    lines += ['    def __init__(self, *, ' + ', '.join(field + '=None' for field in settings) + '):',
              '        self._configure(dict(' + ', '.join(field + '=' + field for field in settings) + '))']
    inline = {'decide': ('true', 'false'), 'choose': ('options',),
              'tag': ('labels',), 'score': ('levels',),
              'filter': ('true', 'false'), 'rank': ('true', 'false'),
              'recognize': ('kinds', 'relations', 'either', 'descriptions', 'instructions', 'entity_definition'),
              'relate': ('relations', 'either')}
    for alternative in definitions['RequestFunction']['oneOf']:
        function = alternative['const']
        recognition = function in ('recognize', 'relate')
        names = list(dict.fromkeys([*controls, 'token', 'on', *inline.get(function, ())]))
        parameters = ', '.join(name + '=MISSING' for name in names)
        value = {'filter': 'records', 'rank': 'records', 'find': 'units', 'annotate': 'records', 'relate': 'entities'}.get(function, 'text')
        question_arg = 'questions' if function == 'annotate' else 'question'
        arguments = value + ', ask=None' if recognition else question_arg + ', ' + value
        values = ', '.join(name + '=' + name for name in names)
        question = 'ask' if recognition else question_arg
        lines += [f'    def {function}(self, {arguments}, *, {parameters}):',
                  f'        controls = dict({values})',
                  f'        return self._named({function!r}, {question}, {value}, **{{key: value for key, value in controls.items() if value is not MISSING}})']
    return '\n'.join(lines) + '\n'
