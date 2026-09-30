"""Public call accounts at the Python boundary, over the offline listener.

These cases protect the new wrapper, batch controls, and detached receipt.
The old answer tests inspect bare values and cannot see missing accounts or
one request per row. No test-only export or provider call is needed.
"""

import hashlib
import json
import os
import signal
import pathlib
from contextlib import contextmanager
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from threading import Thread

import pytest

from conftest import child_env, run, start


def quoted_record(request):
    """The one record a request quotes at the head of its first question."""
    text = next(iter(request["questions"].values()))["instructions"]
    assert text.startswith("The text is "), text
    return json.JSONDecoder().raw_decode(text, len("The text is "))[0]


def test_keywords_and_probability_share_the_answer(backend, tmp_path):
    """The public call holds probability from its own reply, including a column.
    A second probability request or a missing column row changes the exact count."""
    printed = run("""
    import polars as pl, thinkthen as tt
    engine = tt.Engine(cache=False)
    decided = engine.decide("Is it late?", "one", true="a late item")
    chosen = engine.choose("Which team?", "two", options=["billing", "shipping"])
    scored = engine.score("How urgent?", "three", levels=["Routine.", "Now."])
    tagged = engine.tag("Which kind?", "four", labels=["bill", "ship"])
    column = engine.decide("Is it late?", pl.Series(["five", "six"]))
    print(decided.value, decided.probability, chosen.value, chosen.probability,
          scored.value, scored.probability, tagged.value, tagged.probability,
          column.value.to_list(), column.probability.to_list(),
          engine.usage()["requests_sent"])
    """, child_env(backend, tmp_path))
    assert printed.strip() == ("True 0.9 billing 0.9 0.1 None ['bill', 'ship'] None "
                               "[True, True] [0.9, 0.9] 5")
    assert backend.count() == 5


def test_process_cap_reserves_actual_attempts(backend, tmp_path):
    """The Python constructor forwards the active process cap. A refusal
    after one send cannot be simulated by a plan or by a per-call limit."""
    printed = run("""
    import thinkthen as tt
    low = tt.Engine(cache=False, max_requests_total=1)
    print(low.decide("Is it late?", "one").value)
    try:
        low.decide("Is it late?", "two").value
    except tt.UsageError as error:
        print(error.kind, error.retryable, low.usage()["requests_sent"])
    else:
        raise AssertionError("the second attempt crossed the cap")
    high = tt.Engine(cache=False, max_requests_total=2)
    print(high.decide("Is it late?", "three").value,
          high.usage()["requests_sent"])
    """, child_env(backend, tmp_path))
    assert printed.splitlines() == ["True", "usage False 1", "True 1"]
    assert backend.count() == 2


def test_token_cap_variable_refuses_before_any_send(backend, tmp_path):
    """The engine reads the token cap variable through from_env and sends nothing past it."""
    printed = run("""
    import polars as pl, thinkthen as tt
    engine = tt.Engine(cache=False)
    for text in ("one", pl.Series(["two", "three"])):
        try:
            engine.decide("Is it late?", text)
        except tt.UsageError as error:
            print(error.kind, error)
    """, child_env(backend, tmp_path, THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL="10"))
    refusal = ("usage max_estimated_input_tokens_total=10 (encoded-body-bytes-908-v1) "
               "would be exceeded before this call's first request")
    assert printed.splitlines() == [refusal, refusal]
    assert backend.count() == 0


def test_default_pack_and_explicit_batch_one_keep_their_distinct_bodies(backend, tmp_path):
    """A default call packs three records; batch one sends three scalar
    bodies. The value and probability for each record survive both cuts."""
    packed = (b'{"state":"Each question quotes the text it asks about.",'
              b'"model":"jev-1.13.0","questions":{'
              b'"q1":{"type":"noul","instructions":"The text is \\"one\\". Is it late?"},'
              b'"q2":{"type":"noul","instructions":"The text is \\"two\\". Is it late?"},'
              b'"q3":{"type":"noul","instructions":"The text is \\"three\\". Is it late?"}}}')
    single = [b'{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0",'
              b'"questions":{"q1":{"type":"noul","instructions":"The text is \\"' + word
              + b'\\". Is it late?"}}}'
              for word in (b"one", b"two", b"three")]
    with capturing_filter_listener() as (url, bodies):
        env = child_env(backend, tmp_path)
        env["THINKTHEN_BASE_URL"] = url.removesuffix("/systemone")
        printed = run("""
        import thinkthen as tt
        engine = tt.Engine(cache=False)
        rows = ["one", "two", "three"]
        for setting in ({}, {"batch": 1}):
            call = engine.decide("Is it late?", rows, **setting)
            print(call.value, call.probability, call.facts["requests_sent"])
        """, env)
        assert printed.splitlines() == [
            "[False, True, True] [0.1, 0.9, 0.9] 1",
            "[False, True, True] [0.1, 0.9, 0.9] 3",
        ]
        assert bodies == [packed, *single]
    assert backend.count() == 0


