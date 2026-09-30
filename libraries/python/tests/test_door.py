"""The Polars door: a column or a frame in, one engine call, answers out.

Each engine call runs in a child on its own loopback backend. Parity tests
compare the column form with the list form over the same texts, so the
backend's answers need not be known here.
"""

import subprocess
import sys

from conftest import FAKE, child_env, clean_env, run

LISTS = ("filter, rank, find, and relate read a list of str, not a column, and annotate and "
         "recognize read a column only from a Polars or pandas frame with on=. "
         "Pass column.to_list()")
FRAMES = "UsageError annotate with on= takes a Polars or pandas DataFrame; a list of str takes no on="
SETUP = """
    import json, os, polars as pl, pyarrow as pa, thinkthen as tt
    engine = tt.Engine(cache=False)
    late = tt.question(decide="Is it late?")
    team = tt.question(choose="Which team?", options=["billing", "shipping"])
    urgent = tt.question(score="How urgent?", levels=["Routine.", "Soon.", "Now."])
    kinds = tt.question(tag="Which kinds?", labels=["bill", "ship"])
    form = {"version": 1, "questions": {"late": {"decide": "Late?"},
            "team": {"choose": "Which team?", "options": ["billing", "shipping"]}}}
    texts = ["my card was charged twice", "the box came late", "thanks, all good"]
    def said(call):
        try:
            call()
            print("answered")
        except tt.ThinkThenError as error:
            print(type(error).__name__, error)
"""


def test_a_column_answers_as_its_list_does(backend, tmp_path):
    """Decisions 2 and 3: each verb over a Polars ``Series`` returns a
    ``Series`` equal to the list form, and over a pyarrow column a list.
    Regression: a column verb that answers per row, drops a row, or reads
    one chunk for every chunk."""
    printed = run(SETUP + """
    series = pl.Series("body", texts)
    for verb, asked in (("decide", late), ("choose", team), ("score", urgent), ("tag", kinds)):
        listed = [getattr(engine, verb)(asked, text).value for text in texts]
        column = getattr(engine, verb)(asked, series).value
        print(verb, type(column).__name__, column.to_list() == listed,
              getattr(engine, verb)(asked, pa.chunked_array([texts[:1], texts[1:]])).value == listed)
    many = engine.decide_many(late, series, batch=1).value
    print("decide_many", many.to_list() == engine.decide_many(late, texts, batch=1).value)
    before = engine.usage()["requests_sent"]
    engine.decide_many(late, pa.chunked_array([texts[:1], texts[1:]]), batch=1).value
    print("sent", engine.usage()["requests_sent"] - before)
    """, child_env(backend, tmp_path))
    # The backend answers every text alike. The send count shows every
    # chunk's rows were read (R1-3): chunk 0 read twice sends 2.
    assert printed.splitlines() == [f"{verb} Series True True" for verb in
                                    ("decide", "choose", "score", "tag")] + [
                                    "decide_many True", "sent 3"]


def test_what_the_door_refuses_sends_nothing(backend, tmp_path):
    """Decisions 4, 5, and 6: frames that are neither Polars nor pandas and
    list-only verbs and a number column are refused.
    Ticket 0136: a question named as a column, after a missing ``on``, and a
    column nested past 64 levels are refused too. Nothing reaches the backend."""
    printed = run(SETUP + """
    frame = pl.DataFrame({"body": texts})
    said(lambda: engine.annotate(form, pa.table({"body": texts}), on="body").value)
    said(lambda: engine.annotate(form, texts, on="body").value)
    said(lambda: engine.filter(late, pl.Series(texts)).value)
    said(lambda: engine.relate(pl.Series(texts), relations={"r": ("a", "b")}).value)
    said(lambda: engine.details(late, pl.Series(texts)).value)
    try:
        engine.decide(late, pl.Series([1, 2])).value
    except tt.UsageError as error:
        assert (error.facts.records, error.facts.requests_sent) == (0, 0)
        assert error.details == ()
        print(type(error).__name__, error)
    else:
        raise AssertionError("a number column was accepted")
    said(lambda: engine.decide(late, frame).value)
    said(lambda: engine.annotate(form, frame, on="missing").value)
    clash = pl.DataFrame({"body": texts, "late": texts})
    said(lambda: engine.annotate(form, clash, on="body").value)
    said(lambda: engine.annotate(form, frame.with_columns(pl.lit(1).alias("failed")), on="body").value)
    said(lambda: engine.annotate({"version": 1, "questions": {"failed": {"decide": "Late?"}}}, frame, on="body").value)
    said(lambda: engine.annotate(form, clash, on="missing").value)
    deep = pl.Series("deep", [1])
    for _ in range(70):
        deep = deep.implode()
    said(lambda: engine.annotate(form, pl.DataFrame({"body": texts[:1], "deep": deep}), on="body").value)
    said(lambda: engine.recognize(frame, kinds=["x"], relations={"r": ("x", "x")}, on="body").value)
    """, child_env(backend, tmp_path))
    assert printed.splitlines() == [
        FRAMES,
        FRAMES,
        f"UsageError {LISTS}",
        f"UsageError {LISTS}",
        "UsageError details reads one str, not a column",
        "UsageError the column's Arrow format is 'l', not text",
        "UsageError a data frame is not a column; pass df[\"name\"], or annotate with on=",
        "UsageError the frame has no column named 'missing'",
        "UsageError the frame already has a column named 'late'; rename it first",
        "UsageError the frame already has a column named 'failed'; rename it first",
        "UsageError the question name failed is reserved for frame failures",
        "UsageError the frame has no column named 'missing'",
        "UsageError a column's schema nests deeper than 64 levels",
        "UsageError recognize with on= takes no relations; ask them of one text",
    ]
    assert backend.count() == 0


