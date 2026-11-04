"""Check the settled SQL result layout through the installed extension."""
import json
import re

from migration_cases import fixture, command

COMMON = ['position', 'id', 'answer', 'probability', 'status', 'error_code',
          'error_message', 'error_http_status', 'source', 'detail']


def interface(factory):
    with factory() as host:
        host.configure({'backend': 'stub'})
        columns = host.query("DESCRIBE SELECT * FROM thinkthen_decide((SELECT 'hello' AS record), 'Ready?')")
        assert [row['column_name'] for row in columns] == ['record', *COMMON], columns
        types = {row['column_name']: row['column_type'] for row in columns}
        assert types['position'] == 'BIGINT', types
        assert types['status'] == 'thinkthen_status', types
        assert types['error_code'] == 'thinkthen_error_code', types
        assert types['detail'] == 'JSON', types
        for function, args in [('decide', "'Ready?'"), ('filter', "'Ready?'"),
                               ('choose', "'Which?', choices := ['one','two']"),
                               ('tag', "'Which?', labels := ['one','two']"),
                               ('score', "'How?', levels := ['low','high']"),
                               ('rank', "'How?', levels := ['low','high']"),
                               ('find', "'Where?'"), ('recognize', "kinds := ['person']"),
                               ('relate', "rules := [struct_pack(relation := 'knows', source := 'person', target := 'person')]"),
                               ('annotate', "toolbox := '{\"ready\":{\"function\":\"decide\",\"question\":\"Ready?\"}}'::JSON")]:
            invocation = 'thinkthen_' + function + "((SELECT 'hello' AS record), " + args + ')'
            columns = host.query('DESCRIBE SELECT * FROM ' + invocation)
            assert [row['column_name'] for row in columns] == ['record', *COMMON], (function, columns)
            if function in ('find', 'recognize'):
                answer_type = next(row['column_type'] for row in columns if row['column_name'] == 'answer')
                fields = re.findall(r'[(,]\s*"?(text|kind|start_offset|end_offset|probability)"?\s', answer_type)
                expected = ['text', 'start_offset', 'end_offset'] if function == 'find' else ['text', 'kind', 'start_offset', 'end_offset', 'probability']
                assert fields == expected, (function, answer_type)
        row = host.query("SELECT to_json(t) AS result FROM thinkthen_annotate((SELECT 'hello' AS record), toolbox := '{\"ready\":{\"function\":\"decide\",\"question\":\"Ready?\"}}'::JSON) t")
        value = json.loads(row[0]['result'])
        assert value['answer']['ready'] in (True, False, None), value
        assert value['detail']['ready']['function'] == 'decide', value
        host.failure("SELECT * FROM thinkthen_relate((SELECT 'Ada works for Acme.' AS record), rules := [struct_pack(name := 'works_for', source := 'person', target := 'organization')])", 'usage.question')
        host.failure("SELECT * FROM thinkthen_filter((SELECT NULL::VARCHAR AS record), 'Ready?')", 'record.missing')
        row = host.query("SELECT status,error_code::VARCHAR AS error_code FROM thinkthen_filter((SELECT NULL::VARCHAR AS record), 'Ready?', failures := 'row')")
        assert row == [{'status': 'failed', 'error_code': 'record.missing'}], row
        host.failure("SELECT * FROM thinkthen_rank((SELECT 'hello' AS record), 'How?', levels := ['low','high'], top := -1)", 'usage.')
        host.failure("SELECT * FROM thinkthen_choose((SELECT 'hello' AS record), 'Which?', choices := 'one')", 'usage.')
        host.query("SELECT * FROM thinkthen_decide((SELECT 'hello' AS record), 'Ready?', yes_means := 'Yes', no_means := 'No')")
        host.query("SELECT * FROM thinkthen_choose((SELECT 'hello' AS record), 'Which?', choices := ['one','two'], descriptions := MAP {'one':'First'})")
        host.failure("SELECT * FROM thinkthen_choose((SELECT 'hello' AS record), 'Which?', choices := ['one','two'], descriptions := MAP {'missing':'First'})", 'usage.')
        help_rows = host.query("SELECT function_name, description, array_length(examples) AS examples FROM duckdb_functions() WHERE function_name LIKE 'thinkthen_%'")
        assert len(help_rows) == 10 and all(row['description'] and int(row['examples']) > 0 for row in help_rows), help_rows
    print('DuckDB settled result layout and SQL options pass', flush=True)


def blank_inputs(factory, loopback):
    # An actual backend route receives no requests for selected empty evidence.
    with fixture(loopback, '--held', 'headers') as (server, url), factory(url) as host:
        host.configure({'backend': 'systemone'})
        records = ['{"text":""}', '{"text":"  "}', '{"text":{}}', '{"text":[]}', '{"text":null}']
        source = ' UNION ALL '.join("SELECT '" + value + "'::JSON AS record" for value in records)
        rows = host.query("SELECT position,status,error_code::VARCHAR AS error_code FROM thinkthen_decide((" + source + "), 'Ready?', field := '/text')")
        assert [row['error_code'] for row in rows] == ['record.empty'] * 4 + ['record.missing'], rows
        assert all(row['status'] == 'failed' for row in rows), rows
        assert int(command(server, 'count')) == 0
        rows = host.query("SELECT error_code::VARCHAR AS error_code FROM thinkthen_decide((SELECT '{\"id\":42}'::JSON AS record), 'Ready?')")
        assert rows == [{'error_code':'record.empty'}], rows
        assert int(command(server, 'count')) == 0
    with factory() as host:
        host.configure({'backend': 'stub'})
        rows = host.query("SELECT position,status,error_code::VARCHAR AS error_code FROM thinkthen_decide((SELECT record FROM (VALUES (1,'first'),(2,''),(3,'last')) input(n,record) ORDER BY n), 'Ready?')")
        assert [int(row['position']) for row in rows] == [1, 2, 3], rows
        assert [row['status'] for row in rows] == ['answered', 'failed', 'answered'], rows
        assert rows[1]['error_code'] == 'record.empty', rows
        literal = host.query("SELECT status FROM thinkthen_decide((SELECT record FROM (VALUES ('{}'),('[]')) input(record)), 'Ready?')")
        assert literal == [{'status':'answered'}, {'status':'answered'}], literal
        filtered = host.query("SELECT position,status,error_code::VARCHAR AS error_code FROM thinkthen_filter((SELECT record FROM (VALUES (1,'refund'),(2,'parcel'),(3,'uncertain'),(4,NULL)) input(n,record) ORDER BY n), 'Ready?', low := 0.2, high := 0.8, failures := 'row')")
        assert [int(row['position']) for row in filtered] == [2, 3, 4], filtered
        assert [row['status'] for row in filtered] == ['answered', 'answered', 'failed'], filtered
        host.failure("SELECT * FROM thinkthen_filter((SELECT record FROM (VALUES ('parcel'),(NULL),('uncertain'),(NULL)) input(record)), 'Ready?', low := 0.2, high := 0.8)", '2 records failed')
    print('DuckDB selected blank evidence makes no backend call and streams continue', flush=True)
