"""Installed named calls preserve native results and clean up held async work."""
from conftest import child_env, run, start
import pytest


def test_named_calls_use_owned_typed_results_for_the_ten_functions(backend, tmp_path):
    output = run('''
        import pickle, thinkthen as tt
        import thinkthen as c
        with tt.Engine(cache=False, max_retries=0) as engine:
            calls = [
                ('decide', 'Late?', 'note', {}),
                ('choose', 'Team?', 'note', {'options': ['bill', 'ship']}),
                ('tag', 'Kinds?', 'note', {'labels': ['bill', 'ship']}),
                ('score', 'Urgent?', 'note', {'levels': ['low', 'high']}),
                ('filter', 'Late?', ['first', 'second'], {}),
                ('rank', 'Late?', ['first', 'second'], {}),
                ('find', 'Best?', ['first', 'second'], {}),
                ('annotate', {'version': 1, 'questions': {'late': {'decide': 'Late?'}}}, ['note'], {}),
                ('recognize', {'version': 1, 'recognize': {'kinds': {'person': 'A person name.'}}}, 'one name', {}),
                ('relate', {'version': 1, 'relate': {'relations': [{'name': 'knows', 'source': 'person', 'target': 'person'}]}}, [('A', 'person'), ('B', 'person')], {}),
            ]
            for verb, question, evidence, options in calls:
                if verb == 'recognize': done = engine.recognize(evidence, question, **options)
                elif verb == 'relate': done = engine.relate(evidence, question, **options)
                else: done = getattr(engine, verb)(question, evidence, **options)
                assert not hasattr(done, 'details') and not hasattr(type(done), '__bool__'), verb
                assert '__eq__' not in tt.Result.__dict__, verb
                assert done != done.value, verb
                if verb in ('tag', 'score', 'annotate'): assert done.probability is None, verb
                assert len(done.facts.call_id) == 64, verb
                assert done.facts.requests_sent >= 0, verb
                for row in done.results:
                    assert len(row.answer_id) == 64, verb
                    assert row.schema == 'thinkthen.result/2', verb
                assert not hasattr(done, "to_dict"), verb
                assert pickle.loads(pickle.dumps(done))._to_dict() == done._to_dict(), verb
                assert 'one name' not in repr(done)
                print(verb)
            pairs = [('A', 'person'), ('B', 'person')]
            relations = {'knows': ('person', 'person')}
            related = engine.relate(pairs, relations=relations)
            edges = related.value
            assert [(edge.relation, edge.source.name, edge.target.name) for edge in edges] == [('knows', 'A', 'B'), ('knows', 'B', 'A')]
            assert related.results[0].input == [{'name': 'A', 'kind': 'person'}, {'name': 'B', 'kind': 'person'}]
            assert engine.relate(iter(pairs), relations=relations).value == edges
            empty = engine.relate([], relations=relations)
            assert empty.value == []
            assert empty.facts.requests_sent == 0
            detailed = engine.decide({'decide': 'Late?'}, 'note')
            assert detailed.value is True
            assert detailed.results[0].input == 'note'
            assert detailed.results[0].index == 0
            context = engine.decide({'decide': 'Late?'}, c.Records((c.Item(value='note', text=True, context='policy'),)))
            assert len(context.results[0].meta.context_sha256) == 64
            item = engine.decide('Late?', c.Records((c.Item(value='note', text=True, context='policy'),)))
            assert item.results[0].meta.context_sha256 == context.results[0].meta.context_sha256
            assert bool(detailed.value) is True
            unresolved = engine.decide('Late?', ['first', 'second'], threshold='0.2:0.95')
            assert unresolved.value == [None, None] and unresolved.probability == [.9, .9]
            assert engine.decide('Late?', 'note').value is True
            try: engine.decide('Late?', 'note', details=True)
            except TypeError: pass
            else: raise AssertionError('removed details keyword admitted')
            assert engine.choose('Team?', 'note', options=['bill', 'ship']).probability == .9
            import asyncio
            assert asyncio.run(engine.asyncio.decide('Late?', 'note')).value is True
            assert engine.rank('Late?', ['first', 'second']).value[0] == 0
            source = c.Files(paths=(str(__import__('pathlib').Path(__import__('os').environ['XDG_CONFIG_HOME']) / 'source.txt'),), unit='line')
            __import__('pathlib').Path(source.paths[0]).parent.mkdir(parents=True, exist_ok=True)
            __import__('pathlib').Path(source.paths[0]).write_text('\\nfirst\\nsecond\\n')
            selection = tt.read_files(source.paths, unit='line')
            assert engine.decide('Late?', selection).value == [True, True]
            located = engine.decide('Late?', selection)
            assert located.value == [True, True]
            assert located.results[0].source.first_line == 2
            assert located.results[1].source.first_line == 3
            assert [row.index for row in located.results] == [0, 1]
        assert edges[0].relation == 'knows'
        assert pickle.loads(pickle.dumps(related)).value == edges
        failure_engine = tt.Engine(cache=False, max_retries=0, base_url=__import__('os').environ['THINKTHEN_BASE_URL'].replace('/generic/', '/arm/malformed/missing_answer/'))
        failed = failure_engine.annotate({'version': 1, 'questions': {'late': {'decide': 'Late?'}, 'bad': {'decide': 'Refund?'}}}, ['note'])
        embedded = failed.value[0]['bad']
        assert embedded.failed.kind == 'backend'
        assert embedded.failed.cause == 'missing_answer'
        assert failed.facts.requests_sent == 1
        try: bool(embedded)
        except TypeError: pass
        else: raise AssertionError('embedded failure became a truth value')
        assert pickle.loads(pickle.dumps(embedded)) == embedded
    ''', child_env(backend, tmp_path))
    assert output.splitlines() == ['decide', 'choose', 'tag', 'score', 'filter', 'rank', 'find', 'annotate', 'recognize', 'relate']


