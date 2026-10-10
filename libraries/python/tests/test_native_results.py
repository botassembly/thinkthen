from conftest import child_env, run

def test_native_results_keep_owned_values_presence_and_pickle(backend, tmp_path):
    from conftest import child_env, run
    output = run('''
        import gc, json, pickle, time
        from thinkthen import _thinkthen as native
        from thinkthen._native_results import NativeSessionPacketDecideRow, NativeAtomicDecideValue
        engine = native._Engine('{"cache":false,"max_retries":0}')
        request = {'schema': 'thinkthen.request/1', 'call': {
            'function': 'decide', 'question': {'kind': 'text', 'text': 'Late?'},
            'input': {'kind': 'feed', 'name': 'python'},
        }}
        session = engine._request_session(json.dumps(request))
        assert session._push(json.dumps({'item': {'original': {'kind': 'text', 'text': 'note'}}})) == 'accepted'
        session._finish()
        rows = []
        while True:
            packet = session._poll_typed()
            if packet is None:
                time.sleep(.001)
                continue
            if packet['kind'] == 'end': break
            if packet.kind == 'row': rows.append(packet)
        assert len(rows) == 1
        row = rows[0]
        assert isinstance(row, NativeSessionPacketDecideRow)
        assert isinstance(row.value, NativeAtomicDecideValue)
        assert isinstance(row.value.answer.probability, float)
        assert row.value.meta.origin == 'live'
        session.close()
        del session, engine
        gc.collect()
        assert pickle.loads(pickle.dumps(row)) == row
        assert dict(row) == row.to_dict()
        assert repr(row).startswith('<NativeResult ')
        try: row.value = None
        except AttributeError: pass
        else: raise AssertionError('result is mutable')
        document = row.to_dict()
        for value in (None, False, 0, [], {}, {'nested': [False, None, 18446744073709551615]}):
            document['value']['value'] = value
            document['value']['error'] = None
            document['value']['failure'] = {'value': True}
            document['extension'] = {'secret': 'do-not-print', 'json': [False, None]}
            document['value'].pop('question_name', None)
            typed = native._restore_native_result('completesessionPacket', json.dumps(document))
            assert typed.value.value == value
            assert bool(typed.value) == bool(value)
            assert 'question_name' not in typed.value
            try: typed.value.question_name
            except AttributeError: pass
            else: raise AssertionError('absent field became null')
            try: typed.value['question_name']
            except KeyError: pass
            else: raise AssertionError('absent index became null')
            assert 'value' in typed.value
            assert typed.to_dict() == document
            assert pickle.loads(pickle.dumps(typed)) == typed
            assert 'do-not-print' not in str(typed)
            copied = typed.to_dict()
            copied['extension']['json'].append(True)
            assert typed.to_dict() == document
        failed = native._restore_native_result('completeAnnotationValue', json.dumps({
            'kind': 'failed', 'value': {'kind': 'backend', 'cause': 'missing_answer', 'value': False},
        }))
        assert failed.value.kind == 'backend'
        assert failed.value.cause == 'missing_answer'
        assert pickle.loads(pickle.dumps(failed)) == failed
        try: bool(failed)
        except TypeError: pass
        else: raise AssertionError('embedded failure became an answer')
        decision = native._restore_native_result('completeAnnotationValue', json.dumps({
            'kind': 'decision', 'value': False, 'error': None, 'failure': {'value': True},
        }))
        assert bool(decision) is False
        assert decision.to_dict()['error'] is None
        assert pickle.loads(pickle.dumps(decision)) == decision
        print('owned')
    ''', child_env(backend, tmp_path))
    assert output.splitlines() == ['owned']
    assert backend.count() == 1
