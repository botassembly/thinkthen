"""Plant invalid ADR 0055 packed questions; an unvalidated row could fake a yes."""
import copy
from backend import decode_packed_decide, PACKED_EVIDENCE

valid={'state':PACKED_EVIDENCE,'questions':{'q1':{'type':'noul','instructions':'The text is "first". Is it?'},'q2':{'type':'noul','instructions':'The text is "second". Is it?'}}}
assert decode_packed_decide(valid)=={'q1':'first','q2':'second'}
for label,alter in [
    ('wrong-question',lambda x:x['questions']['q2'].update(instructions='The text is "second". Answer another question?')),
    ('malformed-record',lambda x:x['questions']['q2'].update(instructions='The text is {broken. Is it?')),
    ('wrong-kind',lambda x:x['questions']['q2'].update(type='choice')),
    ('duplicate-evidence',lambda x:x['questions']['q2'].update(instructions='The text is "first". Is it?')),
]:
    request=copy.deepcopy(valid);alter(request)
    try:
        decode_packed_decide(request)
    except ValueError:
        print('PACKING_NEGATIVE_PASS',label)
    else:
        raise AssertionError('invalid packed row was accepted: '+label)
