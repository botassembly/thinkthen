"""pandas columns and frames (ticket 0122), in both pandas lanes.

Each engine call runs in a child on its own loopback backend. Values are
checked by position against the list form in the same child. Where pandas 2
and pandas 3 differ, each lane asserts its own row. The generic arm answers
every text alike. So ``recognize``, whose offsets follow the text, and the
index test's row on shared case 27, whose answers differ by text, show values
paired to the wrong rows.
"""

import json
import pathlib
import time

import pandas
import pytest

from conftest import Backend, child_env, one_request, question_keys, run, start
from test_call import capturing_filter_listener, quoted_record


@pytest.mark.parametrize("shape", ["pandas_series", "pandas_frame", "polars_frame"])
def test_portable_questions_ride_one_request_in_public_column_and_frame_shapes(backend, tmp_path, shape):
    """The content cut is gone, by ADR 0111, so the five records ride one
    request that keeps the fixture questions, and each row names its key."""
    fixture = pathlib.Path(__file__).resolve().parents[3] / "specification/fixtures/batching"
    corpus = json.loads((fixture / "portable-records.json").read_text())
    # A column and a one-question frame annotation send the same quoted bodies.
    expected = [(fixture / f"portable-{index}.request.json")
                .read_bytes().removesuffix(b"\n") for index in (1, 2, 3)]
    with capturing_filter_listener() as (url, bodies):
        env = child_env(backend, tmp_path)
        env["THINKTHEN_BASE_URL"] = url.removesuffix("/systemone")
        printed = run(f"""
            import json, pandas as pd, polars as pl, thinkthen as tt
            texts = {corpus['texts']!r}
            shape = {shape!r}
            engine = tt.Engine(cache=False, throttle=1)
            if shape == 'pandas_series':
                source = pd.Series(texts, index=[9, 5, 7, 3, 1], name='body', dtype='string[pyarrow]')
                call = engine.decide({corpus['question']!r}, source)
                values = call.value.to_list()
                order = call.value.index.to_list()
            else:
                source = (pd.DataFrame({{'body': texts}}, index=[9, 5, 7, 3, 1]) if shape == 'pandas_frame'
                          else pl.DataFrame({{'body': texts}}))
                form = {{'version': 1, 'questions': {{'answer': {{'decide': {corpus['question']!r}}}}}}}
                call = engine.annotate(form, source, on='body')
                values = call.value['answer'].to_list()
                order = call.value.index.to_list() if shape == 'pandas_frame' else list(range(5))
            print(json.dumps([values, order, [call.facts["records"], call.facts["requests_sent"]],
                              [[row['index'], list(row['requests'])] for row in call.details]]))
        """, env)
        values, order, facts, details = json.loads(printed)
        keys = one_request(url, bodies, expected, "jev-latest")
        assert values == [True] * 5
        assert order == ([9, 5, 7, 3, 1] if shape != "polars_frame" else list(range(5)))
        assert facts == [5, 1]
        assert details == [[i, [key]] for i, key in enumerate(keys)]
    assert backend.count() == 0

THREE = pandas.__version__.startswith("3.")
TESTS = str(pathlib.Path(__file__).resolve().parent)
LISTS = ("filter, rank, find, and relate read a list of str, not a column, and annotate and "
         "recognize read a column only from a Polars or pandas frame with on=. "
         "Pass column.to_list()")
SETUP = f"""
    import sys
    sys.path.insert(0, {TESTS!r})
    import pandas as pd, pyarrow as pa, thinkthen as tt
    engine = tt.Engine(batch=1, cache=False)
    late = tt.question(decide="Is it late?")
    team = tt.question(choose="Which team?", options=["billing", "shipping"])
    urgent = tt.question(score="How urgent?", levels=["Routine.", "Soon.", "Now."])
    kinds = tt.question(tag="Which kinds?", labels=["bill", "ship"])
    form = {{"version": 1, "questions": {{"late": {{"decide": "Late?"}},
            "team": {{"choose": "Which team?", "options": ["billing", "shipping"]}}}}}}
    texts = ["my card was charged twice", "the box came late", "thanks, all good"]
    VERBS = (("decide", late, "boolean"), ("choose", team, "string"),
             ("score", urgent, "Float64"), ("tag", kinds, "object"))
    def sent():
        return engine.usage()["requests_sent"]
    def said(call):
        before = sent()
        try:
            call()
            print("answered", sent() - before)
        except tt.ThinkThenError as error:
            print(type(error).__name__, error, sent() - before)
    def plain(values):
        return [None if value is pd.NA else value for value in values]
    def listed(verb, asked, rows):
        return [getattr(engine, verb)(asked, text).value for text in rows]
    def names(rows):
        return [[{{"text": one.text, "start": one.start, "end": one.end, "length": one.length,
                  "kind": one.kind, "strength": one.strength}} for one in engine.recognize(text, kinds=["bill", "ship"]).value.entities]
                for text in rows]
"""


