"""Run shared construction rows through the installed Python public API."""
import json
import sys
import thinkthen as tt

for case in json.load(open(sys.argv[1])):
    request = case['request']
    client = tt.Client(adapter='stub')
    function, question = request['function'], request['question']['value']
    records, options = request['records'], request.get('options', {})
    if function == 'filter':
        with client.filter(question, records, **options) as stream:
            actual = list(stream)
        expected = [record for record, row in zip(records, case['expected']) if row.get('answer') is True]
    else:
        if function == 'recognize':
            stream = client.recognize_stream(records, kinds=question, **options)
        elif function == 'relate':
            stream = client.relate_stream(records, rules=question, **options)
        elif function == 'rank':
            stream = client.rank(question, records, **options)
        else:
            stream = getattr(client, function + '_stream')(question, records, **options)
        with stream:
            actual = []
            for row in stream:
                value = {'id': row.id, 'status': row.status, 'answer': row.answer}
                if row.p is not None: value['p'] = row.p
                if row.detail is not None: value['detail'] = row.detail
                actual.append(value)
        expected = case['expected']
    assert actual == expected, (function, actual, expected)
print('candidate Python: ten shared stub cases pass')
