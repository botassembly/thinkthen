"""Translate a shared workload file to native arguments or an R interrupt.

Use through scripts/stress-adapters.py --surface LANGUAGE -- python3 sdk/c/tests/stress.py LANGUAGE COMMAND ...
The installed consumer owns row validation and emits its bounded JSON summary.
"""
import json
import os
from pathlib import Path
import queue
import signal
import subprocess
import sys
import threading

language, *command = sys.argv[1:]
case = json.loads(Path(os.environ['THINKTHEN_STRESS_CASE']).read_text())
if language != 'r':
    command += ['stress-cancel' if case['cancel'] else 'stress', json.dumps(case['client']),
                str(case['records']), case['scenario']['status'], case['scenario'].get('code', ''),
                str(case['probability']), str(case['concurrency']), str(case['rounds']), str(case['identifier_base'])]
    # Native consumers read the first c in the runner's cancel line themselves.
    os.execvp(command[0], command)
if not case['cancel']:
    os.execvp(command[0], command)

# R owns one thread and handles its normal interrupt signal. Observe both stdin
# and child completion so an R startup failure cannot strand the runner's arrival.
with subprocess.Popen(command, stdin=subprocess.DEVNULL) as child:
    events = queue.Queue()
    threading.Thread(target=lambda: events.put(('input', sys.stdin.readline())), daemon=True).start()
    threading.Thread(target=lambda: events.put(('exit', child.wait())), daemon=True).start()
    try:
        kind, value = events.get()
        assert kind == 'input' and value.strip() == 'cancel', 'R stopped before cancellation event'
        child.send_signal(signal.SIGINT)
        assert child.wait() == 0, 'R stress consumer failed'
    finally:
        if child.poll() is None:
            child.kill()
            child.wait()
