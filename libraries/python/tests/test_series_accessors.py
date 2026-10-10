"""Public optional Series dispatch, using independently expected saved replies."""
import json
import pathlib
import pytest
from conftest import child_env, run, REPO
from backend_settings import configuration, isolated

CORPUS = REPO / "conformance/cases.json"


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
            assert [(edge.source.name, edge.target.name) for edge in related.value] == [('Ada', 'Bo'), ('Bo', 'Ada')]
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
        engine = tt.Engine(batch=1, cache=False)
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
        print("rank", [(row["index"], row["record"], row["probability"]) for row in ranked.value])
        found = call("find", "Which?")
        print("find", dict(found.value), [list(row.answer.probabilities.items()) for row in found.details])
        form = {{"version": 1, "questions": {{"late": {{"decide": "Late?"}}}}}}
        annotated = call("annotate", form)
        print("annotate", list(annotated.value.to_list()[0]), annotated.facts["records"])
        if shape == "pandas": assert annotated.value.index.to_list() == [9, 9, 2]
        recognized = (texts.tt.recognize(engine=engine, kinds=["note"]) if shape == "pandas"
                      else engine.recognize(texts, kinds=["note"]))
        rows = recognized.value.to_list()
        assert rows[1] is None
        print("recognize", [[(e.text, e.start, e.end, e.kind) for e in row.entities]
                             if row is not None else None for row in rows])
        entities = (pd.Series([("Ada", "person"), None, ("Bo", "person")],
                             index=[9,9,2], name="entities", dtype=object)
                    if shape == "pandas" else pl.Series("entities",
                        [('Ada', 'person'), None, ('Bo', 'person')], dtype=pl.Object))
        related = (entities.tt.relate(engine=engine, relations={{"knows": ("person", "person")}})
                   if shape == "pandas" else engine.relate(entities, relations={{"knows": ("person", "person")}}))
        print("relate", [(e.relation, e.source.name, e.target.name) for e in related.value])
    """, child_env(backend, tmp_path))
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