def test_partial_judge_plan_and_execution_share_immutable_question(backend, tmp_path):
    """A partial works, and later mutation of its original options cannot
    make the planned body disagree with the one sent by that Judge."""
    with capturing_filter_listener(lambda question: {
        "type": "choice", "probabilities": {"billing": 0.9, "shipping": 0.1}
    }) as (url, bodies):
        env = child_env(backend, tmp_path)
        env["THINKTHEN_BASE_URL"] = url.removesuffix("/systemone")
        printed = run("""
        import functools, thinkthen as tt
        options = ["billing", "shipping"]
        judge = functools.partial(tt.choose, "Which team?", options=options)()
        options.append("changed")
        plan = tt.plan(judge, ["one", "two"])
        call = judge(["one", "two"])
        print(plan["records"], plan["requests"],
              call.value, call.facts["requests_sent"])
        print(plan["first_body"].decode())
        try: tt.plan(lambda rows: rows, ["one"])
        except tt.UsageError as error: print(error.kind)
        """, env)
        account, planned, refusal = printed.splitlines()
        assert account == "2 1 ['billing', 'billing'] 1"
        assert refusal == "usage"
        assert bodies == [planned.encode()]
        assert b"changed" not in bodies[0]
    assert backend.count() == 0


def test_unresolved_choice_omits_probability_but_banded_decide_keeps_yes_probability(backend, tmp_path):
    """Saved cases 03 and 07 distinguish an unresolved choice from a
    banded decision; both still carry facts from exactly one answer."""
    printed = run("""
    import os, pandas as pd, thinkthen as tt
    base = os.environ["THINKTHEN_BASE_URL"]
    case = lambda name: base.replace("/generic/", f"/case/{name}/")
    band = tt.Engine(base_url=case("03-decide-band-unsure"), cache=False).decide(
        "Does this need attention?", "A short note.", threshold="0.2:0.8")
    choice = tt.Engine(base_url=case("07-choose-unsure"), cache=False).choose(
        "Which team owns this?", "Route this note.",
        options=["billing", "shipping", "other"], threshold=0.6)
    column = tt.Engine(cache=False).choose(
        "Which team?", pd.Series(["one", "two"], index=[7, 5], name="body"),
        options=["billing", "shipping"], threshold=1.0)
    print(band.value, band.probability, band.facts["requests_sent"],
          choice.value, choice.probability, choice.facts["requests_sent"],
          column.value.isna().tolist(), column.probability.isna().tolist(),
          column.probability.index.tolist(), column.probability.dtype.name,
          column.facts["requests_sent"])
    """, child_env(backend, tmp_path))
    assert printed.strip() == ("None 0.2 1 None None 1 [True, True] [True, True] "
                               "[7, 5] Float64 1")
    assert backend.count() == 3


