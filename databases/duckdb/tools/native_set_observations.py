"""Set details preserve observations once when callers reorder SQL rows."""
from harness import Backend, expect, rows
from native_queries import value
from receipt_adapter import carrier


def check(run, json_text=True):
    duplicate_entities(run)
    source_rank(run)
    entities = " UNION ALL ".join(f"SELECT {i} AS id, 'Person {i}' AS name" for i in range(1, 5))
    with Backend() as backend:
        got = run([
            "CREATE TEMP TABLE ranked AS SELECT d.* FROM (SELECT UNNEST(thinkthen_rank_details('Refund?',{'a':'alpha','b':'beta','c':'gamma'})) AS d)",
            "SELECT ranked FROM ranked ORDER BY ordinal DESC",
            "CREATE TEMP TABLE related AS SELECT d.* FROM (SELECT UNNEST(thinkthen_relate_details(" + value(entities) + ",['near'])) AS d)",
            "SELECT related FROM related ORDER BY ordinal DESC",
        ], backend.base())
        for verb, actual, count in [('rank', rows(got[1]), 3), ('relate', rows(got[3]), 12)]:
            actual = [row[0] for row in actual]
            expect([row['ordinal'] for row in actual], list(reversed(range(count))), verb + ' keeps ordinals after reordering')
            first = next(row for row in actual if row['ordinal'] == 0)
            expect(bool(first['observations']), True, verb + ' ordinal zero keeps observations')
            expect([row['observations'] for row in actual if row['ordinal'] != 0], [[]] * (count - 1), verb + ' later rows do not repeat observations')
            ordered = sorted(actual, key=lambda row: row['ordinal'])
            events = [event for row in ordered for event in row['observations']]
            expect(events, first['observations'], verb + ' recomposition preserves every observation in order')
            expect([row['failure'] for row in actual], [None] * count, verb + ' successful rows keep no failure')
            expect([row['facts'] for row in actual], [first['facts']] * count, verb + ' keeps owning facts')
            expect(first['facts']['requests_sent'], 1, verb + ' has one send')
            expect([row['probability'] for row in actual], [0.9] * count, verb + ' keeps probabilities')
            receipt = {'results': [row['result'] for row in ordered], 'facts': first['facts'], 'failure': None,
                       'observations': events, 'ordinals': [row['ordinal'] for row in ordered]}
            restored = carrier(receipt, verb, json_text=json_text)
            expect(len(restored['observations']), len(events), verb + ' native observation count survives recomposition')
            if verb == 'rank':
                expect([(event['kind'], event['index']) for event in events],
                       [(kind, index) for index in range(3) for kind in ('question', 'row')], 'rank retains every native question and row event')
                expect([event['inputs'][0]['input'] for event in restored['observations'] if event['kind'] == 'question'],
                       ['alpha', 'beta', 'gamma'], 'rank observations retain all original inputs')
                expect([row['value'] for row in ordered], [1, 2, 3], 'rank preserves positions')
                expect([row['result']['index'] for row in ordered], [0, 1, 2], 'rank preserves original indices')
            else:
                expect([(event['kind'], event['index']) for event in events],
                       [('question', 0)] * count + [('row', 0)], 'relate retains all native question and row events')
                expect([event['position'] for event in events if event['kind'] == 'question'], list(range(count)), 'relate retains every question position')
                expect([len(event['inputs']) for event in events if event['kind'] == 'question'], [4] * count, 'relate observations retain the complete input set')
                expect([len(row['result']['value']) for row in ordered], [1] * count, 'relate results contain only their own edge')
                pairs = [(str(a), str(b)) for a in range(1, 5) for b in range(1, 5) if a != b]
                expect([(row['value']['source']['name'], row['value']['target']['name']) for row in ordered], pairs, 'relate keeps SQL edge identifiers in order')
                expect([(row['result']['value'][0]['source']['name'], row['result']['value'][0]['target']['name']) for row in ordered],
                       [('Person ' + a, 'Person ' + b) for a, b in pairs], 'relate keeps canonical entity names in order')
                expect([(row['result']['value'][0]['relation'], row['result']['value'][0]['probability']) for row in ordered],
                       [('near', 0.9)] * count, 'relate keeps canonical relation probabilities')
                expect(len(restored['native']['value']['value']), count, 'relate recomposes the full native edge set')
                expect(len(restored['native']['value']['input']), 4, 'relate preserves all original entities')
                expect(len({row['result']['answer_id'] for row in ordered}), 1, 'relate preserves native answer identity')
                for key in ('question', 'input', 'meta'):
                    expect([row['result'][key] for row in ordered], [first['result'][key]] * count, 'relate preserves original ' + key)
        expect(backend.count(), 2, 'set details make only their two native sends')


def duplicate_entities(run):
    query = value("SELECT 1 AS id, 'Ada' AS name UNION ALL SELECT 2, 'Ada' UNION ALL SELECT 3, 'Bob'")
    with Backend() as backend:
        actual = rows(run(["SELECT d.ordinal,len(d.observations),d.value.source.name,d.value.target.name "
                           "FROM (SELECT UNNEST(thinkthen_relate_details(" + query + ",['near'])) AS d) "
                           "ORDER BY d.ordinal DESC"], backend.base())[0])
        expect(actual, [[3,0,'3','2'],[2,0,'3','1'],[1,0,'2','3'],[0,3,'1','3']],
               'expanded entity identifiers retain one observation owner')
        expect(backend.count(), 1, 'expanded identifiers retain one native send')
    from receipt_adapter import row_observations
    event = {'kind':'row','index':0}
    owned = {'ordinal':0,'failure':None,'observations':[event,event]}
    expect(row_observations([owned], 'decide'), [event,event], 'native duplicate events retain their order')
    for verb, duplicate in [('decide',{**owned,'ordinal':1}),('rank',owned)]:
        try:
            row_observations([owned,duplicate], verb)
        except AssertionError:
            pass
        else:
            raise AssertionError('duplicate observation ownership must fail')


def source_rank(run):
    import tempfile
    from pathlib import Path
    from native_queries import call
    with tempfile.TemporaryDirectory(prefix='thinkthen-rank-observations-') as folder:
        path = Path(folder) / 'rows.txt'
        path.write_text('alpha\nbeta\ngamma\n')
        query = call('rank', value({'decide':'Refund?'}), {'files':{'paths':[str(path)]}})
        with Backend() as backend:
            actual = rows(run(['SELECT ordinal,observations FROM ' + query + ' ORDER BY ordinal DESC'], backend.base())[0])
            expect([row[0] for row in actual], [2,1,0], 'source rank keeps original ordinals')
            expect([row[1] for row in actual[:2]], [[],[]], 'source rank keeps sole observation ownership')
            expect([(event['kind'], event['index']) for event in actual[2][1]],
                   [(kind,index) for index in range(3) for kind in ('question','row')],
                   'source rank retains every native observation')
            expect(backend.count(), 1, 'source rank retains one packed send')
