"""Public optional Series dispatch, using independently expected saved replies."""
import json
import pathlib
import pytest
from conftest import child_env, run, REPO
from backend_settings import configuration, isolated

CORPUS = REPO / "conformance/cases.json"


@pytest.mark.parametrize("readings,expected,dtype", [
    ({'true': 'refund', 'false': 'retain'}, ['refund', 'retain'], 'String'),
    ({'true': {'action': 'refund'}, 'false': {'action': 'retain'}},
     [{'action': 'refund'}, {'action': 'retain'}], 'Struct'),
    ({'true': ['refund'], 'false': ['retain', 'review']}, [['refund'], ['retain', 'review']], 'List'),
    ({'true': {'a': 1}, 'false': {'b': 2}}, [{'a': 1}, {'b': 2}], 'Object'),
    ({'true': [1], 'false': [True]}, [[1], [True]], 'Object'),
    ({'true': {'nested': {'a': 1}}, 'false': {'nested': {'b': 2}}}, [{'nested': {'a': 1}}, {'nested': {'b': 2}}], 'Object'),
    ({'true': [-1], 'false': [18446744073709551615]}, [[-1], [18446744073709551615]], 'List'),
    ({'true': [1.0], 'false': [9007199254740993]}, [[1.0], [9007199254740993]], 'Object'),
    ({'true': 'refund'}, ['refund', False], 'Object'),
    ({'false': 'retain'}, [True, 'retain'], 'Object'),
    ({'true': {'action': 'refund'}, 'false': 'retain'}, [{'action': 'refund'}, 'retain'], 'Object'),
])
def test_polars_question_files_preserve_authored_readings(backend, tmp_path, readings, expected, dtype):
    question = tmp_path / 'refund.json'
    question.write_text(json.dumps({'decide': 'Does the writer ask for a refund?', **readings}))
    recording = tmp_path / 'recording'
    printed = run(f'''
        import json, threading, polars as pl, thinkthen as tt
        from http.server import BaseHTTPRequestHandler, HTTPServer
        class Reply(BaseHTTPRequestHandler):
            def log_message(self, *args): pass
            def do_POST(self):
                body = self.rfile.read(int(self.headers['Content-Length']))
                self.server.calls += 1
                probability = .9 if b'refund now' in body else .1
                response = json.dumps({{'model': 'jev-latest', 'answers': {{'q1': {{'type': 'noul', 'noul': probability}}}},
                                       'usage': {{'input_tokens': 10, 'output_tokens': 2}}}}).encode()
                self.send_response(200)
                self.send_header("Connection", "close")
                self.send_header('Content-Length', str(len(response)))
                self.end_headers()
                self.wfile.write(response)
        server = HTTPServer(('127.0.0.1', 0), Reply); server.calls = 0
        threading.Thread(target=server.serve_forever, daemon=True).start()
        address = f'http://127.0.0.1:{{server.server_port}}/v1'
        source = pl.Series('body', ['refund now', None, 'good morning'])
        question = tt.QuestionSource(path={str(question)!r})
        with tt.Engine(cache=False, batch=1, base_url=address, record={str(recording)!r}) as engine:
            result = engine.decide(question, source)
            assert json.dumps([row.value for row in result.results], sort_keys=True) == json.dumps({expected!r}, sort_keys=True)
            values = result.value
            assert json.dumps(values.to_list(), sort_keys=True) == json.dumps([{expected[0]!r}, None, {expected[1]!r}], sort_keys=True)
            assert values.dtype.base_type() == getattr(pl, {dtype!r})
            if {expected!r} == [[-1], [18446744073709551615]]: assert values.dtype.inner == pl.Int128
            assert result.positions == (0, 2)
            assert result.facts.records == 2 and result.facts.requests_sent == 2
            assert result.probability.to_list() == [.9, None, .1]
            assert source.to_list() == ['refund now', None, 'good morning']
        with tt.Engine(cache=False, batch=1, base_url=address, replay={str(recording)!r}) as engine:
            replayed = engine.decide(question, source)
            assert json.dumps(replayed.value.to_list(), sort_keys=True) == json.dumps(values.to_list(), sort_keys=True)
            assert replayed.facts.requests_sent == 0 and replayed.facts.records == 2
        assert server.calls == 2
        server.shutdown(); server.server_close()
        print('authored readings retained')
    ''', child_env(backend, tmp_path))
    assert printed.strip() == 'authored readings retained'
    assert backend.count() == 0