def test_null_column_rows_keep_positions_without_requests(backend, tmp_path):
    """A missing input skips the wire, while the two real texts keep their
    own answers and probabilities. The old guards refused the whole column."""
    def chosen(question):
        if question["type"] == "choice":
            return {"type": "choice", "probabilities": {"billing": 0.9, "shipping": 0.1}}
        return None

    with capturing_filter_listener(chosen) as (url, bodies):
        env = child_env(backend, tmp_path)
        env["THINKTHEN_BASE_URL"] = url.removesuffix("/systemone")
        printed = run("""
        import pandas as pd, polars as pl, thinkthen as tt
        engine = tt.Engine(cache=False, batch=1, throttle=1)
        pandas = pd.Series(["one", pd.NA, "two"], dtype="string",
                           index=[9, 5, 7], name="body")
        decided = engine.decide("Is it late?", pandas)
        polars = pl.Series("body", ["one", None, "two"], dtype=pl.String)
        chosen = engine.choose("Which team?", polars, options=["billing", "shipping"])
        empty = engine.decide("Is it late?", pd.Series([pd.NA, pd.NA], dtype="string",
                                                     index=[3, 1], name="body"))
        fallback = engine.decide("Is it late?", pd.Series(["one", None, "two"],
                                                       dtype="category", index=[4, 2, 8]))
        empty_polars = engine.choose("Which team?", pl.Series([None, None], dtype=pl.String),
                                     options=["billing", "shipping"])
        empty_object = engine.decide("Is it late?", pd.Series([None, None], dtype=object))
        cut = pl.concat([pl.Series("body", ["skip", None], dtype=pl.String),
                         pl.Series("body", ["one", "two"], dtype=pl.String)],
                        rechunk=False).slice(1, 3)
        shifted = engine.decide("Is it late?", cut)
        uncertain = engine.choose("Which team?", pl.Series(["one", None], dtype=pl.String),
                                  options=["billing", "shipping"], threshold=1.0)
        print(decided.value.tolist(), decided.probability.tolist(),
              decided.value.index.tolist(), [row["index"] for row in decided.details],
              decided.facts["records"], decided.facts["requests_sent"])
        print(chosen.value.to_list(), chosen.probability.to_list(),
              [row["index"] for row in chosen.details],
              chosen.facts["records"], chosen.facts["requests_sent"])
        print(empty.value.tolist(), empty.probability.tolist(),
              empty.value.index.tolist(), empty.facts["records"], empty.facts["requests_sent"])
        print(fallback.value.tolist(), fallback.probability.tolist(),
              fallback.value.index.tolist(), fallback.facts["requests_sent"])
        print(empty_polars.value.to_list(), empty_polars.probability.to_list(),
              empty_polars.facts["records"], empty_polars.facts["requests_sent"],
              empty_object.value.isna().tolist(), empty_object.facts["requests_sent"])
        print(shifted.value.to_list(), shifted.probability.to_list(),
              [row["index"] for row in shifted.details], shifted.facts["requests_sent"])
        print(uncertain.value.to_list(), uncertain.probability.to_list(),
              [row["index"] for row in uncertain.details], uncertain.facts["requests_sent"])
        """, env)
        assert printed.splitlines() == [
            "[False, <NA>, True] [0.1, <NA>, 0.9] [9, 5, 7] [0, 2] 2 2",
            "['billing', None, 'billing'] [0.9, None, 0.9] [0, 2] 2 2",
            "[<NA>, <NA>] [<NA>, <NA>] [3, 1] 0 0",
            "[False, <NA>, True] [0.1, <NA>, 0.9] [4, 2, 8] 2",
            "[None, None] [None, None] 0 0 [True, True] 0",
            "[None, False, True] [None, 0.1, 0.9] [1, 2] 2",
            "[None, None] [None, None] [0] 1",
        ]
        requests = [json.loads(body) for body in bodies]
        assert [(quoted_record(one), next(iter(one["questions"].values()))["type"])
                for one in requests] == [("one", "noul"), ("two", "noul"),
                                         ("one", "choice"), ("two", "choice"),
                                         ("one", "noul"), ("two", "noul"),
                                         ("one", "noul"), ("two", "noul"),
                                         ("one", "choice")]
        assert len(bodies) == 9
    assert backend.count() == 0