def _packed_partial(question):
    instructions = question["instructions"]
    if instructions.endswith("Partial?") and 'The text is "two"' in instructions:
        return {"type": "noul"}
    return None


def test_pandas_series_and_frame_batch_facts(backend, tmp_path):
    """One captured listener proves frame packing and the saved set tier on both pandas lanes."""
    with capturing_filter_listener(_packed_partial) as (url, bodies):
        env = child_env(backend, tmp_path)
        env["THINKTHEN_BASE_URL"] = url.removesuffix("/systemone")
        printed = run("""
        import json, pandas as pd, thinkthen as tt
        engine = tt.Engine(model="jev-latest", cache=False)
        observed = []
        question = tt.question(decide="Is it late?")
        rows = pd.Series(["one", "two", "three"], index=[9, 5, 7], name="body")
        for setting in (None, 1):
            call = engine.decide(question, rows, **({} if setting is None else {"batch": setting}))
            print("series", call.value.index.tolist(), call.value.name, call.value.dtype.name,
                  call.value.tolist(), call.facts["records"], call.facts["requests_sent"],
                  [detail["index"] for detail in call.details])
            observed.append([list(detail["requests"]) for detail in call.details])
        frame = rows.to_frame().assign(number=[4, 6, 8])
        saved = {"version": 1, "batch": 1,
                 "questions": {"late": {"decide": "Is it late?"}}}
        plain = {"version": 1, "questions": saved["questions"]}
        for label, form, controls in (("default", plain, {}), ("saved", saved, {}),
                                      ("typed", saved, {"batch": "max"})):
            call = engine.annotate(form, frame, on="body", **controls)
            value = call.value
            print("frame", label, value.index.tolist(), list(value.columns), value["number"].tolist(),
                  value["late"].dtype.name, value["late"].tolist(),
                  value["failed"].isna().all(), call.facts["records"],
                  call.facts["requests_sent"], [detail["index"] for detail in call.details])
            observed.append([list(detail["requests"]) for detail in call.details])
        engine_max = tt.Engine(model="jev-latest", batch="max", cache=False)
        overridden = engine_max.annotate(saved, frame, on="body")
        print("engine", overridden.facts["records"], overridden.facts["requests_sent"])
        observed.append([list(detail["requests"]) for detail in overridden.details])
        partial = {"version": 1, "questions": {
            "late": {"decide": "Is it late?"}, "partial": {"decide": "Partial?"}}}
        mixed = engine.annotate(partial, frame, on="body")
        print("partial", mixed.value["partial"].dtype.name,
              [None if pd.isna(cell) else bool(cell) for cell in mixed.value["partial"]],
              mixed.value["failed"].tolist(), mixed.facts["records"], mixed.facts["requests_sent"])
        print("partial details", [(detail["index"], detail["member"], detail.get("answer"),
                                   dict(detail["failed"]) if "failed" in detail else None,
                                   detail["requests_sent"]) for detail in mixed.details])
        observed.append([list(detail["requests"]) for detail in mixed.details])
        context = engine.decide(question, rows, context="review this claim")
        print("context", context.facts["records"], context.facts["requests_sent"])
        observed.append([list(detail["requests"]) for detail in context.details])
        before = engine.usage()["requests_sent"]
        try:
            engine.annotate(saved, frame, on="body", context="forbidden")
        except tt.UsageError as error:
            print("refused", str(error), engine.usage()["requests_sent"] - before)
        else:
            raise AssertionError("annotate context was accepted")
        print(json.dumps(observed))
        """, env)
        *lines, captured = printed.splitlines()
        assert lines == [
            "series [9, 5, 7] body boolean [False, True, True] 3 1 [0, 1, 2]",
            "series [9, 5, 7] body boolean [False, True, True] 3 3 [0, 1, 2]",
            "frame default [9, 5, 7] ['body', 'number', 'late', 'failed'] [4, 6, 8] boolean "
            "[False, True, True] True 3 1 [0, 1, 2]",
            "frame saved [9, 5, 7] ['body', 'number', 'late', 'failed'] [4, 6, 8] boolean "
            "[False, True, True] True 3 3 [0, 1, 2]",
            "frame typed [9, 5, 7] ['body', 'number', 'late', 'failed'] [4, 6, 8] boolean "
            "[False, True, True] True 3 1 [0, 1, 2]",
            "engine 3 1",
            "partial boolean [False, None, True] [None, {'partial': {'failed': "
            "{'kind': 'backend', 'cause': 'missing_probability'}}}, None] 3 1",
            "partial details [(0, 'late', False, None, 1), (0, 'partial', False, None, 0), "
            "(1, 'late', True, None, 0), (1, 'partial', None, "
            "{'cause': 'missing_probability', 'kind': 'backend'}, 0), "
            "(2, 'late', True, None, 0), (2, 'partial', True, None, 0)]",
            "context 3 1",
            "refused annotate does not take a shared context 0",
        ]
        requests = [json.loads(body) for body in bodies]
        assert [len(request["questions"]) for request in requests] == [
            3, 1, 1, 1, 3, 1, 1, 1, 3, 3, 6, 3]
        singleton = (b'{"state":"Each question quotes the text it asks about.","model":"jev-latest",'
                     b'"questions":{"q1":{"type":"noul","instructions":"The text is \\"one\\". Is it late?"}}}')
        assert singleton in bodies[1:4]
        # Each detail names its own question's key, by ADR 0111.
        def each(body):
            return [[key] for key in question_keys(url, body, "jev-latest")]

        def by_state(items):
            return {quoted_record(json.loads(body)): question_keys(url, body, "jev-latest")[0] for body in items}
        states = ["one", "two", "three"]
        expected_keys = [
            each(bodies[0]),
            [[by_state(bodies[1:4])[state]] for state in states],
            each(bodies[4]),
            [[by_state(bodies[5:8])[state]] for state in states],
            each(bodies[8]),
            each(bodies[9]),
            each(bodies[10]),
            each(bodies[11]),
        ]
        assert json.loads(captured) == expected_keys
        assert bodies[0] != bodies[-1]
    assert backend.count() == 0