def test_polars_frame_answers_keep_native_types_false_null_and_field_projection(backend, tmp_path):
    printed = run('''
        import polars as pl, thinkthen as tt
        source = pl.DataFrame({'body': ['first', None, 'second'], 'context': ['a', 'b', 'c']})
        with tt.Engine(cache=False, batch=1) as engine:
            for verb, controls, dtype, expected in [
                    ('decide', {'threshold': .95}, pl.Boolean, [False, None, False]),
                    ('choose', {'options': ['billing', 'shipping']}, pl.String, ['billing', None, 'billing']),
                    ('score', {'levels': ['Routine.', 'Soon.', 'Now.']}, pl.Float64, [.15, None, .15]),
                    ('tag', {'labels': ['bill', 'ship']}, pl.List(pl.String), [['bill', 'ship'], None, ['bill', 'ship']])]:
                call = getattr(engine, verb)('Late?', source, on='body', context_field='/context', **controls)
                assert call.value.dtype == dtype
                assert call.value.to_list() == expected
                assert call.value.name == 'body' and call.positions == (0, 2)
                assert call.probability.dtype == pl.Float64
                assert call.facts.records == 2 and call.facts.requests_sent == 2
                assert call.results[0].input == {'body': 'first', 'context': 'a'}
            assert source.to_dicts()[1] == {'body': None, 'context': 'b'}
        print('typed frame answers')
    ''', child_env(backend, tmp_path))
    assert printed.strip() == 'typed frame answers'
    assert backend.count() == 8


def test_polars_annotation_frames_keep_complete_failure_schema(backend, tmp_path):
    printed = run('''
        import json, threading, polars as pl, thinkthen as tt
        from http.server import BaseHTTPRequestHandler, HTTPServer
        class Reply(BaseHTTPRequestHandler):
            def log_message(self, *args): pass
            def do_POST(self):
                request = json.loads(self.rfile.read(int(self.headers['Content-Length'])))
                self.server.calls += 1
                instructions = request['questions']['q1']['instructions']
                omitted = 'q1' if 'The text is "first"' in instructions else 'q2' if 'The text is "second"' in instructions else None
                answers = {name: {'type': 'noul', 'noul': .9} for name in request['questions'] if name != omitted}
                response = json.dumps({'model': 'jev-latest', 'answers': answers,
                                       'usage': {'input_tokens': 10, 'output_tokens': 2}}).encode()
                self.send_response(200)
                self.send_header('Connection', 'close')
                self.send_header('Content-Length', str(len(response)))
                self.end_headers(); self.wfile.write(response)
        server = HTTPServer(('127.0.0.1', 0), Reply); server.calls = 0
        threading.Thread(target=server.serve_forever, daemon=True).start()
        form = {'version': 1, 'questions': {'first': {'decide': 'First?'}, 'second': {'decide': 'Second?'}}}
        marker = pl.Struct({'failed': pl.Struct({'kind': pl.String, 'cause': pl.String})})
        expected = {'body': pl.String, 'first': pl.Boolean, 'second': pl.Boolean,
                    'failed': pl.Struct({'first': marker, 'second': marker})}
        with tt.Engine(cache=False, batch=1, max_retries=0,
                       base_url=f'http://127.0.0.1:{server.server_port}/v1') as engine:
            for values in ([], [None, None], ['ok', None]):
                source = pl.DataFrame({'body': pl.Series(values, dtype=pl.String)})
                result = engine.annotate(form, source, on='body')
                assert result.value.schema == expected
                assert result.value['failed'].to_list() == [None] * len(values)
                assert result.value['first'].to_list() == [True if value is not None else None for value in values]
            source = pl.DataFrame({'body': ['first', None, 'second', 'ok']})
            result = engine.annotate(form, source, on='body')
            assert result.value.schema == expected
            failure = {'failed': {'kind': 'backend', 'cause': 'missing_answer'}}
            assert result.value['failed'].to_list() == [
                {'first': failure, 'second': None}, None, {'first': None, 'second': failure}, None], result.value['failed'].to_list()
            assert result.value['first'].to_list() == [None, None, True, True]
            assert result.value['second'].to_list() == [True, None, None, True]
        assert server.calls == 4
        server.shutdown(); server.server_close()
        print('stable annotation schema')
    ''', child_env(backend, tmp_path))
    assert printed.strip() == 'stable annotation schema'
    assert backend.count() == 0


