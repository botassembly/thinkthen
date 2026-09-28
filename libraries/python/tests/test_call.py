"""Public call accounts at the Python boundary, over the offline listener.

These cases protect the new wrapper, batch controls, and detached receipt.
The old answer tests inspect bare values and cannot see missing accounts or
one request per row. No test-only export or provider call is needed.
"""

import os
import signal

from conftest import child_env, run, start


def test_batches_labels_and_owned_details(backend, tmp_path):
    printed = run("""
    import thinkthen as tt
    engine = tt.Engine(cache=False)
    decide = tt.question(decide="Is it late?")
    rows = ["one", "two", "three"]
    packed = engine.decide_many(decide, rows)
    separate = engine.decide_many(decide, rows, batch=1)
    assert packed.value == separate.value == [True, True, True]
    assert (packed.facts.records, packed.facts.requests_sent) == (3, 1)
    assert (separate.facts.records, separate.facts.requests_sent) == (3, 3)
    assert [entry["index"] for entry in packed.details] == [0, 1, 2]
    assert sum(entry["requests_sent"] for entry in packed.details) == 1
    assert packed.details[0]["request_digests"]
    try:
        packed.details[0]["index"] = 9
    except TypeError:
        pass
    else:
        raise AssertionError("an observation was mutable")
    choice = engine.choose_many("Which team?", rows, options=["billing", "shipping"])
    score = tt.score_many("How urgent?", rows, levels=["Routine.", "Soon.", "Now."])
    tags = engine.tag_many("Which kinds?", rows, labels=["bill", "ship"])
    assert choice.value == ["billing"] * 3
    assert score.value == [0.15] * 3
    assert tags.value == [["bill", "ship"]] * 3
    assert all(item.facts.records == 3 and len(item.details) == 3
               for item in (choice, score, tags))
    before = engine.usage()["requests_sent"]
    for call in (lambda: engine.decide_many(decide, rows, batch=0),
                 lambda: engine.decide_many(decide, rows, context="  "),
                 lambda: engine.annotate({"version": 1, "questions": {"late": {"decide": "Late?"}}},
                                         rows, context="reference")):
        try:
            call()
        except tt.UsageError:
            pass
        else:
            raise AssertionError("unsupported control was accepted")
    assert engine.usage()["requests_sent"] == before
    shared = engine.decide_many(decide, rows, batch=1, context="reference")
    assert shared.value == packed.value
    assert shared.details[0]["request_digests"] != separate.details[0]["request_digests"]
    print("calls", packed.facts.requests_sent, separate.facts.requests_sent,
          choice.facts.requests_sent, score.facts.requests_sent, tags.facts.requests_sent)
    """, child_env(backend, tmp_path))
    assert printed.strip() == "calls 1 3 1 1 1"
    assert backend.count() == 10


def test_returned_failure_keeps_its_final_account(backend, tmp_path):
    printed = run("""
    import thinkthen as tt
    try:
        tt.Engine(cache=False).decide_many(tt.question(decide="Q?"), ["a", "b"], batch=1)
    except tt.BackendError as error:
        assert (error.facts.records, error.facts.requests_sent) == (0, 1)
        assert error.details == ()
        print("failed", error.kind, error.facts.requests_sent)
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
    assert (column.facts.records, len(column.details)) == (2, 2)
    form = {"version": 1, "questions": {"late": {"decide": "Late?"}}}
    frame = engine.annotate(form, pl.DataFrame({"body": ["one", "two"]}), on="body")
    assert frame.value.columns == ["body", "late", "failed"]
    assert frame.value["failed"].is_null().all()
    assert (frame.facts.records, len(frame.details)) == (2, 2)
    named = engine.recognize(pl.DataFrame({"body": ["one", "two"]}),
                             kinds=["PERSON"], on="body")
    assert named.value.columns == ["row", "text", "start", "end", "length",
                                   "kind", "strength"]
    assert named.facts.records == 2
    assert [entry["index"] for entry in named.details] == [0, 0, 1, 1]
    print("frames", column.facts.records, frame.facts.records, named.facts.records)
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
        assert (final.facts.records, final.facts.requests_sent) == (0, 1)
        assert receipt.done and final.details is not None
        print("final", final.facts.requests_sent, flush=True)
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
        print("final", final.outcome, final.facts.requests_sent, flush=True)
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
        assert (error.facts.records, len(error.details)) == (2, 2)
        print("local", error.facts.records, str(error.__cause__))
    else:
        raise AssertionError("the host rebuilt")
    """, child_env(backend, tmp_path))
    assert printed.strip() == "local 2 cannot rebuild"
    assert backend.count() == 1
