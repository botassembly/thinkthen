"""Stopping a call: a caller's token, Ctrl-C, and a handler's own exit.

Each call runs in a child on the held arm, whose replies wait for a
``release`` line. The parent acts once the backend's count shows the sends
in flight, so a stop never lands before the send it should cut short.
Children print ``time.monotonic()``, which reads one system clock, so the
parent measures from its own signal to the child's ``Cancelled``.

The routine run proves order: ``Cancelled`` arrives while the reply is still
held, and nothing is sent after it. The 100 ms promise needs an idle machine,
so each test's ``within_100_ms`` twin checks it under the stress profile
(ticket 0352).
"""

import os
import signal
import time

import pytest

import thinkthen as tt
from conftest import child_env, start

TEXTS = "[f'note {n}' for n in range(200)]"
HOLD = f"""
    import signal, sys, threading, time, thinkthen as tt
    # A child of a background job inherits an ignored SIGINT. Python's own
    # handler is the one an interactive user has.
    signal.signal(signal.SIGINT, signal.default_int_handler)
    engine = tt.Engine(throttle=8, batch=1, cache=False)
    late = tt.question(decide="Is it late?")
    token = tt.CancelToken()
    def stop():
        sys.stdin.readline()
        print("stopped", time.monotonic(), flush=True)
        token.cancel()
    def settle():
        sys.stdin.readline()
        ended = time.monotonic() + 30
        while tt._thinkthen._live_workers() and time.monotonic() < ended:
            time.sleep(0.01)
        print("live", tt._thinkthen._live_workers(), flush=True)
    texts = {TEXTS}
"""
STOP = """
    threading.Thread(target=stop, daemon=True).start()
"""


@pytest.fixture(params=[
    pytest.param(None, id="order"),
    pytest.param(0.1, id="within_100_ms", marks=pytest.mark.stress),
])
def within(request):
    """No bound in the routine run; 100 ms under the stress profile."""
    return request.param


def quick(took, within):
    """The stop's delay meets the bound, when this twin has one."""
    assert within is None or took < within, took


def stamp(child):
    """The word and the time on the child's next line."""
    word, when, *_ = child.stdout.readline().split(maxsplit=2)
    return word, float(when)


def stopped(child, expected, stopper):
    """Stop the child, then return seconds from the stop to its ``Cancelled``."""
    began = stopper()
    word, when = stamp(child)
    if word == "stopped":
        began, (word, when) = when, stamp(child)
    if word != expected:
        child.kill()
        raise AssertionError(f"{word} in place of {expected}: {child.stderr.read()}")
    return when - began


def settle(backend, child, sent):
    """The count holds after the stop, then the released workers end
    without sending more."""
    time.sleep(0.3)
    assert backend.count() == sent
    backend.release()
    child.stdin.write("released\n")
    child.stdin.flush()
    assert child.stdout.readline().split() == ["live", "0"]
    assert child.wait(timeout=60) == 0, child.stderr.read()
    assert backend.count() == sent


def signal_child(child):
    began = time.monotonic()
    os.kill(child.pid, signal.SIGINT)
    return began


def tell_child(child):
    child.stdin.write("stop\n")
    child.stdin.flush()
    return None


def test_cancelled_is_a_keyboard_interrupt_and_a_thinkthen_error():
    """R2-25: either ``except`` clause catches a stop."""
    assert issubclass(tt.Cancelled, KeyboardInterrupt)
    assert issubclass(tt.Cancelled, tt.ThinkThenError)
    assert tt.Cancelled.kind == "cancelled"


def test_a_token_cancelled_before_the_call_sends_nothing(backend, tmp_path):
    """R1-24: the token is read before the worker starts."""
    child = start(HOLD + """
    token.cancel()
    try:
        engine.decide(late, texts, token=token).value
    except tt.Cancelled as error:
        print("cancelled", time.monotonic(), error)
    """, child_env(backend, tmp_path, "arm/held"))
    assert stamp(child)[0] == "cancelled"
    assert child.wait(timeout=60) == 0
    assert backend.count() == 0


def test_a_token_stops_a_held_batch_before_next_request(backend, tmp_path, within):
    """R1-24: a token cancelled during a 200-text batch raises within one
    tick, and no send follows the first held request."""
    child = start(HOLD + STOP + """
    try:
        engine.decide(late, texts, token=token).value
    except tt.Cancelled as error:
        print("cancelled", time.monotonic(), error, flush=True)
    settle()
    """, child_env(backend, tmp_path, "arm/held"))
    assert backend.wait(1) == 1
    quick(stopped(child, "cancelled", lambda: tell_child(child)), within)
    settle(backend, child, 1)