def test_polars_recognition_frames_keep_empty_entity_schema(backend, tmp_path):
    printed = run('''
        import polars as pl, thinkthen as tt
        with tt.Engine(cache=False) as engine:
            for values in ([], [None, None]):
                source = pl.DataFrame({'body': pl.Series(values, dtype=pl.String)})
                result = engine.recognize(source, kinds=['person'], on='body')
                assert result.value.schema == {'row': pl.Int64, 'text': pl.String, 'start': pl.Int64,
                                              'end': pl.Int64, 'length': pl.Int64, 'kind': pl.String,
                                              'strength': pl.Float64}
                assert result.value.height == 0
                assert result.facts.requests_sent == 0
        print('stable empty entity schema')
    ''', child_env(backend, tmp_path))
    assert printed.strip() == 'stable empty entity schema'
    assert backend.count() == 0


@pytest.mark.parametrize("shape", ["pandas", "polars"])
def test_named_and_async_series_calls_keep_owned_rows_and_original_positions(backend, tmp_path, shape):
    printed = run('''
        import asyncio, pickle, pandas as pd, polars as pl, thinkthen as tt, thinkthen.pandas, thinkthen.polars
        shape = SHAPE
        def series(values, name="body"):
            return pd.Series(values, name=name, index=[9] * len(values), dtype=object) if shape == "pandas" else pl.Series(name, values, dtype=pl.Object)
        def cell(value, at): return value.iloc[at] if shape == "pandas" else value[at]
        import thinkthen as c
        source = series(['first', None, 'second'])
        with tt.Engine(cache=False, batch=1) as engine:
            done = source.tt.decide('Late?', engine=engine)
            assert isinstance(done, tt.Result)
            assert done.positions == (0, 2)
            assert [row.index for row in done.results] == [0, 1]
            if shape == "pandas": assert done.value.index.tolist() == [9, 9, 9]
            assert done.value.to_list() == [True, None, True]
            assert done.probability.to_list() == [.9, None, .9]
            if shape == "polars":
                assert done.value.dtype == pl.Boolean
                assert done.probability.dtype == pl.Float64
            assert done.facts.requests_sent == 2
            assert pickle.loads(pickle.dumps(done)).value.to_list() == done.value.to_list()
            detailed = engine.decide('Late?', source, details=True)
            assert cell(detailed.value, 0) is detailed.results[0]
            assert cell(detailed.value, 1) is None
            async_done = asyncio.run(engine.asyncio.decide('Late?', source))
            assert async_done.value.to_list() == done.value.to_list()
            ranked = engine.rank('Late?', source)
            assert [row['index'] for row in ranked.value] == [0, 2]
            assert ranked.facts.requests_sent == 2
            kept = source.tt.filter('Late?', engine=engine)
            assert kept.value.to_list() == ["first", "second"]
            assert kept.positions == (0, 2)
            explicit = series([c.Item(value=None), None])
            null = engine.decide('Late?', explicit)
            assert null.positions == (0,) and null.results[0].input is None
            entities = series([('Ada', 'person'), None, ('Bo', 'person')])
            related = entities.tt.relate(engine=engine, relations={'knows': ('person', 'person')})
            assert related.facts.requests_sent == 1
            assert [(edge.source.name, edge.target.name) for edge in related.results[0].value] == [('Ada', 'Bo'), ('Bo', 'Ada')]
            failed_engine = tt.Engine(cache=False, max_retries=0, base_url=__import__('os').environ['THINKTHEN_BASE_URL'].replace('/generic/', '/arm/malformed/missing_answer/'))
            failed = failed_engine.annotate({'version': 1, 'questions': {'late': {'decide': 'Late?'}, 'bad': {'decide': 'Refund?'}}}, series(['note', None]))
            embedded = cell(failed.value, 0)['bad']
            assert cell(failed.value, 1) is None and embedded.failed.kind == 'backend'
            assert embedded.failed.cause == 'missing_answer' and failed.facts.requests_sent == 1
            try: bool(embedded)
            except TypeError: pass
            else: raise AssertionError('pandas coerced an embedded failure')
            assert pickle.loads(pickle.dumps(embedded)) == embedded
        assert done.results[0].schema == 'thinkthen.result/2'
        print('owned series rows')
    '''.replace('SHAPE', repr(shape)), child_env(backend, tmp_path))
    assert printed.strip() == 'owned series rows'
    assert backend.count() == 13


