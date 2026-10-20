"""Normalize runtime metadata only along generated native schema paths."""
import json
from pathlib import Path
import re
from decimal import Decimal

SCHEMA = json.loads((Path(__file__).resolve().parents[2] /
                     'specification/result.schema.json').read_text())['$defs']
IDENTITIES = {'call_id', 'observation_id', 'failure_id', 'answer_id',
              'sdk_request_id', 'request_sha256', 'request'}


def branches(schema, value, *, absent_metadata=False):
    """Follow carrier alternatives using their shapes and authored discriminators."""
    if not isinstance(schema, dict):
        return
    if '$ref' in schema:
        yield from branches(SCHEMA[schema['$ref'].rsplit('/', 1)[1]], value,
                            absent_metadata=absent_metadata)
        return
    types = schema.get('type', [])
    types = [types] if isinstance(types, str) else types
    if types and ((isinstance(value, dict) and 'object' not in types) or
                  (isinstance(value, list) and 'array' not in types) or
                  (value is None and 'null' not in types)):
        return
    if isinstance(value, dict):
        required = schema.get('required', [])
        if absent_metadata and 'x-probability-mode' in schema:
            required = [key for key in required if key not in ('pick', 'index')]
        if any(key not in value for key in required):
            return
        for key, field in schema.get('properties', {}).items():
            if key in value and isinstance(field, dict) and 'const' in field:
                if value[key] != field['const']:
                    return
    yield schema
    for field in ('allOf', 'anyOf', 'oneOf'):
        for option in schema.get(field, []):
            yield from branches(option, value, absent_metadata=absent_metadata)


def canonical(document, folders=(), *, expected_oracle=False, absent_metadata=False):
    """Leave semantic data untouched; optionally reindex stored oracle identities."""
    identities = {}

    def visit(value, schema, key=''):
        # An unconstrained schema is user data, regardless of names or content.
        if not isinstance(schema, dict):
            return value
        choices = list(branches(schema, value, absent_metadata=absent_metadata))
        if not choices:
            return value
        if isinstance(value, dict):
            properties = {}
            for option in choices:
                properties.update(option.get('properties', {}))
            additional = next((option['additionalProperties'] for option in choices
                               if 'additionalProperties' in option), True)
            settled = absent_metadata and expected_oracle and any(
                'x-probability-mode' in option for option in choices)
            def absent(name, child):
                return settled and name in ('pick', 'index') and (child is None or
                    isinstance(child, list) and bool(child) and all(item is None for item in child))
            return {name: visit(child, properties.get(name, additional),
                                name if name in properties else '')
                    for name, child in sorted(value.items()) if not absent(name, child)}
        if isinstance(value, list):
            items = next((option['items'] for option in choices if 'items' in option), True)
            return [visit(child, items, 'request_sha256' if key == 'requests' else key)
                    for child in value]
        if key in IDENTITIES and isinstance(value, str):
            group = 'request_sha256' if key == 'request' else key
            native = re.fullmatch('[0-9a-f]{64}', value)
            stored = expected_oracle and re.fullmatch(re.escape(group) + r':[1-9][0-9]*', value)
            if native or stored:
                table = identities.setdefault(group, {})
                return table.setdefault(value, f'{group}:{len(table) + 1}')
        if key in ('seconds', 'wall_ms') and isinstance(value, str):
            assert Decimal(value) >= 0, (key, value)
            return '0'
        if key == 'url' and isinstance(value, str):
            return re.sub(r'http://127\.0\.0\.1:\d+/', 'http://127.0.0.1:PORT/', value)
        if key in ('file', 'message') and isinstance(value, str):
            for prefix, replacement in folders:
                value = value.replace(prefix, replacement)
        return value

    envelope = {'properties': {
        'packets': {'items': {'$ref': '#/$defs/sessionPacket'}},
        'error': {'$ref': '#/$defs/completeCallError'},
        'host': {'properties': {
            'facts': {'$ref': '#/$defs/completeFacts'},
            'results': {'items': {'$ref': '#/$defs/completeDetailed'}},
            'call_result': {'properties': {
                'facts': {'$ref': '#/$defs/completeFacts'},
                'results': {'items': {'$ref': '#/$defs/completeDetailed'}},
                'answer': {'$ref': '#/$defs/completesettledAnswer'},
            }},
        }},
    }}
    return [visit(dump, envelope) for dump in document]
