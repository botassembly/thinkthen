"""Decode actual SQL composites back to the canonical fixture contract."""
import json
from pathlib import Path
from jsonschema import Draft202012Validator

ROOT = Path(__file__).resolve().parents[3]
SCHEMA = json.loads((ROOT / 'crates/thinkthen/src/public/results/complete.schema.json').read_text())
DEFINITIONS = SCHEMA['$defs']


def resolve(schema):
    if isinstance(schema, dict) and '$ref' in schema:
        return resolve(DEFINITIONS[schema['$ref'].split('/')[-1]])
    return schema


def valid(value, schema):
    schema = {**schema, '$defs': DEFINITIONS}
    return Draft202012Validator(schema).is_valid(value)


def decode(value, schema):
    schema = resolve(schema)
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
            choice = resolve(choice)
            if common and isinstance(choice, dict):
                choice = {**choice, 'properties': {**common, **choice.get('properties', {})}}
            try:
                decoded = decode(value, choice)
            except (AssertionError, TypeError, AttributeError, KeyError):
                continue
            if valid(decoded, choice):
                return decoded
        raise AssertionError(('no canonical SQL alternative', value, schema))
    kinds = schema.get('type', [])
    if isinstance(kinds, str):
        kinds = [kinds]
    if isinstance(value, dict) and 'properties' not in schema:
        for kind in kinds:
            selected = kind + '_value'
            if selected in value and value[selected] is not None:
                return decode(value[selected], schema)
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
            out[member] = decode(held, child)
        return out
    if 'object' in kinds and isinstance(schema.get('additionalProperties'), dict):
        if isinstance(value, list):
            return {entry['name']: decode(entry['value'], schema['additionalProperties']) for entry in value}
        return {name: decode(child, schema['additionalProperties']) for name, child in value.items()}
    if 'array' in kinds and isinstance(value, list):
        return [decode(child, schema.get('items', {})) for child in value]
    return value


def native(rows, verb):
    roots = {'decide': 'completeDecide', 'choose': 'completeChoose', 'tag': 'completeTag',
             'score': 'completeScore', 'filter': 'completeFilter', 'rank': 'completeRank',
             'annotate': 'completeAnnotation', 'recognize': 'completeRecognition',
             'find': 'completeFind', 'relate': 'completeRelation'}
    completed = [row for row in rows if row['result'] is not None]
    if verb == 'relate':
        completed = list({json.dumps(row['result'], sort_keys=True): row for row in completed}.values())
    values = [decode(row['result'], DEFINITIONS[roots[verb]]) for row in completed]
    facts = next((decode(row['facts'], DEFINITIONS['completeFacts']) for row in rows if row['facts'] is not None), None)
    failure = next((decode(row['failure'], DEFINITIONS['completeError']) for row in rows if row['failure'] is not None), None)
    raw_events = list({json.dumps(event, sort_keys=True): event for row in rows for event in row.get('observations', [])}.values())
    events = [decode(event, DEFINITIONS['completesessionObservation']) for event in raw_events]
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