def test_null_frame_rows_keep_columns_and_skip_recognition(backend, tmp_path):
    """Both frame doors retain source columns, return null answer cells,
    and keep recognition row numbers after a missing input."""
    printed = run("""
    import pandas as pd, polars as pl, thinkthen as tt
    engine = tt.Engine(cache=False)
    texts = ["my card was charged twice", None, "the box came late"]
    form = {"version": 1, "questions": {"late": {"decide": "Is it late?"},
            "team": {"choose": "Which team?", "options": ["billing", "shipping"]}}}
    polars = pl.DataFrame({"body": pl.Series(texts, dtype=pl.String), "keep": [9, 5, 7]})
    asked = engine.annotate(form, polars, on="body")
    found = engine.recognize(polars, kinds=["bill", "ship"], on="body")
    blank = pl.DataFrame({"body": pl.Series([None, None], dtype=pl.String)})
    blank_asked = engine.annotate(form, blank, on="body")
    blank_found = engine.recognize(blank, kinds=["bill", "ship"], on="body")
    print("polars", asked.value.select(polars.columns).equals(polars),
          asked.value["late"].to_list(), asked.value["team"].to_list(),
          asked.value["failed"].to_list(),
          sorted({row["index"] for row in asked.details}), asked.facts["records"],
          sorted(set(found.value["row"].to_list())), found.facts["records"])
    print("blank", blank_asked.value["late"].to_list(),
          blank_asked.value["failed"].to_list(), blank_asked.facts["records"],
          blank_asked.facts["requests_sent"], blank_found.value.height,
          blank_found.facts["records"], blank_found.facts["requests_sent"])
    index = [9, 5, 7]
    pandas = pd.DataFrame({"body": pd.Series(texts, index=index, dtype="string"),
                           "keep": [9, 5, 7]}, index=index)
    asked = engine.annotate(form, pandas, on="body")
    found = engine.recognize(pandas, kinds=["bill", "ship"], on="body")
    print("pandas", asked.value[pandas.columns].equals(pandas),
          asked.value["late"].astype(object).where(asked.value["late"].notna(), None).tolist(),
          asked.value["team"].astype(object).where(asked.value["team"].notna(), None).tolist(),
          asked.value["failed"].tolist(), asked.value.index.tolist(),
          sorted({row["index"] for row in asked.details}), asked.facts["records"],
          [row is None for row in found.value["names"]], found.facts["records"])
    print("sends", sum(call.facts["requests_sent"] for call in (asked, found)))
    """, child_env(backend, tmp_path))
    assert printed.splitlines()[:2] == [
        "polars True [True, None, True] ['billing', None, 'billing'] "
        "[None, None, None] [0, 2] 2 [1, 3] 2",
        "blank [None, None] [None, None] 0 0 0 0 0",
    ]
    assert printed.splitlines()[2:3] == [
        "pandas True [True, None, True] ['billing', None, 'billing'] "
        "[None, None, None] [9, 5, 7] [0, 2] 2 [False, True, False] 2",
    ]
    assert backend.count() > 0


def test_null_score_and_tag_columns_keep_their_existing_shapes(backend, tmp_path):
    """The shared nullable input reader also serves score and tag, whose
    Call probability stays absent and whose non-null values keep their type."""
    printed = run("""
    import pandas as pd, polars as pl, thinkthen as tt
    engine = tt.Engine(cache=False)
    rows = pl.Series("body", ["one", None, "two"], dtype=pl.String)
    score = engine.score("How urgent?", rows, levels=["Routine.", "Soon.", "Now."])
    tags = engine.tag("Which kinds?", rows, labels=["bill", "ship"])
    object_tags = engine.tag("Which kinds?", pd.Series(["one", float("nan"), "two"],
                                                         dtype=object, index=[9, 5, 7]),
                             labels=["bill", "ship"])
    print(score.value.to_list(), score.probability, score.facts["records"],
          score.facts["requests_sent"], [row["index"] for row in score.details])
    print(tags.value.to_list(), tags.probability, tags.facts["records"],
          tags.facts["requests_sent"], [row["index"] for row in tags.details])
    print(object_tags.value.tolist(), object_tags.value.index.tolist(),
          object_tags.probability, object_tags.facts["records"],
          object_tags.facts["requests_sent"], [row["index"] for row in object_tags.details])
    """, child_env(backend, tmp_path))
    assert printed.splitlines() == [
        "[0.15, None, 0.15] None 2 1 [0, 2]",
        "[['bill', 'ship'], None, ['bill', 'ship']] None 2 1 [0, 2]",
        "[['bill', 'ship'], None, ['bill', 'ship']] [9, 5, 7] None 2 1 [0, 2]",
    ]
    assert backend.count() == 3


