"""Installed details retain typed fields, failures, later rows and no-send refusals."""
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
sys.path.insert(0, str(ROOT / 'databases/sqlite/tests'))
from conditional_backend import ConditionalBackend


def main():
    with tempfile.TemporaryDirectory(prefix='thinkthen-pg-native-') as tmp:
        home = Path(tmp)
        env = child_env(home=home, THINKTHEN_API_KEY='sk-native-sql-loopback')
        target = Path(os.environ.get('CARGO_TARGET_DIR', ROOT / 'target'))
        backend = Backend(target / 'debug/conformance-backend', env)
        host = PostgresqlHost()
        try:
            host.start({**env, 'THINKTHEN_BASE_URL': f'http://127.0.0.1:{backend.port}/arm/full/v1'})

            def query(statement, sends=0):
                before = int(backend.read('count'))
                out, err = sql(host.socket, "SET thinkthen.cache='off'", "SET thinkthen.batch='1'",
                               "SET thinkthen.model='jev-1.13.0'", "SET thinkthen.max_retries=0", statement)
                assert not err, (out, err)
                assert int(backend.read('count')) - before == sends, (out, sends)
                assert 'sk-native-sql-loopback' not in out + err, (out, err)
                return out

            def rows(statement, sends=0):
                return [json.loads(row) for row in query(statement, sends).splitlines()]

            record = "thinkthen_record(text=>'Alpha.')"
            checks = [
                ('decide', 'Fits?', record, True, .9, 1),
                ('choose', '{"choose":"Which?","options":["billing","shipping"]}', record, 'billing', .9, 1),
                ('tag', '{"tag":"Which?","labels":["billing","shipping"]}', record, ['billing', 'shipping'], None, 1),
                ('score', '{"score":"How much?","levels":["low","high"]}', record, .1, None, 1),
                ('filter', 'Fits?', record, 'Alpha.', .9, 1),
                ('rank', '{"decide":"Fits?"}', record, 1, .9, 1),
                ('find', '{"find":"Fits?"}', record + ",thinkthen_record(text=>'Beta.')", 'Alpha.', .9, 1),
                ('annotate', '{"version":1,"questions":{"fit":{"decide":"Fits?"}}}',
                 "thinkthen_record(value=>'{\"body\":\"Alpha.\",\"untouched\":false}'::json)", None, None, 1),
                ('recognize', '{"version":1,"recognize":{"kinds":{"person":"A person."}}}',
                 record, None, None, None),
                ('relate', '{"version":1,"relate":{"relations":[{"name":"works_for","source":"person","target":"organization"}]}}',
                 "thinkthen_record(value=>'{\"name\":\"Alice\",\"kind\":\"person\"}'::json),thinkthen_record(value=>'{\"name\":\"Acme\",\"kind\":\"organization\"}'::json)", None, None, None),
            ]
            for function, question, item, value, probability, sends in checks:
                before = int(backend.read('count'))
                statement = 'SELECT to_jsonb(r) FROM thinkthen_' + function + '_details(' + quoted(question) + ',ARRAY[' + item + ']) r'
                # Recognition and relation request counts follow their actual staged readings.
                if sends is None:
                    out, err = sql(host.socket, "SET thinkthen.cache='off'", "SET thinkthen.model='jev-1.13.0'", statement)
                    assert not err, (function, out, err)
                    sends = int(backend.read('count')) - before
                    got = [json.loads(row) for row in out.splitlines()]
                else:
                    got = rows(statement, sends)
                assert len(got) == 1 and got[0]['ordinal'] == 0, (function, got)
                row = got[0]
                assert set(row) == {'ordinal', 'value', 'probability', 'result', 'failure', 'facts', 'observations'}, row
                assert row['failure'] is None and row['facts']['requests_sent'] == sends, (function, row)
                assert row['result'] and row['observations'], (function, row)
                if value is not None:
                    assert row['value'] == value, (function, row)
                if function == 'recognize':
                    assert isinstance(row['probability'], list), (function, row)
                elif function == 'relate':
                    assert row['value']['source_name'] == 'Alice' and row['value']['target_name'] == 'Acme', row
                    assert row['probability'] == .9, row
                    assert len(row['result']['value']) == 1, row
                else:
                    assert row['probability'] == probability, (function, row)

            for function in ('decide', 'rank'):
                for incremental in ('false', 'true'):
                    failed = rows("SELECT to_jsonb(r) FROM thinkthen_" + function + "_details(" + quoted('{"decide":"Fits?"}') + ",ARRAY[" + record + ",NULL," + record + "],thinkthen_reading(incremental=>" + incremental + ")) r")
                    assert len(failed) == 1 and failed[0]['failure']['kind'] == 'usage', failed
                    assert failed[0]['failure']['message'] == 'record list contains NULL', failed
                    assert failed[0]['facts'] is None and failed[0]['observations'] == [], failed
                    assert 'Alpha.' not in failed[0]['failure']['message'], failed
            with ConditionalBackend(f'http://127.0.0.1:{backend.port}/arm/full/v1') as proxy:
                host.start({**env, 'THINKTHEN_BASE_URL': proxy.base})
                failed = "thinkthen_record(text=>'private evidence')"
                got = rows("SELECT to_jsonb(r) FROM thinkthen_decide_details('Fits?',ARRAY[" + record + "," + failed + ",thinkthen_record(text=>'Beta.')]) r", 2)
                assert proxy.count() == 3, got
                assert [r['ordinal'] for r in got] == [0, 1, 2], got
                assert [r['value'] for r in got] == [True, None, True], got
                assert got[1]['failure']['kind'] == 'backend', got
                assert 'private evidence' not in got[1]['failure']['message'], got
                assert all(row['facts']['requests_sent'] == 3 for row in got), got
            host.start({**env, 'THINKTHEN_BASE_URL': f'http://127.0.0.1:{backend.port}/arm/full/v1'})
            got = rows("SELECT to_jsonb(r) FROM thinkthen_decide_details('Fits?',ARRAY[" + record + "],thinkthen_reading(threshold=>'0.1:0.95')) r", 1)
            assert got[0]['value'] is None and got[0]['probability'] == .9, got
            source = "ROW('caller-read.txt',1,1)::thinkthen_source"
            got = rows("SELECT to_jsonb(r) FROM thinkthen_decide_details('Fits?',ARRAY[thinkthen_record(text=>'Alpha.',source=>" + source + ")]) r", 1)
            assert got[0]['result']['source']['file'] == 'caller-read.txt', got
            nullable = rows("SELECT to_jsonb(r) FROM thinkthen_decide_details('Fits?',ARRAY[thinkthen_record(value=>'null'::json)]) r", 1)
            assert nullable[0]['result']['input'] is None and nullable[0]['result']['input_present'] is True, nullable
            host.start({**env, 'THINKTHEN_BASE_URL': f'http://127.0.0.1:{backend.port}/arm/full/capture/v1'})
            selected = "thinkthen_record(value=>'{\"body\":\"Alpha.\",\"private\":\"not submitted\"}'::json,context=>'')"
            declared = quoted(json.dumps({'decide': 'Fits?', 'item_schema': {'type': 'string'}}))
            located = rows("SELECT to_jsonb(r) FROM thinkthen_decide_details(" + declared + ",ARRAY[" + selected + "],thinkthen_reading(field=>ARRAY['/body'],context=>'Saved fallback.')) r", 1)
            assert located[0]['result']['input']['private'] == 'not submitted', located
            wire = json.loads(json.loads(backend.read('capture'))['bodies'][-1])
            assert 'not submitted' not in json.dumps(wire) and 'Saved fallback.' not in json.dumps(wire), wire
            assert query("SELECT count(*) FROM thinkthen_decide_details(NULL,ARRAY[NULL]::thinkthen_record[],thinkthen_reading(batch=>'bad'))") == '0'
            assert query("SELECT count(*) FROM thinkthen_decide_details('@not-read',NULL::thinkthen_record[],thinkthen_reading(batch=>'bad'))") == '0'
            assert query("SELECT count(*) FROM pg_proc WHERE proname IN ('thinkthen_details','thinkthen_try_details','thinkthen_details_images') OR proname LIKE 'thinkthen_%_complete' OR proname LIKE 'thinkthen_native_records_%'") == '0'
            for statement in ("SELECT thinkthen_details('Fits?','Alpha.')",
                              "SELECT thinkthen_try_details('Fits?','Alpha.')",
                              "SELECT thinkthen_decide('Fits?',ARRAY[" + record + "])"):
                before = int(backend.read('count'))
                out, err = sql(host.socket, statement)
                assert '42883' in err and 'does not exist' in err, (out, err)
                assert int(backend.read('count')) == before
            for reading, expected in (("cancelled=>true", '57014'), ("deadline_ms=>0", '57014')):
                before = int(backend.read('count'))
                out, err = sql(host.socket, "SELECT * FROM thinkthen_decide_details('Fits?',ARRAY[" + record + "],thinkthen_reading(" + reading + "))")
                assert expected in err and 'Alpha.' not in err, (out, err)
                assert int(backend.read('count')) == before
            print('postgresql: all ten details, typed probabilities, later rows, NULL, secrecy and removed exports pass')
        finally:
            host.stop()
            backend.close()


if __name__ == '__main__':
    main()
