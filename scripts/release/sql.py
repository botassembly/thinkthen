"""Run shared construction rows through standard database commands."""
import json
import math
import subprocess
import sys
from pathlib import Path

kind, executable, extension, fixture = sys.argv[1:]
cases = [(case, False) for case in json.loads(Path(fixture).read_text())]
if kind == 'sqlite':
    shared = cases[0][0]
    request = shared['request']
    cases.append(({**shared, 'request': {**request,
        'question': {**request['question'], 'evidence': '/body'},
        'records': [{'body': record} for record in request['records']]}}, True))


def quote(value):
    carriage_return = 'char(13)' if kind == 'sqlite' else 'chr(13)'
    return (' || ' + carriage_return + ' || ').join(
        "'" + part.replace("'", "''") + "'" for part in value.split('\r'))


def scalar(value):
    if value is None:
        return 'NULL'
    if isinstance(value, bool):
        return 'true' if value else 'false'
    return quote(value) if isinstance(value, str) else repr(value)


def arguments(question):
    result = []
    for name, value in question.items():
        if name == 'function':
            continue
        if name == 'threshold' and isinstance(value, dict):
            result.extend((bound, scalar(number)) for bound, number in value.items())
            continue
        if name == 'questions':
            name, value = 'toolbox', quote(json.dumps(value)) + ('::jsonb' if kind == 'postgres' else '::JSON')
        elif name in ('choices', 'labels', 'levels', 'kinds'):
            value = ('ARRAY' if kind == 'postgres' else '') + '[' + ','.join(quote(item) for item in value) + ']'
        elif name == 'rules':
            rules = []
            for rule in value:
                fields = [(key, rule.get(key, False if key == 'either' else None))
                          for key in ('name', 'source', 'target', 'reads', 'either')]
                if kind == 'postgres':
                    rules.append('ROW(' + ','.join(scalar(item) for _, item in fields) + ')::thinkthen.rule')
                else:
                    rules.append('struct_pack(' + ','.join(key + ' := ' + scalar(item) for key, item in fields) + ')')
            value = ('ARRAY' if kind == 'postgres' else '') + '[' + ','.join(rules) + ']'
        else:
            value = scalar(value)
        result.append((name, value))
    return result


def settings_sql(client):
    statements = []
    if kind == 'postgres':
        statements.append('SET thinkthen.allowed_backends = ' + quote(client['backend'] + ':*'))
    for key, value in client.items():
        if key in ('url', 'api_key'):
            assert value is None, ('SQL cannot configure', key)
            continue
        for name, item in (value.items() if key == 'retry' else [(key, value)]):
            if item is None:
                continue
            if kind == 'sqlite':
                statements.append('SELECT thinkthen_set(' + quote(name) + ',' + scalar(item) + ')')
            else:
                prefix = 'thinkthen.' if kind == 'postgres' else 'thinkthen_'
                statements.append('SET ' + prefix + name + ' = ' + scalar(item))
    return '; '.join(statements) + ';\n'


def compact(value):
    if isinstance(value, list):
        return [compact(child) for child in value]
    if isinstance(value, dict):
        return {key: compact(child) for key, child in value.items()
                if (child is not None or key == 'answer' and value.get('status') == 'answered')
                and not (key == 'either' and child is False)}
    return value


def normalize(row, function):
    value = {'function': function, 'position': row['position'], 'status': row['status']}
    for name in ('id', 'source', 'detail'):
        item = row.get(name)
        if item is not None:
            value[name] = json.loads(item) if kind == 'sqlite' else compact(item)
            if name == 'source' and value[name].get('name') is None:
                value[name].pop('name', None)
    if row.get('probability') is not None:
        value['probability'] = row['probability']
    if row['status'] == 'failed':
        value['error'] = {'code': row['error_code'], 'message': row['error_message']}
        if row['error_http_status'] is not None:
            value['error']['http_status'] = row['error_http_status']
    elif row['status'] == 'answered':
        answer = row.get('answer')
        if function == 'rank':
            answer = row['rank']
        elif function == 'find':
            answer = (row.get('found') if kind.startswith('duckdb') else
                None if row['found_text'] is None else
                dict(text=row['found_text'], start=row['found_start'], end=row['found_end']))
        elif kind == 'sqlite' and answer is not None:
            if function in ('decide', 'filter'):
                assert type(answer) is int and answer in (0, 1), row
                answer = bool(answer)
            elif function in ('tag', 'recognize', 'relate', 'annotate'):
                answer = json.loads(answer)
        value['answer'] = compact(answer) if kind != 'sqlite' else answer
    if function == 'filter':
        assert row['kept'] == (row['status'] == 'answered' and value.get('answer') is True), row
    return value


def public_question(question):
    question = json.loads(json.dumps(question))
    if question['function'] == 'relate':
        for rule in question['rules']:
            rule['name'] = rule.pop('relation')
    elif question['function'] == 'annotate':
        question['questions'] = {name: public_question(child) for name, child in question['questions'].items()}
    return question


