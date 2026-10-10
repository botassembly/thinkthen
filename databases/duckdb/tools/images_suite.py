#!/usr/bin/env python3
"""Public stored DuckDB image values and rich recognition grammar."""

import json
import sys
import tempfile
from pathlib import Path
from harness import Backend, case, expect, main, rows, run, said

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / 'sqlite/tests'))
from image_backend import ImageBackend, FIXTURE, expected_body


def literal(text):
    return "'" + text.replace("'", "''") + "'"


ASK = {'decide': 'Is red visible?', 'choose': '{"choose":"Which color?","options":["red","blue"]}',
       'score': '{"score":"How red?","levels":["none","all"]}'}
LIST = '(SELECT list(image ORDER BY ordinal) FROM saved.images)'


def stored(folder, create=False):
    sql = [f"ATTACH {literal(folder + '/images.db')} AS saved"]
    if create:
        sql += ['CREATE TABLE saved.images(ordinal INTEGER, image STRUCT(media VARCHAR, data BLOB, file VARCHAR))']
        for index, name in enumerate(('red.png', 'blue.png', 'red.png')):
            data = (FIXTURE / name).read_bytes().hex()
            sql += [f"INSERT INTO saved.images VALUES({index}, thinkthen_image(from_hex('{data}'), 'image/png'))"]
    return sql


@case
def stored_images_keep_order_duplicates_native_answers_and_zero_send_replay():
    with ImageBackend() as backend, tempfile.TemporaryDirectory() as folder:
        first = run([*stored(folder, True), "SET thinkthen_model = 'd1'",
                     f"SET thinkthen_record = {literal(folder + '/record')}", *[
            f"SELECT thinkthen_{verb}_images({literal(question)}, {LIST}, 'Compare originals.')"
            for verb, question in ASK.items()],
            f"SELECT d, typeof(d), json_type(d) FROM (SELECT thinkthen_details_images({literal(ASK['choose'])}, {LIST}, 'Compare originals.') AS d)"], backend.base,
            extra={'THINKTHEN_CACHE': folder + '/cache', 'LIQUIDAI_API_KEY':'fake-sql-image-key', 'THINKTHEN_BACKEND':'liquid'})
        expect([rows(result) for result in first[-4:-1]], [[[True]], [['red']], [[0.8]]], 'three image scalar values')
        expect(rows(first[-1])[0][1:], ['JSON','OBJECT'], 'native image details JSON operations')
        detail = json.loads(rows(first[-1])[0][0])
        expect(detail['answer']['probabilities'], {'red': 0.8, 'blue': 0.2}, 'native complete probabilities')
        expect(backend.bodies, [expected_body(verb) for verb in (*ASK, 'choose')], 'independent native image request bodies')
        replay = run([*stored(folder), "SET thinkthen_model = 'd1'",
                      "SET thinkthen_cache = 'off'", "SET thinkthen_max_requests_total = 0",
                      f"SET thinkthen_replay = {literal(folder + '/record')}", *[
            f"SELECT thinkthen_{verb}_images({literal(question)}, {LIST}, 'Compare originals.')"
            for verb, question in ASK.items()]], backend.base, keyless=True, extra={'THINKTHEN_BACKEND':'liquid'})
        expect([rows(result) for result in replay[-3:]], [[[True]], [['red']], [[0.8]]], 'reloaded stored images and keyless capped strict replay')
        expect(backend.count(), 4, 'strict replay sends nothing')