def test_each_verb_answers_a_series_as_its_list_does(backend, tmp_path):
    """Proof 1: each verb over each Series kind the lane offers gives the
    caller's class back, named as the input, in the verb's dtype, with the
    list form's values by position. Regression: the tag's plain list comes
    back, or a dtype drifts."""
    printed = run(SETUP + """
    offered = (["str"] if pd.__version__.startswith("3.") else []) + [
        "string[pyarrow]", "object", "category"]
    for dtype in offered:
        series = pd.Series(texts, dtype=dtype, name="body")
        for verb, asked, want in VERBS:
            before = sent()
            got = getattr(engine, verb)(asked, series).value
            sends = sent() - before
            print(dtype, verb, type(got) is pd.Series, got.name, got.dtype.name == want,
                  plain(got.tolist()) == listed(verb, asked, texts), sends)
    """, child_env(backend, tmp_path))
    offered = (["str"] if THREE else []) + ["string[pyarrow]", "object", "category"]
    assert printed.splitlines() == [
        f"{dtype} {verb} True body True True {sends}" for dtype in offered
        for verb, sends in (("decide", 3), ("choose", 3), ("score", 3), ("tag", 3))]


def test_a_failed_question_keeps_its_dtype_and_a_separate_marker(backend, tmp_path):
    """On the malformed arm, the failed answer stays null in a typed column.
    The companion holds the exact marker. A Series stops after its first failed batch."""
    printed = run(SETUP + """
    got = engine.annotate(form, pd.DataFrame({"body": texts[:2]}), on="body").value
    print(got["late"].dtype.name, got["team"].dtype.name, got["team"].isna().all(),
          got["failed"].dtype.name, got["failed"].tolist())
    said(lambda: engine.choose(team, pd.Series(texts[:2])).value)
    """, child_env(backend, tmp_path, "arm/malformed/missing_answer"))
    marker = {"team": {"failed": {"kind": "backend", "cause": "missing_answer"}}}
    assert printed.splitlines() == [
        f"boolean string True object {[marker, marker]}",
        "BackendError the reply was refused: the response carries no answer for question `q1` 1"]


