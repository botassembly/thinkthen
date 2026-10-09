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
    if verb=='decide': assert document['native']['error']['stopped']['at']==2, document
    assert 'secret' not in value
for inputs in ({'records':[], 'incremental':None}, {'records':[], 'unknown':True}):
    value=db.execute('SELECT thinkthen_decide_complete(?,?)',
        ['@missing-saved-question.json',json.dumps(inputs)]).fetchone()[0]
    assert json.loads(value)['native']['error']['kind']=='usage', value
say(results=results)
''', environment(backend))
    expect(got['results'], ['usage'] * 6, 'shared complete admission')
    expect(backend.close(), 0, 'eager invalid complete calls send nothing')


def test_incremental_complete_retains_prefix_before_malformed_image_descriptors():
    backend = Backend()
    got = child('''
db = connect()
db.execute('SELECT thinkthen_configure(?)', [json.dumps({'cache':False,'batch':1})])
for invalid in ({'text':'Beta.','images':None}, {'text':'Beta.','images':'invalid'}, None):
    for incremental in (False, True):
        inputs={'records':[{'text':'Alpha.'},invalid], 'incremental':incremental}
        document=json.loads(db.execute('SELECT thinkthen_decide_complete(?,?)',
            [json.dumps({'decide':'Fits?'}),json.dumps(inputs)]).fetchone()[0])
        value=document['native']
        assert value['error']['kind']=='usage', value
        assert len(document.get('completed', []))==int(incremental), document
        assert value.get('facts', {}).get('requests_sent', 0)==int(incremental), document
say(ok=True)
''', environment(backend))
    expect(got['ok'], True, 'incremental malformed descriptors retain their prefix')
    expect(backend.close(), 3, 'one send per incremental prefix and no eager sends')


def test_complete_annotation_validates_member_projection_before_schema():
    backend = Backend()
    got = child('''
db=connect()
db.execute('SELECT thinkthen_configure(?)', [json.dumps({'cache':False,'batch':1})])
question={'version':1,'questions':{'body':{'decide':'Fits?','on':['/body'],'item_schema':{'type':'string'}}}}
for invalid in (False, True):
    records=[{'json':{'body':'Alpha.'}}]
    if invalid: records.append({'json':{'body':42}})
    document=json.loads(db.execute('SELECT thinkthen_annotate_complete(?,?)',
        [json.dumps(question),json.dumps({'records':records})]).fetchone()[0])
    if invalid: assert document['native']['error']['kind']=='usage', document
    else: assert len(document['native']['value'])==1 and document['ordinals']==[0], document
say(ok=True)
''', environment(backend))
    expect(got['ok'], True, 'annotation admits each member selected evidence')
    expect(backend.close(), 1, 'valid annotation sends once and invalid eager rows send nothing')


def test_complete_filter_retains_true_false_and_incremental_terminal_prefix():
    backend = Backend()
    got = child('''
db=connect()
db.execute('SELECT thinkthen_configure(?)', [json.dumps({'cache':False,'batch':1})])
for threshold in (0.5, 0.95):
    for incremental in (False, True):
        records=[{'text':'Alpha.'}]
        if incremental: records.append({'text':'Beta.','images':None})
        document=json.loads(db.execute('SELECT thinkthen_filter_complete(?,?)',
            [json.dumps({'decide':'Fits?','threshold':threshold}),json.dumps({'records':records,'incremental':incremental})]).fetchone()[0])
        if incremental: assert document['native']['error']['kind']=='usage', document
        rows=document['completed'] if incremental else document['native']['value']
        assert len(rows)==1 and rows[0]['value']==(threshold==0.5) and document['ordinals']==[0], document
        assert document['native']['facts']['requests_sent']==1, document
say(ok=True)
''', environment(backend))
    expect(got['ok'], True, 'complete filter retains rejected observations and their ordinals')
    expect(backend.close(), 4, 'each true or false complete filter call sends once')


def test_saved_selectors_keep_projection_locations_and_refuse_conflicting_collections():
    backend = Backend()
    got = child('''
from pathlib import Path
folder=Path(os.environ['XDG_CONFIG_HOME'])/'thinkthen/questions'
folder.mkdir(parents=True)
question=folder/'saved.json'
question.write_text(json.dumps({'decide':'Fits?','on':['/body'],'item_schema':{'type':'string'}}))
db=connect()
db.execute('SELECT thinkthen_configure(?)', [json.dumps({'cache':False,'batch':1})])
for selector in ('@@saved', '@'+str(question)):
    inputs={'records':[{'json':{'body':'Alpha.','private':'excluded'},'source':{'file':'source.jsonl','first_line':4,'last_line':4}}]}
    document=json.loads(db.execute('SELECT thinkthen_decide_complete(?,?,?)',
        [selector,json.dumps(inputs),json.dumps({'threshold':0.95})]).fetchone()[0])
    assert document['native']['value'][0]['value'] is False, document
    assert document['ordinals']==[0] and document['native']['facts']['requests_sent']==1, document
    assert document['observations'][0]['inputs'][0]['source']=={'file':'source.jsonl','first_line':4,'last_line':4}, document
question.write_text(json.dumps({'decide':'Fits?','threshold':0.5}))
document=json.loads(db.execute('SELECT thinkthen_decide_complete(?,?,?)',
    ['@@saved',json.dumps({'records':[{'text':'Alpha.'}]}),json.dumps({'threshold':0.95})]).fetchone()[0])
assert document['native']['error']['kind']=='usage' and 'facts' not in document['native'], document
for inputs in ({'records':[], 'files':{'paths':['missing']}}, {}, {'records':[], 'incremental':None}):
    document=json.loads(db.execute('SELECT thinkthen_decide_complete(?,?)',
        ['@missing-saved-question.json',json.dumps(inputs)]).fetchone()[0])
    assert document['native']['error']['kind']=='usage' and 'facts' not in document['native'], document
say(ok=True)
''', environment(backend))
    expect(got['ok'], True, 'saved selectors preserve caller controls and located projection')
    expect(backend.close(), 2, 'invalid input collections refuse before saved resolution and sends')


if __name__ == '__main__':
    raise SystemExit(main(globals()))