@case
def complete_images_keep_native_blobs_facts_replay_and_local_refusals():
    with ImageBackend() as backend, tempfile.TemporaryDirectory() as folder:
        extra = {'THINKTHEN_BACKEND':'liquid', 'LIQUIDAI_API_KEY':'fake-sql-image-key'}
        call = f"SELECT d, typeof(d), d->>'$.native.value[0].value' FROM (SELECT thinkthen_decide_complete('Is red visible?', {LIST}) AS d)"
        got = run([*stored(folder, True), "SET thinkthen_model='d1'",
                   f"SET thinkthen_record={literal(folder + '/record')}", call], backend.base, extra=extra)
        expect(rows(got[-1])[0][1:], ['JSON','true'], 'native image complete JSON operations')
        value = json.loads(rows(got[-1])[0][0])
        expect(value['ordinals'], [0], 'one native image record')
        expect(value['native']['value'][0]['value'], True, 'complete native decision')
        expect(value['native']['facts']['requests_sent'], 1, 'one complete request')
        expect(backend.bodies, [expected_body('decide', text='')], 'authored image order and duplicates')
        got = run([*stored(folder), "SET thinkthen_model='d1'", "SET thinkthen_cache='off'",
                   "SET thinkthen_max_requests_total=0", f"SET thinkthen_replay={literal(folder + '/record')}", call],
                  backend.base, keyless=True, extra={'THINKTHEN_BACKEND':'liquid'})
        replay = json.loads(rows(got[-1])[0][0])
        expect([(r['value'],r['answer_id'],r['input']) for r in replay['native']['value']],
               [(r['value'],r['answer_id'],r['input']) for r in value['native']['value']], 'strict replay identity')
        expect(replay['native']['facts']['requests_sent'], 0, 'strict replay sends nothing')
        bad = "[{media:'image/png',data:'private-image-evidence'::BLOB,file:NULL}]"
        missing, trace = folder + "/unread-question.json", Path(folder) / "null.trace"
        wrap = ['strace','-f','-e','trace=openat,newfstatat,statx,access,readlink','-o',str(trace)] if sys.platform == 'linux' else None
        nulls = [f"SELECT thinkthen_decide_complete(NULL, {partner}, 'bad') IS NULL" for partner in
                 (bad, "'{bad}'", "[]::STRUCT(media VARCHAR,data BLOB,file VARCHAR)[]")]
        nulls += [f"SELECT thinkthen_decide_complete({literal('@' + missing)}, {partner}, 'bad') IS NULL"
                  for partner in ("NULL::VARCHAR", "NULL::STRUCT(media VARCHAR,data BLOB,file VARCHAR)[]")]
        got = run(["SET thinkthen_refresh_cache=2", *nulls,
                   "SELECT thinkthen_decide_complete('Is red visible?', '{bad}')"], backend.base, extra=extra)
        expect([rows(r) for r in got[1:-1]], [[[True]]] * len(nulls), 'required NULL precedes malformed session settings')
        expect('thinkthen_refresh_cache is 0 or 1' in got[-1].get('error', ''), True, 'live rows still validate session settings')
        probe = folder + '/null-record/.probe'
        got = run([f"SET thinkthen_record={literal(folder + '/null-record')}", *nulls], backend.base, extra=extra, wrap=wrap)
        expect([rows(r) for r in got[1:]], [[[True]]] * len(nulls), 'required NULL skips recording settings')
        if wrap:
            expect(sum(f'"{probe}"' in line for line in trace.read_text().splitlines()), 0, 'NULL rows never probe recording folder')
        got = run([f"SELECT thinkthen_decide_complete(NULL, {bad}, 'bad') IS NULL",
                   f"SELECT thinkthen_decide_complete({literal('@' + missing)}, NULL, 'bad') IS NULL",
                   f"SELECT thinkthen_decide_complete('Is red visible?', {bad})",
                   "SELECT thinkthen_decide_complete('Is red visible?', []::STRUCT(media VARCHAR,data BLOB,file VARCHAR)[])",
                   "SELECT thinkthen_decide_complete('Is red visible?', '{bad}')"], backend.base, extra=extra, wrap=wrap)
        expect([rows(r) for r in got[:2]], [[[True]],[[True]]], 'required NULL skips malformed partners')
        for result in got[2:]:
            envelope = json.loads(rows(result)[0][0])
            expect(envelope['native']['error']['kind'], 'usage', 'malformed native or text input refuses')
            expect('private-image-evidence' in json.dumps(envelope), False, 'admission withholds pixels')
        if wrap:
            calls = trace.read_text().splitlines()
            expect(bool(calls), True, 'file tracer observes installed consumer')
            expect(sum(f'"{missing}"' in line for line in calls), 0, 'NULL inputs never inspect question file')
        expect(backend.count(), 1, 'replay and local refusals send nothing')
        backend.status, backend.reply = 503, b'{"error":"private-backend-body"}'
        failed = run([*stored(folder), "SET thinkthen_model='d1'", "SET thinkthen_max_retries=0", call], backend.base, extra=extra)
        envelope = json.loads(rows(failed[-1])[0][0])
        expect(envelope['native']['error']['kind'], 'backend', 'backend failure preserves complete kind')
        expect('private-backend-body' in json.dumps(envelope), False, 'failure withholds response')
        expect(backend.count(), 2, 'one failed attempt after the successful request')


