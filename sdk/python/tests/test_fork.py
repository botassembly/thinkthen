"""A forked child can go on using the client its parent made."""

from __future__ import annotations

import os
import signal
import sys

import pytest

import thinkthen as tt


@pytest.mark.skipif(
    sys.platform != "linux", reason="the check forks a process that holds threads"
)
def test_a_forked_child_calls_the_client_its_parent_used() -> None:
    client = tt.Client(backend="stub")
    client.decide("Ready?", "alpha")
    child = os.fork()
    if child == 0:
        status = 1
        try:
            # A call that never returns ends the child by this signal.
            signal.alarm(30)
            client.decide("Ready?", "beta")
            client.close()
            status = 0
        finally:
            os._exit(status)
    _, status = os.waitpid(child, 0)
    client.close()
    assert os.waitstatus_to_exitcode(status) == 0
