"""The native Python door carries one explicit closed wrapper identity."""
import json
import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import pytest
from thinkthen import _thinkthen as native


def test_canonical_session_keeps_async_polling_and_cleanup_responsive(backend, tmp_path):
    from conftest import child_env, start
    child = start('''
        import asyncio, json, sys, threading
        from thinkthen import _thinkthen as native
        engine = native._Engine(cache=False, max_retries=0, batch=1, throttle=1)
        request = {'schema': 'thinkthen.request/1', 'call': {
            'function': 'decide', 'question': {'kind': 'text', 'text': 'Late?'},
            'input': {'kind': 'feed', 'name': 'python'},
        }}
        session = engine._request_session(json.dumps(request))
        item = json.dumps({'item': {'original': {'kind': 'text', 'text': 'note'}}})
        async def main():
            ticks = 0
            while session._push(item) == 'full':
                ticks += 1
                await asyncio.sleep(.001)
            stopped = threading.Event()
            def stop():
                sys.stdin.readline()
                stopped.set()
            threading.Thread(target=stop, daemon=True).start()
            async def heartbeat():
                nonlocal ticks
                while not stopped.is_set():
                    ticks += 1
                    await asyncio.sleep(.001)
            beat = asyncio.create_task(heartbeat())
            while not stopped.is_set():
                packet = session._poll()
                if packet is not None:
                    assert json.loads(packet)['kind'] == 'observation'
                await asyncio.sleep(.001)
            await beat
            assert ticks > 0
            assert session._push(item) == 'accepted'
            session.cancel()
            assert session._push(item) == 'closed'
            session.close()
            session.close()
            assert json.loads(session._poll()) == {'kind': 'end'}
            print('closed', flush=True)
            sys.stdin.readline()
        asyncio.run(main())
    ''', child_env(backend, tmp_path, 'arm/held'))
    try:
        assert backend.wait(1) == 1
        child.stdin.write('stop\n')
        child.stdin.flush()
        assert child.stdout.readline().strip() == 'closed'
        assert backend.count() == 1
    finally:
        backend.release()
        if child.poll() is None:
            child.stdin.write('released\n')
            child.stdin.flush()
        _, error = child.communicate(timeout=5)
    assert child.returncode == 0, error


def test_canonical_session_admits_and_finishes_through_native_rules(backend, tmp_path):
    from conftest import child_env, run
    output = run('''
        import json, time
        from thinkthen import _thinkthen as native
        engine = native._Engine(cache=False, max_retries=0)
        request = {'schema': 'thinkthen.request/1', 'call': {
            'function': 'decide', 'question': {'kind': 'text', 'text': 'Late?'},
            'input': {'kind': 'feed', 'name': 'python'},
        }}
        for value in ('{bad', json.dumps({**request, 'unknown': True})):
            try:
                engine._request_session(value)
            except native.UsageError:
                pass
            else:
                raise AssertionError('invalid request admitted')
        session = engine._request_session(json.dumps(request))
        session._finish(json.dumps({'kind': 'invalid_input'}))
        packets = []
        while True:
            packet = session._poll()
            if packet is None:
                time.sleep(.001)
                continue
            packet = json.loads(packet)
            if packet['kind'] == 'end': break
            packets.append(packet)
        terminal = packets[-1]
        assert terminal['kind'] == 'terminal'
        assert terminal['failure']['error']['kind'] == 'usage'
        session.close()
        print('refused')
    ''', child_env(backend, tmp_path))
    assert output.splitlines() == ['refused']
    assert backend.count() == 0


def test_complete_surface_is_closed_immediate_and_used_by_eager_and_lazy_calls(monkeypatch,tmp_path):
    for name in ('HOME','XDG_CONFIG_HOME','XDG_CACHE_HOME','XDG_STATE_HOME'):
        monkeypatch.setenv(name,str(tmp_path/name))
    monkeypatch.setenv('THINKTHEN_API_KEY','sk-surface-loopback')
    agents=[]
    class Handler(BaseHTTPRequestHandler):
        def do_POST(self):
            self.rfile.read(int(self.headers['Content-Length']))
            agents.append(self.headers['User-Agent'])
            body=b'{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}'
            self.send_response(200)
            self.send_header('Content-Length',str(len(body)))
            self.send_header("Connection", "close")
            self.end_headers()
            self.wfile.write(body)
        def log_message(self,*args): pass
    server=ThreadingHTTPServer(('127.0.0.1',0),Handler)
    worker=threading.Thread(target=server.serve_forever)
    worker.start()
    try:
        engine=native._Engine(base_url=f'http://127.0.0.1:{server.server_port}/v1',model='fixed',cache=False,max_retries=0)
        request=json.dumps({'verb':'decide','question':{'role':'atomic','body':{'decide':'Q'}},'input':{'kind':'records','records':[{'content':{'kind':'text','value':'a'},'images':[]}]}})
        for surface in ('typescript','Python',' python','unknown-secret',''):
            for method in (engine._complete,engine._complete_batch):
                with pytest.raises(native.ThinkThenError,match='surface must be python, pandas or python-polars') as error:
                    method('{bad',None,None,surface)
                assert surface not in str(error.value) or surface in ('',' python')
        assert agents==[]
        for surface in (None,'python','pandas','python-polars'):
            if surface is None:engine._complete(request,None,None)
            else:engine._complete(request,None,None,surface)
            batch=engine._complete_batch(request,None,None,surface)
            assert len(agents)%2==1
            event=json.loads(batch._pull())
            assert event['row']['value'] is True
            assert 'facts' in json.loads(batch._pull())
            batch.close()
        assert agents==[f'thinkthen/0.2.0 ({s})' for s in ('python','python','pandas','python-polars') for _ in range(2)]
    finally:
        server.shutdown();server.server_close();worker.join()


