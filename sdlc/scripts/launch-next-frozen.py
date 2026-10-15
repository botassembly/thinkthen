#!/usr/bin/env python3
"""Launch tracked gate owners after a coordinator freezes a commit."""
import argparse
import json
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[2]
BASE = Path(tempfile.gettempdir()) / 'thinkthen-0543-final-target'
SCRIPTS = Path(__file__).resolve().parent

p = argparse.ArgumentParser(description=__doc__)
p.add_argument('kind', choices=('local', 'hosted'))
p.add_argument('sha')
p.add_argument('output', type=Path)
p.add_argument('--run', action='store_true', required=True)
a = p.parse_args()
if not re.fullmatch(r'[0-9a-f]{40}', a.sha):
    p.error('SHA must be a full commit')
out = a.output.resolve()
if out.parent != BASE or a.sha[:9] not in out.name:
    p.error('output must be a new SHA-named folder directly under ' + str(BASE))
assert subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip() == a.sha
assert not subprocess.check_output(['git', 'status', '--porcelain'], cwd=ROOT, text=True)
if a.kind == 'hosted':
    remote = subprocess.check_output(['git', 'ls-remote', 'origin', 'refs/heads/ticket/0543-final'], cwd=ROOT, text=True).split()[0]
    assert remote == a.sha, 'push the frozen SHA before dispatch'
else:
    assert shutil.disk_usage(ROOT).free >= 50 * 1024**3
out.mkdir(mode=0o700)
(out / 'OWNER').write_text('Ticket: 0543\nRevision: ' + a.sha + '\nRole: ' + a.kind + ' frozen gate owner\n')
with (out / 'supervisor.log').open('wb') as log:
    args = [sys.executable, '-u', str(SCRIPTS / ('release-' + a.kind + '-gates.py')), a.sha, str(out)]
    child = subprocess.Popen(args, cwd=ROOT, stdout=log, stderr=subprocess.STDOUT,
                             start_new_session=True, stdin=subprocess.DEVNULL)
(out / 'launch.pid').write_text(str(child.pid) + '\n')
print(json.dumps({'kind': a.kind, 'revision': a.sha, 'pid': child.pid, 'output': str(out)}))
