#!/usr/bin/env python3
"""Snapshot the unchanged consumer externally at its existing round checkpoint."""
import json
import os
from pathlib import Path
import runpy
import time

output = Path(os.environ['THINKTHEN_MEMORY_OUTPUT'])
pid = os.getppid()
round_number = len(list(output.glob('round-*-status.txt'))) + 1
for name in ('status', 'smaps', 'smaps_rollup', 'maps'):
    (output / f'round-{round_number:03d}-{name}.txt').write_bytes(Path(f'/proc/{pid}/{name}').read_bytes())
for name in (f'perf-{pid}.map', f'perfinfo-{pid}.map'):
    source = Path('/tmp') / name
    if source.exists():
        (output / f'round-{round_number:03d}-{name}').write_bytes(source.read_bytes())
with (output / 'checkpoints.jsonl').open('a') as stream:
    stream.write(json.dumps(dict(round=round_number,pid=pid,time_ns=time.time_ns())) + '\n')
runpy.run_path(str(Path(__file__).with_name('stress-checkpoint')), run_name='__main__')
