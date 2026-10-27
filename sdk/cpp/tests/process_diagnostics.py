"""Capture an ABI consumer's output and resource usage without sampling."""
import os
import signal
import subprocess
import sys
import tempfile


def run(command):
    # Files avoid pipe backpressure while wait4 collects this child's usage.
    with tempfile.TemporaryFile(mode="w+") as stdout, tempfile.TemporaryFile(mode="w+") as stderr:
        with subprocess.Popen(command, stdout=stdout, stderr=stderr) as process:
            _, status, usage = os.wait4(process.pid, 0)
            process.returncode = os.waitstatus_to_exitcode(status)
        stdout.seek(0)
        stderr.seek(0)
        result = subprocess.CompletedProcess(command, process.returncode, stdout.read(), stderr.read())
    if result.returncode:
        peak_bytes = usage.ru_maxrss * (1 if sys.platform == "darwin" else 1024)
        reason = (f"signal={signal.Signals(-result.returncode).name} ({-result.returncode})"
                  if result.returncode < 0 else f"exit={result.returncode}")
        print(f"C++ ABI consumer pid={process.pid} {reason} peak_rss_bytes={peak_bytes}; "
              "child peak RSS does not identify the signal sender or the job memory peak",
              file=sys.stderr, flush=True)
    return result
