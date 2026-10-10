"""SQL file framing preserves table originals, context, physical rows and eager refusals."""
from helper import Backend, child, environment, expect, main


def test_complete_tables_keep_multiline_originals_context_and_false_results():
    backend = Backend()
    got = child('''
from pathlib import Path
root=Path(os.environ['SCRATCH'])
db=connect()
db.execute('SELECT thinkthen_configure(?)',[json.dumps({'cache':False,'batch':1})])
for format, text in [('csv','body,policy\\n"Alpha.\\nBeta.",Refund policy\\n'),
                     ('tsv','body\\tpolicy\\n"Alpha.\\nBeta."\\tRefund policy\\n')]:
    source=root/('input.'+format);source.write_text(text)
    inputs={'files':{'paths':[str(source)],'format':format},'reading':{'fields':['/body'],'context':'/policy'}}
    value=json.loads(db.execute('SELECT thinkthen_filter_complete(?,?)',
        [json.dumps({'decide':'Fits?','threshold':0.95}),json.dumps(inputs)]).fetchone()[0])
    assert 'error' not in value['native'], value
    assert value['native']['value'][0]['value'] is False, value
    assert value['ordinals']==[0] and value['native']['facts']['requests_sent']==1,value
    observed=value['observations'][0]['inputs'][0]
    assert observed['input']=={'body':'Alpha.\\nBeta.','policy':'Refund policy'},observed
    assert observed['source']=={'file':str(source),'first_line':2,'last_line':3},observed
say(ok=True)
''', environment(backend))
    expect(got['ok'], True, 'table originals and physical coordinates survive false SQL judgments')
    expect(backend.close(), 2, 'each valid table sends once')


def test_complete_tables_refuse_headers_controls_and_options_without_sends():
    backend = Backend()
    got = child('''
from pathlib import Path
root=Path(os.environ['SCRATCH']); db=connect()
for format, delimiter in [('csv',','),('tsv','\\t')]:
    for text, options, reading in [
        ('body'+delimiter+'body\\nAlpha'+delimiter+'Beta\\n',{},{}),
        ('body'+delimiter+'options\\nAlpha'+delimiter+'bad\\n',{}, {'fields':['/body'],'options':'/options'}),
        ('body\\nAlpha\\n',{'reading':{'unit':'file'}},{}),
        ('body\\nAlpha\\n',{'reading':{'unit':'window','window':2}},{}),
        ('body\\nAlpha\\n',{'reading':{'unit':'file'},'media':'image'},{}),
    ]:
        source=root/('bad.'+format);source.write_text(text)
        inputs={'files':{'paths':[str(source)],'format':format,'options':options},'reading':reading}
        value=json.loads(db.execute('SELECT thinkthen_decide_complete(?,?)',
            [json.dumps({'decide':'Fits?'}),json.dumps(inputs)]).fetchone()[0])
        assert value['native']['error']['kind']=='usage',value
        assert value['native'].get('facts',{}).get('requests_sent',0)==0,value
say(ok=True)
''', environment(backend))
    expect(got['ok'], True, 'table headers, projections and contradictory physical controls refuse')
    expect(backend.close(), 0, 'invalid tables send nothing')


if __name__ == '__main__':
    raise SystemExit(main(globals()))
