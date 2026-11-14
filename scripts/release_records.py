"""Encode construction records for installed command smoke checks."""
import json


def encode_records(records):
    return ''.join(json.dumps(record) + '\n' for record in records).encode('utf-8')


def command_output(case):
    """Project canonical rows into JSON results or filter's original records."""
    records = case['request']['records']
    if case['request']['question']['function'] == 'filter':
        return [records[row['position'] - 1] for row in case['expected']
                if row['status'] == 'answered' and row.get('answer') is True]
    rows = []
    for canonical in case['expected']:
        row = dict(canonical, item=records[canonical['position'] - 1])
        rows.append(row)
    return rows
