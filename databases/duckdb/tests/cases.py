"""Map typed construction questions to native DuckDB arguments."""
import json


def quote(value):
    return ' || chr(0) || '.join("'" + part.replace("'", "''") + "'" for part in value.split('\0'))


def scalar(value):
    if value is None:
        return 'NULL'
    if isinstance(value, bool):
        return str(value).lower()
    return quote(value) if isinstance(value, str) else repr(value)


def arguments(question):
    result = [scalar(question['question'])] if 'question' in question else []
    for key, value in question.items():
        if key in ('function', 'question'):
            continue
        if key in ('true', 'false'):
            key = {'true': 'yes_means', 'false': 'no_means'}[key]
        if key == 'threshold' and isinstance(value, dict):
            result.extend(bound + ' := ' + scalar(number) for bound, number in value.items())
            continue
        if key == 'questions':
            value = question['questions']
            key, value = 'toolbox', quote(json.dumps(value)) + '::JSON'
        elif key in ('choices', 'labels', 'levels', 'kinds'):
            described = {item['name']: item['description'] for item in value if isinstance(item, dict)}
            if described:
                result.append("descriptions := MAP {" + ','.join(quote(name) + ':' + quote(text) for name, text in described.items()) + '}')
            value = '[' + ','.join(scalar(item['name']) if isinstance(item, dict) else scalar(item) for item in value) + ']'
        elif key == 'rules':
            rules = []
            for rule in value:
                rules.append('struct_pack(' + ','.join(name + ' := ' + scalar(item) for name, item in rule.items()) + ')')
            value = '[' + ','.join(rules) + ']'
        else:
            value = scalar(value)
        result.append(key + ' := ' + value)
    return result


def invocation(request):
    records = request.get('records', [])
    values = ','.join(f'({index}, {quote(json.dumps(record))})' for index, record in enumerate(records))
    source = ('SELECT value::JSON AS record FROM (VALUES ' + values + ') src(n,value) ORDER BY n'
              if records else 'SELECT NULL::JSON AS record WHERE false')
    if 'name' in request:
        question_arguments = [*arguments(request['question']), 'name := ' + scalar(request['name'])]
    else:
        question_arguments = arguments(request['question'])
    return 'thinkthen_' + request['question']['function'] + '((' + source + ')' + ''.join(', ' + value for value in question_arguments) + ')'


def canonical(row, function):
    """Omit SQL NULL fields that represent absent optional contract fields."""
    for key in ('answer', 'detail', 'source'):
        if key in row:
            row[key] = compact(row[key])
    if function == 'annotate' and row['detail'] is not None:
        row['answer'] = {key: canonical(child, child['function']) for key, child in row.pop('detail').items()}
    if row.get('error_code') is not None:
        row['error'] = {key: row['error_' + key] for key in ('code', 'message', 'http_status') if row.get('error_' + key) is not None}
    if row.get('error') is not None:
        row['error'] = {key: value for key, value in row['error'].items() if value is not None}
    return {key: value for key, value in row.items() if value is not None or (key == 'answer' and row['status'] == 'answered')}


def compact(value):
    if isinstance(value, list):
        return [compact(child) for child in value]
    if isinstance(value, dict):
        if 'function' in value and 'status' in value:
            return canonical(value, value['function'])
        return {{'start_offset':'start','end_offset':'end'}.get(key, key): compact(child) for key, child in value.items()
                if child is not None and not (key == 'either' and child is False)}
    return value
