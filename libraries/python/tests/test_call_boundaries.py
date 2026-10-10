"""Named calls keep credentials private, native limits and actual settlement."""
import pytest
from conftest import child_env, run, FAKE, REPO

@pytest.mark.parametrize('arm', ['arm/401', 'arm/503', 'arm/refuse'])
def test_named_failure_messages_and_result_reprs_do_not_expose_credentials(backend,tmp_path,arm):
    output=run('''
        import os, thinkthen as tt
        key=os.environ['THINKTHEN_API_KEY']
        engine=tt.Engine(cache=False,max_retries=0)
        calls=[('decide','Late?',['note'],{}),('choose','Team?',['note'],{'options':['a','b']}),
               ('score','Urgent?',['note'],{'levels':['low','high']}),('tag','Kinds?',['note'],{'labels':['a','b']}),
               ('filter','Late?',['note'],{}),('rank','Late?',['a','b'],{}),('find','Best?',['a','b'],{}),
               ('annotate',{'version':1,'questions':{'late':{'decide':'Late?'}}},['note'],{}),
               ('recognize',{'version':1,'recognize':{'kinds':{'person':'name'}}},'Ada',{}),
               ('relate',{'version':1,'relate':{'relations':[{'name':'knows','source':'person','target':'person'}]}},[('Ada','person'),('Bo','person')],{})]
        for verb,question,source,controls in calls:
            try:
                value=getattr(engine,verb)(source,question,**controls) if verb in ('recognize','relate') else getattr(engine,verb)(question,source,**controls)
                shown=repr(value)+repr(value.results)+repr(value.terminal)
            except tt.ThinkThenError as error: shown=str(error)+repr(error)+repr(getattr(error,'results',()))
            assert key not in shown and key not in repr(engine)
        print('private')
    ''',child_env(backend,tmp_path,arm))
    assert output.splitlines()==['private']

def test_named_engine_refuses_address_credentials_without_printing_them(backend,tmp_path):
    output=run('''
        import thinkthen as tt, os
        for address in ['http://name:private-word@127.0.0.1:1/v1','http://127.0.0.1:1/v1?key=private-word']:
            try: tt.Engine(base_url=address,cache=False)
            except tt.UsageError as error: assert 'private-word' not in str(error)
            else: raise AssertionError('credential address accepted')
        print('refused')
    ''',child_env(backend,tmp_path))
    assert output.splitlines()==['refused']
    assert backend.count()==0

def test_native_preview_sends_nothing_and_module_calls_own_an_engine(backend,tmp_path):
    output=run('''
        import thinkthen as tt
        engine=tt.Engine(cache=False)
        plan=engine.plan('decide','Late?',['a','b'],batch=1)
        assert plan['records']==plan['requests']==2
        assert isinstance(plan['estimated_bytes'],int)
        assert tt.decide('Late?','a').value is True
        assert engine.decide('Late?','a').value is True
        print('planned')
    ''',child_env(backend,tmp_path))
    assert output.splitlines()==['planned']
    assert backend.count()==2

def test_installed_recognize_keeps_nonempty_aggregate_before_empty_final(backend,tmp_path):
    from pathlib import Path
    import json
    case=next(case for case in json.loads((REPO/'conformance/cases.json').read_text())['cases'] if case['id']=='44-recognize-C12-relations')
    output=run(f"""
        import thinkthen as tt
        with tt.Engine(cache=False,max_retries=0,model='jev-1.13.0') as engine:
            done=engine.recognize({case['exchanges'][0]['evidence']!r},{case['question']!r})
            assert [entity.text for row in done.results for entity in row.value.entities]==['Amara','Kestrel Labs']
            assert done.facts.requests_sent>0
            entities=done.results[0].value.entities
            plan=engine.plan('relate',{{'version':1,'relate':{{'relations':[{{'name':'works','source':'person','target':'organization'}}]}}}},entities)
            assert plan['records']==len(entities)
        print('kept')
    """,child_env(backend,tmp_path,'case/44-recognize-C12-relations'))
    assert output.splitlines()==['kept']
