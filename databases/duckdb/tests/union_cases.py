"""Keep one invocation when DuckDB splits input into several pipelines."""
import time


def unions(factory):
    # Feature: UNION inputs retain statement-wide positions, ranks and top.
    with factory() as host:
        host.configure({'backend': 'stub'})
        host.query('SET threads=8')
        source = ("SELECT range AS document_id, range::VARCHAR AS record FROM range(50000) "
                  "UNION ALL SELECT range AS document_id, range::VARCHAR AS record FROM range(50000,100000)")
        started = time.monotonic()
        counts = host.query("SELECT count(*) AS total, count(DISTINCT position) AS positions, "
                            "count(*) FILTER (WHERE status != 'answered' OR answer IS NULL OR record != document_id::VARCHAR) AS invalid "
                            "FROM thinkthen_decide((" + source + "), 'Ready?') AS answers "
                            "JOIN (SELECT range AS id FROM range(100000)) AS ids ON document_id=ids.id "
                            "GROUP BY status", timeout=120)
        assert counts == [{'total': '100000', 'positions': '100000', 'invalid': '0'}], counts
        print(f'DuckDB joined UNION 100000 rows: {time.monotonic() - started:.6f}s', flush=True)
        source = ("SELECT range AS document_id, range::VARCHAR AS record, "
                  "struct_pack(copy := range) AS original FROM range(20)")
        union = (source + ' WHERE range < 3 UNION ALL ' + source +
                 ' WHERE range >= 3 AND range < 13 UNION ALL ' + source + ' WHERE range >= 13')
        args = ", 'Order?', levels := ['low','medium','high']"
        # ORDER BY forces one pipeline. The answer must be independent of the
        # plan shape, including which branch contains the global winner.
        reference = host.query('SELECT document_id, position, answer FROM thinkthen_rank((' +
                               source + ' ORDER BY document_id)' + args + ', top := 1)')
        actual = host.query('SELECT document_id, position, answer FROM thinkthen_rank((' +
                            union + ')' + args + ', top := 1)')
        assert reference == [{'document_id': '4', 'position': '5', 'answer': '1'}], reference
        assert actual == reference, (reference, actual)
        ranked = host.query('SELECT document_id, position, answer, original.copy AS copy FROM thinkthen_rank((' +
                            union + ')' + args + ') ORDER BY answer')
        assert len(ranked) == 20 and [int(row['answer']) for row in ranked] == list(range(1, 21)), ranked
        assert all(int(row['document_id']) + 1 == int(row['position']) and row['copy'] == row['document_id']
                   for row in ranked), ranked
        # Each nonempty branch crosses DuckDB's native chunk boundary; empty
        # branches must still finish without closing the shared engine input.
        large = ("SELECT range AS document_id, range::VARCHAR AS record FROM range(8199)")
        branches = [large + ' WHERE random() < 0', large + ' WHERE range < 4099',
                    large + ' WHERE random() < 0', large + ' WHERE range >= 4099', large + ' WHERE random() < 0']
        counts = host.query('SELECT count(*) AS total, count(DISTINCT position) AS positions, '
                            'count(*) FILTER (WHERE document_id + 1 != position) AS misplaced '
                            'FROM thinkthen_decide((' + ' UNION ALL '.join(branches) + "), 'Ready?')")
        assert counts == [{'total': '8199', 'positions': '8199', 'misplaced': '0'}], counts
        expected = host.query('SELECT document_id, position, answer FROM thinkthen_rank((' +
                              large + ' ORDER BY document_id)' + args + ', top := 1)')
        actual = host.query('SELECT document_id, position, answer FROM thinkthen_rank((' +
                           ' UNION ALL '.join(branches) + ')' + args + ', top := 1)')
        assert len(expected) == 1 and actual == expected, (expected, actual)
    print('DuckDB UNION inputs keep global top, contiguous ranks and native row identity', flush=True)


def benchmark(factory):
    # Compare the existing ordered top-one control without a routine time gate.
    with factory() as host:
        host.configure({'backend': 'stub'})
        host.query('SET threads=8')
        large = "SELECT range AS document_id, range::VARCHAR AS record FROM range(8199)"
        query = ('SELECT document_id, position, answer FROM thinkthen_rank((' + large +
                 " ORDER BY document_id), 'Order?', levels := ['low','medium','high'], top := 1)")
        expected = host.query(query)
        assert len(expected) == 1 and expected[0]['answer'] == '1', expected
        assert int(expected[0]['document_id']) + 1 == int(expected[0]['position']), expected
        samples = []
        for _ in range(3):
            started = time.monotonic()
            assert host.query(query) == expected
            samples.append(time.monotonic() - started)
        print(f'DuckDB ordered top-one result={expected} seconds={samples}', flush=True)