def test_annotate_on_a_frame_keeps_every_row_and_column(backend, tmp_path):
    """R1-3, R1-4, R2-12, and R4-15 (b): three chunks, a slice at a
    nonzero offset, and Categorical, Enum, struct, and list columns come
    back equal, and each row's own text is sent. Freed memory reads as 0xa5
    under ``MALLOC_PERTURB_``, so a stale chunk shows. Polars may hand the
    three chunks over as fewer batches; the Rust unit test
    ``each_batch_gets_its_own_rows_of_a_new_column`` pins each batch's cut."""
    printed = run(SETUP + """
    kinds = pl.Enum(["bill", "ship", "other"])
    def frame(rows):
        return pl.DataFrame({"body": rows, "cat": pl.Series(rows, dtype=pl.Categorical),
                             "kind": pl.Series(["bill"] * len(rows), dtype=kinds),
                             "pair": [{"a": n, "b": str(n)} for n in range(len(rows))],
                             "list": [[n, n] for n in range(len(rows))]})
    rows = [f"{text} {n}" for n in range(4) for text in texts]
    chunked = pl.concat([frame(rows[:4]), frame(rows[4:9]), frame(rows[9:])], rechunk=False)
    whole = frame(["the same", "the same"] + rows[:5])
    print(chunked.n_chunks())
    cached = tt.Engine(batch=1, cache=os.environ["THINKTHEN_CACHE"])
    wanted = cached.annotate(form, rows).value
    for given, expected in ((chunked, wanted), (whole.slice(2, 5), wanted[:5])):
        before = cached.usage()["requests_sent"]
        got = cached.annotate(form, given, on="body").value
        print(got.columns, got.schema == {**given.schema, "late": pl.Boolean, "team": pl.String,
                                          "failed": got.schema["failed"]},
              got.schema["failed"].base_type() == pl.Struct, got["failed"].is_null().all(),
              got.select(given.columns).equals(given),
              got.select("late", "team").to_dicts() == expected,
              cached.usage()["requests_sent"] - before)
    """, child_env(backend, tmp_path, MALLOC_PERTURB_="165"))
    columns = "['body', 'cat', 'kind', 'pair', 'list', 'late', 'team', 'failed']"
    # The backend answers every text alike. The list call cached every row's
    # text, so a frame call that reads its own rows sends nothing.
    assert printed.splitlines() == ["3"] + 2 * [f"{columns} True True True True True 0"]


def test_a_frame_keeps_types_and_nested_failures():
    """The two-row listener mixes answered and failed score cells. The score
    stays Float64, a successful empty tag stays a list, and failed tags are null."""
    printed = run("""
    import http.server, json, threading, polars as pl, thinkthen as tt
    seen = []
    class Listener(http.server.BaseHTTPRequestHandler):
        def do_POST(self):
            asked = json.loads(self.rfile.read(int(self.headers["Content-Length"])))
            seen.append(asked["state"])
            level = {"a": {"0": 0.25, "1": 0.5, "2": 0.25}, "b": {"1": 0.5, "2": 0.5}}
            def reply(one):
                if one["type"] == "score":
                    return {"type": "score", "probabilities": level[asked["state"]]}
                if asked["state"] == "b" and '"bill"' in one["instructions"]:
                    return {"type": "noul"}
                return {"type": "noul", "noul": 0.1}
            answers = {name: reply(one) for name, one in asked["questions"].items()}
            body = json.dumps({"model": asked["model"], "answers": answers}).encode()
            self.send_response(200)
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)
        def log_message(self, *_):
            pass
    server = http.server.HTTPServer(("127.0.0.1", 0), Listener)
    threading.Thread(target=server.serve_forever, daemon=True).start()
    engine = tt.Engine(base_url=f"http://127.0.0.1:{server.server_port}/v1", cache=False, batch=1)
    form = {"version": 1, "questions": {"late": {"decide": "Late?"},
            "urgent": {"score": "How urgent?", "levels": ["Routine.", "Soon.", "Now."]},
            "kinds": {"tag": "Which kinds?", "labels": ["bill", "ship"]}}}
    got = engine.annotate(form, pl.DataFrame({"body": ["a", "b"]}), on="body").value
    print(got.schema["urgent"] == pl.Float64, got.schema["kinds"] == pl.List(pl.String),
          got.schema["failed"].base_type() == pl.Struct, got["urgent"].to_list(),
          got["kinds"].to_list(), got["failed"].to_list(), len(seen), sep="\\n")
    empty = engine.annotate(form, pl.DataFrame({"body": []}, schema={"body": pl.String}), on="body").value
    print(empty.schema == got.schema, empty.height, len(seen))
    """, clean_env(THINKTHEN_API_KEY=FAKE))
    assert printed.splitlines() == [
        "True", "True", "True", "[1.0, None]", "[[], None]",
        "[None, {'late': None, 'urgent': {'failed': {'kind': 'backend', "
        "'cause': 'missing_probability'}}, 'kinds': {'failed': {'kind': 'backend', "
        "'cause': 'missing_probability'}}}]", "2", "True 0 2"]


