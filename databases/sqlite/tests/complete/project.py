"""Project actual formal native envelopes into the unchanged shared assertions."""
import base64
from pathlib import Path
import sys
sys.path.insert(0,str(Path(__file__).resolve().parents[4]/'conformance'))
from c_parity import ERRORS

def projected(row,verb,index,events):
    meta=row['meta']
    observations=meta.get('observations',[])
    inputs=[item for e in events if e['kind']=='question' and e['index']==index for item in e['inputs']]
    out={'value':row['value'],'index':index,'answer_id':row['answer_id'],
         'origin':{'live':1,'cache':2,'replay':3}.get(meta.get('origin')),
         'answered_by':meta.get('answered_by'),'observations':len(observations),
         'sources':len(meta.get('question_sources',[])),
         'observation_ids':[o.get('observation_id',o.get('failure_id')) for o in observations],
         'input':row.get('input'),'detail_inputs':[{**{'input':i['input']},**(i.get('source') or {})} for i in inputs]}
    out.update(row.get('source') or next((i['source'] for i in inputs if i.get('source')),{}))
    images=next((i['images'] for i in inputs if i.get('images')),[])
    if images:
        out['images']=[base64.b64decode(i['base64']).hex() for i in images]
        out['image_properties']=[[1 if i['media'] in ('jpeg','image/jpeg') else 2,i['width'],i['height']] for i in images]
        if isinstance(out['input'],dict) and 'images' in out['input']:out['input']=out['input'].get('text')
    if verb=='decide' and isinstance(out['value'],bool):out['value']=row['question'].get('true' if out['value'] else 'false',out['value'])
    for key in ('name','wording_version'):
        if key in row.get('question',{}):out[key]=row['question'][key]
    answer=row.get('answer',{})
    for key in ('probability','probabilities'):
        if key in answer:out[key]=answer[key]
    if verb=='annotate':
        out['member_authors']=[a.get('question',{}) for a in row.get('answers',{}).values()]
    if verb=='find':out['index']=row.get('answer',{}).get('selected')
    return out

def project(doc,verb):
    if 'host_error' in doc:
        message=doc['host_error']
        if doc.get('host_code')==9 or doc.get('host_cancelled'):return {'code':5,'message':message}
        kind=next((k for k in ERRORS if 'thinkthen '+k+':' in message),None)
        assert kind is not None,message
        return {'code':ERRORS[kind],'message':message}
    if 'error' in doc:
        error=doc['error'];out={'code':ERRORS[error['kind']],'message':error['message'],'completed':[projected(row,verb,(doc.get('ordinals') or list(range(len(doc.get('completed',[])))))[at],doc.get('observations',[])) for at,row in enumerate(doc.get('completed',[]))]}
        if 'facts' in doc:out.update(requests_sent=doc['facts']['requests_sent'],records=doc['facts']['records'])
        if 'at' in error.get('stopped',{}):out['stopped_at']=error['stopped']['at']
        return out
    values=doc['value'] if isinstance(doc['value'],list) else [doc['value']]
    facts=doc['facts'];events=doc.get('observations',[])
    out={'code':0,'schema':values[0]['schema'] if values else 'thinkthen.result/2','requests_sent':facts['requests_sent'],'cache_answers':facts['cache_answers'],'call_id':facts['call_id'],'observations':len(events)}
    out['rows']=[projected(row,verb,(doc.get('ordinals') or list(range(len(values))))[at],events) for at,row in enumerate(values)]
    if verb=='find':out['rows'][0]['index']=doc['selection']
    return out
