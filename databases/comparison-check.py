"""Ensure SQL corpus comparison rejects fields that normalization could hide."""
from copy import deepcopy
from pathlib import Path
import sys

sys.path.insert(0, str(Path(__file__).resolve().parent / 'duckdb/tests'))
from migration_cases import decode, semantic, strict_equal
from typed_api import contract_row

source = dict(backend='openai', model='selected', question_identity='a' * 64)
missing = []
assert strict_equal({'answer': 1}, {'answer': 1.0})  # JSON has one numeric domain.
assert not strict_equal({'answer': {'score': True}}, {'answer': {'score': 1.0}})
assert not strict_equal({'answer': [1]}, {'answer': [True]})


def refuses(label, operation):
    try:
        operation()
    except AssertionError:
        return
    missing.append(label)


def decoded(provenance):
    return decode((1, 'answered', True, 0.9, None, None, None, None, provenance),
                  'decide', 'openai', 'selected')


assert decoded(deepcopy(source))['answer'] is True
for key, value in [('backend', 'wrong'), ('model', 'wrong'), ('extra', True)]:
    refuses('outer source ' + key, lambda key=key, value=value: decoded({**source, key: value}))
for provenance in (source, None):
    child = dict(function='decide', status='answered', answer=True, source=deepcopy(provenance))
    refuses('annotation child source', lambda child=child: semantic(dict(function='annotate', position=1,
        status='answered', answer={'decision': deepcopy(child)}, source=deepcopy(source))))
native = dict(position=1, id=None, status='answered', probability=0.9, answer=True,
    error_code=None, error_message=None, error_http_status=None, source=deepcopy(source), detail=None)
assert contract_row(deepcopy(native), 'decide')['answer'] is True
refuses('unexpected SQL column', lambda: contract_row({**native, 'extra': True}, 'decide'))
refuses('nullable unknown source field', lambda: contract_row({**native, 'source': {**source, 'extra': None}}, 'decide'))
refuses('nullable annotation source before SQL compaction', lambda: contract_row({**native,
    'answer': {'decision': dict(function='decide', status='answered', answer=True, source=None)}}, 'annotate'))
for status in ('answered', 'unsure'):
    for field, value in [('error_code', 'backend.reply'), ('error_message', 'stale'), ('error_http_status', 503)]:
        row = {**native, 'status': status, 'answer': True if status == 'answered' else None, field: value}
        refuses(status + ' inactive ' + field, lambda row=row: contract_row(deepcopy(row), 'decide'))
assert not missing, ('SQL comparison accepted invalid rows', missing)
print('SQL comparison refuses hidden source and inactive error fields')
