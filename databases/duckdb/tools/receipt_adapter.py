"""Project actual typed SQL receipts into shared native-result assertions."""
import json
from pathlib import Path
from receipt_types import ROOTS, model_for, name

ROOT = Path(__file__).resolve().parents[3]
MODEL = model_for(json.loads((ROOT / 'crates/thinkthen/src/public/results/complete.schema.json').read_text()))
for root in ROOTS:
    MODEL.shape({'$ref': '#/$defs/' + root}, root)
RESULTS = dict(zip(('decide', 'choose', 'tag', 'score', 'filter', 'rank', 'annotate', 'recognize', 'find', 'relate'), ROOTS))


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
    native = {}
    facts = restore(('object', name('completeFacts')), receipt['facts'], json_text)
    error = restore(('object', name('completeError')), receipt['failure'], json_text)
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