for case, selected_evidence in cases:
    request = case['request']
    question = public_question(request['question'])
    function = question['function']
    settings = settings_sql(case['client'])
    if kind == 'sqlite':
        cells = [quote(json.dumps(record)) for record in request['records']]
        source = ' UNION ALL '.join('SELECT ' + cell + ' AS record' for cell in cells) if cells else 'SELECT NULL AS record WHERE 0'
        constraints, values = ['source_sql = ' + quote(source), "format = 'json'"], []
        for name, item in question.items():
            if name == 'function':
                continue
            if name == 'threshold' and isinstance(item, dict):
                constraints.extend(bound + ' = ' + scalar(number) for bound, number in item.items())
            else:
                name = 'toolbox' if name == 'questions' else name
                constraints.append(name + ' = ' + (quote(json.dumps(item)) if isinstance(item, (dict, list)) else scalar(item)))
        if 'name' in request:
            constraints.append('name = ' + quote(request['name']))
        statement = 'SELECT * FROM thinkthen_' + function + ' WHERE ' + ' AND '.join(constraints) + ';'
        load_path = json.dumps(extension.replace('\\', '/'), ensure_ascii=False)
        sql = '.bail on\n.load ' + load_path + '\n' + settings + '.mode json\n' + statement
        command = [executable, ':memory:']
    else:
        args = arguments(question)
        if 'name' in request:
            args.append(('name', quote(request['name'])))
        if kind.startswith('duckdb'):
            values = ','.join(f'({i},{quote(json.dumps(record))}::JSON)' for i, record in enumerate(request['records']))
            source = (f'(SELECT value AS record FROM (VALUES {values}) input(n,value) ORDER BY n)'
                      if values else '(SELECT NULL::JSON AS record WHERE false)')
            ordered = [value for key, value in args if key == 'question']
            ordered.extend(key + ' := ' + value for key, value in args if key != 'question')
            invocation = 'thinkthen_' + function + '(' + source + ''.join(', ' + value for value in ordered) + ')'
            sql = 'LOAD ' + quote(extension) + '; ' + settings + 'SELECT to_json(t)::VARCHAR AS row FROM ' + invocation + ' t;'
            command = [executable, '-unsigned', '-json', ':memory:']
        else:
            assert kind == 'postgres', kind
            source = 'SELECT value AS record FROM jsonb_array_elements(' + quote(json.dumps(request['records'])) + '::jsonb) WITH ORDINALITY input(value,n) ORDER BY n'
            invocation = 'thinkthen.' + function + '(' + quote(source) + ''.join(', ' + key + ' => ' + value for key, value in args) + ')'
            sql = settings + "SELECT coalesce(json_agg(row_to_json(t)), '[]') FROM " + invocation + ' t;'
            command = [executable, '-X', '-A', '-t', '-v', 'ON_ERROR_STOP=1']
    if kind == 'duckdb-python':
        import duckdb
        with duckdb.connect(config={'allow_unsigned_extensions': True}) as connection:
            result = connection.execute(sql)
            columns = [column[0] for column in result.description]
            result = [dict(zip(columns, row)) for row in result.fetchall()]
    else:
        completed = subprocess.run(command, input=sql, text=True, capture_output=True)
        if completed.returncode:
            raise RuntimeError((kind, function, completed.stdout, completed.stderr))
        lines = completed.stdout.strip().splitlines()
        start = next(i for i, line in enumerate(lines) if line.lstrip().startswith('['))
        result = json.loads('\n'.join(lines[start:]))
    if kind.startswith('duckdb'):
        result = [json.loads(row['row']) for row in result]
    actual = [normalize(row, function) for row in result]
    expected = case['expected']
    if selected_evidence:
        for row, wanted in zip(actual, expected, strict=True):
            identity = row['source']['question_identity']
            assert len(identity) == 64 and all(character in '0123456789abcdef' for character in identity), identity
            assert identity != wanted['source']['question_identity'], identity
            assert {key: value for key, value in row['source'].items() if key != 'question_identity'} == {
                key: value for key, value in wanted['source'].items() if key != 'question_identity'}, row
        actual = [{key: value for key, value in row.items() if key != 'source'} for row in actual]
        expected = [{key: value for key, value in row.items() if key != 'source'} for row in expected]
    if kind == 'sqlite':
        for row, wanted in zip(actual, expected, strict=True):
            if 'probability' in row and 'probability' in wanted:
                assert math.isclose(row['probability'], wanted['probability'], rel_tol=0,
                                    abs_tol=math.ulp(wanted['probability'])), (kind, function, row, wanted)
                row['probability'] = wanted['probability']
    assert actual == expected, (kind, function, actual, expected)
print(f'candidate {kind}: {len(cases)} stub cases pass')
