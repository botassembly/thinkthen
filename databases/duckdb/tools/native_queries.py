"""Test-only authored descriptors become ordinary native SQL constructor arguments."""
import json


def value(item):
    if item is None:
        return 'NULL'
    if isinstance(item, bool):
        return 'true' if item else 'false'
    if isinstance(item, (int, float)):
        return str(item)
    if isinstance(item, str):
        return "'" + item.replace("'", "''") + "'"
    if isinstance(item, list):
        return '[' + ','.join(value(member) for member in item) + ']'
    return '{' + ','.join(value(key) + ':' + value(member) for key, member in item.items()) + '}'


def call(verb, question, descriptor, settings=None):
    if isinstance(descriptor, str):
        try:
            descriptor = json.loads(descriptor)
        except json.JSONDecodeError:
            descriptor = {'records': descriptor}
    if 'files' in descriptor:
        files = descriptor['files']
        fields = {'source': True}
        if isinstance(files, dict):
            fields.update({key: member for key, member in files.items() if key != 'options'})
            options = files.get('options', {})
            fields.update(options.get('reading', {}))
            if 'media' in options:
                fields['media'] = options['media']
        else:
            fields['paths'] = files
        records = '[' + value(fields) + ']'
    else:
        rows = descriptor.get('records', [])
        if isinstance(rows, list):
            records = '[' + ','.join(record(row) for row in rows) + ']'
        else:
            records = value(rows)
    reading = dict(descriptor.get('reading', {}))
    for original, canonical in [('fields', 'field'), ('context', 'context_field'), ('options', 'options_field'),
                                ('examples', 'examples_field'), ('seed_spans', 'seed_spans_field')]:
        if original in reading:
            reading[canonical] = reading.pop(original)
    reading['attempts'] = descriptor.get('attempts', False)
    return f'(SELECT detail.* FROM (SELECT UNNEST(thinkthen_{verb}_details(question := {question}, input := {records}, reading := {value(reading)}, ' + \
        f'settings := {value(settings)}, incremental := {value(descriptor.get("incremental", False))}, ' + \
        f'cancelled := {value(descriptor.get("cancelled", False))})) AS detail))'


def record(item):
    fields = []
    for key, member in item.items():
        expression = value(member)
        if key == 'json':
            expression = value(json.dumps(member, ensure_ascii=False, separators=(',', ':'))) + '::JSON'
        fields.append(value(key) + ':' + expression)
    return '{' + ','.join(fields) + '}'