def test_recognize_on_a_frame_equals_each_text_alone(backend, tmp_path):
    """Decision 4: one row per name, with its source row counted from 1,
    equal to ``recognize`` of each text; its span rows feed ``relate``."""
    printed = run(SETUP + """
    frame = pl.DataFrame({"body": texts})
    got = engine.recognize(frame, kinds=["bill", "ship"], on="body").value
    alone = [(row, one.text, one.start, one.end, one.length, one.kind, one.strength)
             for row, text in enumerate(texts, 1)
             for one in engine.recognize(text, kinds=["bill", "ship"]).value.entities]
    spans = list({(span["text"], span["kind"]): span
                  for span in got.select("text", "kind").to_dicts()}.values())
    rules = {"knows": ("*", "*")}
    direct = engine.relate([(span["text"], span["kind"]) for span in spans],
                           relations=rules).value
    handed = engine.relate(spans, relations=rules).value
    edges = lambda rows: [(one.relation, one.source.name, one.target.name) for one in rows]
    print(got.columns == ["row", "text", "start", "end", "length", "kind", "strength"],
          got.rows() == alone, len(spans) > 1, len(edges(handed)) > 0,
          edges(handed) == edges(direct))
    """, child_env(backend, tmp_path))
    assert printed.strip() == "True True True True True"


def test_importing_the_package_leaves_polars_and_pandas_out(backend, tmp_path):
    """The wheel never imports Polars or pandas. With pandas blocked, a list
    and a Polars Series still answer. Regression: an import at module top,
    inside a ``try`` or not."""
    done = subprocess.run([sys.executable, "-c", "import sys, thinkthen; print("
                           "'polars' in sys.modules, 'pandas' in sys.modules)"],
                          capture_output=True, text=True, check=True, env=clean_env())
    assert done.stdout.strip() == "False False"
    printed = run("""
    import sys
    sys.modules["pandas"] = None
    import polars as pl, thinkthen as tt
    late = tt.question(decide="Is it late?")
    print(tt.decide_many(late, ["a", "b"]).value, tt.decide_many(late, pl.Series(["a"])).value.to_list())
    """, child_env(backend, tmp_path))
    assert printed.strip() == "[True, True] [True]"


def test_the_slide_sample_runs_as_drawn(backend, tmp_path):
    """The slide's ``tt.annotate("form.json", df, on="body").value`` on a Polars
    frame adds one column per question to the frame the slide drew."""
    (tmp_path / "form.json").write_text(
        '{"version":1,"questions":{"wants_refund":{"decide":"Does the customer ask for a '
        'refund?"},"team":{"choose":"Which team owns this?","options":["billing","shipping",'
        '"account"]},"urgency":{"score":"How urgent is this?","levels":["Routine.","Soon.",'
        '"Immediate."]}}}')
    printed = run(f"""
    import os, polars, thinkthen as tt
    os.chdir({str(tmp_path)!r})
    df = polars.DataFrame({{"body": ["I was charged twice. Please refund the duplicate."]}})
    df = tt.annotate("form.json", df, on="body").value
    print(df.columns, df["wants_refund"].to_list(), df["team"].to_list(),
          df["urgency"].to_list(), df["failed"].to_list())
    """, child_env(backend, tmp_path))
    # The generic replay returns noul=.9, first choice .9, and score
    # 0*.9 + 1*.05 + 2*.05 = .15 under the weighted-level contract.
    assert printed.strip() == ("['body', 'wants_refund', 'team', 'urgency', 'failed'] "
                               "[True] ['billing'] [0.15] [None]")
