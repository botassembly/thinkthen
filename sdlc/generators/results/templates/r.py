"""Native R representation conversions over the shared result graph."""
import json

ROOTS = ('completeplan', 'completesessionPacket', 'completeFacts', 'completeCallError', 'completeAtomic_DecideValue',
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
    return ('\n'.join(lines) + '\n' + answer_helpers(definitions)).replace('|value| plain(value)', 'plain')


def inputs(schema):
    """Derive explicit R controls and authored recognition builders."""
    definitions = schema['$defs']
    lines = ['# Generated from the canonical request schema; do not edit.']
    for name, key in [('tt_options', 'RequestOptions'), ('.tt_engine_settings', 'EngineSettings')]:
        fields = [field for field in definitions[key]['properties'] if field != 'details']
        lines += [name + ' <- function(' + ', '.join(field + ' = NULL' for field in fields) + ') {',
                  '  values <- list(' + ', '.join(field + ' = ' + field for field in fields) + ')',
                  ('  values[c(' + ', '.join('!' + 'missing(' + field + ')' for field in fields) + ')]' if name == 'tt_options' else '  values[!vapply(values, is.null, TRUE)]'), '}']
    for alternative in definitions['RequestFunction']['oneOf']:
        function = alternative['const']
        lines.append('tt_' + function + ' <- function(question, input, options = tt_options(), deadline_ms = NULL, completion = NULL) .tt_named_call("' + function + '", question, input, options, deadline_ms, completion)')
    order = ['base_url', 'model', 'throttle', 'max_requests', 'max_requests_total', 'max_request_bytes', 'cache', 'timeout', 'max_retries', 'record', 'replay', 'profile', 'batch']
    extra = [name for name in definitions['EngineSettings']['properties'] if name not in order]
    fields = order + extra
    lines += ['tt_engine <- function(' + ', '.join(name + ' = ' + ('FALSE' if name == 'refresh_cache' else 'NULL') for name in order) + ', ..., ' + ', '.join(name + ' = ' + ('FALSE' if name == 'refresh_cache' else 'NULL') for name in extra) + ') {',
              '  if (length(list(...))) .tt_unknown(...)',
              '  settings <- .tt_engine_settings(' + ', '.join(name + ' = ' + name for name in fields) + ')',
              '  .tt_call(tt_engine_set(if (length(settings)) .tt_json(settings) else "{}"))',
              '  invisible(NULL)', '}']
    recognition = next(value['properties']['recognize'] for value in definitions['RequestDefinition']['anyOf']
                       if 'recognize' in value.get('properties', {}))
    for function, fields in [('recognize', recognition['properties']),
                             ('relate', definitions['Authored_relate']['properties']['relate']['properties'])]:
        names = list(fields)
        lines += ['tt_' + function + '_question <- function(' + ', '.join(field + ' = NULL' for field in names) + ') {',
                  '  values <- list(' + ', '.join(field + ' = ' + field for field in names) + ')',
                  '  tt_question(list(version = 1L, ' + function + ' = values[!vapply(values, is.null, TRUE)]))', '}']
    return '\n'.join(lines) + '\n'


def answer_helpers(definitions):
    from host_answers import call_results
    def at(path, value='row'):
        return 'member(' + value + ', &[' + ', '.join(json.dumps(x) for x in path) + '])'
    lines = [
        "fn member<'a>(mut value: &'a Value, path: &[&str]) -> Option<&'a Value> {",
        'for key in path { value = value.get(*key)?; } Some(value)', '}',
        '#[expect(clippy::too_many_lines, reason = "mechanical projection generated from the shared result graph")]',
        'pub(crate) fn call_value(function: &str, rows: &Value, scalar: bool) -> Crossed<Robj> {',
        'match function {']
    contracts = call_results(definitions)
    helpers = []
    for function, contract in contracts.items():
        payload = contract['receipt_payload']
        key = payload.get('items', payload)['$ref'].split('/')[-1]
        projection = contract['value_projection']
        read = expression(definitions[key]['properties'][projection],
                          'row.get(' + json.dumps(projection) + ').ok_or_else(|| crate::defect("native result has no projected value"))?')
        if payload.get('type') == 'array':
            body = 'array(rows, |row| ' + read + ')'
            if contract['call_shape'] == 'input':
                body = 'if scalar { match rows.as_array().and_then(|rows| rows.first()) { Some(row) => ' + read + ', None => plain(&Value::Null) } } else { ' + body + ' }'
        else:
            body = '{ let row = rows; ' + read + ' }'
        lines.append(json.dumps(function) + ' => ' + body + ',')
    lines += ['_ => Err(crate::defect("native result has no named value")),', '}', '}',
              'pub(crate) fn call_probability(function: &str, rows: &Value, observations: &[Value], scalar: bool) -> Crossed<Robj> {', 'match function {']
    for function, contract in contracts.items():
        mode = contract['probability_mode']
        if mode not in ('yes', 'picked'):
            body = 'plain(&Value::Null)'
        else:
            odds = at(contract['probability_path'])
            if mode == 'picked':
                odds += '.and_then(|values| ' + at(contract['pick_path']) + '.and_then(Value::as_str).and_then(|pick| values.get(pick)))'
            odds += '.filter(|value| !value.is_null())'
            resolution = contract.get('resolution')
            if resolution:
                guards = ' && '.join(at(path, 'observation') + '.and_then(Value::as_str) == Some(' + json.dumps(literal) + ')' for path, literal in resolution['guards'])
                guards += ' && ' + at(resolution['index_path'], 'observation') + '.and_then(Value::as_u64) == row.get("index").and_then(Value::as_u64)'
                guards += ' && ' + at(resolution['path'], 'observation') + '.is_some_and(|value| !value.is_null())'
                odds = 'if observations.iter().any(|observation| ' + guards + ') { ' + odds + ' } else { None }'
            elif contract.get('selection_path'):
                odds = 'if ' + at(contract['selection_path']) + '.is_some_and(|value| !value.is_null()) { ' + odds + ' } else { None }'
            read = 'probability_' + function + '(row, observations)'
            helper = ['fn probability_' + function + '(row: &Value, ' + ('observations' if resolution else '_observations') + ': &[Value]) -> Crossed<Robj> {',
                      'plain((' + odds + ').unwrap_or(&Value::Null))', '}']
            helpers.extend(helper)
            if contract['receipt_payload'].get('type') == 'array':
                body = 'array(rows, |row| ' + read + ')'
                if contract['call_shape'] == 'input':
                    body = 'if scalar { match rows.as_array().and_then(|rows| rows.first()) { Some(row) => ' + read + ', None => plain(&Value::Null) } } else { ' + body + ' }'
            else:
                body = '{ let row = rows; ' + read + ' }'
        lines.append(json.dumps(function) + ' => ' + body + ',')
    lines += ['_ => Err(crate::defect("native result has no named probability")),', '}', '}']
    return '\n'.join(lines + helpers) + '\n'