def test_a_frame_gains_answer_columns_and_keeps_its_own(backend, tmp_path):
    """Proof 1: ``annotate`` keeps every original column equal and adds the
    set's columns with the list form's values by position. ``recognize``
    adds one ``names`` column whose lists equal each text alone. The caller's
    frame is unchanged."""
    printed = run(SETUP + """
    frame = pd.DataFrame({"body": texts, "n": [1, 2, 3],
                          "cat": pd.Series(["x", "y", "x"], dtype="category")})
    copy = frame.copy()
    got = engine.annotate(form, frame, on="body").value
    wanted = engine.annotate(form, texts).value
    print(list(got.columns), got[list(frame.columns)].equals(frame),
          [got[name].dtype.name for name in ("late", "team")],
          [dict(zip(("late", "team"), plain(row))) for row in
           zip(got["late"].tolist(), got["team"].tolist())] == wanted,
          got["failed"].isna().all())
    found = engine.recognize(frame, kinds=["bill", "ship"], on="body").value
    print(list(found.columns), found["names"].dtype.name, found["names"].tolist() == names(texts),
          sum(map(len, names(texts))) > 0, frame.equals(copy))
    """, child_env(backend, tmp_path))
    assert printed.splitlines() == [
        "['body', 'n', 'cat', 'late', 'team', 'failed'] True ['boolean', 'string'] True True",
        "['body', 'n', 'cat', 'names'] object True True True",
    ]


def test_the_callers_index_survives(backend, tmp_path):
    """Proof 2 and R3-19: on a non-default index, a row ``MultiIndex``,
    repeated labels, and an empty input, every answer keeps the caller's
    index and name, with the list form's values by position. On unique labels
    a join holds no missing value. On shared case 27, whose five answers
    differ by text, ``decide`` and ``decide_many`` keep each answer on its
    row. Regression: a fresh ``RangeIndex`` turns the join all missing, and
    values paired to the wrong rows move a name or an answer."""
    printed = run(SETUP + """
    import json, os
    case = [one for one in json.load(open(os.path.join(sys.path[0], "..", "..", "..",
            "conformance", "cases.json")))["cases"] if one["id"] == "27-decide-many"][0]
    refund = tt.question(**case["question"])
    records = pd.Series([one["evidence"] for one in case["exchanges"]], index=[50, 40, 30, 20, 10])
    base = os.environ["THINKTHEN_BASE_URL"].replace("/generic/", "/case/27-decide-many/")
    for verb in ("decide",):
        got = getattr(tt.Engine(base_url=base, batch=1, cache=False), verb)(refund, records).value
        print("case 27", verb, got.index.equals(records.index), got.tolist())
    indexes = {"labels": [5, 7, 9], "rows": pd.MultiIndex.from_tuples(
        [("a", 1), ("a", 2), ("b", 1)]), "repeated": [1, 1, 2], "empty": []}
    for kind, index in indexes.items():
        rows = texts[:len(index)]
        series = pd.Series(rows, index=index, name="body", dtype=object)
        frame = series.to_frame()
        joins = kind in ("labels", "rows")
        for verb, asked, _ in VERBS:
            got = getattr(engine, verb)(asked, series).value
            joined = frame.join(got.rename("x"))["x"].notna().all() if joins else "-"
            print(kind, verb, got.index.equals(series.index), got.name,
                  plain(got.tolist()) == listed(verb, asked, rows), joined)
        before = sent()
        got = engine.annotate(form, frame, on="body").value
        wanted = engine.annotate(form, rows).value
        joined = frame.join(got[["late"]])["late"].notna().all() if joins else "-"
        print(kind, "annotate", got.index.equals(frame.index), [
            dict(zip(("late", "team"), plain(row))) for row in
            zip(got["late"].tolist(), got["team"].tolist())] == wanted, joined)
        found = engine.recognize(frame, kinds=["bill", "ship"], on="body").value
        print(kind, "recognize", found.index.equals(frame.index),
              found["names"].tolist() == names(rows), sent() - before > 0)
    """, child_env(backend, tmp_path))
    verbs = ("decide", "choose", "score", "tag")
    wanted = [f"case 27 {verb} True [True, False, True, True, False]"
              for verb in ("decide",)]
    for kind in ("labels", "rows", "repeated", "empty"):
        joined = "True" if kind in ("labels", "rows") else "-"
        wanted += [f"{kind} {verb} True body True {joined}" for verb in verbs]
        wanted += [f"{kind} annotate True True {joined}",
                   f"{kind} recognize True True {kind != 'empty'}"]
    assert printed.splitlines() == wanted