@pytest.mark.parametrize("shape", ["list", "polars_series"])
def test_portable_max_content_cuts_in_public_bulk_text_shapes(backend, tmp_path, shape):
    fixture = pathlib.Path(__file__).resolve().parents[3] / "specification/fixtures/batching"
    corpus = json.loads((fixture / "portable-records.json").read_text())
    expected = [(fixture / f"portable-{index}.request.json").read_bytes().removesuffix(b"\n")
                for index in (1, 2, 3)]
    with capturing_filter_listener() as (url, bodies):
        env = child_env(backend, tmp_path)
        env["THINKTHEN_BASE_URL"] = url.removesuffix("/systemone")
        printed = run(f"""
            import json, polars as pl, thinkthen as tt
            rows = {corpus['texts']!r}
            if {shape!r} == 'polars_series':
                rows = pl.Series('body', rows)
            call = tt.Engine(cache=False, throttle=1).decide({corpus['question']!r}, rows)
            value = call.value.to_list() if hasattr(call.value, 'to_list') else call.value
            print(json.dumps([value, [call.facts["records"], call.facts["requests_sent"]],
                              [[row['index'], list(row['requests'])] for row in call.details]]))
        """, env)
        values, facts, details = json.loads(printed)
        assert bodies == expected
        assert values == [True] * 5
        assert facts == [5, 3]
        hashes = [hashlib.sha256(b"systemone\n" + url.encode() + b"\n" + body).hexdigest()
                  for body in expected]
        assert details == [[i, [hashes[group]]] for i, group in enumerate((0, 0, 1, 1, 2))]
    assert backend.count() == 0


@contextmanager
def capturing_filter_listener(answer=None, usage=True):
    """Keep wire bodies; a callback's None retains the normal decision answer."""
    bodies = []

    class Handler(BaseHTTPRequestHandler):
        protocol_version = "HTTP/1.1"

        def do_POST(self):
            body = self.rfile.read(int(self.headers["Content-Length"]))
            bodies.append(body)
            request = json.loads(body)
            answers = {}
            for name, question in request["questions"].items():
                chosen = answer(question) if answer is not None else None
                if chosen is not None:
                    answers[name] = chosen
                    continue
                first = request["state"] == "one" or 'The text is "one"' in question["instructions"]
                answers[name] = {"type": "noul", "noul": 0.1 if first else 0.9}
            reply_fields = {"model": "jev-latest", "answers": answers}
            if usage:
                reply_fields["usage"] = {"input_tokens": 6, "output_tokens": 3}
            reply = json.dumps(reply_fields,
                               separators=(",", ":")).encode()
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(reply)))
            self.end_headers()
            self.wfile.write(reply)

        def log_message(self, *_):
            pass

    listener = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    thread = Thread(target=listener.serve_forever, daemon=True)
    thread.start()
    try:
        yield f"http://127.0.0.1:{listener.server_port}/v1/systemone", bodies
    finally:
        listener.shutdown()
        listener.server_close()
        thread.join(timeout=5)


def test_judge_eager_and_stream_share_exact_distinct_record_bodies(backend, tmp_path):
    """Equivalent content cuts have identical wire bytes; a stream's
    probability comes from that reply and adds no second operation."""
    expected = (b'{"state":"Each question quotes the text it asks about.",'
                b'"model":"jev-1.13.0","questions":{'
                b'"q1":{"type":"noul","instructions":"The text is \\"one\\". Is it late?"},'
                b'"q2":{"type":"noul","instructions":"The text is \\"two\\". Is it late?"},'
                b'"q3":{"type":"noul","instructions":"The text is \\"three\\". Is it late?"}}}')
    with capturing_filter_listener() as (url, bodies):
        env = child_env(backend, tmp_path)
        env["THINKTHEN_BASE_URL"] = url.removesuffix("/systemone")
        printed = run("""
        import thinkthen as tt
        engine = tt.Engine(cache=False)
        rows = ["one", "two", "three"]
        eager = engine.decide("Is it late?", rows)
        lazy = engine.decide("Is it late?", probability=True)(iter(rows))
        print(eager.value, eager.probability, eager.facts["requests_sent"])
        print(list(lazy), lazy.facts["records"], lazy.facts["requests_sent"])
        """, env)
        assert printed.splitlines() == [
            "[False, True, True] [0.1, 0.9, 0.9] 1",
            "[(False, 0.1), (True, 0.9), (True, 0.9)] 3 1",
        ]
        assert bodies == [expected, expected]
    assert backend.count() == 0


