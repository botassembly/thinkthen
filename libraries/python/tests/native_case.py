"""Actual installed Python consumer; known outputs are accessed as public types."""
import json, sys
from thinkthen import complete as c
from thinkthen import ThinkThenError, CancelToken

# Test framing is arbitrary caller data. The adapter and native engine own admission.
def main():
    document=json.loads(sys.stdin.readline())
    settings=document['settings']
    q=document['question']
    source=document['input']
    question=c.QuestionSource(**q)
    if source['kind']=='files':
        reading=source['options']['reading']
        source=c.Files(paths=tuple(source['paths']),unit=reading['unit'],window=reading.get('window',c.ABSENT),media=source['options']['media'],jsonl=source.get('jsonl',False))
    else:
        items=[]
        for row in source['records']:
            content=row['content']
            context=row.get('context',c.ABSENT)
            options=row.get('options',c.ABSENT)
            items.append(c.Item(value=content.get('value'),text=content['kind']=='text',image_only=content['kind']=='images',
                images=tuple(c.Image(data=bytes(i['bytes']),media=i['media']) for i in row.get('images',[])),
                context=context if context is c.ABSENT else context['value'],
                options=options if options is c.ABSENT else tuple((i['name'],i.get('description',c.ABSENT)) for i in options)))
        source=c.Records(tuple(items))
    token=CancelToken()
    if document.get('cancel'):token.cancel()
    prefix=[]
    try:
        engine=c.Engine(**settings)
        if document.get("held_cancel"):
            import threading,time
            def stop():
                time.sleep(.15);token.cancel()
            threading.Thread(target=stop,daemon=True).start()
        # All ten named functions have distinct public calls and typed return carriers.
        methods={'decide':engine.decide,'choose':engine.choose,'tag':engine.tag,'score':engine.score,
                 'filter':engine.filter,'rank':engine.rank,'find':engine.find,'annotate':engine.annotate,
                 'recognize':engine.recognize,'relate':engine.relate}

        if document.get('incremental'):
            batch=getattr(engine,document['verb']+'_batch')(question,source,token=token,deadline_ms=document.get('deadline_ms'),attempts=True,context=document.get('shared_context'))
            if document.get("batch_probe"):
                print("ready",flush=True);sys.stdin.readline()
            for row in batch: prefix.append(row)
            done=c.Completed(tuple(row.result for row in prefix),batch.facts,tuple(row.ordinal for row in prefix),tuple(row.input for row in prefix))
        else:
            done=methods[document['verb']](question,source,token=token,deadline_ms=document.get('deadline_ms'),attempts=True,context=document.get('shared_context'))
        encoded_facts=c.to_json(done.facts)
        assert isinstance(done.facts.largest_request_bytes,int)
        assert done.facts.largest_request_estimated_input_tokens is None or isinstance(done.facts.largest_request_estimated_input_tokens,int)
        assert isinstance(done.facts.token_estimate_method,str)
        for key in ('largest_request_bytes','largest_request_estimated_input_tokens','token_estimate_method'):
            assert encoded_facts[key] == getattr(done.facts,key)
        if done.facts.usage_persistence is not c.ABSENT:
            observation=done.facts.usage_persistence
            assert isinstance(observation,c.PersistenceObservation)
            assert observation.state in ('disabled','pending','written','failed')
            assert observation.observed_at == 'facts_snapshot'
            assert encoded_facts['usage_persistence'] == c.to_json(observation)
        assert isinstance(done.facts.call_id,c.CallId)
        assert all(isinstance(r.answer_id,c.AnswerId) for r in done.results)
        for result in done.results:
            if isinstance(result,c.RankResult) and result.members is not c.ABSENT:
                for member in result.members:
                    assert isinstance(member,c.RankMember) and isinstance(member.result,c.RankMemberResult)
                    assert isinstance(member.result.answer_id,c.AnswerId)
                    assert member.result.value>0 and isinstance(member.result.answer,c.YesNo)
                    assert isinstance(member.result.question,c.DecideQuestion) and isinstance(member.result.meta,c.Meta)
        packet={'results':[c.to_json(r) for r in done.results],'facts':c.to_json(done.facts),'ordinals':done.ordinals,'inputs':[c.to_json(i) for i in done.inputs]}
        print(json.dumps(packet,separators=(',',':'),ensure_ascii=False))
    except ThinkThenError as error:
        facts=getattr(error,'facts',None)
        complete=getattr(error,'complete',None)
        if complete is not None:
            print(json.dumps({'error':c.to_json(complete),'facts':None if complete.facts is c.ABSENT else c.to_json(complete.facts),'completed':{'results':[c.to_json(r.result) for r in prefix],'facts':c.to_json(complete.facts),'ordinals':[r.ordinal for r in prefix],'inputs':[c.to_json(r.input) for r in prefix]} if prefix else None}));return
        print(json.dumps({'error':{'kind':error.kind,'message':str(error),'retryable':error.retryable},'facts':dict(facts) if facts else None}))

if __name__=='__main__': main()
