"""Exercise cursor ownership, source identifiers and connection snapshots."""
from concurrent.futures import ThreadPoolExecutor
from contextlib import closing
from threading import Barrier

import psycopg2


def verify(database, query, failure):
    # Feature: bounded source chunks retain each explicit native identifier.
    source = "SELECT n AS key, CASE WHEN n % 127=0 THEN NULL ELSE 'evidence ' || n END::text AS body FROM generate_series(1,1025) n ORDER BY n"
    rows = query(database, "SELECT position,id,status,error_code FROM thinkthen.decide(%s,'Ready?',record_column=>'body',id_column=>'key')", (source,))
    assert len(rows) == 1025
    assert rows == [(n, n, 'failed' if n % 127 == 0 else 'answered', 'record.missing' if n % 127 == 0 else None) for n in range(1,1026)], rows
    ranked = query(database, "SELECT position,id,status,rank FROM thinkthen.rank(%s,'Urgent?',ARRAY['low','high'],top=>1,record_column=>'body',id_column=>'key')", (source,))
    assert len(ranked) == 9 and ranked[0][2:] == ('answered',1), ranked
    assert all(position == identifier for position,identifier,_,_ in ranked)
    assert [row[0] for row in ranked[1:]] == list(range(127,1026,127)), ranked
    # Feature: selected JSON identifiers keep every JSON class and exact numeric bytes.
    identifiers = ['1e1000', '123456789012345678901234567890.123456789',
                   'true', 'null', '"alpha"', '[1e1000,"alpha"]', '{"number":1e1000}']
    for identifier in identifiers:
        source = "SELECT '" + identifier + "'::json AS key,'alpha'::text AS record"
        rows = query(database, "SELECT position,id IS NOT DISTINCT FROM %s::jsonb,status,error_code FROM thinkthen.decide(%s,'Ready?',id_column=>'key')", (identifier, source))
        assert rows == [(1, True, 'answered', None)], (identifier, rows)
    source = "SELECT 1e1000::numeric AS key,'alpha'::text AS record"
    assert query(database, "SELECT position,id = '1e1000'::jsonb,status,error_code FROM thinkthen.decide(%s,'Ready?',id_column=>'key')", (source,)) == [(1, True, 'answered', None)]
    # Feature: nullable composite rule options use the shared defaults.
    source = "SELECT 'Alice works at Acme'::text AS record"
    statements = ["ROW('works at','person','company',NULL,NULL)",
                  "ROW('works at','person','company',NULL,false)"]
    results = [query(database, "SELECT position,status,answer,detail,(source).question_identity FROM thinkthen.relate(%s,rules=>ARRAY[" + rule + "]::thinkthen.rule[]) t", (source,)) for rule in statements]
    assert len(results[0]) == 1 and results[0][0][1] == 'answered', results
    assert results[0] == results[1], results
    # Feature: escaped zero bytes in JSON evidence do not require output conversion.
    source = "SELECT '{\"body\":\"\\u0000\"}'::json AS record"
    rows = query(database, "SELECT position,id,status,error_code FROM thinkthen.decide(%s,'Ready?')", (source,))
    assert rows == [(1, None, 'answered', None)], rows
    # Feature: host representation failures remain failed rows with source and position.
    for source, selected in [("SELECT '\"\\u0000\"'::json AS key,'alpha'::text AS record", True),
                             ("SELECT '[1e1000,\"\\u0000\"]'::json AS key,'alpha'::text AS record", True),
                             ("SELECT '{\"\\u0000\":1e1000}'::json AS key,'alpha'::text AS record", True),
                             ("SELECT '{\"x\":\"\\u0000\",\"x\":1e1000}'::json AS key,'alpha'::text AS record", True)]:
        rows = query(database, "SELECT position,id,status,answer,probability,error_code,(source).backend FROM thinkthen.decide(%s,'Ready?',id_column=>%s)", (source, 'key' if selected else None))
        assert rows == [(1,None,'failed',None,None,'record.json','stub')], rows
    # Feature: a literal execution name changes the engine's identity.
    unnamed = query(database, "SELECT (source).question_identity FROM thinkthen.decide('SELECT ''alpha'' AS record','Ready?')")
    named = query(database, "SELECT (source).question_identity FROM thinkthen.decide('SELECT ''alpha'' AS record','Ready?',name=>'review')")
    assert named != unnamed
    failure(database, "SELECT * FROM thinkthen.annotate('SELECT ''alpha'' AS record','{\"bad\":{\"function\":\"rank\",\"question\":\"Urgent?\",\"levels\":[\"low\",\"high\"]}}'::jsonb)", '22023', 'usage.question')
    # Feature: separate connections retain independent settings and source cursors.
    barrier = Barrier(2)

    def independent(allowed):
        with closing(psycopg2.connect(database.dsn)) as connection:
            connection.autocommit = True
            query(connection, "SET thinkthen.allowed_backends = %s", ('stub:*' if allowed else '',))
            query(connection, "SET thinkthen.backend = 'stub'")
            barrier.wait(timeout=30)
            if allowed:
                result = query(connection, "SELECT count(*) FROM thinkthen.decide(%s,'Ready?',record_column=>'body')", (source,))
                assert result == [(1025,)], result
            else:
                failure(connection, "SELECT * FROM thinkthen.decide('SELECT ''alpha'' AS record','Ready?')", '42501', 'usage.sql-selection')
            assert query(connection, 'SELECT count(*) FROM pg_cursors') == [(0,)]

    # The representation loop above used a separate local source string.
    source = "SELECT n::text AS body FROM generate_series(1,1025) n"
    with ThreadPoolExecutor(max_workers=2) as workers:
        list(workers.map(independent, [True, False]))
    query(database, 'BEGIN')
    query(database, "SET LOCAL thinkthen.allowed_backends = ''")
    query(database, 'SAVEPOINT policy')
    query(database, "SET LOCAL thinkthen.allowed_backends = 'stub:*'")
    assert len(query(database, "SELECT * FROM thinkthen.decide('SELECT ''alpha'' AS record','Ready?')")) == 1
    query(database, 'ROLLBACK TO SAVEPOINT policy')
    failure(database, "SELECT * FROM thinkthen.decide('SELECT ''alpha'' AS record','Ready?')", '42501', 'usage.sql-selection')
    query(database, 'ROLLBACK')
    assert len(query(database, "SELECT * FROM thinkthen.decide('SELECT ''alpha'' AS record','Ready?')")) == 1