def test_explicit_tally_counts_completed_calls_cache_and_missing_usage(backend, tmp_path):
    """Two simultaneously started calls, one identical cached replay, and a
    no-usage reply use the core tally without a mutable last-call slot."""
    with capturing_filter_listener() as (url, first), \
         capturing_filter_listener(usage=False) as (other, second):
        env = child_env(backend, tmp_path, OWNED_CACHE=str(tmp_path / "owned-cache"),
                        OTHER_URL=other.removesuffix("/systemone"))
        env["THINKTHEN_BASE_URL"] = url.removesuffix("/systemone")
        printed = run("""
        import os, threading, concurrent.futures, thinkthen as tt
        tally = tt.Tally()
        engine = tt.Engine(cache=os.environ["OWNED_CACHE"])
        judge = engine.decide("Is it late?", tally=tally)
        gate = threading.Barrier(2)
        def ask(text):
            gate.wait(timeout=5)
            return judge(text).value
        with concurrent.futures.ThreadPoolExecutor(max_workers=2) as pool:
            done = list(pool.map(ask, ["one", "two"]))
        print(done, tally.facts["records"], tally.facts["requests_sent"],
              tally.facts["cache_answers"], tally.facts.get("input_tokens") is not None,
              tally.facts["seconds"] >= 0)
        print(judge("one").value, tally.facts["records"], tally.facts["requests_sent"],
              tally.facts["cache_answers"])
        no_usage = tt.Engine(base_url=os.environ["OTHER_URL"], cache=False).decide(
            "Is it late?", tally=tally)
        print(no_usage("three").value, tally.facts["records"], tally.facts["requests_sent"],
              tally.facts["cache_answers"], tally.facts.get("input_tokens"),
              tally.facts.get("output_tokens"), tally.facts.get("model"))
        """, env)
        assert printed.splitlines() == [
            "[False, True] 2 2 0 True True",
            "False 3 2 1",
            "True 4 3 1 None None jev-latest",
        ]
        assert len(first) == 2
        assert len(second) == 1
    assert backend.count() == 0


def test_filter_stream_selective_take_stops_before_remaining_input(backend, tmp_path):
    """A finite take consumes the required false row and only three later
    matches; closing bills started requests and never reads the tail."""
    with capturing_filter_listener() as (url, bodies):
        env = child_env(backend, tmp_path)
        env["THINKTHEN_BASE_URL"] = url.removesuffix("/systemone")
        printed = run("""
        import itertools, thinkthen as tt
        read = []
        def source():
            for text in ["one", "two", "three", "four", "five", "six"]:
                read.append(text)
                yield text
        stream = tt.filter("Is it late?", batch=1)(source())
        with stream:
            print(list(itertools.islice(stream, 3)))
        print(read, stream.facts["records"], stream.facts["requests_sent"])
        """, env)
        assert printed.splitlines() == [
            "['two', 'three', 'four']",
            "['one', 'two', 'three', 'four'] 4 4",
        ]
        assert len(bodies) == 4
    assert backend.count() == 0