def test_priced_recognition_collection_uses_native_checked_cost(backend, tmp_path):
    configuration(tmp_path, {"usd_per_million_input": "0", "usd_per_million_output": "0"})
    env = child_env(backend, tmp_path, "case/41-offsets-past-an-accent-and-an-emoji")
    env.update(isolated(tmp_path))
    case = next(row for row in json.loads(CORPUS.read_text())["cases"] if row["id"] == "41-offsets-past-an-accent-and-an-emoji")
    printed = run(f"""
        import pandas as pd, thinkthen as tt, thinkthen.pandas
        call = pd.Series([{case['text']!r}, None, {case['text']!r}]).tt.recognize({case['question']!r}, engine=tt.Engine(cache=False))
        assert call.value.iloc[1] is None
        print(call.facts['estimated_cost_usd'], call.facts['requests_sent'])
    """, env)
    assert printed.strip() == '0.000000 4'
    assert backend.count() == 4


@pytest.mark.parametrize("shape", ["pandas", "polars"])
def test_ten_series_functions_preserve_null_duplicate_and_whole_set_identity(backend, tmp_path, shape):
    printed = run(f"""
        import json, pandas as pd, polars as pl, thinkthen as tt
        import thinkthen.pandas, thinkthen.polars
        shape = {shape!r}
        engine = tt.Engine(batch=1, cache=False, record=RECORDING)
        def series(values):
            return (pd.Series(values, name="body", index=[9, 9, 2], dtype=object)
                    if shape == "pandas" else pl.Series("body", values, strict=False))
        texts = series(["one note", None, "one note"])
        def call(verb, *args, **kw):
            return getattr(texts.tt, verb)(*args, engine=engine, **kw)
        for verb, question, kw in [
                ("decide", "Late?", {{}}),
                ("choose", "Which?", {{"options": ["billing", "shipping"]}}),
                ("score", "How urgent?", {{"levels": ["Routine.", "Soon.", "Now."]}}),
                ("tag", "Which kinds?", {{"labels": ["bill", "ship"]}})]:
            result = call(verb, question, **kw)
            values = [None if value is pd.NA else value for value in result.value.to_list()]
            if shape == "polars":
                assert result.value.dtype == dict(decide=pl.Boolean, choose=pl.String, score=pl.Float64, tag=pl.List(pl.String))[verb]
                assert result.probability.dtype == pl.Float64
            if verb == "tag":
                values = [list(value) if value is not None else None for value in values]
            assert result.value.name == "body"
            if shape == "pandas": assert result.value.index.to_list() == [9, 9, 2]
            assert result.facts["records"] == 2
            assert list(result.positions) == [0, 2]
            print(verb, json.dumps(values))
        kept = call("filter", "Late?")
        print("filter", kept.value.to_list(), kept.value.name)
        if shape == "pandas": assert kept.value.index.to_list() == [9, 2]
        ranked = call("rank", "Relevant?")
        print("rank", [(row["index"], row["record"], row["probability"]) for row in (ranked.value.to_list() if shape == "polars" else ranked.value)])
        found = call("find", "Which?")
        print("find", dict(found.value[0] if shape == "polars" else found.value), [list(row.answer.probabilities.items()) for row in found.details])
        form = {{"version": 1, "questions": {{"late": {{"decide": "Late?"}}}}}}
        annotated = call("annotate", form)
        print("annotate", list(annotated.value.to_list()[0]), annotated.facts["records"])
        if shape == "pandas": assert annotated.value.index.to_list() == [9, 9, 2]
        recognized = (texts.tt.recognize(engine=engine, kinds=["note"]) if shape == "pandas"
                      else engine.recognize(texts, kinds=["note"]))
        rows = recognized.value.to_list()
        if shape == "polars":
            assert recognized.value.dtype.base_type() == pl.Struct
            assert recognized.value.dtype.to_schema()["entities"].base_type() == pl.List
            assert recognized.value.dtype.to_schema()["entities"].inner.base_type() == pl.Struct
            assert annotated.value.dtype == pl.Struct({{"late": pl.Boolean}})
            assert ranked.value.dtype == pl.Struct({{"index": pl.Int64, "record": pl.String, "probability": pl.Float64}})
            assert found.value.dtype == pl.Struct({{"index": pl.Int64, "unit": pl.String, "probability": pl.Float64}})
            assert kept.value.dtype == texts.dtype
            rows = [None] * len(texts)
            for at, row in zip(recognized.positions, recognized.results): rows[at] = row.value
        assert rows[1] is None
        print("recognize", [[(e.text, e.start, e.end, e.kind) for e in row.entities]
                             if row is not None else None for row in rows])
        entities = (pd.Series([("Ada", "person"), None, ("Bo", "person")],
                             index=[9,9,2], name="entities", dtype=object)
                    if shape == "pandas" else pl.Series("entities",
                        [('Ada', 'person'), None, ('Bo', 'person')], dtype=pl.Object))
        related = (entities.tt.relate(engine=engine, relations={{"knows": ("person", "person")}})
                   if shape == "pandas" else engine.relate(entities, relations={{"knows": ("person", "person")}}))
        if shape == "polars":
            assert related.value.dtype.base_type() == pl.Struct
            assert related.value.dtype.to_schema()["source"] == pl.Struct({{"kind": pl.String, "name": pl.String}})
        print("relate", [(e.relation, e.source.name, e.target.name) for e in related.results[0].value])
        if shape == "polars":
            replay = tt.Engine(batch=1, cache=False, replay=RECORDING)
            for verb, args, controls, first in [
                    ("filter", ["Late?", texts], {{}}, kept),
                    ("rank", ["Relevant?", texts], {{}}, ranked),
                    ("find", ["Which?", texts], {{}}, found),
                    ("annotate", [form, texts], {{}}, annotated),
                    ("recognize", [texts], {{"kinds": ["note"]}}, recognized),
                    ("relate", [entities], {{"relations": {{"knows": ("person", "person")}}}}, related)]:
                second = getattr(replay, verb)(*args, **controls)
                assert second.value.dtype == first.value.dtype
                assert second.value.to_list() == first.value.to_list()
                assert second.facts.requests_sent == 0
                assert all(type(row) is type(original) for row, original in zip(second.results, first.results))
                detailed = getattr(replay, verb)(*args, details=True, **controls)
                assert detailed.facts.requests_sent == 0
                detailed_rows = detailed.value if isinstance(detailed.value, list) else detailed.value.to_list()
                assert detailed_rows[0] is detailed.results[0]
                frame = pl.DataFrame({{"body": entities if verb == "relate" else texts}})
                framed_args = [frame] if verb in ("recognize", "relate") else [args[0], frame]
                framed = getattr(replay, verb)(*framed_args, on="body", **controls)
                assert framed.facts.requests_sent == 0
                if verb == "filter": assert framed.value.to_dicts() == [{{"body": "one note"}}] * 2
                elif verb == "annotate":
                    assert framed.value["late"].dtype == pl.Boolean
                    assert framed.value["late"].to_list() == [True, None, True]
                elif verb == "recognize":
                    assert framed.value["strength"].dtype == pl.Float64
                    assert framed.value["text"].to_list() == ["one note", "one note"]
                    assert framed.value["row"].to_list() == [1, 3]
                else:
                    assert framed.value.dtype == first.value.dtype
                    assert framed.value.to_list() == first.value.to_list()
    """.replace("RECORDING", repr(str(tmp_path / "recording"))), child_env(backend, tmp_path))
    assert printed.splitlines() == [
        'decide [true, null, true]', 'choose ["billing", null, "billing"]',
        'score [0.15, null, 0.15]', 'tag [["bill", "ship"], null, ["bill", "ship"]]',
        "filter ['one note', 'one note'] body",
        "rank [(0, 'one note', 0.9), (2, 'one note', 0.9)]",
        "find {'index': 0, 'unit': 'one note', 'probability': 0.9} [[('u001', 0.9), ('u002', 0.1)]]",
        "annotate ['late'] 2",
        "recognize [[('one note', 0, 8, 'note')], None, [('one note', 0, 8, 'note')]]",
        "relate [('knows', 'Ada', 'Bo'), ('knows', 'Bo', 'Ada')]",
    ]
    assert backend.count() == 19


