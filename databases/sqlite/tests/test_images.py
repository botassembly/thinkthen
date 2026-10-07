#!/usr/bin/env python3
"""Public stored image judgments use native order, validation and strict replay."""

import json
import pathlib
from helper import Backend, Child, child, environment, expect, main
from image_backend import ImageBackend, FIXTURE, expected_body

ASK = {"decide": "Is red visible?", "choose": '{"choose":"Which color?","options":["red","blue"]}',
       "score": '{"score":"How red?","levels":["none","all"]}'}

STORED = f'''
db = connect(os.path.join(os.environ['SCRATCH'], 'images.sqlite'))
db.execute('CREATE TABLE stored(image BLOB)')
for name in ['red.png', 'blue.png', 'red.png']:
    data = open({str(FIXTURE)!r} + '/' + name, 'rb').read()
    db.execute('INSERT INTO stored VALUES(thinkthen_image(?, ?))', (data, 'image/png'))
db.close()
db = connect(os.path.join(os.environ['SCRATCH'], 'images.sqlite'))
images = [row[0] for row in db.execute('SELECT image FROM stored ORDER BY rowid')]
packed = db.execute('SELECT thinkthen_images(?, ?, ?)', images).fetchone()[0]
'''


def test_stored_order_duplicate_scalars_details_cache_and_replay():
    with ImageBackend() as backend:
        env = environment(None, THINKTHEN_BASE_URL=backend.base, THINKTHEN_API_KEY='fake-sql-image-key', LIQUIDAI_API_KEY='fake-sql-image-key', THINKTHEN_BACKEND='liquid')
        record = str(pathlib.Path(env['SCRATCH']) / 'record')
        code = STORED + f'''
db.execute('SELECT thinkthen_configure(?)', (json.dumps({{"model":"d1","record":{record!r}}}),))
answers = {{}}
for verb, question in {ASK!r}.items():
    answers[verb] = run(db, 'SELECT thinkthen_' + verb + '_images(?, ?, ?)', (question, packed, 'Compare originals.'))
detail = run(db, 'SELECT thinkthen_details_images(?, ?, ?)', ({ASK['choose']!r}, packed, 'Compare originals.'))
say(answers=answers, detail=detail)
'''
        got = child(code, env)
        expect(got['answers'], {'decide': [[1]], 'choose': [['red']], 'score': [[0.8]]}, 'three native scalar answers')
        detail = json.loads(got['detail'][0][0])
        expect(detail['answer']['probabilities'], {'red': 0.8, 'blue': 0.2}, 'complete probabilities')
        expect(backend.bodies, [expected_body(verb) for verb in (*ASK, 'choose')], 'independent ordered original bodies and one evaluation per public call while recording')
        replay = child(STORED.replace('images.sqlite', 'images-replay.sqlite') + f'''
db.execute('SELECT thinkthen_configure(?)', (json.dumps({{"model":"d1","cache":False,"replay":{record!r}}}),))
say(values=[run(db, 'SELECT thinkthen_' + verb + '_images(?, ?, ?)', (question, packed, 'Compare originals.')) for verb, question in {ASK!r}.items()])
''', {**env, 'THINKTHEN_MAX_REQUESTS_TOTAL':'0'})
        expect(replay['values'], [[[1]], [['red']], [[0.8]]], 'strict replay answers with a zero send cap')
        miss = child(STORED.replace('images.sqlite', 'images-miss.sqlite') + f'''
db.execute('SELECT thinkthen_configure(?)', (json.dumps({{"model":"d1","cache":False,"replay":{record!r}}}),))
say(error=run(db, 'SELECT thinkthen_decide_images(?, ?, ?)', ('A different question?', packed, 'Compare originals.')))
''', {**env, 'THINKTHEN_MAX_REQUESTS_TOTAL':'0'})
        expect(miss['error'], 'thinkthen local: the replay folder holds no answer for this question (retryable: no)', 'strict miss never becomes unsure')
        expect(backend.count(), 4, 'replay and misses send nothing')


def test_null_bad_tags_media_pixels_count_and_implicit_blob_send_nothing():
    with ImageBackend() as backend:
        env = environment(None, THINKTHEN_BASE_URL=backend.base, THINKTHEN_API_KEY='fake-sql-image-key', LIQUIDAI_API_KEY='fake-sql-image-key', THINKTHEN_BACKEND='liquid')
        got = child(STORED + '''
db.execute('SELECT thinkthen_configure(?)', ('{"model":"d1","cache":false}',))
nulls = [run(db, sql) for sql in ['SELECT thinkthen_image(NULL, "image/png")',
 "SELECT thinkthen_image(x'00', NULL)", "SELECT thinkthen_decide_images(NULL, x'00')",
 'SELECT thinkthen_choose_images("Which?", NULL)', 'SELECT thinkthen_score_images("How?", NULL)']]
bad = [run(db, sql, args) for sql, args in [
 ('SELECT thinkthen_image(?, ?)', (b'private-image-evidence', 'image/png')),
 ('SELECT thinkthen_image(?, ?)', (b'private-image-evidence', 'image/gif')),
 ('SELECT thinkthen_image(zeroblob(25165825), ?)', ('image/png',)),
 ('WITH padded(image) AS (SELECT thinkthen_image(?, ?)) SELECT thinkthen_images(image, image) FROM padded',
  (open(''' + repr(str(FIXTURE / 'red.png')) + ''', 'rb').read() + b'x' * (13*1024*1024), 'image/png')),
 ('SELECT thinkthen_images()', ()), ('SELECT thinkthen_images(NULL)', ()),
 ('SELECT thinkthen_images(?,?,?,?,?,?,?,?,?)', tuple(images[0] for _ in range(9))),
 ('SELECT thinkthen_decide_images(?, ?)', ('Is red visible?', b'private-image-evidence')),
 ('SELECT thinkthen_decide_images(?, ?)', ('Is red visible?', packed + b'x')),
 ('SELECT thinkthen_details_images(?, ?)', ('{"tag":"Which?","labels":["red"]}', packed)),
 ('SELECT thinkthen_decide_images(?, ?, NULL, ?)', ('Is red visible?', packed, '{"model":"unadmitted"}')),
 ('SELECT thinkthen_tag(?, ?)', ('{"tag":"Which?","labels":["red"]}', images[0])),
 ('SELECT thinkthen_decide(?, ?)', ('Is red visible?', images[0]))]]
file = db.execute('SELECT thinkthen_image_file(?)', (''' + repr(str(FIXTURE / 'red.png')) + ''',)).fetchone()[0]
say(nulls=nulls,bad=bad,file=run(db,'SELECT thinkthen_image_file_name(?)',(file,)))
''', env)
        expect(got['nulls'], [[[None]]] * 5, 'NULL operands do not inspect invalid partners')
        assert all(isinstance(error, str) and 'thinkthen usage:' in error for error in got['bad']), got['bad']
        assert 'private-image-evidence' not in json.dumps(got)
        expect(got['file'], [[str(FIXTURE / 'red.png')]], 'native image source retains file without text positions')
        expect(backend.count(), 0, 'local invalid values and NULLs send nothing')


