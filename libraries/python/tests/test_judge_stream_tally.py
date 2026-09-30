"""Public Judge, Stream and Tally boundaries under the offline listener."""

import os
import signal

from conftest import child_env, run, start


def test_shape_rule_plan_and_removed_names(backend, tmp_path):
    """An omitted input captures one question; only true iterators are lazy.
    Planning reads the same captured specification without calling a judge."""
    printed = run("""
    import numpy as np, pandas as pd, thinkthen as tt
    engine = tt.Engine(cache=False)
    parts = ["billing", "shipping"]
    judge = engine.choose("Which team?", options=parts)
    parts.append("changed")
    before = engine.usage()["requests_sent"]
    preview = tt.plan(judge, ["one", "two"])
    print(type(judge).__name__, preview["records"], preview["requests"],
          b"changed" not in preview["first_body"], engine.usage()["requests_sent"] == before)
    question = engine.decide("Is it late?")
    for rows in (["one"], ("one",), np.array(["one"]),
                 {"one": 1}.keys(), pd.Index(["one"])):
        print(type(question(rows)).__name__)
    try:
        question(range(1))
    except tt.UsageError as error:
        print(error.kind, "record 0" in str(error))
    stream = question(iter(["one"]))
    print(type(stream).__name__, stream.facts, list(stream), stream.facts.records)
    try:
        question({"one"})
    except tt.UsageError as error:
        print(error.kind, error.retryable)
    try:
        bool(question("one"))
    except TypeError as error:
        print("tt.filter(q)(xs)" in str(error))
    for name in ("decide_many", "choose_many", "score_many", "tag_many"):
        for owner in (tt, engine):
            try: getattr(owner, name)
            except AttributeError as error:
                print(str(error))
    """, child_env(backend, tmp_path))
    assert printed.splitlines() == [
        "Judge 2 1 True True", *["Call"] * 5, "usage True",
        "Stream None [True] 1", "usage False", "True",
        *[f"thinkthen: {name} was removed; apply the judge to a list: tt.decide(q)(rows)"
          for name in ("decide_many", "choose_many", "score_many", "tag_many") for _ in range(2)],
    ]
    assert backend.count() == 7  # five eager shapes, one stream, one scalar bool witness.


def test_cursor_stream_reads_on_caller_thread_and_closes(backend, tmp_path):
    """The real core batch asks a caller-thread sqlite3 cursor only in next()."""
    printed = run("""
    import itertools, sqlite3, threading, thinkthen as tt
    owner = threading.get_ident()
    db = sqlite3.connect(":memory:")
    db.execute("create table notes (body text)")
    db.executemany("insert into notes values (?)", [(f"note {i}",) for i in range(20)])
    pulled = []
    def source():
        for (body,) in db.execute("select body from notes order by rowid"):
            assert threading.get_ident() == owner
            pulled.append(body)
            yield body
    judge = tt.Engine(cache=False).decide("Is it late?", batch=1)
    stream = judge(source())
    print(len(pulled), stream.facts)
    with stream:
        print(list(itertools.islice(stream, 3)))
    print(len(pulled), stream.facts.records, stream.facts.requests_sent)
    db.close()
    """, child_env(backend, tmp_path))
    assert printed.splitlines() == ["0 None", "[True, True, True]", "3 3 3"]
    assert backend.count() == 3


def test_judge_pickle_spawn_and_no_key_plan(backend, tmp_path):
    """A spawn child gets its own key from its explicit test environment;
    the judge pickle carries a validated question and no key or tally copy."""
    env = child_env(backend, tmp_path)
    printed = run("""
    import multiprocessing as mp, os, pickle, thinkthen as tt
    judge = tt.Engine(cache=False).decide("Is it late?")
    saved = pickle.dumps(judge)
    print(os.environ["THINKTHEN_API_KEY"].encode() not in saved,
          type(pickle.loads(saved)).__name__)
    child = mp.get_context("spawn").Process(
        target=exec, args=("assert judge('one').value is True", {"judge": judge}))
    child.start()
    child.join(timeout=10)
    print(child.exitcode)
    for value in (tt.decide("Is it late?", tally=tt.Tally()),
                  tt.decide("Is it late?")(iter(["one"]))):
        try: pickle.dumps(value)
        except TypeError as error: print(type(error).__name__)
    """, env)
    assert printed.splitlines() == ["True Judge", "0", "TypeError", "TypeError"]
    assert backend.count() == 1

    no_key = dict(env)
    del no_key["THINKTHEN_API_KEY"]
    printed = run("""
    import pandas as pd, polars as pl, thinkthen as tt
    judge = tt.decide("Is it late?")
    plan = tt.plan(judge, ["one", "two"])
    print(plan["records"], plan["requests"], plan["estimated_bytes"] > 0)
    print(tt.plan(judge, pd.Series(["one", pd.NA], dtype="string"))["records"],
          tt.plan(judge, pl.Series(["one", None]))["records"],
          tt.plan(judge, pd.Index(["one", "two"]))["records"])
    try: tt.plan(judge, iter(["one"]))
    except tt.UsageError as error: print(error.kind)
    """, no_key)
    assert printed.splitlines() == ["2 1 True", "1 1 2", "usage"]
    assert backend.count() == 1


def test_stream_first_failed_row_freezes_partial_facts_and_tally(backend, tmp_path):
    """A failed first row can send one request while completing zero records;
    the Stream and core Tally retain that same finished account."""
    printed = run("""
    import thinkthen as tt
    tally = tt.Tally()
    stream = tt.Engine(cache=False).decide("Is it late?", batch=1, tally=tally)(
        iter(["one", "two"]))
    try: next(stream)
    except tt.BackendError as error:
        print(error.kind, error.retryable, error.facts.records, error.facts.requests_sent)
    print(stream.facts.records, stream.facts.requests_sent,
          tally.facts.records, tally.facts.requests_sent)
    """, child_env(backend, tmp_path, "arm/malformed/missing_answer"))
    assert printed.splitlines() == ["backend False 0 1", "0 1 0 1"]
    assert backend.count() == 1


