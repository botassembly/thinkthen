"""A column call is one engine call: one deadline, one token, and the
same throttle as the list form. Each child builds its own
``tt.Engine(throttle=8)`` on its own backend's delay or held arm."""

import time

import pytest

from conftest import Backend, child_env, run, start

SETUP = """
    import sys, threading, time, polars as pl, thinkthen as tt
    engine = tt.Engine(throttle=8, batch=1, cache=False)
    late = tt.question(decide="Is it late?")
    urgent = tt.question(score="How urgent?", levels=["Routine.", "Soon.", "Now."])
    texts = [f"note {n}" for n in range(200)]
"""


def test_a_short_column_shares_one_deadline_and_token(backend, tmp_path):
    """A bounded column stops under each public call-wide control. The
    deadline is shorter than one delayed reply. The token fires once the
    held arm counts a send, whose reply it holds, so no timer races a reply.
    At most one throttle's sends go out, and none after the cancel
    (ticket 0352)."""
    printed = run(SETUP + """
    try:
        engine.score(urgent, pl.Series(texts[:12]), deadline_ms=50).value
    except tt.DeadlineError:
        print("deadline")
    """, child_env(backend, tmp_path, "arm/delay/100"))
    assert printed.splitlines() == ["deadline"]
    assert backend.count() <= 8
    held = Backend()
    try:
        child = start(SETUP + """
    token = tt.CancelToken()
    def stop():
        sys.stdin.readline()
        token.cancel()
    threading.Thread(target=stop, daemon=True).start()
    try:
        engine.score(urgent, pl.Series(texts[12:24]), token=token).value
    except tt.Cancelled:
        print("cancelled", flush=True)
    """, child_env(held, tmp_path, "arm/held"))
        assert held.wait(1) >= 1
        child.stdin.write("stop\n")
        child.stdin.flush()
        assert child.stdout.readline().strip() == "cancelled", child.stderr.read()
        sent = held.count()
        held.release()
        assert child.wait(timeout=60) == 0, child.stderr.read()
        assert 1 <= sent <= 8 and held.count() == sent
    finally:
        held.close()


@pytest.mark.stress
def test_one_deadline_and_one_token_cover_a_column(backend, tmp_path):
    """R1-24, the Polars half: at 100 ms a reply, a 1 s deadline stops a
    200-row ``score`` near 1 s with at most 96 sends, and a token set at
    0.5 s stops it within 8 further sends. Regression: options built per
    row restart the deadline, and all 200 send."""
    child = start(SETUP + """
    began = time.monotonic()
    try:
        engine.score(urgent, pl.Series(texts), deadline_ms=1000).value
    except tt.DeadlineError:
        print("deadline", time.monotonic() - began, flush=True)
    else:
        print("answered", time.monotonic() - began, flush=True)
    sys.stdin.readline()
    token = tt.CancelToken()
    threading.Timer(0.5, lambda: (print("stop", flush=True), token.cancel())).start()
    try:
        engine.score(urgent, pl.Series([f"other {n}" for n in range(200)]), token=token).value
    except tt.Cancelled:
        print("cancelled", flush=True)
    """, child_env(backend, tmp_path, "arm/delay/100"))
    word, seconds = child.stdout.readline().split()
    assert word == "deadline" and 0.9 < float(seconds) < 1.3, (word, seconds)
    first = backend.count()
    assert first <= 96
    child.stdin.write("next\n")
    child.stdin.flush()
    assert child.stdout.readline().strip() == "stop"
    at_stop = backend.count()
    assert child.stdout.readline().strip() == "cancelled"
    time.sleep(0.3)
    assert backend.count() - at_stop <= 8
    assert child.wait(timeout=60) == 0, child.stderr.read()


@pytest.mark.stress
def test_a_column_runs_at_the_lists_throttle(backend, tmp_path):
    """The throttle proof Ian named: 200 texts at 100 ms a reply and
    throttle 8 take about 2.5 s as a list and as a ``Series``, within 5
    percent of each other, with 200 sends each and equal answers. The
    upper bound allows for a loaded machine; the ratio is the proof.
    Regression: a per-row call holds 1 in flight and runs eight times
    longer."""
    printed = run(SETUP + """
    def timed(records):
        began = time.monotonic()
        answers = engine.decide(late, records).value
        return time.monotonic() - began, list(answers)
    listed, series = timed(texts), timed(pl.Series(texts))
    print(listed[0], series[0], listed[1] == series[1])
    """, child_env(backend, tmp_path, "arm/delay/100"))
    listed, series, equal = printed.split()
    assert equal == "True"
    assert 2.4 < float(listed) < 4.0 and abs(float(series) - float(listed)) / float(listed) < 0.05
    assert backend.count() == 400


def test_a_column_holds_the_throttle_in_flight(tmp_path):
    """Eight independent packed groups occupy eight permits on the held arm.
    A single default-packed call only makes one request, so it cannot prove
    the throttle. Lists and Series each use their own backend gate."""
    for shape in ("list", "series"):
        backend = Backend()
        try:
            child = start(SETUP + f"""
    engine = tt.Engine(throttle=8, batch="max", cache=False)
    groups = [[f"group {{group}} row {{row}}" for row in range(3)] for group in range(8)]
    answers = []
    def ask(group):
        rows = pl.Series(group) if {shape!r} == "series" else group
        call = engine.decide(late, rows)
        values = call.value.to_list() if hasattr(call.value, "to_list") else call.value
        answers.append((values, call.facts["requests_sent"]))
    held = [threading.Thread(target=ask, args=(group,)) for group in groups]
    for thread in held:
        thread.start()
    for thread in held:
        thread.join()
    assert len(answers) == 8 and all(value == [True] * 3 and sent == 1
                                      for value, sent in answers)
    """,
                          child_env(backend, tmp_path, "arm/held"))
            assert backend.wait(8) == 8
            time.sleep(0.3)
            assert backend.count() == 8, shape
            backend.release()
            assert child.wait(timeout=60) == 0, child.stderr.read()
        finally:
            backend.close()


def test_recognize_on_a_frame_spends_one_deadline(backend, tmp_path):
    """The frame loop resolves its deadline once: 3 texts at 600 ms a reply
    with ``deadline_ms=1000`` raise ``DeadlineError`` after at most 2 sends.
    Regression: a relative deadline per text gives each 1 s, and all 3
    send and answer."""
    printed = run(SETUP + """
    frame = pl.DataFrame({"body": ["one note", "two notes", "three notes"]})
    try:
        engine.recognize(frame, kinds=["note"], on="body", deadline_ms=1000).value
        print("answered")
    except tt.DeadlineError as error:
        print(type(error).__name__)
    """, child_env(backend, tmp_path, "arm/delay/600"))
    assert printed.strip() == "DeadlineError"
    assert backend.count() <= 2
