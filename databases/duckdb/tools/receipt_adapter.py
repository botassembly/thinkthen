"""Project actual typed SQL receipts into shared native-result assertions."""
import json
from pathlib import Path
from receipt_types import ROOTS, model_for, name

ROOT = Path(__file__).resolve().parents[3]
MODEL = model_for(json.loads((ROOT / 'crates/thinkthen/src/public/results/complete.schema.json').read_text()))
for root in ROOTS:
    MODEL.shape({'$ref': '#/$defs/' + root}, root)
RESULTS = dict(zip(('decide', 'choose', 'tag', 'score', 'filter', 'rank', 'annotate', 'recognize', 'find', 'relate'), ROOTS))


def required_present(value, field, schema):
    if value[field] is not None or value.get(field + '_present'):
        return True
    return any(not isinstance(choice, dict) or not choice or choice.get('type') == 'null'
               for choice in MODEL.alternatives(schema))


def restore(descriptor, value, json_text=False):
    if value is None:
        return None
    if descriptor == 'json':
        return json.loads(value) if json_text and isinstance(value, str) else value
    if descriptor in ('unsigned', 'integer'):
        return int(value)
    if isinstance(descriptor, str):
        return value
    mode, child = descriptor
    if mode == 'object':
        fields, union, mapping = MODEL.types[child]
        if union:
            for field, shape in fields.items():
                if value.get(field) is not None or value.get(field + '_present'):
                    return restore(shape[1], value[field], json_text)
            return None
        result = {}
        required = MODEL.required[child]
        populated = {field for field in fields if value[field] is not None}
        for alternative in MODEL.object_alternatives.get(child, []):
            members = alternative['properties']
            if populated <= members.keys() and all(
                    required_present(value, field, members[field])
                    for field in alternative.get('required', [])) and all(
                    value[field] == shape['const'] for field, shape in members.items() if 'const' in shape):
                required = required | set(alternative.get('required', []))
        for field, shape in fields.items():
            held = value[field]
            if shape[1] == 'json':
                if value[field + '_present']:
                    result[field] = restore(shape[1], held, json_text)
            elif held is not None or mapping or field in required:
                result[field] = restore(shape[1], held, json_text)
        return result
    if mode == 'mapping':
        entries = [restore(child, item, json_text) for item in value]
        return {entry['name']: entry['value'] for entry in entries}
    return [restore(child, item, json_text) for item in value]


def carrier(receipt, verb, json_text=False):
    results = [restore(('object', name(RESULTS[verb])), result, json_text) for result in receipt['results']]
    if verb == 'relate' and results:
        first = results[0]
        assert all(len(row['value']) == 1 for row in results)
        assert all({key:value for key,value in row.items() if key != 'value'} ==
                   {key:value for key,value in first.items() if key != 'value'} for row in results)
        results = [{**first, 'value':[row['value'][0] for row in results]}]
    native = {}
    facts = restore(('object', name('completeFacts')), receipt['facts'], json_text)
    error = restore(('object', name('completeError')), receipt['failure'], json_text)
    if error is not None and receipt['failure'].get('cause') is not None:
        error['cause'] = receipt['failure']['cause']
    if facts is not None:
        native['facts'] = facts
    if error is not None:
        native['error'] = error
    else:
        native['value'] = results[0] if verb in ('find', 'relate') else results
    wrapper = {'native': native, 'observations': [restore(('object', name('completesessionObservation')), event, json_text) for event in receipt['observations']]}
    if receipt['ordinals'] is not None:
        wrapper['ordinals'] = receipt['ordinals']
    if verb == 'find' and results:
        wrapper['selection'] = receipt['selection']
    if error is not None and results:
        wrapper['completed'] = results[0] if verb in ('find', 'relate') else results
    return wrapper


def row_observations(rows, verb):
    """Retain native event order and refuse ownership shared between rows."""
    if verb in ('rank', 'relate') and all(row['failure'] is None for row in rows):
        owners = [row for row in rows if row['ordinal'] == 0]
        assert len(owners) == 1, 'native set requires exactly one observation owner'
        owner = owners[0]
        assert all(row['observations'] == [] for row in rows if row['ordinal'] != 0)
        return owner['observations']
    events, previous = [], set()
    for row in rows:
        owned = row['observations'] or []
        identities = {json.dumps(event, sort_keys=True) for event in owned}
        assert previous.isdisjoint(identities), 'native observation belongs to multiple rows'
        events.extend(owned)
        previous.update(identities)
    return events
