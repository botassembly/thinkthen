"""Use the installed named Python API and Rust-owned generated result fields."""
import json
import sys
import threading
from pathlib import Path
import thinkthen as tt

LIBRARY = __import__('os').environ.get('THINKTHEN_FRAME_LIBRARY')

def main():
    document = json.loads(sys.stdin.readline())
    question = tt.QuestionSource(**{key:value for key,value in document['question'].items() if key!='role'})
    framed = document['input']
    if framed['kind'] == 'files':
        reading = framed['options']['reading']
        source = tt.Files(paths=tuple(framed['paths']), unit=reading['unit'],
                          window=reading.get('window', tt.ABSENT), media=framed['options']['media'], jsonl=framed.get('jsonl', False))
    else:
        values = []
        for row in framed['records']:
            content = row['content']
            context = row.get('context', tt.ABSENT)
            context = context if context is tt.ABSENT else context['value']
            options = row.get('options', tt.ABSENT)
            options = options if options is tt.ABSENT else tuple((item['name'], item.get('description', tt.ABSENT)) for item in options)
            values.append(tt.Item(value=content.get('value'), text=content['kind']=='text', image_only=content['kind']=='images',
                images=tuple(tt.Image(data=bytes(image['bytes']), media=image['media']) for image in row.get('images', [])), context=context, options=options))
        source = tt.Records(tuple(values))
        if document.get('incremental'):
            source = iter(values)
        elif LIBRARY == 'pandas':
            import pandas as pd
            import thinkthen.pandas
            source = pd.Series(values, index=[7]*len(values), name='original', dtype=object)
        elif LIBRARY == 'polars':
            import polars as pl
            import thinkthen.polars
            source = pl.Series('original', values, dtype=pl.Object)
    token = tt.CancelToken()
    if document.get('cancel'): token.cancel()
    controls = {'token': token, 'attempts': True}
    if document.get('deadline_ms') is not None: controls['deadline_ms'] = document['deadline_ms']
    if document.get('shared_context') is not None: controls['context'] = document['shared_context']
    verb = document['verb']
    prefix = []
    try:
        with tt.Engine(**document['settings']) as engine:
            if document.get('incremental'):
                with engine.iterate(verb, question, source, **controls) as session:
                    if document.get('batch_probe'):
                        print('ready', flush=True); sys.stdin.readline()
                    if document.get('held_cancel'):
                        def stop():
                            __import__('time').sleep(.15); session.cancel()
                        threading.Thread(target=stop, daemon=True).start()
                    prefix.extend(session)
                    results, facts = prefix, session.facts
            else:
                if document.get('held_cancel'):
                    def stop():
                        __import__('time').sleep(.15); token.cancel()
                    threading.Thread(target=stop, daemon=True).start()
                if verb in ('recognize', 'relate'): done = getattr(engine, verb)(source, question, **controls)
                elif LIBRARY is not None and isinstance(source, (tt.Files, tt.Records)) is False:
                    done = getattr(source.tt, verb)(question, engine=engine, **controls)
                else: done = getattr(engine, verb)(question, source, **controls)
                results, facts = done.results, done.facts
            assert facts is not None and len(facts.call_id)==64
            assert isinstance(facts.requests_sent,int)
            for result in results:
                assert result.schema=='thinkthen.result/2'
                assert isinstance(result.answer_id,str) and len(result.answer_id)==64
                if result.meta.observations:
                    assert result.meta.origin in ('live','cache','replay','proxy','memory')
                else:
                    assert result.meta.origin is None and getattr(result.meta, 'answered_by', None) is None
                assert result.to_dict()['answer_id']==result.answer_id
                if verb=='recognize':
                    if 'entities' in result.value:
                        for entity in result.value.entities:
                            assert isinstance(entity.start,int) and isinstance(entity.text,str)
                    elif 'proposals' in result.value:
                        for entity in result.value.proposals:
                            assert isinstance(entity.start,int) and isinstance(entity.name,str)
            print(json.dumps({'native':True,'results':[result.to_dict() for result in results], 'facts':facts.to_dict()},ensure_ascii=False))
    except tt.ThinkThenError as error:
        failure=getattr(error,'complete',None)
        facts=getattr(error,'facts',None)
        results=getattr(error,'results',prefix)
        detail=failure.error.to_dict() if failure is not None else {'kind':error.kind,'message':str(error),'retryable':error.retryable}
        encoded=facts.to_dict() if hasattr(facts,'to_dict') else facts
        print(json.dumps({'native':True,'error':detail,'facts':encoded,'completed':{'native':True,'results':[result.to_dict() for result in results],'facts':encoded} if results and encoded else None},ensure_ascii=False))

if __name__ == '__main__': main()
