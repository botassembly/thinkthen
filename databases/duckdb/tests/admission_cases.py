"""Public SQL projections of shared JSON admission cases.

Native arrays, composites and setting values cannot carry all malformed JSON.
Report those canonical boundaries as unsupported and exercise annotation's public
JSON child boundary separately. Expected paths describe that changed boundary;
actual diagnostics are never rewritten.
"""
import json


def cases(root, surface):
    fixtures = json.loads((root / 'conformance/public-contract/usage-admission.json').read_text())
    reasons = {
        'Integer score levels': 'native levels convert or reject members before canonical JSON admission',
        'TypeScript saved toolbox array evidence': 'SQL has no saved toolbox entries envelope',
        'SQLite wrong-type settings': 'native setting values reject strings before canonical settings JSON admission',
        'R tags in a serialized question': 'native tag arguments expose labels and cannot carry an unknown JSON field',
        'Malformed relation rule source': 'native rule composites convert or reject source before canonical JSON admission',
    }
    for case in fixtures:
        if case.get('kind') != 'admission':
            continue
        name = case['name']
        reason = reasons[name]  # A new fixture needs an explicit SQL disposition.
        direct = surface == 'sqlite' and name == 'Malformed relation rule source'
        print(f'{surface} admission {name}: ' + ('direct' if direct else 'unsupported: ' + reason))
        if case['boundary'] == 'settings':
            continue
        if direct:
            yield case, {'question': case['input'], 'records': ['evidence']}, case['message']
            continue
        child = json.loads(json.dumps(case['input']))
        if case['boundary'] == 'toolbox':
            child = child['entries']['ready']['question']
            prefix = 'toolbox.entries.[0].question.'
        else:
            assert case['boundary'] == 'question', case
            prefix = 'question.'
        assert case['message'].startswith(prefix), case
        # SQL's relation-name translation admits rules before the parent question.
        message = (case['message'] if child['function'] == 'relate' else
                   'question.questions.[0].' + case['message'][len(prefix):])
        print(f'{surface} admission {name}: public projection through annotate.toolbox child')
        yield case, {'question': {'function': 'annotate', 'questions': {'ready': child}},
                     'records': ['evidence']}, message


def diagnostic(case, code, message, expected):
    assert 'secret-placeholder' not in message, case['name']
    assert (code, message) == (case['code'], expected), (case['name'], code, message, expected)