def test_the_edge_rows_that_answer(backend, tmp_path):
    """The edge table's answering rows: the empty Series and frame with zero
    sends, integer column labels, and a question named ``self``, which a
    keyword ``assign`` would refuse after every send."""
    printed = run(SETUP + """
    before = sent()
    got = engine.decide(late, pd.Series([], dtype=object, name="body", index=[])).value
    print("empty", type(got).__name__, got.name, got.dtype.name, len(got), sent() - before)
    frame = pd.DataFrame({"body": []})
    got = engine.annotate(form, frame, on="body").value
    print("empty frame", frame["body"].dtype.name, list(got.columns),
          [got[name].dtype.name for name in got.columns], len(got), sent() - before)
    before = sent()
    got = engine.annotate(form, pd.DataFrame({3: texts}), on=3).value
    print("label 3", list(got.columns), got[3].tolist() == texts, sent() - before)
    before = sent()
    got = engine.annotate({"version": 1, "questions": {"self": {"decide": "Late?"}}},
                          pd.DataFrame({"body": texts}), on="body").value
    print("self", list(got.columns), got["self"].dtype.name, sent() - before)
    """, child_env(backend, tmp_path))
    assert printed.splitlines() == [
        "empty Series body boolean 0 0",
        "empty frame float64 ['body', 'late', 'team', 'failed'] ['float64', 'boolean', 'string', 'object'] 0 0",
        "label 3 [3, 'late', 'team', 'failed'] True 3",
        "self ['body', 'self', 'failed'] boolean 3",
    ]


def test_every_pandas_refusal_sends_nothing(backend, tmp_path):
    """Proof 5, with R1-6 (the ``filter`` row) and R2-11 (the frame to
    ``annotate`` without ``on=``): every row pins its whole sentence, and
    the backend counts no request. The first two rows show the route: pandas
    named calls use the shared native header admission on both pandas versions."""
    printed = run(SETUP + """
    column = pd.Series(texts)
    frame = pd.DataFrame({"body": texts})
    said(lambda: engine.decide(late, pd.Series([1, 2])).value)
    said(lambda: engine.decide(late, pd.Series(["a", 1], dtype=object)).value)
    said(lambda: engine.details(late, column).value)
    said(lambda: engine.decide(late, frame).value)
    said(lambda: engine.annotate(form, frame).value)
    said(lambda: engine.annotate(form, frame, on="missing").value)
    said(lambda: engine.annotate(form, pd.DataFrame([texts, texts], index=["body", "body"]).T,
                                 on="body").value)
    said(lambda: engine.annotate(form, pd.DataFrame(
        {("body", "x"): texts}), on=("body", "x")).value)
    said(lambda: engine.annotate(form, frame.assign(late=1), on="body").value)
    said(lambda: engine.annotate(form, frame.assign(failed=1), on="body").value)
    said(lambda: engine.annotate({"version": 1, "questions": {"failed": {"decide": "Late?"}}}, frame, on="body").value)
    said(lambda: engine.recognize(frame, kinds=["x"], relations={"r": ("x", "x")}, on="body").value)
    said(lambda: engine.annotate(form, frame, on=["body"]).value)
    said(lambda: engine.recognize(frame.assign(names=1), kinds=["x"], on="body").value)
    said(lambda: engine.annotate(form, pa.table({"body": texts}), on="body").value)
    """, child_env(backend, tmp_path))
    assert printed.splitlines() == [
        "UsageError invalid canonical request 0",
        "UsageError invalid canonical request 0",
        "UsageError details reads one str, not a column 0",
        "UsageError a data frame is not a column; pass df[\"name\"], or annotate with on= 0",
        'UsageError a data frame is not a column; pass df["name"], or annotate with on= 0',
        "UsageError the frame has no column named 'missing' 0",
        "UsageError the frame has more than one column named 'body' 0",
        "UsageError on= reads a frame whose column labels have one level 0",
        "UsageError the frame already has a column named 'late'; rename it first 0",
        "UsageError the frame already has a column named 'failed'; rename it first 0",
        "UsageError the question name failed is reserved for frame failures 0",
        "UsageError recognize with on= takes no relations; ask them of one text 0",
        "UsageError on= takes one column label, such as \"body\" 0",
        "UsageError the frame already has a column named 'names'; rename it first 0",
        "UsageError annotate with on= takes a Polars or pandas DataFrame; "
        "a list of str takes no on= 0",
    ]
    assert backend.count() == 0


