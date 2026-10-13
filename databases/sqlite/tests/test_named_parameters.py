"""Named keyed parameters preserve request bytes, caching and eager refusal."""
import json
from helper import Backend, child, environment, expect, main


def test_named_keyed_parameters_share_effective_settings_and_cache_identity():
    backend = Backend()
    got = child('''
db=connect()
db.execute('SELECT thinkthen_configure(?)',[json.dumps({'cache':False})])
packed='{"a":"Alpha.","b":"Beta."}'
sql='SELECT key,value FROM thinkthen_decide_many(?,?) WHERE model=? AND context=? AND batch=? AND threshold=?'
args=('Fits?',packed,'named-model','Shared context',2,'0.3:0.7')
first=run(db,sql,args)
same=run(db,sql,args)
settings=json.dumps({'model':'named-model','context':'Shared context','batch':2,'threshold':'0.3:0.7'},separators=(',',':'))
legacy=run(db,'SELECT key,value FROM thinkthen_decide_many(?,?,?)',('Fits?',packed,settings))
changed=run(db,sql,args[:3]+('Changed context',)+args[4:])
rank=run(db,'SELECT key,rank FROM thinkthen_rank(?,?) WHERE model=? AND context=? AND batch=?',('Fits?',packed,'named-model','Shared context',2))
say(first=first,same=same,legacy=legacy,changed=changed,rank=rank)
''', environment(backend, 'arm/full/capture'))
    for name in ('first','same','legacy','changed'):
        expect(got[name], [['a',1],['b',1]], 'named controls preserve keyed rows')
    expect(got['rank'], [['a',1],['b',2]], 'named rank controls retain order')
    bodies=[json.loads(body) for body in backend.capture()]
    expect([body['model'] for body in bodies], ['named-model']*3, 'named model reaches the wire')
    expect(backend.close(),3,'equivalent settings reuse rows; changed context and rank send once')



def test_named_candidates_preserve_descriptions_and_authored_order():
    backend=Backend()
    got=child('''
db=connect()
options='{"shipping":"Parcel questions","billing":"Money questions"}'
value=run(db,'SELECT key,value,probability FROM thinkthen_choose_many(?,?) WHERE options=?',
          ('Which team?','{"7":"A short note."}',options))
invalid=run(db,'SELECT key FROM thinkthen_choose_many(?,?) WHERE options=?',
            ('Which team?','{"7":"A short note."}','{"billing":"Money","billing":"Parcel"}'))
say(value=value,invalid=invalid)
''',environment(backend,'arm/full/capture'))
    expect(got['value'],[['7','shipping',0.9]],'first authored candidate retains its label')
    expect(got['invalid'].startswith('thinkthen usage:'),True,'duplicate candidate fields refuse')
    body=backend.capture()[0]
    expect(body.index('Parcel questions') < body.index('Money questions'),True,'candidate descriptions retain order')
    expect(backend.close(),1,'only the valid named candidates send')


def test_named_keyed_parameters_refuse_types_and_conflicts_before_sends():
    backend=Backend()
    got=child('''
db=connect()
errors=[]
for predicate,args in [('batch=?',('2',)),('deadline_ms=?',(1.5,)),('model=?',(7,)),
                       ('none=?',(2,)),('options=?',('not-json',)),
                       ('context=?',('shared',))]:
    settings='{"context":"existing"}' if predicate=='context=?' else '{}'
    errors.append(run(db,'SELECT key FROM thinkthen_decide_many(?,?,?) WHERE '+predicate,
                      ('Fits?','{"a":"Alpha."}',settings)+args))
null=run(db,'SELECT key FROM thinkthen_decide_many(NULL,?) WHERE batch=?',('{"a":"Alpha."}','wrong'))
say(errors=errors,null=null)
''', environment(backend))
    for error in got['errors']:
        expect(isinstance(error,str) and error.startswith('thinkthen usage:'),True,'typed named refusal')
    expect(got['null'],[],'required NULL skips malformed controls')
    expect(backend.close(),0,'invalid named parameters send nothing')


if __name__=='__main__':
    raise SystemExit(main(globals()))
