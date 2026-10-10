"""Discover descriptions through PostgreSQL's installed function catalog."""

import json
import sys

from find_cases import sql


DISCOVERY = """
SELECT p.oid::regprocedure::text AS signature,
       pg_catalog.obj_description(p.oid, 'pg_proc') AS description
FROM pg_catalog.pg_proc p
JOIN pg_catalog.pg_depend d
  ON d.objid = p.oid AND d.classid = 'pg_catalog.pg_proc'::regclass
JOIN pg_catalog.pg_extension e
  ON e.oid = d.refobjid AND d.refclassid = 'pg_catalog.pg_extension'::regclass
WHERE e.extname = 'thinkthen' AND d.deptype = 'e'
ORDER BY signature
"""


def main(socket):
    out, err = sql(socket, 'SELECT json_agg(found) FROM (' + DISCOVERY + ') found')
    assert not err, err
    rows = json.loads(out)
    assert rows, 'the installed extension owns no functions'
    for row in rows:
        assert row['description'] and row['description'].strip(), row
    def description(signature):
        found = [row['description'] for row in rows if row['signature'] == signature]
        assert len(found) == 1, (signature, found)
        return found[0]
    image = description('thinkthen_decide_images_complete(text,thinkthen_image_value[],text)')
    assert all(word in image for word in ('bytea', 'NULL', 'client', 'error envelopes')), image
    removed = description('thinkthen_decide(text,text[])')
    assert all(word in removed for word in ('Removed', 'Usage', 'NULL', 'sends nothing')), removed
    loader = description('thinkthen_question_file(text)')
    assert all(word in loader for word in ('literal', 'privilege', 'confinement', 'NULL')), loader
    # PostgreSQL exposes the same comment through pg_description and obj_description.
    out, err = sql(socket, "SELECT count(*) FROM (" + DISCOVERY + ") found "
                   "JOIN pg_catalog.pg_description d ON d.objoid=found.signature::regprocedure "
                   "AND d.classoid='pg_catalog.pg_proc'::regclass AND d.objsubid=0 "
                   "WHERE d.description IS DISTINCT FROM found.description")
    assert not err and out == '0', (out, err)
    print('postgresql: every installed function and overload has a catalog description')


if __name__ == '__main__':
    main(sys.argv[1])
