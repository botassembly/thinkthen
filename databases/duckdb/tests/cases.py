"""SQL translation shared by installed DuckDB host checks."""
import json


def quote(value):
    return " || chr(0) || ".join("'" + part.replace("'", "''") + "'" for part in value.split("\0"))


def invocation(request):
    records = request.get('records', [])
    structured = any(not isinstance(record, str) for record in records)
    if records:
        values = ','.join(f'({i}, {quote(json.dumps(value) if structured else value)})'
                          for i, value in enumerate(records))
        cast = '::JSON' if structured else ''
        source = f'(SELECT value{cast} AS record FROM (VALUES {values}) input(ordinal,value) ORDER BY ordinal)'
    else:
        source = '(SELECT NULL::VARCHAR AS record WHERE false)'
    question = quote(json.dumps(request['question']))
    options = quote(json.dumps(request.get('options', {})))
    return f'thinkthen_{request["function"]}({source}, {question}, options := {options})'


def rows(result):
    answer = []
    for row in result:
        value = {'status': row['status']}
        for name in ('answer', 'error', 'detail'):
            if row[name] is not None:
                value[name] = json.loads(row[name])
        if row['p'] is not None:
            value['p'] = float(row['p'])
        answer.append(value)
    return answer


def scrub(value):
    if isinstance(value, dict):
        return {key: scrub(item) for key, item in value.items() if key != 'message'}
    if isinstance(value, list):
        return [scrub(item) for item in value]
    return value


def expected(case):
    return [{key: value for key, value in row.items() if key != 'id'} for row in case['expected']]


