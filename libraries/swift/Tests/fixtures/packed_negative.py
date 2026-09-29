"""Current-main packed-question parsing must reject malformed quoted rows."""
from backend import PACKED_STATE, packed_rows
request = {'state': PACKED_STATE, 'questions': {'q1': {'instructions': 'The text is "first". Is it?'}}}
assert packed_rows(request) == {'q1': 'first'}
for invalid in ('first Is it?', 'The text is "first". Not this?', 'The text is ["first"]. Is it?'):
    request['questions']['q1']['instructions'] = invalid
    try:
        packed_rows(request)
    except ValueError:
        continue
    raise AssertionError(f'accepted malformed packed instruction: {invalid!r}')
print('PACKED_PARSER_NEGATIVE_PASS three malformed quoted rows refused')
