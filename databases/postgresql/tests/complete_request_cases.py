"""Installed PostgreSQL typed fields retain shared Request admission and prefixes."""
import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'databases/sqlite/tests/complete'))
from parity import Backend, execute
from postgresql_host import PostgresqlHost
from c_parity import ERRORS
from project import project
sys.path.insert(0, str(ROOT / 'conformance/children'))
from children import child_env


def main():
    with tempfile.TemporaryDirectory(prefix='thinkthen-pg-request-') as tmp:
        home = Path(tmp)
        env = child_env(keep=('LANG', 'LC_ALL', 'LD_LIBRARY_PATH'), home=home,
                        THINKTHEN_API_KEY='sk-conformance-loopback')
        backend = Backend(ROOT / 'target/debug/conformance-backend', env)
        host = PostgresqlHost()
        base = f'http://127.0.0.1:{backend.port}/generic/v1'
        try:
            host.start({**env, 'THINKTHEN_BASE_URL': base})
            def call(verb, question, inputs, sends, client_read=True):
                before = int(backend.read('count'))
                frame = {'verb': verb, 'question': json.dumps(question),
                         'inputs': inputs, 'controls': {}, 'held_cancel': False,
                         'engine_settings': {'base_url': base, 'cache': False,
                                             'model': 'jev-1.13.0', 'batch': 1,
                                             'max_retries': 0}}
                if client_read:
                    got = execute('postgresql', frame, env, home, backend)
                else:
                    child = subprocess.run(
                        [sys.executable, str(ROOT / 'databases/sqlite/tests/complete/postgresql_child.py'), host.socket],
                        input=json.dumps(frame) + '\n', env=env, cwd=home,
                        text=True, capture_output=True, timeout=120, check=True)
                    assert not child.stderr, child.stderr
                    got = project(json.loads(child.stdout), verb)
                assert int(backend.read('count')) - before == sends, got
                assert got.get('requests_sent', 0) == sends, got
                return got

            question = {'decide': 'Fits?', 'item_schema': {
                'type': 'object', 'properties': {'body': {'type': 'string'}},
                'required': ['body']}}
            got = call('decide', question, {'records': [
                {'json': {'body': 'Alpha.'}}, {'text': 'secret-original'}]}, 0)
            assert got['code'] == ERRORS['usage'] and got['stopped_at'] == 2, got
            assert 'secret-original' not in got['message'], got

            for invalid in ({'text': 'Beta.', 'images': None},
                            {'text': 'Beta.', 'images': 'invalid'}, None):
                for incremental in (False, True):
                    got = call('decide', {'decide': 'Fits?'}, {'records': [
                        {'text': 'Alpha.'}, invalid], 'incremental': incremental},
                        int(incremental))
                    assert got['code'] == ERRORS['usage'], got
                    assert len(got['completed']) == int(incremental), got
                    assert got.get('stopped_at') == 2, got
                    if incremental:
                        assert got['completed'][0]['index'] == 0, got
                        assert got['completed'][0]['input'] == 'Alpha.', got

            question = {'version': 1, 'questions': {'body': {
                'decide': 'Fits?', 'on': '/body', 'item_schema': {'type': 'string'}}}}
            got = call('annotate', question, {'records': [
                {'json': {'body': 'Alpha.', 'untouched': False}}]}, 1)
            assert got['rows'][0]['index'] == 0, got
            assert got['rows'][0]['input']['untouched'] is False, got
            got = call('annotate', question, {'records': [
                {'json': {'body': 'Alpha.'}}, {'json': {'body': 42}}]}, 0)
            assert got['code'] == ERRORS['usage'] and got['stopped_at'] == 2, got

            for threshold in (0.5, 0.95):
                for incremental in (False, True):
                    records = [{'text': 'Alpha.'}]
                    if incremental:
                        records.append({'text': 'Beta.', 'images': None})
                    got = call('filter', {'decide': 'Fits?', 'threshold': threshold},
                               {'records': records, 'incremental': incremental}, 1)
                    rows = got['completed'] if incremental else got['rows']
                    assert len(rows) == 1 and rows[0]['value'] is (threshold == 0.5), got
                    assert rows[0]['index'] == 0 and rows[0]['input'] == 'Alpha.', got

            got = call('decide', {'decide': 'Fits?'},
                       {'files': {'paths': ['/unreadable-server-evidence']}}, 0, client_read=False)
            assert got['code'] == ERRORS['usage'] and 'facts' not in got, got
            assert 'client-read records' in got['message'], got
            for inputs in ({'records': [], 'incremental': None},
                           {'records': [], 'unknown': True}):
                got = call('decide', {'decide': 'Fits?'}, inputs, 0)
                assert got['code'] == ERRORS['usage'] and 'facts' not in got, got
            assert int(backend.read('count')) == 8
            print('postgresql: typed declaration, projection, eager/prefix, false filter and file-authority cases pass')
        finally:
            host.stop()
            backend.close()


if __name__ == '__main__':
    main()
