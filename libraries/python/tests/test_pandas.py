"""pandas columns and frames (ticket 0122), in both pandas lanes.

Each engine call runs in a child on its own loopback backend. Values are
checked by position against the list form in the same child. Where pandas 2
and pandas 3 differ, each lane asserts its own row. The generic arm answers
every text alike. So ``recognize``, whose offsets follow the text, and the
index test's row on shared case 27, whose answers differ by text, show values
paired to the wrong rows.
"""

import pathlib
import time

import pandas
import pytest

from conftest import Backend, child_env, run, start

THREE = pandas.__version__.startswith("3.")
TESTS = str(pathlib.Path(__file__).resolve().parent)
LISTS = ("filter, rank, find, and relate read a list of str, not a column, and annotate and "
         "recognize read a column only from a Polars or pandas frame with on=. "
         "Pass column.to_list()")
NULLS = "the column holds nulls; the engine needs text, so drop or fill them first"
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
        if verb == "decide_many":
            return engine.decide_many(asked, rows).value
        return [getattr(engine, verb)(asked, text).value for text in rows]
    def names(rows):
        return [[{{"text": one.text, "start": one.start, "end": one.end, "length": one.length,
                  "kind": one.kind, "strength": one.strength}} for one in engine.recognize(text, kinds=["bill", "ship"]).value.entities]
                for text in rows]
"""


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
        for verb, asked, want in VERBS + (("decide_many", late, "boolean"),):
            before = sent()
            got = getattr(engine, verb)(asked, series).value
            sends = sent() - before
            print(dtype, verb, type(got) is pd.Series, got.name, got.dtype.name == want,
                  plain(got.tolist()) == listed(verb, asked, texts), sends)
    """, child_env(backend, tmp_path))
    offered = (["str"] if THREE else []) + ["string[pyarrow]", "object", "category"]
    assert printed.splitlines() == [
        f"{dtype} {verb} True body True True {sends}" for dtype in offered
        for verb, sends in (("decide", 3), ("choose", 3), ("score", 3), ("tag", 3),
                            ("decide_many", 3))]


def test_a_failed_question_keeps_its_dtype_and_a_separate_marker(backend, tmp_path):
    """On the malformed arm, the failed answer stays null in a typed column.
    The companion holds the exact marker. A Series verb still raises."""
    printed = run(SETUP + """
    got = engine.annotate(form, pd.DataFrame({"body": texts[:2]}), on="body").value
    print(got["late"].dtype.name, got["team"].dtype.name, got["team"].isna().all(),
          got["failed"].dtype.name, got["failed"].tolist())
    said(lambda: engine.choose(team, pd.Series(texts[:2])).value)
    """, child_env(backend, tmp_path, "arm/malformed/missing_answer"))
    marker = {"team": {"failed": {"kind": "backend", "cause": "missing_answer"}}}
    assert printed.splitlines() == [
        f"boolean string True object {[marker, marker]}",
        "BackendError the reply was refused: the response carries no answer for question `q1` 2"]


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
    for verb in ("decide", "decide_many"):
        got = getattr(tt.Engine(base_url=base, batch=1, cache=False), verb)(refund, records).value
        print("case 27", verb, got.index.equals(records.index), got.tolist())
    indexes = {"labels": [5, 7, 9], "rows": pd.MultiIndex.from_tuples(
        [("a", 1), ("a", 2), ("b", 1)]), "repeated": [1, 1, 2], "empty": []}
    for kind, index in indexes.items():
        rows = texts[:len(index)]
        series = pd.Series(rows, index=index, name="body", dtype=object)
        frame = series.to_frame()
        joins = kind in ("labels", "rows")
        for verb, asked, _ in VERBS + (("decide_many", late, "boolean"),):
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
    verbs = ("decide", "choose", "score", "tag", "decide_many")
    wanted = [f"case 27 {verb} True [True, False, True, True, False]"
              for verb in ("decide", "decide_many")]
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
    3 reads its own export, and pandas 2 reads a list."""
    printed = run(SETUP + """
    column = pd.Series(texts)
    frame = pd.DataFrame({"body": texts})
    said(lambda: engine.decide(late, pd.Series([1, 2])).value)
    said(lambda: engine.decide(late, pd.Series(["a", 1], dtype=object)).value)
    for hole in (None, float("nan"), pd.NA):
        said(lambda: engine.tag(kinds, pd.Series(["a", hole, "b"], dtype=object)).value)
    said(lambda: engine.decide(late, pd.Series([None, None], dtype=object)).value)
    said(lambda: engine.filter(late, column).value)
    class Mine(pd.Series):
        pass
    said(lambda: engine.filter(late, Mine(texts)).value)
    said(lambda: engine.rank("Late?", column).value)
    said(lambda: engine.find("Late?", column).value)
    said(lambda: engine.relate(column, relations={"r": ("a", "b")}).value)
    said(lambda: engine.annotate(form, column).value)
    said(lambda: engine.recognize(column, kinds=["x"]).value)
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
    said(lambda: engine.decide_many(late, pd.Index(texts)).value)
    said(lambda: engine.annotate(form, pa.table({"body": texts}), on="body").value)
    """, child_env(backend, tmp_path))
    route = ["UsageError the column's Arrow format is 'l', not text 0"] if THREE else [
        "UsageError record 0 is not a str 0"]
    assert printed.splitlines() == route + [
        "UsageError record 1 is not a str 0",
        *3 * [f"UsageError {NULLS} 0"],
        f"UsageError {NULLS} 0",
        *7 * [f"UsageError {LISTS} 0"],
        "UsageError details reads one str, not a column 0",
        "UsageError a data frame is not a column; pass df[\"name\"], or annotate with on= 0",
        f"UsageError {LISTS} 0",
        "UsageError the frame has no column named 'missing' 0",
        "UsageError the frame has more than one column named 'body' 0",
        "UsageError on= reads a frame whose column labels have one level 0",
        "UsageError the frame already has a column named 'late'; rename it first 0",
        "UsageError the frame already has a column named 'failed'; rename it first 0",
        "UsageError the question name failed is reserved for frame failures 0",
        "UsageError recognize with on= takes no relations; ask them of one text 0",
        "UsageError on= takes one column label, such as \"body\" 0",
        "UsageError the frame already has a column named 'names'; rename it first 0",
        "UsageError thinkthen reads a pandas Series, not a pandas Index; pass a pandas Series 0",
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
        answers = engine.decide_many(late, records).value
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
                          f"    engine.decide_many(late, {records}).value\n",
                          child_env(backend, tmp_path, "arm/held"))
            assert backend.wait(1) == 1
            time.sleep(0.3)
            assert backend.count() == 1, records
            backend.release()
            assert child.wait(timeout=10) == 0, child.stderr.read()
        finally:
            backend.close()


@pytest.mark.skipif(not THREE, reason="pandas 2 offers no Arrow export")
def test_pandas_3_text_crosses_in_place(backend, tmp_path):
    """Proof 4: on pandas 3, ``_arrow_probe`` over a ``str`` and a
    ``string[pyarrow]`` Series reads each text where pandas' own export puts
    it. The pandas 2 lane skips it; its precondition shows no export. Regression:
    a copy before the hand-off moves every text."""
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
