"""Installed named SQL controls preserve native readings and admission boundaries."""
import json
import os
from pathlib import Path
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'databases/sqlite/tests/complete'))
from parity import Backend
from postgresql_host import PostgresqlHost
sys.path.insert(0, str(ROOT / 'conformance/children'))
from children import child_env
from find_cases import sql, quoted


def main():
    with tempfile.TemporaryDirectory(prefix='thinkthen-pg-native-') as tmp:
        home = Path(tmp)
        env = child_env(home=home, THINKTHEN_API_KEY='sk-native-sql-loopback')
        target = Path(os.environ.get('CARGO_TARGET_DIR', ROOT / 'target'))
        backend = Backend(target / 'debug/conformance-backend', env)
        host = PostgresqlHost()
        try:
            host.start({**env, 'THINKTHEN_BASE_URL': f'http://127.0.0.1:{backend.port}/arm/full/capture/v1'})

            def query(statement, sends=0):
                before = int(backend.read('count'))
                out, err = sql(host.socket, "SET thinkthen.cache='off'", "SET thinkthen.batch='1'",
                               "SET thinkthen.model='jev-1.13.0'", "SET thinkthen.max_retries=0", statement)
                assert not err, (out, err)
                assert int(backend.read('count')) - before == sends, out
                return out

            signature = "thinkthen_details(text,text,json,text,text,text,text,bigint)"
            assert query("SELECT prorettype::regtype FROM pg_proc WHERE oid=" +
                         quoted(signature) + "::regprocedure") == 'thinkthen_question_detail'
            receipt = "thinkthen_details('Fits?','Alpha.')"
            assert query("SELECT ((d).value).boolean_value, ((d).probabilities).kind, "
                         "(((d).probabilities).value).number_value, length((d).answer_id), "
                         "(d).model, (d).requests_sent, (d).cached FROM (SELECT " + receipt + " AS d) s", 1) == 't|yes_no|0.9|64|jev-1.13.0|1|f'
            assert query("SELECT (d.value).boolean_value IS NULL, d.answer_id IS NOT NULL "
                         "FROM thinkthen_details('Fits?','Alpha.',threshold=>'0.1:0.95') d", 1) == 't|t'
            assert query("SELECT thinkthen_details(NULL,'Alpha.',threshold=>'bad') IS NULL, "
                         "thinkthen_details('@not-read',NULL,threshold=>'bad') IS NULL") == 't|t'

            questions = [
                ('{"choose":"Which?","options":["billing","shipping"]}', 'string_value', 'billing'),
                ('{"score":"How much?","levels":["low","high"]}', 'number_value', '0.1'),
                ('{"tag":"Which?","labels":["billing","shipping"]}', 'array_value', '{billing,shipping}'),
            ]
            for question, field, answer in questions:
                receipt = "thinkthen_details(" + quoted(question) + ",'Alpha.')"
                assert query("SELECT (d.value)." + field + ", (d.probabilities).kind, "
                             "cardinality(((d.probabilities).value).array_value), d.input IS NULL, "
                             "d.inputs IS NULL FROM " + receipt + " d", 1) == answer + '|named|2|t|t'
            for statement, state, message in [
                ("SELECT thinkthen_details('Fits?','private-original',threshold=>'bad')", '22023', 'threshold'),
                ("SELECT thinkthen_details('@missing-question','private-original')", '58030', 'question'),
            ]:
                before = int(backend.read('count'))
                out, err = sql(host.socket, statement)
                assert state in err and message in err and 'private-original' not in err, (out, err)
                assert int(backend.read('count')) == before
            query("CREATE ROLE receipt_reader; GRANT EXECUTE ON FUNCTION " + signature + " TO receipt_reader")
            before = int(backend.read('count'))
            out, err = sql(host.socket, "SET ROLE receipt_reader", "SELECT thinkthen_details('@missing-question','private-original')")
            assert '22023' in err and 'needs pg_read_server_files' in err and 'private-original' not in err, (out, err)
            assert int(backend.read('count')) == before

            record = "thinkthen_record(text=>'Alpha.')"
            before = int(backend.read('count'))
            out, err = sql(host.socket, "SELECT thinkthen_reading(proxy=>'{}'::json)")
            assert '42883' in err and 'does not exist' in err, (out, err)
            assert int(backend.read('count')) == before

            query("SELECT count(*) FROM thinkthen_decide(NULL,ARRAY[NULL]::thinkthen_record[],thinkthen_reading(batch=>'bad'))")
            query("SELECT count(*) FROM thinkthen_decide('@not-read',NULL::thinkthen_record[],thinkthen_reading(batch=>'bad'))")
            failed = json.loads(query("SELECT to_jsonb(r) FROM thinkthen_decide('Fits?',ARRAY[" + record + ",NULL]) r"))
            assert failed['failure']['kind'] == 'usage' and failed['facts'] is None, failed
            assert 'Alpha.' not in failed['failure']['message'], failed
            failed = json.loads(query("SELECT to_jsonb(r) FROM thinkthen_decide('Fits?',ARRAY[thinkthen_record(text=>'private-reader-row',read_error=>ROW('local','Reader stopped.')::thinkthen_reader_failure)]) r"))
            assert failed['failure']['kind'] == 'usage' and failed['facts'] is None, failed
            assert failed['failure']['message'] == 'a terminal reader error has no record fields', failed
            assert 'private-reader-row' not in json.dumps(failed), failed

            rows = [json.loads(row) for row in query("SELECT to_jsonb(r) FROM thinkthen_decide('Fits?',ARRAY[" + record +
                         ",NULL],thinkthen_reading(incremental=>true,attempts=>true)) r", 1).splitlines()]
            assert rows[0]['value'] is True and rows[0]['ordinal'] == 0, rows
            assert rows[0]['result']['input'] == 'Alpha.', rows
            assert rows[-1]['failure']['stopped']['at'] == 2, rows
            assert rows[-1]['facts']['requests_sent'] == 1 and len(rows[-1]['facts']['attempts']) == 1, rows
            assert rows[-1]['failure']['kind'] == 'usage', rows
            row = json.loads(query("SELECT to_jsonb(r) FROM thinkthen_decide('Fits?',ARRAY[" + record +
                             "],thinkthen_reading(context=>'',threshold=>'0.95',attempts=>true)) r", 1))
            assert row['value'] is False and row['facts']['requests_sent'] == 1, row
            assert row['result']['threshold']['number_value'] == 0.95, row
            assert len(row['facts']['call_id']) == 64 and row['result']['meta']['requests_sent'] == 1, row
            assert row['observations'] and row['observations'][0]['kind'] == 'question', row
            assert query("SELECT array_append(observations,(observations)[1]) IS NOT NULL FROM thinkthen_decide('Fits?',ARRAY[" + record + "]) r", 1) == 't'
            saved = quoted(json.dumps({'decide': 'Fits?', 'threshold': 0.95}))
            row = json.loads(query("SELECT to_jsonb(r) FROM thinkthen_decide(" + saved + ",ARRAY[thinkthen_record(text=>'Alpha.',context=>'')],thinkthen_reading(context=>'Saved fallback.',threshold=>'0.5')) r", 1))
            assert row['value'] is True and row['result']['threshold']['number_value'] == 0.5, row
            wire = json.loads(json.loads(backend.read('capture'))['bodies'][-1])
            assert 'Saved fallback.' not in json.dumps(wire), wire
            assert wire['questions']['q1']['instructions'] == 'The text is \"Alpha.\". Fits?', wire
            saved = quoted(json.dumps({'find': 'Fits?', 'on': '/body'}))
            item = "thinkthen_record(value=>'{\"body\":\"Alpha.\",\"private\":\"not submitted\"}'::json)"
            row = json.loads(query("SELECT to_jsonb(r) FROM thinkthen_find(" + saved + ",ARRAY[" + item + "," + item + "]) r", 1))
            assert row['index'] == 0, row
            wire = json.loads(json.loads(backend.read('capture'))['bodies'][-1])
            assert 'not submitted' not in json.dumps(wire), wire

            row = json.loads(query("SELECT to_jsonb(r) FROM thinkthen_choose('Which team owns this?',ARRAY[" + record +
                             "],thinkthen_reading(options=>ARRAY[ROW('billing',NULL)::thinkthen_option,ROW('shipping',NULL)::thinkthen_option])) r", 1))
            assert row['value'] == 'billing' and row['result']['answer']['confidence'] == 0.9, row
            assert row['result']['answer']['probability'] is None, row
            assert {entry['name']: entry['value'] for entry in row['result']['answer']['probabilities']} == {'billing': 0.9, 'shipping': 0.1}, row
            assert row['result']['question']['options'] == ['billing', 'shipping'], row
            row = json.loads(query("SELECT to_jsonb(r) FROM thinkthen_decide('Fits?',ARRAY[" + record +
                             "],thinkthen_reading(threshold=>'0.1:0.95')) r", 1))
            assert row['value'] is None and row['result']['value_present'] is True, row
            event_value = row['observations'][0]['detail']['value']
            assert event_value is not None and all(value is None for value in event_value.values()), row

            assert query("SELECT (thinkthen_record(value=>'null'::json)).value IS NULL") == 'f'
            assert query("SELECT (thinkthen_record()).value IS NULL") == 't'
            row = json.loads(query("SELECT to_jsonb(r) FROM thinkthen_decide('Fits?',ARRAY[thinkthen_record(value=>'null'::json)]) r", 1))
            assert row['result']['input'] is None and row['result']['input_present'] is True, row
            assert query("SELECT record_count FROM thinkthen_plan('Fits?',ARRAY[" + record + "]) ") == '1'
            members = "ARRAY[ROW('first',NULL)::thinkthen_option,ROW('second',NULL)::thinkthen_option]"
            plans = [('decide', '', 1), ('filter', '', 1), ('rank', '', 1),
                     ('choose', ',reading=>thinkthen_reading(options=>' + members + ')', 1),
                     ('score', ',reading=>thinkthen_reading(levels=>' + members + ')', 1),
                     ('tag', ',reading=>thinkthen_reading(labels=>' + members + ')', 2)]
            for function, reading, requests in plans:
                question = json.dumps({'decide': 'Fits?'}) if function == 'rank' else 'Fits?'
                row = json.loads(query("SELECT to_jsonb(r) FROM thinkthen_plan(" + quoted(question) + ",ARRAY[" + record + "]" + reading + ",function=>" + quoted(function) + ") r"))
                plan = row['result']
                assert set(plan) == {'records', 'requests', 'estimated_bytes', 'estimated_input_tokens', 'first_body_utf8', 'largest_request_bytes', 'largest_request_estimated_input_tokens', 'token_estimate_method', 'upper_bound'}, plan
                assert plan['records'] == 1 and plan['requests'] == requests, plan
                assert plan['estimated_bytes'] >= plan['largest_request_bytes'] > 0, plan
                assert plan['largest_request_estimated_input_tokens'] > 0 and plan['token_estimate_method'], plan
                assert set(plan['estimated_input_tokens']) == {'lower', 'upper'}, plan
                assert plan['estimated_input_tokens']['upper'] >= plan['estimated_input_tokens']['lower'] > 0, plan
                assert isinstance(plan['upper_bound'], bool), plan
                body = json.loads(plan['first_body_utf8'])
                assert body['model'] == 'jev-1.13.0' and body['questions'], plan
                assert 'Alpha.' in json.dumps(body), plan
            unsupported = {
                'find': json.dumps({'find': 'Fits?'}),
                'annotate': json.dumps({'version': 1, 'questions': {'fit': {'decide': 'Fits?'}}}),
                'recognize': json.dumps({'version': 1, 'recognize': {'kinds': {'person': "A person's name."}}}),
                'relate': json.dumps({'version': 1, 'relate': {'relations': [{'name': 'knows', 'source': 'person', 'target': 'person'}]}}),
                'wrong': 'Fits?',
                'rank': 'Fits?',
            }
            for function, question in unsupported.items():
                before = int(backend.read('count'))
                out, err = sql(host.socket, "SELECT * FROM thinkthen_plan(" + quoted(question) + ",ARRAY[" + record + "],function=>" + quoted(function) + ")")
                expected = 'unknown function' if function == 'wrong' else 'rank requires saved decide, score or question set JSON' if function == 'rank' else 'plan requires an atomic or rank question'
                assert '22023' in err and expected in err, (function, out, err)
                assert int(backend.read('count')) == before
            query("SELECT count(*) FROM thinkthen_decide('Fits?',ARRAY[" + record +
                  "],thinkthen_reading(max_requests_total=>0)) r")
            source = "ROW('caller-read.txt',1,1)::thinkthen_source"
            row = json.loads(query("SELECT to_jsonb(r) FROM thinkthen_decide('Fits?',ARRAY[thinkthen_record(text=>'Alpha.',source=>" + source + ")]) r", 1))
            assert row['result']['source']['file'] == 'caller-read.txt', row
            assert row['result']['source']['first_line'] == row['result']['source']['last_line'] == 1, row
            sourced = "thinkthen_record(text=>'Alpha.',source=>" + source + ")"
            failed = json.loads(query("SELECT to_jsonb(r) FROM thinkthen_filter('Fits?',ARRAY[" + sourced + "," + record + "],thinkthen_reading(files_only=>true)) r"))
            assert failed['failure']['kind'] == 'usage' and failed['facts'] is None, failed
            assert failed['failure']['message'] == 'files_only requires filter source input', failed
            same_file = "thinkthen_record(text=>'Beta.',source=>ROW('caller-read.txt',2,2)::thinkthen_source)"
            other = "thinkthen_record(text=>'Gamma.',source=>ROW('other.txt',1,1)::thinkthen_source)"
            rows = [json.loads(row) for row in query("SELECT to_jsonb(r) FROM thinkthen_filter('Fits?',ARRAY[" + sourced + "," + same_file + "," + other + "],thinkthen_reading(files_only=>true,attempts=>true)) r", 3).splitlines()]
            assert [row['result']['source']['file'] for row in rows] == ['caller-read.txt', 'other.txt'], rows
            assert [row['value'] for row in rows] == ['Alpha.', 'Gamma.'], rows
            assert all(row['facts']['records'] == 3 and row['facts']['requests_sent'] == 3 for row in rows), rows
            assert all(len(row['facts']['attempts']) == 3 for row in rows), rows
            rows = [json.loads(row) for row in query("SELECT to_jsonb(r) FROM thinkthen_filter('Fits?',ARRAY[" + sourced + "," + record + "],thinkthen_reading(files_only=>true,incremental=>true)) r", 1).splitlines()]
            assert rows[0]['result']['source']['file'] == 'caller-read.txt', rows
            assert rows[-1]['failure']['kind'] == 'usage' and rows[-1]['facts']['requests_sent'] == 1, rows
            host.start({**env, 'THINKTHEN_BASE_URL': f'http://127.0.0.1:{backend.port}/arm/refuse/v1'})
            before = int(backend.read('count'))
            out, err = sql(host.socket, "SET thinkthen.max_retries=0", "SET thinkthen.cache='off'",
                           "SELECT thinkthen_details('Fits?','private-original')")
            assert '38000' in err and 'private-original' not in err, (out, err)
            assert int(backend.read('count')) - before == 1
            print('postgresql: native typed records, readings, facts, prefix, NULL, source and no-send plan pass')
        finally:
            host.stop()
            backend.close()


if __name__ == '__main__':
    main()