@case
def image_null_bad_media_limits_implicit_values_and_file_permissions_never_send():
    with ImageBackend() as backend:
        red = (FIXTURE / 'red.png').read_bytes().hex()
        value = f"thinkthen_image(from_hex('{red}'), 'image/png')"
        good = f'[{value}]'
        statements = ["SELECT thinkthen_image(NULL, 'image/png') IS NULL", "SELECT thinkthen_image(''::BLOB, NULL) IS NULL",
                      "SELECT thinkthen_decide_images(NULL, NULL) IS NULL",
                      "SELECT thinkthen_choose_images('Which?', NULL) IS NULL", "SELECT thinkthen_score_images('How?', NULL) IS NULL"]
        invalid = ["SELECT thinkthen_image('private-image-evidence'::BLOB, 'image/png')",
                   "SELECT thinkthen_image('private-image-evidence'::BLOB, 'image/gif')",
                   "SELECT thinkthen_image(repeat('x',25165825)::BLOB, 'image/png')",
                   f"WITH padded AS (SELECT thinkthen_image(from_hex('{red}') || repeat('x',13631488)::BLOB,'image/png') AS image) SELECT thinkthen_decide_images('Is red visible?', [image,image]) FROM padded",
                   "SELECT thinkthen_decide_images('Is red visible?', [])",
                   "SELECT thinkthen_decide_images('Is red visible?', [NULL])",
                   f"SELECT thinkthen_decide_images('Is red visible?', [{','.join([value]*9)}])",
                   "SELECT thinkthen_decide_images('Is red visible?', [{media:'image/png',data:'private-image-evidence'::BLOB,file:NULL}])",
                   f"SELECT thinkthen_details_images('{{\"tag\":\"Which?\",\"labels\":[\"red\"]}}', {good})",
                   f"SELECT thinkthen_decide_images('Is red visible?', from_hex('{red}'))"]
        got = run([*statements, *invalid, f"SELECT (thinkthen_image_file({literal(str(FIXTURE/'red.png'))})).file",
                   'SET enable_external_access = false', f"SELECT thinkthen_image_file({literal(str(FIXTURE/'red.png'))})"], backend.base)
        expect([rows(result) for result in got[:5]], [[[True]]] * 5, 'NULL operands')
        for result in got[5:5+len(invalid)]:
            expect('error' in result, True, 'invalid input refuses')
            # DuckDB's binder can show SQL source, so payload secrecy checks
            # target native diagnostics, whose first sentence withholds pixels.
            if 'thinkthen usage:' in result['error']:
                message = result['error'].split('\n')[0]
                expect('private-image-evidence' in message, False, 'native media error withholds bytes')
        expect(rows(got[-3]), [[str(FIXTURE/'red.png')]], 'file identity survives explicit native reader')
        expect('error' in got[-1], True, 'host external-access permission is enforced')
        expect(backend.count(), 0, 'all local refusals send nothing')