@pytest.mark.parametrize('cleanup_fails', [False, True])
def test_named_async_cancellation_closes_producer_before_provider_returns(backend, tmp_path, cleanup_fails):
    child = start('''
        import asyncio, sys, threading, thinkthen as tt
        async def main():
            reads = []
            closed = []
            def inputs():
                try:
                    while True:
                        reads.append(len(reads))
                        yield 'note'
                finally:
                    closed.append(True)
                    if __import__('os').environ['CLEANUP_FAILS'] == 'True':
                        raise RuntimeError('producer cleanup failed')
            async with tt.Engine(cache=False, max_retries=0, batch=1, throttle=1) as engine:
                task = asyncio.create_task(engine.asyncio.decide('Late?', inputs()))
                ready = threading.Event()
                def stop():
                    sys.stdin.readline()
                    ready.set()
                threading.Thread(target=stop, daemon=True).start()
                ticks = 0
                while not ready.is_set():
                    ticks += 1
                    await asyncio.sleep(.001)
                assert ticks > 0
                await asyncio.sleep(.02)
                assert len(reads) <= 3
                task.cancel()
                try: await task
                except asyncio.CancelledError: pass
                else: raise AssertionError('task ignored cancellation')
                assert closed == [True]
                before = len(reads)
                await asyncio.sleep(.01)
                assert len(reads) == before
                assert not engine._sessions
                print('closed', flush=True)
                sys.stdin.readline()
        asyncio.run(main())
    ''', child_env(backend, tmp_path, 'arm/held', CLEANUP_FAILS=str(cleanup_fails)))
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


def test_engine_close_cleans_all_sessions_when_producers_raise(backend, tmp_path):
    output = run('''
        import asyncio, thinkthen as tt
        async def main():
            closed = []
            started = set()
            def inputs(index):
                try:
                    started.add(index)
                    while True: yield 'note'
                finally:
                    closed.append(index)
                    raise RuntimeError('producer cleanup failed')
            engine = tt.Engine(cache=False, max_retries=0, batch=1, throttle=1)
            tasks = [asyncio.create_task(engine.asyncio.decide('Late?', inputs(index))) for index in range(2)]
            while len(started) < 2: await asyncio.sleep(.001)
            try: engine.close()
            except RuntimeError as error: assert str(error) == 'producer cleanup failed'
            else: raise AssertionError('cleanup error was discarded')
            assert sorted(closed) == [0, 1]
            assert not engine._sessions
            for task in tasks: task.cancel()
            for task in tasks:
                try: await task
                except asyncio.CancelledError: pass
            print('closed all')
        asyncio.run(main())
    ''', child_env(backend, tmp_path, 'arm/held'), timeout=5)
    assert output.splitlines() == ['closed all']