@pytest.mark.stress
def test_a_series_runs_at_the_lists_throttle(backend, tmp_path):
    """Proof 3: at 100 ms a reply and throttle 8, 200 texts take about 2.5 s
    as a list and as a Series on each route the lane offers, within 5 percent
    of the list, with 200 sends each and equal answers. Regression: a
    per-row call holds 1 in flight and runs eight times longer."""
    routes = '["str", "category"]' if THREE else '["object"]'
    printed = run(SETUP + f"""
    import time
    engine = tt.Engine(throttle=8, batch=1, cache=False)
    rows = [f"note {{n}}" for n in range(200)]
    def timed(records):
        began = time.monotonic()
        answers = engine.decide(late, records).value
        return time.monotonic() - began, plain(list(answers))
    wall, answers = timed(rows)
    print(wall)
    for dtype in {routes}:
        seconds, got = timed(pd.Series(rows, dtype=dtype))
        print(dtype, seconds, got == answers)
    """, child_env(backend, tmp_path, "arm/delay/100"))
    first, *routes = printed.splitlines()
    assert 2.4 < float(first) < 4.0
    assert [route.split()[2] for route in routes] == ["True"] * len(routes)
    for route in routes:
        assert abs(float(route.split()[1]) - float(first)) / float(first) < 0.05, route
    assert backend.count() == 200 * (1 + len(routes))


def test_a_series_stops_at_the_held_request(tmp_path):
    """Proof 3: a list and each Series route stop pulling after their first
    held request. Each form gets its own backend."""
    forms = ["rows", "pd.Series(rows, dtype='category')"] + (
        ["pd.Series(rows, dtype='str')"] if THREE else ["pd.Series(rows, dtype=object)"])
    for records in forms:
        backend = Backend()
        try:
            child = start(SETUP + "    engine = tt.Engine(throttle=8, batch=1, cache=False)\n"
                          "    rows = [f'note {n}' for n in range(20)]\n"
                          f"    engine.decide(late, {records}).value\n",
                          child_env(backend, tmp_path, "arm/held"))
            assert backend.wait(1) == 1
            time.sleep(0.3)
            assert backend.count() == 1, records
            backend.release()
            assert child.wait(timeout=60) == 0, child.stderr.read()
        finally:
            backend.close()


def test_pandas_text_uses_the_export_its_version_offers(backend, tmp_path):
    """Proof 4: on pandas 3, ``_arrow_probe`` over a ``str`` and a
    ``string[pyarrow]`` Series reads each text where pandas' own export puts
    it. The pandas 2 lane skips it; its precondition shows no export. Regression:
    a copy before the hand-off moves every text."""
    if not THREE:
        assert not hasattr(pandas.Series(["a"], dtype="string[pyarrow]"), "__arrow_c_stream__")
        assert backend.count() == 0
        return
    printed = run(SETUP + """
    import ctypes
    from arrow_c import pull
    rows = [f"a text of row {n}" for n in range(1000)]
    for dtype in ("str", "string[pyarrow]"):
        series = pd.Series(rows, dtype=dtype)
        _, schema, [batch] = pull(series.__arrow_c_stream__())
        width = 8 if schema.format == b"U" else 4
        ends = ctypes.string_at(batch.buffers[1], width * (batch.offset + batch.length + 1))
        start = lambda row: int.from_bytes(ends[width * (batch.offset + row):][:width], "little")
        pandas_sees = [batch.buffers[2] + start(row) for row in range(batch.length)]
        batch.release(ctypes.byref(batch))
        print(dtype, tt._thinkthen._arrow_probe(series) == pandas_sees, len(pandas_sees))
    """, child_env(backend, tmp_path))
    assert printed.splitlines() == ["str True 1000", "string[pyarrow] True 1000"]


