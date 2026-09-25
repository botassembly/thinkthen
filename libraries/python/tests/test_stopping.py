"""Stopping a call: a caller's token, Ctrl-C, and a handler's own exit.

Each call runs in a child on the held arm, whose replies wait for a
``release`` line. The parent acts once the backend's count shows the sends
in flight, so a stop never lands before the send it should cut short.
Children print ``time.monotonic()``, which reads one system clock, so the
parent measures from its own signal to the child's ``Cancelled``.
"""

import os
import signal
import time

import thinkthen as tt
from conftest import child_env, start

TEXTS = "[f'note {n}' for n in range(200)]"
HOLD = f"""
    import signal, sys, threading, time, thinkthen as tt
    # A child of a background job inherits an ignored SIGINT. Python's own
    # handler is the one an interactive user has.
    signal.signal(signal.SIGINT, signal.default_int_handler)
    engine = tt.Engine(throttle=8, cache=False)
    late = tt.question(decide="Is it late?")
    token = tt.CancelToken()
    def stop():
        sys.stdin.readline()
        print("stopped", time.monotonic(), flush=True)
        token.cancel()
    def settle():
        sys.stdin.readline()
        ended = time.monotonic() + 2
        while tt._thinkthen._live_workers() and time.monotonic() < ended:
            time.sleep(0.01)
        print("live", tt._thinkthen._live_workers(), flush=True)
    texts = {TEXTS}
"""
STOP = """
    threading.Thread(target=stop, daemon=True).start()
"""


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
    assert child.wait(timeout=10) == 0, child.stderr.read()
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
        engine.decide_many(late, texts, token=token)
    except tt.Cancelled as error:
        print("cancelled", time.monotonic(), error)
    """, child_env(backend, tmp_path, "arm/held"))
    assert stamp(child)[0] == "cancelled"
    assert child.wait(timeout=10) == 0
    assert backend.count() == 0


def test_a_token_stops_a_held_batch_at_the_throttle(backend, tmp_path):
    """R1-24: a token cancelled from another thread during a 200-text batch
    at throttle 8 raises ``Cancelled`` within one tick, and no send follows
    the 8 in flight."""
    child = start(HOLD + STOP + """
    try:
        engine.decide_many(late, texts, token=token)
    except tt.Cancelled as error:
        print("cancelled", time.monotonic(), error, flush=True)
    settle()
    """, child_env(backend, tmp_path, "arm/held"))
    assert backend.wait(8) == 8
    assert stopped(child, "cancelled", lambda: tell_child(child)) < 0.1
    settle(backend, child, 8)


def test_a_token_stops_a_held_single_send(backend, tmp_path):
    """Other acceptance: the 50 ms tick reads the caller's token during one
    blocking send, so ``Cancelled`` arrives within 100 ms."""
    child = start(HOLD + STOP + """
    try:
        engine.decide(late, "one note", token=token)
    except tt.Cancelled as error:
        print("cancelled", time.monotonic(), error, flush=True)
    settle()
    """, child_env(backend, tmp_path, "arm/held"))
    assert backend.wait(1) == 1
    assert stopped(child, "cancelled", lambda: tell_child(child)) < 0.1
    settle(backend, child, 1)


def test_ctrl_c_stops_a_held_single_send_at_once(backend, tmp_path):
    """R4-23 (single): ``SIGINT`` during one held send raises ``Cancelled``
    within 100 ms, and the send is not repeated."""
    child = start(HOLD + """
    try:
        engine.decide(late, "one note")
    except tt.Cancelled as error:
        print("cancelled", time.monotonic(), error, flush=True)
    settle()
    """, child_env(backend, tmp_path, "arm/held"))
    assert backend.wait(1) == 1
    assert stopped(child, "cancelled", lambda: signal_child(child)) < 0.1
    settle(backend, child, 1)


def test_ctrl_c_stops_a_held_batch_at_once(backend, tmp_path):
    """Amendment change 3: ``SIGINT`` with 8 sends held raises ``Cancelled``
    within 100 ms, the count stays at 8, and after the release every worker
    ends with no further send."""
    child = start(HOLD + """
    try:
        engine.decide_many(late, texts)
    except KeyboardInterrupt as error:
        print("cancelled", time.monotonic(), type(error).__name__, error, flush=True)
    settle()
    """, child_env(backend, tmp_path, "arm/held"))
    assert backend.wait(8) == 8
    assert stopped(child, "cancelled", lambda: signal_child(child)) < 0.1
    settle(backend, child, 8)


def test_a_handlers_system_exit_passes_through_unchanged(backend, tmp_path):
    """R2-25: only a ``KeyboardInterrupt`` becomes ``Cancelled``. A handler
    that exits surfaces its own ``SystemExit``."""
    child = start(HOLD + """
    signal.signal(signal.SIGINT, lambda *_: sys.exit(3))
    try:
        engine.decide_many(late, texts)
    except tt.Cancelled:
        print("cancelled", time.monotonic(), flush=True)
    except SystemExit as leaving:
        print("exit", time.monotonic(), leaving.code, flush=True)
    settle()
    """, child_env(backend, tmp_path, "arm/held"))
    assert backend.wait(8) == 8
    assert stopped(child, "exit", lambda: signal_child(child)) < 0.1
    settle(backend, child, 8)
