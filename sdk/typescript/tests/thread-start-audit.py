#!/usr/bin/env python3
"""Audit actual pthread starts for a 1,000-row installed-package stream."""
import os
from pathlib import Path
import subprocess
import sys
import tempfile

source = Path(__file__).resolve().parents[3] / 'scripts/tests/thread-start-audit.c'
support = Path(__file__).with_name('support.js').resolve().as_uri()
program = '''
const {sdk} = await import(process.env.THINKTHEN_AUDIT_SUPPORT);
const client = new sdk.ThinkThen({backend: "stub"});
try {
  async function* records() { for (let i = 0; i < 1000; i++) yield "record"; }
  let count = 0;
  if (process.env.THINKTHEN_AUDIT_STREAM === "1") {
    for await (const row of client.decideStream("Ready?", records())) {
      if (row.position !== ++count || row.status !== "answered") throw new Error("wrong row");
    }
    if (count !== 1000) throw new Error("wrong row count");
  }
} finally { await client.close(); }
'''
with tempfile.TemporaryDirectory(prefix='thinkthen-thread-audit-') as folder:
    root = Path(folder)
    library = root / ('audit.dylib' if sys.platform == 'darwin' else 'audit.so')
    flags = ['-dynamiclib'] if sys.platform == 'darwin' else ['-shared', '-fPIC', '-ldl']
    subprocess.run(['cc', '-Wall', '-Wextra', '-Werror', *flags, str(source), '-o', str(library)], check=True)
    results = []
    for mode in ('0', '1'):
        log = root / (mode + '.log')
        env = dict(os.environ, THINKTHEN_AUDIT_SUPPORT=support, THINKTHEN_AUDIT_STREAM=mode,
                   THINKTHEN_THREAD_AUDIT=str(log), THINKTHEN_THREAD_NAME="thinkthen-node")
        env['DYLD_INSERT_LIBRARIES' if sys.platform == 'darwin' else 'LD_PRELOAD'] = str(library)
        subprocess.run(['node', '--input-type=module', '-e', program], env=env, check=True, timeout=30)
        lines = log.read_text().splitlines()
        print(f'audit mode={mode}: {lines}')
        results.append((sum(line.startswith('start\t') for line in lines), lines.count('named')))
    baseline, stream = results
    assert baseline[1] == 0, results
    assert stream[1] == 1, results
    # The engine owns one driver, four answer workers and one packer;
    # the addon adds exactly one bridge for the entire stream.
    assert stream[0] - baseline[0] == 7, results
    print(f'1000 rows: actual pthread starts baseline={baseline[0]}, stream={stream[0]}; '
          'one addon bridge plus six shared engine threads')
