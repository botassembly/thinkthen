"""Prove failure diagnostics with real child processes."""
import contextlib
import io
import re
import signal
import sys

from process_diagnostics import run

for code, expected_returncode, reason in [
    ("print('accepted')", 0, None),
    ("import sys; print('output'); print('error', file=sys.stderr); sys.exit(7)", 7, "exit=7"),
    ("import os, signal; data = bytearray(16 * 1024 * 1024); "
     "print('before kill', flush=True); os.kill(os.getpid(), signal.SIGKILL)",
     -signal.SIGKILL, "signal=SIGKILL (9)"),
]:
    diagnostics = io.StringIO()
    with contextlib.redirect_stderr(diagnostics):
        result = run([sys.executable, "-c", code])
    assert result.returncode == expected_returncode, result
    if reason is None:
        assert result.stdout == "accepted\n" and not result.stderr and not diagnostics.getvalue()
    else:
        message = diagnostics.getvalue()
        assert reason in message and "does not identify the signal sender" in message, message
        peak = re.search(r"peak_rss_bytes=(\d+)", message)
        assert peak and int(peak[1]) > 0, message
        if expected_returncode < 0:
            assert int(peak[1]) >= 16 * 1024 * 1024 and result.stdout == "before kill\n", (message, result)
        else:
            assert result.stdout == "output\n" and result.stderr == "error\n", result
print("C++ ABI diagnostics preserve output and exits and report signal and child peak RSS")
