#!/usr/bin/env python3
"""Public composite storage/reload and loopback image judgments in PostgreSQL."""

import json
import pathlib
import sys
from find_cases import sql, quoted

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[2] / 'sqlite/tests'))
from image_backend import FIXTURE, expected_body


def statements(create):
    images = []
    for name in ('red.png', 'blue.png', 'red.png'):
        data = (FIXTURE / name).read_bytes().hex()
        images.append(f"thinkthen_image(decode('{data}','hex'),'image/png')")
    return ["SET thinkthen.model='d1'",
            *(['CREATE TABLE images(ordinal integer, image thinkthen_image_value)',
               *[f'INSERT INTO images VALUES({index},{value})' for index, value in enumerate(images)]] if create else []),
            "SELECT thinkthen_decide_images('Is red visible?', (SELECT array_agg(image ORDER BY ordinal) FROM images), 'Compare originals.')",
            'SELECT thinkthen_choose_images(' + quoted('{"choose":"Which color?","options":["red","blue"]}') + ", (SELECT array_agg(image ORDER BY ordinal) FROM images), 'Compare originals.')",
            'SELECT thinkthen_score_images(' + quoted('{"score":"How red?","levels":["none","all"]}') + ", (SELECT array_agg(image ORDER BY ordinal) FROM images), 'Compare originals.')",
            'SELECT thinkthen_details_images(' + quoted('{"choose":"Which color?","options":["red","blue"]}') + ", (SELECT array_agg(image ORDER BY ordinal) FROM images), 'Compare originals.')->'answer'->'probabilities'",
            'SELECT (image).file IS NULL FROM images ORDER BY ordinal']


def invalid(socket):
    nulls = ["SELECT thinkthen_image(NULL,'image/png') IS NULL", "SELECT thinkthen_image(''::bytea,NULL) IS NULL",
             'SELECT thinkthen_decide_images(NULL,NULL) IS NULL', "SELECT thinkthen_choose_images('Which?',NULL) IS NULL",
             "SELECT thinkthen_score_images('How?',NULL) IS NULL"]
    out, err = sql(socket, *nulls)
    assert not err and out.splitlines() == ['t'] * 5, (out, err)
    red = "thinkthen_image(decode('" + (FIXTURE / 'red.png').read_bytes().hex() + "','hex'),'image/png')"
    for statement in ["SELECT thinkthen_image('private-image-evidence'::bytea,'image/png')",
                      "SELECT thinkthen_image('private-image-evidence'::bytea,'image/gif')",
                      "SELECT thinkthen_image(repeat('x',25165825)::bytea,'image/png')",
                      "WITH padded AS (SELECT thinkthen_image(decode('" + (FIXTURE / 'red.png').read_bytes().hex() + "','hex') || repeat('x',13631488)::bytea,'image/png') AS image) SELECT thinkthen_decide_images('Is red visible?',ARRAY[image,image]) FROM padded",
                      "SELECT thinkthen_decide_images('Is red visible?',ARRAY[]::thinkthen_image_value[])",
                      "SELECT thinkthen_decide_images('Is red visible?',ARRAY[NULL]::thinkthen_image_value[])",
                      "SELECT thinkthen_decide_images('Is red visible?',ARRAY[" + ','.join([red]*9) + '])',
                      "SELECT thinkthen_decide_images('Is red visible?',ARRAY[ROW('image/png','private-image-evidence'::bytea,NULL)::thinkthen_image_value])",
                      "SELECT thinkthen_details_images('{\"tag\":\"Which?\",\"labels\":[\"red\"]}',ARRAY[" + red + '])']:
        out, err = sql(socket, "SET thinkthen.model='d1'", statement)
        assert '22023' in err and 'thinkthen usage:' in err, (out, err)
        assert 'private-image-evidence' not in err, err
    out, err = sql(socket, "SELECT thinkthen_decide_images('Is red visible?','private-image-evidence'::bytea)")
    assert '42883' in err, (out, err)
    out, err = sql(socket, "SELECT has_function_privilege('public','thinkthen_image(bytea,text)','EXECUTE')")
    assert not err and out == 'f', (out, err)


def verify(socket, create):
    out, err = sql(socket, *statements(create))
    lines = out.splitlines()
    assert not err and lines[:3] == ['t', 'red', '0.8'], (out, err)
    assert json.loads(lines[3]) == {'red': 0.8, 'blue': 0.2}, out
    assert lines[4:] == ['t'] * 3, out


if __name__ == '__main__':
    mode, path = sys.argv[1:3]
    if mode == 'invalid':
        invalid(path)
    elif mode in ('verify', 'replay'):
        verify(path, mode == 'verify')
    elif mode == 'bodies':
        bodies = json.loads(pathlib.Path(path).read_text())
        assert bodies == [expected_body(verb) for verb in ('decide','choose','score')], bodies
    else:
        raise SystemExit('unknown image case mode')
