"""Run shared construction rows through the installed Python public API."""
from collections.abc import Mapping
from dataclasses import fields, is_dataclass
from enum import Enum
import json
import sys

import thinkthen as tt


def plain(value):
    if isinstance(value, Enum):
        return value.value
    if is_dataclass(value):
        if hasattr(value, 'status'):
            return outcome_json(value)
        return {
            field.name: plain(getattr(value, field.name)) for field in fields(value)
            if (field.name != 'name' or getattr(value, field.name) is not None)
            and (field.name != 'either' or getattr(value, field.name))
        }
    if isinstance(value, Mapping):
        return {key: plain(item) for key, item in value.items()}
    if isinstance(value, (list, tuple)):
        return [plain(item) for item in value]
    return value


def outcome_json(row):
    value = {
        'function': row.function,
        'status': row.status,
    }
    if row.status == 'answered':
        value['answer'] = plain(row.answer)
    if row.probability is not None:
        value['probability'] = row.probability
    if row.detail is not None:
        value['detail'] = plain(row.detail)
    if row.error is not None:
        value['error'] = {'code': row.error.code, 'message': row.error.message}
        if row.error.http_status is not None:
            value['error']['http_status'] = row.error.http_status
    return value


def row_json(row):
    value = outcome_json(row)
    value.update(position=row.position, source=plain(row.source))
    if row.id is not None:
        value['id'] = row.id
    return value


def admitted(definition):
    options = dict(definition)
    function = options.pop('function')
    if function == 'relate':
        options['rules'] = [tt.RelationRule(**rule) for rule in options['rules']]
    if isinstance(options.get('threshold'), dict):
        band = options['threshold']
        options['threshold'] = tt.Band(band['low'], band['high'])
    return getattr(tt, function.capitalize() + 'Question')(**options)


with open(sys.argv[1], encoding='utf-8') as source:
    cases = json.load(source)
functions = set()
for case in cases:
    request = case['request']
    definition = request['question']
    function = definition['function']
    functions.add(function)
    settings = case['client']
    client = tt.Client(
        backend=settings['backend'], model=settings.get('model'),
        url=settings.get('url'), batch=settings.get('batch'),
    )
    records = request['records']
    if function == 'annotate':
        stream = client.annotate_stream(
            tt.AnnotationQuestions({
                name: admitted(value) for name, value in definition['questions'].items()
            }), records, evidence=definition.get('evidence'),
        )
    else:
        toolbox = tt.Toolbox({'case': admitted(definition)})
        question = getattr(toolbox, function)('case')
        if function in {'filter', 'rank'}:
            stream = getattr(client, function)(question, records)
        else:
            stream = getattr(client, function + '_stream')(question, records)
    with stream:
        actual = [row_json(row) for row in stream]
    assert actual == case['expected'], (function, actual, case['expected'])
assert len(functions) == 10, functions
print('candidate Python: ten shared stub cases pass')
