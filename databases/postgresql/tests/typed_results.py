"""Decode actual SQL composites back to the canonical fixture contract."""
import json
import sys
from pathlib import Path
from jsonschema import Draft202012Validator

ROOT = Path(__file__).resolve().parents[3]
SCHEMA = json.loads((ROOT / 'crates/thinkthen/src/public/results/complete.schema.json').read_text())
DEFINITIONS = SCHEMA['$defs']
sys.path.insert(0, str(ROOT / 'sdlc/generators/results/templates'))
from postgresql import sql_schema
SQL_DEFINITIONS = sql_schema(SCHEMA)['$defs']


def resolve(schema, definitions=DEFINITIONS):
    if isinstance(schema, dict) and '$ref' in schema:
        return resolve(definitions[schema['$ref'].split('/')[-1]], definitions)
    return schema


def valid(value, schema, definitions=DEFINITIONS):
    schema = {**schema, '$defs': definitions}
    return Draft202012Validator(schema).is_valid(value)


def decode(value, schema, definitions=DEFINITIONS):
    schema = resolve(schema, definitions)
    if not isinstance(schema, dict) or not schema:
        return value
    if isinstance(value, dict) and value and all(key.endswith('_value') for key in value):
        selected = [child for child in value.values() if child is not None]
        if len(selected) == 1:
            value = selected[0]
        elif not selected:
            value = None
    choices = schema.get('oneOf', schema.get('anyOf'))
    if choices and not all(set(choice) <= {'required', 'not'} for choice in choices):
        common = schema.get('properties', {})
        for choice in choices:
            choice = resolve(choice, definitions)
            if common and isinstance(choice, dict):
                choice = {**choice, 'properties': {**common, **choice.get('properties', {})}}
            try:
                decoded = decode(value, choice, definitions)
            except (AssertionError, TypeError, AttributeError, KeyError):
                continue
            if valid(decoded, choice, definitions):
                return decoded
        raise AssertionError(('no canonical SQL alternative', value, schema))
    kinds = schema.get('type', [])
    if isinstance(kinds, str):
        kinds = [kinds]
    if isinstance(value, dict) and 'properties' not in schema:
        for kind in kinds:
            selected = kind + '_value'
            if selected in value and value[selected] is not None:
                return decode(value[selected], schema, definitions)
    if value is None:
        return None
    if 'properties' in schema:
        assert isinstance(value, dict), (value, schema)
        fields = schema['properties']
        required = schema.get('required', [])
        out = {}
        for member, child in fields.items():
            if member not in value:
                continue
            held = value[member]
            present = value.get(member + '_present')
            if present is False:
                continue
            if held is None and member not in required and present is not True:
                continue
            out[member] = decode(held, child, definitions)
        return out
    if 'object' in kinds and isinstance(schema.get('additionalProperties'), dict):
        if isinstance(value, list):
            return {entry['name']: decode(entry['value'], schema['additionalProperties'], definitions) for entry in value}
        return {name: decode(child, schema['additionalProperties'], definitions) for name, child in value.items()}
    if 'array' in kinds and isinstance(value, list):
        return [decode(child, schema.get('items', {}), definitions) for child in value]
    return value


def native(rows, verb):
    if not rows:
        return {'empty_sql_rows': [], 'details_rows': []}
    roots = {'decide': 'completeDecide', 'choose': 'completeChoose', 'tag': 'completeTag',
             'score': 'completeScore', 'filter': 'completeFilter', 'rank': 'completeRank',
             'annotate': 'completeAnnotation', 'recognize': 'completeRecognition',
             'find': 'completeFind', 'relate': 'completeRelation'}
    completed = [row for row in rows if row['result'] is not None]
    if verb == 'rank':
        completed.sort(key=lambda row: row['result']['value'])
    schema = DEFINITIONS[roots[verb]]
    values = []
    retained_sql = None
    for row in completed:
        result = row['result']
        metadata = json.dumps({key: child for key, child in result.items() if key != 'value'}, sort_keys=True) if verb == 'relate' else None
        if values and verb == 'relate' and metadata == retained_sql:
            # SQL repeats the aggregate answer on every edge. JSON equality
            # preserves boolean versus number types and presence flags.
            values.append({**values[0], 'value': decode(result['value'], schema['properties']['value'])})
        else:
            values.append(decode(result, schema))
        if len(values) == 1:
            retained_sql = metadata
    if verb == 'relate' and values:
        retained = {key: child for key, child in values[0].items() if key != 'value'}
        assert all({key: child for key, child in value.items() if key != 'value'} == retained for value in values), values
        assert all(len(value['value']) == 1 for value in values), values
        values = [{**retained, 'value': [value['value'][0] for value in values]}]
    facts = next((decode(row['facts'], DEFINITIONS['completeFacts']) for row in rows if row['facts'] is not None), None)
    failure = next((decode(row['failure'], DEFINITIONS['completeError']) for row in rows if row['failure'] is not None), None)
    raw_events = list({json.dumps(event, sort_keys=True): event for row in rows for event in row.get('observations', [])}.values())
    events = [decode(event, SQL_DEFINITIONS['completesessionObservation'], SQL_DEFINITIONS) for event in raw_events]
    observations = []
    for raw_event, event in zip(raw_events, events):
        if event['kind'] != 'question':
            observations.append(event)
            continue
        detail = event['detail']
        located = {source['index']: source['source'] for source in detail.get('input_sources', [])}
        inputs = []
        for index, original in enumerate(detail['inputs']):
            item = {'input': original}
            if index in located:
                item['source'] = located[index]
            if isinstance(original, dict) and 'images' in original:
                item['images'] = original['images']
                item['input'] = original.get('text')
            inputs.append(item)
        if raw_event.get('inputs') is not None:
            inputs = [{key: value for key, value in item.items() if key != 'input_present' and (value is not None or key == 'input')} for item in raw_event['inputs']]
        observations.append({**event, 'inputs': inputs, 'answer_id': detail.get('answer_id'),
                             'failure_id': detail.get('failure_id'), 'question_sources': detail['question_sources'],
                             'observations': detail['observations'], 'raw_pick': detail.get('raw_pick'),
                             'author': detail['question']})
    result = {'native': {}, 'observations': observations}
    if facts is not None:
        result['native']['facts'] = facts
    if failure is not None:
        result['native'].update(error=failure, completed=values)
    else:
        result['native']['value'] = values[0] if verb in ('find', 'relate') else values
    if verb == 'find' and rows and failure is None:
        result['selection'] = values[0].get('index')
    elif verb != 'relate':
        result['ordinals'] = [row['ordinal'] for row in completed if row.get('ordinal') is not None]
    Draft202012Validator(SCHEMA).validate(result['native'])
    return result
