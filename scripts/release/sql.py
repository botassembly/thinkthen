"""Run shared construction rows through standard database commands."""
import json
import math
import subprocess
import sys
from pathlib import Path

kind, executable, extension, fixture = sys.argv[1:]
cases = json.loads(Path(fixture).read_text())
if kind == 'sqlite':
    shared = cases[0]
    request = shared['request']
    cases.append({**shared, 'request': {**request,
        'records': [{'body': record} for record in request['records']],
        'options': {**request.get('options', {}), 'on': '/body'}}})


def quote(value):
    return "'" + value.replace("'", "''") + "'"


for case in cases:
    request = case['request']
    function = request['function']
    settings = json.dumps(case['client'])
    question = quote(json.dumps(request['question']))
    options = quote(json.dumps(request.get('options', {})))
    if kind == 'sqlite':
        # UNION ALL retains each cell's JSON subtype without a sorting boundary.
        cells = [quote(record) if isinstance(record, str) else f"json({quote(json.dumps(record))})"
                 for record in request['records']]
        source = ' UNION ALL '.join(f'SELECT {cell} AS record' for cell in cells)
        statement = f'SELECT * FROM thinkthen_{function}({quote(source)},{question},{options});'
        load_path = json.dumps(extension.replace('\\', '/'), ensure_ascii=False)
        sql = f'.load {load_path}\nSELECT thinkthen_configure({quote(settings)});\n.mode json\n' + statement
        command = [executable, ':memory:']
    elif kind.startswith('duckdb'):
        structured = any(not isinstance(record, str) for record in request['records'])
        values = ','.join(f'({i},{quote(json.dumps(record) if structured else record)})' for i, record in enumerate(request['records']))
        cast = '::JSON' if structured else ''
        source = f'(SELECT value{cast} AS record FROM (VALUES {values}) input(n,value) ORDER BY n)'
        sql = f"LOAD {quote(extension)}; SET thinkthen_settings={quote(settings)}; SELECT status, answer::VARCHAR AS answer, p, error::VARCHAR AS error, detail::VARCHAR AS detail FROM thinkthen_{function}({source},{question},options := {options});"
        command = [executable, '-unsigned', '-json', ':memory:']
    else:
        source = "SELECT value AS record FROM json_array_elements(" + quote(json.dumps(request['records'])) + "::json) WITH ORDINALITY AS input(value,n) ORDER BY n"
        sql = f"SET thinkthen.settings={quote(settings)}; SELECT coalesce(json_agg(row_to_json(result)), '[]') FROM thinkthen_{function}({quote(source)},{question},{options}::json) result;"
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
            raise RuntimeError((kind, request["function"], completed.stdout, completed.stderr))
        lines = completed.stdout.strip().splitlines()
        # SQLite configure emits one empty line before its JSON output.
        start = next(i for i, line in enumerate(lines) if line.lstrip().startswith('['))
        result = json.loads('\n'.join(lines[start:]))
    actual = []
    for row in result:
        value = {'status': row['status']}
        for name in ('answer', 'error', 'detail', 'p'):
            item = row.get(name)
            if kind != 'postgres' and name in ('answer', 'error', 'detail') and isinstance(item, str):
                item = json.loads(item)
            if item is not None or name == 'answer': value[name] = item
        actual.append(value)
    expected = [{key: value for key, value in row.items() if key != 'id'} for row in case['expected']]
    if kind == 'sqlite':
        # SQLite 3.41 CLI decimal round trips can differ by one binary64 step.
        # Keep row counts and every other field exact.
        for row, wanted in zip(actual, expected, strict=True):
            if 'p' in row and 'p' in wanted:
                assert math.isclose(row['p'], wanted['p'], rel_tol=0, abs_tol=math.ulp(wanted['p'])), (kind, function, row, wanted)
                row['p'] = wanted['p']
    assert actual == expected, (kind, function, actual, expected)
print(f'candidate {kind}: {len(cases)} stub cases pass')
