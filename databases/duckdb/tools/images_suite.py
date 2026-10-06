#!/usr/bin/env python3
"""Public stored DuckDB image values and rich recognition grammar."""

import json
import sys
import tempfile
from pathlib import Path
from harness import Backend, case, expect, main, rows, run

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
            f"SELECT thinkthen_details_images({literal(ASK['choose'])}, {LIST}, 'Compare originals.')"], backend.base,
            extra={'THINKTHEN_CACHE': folder + '/cache', 'LIQUIDAI_API_KEY':'fake-sql-image-key', 'THINKTHEN_BACKEND':'liquid'})
        expect([rows(result) for result in first[-4:-1]], [[[True]], [['red']], [[0.8]]], 'three image scalar values')
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


if __name__ == '__main__':
    raise SystemExit(main())
