"""Installed named calls preserve native results and clean up held async work."""
from conftest import child_env, run, start


def test_named_calls_use_owned_typed_results_for_the_ten_functions(backend, tmp_path):
    output = run('''
        import pickle, thinkthen as tt
        from thinkthen import complete as c
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
            detailed = engine.decide(c.DecideSpec(decide='Late?'), c.TextInput(text='note'), details=True)
            assert detailed.value is detailed.results[0]
            assert detailed.value.input == 'note'
            assert detailed.value.index == 0
            context = engine.decide(c.DecideSpec(decide='Late?'), c.RecordInput(records=('note',), context='policy'))
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
            located = engine.decide('Late?', source, details=True)
            assert located.results[0].source.first_line == 2
            assert located.results[1].source.first_line == 3
            assert [row.index for row in located.results] == [0, 1]
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


def test_named_async_cancellation_closes_producer_before_provider_returns(backend, tmp_path):
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
                finally: closed.append(True)
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