def test_tally_waits_for_two_started_held_calls(backend, tmp_path):
    """Two native calls have both reached the held listener before release;
    the shared core Tally closes each once after its reply completes."""
    child = start("""
    import concurrent.futures, thinkthen as tt
    tally = tt.Tally()
    judge = tt.Engine(cache=False, throttle=2).decide("Is it late?", tally=tally)
    with concurrent.futures.ThreadPoolExecutor(max_workers=2) as pool:
        values = list(pool.map(judge, ["one", "two"]))
    print([one.value for one in values], tally.facts.records,
          tally.facts.requests_sent, tally.facts.cache_answers, flush=True)
    """, child_env(backend, tmp_path, "arm/held"))
    assert backend.wait(2) == 2
    assert backend.count() == 2
    backend.release()
    assert child.stdout.readline().strip() == "[True, True] 2 2 0"
    assert child.wait(timeout=10) == 0, child.stderr.read()
    assert backend.count() == 2


def test_stream_token_before_first_pull_and_fork_guard(backend, tmp_path):
    """A fired token sends nothing; a child cannot read its parent's stream."""
    printed = run("""
    import os, thinkthen as tt
    engine = tt.Engine(cache=False, batch=1)
    judge = engine.decide("Is it late?")
    token = tt.CancelToken(); token.cancel()
    stopped = judge(iter(["one"]), token=token)
    try: next(stopped)
    except tt.Cancelled as error: print(error.kind, stopped.facts.requests_sent)
    stream = judge(iter(["one", "two"]))
    print(next(stream))
    pipe_in, pipe_out = os.pipe()
    pid = os.fork()
    if pid == 0:
        os.close(pipe_in)
        try: next(stream)
        except tt.UsageError as error: os.write(pipe_out, error.kind.encode())
        os._exit(0)
    os.close(pipe_out)
    print(os.read(pipe_in, 20).decode(), os.waitpid(pid, 0)[1], list(stream))
    """, child_env(backend, tmp_path))
    assert printed.splitlines() == ["cancelled 0", "True", "usage 0 [True]"]
    assert backend.count() == 2


def test_stream_interrupt_retains_later_completion_receipt(backend, tmp_path):
    """The caller sees Cancelled promptly; the held reply later completes
    one row before Stop converts the outcome to cancellation."""
    child = start("""
    import signal, sys, thinkthen as tt
    signal.signal(signal.SIGINT, signal.default_int_handler)
    stream = tt.Engine(cache=False, batch=1).decide("Is it late?")(
        iter(["one", "two"]))
    try: next(stream)
    except tt.Cancelled as error:
        print(error.kind, hasattr(error, "completion"), flush=True)
        sys.stdin.readline()
        done = error.completion.result(timeout=5)
        print(done.outcome, done.facts.records, done.facts.requests_sent, flush=True)
    """, child_env(backend, tmp_path, "arm/held"))
    assert backend.wait(1) == 1
    os.kill(child.pid, signal.SIGINT)
    assert child.stdout.readline().strip() == "cancelled True"
    assert backend.count() == 1
    backend.release()
    child.stdin.write("released\n")
    child.stdin.flush()
    assert child.stdout.readline().strip() == "failed 1 1"
    assert child.wait(timeout=10) == 0, child.stderr.read()
    assert backend.count() == 1


def test_stream_second_reader_refuses_while_first_waits(backend, tmp_path):
    """The held first read owns the stream; a second reader cannot race its
    Python source or take the next row."""
    child = start("""
    import sys, threading, thinkthen as tt
    stream = tt.Engine(cache=False, batch=1).decide("Is it late?")(
        iter(["one", "two"]))
    def reader(): print("first", next(stream), flush=True)
    first = threading.Thread(target=reader)
    first.start()
    sys.stdin.readline()
    try: next(stream)
    except tt.UsageError as error: print("second", error.kind, flush=True)
    sys.stdin.readline()
    first.join(timeout=5)
    stream.close()
    print("finished", stream.facts.records, stream.facts.requests_sent, flush=True)
    """, child_env(backend, tmp_path, "arm/held"))
    assert backend.wait(1) == 1
    child.stdin.write("probe\n")
    child.stdin.flush()
    assert child.stdout.readline().strip() == "second usage"
    assert backend.count() == 1
    backend.release()
    assert child.stdout.readline().strip() == "first True"
    assert child.poll() is None, child.stderr.read()
    child.stdin.write("released\n")
    child.stdin.flush()
    assert child.stdout.readline().strip() == "finished 1 1"
    assert child.wait(timeout=10) == 0, child.stderr.read()
    assert backend.count() == 1


def test_dropped_stream_releases_its_native_worker(backend, tmp_path):
    """Dropping the last Python reference cancels the one native batch and
    its scheduler without reading the rest of the caller's source."""
    printed = run("""
    import gc, os, time, thinkthen as tt
    threads = lambda: len(os.listdir("/proc/self/task"))
    baseline = threads()
    read = []
    def source():
        for text in ["one", "two", "three"]:
            read.append(text)
            yield text
    stream = tt.Engine(cache=False, batch=1).decide("Is it late?")(source())
    print(next(stream))
    del stream
    gc.collect()
    until = time.monotonic() + 2
    while threads() != baseline and time.monotonic() < until:
        time.sleep(0.01)
    print(read, threads() == baseline)
    """, child_env(backend, tmp_path))
    assert printed.splitlines() == ["True", "['one'] True"]
    assert backend.count() == 1
