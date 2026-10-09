"""Native R representation conversions over the shared result graph."""
import json

ROOTS = ('completeFacts', 'completeCallError', 'completeAtomic_DecideValue',
         'completeAtomic_Nullable_string', 'completeAtomic_Array_of_string',
         'completeAtomic_double', 'completeAtomic_boolean', 'completeAtomic_NonZeroUsize',
         'completeFind', 'completeAnnotation', 'completeRecognition', 'completeRelation')

# R naming compatibility belongs to the target, never to the result schema.
NAMES = {'completeImage': 'NativeImage',
         'completeAtomic_DecideValue': 'DecideResult',
         'completeAtomic_Nullable_string': 'ChooseResult',
         'completeAtomic_Array_of_string': 'TagResult', 'completeAtomic_double': 'ScoreResult',
         'completeAtomic_boolean': 'FilterResult', 'completeAtomic_NonZeroUsize': 'RankResult',
         'completeAnnotation': 'AnnotateResult', 'completeRecognition': 'RecognizeResult',
         'completeRelation': 'RelateResult', 'completeFind': 'FindResult',
         'completeanswer_yes_no': 'YesNo', 'completeanswer_choice': 'Choice',
         'completeanswer_tag': 'Tags', 'completeanswer_score': 'Score',
         'completeObservation_observation_id': 'Observed',
         'completeObservation_failure_id': 'FailedObservation',
         'completeAnnotationMember_answer_id': 'AnnotationSuccess',
         'completeAnnotationMember_failure_id': 'AnnotationFailure',
         'completeRelationMember_answer_id': 'RelationSuccess',
         'completeRelationMember_failure_id': 'RelationFailure'}


def name(key):
    if key in NAMES:
        return NAMES[key]
    if key.startswith('completeReadableQuestion_'):
        return key.rsplit('_', 1)[1].title() + 'Question'
    short = key.removeprefix('complete')
    return short[:1].upper() + short[1:]


def expression(source, value='value'):
    if not isinstance(source, dict):
        return f'plain({value})'
    if source.get('pattern') == '^[0-9a-f]{64}$':
        return f'tagged(plain({value})?, "Digest", "thinkthen_identity")'
    if '$ref' in source:
        return f'convert({json.dumps(source["$ref"].removeprefix("#/$defs/"))}, {value})'
    alternatives = source.get('anyOf', source.get('oneOf', []))
    refs = [item for item in alternatives if '$ref' in item]
    if len(refs) == 1:
        return f'if {value}.is_null() {{ plain({value}) }} else {{ {expression(refs[0], value)} }}'
    kinds = source.get('type', [])
    if not isinstance(kinds, list):
        kinds = [kinds]
    if 'array' in kinds:
        return f'array({value}, |value| {expression(source.get("items", {}))})'
    if 'object' in kinds and isinstance(source.get('additionalProperties'), dict):
        return f'mapping({value}, |value| {expression(source["additionalProperties"])})'
    return f'plain({value})'


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
             'use extendr_api::prelude::*;', 'use serde_json::Value;',
             'use super::native_results::{array, mapping, object, plain, tagged};',
             'use crate::calls::Crossed;',
             '#[expect(clippy::too_many_lines, reason = "mechanical dispatch generated from the shared result graph")]',
             'pub(crate) fn convert(kind: &str, value: &Value) -> Crossed<Robj> {',
             'match kind {']
    for key, source in definitions.items():
        lines.append(json.dumps(key) + ' => {')
        if 'variants' in source:
            for child, tag in source['variants']:
                lines.append(f'if {predicate(tag)} {{ return convert({json.dumps(child)}, value); }}')
            lines.append('Err(crate::defect("native result has no generated alternative"))')
        elif 'properties' in source:
            fields = ',\n'.join(f'({json.dumps(member)}, {str(member in source.get("required", [])).lower()}, |value| {expression(field)})'
                                for member, field in source['properties'].items())
            lines.append(f'object(value, {json.dumps(name(key))}, &[{fields}])')
        elif source.get('type') == 'string' and source.get('pattern') == '^[0-9a-f]{64}$':
            lines.append(f'tagged(plain(value)?, {json.dumps(name(key))}, "thinkthen_identity")')
        elif 'primitive_schemas' in source:
            item = source['primitive_schemas'].get('object')
            if item:
                lines.append(f'if value.is_object() {{ {expression(item)} }} else {{ plain(value) }}')
            else:
                lines.append('plain(value)')
        else:
            lines.append(expression(source))
        lines.append('},')
    lines += ['_ => Err(crate::defect("unknown generated R result type")),', '}', '}']
    return ('\n'.join(lines) + '\n').replace('|value| plain(value)', 'plain')
