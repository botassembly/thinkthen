"""Project actual copied public views into the existing shared C assertions."""
import json


def string(v): return v['data']
def optional(v): return v['value'] if v['present'] else None
def content(v): return json.loads(string(v['data'])) if v['kind'] == 2 else string(v['data'])
def original(v): return content(v['value']) if v['present'] else None
def strings(v): return [string(s) for s in v['data']]
def decision(v):
    return None if v['kind'] == 0 else bool(v['data']['boolean']) if v['kind'] == 1 else content(v['data']['authored'])
def member(v):
    return {1: lambda: decision(v['data']['decide']), 2: lambda: None if not v['data']['choose']['present'] else string(v['data']['choose']['value']), 3: lambda: strings(v['data']['tag']), 4: lambda: v['data']['score']}[v['kind']]()
def probabilities(v): return {string(p['name']): p['probability'] for p in v['data']}
def endpoint(v): return {key: string(v[key]) for key in ('name','kind')}
def entity(v): return {key: string(value) if key in ('text','kind') else value for key,value in v.items()}
def edge(v,recognized=False):
    convert=entity if recognized else endpoint
    result={'relation':string(v['relation']),'source':convert(v['source']),'target':convert(v['target']),'probability':v['probability']}
    if v['either']:result['either']=True
    return result

def location(v):
    if not v['present']:return {}
    return {key:string(value['value']) if key=='file' else value['value'] for key,value in v['value'].items() if value['present']}
def author(v):
    return {key:string(v[key]['value']) if key=='name' else int(v[key]['value']) for key in ('name','wording_version') if v[key]['present']}

def row(result,verb,at):
    v=result['rows'][at]; common=v['common']; meta=common['meta']; value=v.get('value')
    if verb=='decide':value=decision(value)
    elif verb=='choose':value=string(value['value']) if value['present'] else None
    elif verb=='tag':value=strings(value)
    elif verb=='filter':value=bool(value)
    elif verb=='rank':value=optional(value)
    elif verb=='find':value=original(value)
    elif verb=='annotate':
        causes=['','missing_answer','wrong_kind','missing_probability','invalid_probability','invalid_distribution','unexpected_probability']
        value={string(m['name']):member(m['data']['success']['value']) if m['state']==1 else {'failed':{'kind':'backend','cause':causes[m['data']['failure']['cause']]}} for m in v['answers']['data']}
    elif verb=='recognize':
        value={'entities':[entity(e) for e in value['entities']['data']]}
        if v['value']['relations']['present']:value['relations']=[edge(e,True) for e in v['value']['relations']['value']['data']]
    elif verb=='relate':value=[edge(e) for e in value['data']]
    index=0
    if verb=='find':index=optional(v['index'])
    else:
        for observation in result['observations']:
            if observation['kind']==2:
                event=observation['data']['row']; data=event['data'].get(verb)
                if data and string(data['common']['answer_id'])==string(common['answer_id']):index=event['index'];break
    answer={'value':value,'index':index,'answer_id':string(common['answer_id']),
            'origin':optional(meta['origin']),'answered_by':string(meta['answered_by']['value']) if meta['answered_by']['present'] else None,
            'observations':meta['observations']['len'],'sources':meta['question_sources']['len'],
            'observation_ids':[string(o['data']['observation_id'] if o['kind']==1 else o['data']['failure_id']) for o in meta['observations']['data']],
            'input':original(common['input']),**location(common['position']),**author(result['authors'][at])}
    if common['images']['present']:
        images=common['images']['value']['data'];answer['images']=[i['bytes'] for i in images];answer['image_properties']=[[i[k] for k in ('media','width','height')] for i in images]
    if common['answer']['present']:
        a=common['answer']['value'];data=a['data']
        if a['kind']==1:answer['probability']=data['probability']
        else:answer['probabilities']=probabilities(data['tag'] if a['kind']==3 else data[{2:'choice',4:'score',5:'find',7:'find'}[a['kind']]]['probabilities'])
    answer['detail_inputs']=[{'input':original(i['original']),**location(i['position'])} for i in result['details'][at]['inputs']['data']]
    if verb=='annotate':answer['member_authors']=[author(a) for a in result['memberAuthors'][at]]
    if verb=='rank' and result['rankMembers'][at]:
        def rank_facts(target,common,details):
            meta=common['meta']; target['model']=string(meta['model'])
            target['context_digest']=string(meta['context_sha256']['value']) if meta['context_sha256']['present'] else None
            usage=details['usage'];target['usage']={key:int(usage[key]['value']) for key in ('input_tokens','output_tokens') if usage[key]['present']}
            target['source_batch_sizes']=[int(source['batch_size']['value']) if source['batch_size']['present'] else None for source in details['question_sources']['data']]
        rank_facts(answer,common,result['details'][at]);answer['question_name']=string(v['question_name']['value']);answer['members']=[]
        for j,member in enumerate(result['rankMembers'][at]):
            mc=member['common'];mm=mc['meta'];a=author(result['memberAuthors'][at][j])
            child={'name':string(member['question_name']['value']),'value':optional(member['value']),'probability':mc['answer']['value']['data']['probability'],
                   'answer_id':string(mc['answer_id']),'author':a.get('name'),'observations':mm['observations']['len'],'sources':mm['question_sources']['len']}
            if 'wording_version' in a:child['wording_version']=a['wording_version']
            rank_facts(child,mc,result['rankMemberDetails'][at][j]);answer['members'].append(child)
    return answer

def project(payload,verb):
    if 'failure' in payload:
        s=payload['failure'];e=s['error']['value'];out={'code':e['code'],'message':string(e['message'])}
        if s['facts']['present']:out.update(requests_sent=int(s['facts']['value']['requests_sent']),records=int(s['facts']['value']['records']))
        if e['stopped']['present'] and e['stopped']['value']['at']['present']:out['stopped_at']=e['stopped']['value']['at']['value']
    else:
        r=payload['result'];s=r['summary'];facts=s['facts']['value'];out={'code':0,'schema':string(s['schema']),'requests_sent':int(facts['requests_sent']) if facts else 0,'cache_answers':int(facts['cache_answers']) if facts else 0,'observations':s['observation_count'],'call_id':string(facts['call_id']) if facts else '', 'rows':[row(r,verb,i) for i in range(s['count'])]}
    if 'result' in payload and payload['result']['summary']['facts']['present']:
        facts=payload['result']['summary']['facts']['value'];out['records']=int(facts['records'])
        for key in ('input_tokens','output_tokens'):
            if facts[key]['present']:out[key]=int(facts[key]['value'])
    if 'completed' in payload:out['completed']=[answer for r in payload['completed'] for answer in project({'result':r},verb)['rows']]
    return out