def test_batches_labels_and_owned_details(backend, tmp_path):
    printed = run("""
    import thinkthen as tt
    engine = tt.Engine(cache=False)
    decide = tt.question(decide="Is it late?")
    rows = ["one", "two", "three"]
    packed = engine.decide(decide, rows)
    separate = engine.decide(decide, rows, batch=1)
    assert packed.value == separate.value == [True, True, True]
    assert (packed.facts["records"], packed.facts["requests_sent"]) == (3, 1)
    assert (separate.facts["records"], separate.facts["requests_sent"]) == (3, 3)
    assert [entry["index"] for entry in packed.details] == [0, 1, 2]
    assert sum(entry["requests_sent"] for entry in packed.details) == 1
    assert packed.details[0]["requests"]
    try:
        packed.details[0]["index"] = 9
    except TypeError:
        pass
    else:
        raise AssertionError("an observation was mutable")
    choice = engine.choose("Which team?", rows, options=["billing", "shipping"])
    score = tt.score("How urgent?", rows, levels=["Routine.", "Soon.", "Now."])
    tags = engine.tag("Which kinds?", rows, labels=["bill", "ship"])
    assert choice.value == ["billing"] * 3
    assert score.value == [0.15] * 3
    assert tags.value == [["bill", "ship"]] * 3
    assert all(item.facts["records"] == 3 and len(item.details) == 3
               for item in (choice, score, tags))
    before = engine.usage()["requests_sent"]
    for call in (lambda: engine.decide(decide, rows, batch=0),
                 lambda: engine.decide(decide, rows, context="  "),
                 lambda: engine.annotate({"version": 1, "questions": {"late": {"decide": "Late?"}}},
                                         rows, context="reference")):
        try:
            call()
        except tt.UsageError:
            pass
        else:
            raise AssertionError("unsupported control was accepted")
    assert engine.usage()["requests_sent"] == before
    shared = engine.decide(decide, rows, batch=1, context="reference")
    assert shared.value == packed.value
    assert shared.details[0]["requests"] != separate.details[0]["requests"]
    print("calls", packed.facts["requests_sent"], separate.facts["requests_sent"],
          choice.facts["requests_sent"], score.facts["requests_sent"], tags.facts["requests_sent"])
    """, child_env(backend, tmp_path))
    assert printed.strip() == "calls 1 3 1 1 1"
    assert backend.count() == 10

    with capturing_filter_listener() as (url, bodies):
        env = child_env(backend, tmp_path)
        env["THINKTHEN_BASE_URL"] = url.removesuffix("/systemone")
        printed = run("""
        import json, thinkthen as tt
        engine = tt.Engine(model="jev-latest", throttle=1, cache=False)
        question = tt.question(decide="Is it late?")
        rows = ["one", "two", "one"]
        calls = [engine.filter(question, rows), engine.filter(question, rows, batch=1)]
        for call in calls:
            print(json.dumps({"value": call.value,
                              "url": call.details[0]["url"],
                              "facts": [call.facts["records"], call.facts["requests_sent"],
                                        call.facts.get("input_tokens"), call.facts.get("output_tokens")],
                              "details": [[row["index"], row["answer"],
                                           list(row["requests"]),
                                           dict(row["usage"])] for row in call.details]}))
        """, env)
        packed, separate = map(json.loads, printed.splitlines())
        assert [packed["value"], separate["value"]] == [["two"]] * 2
        assert [packed["facts"], separate["facts"]] == [[3, 1, 6, 3], [3, 3, 18, 9]]
        assert packed["url"] == url, (packed["url"], url)
        assert len(bodies) == 4
        one = (b'{"state":"Each question quotes the text it asks about.","model":"jev-latest",'
               b'"questions":{"q1":{"type":"noul","instructions":"The text is \\"one\\". Is it late?"}}}')
        two = (b'{"state":"Each question quotes the text it asks about.","model":"jev-latest",'
               b'"questions":{"q1":{"type":"noul","instructions":"The text is \\"two\\". Is it late?"}}}')
        assert bodies[1:] == [one, two, one]
        digest = lambda body: hashlib.sha256(b"systemone\n" + url.encode() + b"\n" + body).hexdigest()
        assert [row[:3] for row in packed["details"]] == [
            [index, index == 1, [digest(bodies[0])]] for index in range(3)]
        assert [row[:3] for row in separate["details"]] == [
            [index, index == 1, [digest(body)]] for index, body in enumerate(bodies[1:])]
        assert separate["details"][0][2] == separate["details"][2][2]
        for call in (packed, separate):
            assert sum(row[3]["input_tokens"] for row in call["details"]) == call["facts"][2]
            assert sum(row[3]["output_tokens"] for row in call["details"]) == call["facts"][3]


def test_returned_failure_keeps_its_final_account(backend, tmp_path):
    printed = run("""
    import thinkthen as tt
    try:
        tt.Engine(cache=False).decide(tt.question(decide="Q?"), ["a", "b"], batch=1)
    except tt.BackendError as error:
        assert (error.facts["records"], error.facts["requests_sent"]) == (0, 1)
        assert error.details == ()
        print("failed", error.kind, error.facts["requests_sent"])
    else:
        raise AssertionError("the malformed reply passed")
    """, child_env(backend, tmp_path, "arm/malformed/missing_answer"))
    assert printed.strip() == "failed backend 1"
    assert backend.count() == 1


