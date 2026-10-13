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
            import os
            with tt.Engine(cache=False,base_url=os.environ['THINKTHEN_BASE_URL'].replace('/case/44-recognize-C12-relations/','/generic/')) as relations:
                linked=relations.relate(entities,relations={{'works':('person','organization')}})
                assert linked.facts.requests_sent>0
        print('kept')
    """,child_env(backend,tmp_path,'case/44-recognize-C12-relations'))
    assert output.splitlines()==['kept']


def test_native_admission_and_removed_keywords(backend, tmp_path):
    output = run('''
        import thinkthen as tt
        engine = tt.Engine(cache=False)
        try: engine.plan('decide', 'Late?', ['note'], batch=0)
        except tt.UsageError as error:
            assert str(error) == 'batch requires a positive whole number or max'
            assert error.kind == 'usage'
        else: raise AssertionError('zero batch admitted')
        try: engine.rank('Late?', ['note'], ask_model='removed')
        except TypeError: pass
        else: raise AssertionError('removed keyword accepted')
        try: engine.plan('decide', {'decide': 'Late?', 'item_schema': 'string'}, ['note'])
        except tt.UsageError as error:
            assert str(error) == 'the question declaration uses an unsupported feature'
        else: raise AssertionError('unsupported declaration admitted')
        for field in ['snippet_pieces', 'deadline_ms', 'max_requests_total', 'top']:
            try: engine.plan('recognize', {'version': 1, 'recognize': {'kinds': {}}}, ['note'], **{field: True})
            except tt.UsageError as error: assert str(error) == 'invalid canonical request'
            else: raise AssertionError('boolean integer admitted')
        for unit, window in [('bad', None), ('line', 2), ('window', 0), ('window', True), ('window', -1)]:
            try: tt.read_files('unread-private-path', unit=unit, window=window)
            except tt.UsageError as error:
                assert str(error) == 'reader window requires unit window and a positive whole number'
            else: raise AssertionError('invalid reading deferred until consumption')
        print('native admission')
    ''', child_env(backend, tmp_path))
    assert output.splitlines() == ['native admission']
    assert backend.count() == 0


def test_inline_label_conversion_preserves_order_and_refuses_invalid_labels_without_sending(backend, tmp_path):
    output = run('''
        from enum import Enum
        from typing import Literal
        import thinkthen as tt
        class Labels(Enum):
            zebra = "zebra"
            alpha = "alpha"
        class Aliases(Enum):
            one = "same"
            two = "same"
        engine = tt.Engine(cache=False)
        for labels in (Labels, Literal["zebra", "alpha"]):
            answer = engine.choose("Which?", "a", options=labels)
            assert answer.value == "zebra"
            assert list(answer.details[0].question.options) == ["zebra", "alpha"]
        for labels, message in ((Aliases, "Enum labels must not have aliases"),
                                (Literal[1, "alpha"], "Literal labels are strings")):
            try: engine.choose("Which?", "a", options=labels)
            except tt.UsageError as error: assert str(error) == message and error.kind == "usage"
            else: raise AssertionError("invalid labels were admitted")
        try: engine.recognize("a", kinds=Labels, descriptions={"unknown": "meaning"})
        except tt.UsageError as error: assert str(error) == "descriptions= names an unknown label"
        else: raise AssertionError("unknown description admitted")
        print("ordered labels")
    ''', child_env(backend, tmp_path))
    assert output.splitlines() == ["ordered labels"]
    assert backend.count() == 2


def test_native_typed_controls_preview_and_null_refusals_send_nothing(backend, tmp_path):
    output = run('''
        import thinkthen as tt
        engine = tt.Engine(cache=False, backend=None, model=None, max_requests_total=0)
        plan = engine.plan('decide', 'Late?', ['note'], attempts=True, details=True,
                           batch=1, deadline_ms=-1, context='', model='local', threshold=.5)
        assert plan['records'] == plan['requests'] == 1
        assert plan['estimated_bytes'] > 0
        assert plan['upper_bound'] is False
        for controls in ({'model': None}, {'deadline_ms': None}, {'context': None},
                         {'attempts': 'yes'}, {'seed_spans': [{'start': 0}]}, {'stage_context': {'boundary': 9}}):
            try: engine.plan('decide', 'Late?', ['note'], **controls)
            except tt.UsageError as error: assert error.kind == 'usage'
            else: raise AssertionError('malformed control admitted')
        print('typed controls')
    ''', child_env(backend, tmp_path))
    assert output.splitlines() == ['typed controls']
    assert backend.count() == 0
