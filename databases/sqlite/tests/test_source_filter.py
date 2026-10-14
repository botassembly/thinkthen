"""Typed file filtering uses native Source selection and preserves ordinary Feed calls."""
from helper import Backend, child, environment, expect, main


def test_source_file_selection_keeps_order_locations_and_owning_facts():
    backend=Backend()
    got=child('''
from pathlib import Path
root=Path(os.environ['SCRATCH']);a=root/'a.txt';b=root/'b.txt'
a.write_text('Alpha.\\nBeta.\\nDelta.\\n');b.write_text('Gamma.\\n')
paths=json.dumps([str(b),str(a),str(b)])
db=connect();db.execute('SELECT thinkthen_configure(?)',[json.dumps({'cache':False})])
sql='SELECT file,first_line,last_line,ordinal,value,probability,requests_sent,records,call_id,answer_id FROM thinkthen_filter(?,?) WHERE files_only=1 AND batch=2'
source=run(db,sql,('Fits?',paths))
feed=complete(db, 'filter', *['Fits?', json.dumps({'files': {'paths': [str(b), str(a), str(b)]}}), json.dumps({'batch': 2})])
assert 'error' not in feed['native'],feed
say(source=source,files=[str(b),str(a)],feed_rows=len(feed['native']['value']),
    feed_ordinals=feed['ordinals'],feed_records=feed['native']['facts']['records'],
    feed_sends=feed['native']['facts']['requests_sent'])
''',environment(backend,'arm/full/capture'))
    rows=got['source']
    expect(isinstance(rows,list),True,'typed Source query returns rows')
    expect([row[:8] for row in rows],[[got['files'][0],1,1,0,1,0.9,2,5],
                                     [got['files'][1],1,1,1,1,0.9,2,5]],
           'native files_only deduplicates selected files in occurrence order')
    expect(len({row[8] for row in rows}),1,'each row carries the owning call identity')
    expect(all(len(row[8])==64 and len(row[9])==64 for row in rows),True,'native call and answer identities')
    expect((got['feed_rows'],got['feed_ordinals'],got['feed_records'],got['feed_sends']),
           (5,list(range(5)),5,2),'ordinary Feed retains every row and existing facts')
    expect(backend.close(),4,'Source and ordinary Feed each evaluate once')



def test_source_table_projection_keeps_physical_rows_and_separate_context():
    backend=Backend()
    got=child('''
from pathlib import Path
source=Path(os.environ['SCRATCH'])/'rows.csv'
source.write_text('body,policy,private\\n"Alpha.\\nBeta.",Refund policy,PRIVATE_INPUT\\n')
db=connect()
rows=run(db,'SELECT file,first_line,last_line,value FROM thinkthen_filter(?,?) WHERE format=? AND fields=? AND context_field=?',
         ('Fits?',json.dumps([str(source)]),'csv','["/body"]','/policy'))
say(rows=rows,file=str(source))
''',environment(backend,'arm/full/capture'))
    expect(got['rows'],[[got['file'],2,3,1]],'native CSV framing retains multiline physical coordinates')
    body=backend.capture()[0]
    expect('Refund policy' in body and 'Alpha.' in body and 'Beta.' in body,True,'projected evidence and separate context reach the wire')
    expect('PRIVATE_INPUT' in body or got['file'] in body,False,'excluded fields and physical paths stay out of model evidence')
    expect(backend.close(),1,'one composed native Source row sends once')


def test_source_invalid_controls_and_empty_inputs_send_nothing():
    backend=Backend()
    got=child('''
db=connect();errors=[]
for predicate,value in [('files_only',2),('unit','bad'),('window',0),('media','image'),('format','bad'),('batch','2')]:
    errors.append(run(db,'SELECT file FROM thinkthen_filter(?,?) WHERE '+predicate+'=?',
        ('Fits?','["missing-source"]',value)))
empty=run(db,'SELECT file FROM thinkthen_filter(?,?)',('Fits?','[]'))
null=run(db,'SELECT file FROM thinkthen_filter(NULL,?) WHERE unit=?',('["missing"]',7))
conflict=run(db,'SELECT file FROM thinkthen_filter(?,?) WHERE settings=? AND context=?',
    ('Fits?','["missing"]','{"context":"one"}','two'))
say(errors=errors,empty=empty,null=null,conflict=conflict)
''',environment(backend))
    expect(all(isinstance(error,str) and error.startswith('thinkthen usage:') for error in got['errors']),True,
           'invalid source controls keep native usage refusals')
    expect(got['empty'].startswith('thinkthen usage:'),True,'native Source requires at least one path')
    expect(got['null'],[],'required NULL skips other source controls')
    expect(got['conflict'].startswith('thinkthen usage:'),True,'named duplicate controls refuse before source access')
    expect(backend.close(),0,'invalid and empty Source calls send nothing')


if __name__=='__main__':
    raise SystemExit(main(globals()))
