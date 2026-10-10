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
                done.value
                assert len(done.facts.call_id) == 64, verb
                assert done.facts.requests_sent >= 0, verb
                for row in done.results:
                    assert len(row.answer_id) == 64, verb
                    assert row.schema == 'thinkthen.result/2', verb
                assert pickle.loads(pickle.dumps(done)).to_dict() == done.to_dict(), verb
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
            detailed = engine.decide({'decide': 'Late?'}, 'note', details=True)
            assert detailed.value is detailed.results[0]
            assert detailed.value.input == 'note'
            assert detailed.value.index == 0
            context = engine.decide({'decide': 'Late?'}, c.Records((c.Item(value='note', text=True, context='policy'),)))
            assert len(context.results[0].meta.context_sha256) == 64
            item = engine.decide('Late?', c.Records((c.Item(value='note', text=True, context='policy'),)))
            assert item.results[0].meta.context_sha256 == context.results[0].meta.context_sha256
            assert bool(detailed.value) is True
            assert engine.decide('Late?', 'note').value is True
            assert engine.choose('Team?', 'note', options=['bill', 'ship']).probability == .9
            import asyncio
            assert asyncio.run(engine.asyncio.decide('Late?', 'note')).value is True
            assert engine.rank('Late?', ['first', 'second']).value[0]['index'] == 0
            source = c.Files(paths=(str(__import__('pathlib').Path(__import__('os').environ['XDG_CONFIG_HOME']) / 'source.txt'),), unit='line')
            __import__('pathlib').Path(source.paths[0]).parent.mkdir(parents=True, exist_ok=True)
            __import__('pathlib').Path(source.paths[0]).write_text('\\nfirst\\nsecond\\n')
            selection = tt.read_files(source.paths, unit='line')
            assert engine.decide('Late?', selection).value == [True, True]
            located = engine.decide('Late?', selection, details=True)
            assert located.value[0] is located.results[0]
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
        for controls in ({'unknown': True}, {'deadline_ms': None}, {'batch': 0}, {'threshold': 3}):
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
@pytest.mark.parametrize('failure', ['admission', 'reader', 'backend'])
def test_producer_cleanup_preserves_typed_failure_and_prefix(backend, tmp_path, asynchronous, failure):
    output = run('''
        import asyncio, os, thinkthen as tt
        mode = os.environ['FAILURE']
        closed = []
        class Inputs:
            def __init__(self): self.rows = iter(['first', 42, 'later'])
            def __iter__(self): return self
            def __next__(self): return next(self.rows)
            def close(self):
                closed.append(True)
                raise RuntimeError('producer cleanup failed')
        engine = tt.Engine(cache=False, max_retries=0, batch=1, throttle=1)
        question = {'decide': 'Late?', 'item_schema': {'type': 'string'}}
        if mode == 'admission': question = {'decide': ''}
        expected = tt.BackendError if mode == 'backend' else tt.UsageError
        try:
            if os.environ['ASYNCHRONOUS'] == 'True':
                asyncio.run(engine.asyncio.decide(question, Inputs()))
            else: engine.decide(question, Inputs())
        except expected as error:
            assert closed == [True]
            assert not engine._sessions
            if mode == 'admission': assert error.facts is None
            else:
                assert error.facts.requests_sent == 1
                assert error.kind == ('backend' if mode == 'backend' else 'usage')
                assert error.complete.error.kind == error.kind
                assert error.terminal.failure.to_dict() == error.complete.to_dict()
                assert [row.input for row in error.results] == ([] if mode == 'backend' else ['first'])
        else: raise AssertionError('call failure was lost')
        engine.close()
        print('preserved')
    ''', child_env(backend, tmp_path,
                   'arm/malformed/missing_answer' if failure == 'backend' else 'generic',
                   FAILURE=failure, ASYNCHRONOUS=str(asynchronous)))
    assert output.splitlines() == ['preserved']
    assert backend.count() == (0 if failure == 'admission' else 1)