def test_series_filter_replay_retains_original_rows_and_sends_nothing(backend, tmp_path):
    case = next(case for case in json.loads(CORPUS.read_text())["cases"] if case["id"] == "27-decide-many")
    folder = tmp_path / "recording"
    printed = run(f"""
        import pandas as pd, thinkthen as tt, thinkthen.pandas
        texts = { [row['evidence'] for row in case['exchanges']]!r}
        source = pd.Series(texts, index=[9, 9, 2, 2, 1], name="body")
        question = {case['question']!r}
        engine = tt.Engine(batch=1, cache=False, record={str(folder)!r})
        first = source.tt.filter(question, engine=engine)
        print(first.value.index.to_list(), first.facts['requests_sent'])
        replay = tt.Engine(batch=1, cache=False, replay={str(folder)!r})
        second = source.tt.filter(question, engine=replay)
        assert second.value.equals(first.value)
        print(second.facts['requests_sent'], second.facts['cache_answers'])
    """, child_env(backend, tmp_path, "case/27-decide-many"))
    assert printed.splitlines() == ['[9, 2, 2] 5', '0 0']
    assert backend.count() == 5


def test_base_import_does_not_require_or_import_pandas(backend, tmp_path):
    assert run("""
        import sys
        class RefusePandas:
            def find_spec(self, fullname, path=None, target=None):
                if fullname == 'pandas': raise ImportError('pandas is absent')
        sys.meta_path.insert(0, RefusePandas())
        import thinkthen
        assert 'pandas' not in sys.modules
        print('base import works')
    """, child_env(backend, tmp_path)).strip() == 'base import works'
    assert backend.count() == 0