def test_named_calls_refuse_invalid_headers_without_sending(backend, tmp_path):
    output = run('''
        import thinkthen as tt
        engine = tt.Engine(cache=False, max_retries=0)
        for controls in ({'deadline_ms': None}, {'batch': 0}, {'threshold': 3}):
            try: engine.decide('Late?', 'private note', **controls)
            except tt.UsageError: pass
            else: raise AssertionError('malformed header admitted')
        token = tt.CancelToken()
        token.cancel()
        try: engine.decide('Late?', 'note', token=token)
        except tt.Cancelled as error:
            assert error.kind == 'cancelled'
            assert error.facts is None
        else: raise AssertionError('cancelled token admitted')
        assert not engine._sessions
        print('refused')
    ''', child_env(backend, tmp_path))
    assert output.splitlines() == ['refused']
    assert backend.count() == 0


@pytest.mark.parametrize('asynchronous', [False, True])
@pytest.mark.parametrize('failure', ['admission', 'reader', 'backend', 'on', 'token'])
@pytest.mark.parametrize('cleanup', ['call', 'descriptor'])
def test_producer_cleanup_preserves_typed_failure_and_prefix(backend, tmp_path, asynchronous, failure, cleanup):
    output = run('''
        import asyncio, os, thinkthen as tt
        mode = os.environ['FAILURE']
        closed = []
        class Inputs:
            def __init__(self): self.rows = iter(['first', 42, 'later'])
            def __iter__(self): return self
            def __next__(self): return next(self.rows)
            @property
            def close(self):
                if os.environ['CLEANUP'] == 'descriptor':
                    closed.append(True)
                    raise RuntimeError('producer close lookup failed')
                return self.cleanup
            def cleanup(self):
                closed.append(True)
                raise RuntimeError('producer cleanup failed')
        engine = tt.Engine(cache=False, max_retries=0, batch=1, throttle=1)
        question = {'decide': 'Late?', 'item_schema': {'type': 'string'}}
        if mode == 'admission': question = {'decide': ''}
        marker = RuntimeError('token inspection failed')
        class Token:
            @property
            def cancelled(self): raise marker
        controls = {'on': 42} if mode == 'on' else {'token': Token()} if mode == 'token' else {}
        expected = {'backend': tt.BackendError, 'on': TypeError, 'token': RuntimeError}.get(mode, tt.UsageError)
        try:
            if os.environ['ASYNCHRONOUS'] == 'True':
                asyncio.run(engine.asyncio.decide(question, Inputs(), **controls))
            else: engine.decide(question, Inputs(), **controls)
        except expected as error:
            assert closed == [True]
            assert not engine._sessions
            if mode in ('admission', 'on', 'token'): assert getattr(error, 'facts', None) is None
            else:
                assert error.facts.requests_sent == 1
                assert error.kind == ('backend' if mode == 'backend' else 'usage')
                assert error.complete.error.kind == error.kind
                assert error.terminal.failure._to_dict() == error.complete._to_dict()
                assert [row.input for row in error.results] == ([] if mode == 'backend' else ['first'])
            if mode == 'token': assert error is marker
            if mode == 'on': assert str(error) == "'int' object is not iterable"
        else: raise AssertionError('call failure was lost')
        engine.close()
        print('preserved')
    ''', child_env(backend, tmp_path,
                   'arm/malformed/missing_answer' if failure == 'backend' else 'generic',
                   FAILURE=failure, ASYNCHRONOUS=str(asynchronous), CLEANUP=cleanup))
    assert output.splitlines() == ['preserved']
    assert backend.count() == (0 if failure in ('admission', 'on', 'token') else 1)


