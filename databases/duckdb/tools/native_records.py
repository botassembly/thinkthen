"""Details retain typed row values, ordered originals and counted sends."""
from harness import Backend, expect, rows


def table(verb, question, records, reading='NULL', settings='NULL'):
    return f'(SELECT detail.* FROM (SELECT UNNEST(thinkthen_{verb}_details({question},{records},reading := {reading},settings := {settings})) AS detail))'


def check(run):
    with Backend() as backend:
        got = run([
            "SET thinkthen_model='jev-1.13.0'",
            "SELECT ordinal,value,probability,result.value,facts.records,facts.requests_sent,failure.kind FROM " +
            table('decide', "{'decide':'Refund?','true':{'meaning':'refund'},'false':{'meaning':'other'}}",
                  "[{'text':'alpha'},{'text':'beta'},{'text':'alpha'}]", "{'batch':2,'attempts':true}") + ' ORDER BY ordinal',
            "SELECT d.value,d.probability,d.result.value,d.facts.requests_sent FROM "
            "(SELECT thinkthen_choose_details('Which team?','alpha',['billing','shipping']) AS d)",
            "SELECT thinkthen_decide_details(NULL,'alpha',settings := 'bad') IS NULL",
            "SELECT thinkthen_decide_details('Refund?',NULL,settings := 'bad') IS NULL",
            "SELECT thinkthen_decide_details('Refund?','alpha',settings := {'deadline_ms':0})",
        ], backend.base())
        expect(rows(got[1]), [[0, True, 0.9, '{"meaning":"refund"}',3,1,None],
                              [1, True, 0.9, '{"meaning":"refund"}',3,1,None],
                              [2, True, 0.9, '{"meaning":"refund"}',3,1,None]], 'ordered typed decision rows')
        expect(rows(got[2]), [['billing',0.9,'billing',1]], 'ordinary typed choice')
        expect(rows(got[3]), [[True]], 'required question NULL shields settings')
        expect(rows(got[4]), [[True]], 'required input NULL shields settings')
        expect('error' in got[5], True, 'deadline raises')
        expect(backend.count(), 2, 'two admitted calls send; deadline and NULLs send nothing')
    ordinary_forms(run)
    removed(run)



def ordinary_forms(run):
    from native_queries import value
    question_set = "{'version':1,'questions':{'refund':{'decide':'Refund?'}}}"
    query = value("SELECT 1 AS id, 'Ada' AS name UNION ALL SELECT 2, 'Bob'")
    with Backend() as backend:
        got = run([
            "SELECT d.value,d.probability,d.failure.kind FROM (SELECT thinkthen_score_details('Strength?','alpha',['weak','strong']) AS d)",
            "SELECT d.value,d.probability,d.failure.kind FROM (SELECT thinkthen_tag_details('Topics?','alpha',['billing','shipping']) AS d)",
            "SELECT d.value,d.probability,d.failure.kind FROM (SELECT thinkthen_filter_details('Refund?','alpha') AS d)",
            "SELECT d.value,d.probability,d.failure.kind FROM (SELECT thinkthen_find_details('Find?',['alpha','beta']) AS d)",
            "SELECT d.ordinal,d.value,d.probability,d.failure.kind FROM (SELECT UNNEST(thinkthen_rank_details('Refund?',{'a':'alpha','b':'beta'})) AS d)",
            "SELECT d.value[1].value.boolean_value,d.probability,d.failure.kind FROM (SELECT thinkthen_annotate_details("+question_set+",'alpha') AS d)",
            "SELECT d.value,d.probability,d.failure.kind FROM (SELECT thinkthen_recognize_details('Ada',['person']) AS d)",
            f"SELECT d.value.source.name,d.value.target.name,d.probability,d.failure.kind FROM (SELECT UNNEST(thinkthen_relate_details({query},['near'])) AS d)",
            f"SELECT * FROM thinkthen_relate({query},['near'])",
            "SELECT thinkthen_filter('Refund?','alpha')",
        ], backend.base())
        wanted = [[[0.1,None,None]],[[['billing','shipping'],None,None]], [['"alpha"',0.9,None]],
                  [['"alpha"',0.9,None]],[[0,1,0.9,None],[1,2,0.9,None]],[[True,None,None]],
                  [[[],[],None]],[['1','2',0.9,None],['2','1',0.9,None]],
                  [['near','1','2',0.9,False],['near','2','1',0.9,False]],[['alpha']]]
        expect([rows(item) for item in got], wanted, 'ordinary typed details across the function family')
        expect(backend.count(), 7, 'ordinary details share actual cache identity with plain calls')

def removed(run):
    with Backend() as backend:
        names = ['thinkthen_details','thinkthen_try_details','thinkthen_details_images']
        names += [f'thinkthen_{verb}_complete' for verb in ('decide','choose','tag','score','filter','rank','find','annotate','recognize','relate')]
        names += [f'thinkthen_native_records_{verb}' for verb in ('decide','choose','tag','score','filter','rank','find','annotate','recognize','relate')]
        got = run([f"SELECT {name}('question','input')" for name in names], backend.base())
        expect(all('does not exist' in item.get('error','') for item in got), True, 'retired names are absent')
        expect(backend.count(), 0, 'retired functions send nothing')