def test_series_find_reads_the_whole_candidate_set_and_keeps_original_positions(backend, tmp_path):
    printed = run("""
        import pandas as pd, thinkthen as tt, thinkthen.pandas
        units = pd.Series(['First passage.', None, 'Second passage.', 'Third passage.'],
                          index=[9, 9, 2, 2], name='body')
        call = units.tt.find('Which passage answers the question?', none=True,
                             engine=tt.Engine(cache=False))
        print(dict(call.value))
        print([list(row.answer.probabilities.items()) for row in call.details])
    """, child_env(backend, tmp_path, "case/18-find-second"))
    assert printed.splitlines() == [
        "{'index': 2, 'unit': 'Second passage.', 'probability': 0.8}",
        "[[('none', 0.05), ('u001', 0.1), ('u002', 0.8), ('u003', 0.05)]]",
    ]
    assert backend.count() == 1


def test_series_recognition_keeps_saved_spans_and_relation_endpoints(backend, tmp_path):
    case = next(case for case in json.loads(CORPUS.read_text())["cases"] if case["id"] == "42-recognize-C01-relations")
    printed = run(f"""
        import pandas as pd, thinkthen as tt, thinkthen.pandas
        source = pd.Series([{case['text']!r}, None, {case['text']!r}], index=[9,9,2], name='body')
        call = source.tt.recognize({case['question']!r}, engine=tt.Engine(cache=False))
        assert call.value.index.equals(source.index) and call.value.name == source.name
        assert call.value.iloc[1] is None
        for at in (0, 2):
            found = call.value.iloc[at]
            print([(e.text, e.start, e.end, e.kind) for e in found.entities])
            print([(e.relation, e.source.text, e.target.text) for e in found.relations])
        assert set(call.positions) == {{0, 2}}
        print(call.facts['records'], call.facts['requests_sent'])
    """, child_env(backend, tmp_path, "case/42-recognize-C01-relations"))
    assert printed.splitlines()[:4] == 2 * [
        "[('Maria Chen', 0, 10, 'person'), ('Northwind Freight', 18, 35, 'organization'), ('Chicago', 39, 46, 'place')]",
        "[('works_for', 'Maria Chen', 'Northwind Freight')]",
    ]
    assert printed.splitlines()[4] == '2 6'
    assert backend.count() == 6