def test_a_token_stops_a_held_single_send(backend, tmp_path, within):
    """Other acceptance: the 50 ms tick reads the caller's token during one
    blocking send, so ``Cancelled`` arrives within 100 ms."""
    child = start(HOLD + STOP + """
    try:
        engine.decide(late, "one note", token=token).value
    except tt.Cancelled as error:
        print("cancelled", time.monotonic(), error, flush=True)
    settle()
    """, child_env(backend, tmp_path, "arm/held"))
    assert backend.wait(1) == 1
    quick(stopped(child, "cancelled", lambda: tell_child(child)), within)
    settle(backend, child, 1)


def test_a_token_fired_as_the_reply_lands_cancels_the_call(backend, tmp_path):
    """A token fired before a held reply is released wins at the wait boundary."""
    child = start(HOLD + """
    def fire():
        sys.stdin.readline()
        token.cancel()
        print("stopped", flush=True)
    threading.Thread(target=fire, daemon=True).start()
    try:
        answer = engine.decide(late, "one note", token=token).value
        print("answer", answer, flush=True)
    except tt.Cancelled as error:
        print("cancelled", str(error), flush=True)
    """, child_env(backend, tmp_path, "arm/held"))
    assert backend.wait(1) == 1
    tell_child(child)
    assert child.stdout.readline().strip() == "stopped"
    backend.release()
    assert child.stdout.readline().strip() == "cancelled the call was cancelled"
    assert child.wait(timeout=60) == 0, child.stderr.read()
    assert backend.count() == 1


@pytest.mark.skipif(os.name == "nt", reason="os.kill(SIGINT) does not deliver a Windows console Ctrl-C")
def test_ctrl_c_stops_a_held_single_send_at_once(backend, tmp_path, within):
    """R4-23 (single): ``SIGINT`` during one held send raises ``Cancelled``
    within 100 ms, and the send is not repeated."""
    child = start(HOLD + """
    try:
        engine.decide(late, "one note").value
    except tt.Cancelled as error:
        print("cancelled", time.monotonic(), error, flush=True)
    settle()
    """, child_env(backend, tmp_path, "arm/held"))
    assert backend.wait(1) == 1
    quick(stopped(child, "cancelled", lambda: signal_child(child)), within)
    settle(backend, child, 1)


@pytest.mark.skipif(os.name == "nt", reason="os.kill(SIGINT) does not deliver a Windows console Ctrl-C")
def test_ctrl_c_stops_a_held_batch_at_once(backend, tmp_path, within):
    """Amendment change 3: ``SIGINT`` with a send held raises within 100 ms.
    After release the worker ends without another send."""
    child = start(HOLD + """
    try:
        engine.decide(late, texts).value
    except KeyboardInterrupt as error:
        print("cancelled", time.monotonic(), type(error).__name__, error, flush=True)
    settle()
    """, child_env(backend, tmp_path, "arm/held"))
    assert backend.wait(1) == 1
    quick(stopped(child, "cancelled", lambda: signal_child(child)), within)
    settle(backend, child, 1)


@pytest.mark.skipif(os.name == "nt", reason="os.kill(SIGINT) does not deliver a Windows console Ctrl-C")
def test_a_handlers_system_exit_passes_through_unchanged(backend, tmp_path, within):
    """R2-25: only a ``KeyboardInterrupt`` becomes ``Cancelled``. A handler
    that exits surfaces its own ``SystemExit``."""
    child = start(HOLD + """
    signal.signal(signal.SIGINT, lambda *_: sys.exit(3))
    try:
        engine.decide(late, texts).value
    except tt.Cancelled:
        print("cancelled", time.monotonic(), flush=True)
    except SystemExit as leaving:
        print("exit", time.monotonic(), leaving.code, flush=True)
    settle()
    """, child_env(backend, tmp_path, "arm/held"))
    assert backend.wait(1) == 1
    quick(stopped(child, "exit", lambda: signal_child(child)), within)
    settle(backend, child, 1)


@pytest.mark.skipif(os.name == "nt", reason="os.kill(SIGINT) does not deliver a Windows console Ctrl-C")
def test_ctrl_c_stops_a_held_polars_column_at_once(backend, tmp_path, within):
    """R4-23, the Polars half: a column ``score`` runs on the detachable
    worker, so ``SIGINT`` with a send held raises within 100 ms and the
    count stays at 1. Regression: a column run on the
    calling thread, or a worker joined in place of detached."""
    child = start(HOLD + """
    import polars as pl
    urgent = tt.question(score="How urgent?", levels=["Routine.", "Soon.", "Now."])
    try:
        engine.score(urgent, pl.Series(texts)).value
    except tt.Cancelled as error:
        print("cancelled", time.monotonic(), error, flush=True)
    settle()
    """, child_env(backend, tmp_path, "arm/held"))
    assert backend.wait(1) == 1
    quick(stopped(child, "cancelled", lambda: signal_child(child)), within)
    settle(backend, child, 1)