@pytest.mark.parametrize("shape", ["pandas", "polars"])
def test_named_frame_calls_keep_native_rows_selectors_and_original_positions(backend, tmp_path, shape):
    printed = run("""
        import asyncio, json, os, pathlib, pickle, pandas as pd, polars as pl, thinkthen as tt
        from thinkthen import complete as c
        shape = SHAPE
        def frame(data):
            return pd.DataFrame(data, index=[9, 9, 2]) if shape == 'pandas' else pl.DataFrame(data, strict=False)
        source = frame({'body': ['first note', None, 'second note'], 'number': [1,2,3]})
        engine = tt.Engine(cache=False)
        for verb, question, kw in [('decide','Late?',{}),
                ('choose','Which?',{'options':['billing','shipping']}),
                ('score','Urgent?',{'levels':['low','high']}),
                ('tag','Kinds?',{'labels':['bill','ship']})]:
            done = getattr(engine, verb)(question, source, on='body', **kw)
            assert isinstance(done, tt.Result) and done.positions == (0,2)
            assert [row.index for row in done.results] == [0,1]
            assert done.value.to_list()[1] is None and done.value.name == 'body'
            assert done.facts.requests_sent == 1
            assert pickle.loads(pickle.dumps(done)).results == done.results
        kept = engine.filter('Late?', source, on='body')
        assert kept.value['number'].to_list() == [1,3]
        ranked = engine.rank('Late?', source, on='body')
        assert [row['index'] for row in ranked.value] == [0,2] and ranked.facts.requests_sent == 1
        found = engine.find('Which?', source, on='body')
        assert found.value['index'] == 0 and found.facts.requests_sent == 1
        declared = pathlib.Path(os.environ['HOME']) / 'recognition.json'
        declared.write_text('{"version":1,"recognize":{"kinds":{"note":null}}}')
        recognized = engine.recognize(source, declared, on='body')
        assert recognized.positions == (0,2) and recognized.facts.requests_sent == 4
        assert recognized.results[0].value.entities[0].text == 'first note'
        if shape == 'pandas':
            assert recognized.value.index.to_list() == [9,9,2]
            assert recognized.value['names'].iloc[1] is None
            assert recognized.value['number'].to_list() == [1,2,3]
        else:
            assert recognized.value['row'].to_list() == [1,3]
        form = {'version':1, 'questions':{'late':{'decide':'Late?'},
                    'team':{'choose':'Which?', 'options':['billing','shipping']},
                    'urgent':{'score':'Urgent?', 'levels':['low','high']},
                    'tags':{'tag':'Kinds?', 'labels':['bill','ship']}}}
        annotation_file = pathlib.Path(os.environ['HOME']) / 'questions.json'
        annotation_file.write_text(json.dumps(form))
        annotated = engine.annotate(annotation_file, source, on='body')
        assert isinstance(annotated, tt.Result) and annotated.positions == (0,2)
        assert annotated.value['number'].to_list() == [1,2,3]
        empty = source.iloc[:0] if shape == 'pandas' else source.head(0)
        nulls = frame({'body':[None,None,None], 'number':[1,2,3]})
        for blank in (empty, nulls):
            selected = engine.filter('Late?', blank, on='body')
            assert selected.facts.requests_sent == 0 and list(selected.value.columns) == list(blank.columns)
            assert len(selected.value) == 0
            got = engine.annotate(form, blank, on='body')
            assert got.facts.requests_sent == 0 and got.positions == ()
            if shape == 'pandas':
                assert [str(got.value[name].dtype) for name in form['questions']] == ['boolean','string','Float64','object']
            else:
                assert [got.value.schema[name] for name in form['questions']] == [pl.Boolean,pl.String,pl.Float64,pl.List(pl.String)]
            assert got.value['failed'].isna().all() if shape == 'pandas' else got.value['failed'].is_null().all()
        collision = source.assign(late=1) if shape == 'pandas' else source.with_columns(pl.lit(1).alias('late'))
        before = engine.usage()['requests_sent']
        try: engine.annotate(form, collision, on='body')
        except tt.UsageError: pass
        else: raise AssertionError('annotation overwrote a column')
        assert engine.usage()['requests_sent'] == before
        again = pickle.loads(pickle.dumps(recognized))
        assert again.results == recognized.results
        async_done = asyncio.run(engine.asyncio.decide('Late?', source, on='body'))
        assert async_done.positions == (0,2)
        contextual = frame({'body':['one',None,'two'], 'ctx':['review','skip',''],
                            'shortlist':[['shipping','billing'],['billing','shipping'],['billing','shipping']]})
        picked = engine.choose('Which?', contextual, on='body', options=['billing','shipping'],
                               context_field='/ctx', options_field='/shortlist')
        assert picked.results[0].input['body'] == 'one' and picked.positions == (0,2)
        assert picked.value.to_list() == ['shipping',None,'billing']
        assert picked.facts.requests_sent == 2
        entities = frame({'entity':[{'name':'Ada','kind':'person'},None,{'name':'Bo','kind':'person'}]})
        related = engine.relate(entities, on='entity', relations={'knows':('person','person')})
        assert related.facts.requests_sent == 1 and len(related.value) == 2
        items = [c.Item(value={'text':'explicit'}), c.Item(value=None), None]
        explicit = (pd.DataFrame({'body':items}, index=[9,9,2]) if shape == 'pandas'
                    else pl.DataFrame([pl.Series('body',items,dtype=pl.Object)]))
        json_done = engine.decide('Late?', explicit, on='body')
        assert json_done.positions == (0,1)
        assert json_done.results[0].input == {'text':'explicit'} and json_done.results[1].input is None
        assert pickle.loads(pickle.dumps(json_done)).results == json_done.results
        bad_engine = tt.Engine(cache=False, max_retries=0, base_url=os.environ['THINKTHEN_BASE_URL'].replace('/generic/', '/arm/malformed/missing_answer/'))
        partial = bad_engine.annotate({'version':1,'questions':{'late':{'decide':'Late?'},'bad':{'decide':'Refund?'}}},source,on='body')
        failure = partial.results[-1].value['bad']
        assert failure.failed.kind == 'backend' and failure.failed.cause == 'missing_answer'
        assert pickle.loads(pickle.dumps(failure)) == failure
        try: bool(failure)
        except TypeError: pass
        else: raise AssertionError('frame coerced an embedded failure')
        before = engine.usage()['requests_sent']
        for bad in [lambda:engine.decide('Late?',source,on='missing'),
                    lambda:engine.recognize(source,on='body',relations={'r':('*','*')}),
                    lambda:engine.decide('',source,on='body')]:
            try: bad()
            except tt.UsageError: pass
            else: raise AssertionError('invalid frame call was accepted')
        assert engine.usage()['requests_sent'] == before
        cached = tt.Engine(cache=pathlib.Path(os.environ['HOME']) / 'owned-cache')
        first = cached.decide('Cached?', source, on='body')
        second = cached.decide('Cached?', source, on='body')
        assert first.facts.requests_sent == 1 and second.facts.requests_sent == 0
        assert first.results[0].answer_id == second.results[0].answer_id
        engine.close()
        assert recognized.results[0].value.entities[0].text == 'first note'
        print('owned frame rows')
    """.replace('SHAPE', repr(shape)), child_env(backend, tmp_path))
    assert printed.strip() == 'owned frame rows'
    assert backend.count() == 19