def test_array_looking_recognize_kinds_never_send():
    backend = Backend()
    got = child('''db = connect()
say(errors=[run(db, 'SELECT * FROM thinkthen_recognize(?, ?)', ('text', kinds))
 for kinds in ['["Person"]', ' [broken', '[{"Person":"description"}]', '["Person",{"Place":"where"}]']])
''', environment(backend))
    expected = 'thinkthen usage: recognize kinds take comma names or a recognize JSON object, not a JSON array (retryable: no)'
    expect(got['errors'], [expected] * 4, 'exact grammar refusal')
    expect(backend.close(), 0, 'array grammar refuses without a send')


def test_rich_recognize_duplicate_keys_refuse_without_laundering_or_send():
    backend = Backend()
    got = child('''db = connect()
source = '{"recognize":{"kinds":{"Person":"human","Person":"duplicate"}}}'
path = os.path.join(os.environ['SCRATCH'], 'duplicate.json')
open(path, 'w').write(source)
say(errors=[run(db, 'SELECT * FROM thinkthen_recognize(?, ?)', ('text', kinds))
 for kinds in [source, '@' + path]])
''', environment(backend))
    assert got['errors'][0].startswith('thinkthen usage:'), got
    assert got['errors'][1].startswith('thinkthen local:'), got
    expect(backend.close(), 0, 'inline and file duplicate keys refuse without a send')


def test_stored_images_reuse_native_cache_without_a_second_judgment():
    with ImageBackend() as backend:
        env = environment(None, THINKTHEN_BASE_URL=backend.base, THINKTHEN_BACKEND='liquid', LIQUIDAI_API_KEY='fake-sql-image-key')
        got = child(STORED + '''
db.execute('SELECT thinkthen_configure(?)', ('{"model":"d1"}',))
say(results=[run(db,'SELECT thinkthen_decide_images(?, ?, ?)',('Is red visible?',packed,'Compare originals.')) for _ in range(2)])
''', env)
        expect(got['results'], [[[1]], [[1]]], 'cached scalar remains the native answer')
        expect(backend.bodies, [expected_body('decide')], 'one evaluation then one native image cache hit')


def test_image_interrupt_and_native_backend_failure_keep_their_error_kind():
    for verb in ASK:
        with ImageBackend() as backend:
            backend.hold = True
            env = environment(None, THINKTHEN_BASE_URL=backend.base, THINKTHEN_BACKEND='liquid', LIQUIDAI_API_KEY='fake-sql-image-key')
            held = Child(STORED + f'''
db.execute('SELECT thinkthen_configure(?)', ('{{"model":"d1","cache":false}}',))
result = {{}}
def ask():
    result['answer'] = run(db, 'SELECT thinkthen_{verb}_images(?, ?, ?)', ({ASK[verb]!r}, packed, 'Compare originals.'))
thread = threading.Thread(target=ask)
thread.start()
sys.stdin.readline()
db.interrupt()
thread.join(timeout=5)
say(result=result)
''', env)
            assert backend.arrived.wait(timeout=10), 'image call never reached owned listener'
            held.process.stdin.write('\n')
            held.process.stdin.flush()
            got = held.result()
            expect(got['result']['answer'], 'thinkthen cancelled: the call was cancelled (retryable: no)', verb + ' interrupt before held reply')
            expect(backend.count(), 1, 'one held image send and no retry')
    with ImageBackend() as backend:
        backend.status = 503
        backend.reply = b'{"error":"private-backend-body"}'
        env = environment(None, THINKTHEN_BASE_URL=backend.base, THINKTHEN_BACKEND='liquid', LIQUIDAI_API_KEY='fake-sql-image-key')
        got = child(STORED + '''
db.execute('SELECT thinkthen_configure(?)', ('{"model":"d1","cache":false,"max_retries":0}',))
say(error=run(db,'SELECT thinkthen_decide_images(?,?)',('Is red visible?', packed)))
''', env)
        expect(got['error'], 'thinkthen backend: the backend answered with status 503 (retryable: yes)', 'backend error is not NULL or a leaked body')
        expect(backend.count(), 1, 'one failing image attempt')


if __name__ == '__main__':
    raise SystemExit(main(globals()))
