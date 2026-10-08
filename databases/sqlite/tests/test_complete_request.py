"""Installed named complete calls retain shared eager declaration and seed admission."""
from helper import Backend, child, environment, expect, main


def test_eager_complete_refuses_late_declared_items_and_recognition_controls_without_sends():
    backend = Backend()
    got = child('''
db = connect()
cases = [
    ('decide', {'decide':'Fits?','item_schema':{'type':'object','properties':{'body':{'type':'string'}},'required':['body']}},
     {'records':[{'json':{'body':'Alpha.'}},{'text':'secret-original'}]}),
    ('recognize', {}, {'records':[{'text':'Alpha.'},{'text':'Beta.','seed_spans':[{'start':1,'end':3}]}]}),
    ('recognize', {}, {'records':[{'text':'Alpha.'},{'text':'Beta.','examples':['[Ada | secret-kind]']}]}),
    ('recognize', {}, {'records':[{'text':'Alpha.','seed_spans':None}]}),
    ('recognize', {}, {'records':[{'text':'Alpha.','examples':None}]}),
    ('recognize', {}, {'records':[{'json':{'body':'Alpha.','seeds':None}}],
                       'reading':{'fields':['/body'],'seed_spans':'/seeds'}}),
]
results=[]
for verb, question, inputs in cases:
    value=db.execute('SELECT thinkthen_'+verb+'_complete(?,?)',
        [json.dumps(question),json.dumps(inputs)]).fetchone()[0]
    document=json.loads(value)
    results.append(document['native']['error']['kind'])
    assert 'secret' not in value
for inputs in ({'records':[], 'incremental':None}, {'records':[], 'unknown':True}):
    value=db.execute('SELECT thinkthen_decide_complete(?,?)',
        ['@missing-saved-question.json',json.dumps(inputs)]).fetchone()[0]
    assert json.loads(value)['native']['error']['kind']=='usage', value
say(results=results)
''', environment(backend))
    expect(got['results'], ['usage'] * 6, 'shared complete admission')
    expect(backend.close(), 0, 'eager invalid complete calls send nothing')


if __name__ == '__main__':
    raise SystemExit(main(globals()))