@pytest.mark.parametrize("shape", ["pandas", "polars"])
def test_saved_member_selectors_read_the_same_json_text_in_eager_feed_source_and_frame(backend, tmp_path, shape):
    case = next(row for row in json.loads((pathlib.Path(__file__).resolve().parents[3] / 'conformance/cases.json').read_text())['cases'] if row['id'] == '18-annotate-two-groups')
    output = run(f"""
        import json, os, pathlib, pandas as pd, polars as pl, thinkthen as tt
        from thinkthen import complete as c
        record = {case['record']!r}
        body = json.dumps(record)
        root = pathlib.Path(os.environ['HOME'])
        question = root / 'questions.json'; question.write_text(json.dumps({case['question_set']!r}))
        document = root / 'document.json'; document.write_text(body)
        source = (pd.DataFrame({{'input':[body], 'other':['keep']}}, index=[9]) if {shape!r} == 'pandas'
                  else pl.DataFrame({{'input':[body], 'other':['keep']}}))
        engine = tt.Engine(cache=False, batch='max')
        for value, controls in [([body], {{}}), (iter([body]), {{}}),
                                (c.Files(paths=(str(document),),unit='file'), {{}}), (source, {{'on':'input'}})]:
            call = engine.annotate(question, value, **controls)
            assert call.results[0].value == {{'summary':True, 'body':True}}
            assert call.facts.requests_sent == 1
            assert call.results[0].input == record
            if controls:
                assert call.value['other'].to_list() == ['keep'] and call.positions == (0,)
            if isinstance(value, c.Files): assert call.results[0].source.file == str(document)
        explicit = engine.annotate({{'version':1,'questions':{{'selected':{{'decide':'Fits?'}}}}}},
                                   [c.Item(value=record)], field=['/summary'])
        assert explicit.results[0].input == record and explicit.results[0].value == {{'selected':True}}
        before = engine.usage()['requests_sent']
        for items, controls in [([body], {{'field':['/summary']}}),
                                ([c.Item(value=body,text=True,images=(c.Image(media='png',data=b''),))], {{}})]:
            try: engine.annotate(question, items, **controls)
            except tt.UsageError: pass
            else: raise AssertionError('annotation accepted unsupported projection or images')
        assert engine.usage()['requests_sent'] == before
        print('shared annotation documents')
    """, child_env(backend, tmp_path))
    assert output.strip() == 'shared annotation documents'
    assert backend.count() == 5