@case
def described_recognize_kinds_use_native_grammar_and_keep_list_names():
    spec = '{"version":1,"recognize":{"kinds":{"Person":"A human","Place":"A locality"}}}'
    with Backend() as backend:
        got = run([f"SELECT thinkthen_recognize('Maria Chen in Paris', {literal(spec)})",
                   "SELECT thinkthen_recognize('Maria Chen in Paris', ['Person','Place'])"], backend.base('arm/full/capture'))
        expect('rows' in got[0] and 'rows' in got[1], True, 'described kinds and retained name list execute')
        bodies = [json.loads(body) for body in backend.capture()]
        criteria = [question.get('criteria') for body in bodies for question in body['questions'].values()]
        expect({'Person':'A human','Place':'A locality','none of these':'They are not a proper name, or no listed kind covers what they name.'} in criteria, True, 'authored descriptions reach native wire criteria')


@case
def rich_recognize_invalid_arrays_mixed_and_duplicates_send_nothing():
    invalid = ('{"recognize":{"kinds":{"Person":"human","Person":"duplicate"}}}',
               '{"recognize":{"kinds":["Person",{"Place":"where"}]}}', '["Person"]', '[broken')
    with Backend() as backend:
        got = run([f"SELECT thinkthen_recognize('private evidence', {literal(source)})" for source in invalid], backend.base())
        for result in got:
            expect('thinkthen usage:' in result.get('error', ''), True, 'native grammar refusal')
        expect(backend.count(), 0, 'invalid rich recognize never sends')


@case
def null_recognize_evidence_skips_constant_rich_kinds_and_file_io():
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        missing = str(Path(folder) / 'unreadable-kinds.json')
        trace = Path(folder) / 'file-io.trace'
        invalid = ('[broken', '["Person"]',
                   '{"recognize":{"kinds":["Person",{"Place":"where"}]}}',
                   '{"recognize":{"kinds":{"Person":"human","Person":"duplicate"}}}')
        statements = [f"SELECT thinkthen_recognize(NULL, {literal(source)})" for source in invalid]
        statements += [f"SELECT thinkthen_recognize(NULL, {literal('@' + missing)})",
                       "SET enable_external_access = false",
                       f"SELECT thinkthen_recognize(NULL, {literal('@' + missing)})",
                       f"SELECT thinkthen_recognize(input, {literal('[broken')}) FROM (VALUES (NULL::VARCHAR),(NULL::VARCHAR)) t(input)"]
        # Reuse the existing public-consumer tracer on Linux; other hosts keep
        # the same NULL and permission regressions without a Linux syscall tool.
        wrap = ['strace', '-f', '-e', 'trace=openat,newfstatat,statx,access,readlink', '-o', str(trace)] if sys.platform == 'linux' else None
        got = run(statements, backend.base(), keyless=True,
                  extra={'THINKTHEN_MAX_REQUESTS_TOTAL':'0'}, wrap=wrap)
        expect([rows(result) for result in got[:5]], [[[None]]] * 5, 'NULL evidence skips constant invalid grammar and missing kinds file')
        expect(rows(got[-2]), [[None]], 'NULL evidence skips host file permission checks')
        expect(rows(got[-1]), [[None],[None]], 'each NULL row skips constant rich kinds validation')
        if wrap:
            calls = trace.read_text().splitlines()
            expect(bool(calls), True, 'existing tracer observed the public consumer')
            expect(sum(f'"{missing}"' in line for line in calls), 0, 'NULL kinds file is never opened or inspected')
        refused = run([f"SELECT thinkthen_recognize('evidence', {literal('@' + missing)})"],
                      backend.base(), keyless=True, extra={'THINKTHEN_MAX_REQUESTS_TOTAL':'0'})
        expect(said(refused[0]), f'thinkthen local: the question file {missing} was not read: it does not exist or could not be opened (retryable: no)', 'non-NULL evidence still requires readable rich kinds')
        expect(backend.count(), 0, 'NULL evidence and unreadable non-NULL kinds send nothing')


if __name__ == '__main__':
    raise SystemExit(main())