@pytest.mark.parametrize("shape", ["pandas", "polars"])
def test_empty_invalid_expired_and_cancelled_series_calls_send_nothing(backend, tmp_path, shape):
    printed = run("""
        import pandas as pd, polars as pl, thinkthen as tt, thinkthen.pandas, thinkthen.polars
        shape = SHAPE
        def series(values):
            return pd.Series(values, name="body", dtype=object) if shape == "pandas" else pl.Series("body", values, dtype=pl.Object)
        engine = tt.Engine(cache=False)
        empty = series([None, None])
        for verb in ('decide', 'choose', 'score', 'tag', 'filter', 'rank', 'annotate', 'recognize'):
            kw = {'engine': engine}
            args = ['Late?']
            if verb == 'choose': kw['options'] = ['a', 'b']
            if verb == 'score': kw['levels'] = ['low', 'high']
            if verb == 'tag': kw['labels'] = ['a', 'b']
            if verb == 'annotate': args = [{'version':1, 'questions': {'late': {'decide':'Late?'}}}]
            if verb == 'recognize': args = []
            call = getattr(empty.tt, verb)(*args, **kw)
            assert call.facts['records'] == call.facts['requests_sent'] == 0
            if shape == "polars" and verb in ('decide', 'choose', 'score', 'tag'):
                assert call.value.dtype == dict(decide=pl.Boolean, choose=pl.String, score=pl.Float64, tag=pl.List(pl.String))[verb]
                assert call.probability.dtype == pl.Float64
        source = series(['a', 'b'])
        token = tt.CancelToken(); token.cancel()
        for verb, kw in [('filter', {'deadline_ms':0}), ('rank', {'token':token}),
                         ('find', {'deadline_ms':0})]:
            try: getattr(source.tt, verb)('Which?', engine=engine, **kw)
            except (tt.DeadlineError, tt.Cancelled) as error: print(verb, error.kind)
            else: raise AssertionError('control was ignored')
        try: series(['a', 1]).tt.rank('Which?', engine=engine)
        except tt.UsageError as error: print(error.kind)
        else: raise AssertionError('bad evidence was accepted')
    """.replace("SHAPE", repr(shape)), child_env(backend, tmp_path))
    assert printed.splitlines() == ['filter deadline', 'rank cancelled', 'find deadline', 'usage']
    assert backend.count() == 0