def test_columns_and_frames_rebuild_inside_call_value(backend, tmp_path):
    """The old frame tests inspect schema, but cannot catch dropped call facts."""
    printed = run("""
    import pandas as pd, polars as pl, thinkthen as tt
    engine = tt.Engine(cache=False)
    question = tt.question(decide="Is it late?")
    source = pd.Series(["one", "two"], index=[4, 8], name="body")
    column = engine.decide(question, source)
    assert column.value.to_list() == [True, True]
    assert column.value.index.to_list() == [4, 8]
    assert column.value.name == "body"
    assert (column.facts["records"], len(column.details)) == (2, 2)
    form = {"version": 1, "questions": {"late": {"decide": "Late?"}}}
    frame = engine.annotate(form, pl.DataFrame({"body": ["one", "two"]}), on="body")
    assert frame.value.columns == ["body", "late", "failed"]
    assert frame.value["failed"].is_null().all()
    assert (frame.facts["records"], len(frame.details)) == (2, 2)
    named = engine.recognize(pl.DataFrame({"body": ["one", "two"]}),
                             kinds=["PERSON"], on="body")
    assert named.value.columns == ["row", "text", "start", "end", "length",
                                   "kind", "strength"]
    assert named.facts["records"] == 2
    assert [entry["index"] for entry in named.details] == [0, 0, 1, 1]
    print("frames", column.facts["records"], frame.facts["records"], named.facts["records"])
    """, child_env(backend, tmp_path))
    assert printed.strip() == "frames 2 2 2"


def test_held_stop_has_a_retryable_final_receipt(backend, tmp_path):
    child = start("""
    import sys, threading, thinkthen as tt
    engine = tt.Engine(cache=False)
    token = tt.CancelToken()
    def stop():
        sys.stdin.readline()
        token.cancel()
    threading.Thread(target=stop, daemon=True).start()
    try:
        engine.decide(tt.question(decide="Q?"), "one", token=token)
    except tt.Cancelled as error:
        receipt = error.completion
        assert receipt.done is False
        try:
            receipt.result(timeout=0)
        except TimeoutError:
            print("pending", flush=True)
        else:
            raise AssertionError("held worker was final")
        sys.stdin.readline()
        final = receipt.result(timeout=3)
        assert final.outcome == "failed"
        assert (final.facts["records"], final.facts["requests_sent"]) == (0, 1)
        assert receipt.done and final.details is not None
        assert (final.kind, final.message, final.retryable) == (
            "cancelled", "the call was cancelled", False)
        print("final", final.facts["requests_sent"], flush=True)
    """, child_env(backend, tmp_path, "arm/held"))
    assert backend.wait(1) == 1
    child.stdin.write("stop\n")
    child.stdin.flush()
    assert child.stdout.readline().strip() == "pending"
    assert backend.count() == 1
    backend.release()
    child.stdin.write("released\n")
    child.stdin.flush()
    assert child.stdout.readline().strip() == "final 1"
    assert child.wait(timeout=10) == 0, child.stderr.read()
    assert backend.count() == 1


def test_system_exit_retains_type_code_and_completion(backend, tmp_path):
    child = start("""
    import signal, sys, thinkthen as tt
    signal.signal(signal.SIGINT, lambda *_: sys.exit(23))
    try:
        tt.Engine(cache=False).decide(tt.question(decide="Q?"), "one")
    except SystemExit as error:
        assert type(error) is SystemExit and error.code == 23
        receipt = error.completion
        print("exit", error.code, receipt.done, flush=True)
        sys.stdin.readline()
        final = receipt.result(timeout=3)
        print("final", final.outcome, final.facts["requests_sent"], flush=True)
    """, child_env(backend, tmp_path, "arm/held"))
    assert backend.wait(1) == 1
    os.kill(child.pid, signal.SIGINT)
    assert child.stdout.readline().strip() == "exit 23 False"
    backend.release()
    child.stdin.write("released\n")
    child.stdin.flush()
    assert child.stdout.readline().strip() == "final failed 1"
    assert child.wait(timeout=10) == 0, child.stderr.read()
    assert backend.count() == 1


def test_host_reconstruction_error_keeps_frozen_call(backend, tmp_path):
    printed = run("""
    import pandas as pd, thinkthen as tt
    class Broken(pd.Series):
        def __init__(self, *args, **kwargs):
            if kwargs.get("dtype") == "boolean":
                raise RuntimeError("cannot rebuild")
            super().__init__(*args, **kwargs)
    source = Broken(["one", "two"], name="body")
    try:
        tt.Engine(cache=False).decide(tt.question(decide="Q?"), source)
    except tt.LocalError as error:
        assert isinstance(error.__cause__, RuntimeError)
        assert (error.facts["records"], len(error.details)) == (2, 2)
        print("local", error.facts["records"], str(error.__cause__))
    else:
        raise AssertionError("the host rebuilt")
    """, child_env(backend, tmp_path))
    assert printed.strip() == "local 2 cannot rebuild"
    assert backend.count() == 1