def test_complete_recognition_reads_proposals_and_preserves_old_documents():
    from thinkthen import complete as c
    answer = {'pieces': [], 'names': [], 'pairs': [], 'proposals': [
        {'start': 11, 'end': 14, 'span_probability': .4, 'selected': {'start': 11, 'end': 14},
         'kind': 'person', 'strength': .36, 'kept': False},
        {'start': 3, 'end': 6, 'span_probability': .9, 'kept': False}]}
    result = c.decode('RecognitionAnswer', answer)
    assert result.proposals[0].strength == .36
    assert result.proposals[0].kind == 'person'
    assert result.proposals[1].strength is c.ABSENT
    assert c.to_json(result) == answer
    old = {key: value for key, value in answer.items() if key != 'proposals'}
    assert c.decode('RecognitionAnswer', old).proposals is c.ABSENT
    with pytest.raises(ValueError):
        c.decode('RecognitionAnswer', {**answer, 'invented': True})


def test_tagged_json_string_context_refuses_before_sending(backend, tmp_path):
    from conftest import child_env, run
    output = run('''
        import json
        from thinkthen import _thinkthen as native
        engine = native._Engine(cache=False, max_retries=0)
        request = json.dumps({
            'verb': 'decide',
            'question': {'role': 'atomic', 'body': {'decide': 'Q'}},
            'input': {'kind': 'records', 'records': [{
                'content': {'kind': 'text', 'value': 'record'},
                'context': {'kind': 'json', 'value': 'private context'},
            }]},
        })
        try:
            engine._complete(request, None, None)
        except native.ThinkThenError as error:
            assert 'the per-item context does not match context_schema' in str(error)
            assert 'private context' not in str(error)
        else:
            raise AssertionError('tagged JSON context became text context')
        batch = engine._complete_batch(request, None, None)
        event = json.loads(batch._pull())
        batch.close()
        assert event['error']['kind'] == 'usage'
        assert 'the per-item context does not match context_schema' in event['error']['message']
        assert 'private context' not in event['error']['message']
        print('refused')
    ''', child_env(backend, tmp_path))
    assert output.splitlines() == ['refused']
    assert backend.count() == 0


@pytest.mark.parametrize('depth,kind', [(126, 'cancelled'), (127, 'cancelled'), (128, 'usage')])
def test_complete_json_nesting_keeps_original_parser_limit(backend, tmp_path, depth, kind):
    from conftest import child_env, run
    output = run(f'''
        import json
        from thinkthen import _thinkthen as native
        value = 'private original'
        for _ in range({depth}):
            value = {{'child': value}}
        request = json.dumps({{
            'verb': 'decide',
            'question': {{'role': 'atomic', 'body': {{'decide': 'Q'}}}},
            'input': {{'kind': 'records', 'records': [{{
                'content': {{'kind': 'json', 'value': value}},
            }}]}},
            'cancel': True,
        }})
        engine = native._Engine(cache=False, max_retries=0)
        try:
            engine._complete(request, None, None)
        except native.ThinkThenError as error:
            assert error.kind == {kind!r}, error.kind
            assert 'private original' not in str(error)
            print(error.kind)
        else:
            raise AssertionError('cancelled or excessive-depth original admitted')
    ''', child_env(backend, tmp_path))
    assert output.splitlines() == [kind]
    assert backend.count() == 0


@pytest.mark.parametrize('options', [[], [{'name': 'same'}, {'name': 'same'}],
                                      [{'name': 'first', 'description': 42}]])
def test_complete_shortlist_refusals_agree_between_eager_and_streamed_calls(backend, tmp_path, options):
    from conftest import child_env, run
    output = run(f'''
        import json
        from thinkthen import _thinkthen as native
        engine = native._Engine(cache=False, max_retries=0)
        request = json.dumps({{
            'verb': 'choose',
            'question': {{'role': 'atomic', 'body': {{'choose': 'Q', 'options': ['first', 'second']}}}},
            'input': {{'kind': 'records', 'records': [{{
                'content': {{'kind': 'text', 'value': 'private original'}},
                'options': {options!r},
            }}]}},
        }})
        try:
            engine._complete(request, None, None)
        except native.ThinkThenError as error:
            assert error.kind == 'usage'
            message = str(error)
            assert 'private original' not in message
        else:
            raise AssertionError('invalid shortlist admitted')
        batch = engine._complete_batch(request, None, None)
        event = json.loads(batch._pull())
        batch.close()
        assert event['error']['kind'] == 'usage'
        assert event['error']['message'] == message
        print('refused')
    ''', child_env(backend, tmp_path))
    assert output.splitlines() == ['refused']
    assert backend.count() == 0
