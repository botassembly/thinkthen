"""Encode construction records for installed command smoke checks."""
import json


def encode_records(records):
    return ''.join(json.dumps(record) + '\n' for record in records).encode('utf-8')