def test_producer_exception_and_admission_message_survive_native_boundary(backend, tmp_path):
    output = run('''
        import thinkthen as tt
        marker = RuntimeError('private producer marker')
        def inputs():
            raise marker
            yield 'unreachable'
        engine = tt.Engine(cache=False)
        try: engine.decide('Is it?', inputs())
        except tt.LocalError as error:
            assert error.__cause__ is marker
            assert str(error) == 'session producer raised an exception'
            assert error.facts.requests_sent == 0
        else: raise AssertionError('producer failure answered')
        try: engine.decide({'decide':'Is it?', 'item_schema':{'type':'array'}}, 'unreachable')
        except tt.UsageError as error:
            assert str(error) == 'the question declaration uses an unsupported feature'
        else: raise AssertionError('unsupported declaration admitted')
        engine.close()
        print('retained')
    ''', child_env(backend, tmp_path))
    assert output.splitlines() == ['retained']
    assert backend.count() == 0


def test_shipped_function_examples_replay_installed_answers_without_sending(backend, tmp_path):
    import json
    from conftest import REPO
    examples = json.loads((REPO / 'libraries/python/examples.json').read_text())['examples']
    functions = ('decide', 'choose', 'tag', 'score', 'filter', 'rank', 'find', 'annotate', 'recognize', 'relate')
    for function in functions:
        sample = examples[function]
        folder = tmp_path / function
        folder.mkdir()
        for name, content in sample.get('files', {}).items():
            (folder / name).write_text(content)
        code = f'''
            import ast, os, thinkthen
            os.chdir({str(folder)!r})
            with thinkthen.Engine(cache=False, **SETTINGS) as tt:
                answer = {sample['python']}
                assert answer == ast.literal_eval({sample['expected']!r}), repr(answer)
                print(repr(answer))
        '''
        env = child_env(backend, folder)
        recorded = run(code.replace('SETTINGS', repr({'record': str(folder / 'recording')})), env)
        count = backend.count()
        replayed = run(code.replace('SETTINGS', repr({'replay': str(folder / 'recording')})), env)
        assert replayed == recorded
        assert backend.count() == count, function


def test_recognition_probabilities_follow_kept_spans_with_duplicate_names(backend, tmp_path):
    output = run('''
        import json, threading, thinkthen as tt
        from http.server import BaseHTTPRequestHandler, HTTPServer
        class Reply(BaseHTTPRequestHandler):
            def log_message(self, *args): pass
            def do_POST(self):
                body = json.loads(self.rfile.read(int(self.headers['Content-Length'])))
                answers = {}
                for at, (name, question) in enumerate(sorted(body['questions'].items())):
                    criteria = question['criteria']
                    picked = 'SINGLE' if 'SINGLE' in criteria else 'person'
                    probability = [.9, .7][at] if picked == 'SINGLE' else .6
                    odds = {key: probability if key == picked else (1-probability)/(len(criteria)-1)
                            for key in criteria}
                    answers[name] = {'type': 'choice', 'probabilities': odds}
                response = json.dumps({'model': 'jev-latest', 'answers': answers}).encode()
                self.send_response(200); self.send_header('Content-Length', str(len(response)))
                self.send_header("Connection", "close")
                self.end_headers(); self.wfile.write(response)
        server = HTTPServer(('127.0.0.1', 0), Reply)
        threading.Thread(target=server.serve_forever, daemon=True).start()
        with tt.Engine(cache=False, base_url=f'http://127.0.0.1:{server.server_port}/v1') as engine:
            call = engine.recognize('Ada Ada', kinds=['person'], threshold=.1)
            entities = call.value.entities
            assert [(entity.text, entity.start, entity.end) for entity in entities] == [('Ada', 0, 3), ('Ada', 4, 7)]
            proposals = [proposal for proposal in call.results[0].answer.proposals if proposal.kept]
            assert call.probability == [proposal.span_probability for proposal in proposals]
            assert call.probability[0] != call.probability[1]
            assert all(chance != entity.strength for chance, entity in zip(call.probability, entities))
            boundary = engine.recognize('Ada Ada', mode='boundary_only', threshold=.1)
            assert boundary.probability == [proposal.probability for proposal in boundary.value.proposals]
            assert [(proposal.text, proposal.start, proposal.end) for proposal in boundary.value.proposals] == [('Ada', 0, 3), ('Ada', 4, 7)]
        server.shutdown(); server.server_close()
        print('span probabilities')
    ''', child_env(backend, tmp_path))
    assert output.splitlines() == ['span probabilities']
