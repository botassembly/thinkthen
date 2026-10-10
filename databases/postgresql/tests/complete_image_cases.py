"""Installed complete image calls preserve native bytea, errors and replay."""
import json
import os
import sys
import tempfile
from pathlib import Path
from find_cases import sql, quoted

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'databases/sqlite/tests'))
from image_backend import ImageBackend, FIXTURE, expected_body
sys.path.insert(0, str(ROOT / 'databases/sqlite/tests/complete'))
from postgresql_host import PostgresqlHost
sys.path.insert(0, str(ROOT / 'conformance/children'))
from children import child_env


def main():
    with tempfile.TemporaryDirectory(prefix='thinkthen-pg-images-') as tmp, ImageBackend() as backend:
        home = Path(tmp)
        env = child_env(keep=('LANG', 'LC_ALL', 'LD_LIBRARY_PATH'), home=home,
                        THINKTHEN_BACKEND='liquid', LIQUIDAI_API_KEY='sk-sql-image-loopback',
                        THINKTHEN_BASE_URL=backend.base)
        host = PostgresqlHost()
        settings = json.dumps({'record': str(home / 'record'), 'cache': False, 'max_retries': 0})
        images = ','.join("thinkthen_image(decode('" + (FIXTURE / name).read_bytes().hex() +
                          "','hex'),'image/png')" for name in ('red.png', 'blue.png', 'red.png'))
        try:
            host.start(env)
            def query(statement):
                out, err = sql(host.socket, "SET thinkthen.model='d1'", statement)
                assert not err, (out, err)
                return out
            query('CREATE TABLE images(ordinal integer, image thinkthen_image_value)')
            query('INSERT INTO images SELECT n, (ARRAY[' + images + '])[n] FROM generate_series(1,3) n')
            source = '(SELECT array_agg(image ORDER BY ordinal) FROM images)'
            def complete(options):
                controls = {'record': '', 'replay': '', 'max_requests_total': -1, 'max_retries': 0}
                controls.update(json.loads(options) if options is not None else {})
                controls['cache'] = str(home / 'cache') if controls.get('cache', True) else 'off'
                statements = ['SET thinkthen.' + key + '=' + quoted(str(value)) for key, value in controls.items()]
                out, err = sql(host.socket, "SET thinkthen.model='d1'", *statements,
                               "SELECT thinkthen_decide_images_complete('Is red visible?'," + source + ',NULL)')
                assert not err, (out, err)
                return json.loads(out)
            completed = complete(settings)
            assert completed['ordinals'] == [0], completed
            got = completed['native']
            assert 'value' in got, got
            assert got['value'][0]['value'] is True, got
            assert got['facts']['requests_sent'] == 1 and backend.bodies == [expected_body('decide', text='')], got
            replay = complete(json.dumps({'replay': str(home / 'record'), 'cache': False, 'max_requests_total': 0}))['native']
            assert 'facts' in replay, replay
            assert replay['facts']['requests_sent'] == 0 and backend.count() == 1, replay
            assert [(r['value'], r['answer_id'], r['input']) for r in replay['value']] == [(r['value'], r['answer_id'], r['input']) for r in got['value']]
            for statement in ["SELECT thinkthen_decide_images_complete(NULL,ARRAY[NULL]::thinkthen_image_value[],'bad') IS NULL",
                              "SELECT thinkthen_decide_images_complete('@unread-question.json',NULL,'bad') IS NULL"]:
                assert query(statement) == 't'
            for value in ('ARRAY[]::thinkthen_image_value[]', 'ARRAY[NULL]::thinkthen_image_value[]',
                          "ARRAY[ROW('image/png','private-image-evidence'::bytea,NULL)::thinkthen_image_value]"):
                failed = json.loads(query("SELECT thinkthen_decide_images_complete('Is red visible?'," + value + ')'))
                assert failed['native']['error']['kind'] == 'usage' and 'private-image-evidence' not in json.dumps(failed), failed
            assert backend.count() == 1
            bad_text = json.loads(query("SELECT thinkthen_decide_complete('Is red visible?','bad')"))
            assert bad_text['native']['error']['kind'] == 'usage', bad_text
            first = complete(None)['native']
            cached = complete(None)['native']
            assert first['value'][0]['value'] is True and cached['value'][0]['value'] is True
            assert first['facts']['requests_sent'] == 1 and cached['facts']['requests_sent'] == 0, (first, cached)
            assert backend.count() == 2
            assert query("SELECT thinkthen_decide_images('Is red visible?'," + source + ",'Compare originals.')") == 't'
            assert backend.count() == 3 and backend.bodies[-1] == expected_body('decide')
            out, err = sql(host.socket, "SET thinkthen.model='d1'",
                           "SELECT thinkthen_decide_images('Is red visible?',ARRAY[NULL]::thinkthen_image_value[])")
            assert '22023' in err and 'image list contains NULL' in err, (out, err)
            backend.status, backend.reply = 400, b'private-backend-response'
            failed = complete(json.dumps({'cache': False, 'max_retries': 0}))
            assert failed['native']['error']['kind'] == 'backend' and failed['native']['facts']['requests_sent'] == 1, failed
            assert 'private-backend-response' not in json.dumps(failed) and 'sk-sql-image-loopback' not in json.dumps(failed)
            assert backend.count() == 4
            print('postgresql: native complete image order, replay, NULL, malformed input and secret failure pass')
        finally:
            host.stop()


if __name__ == '__main__':
    main()
