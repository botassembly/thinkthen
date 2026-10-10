"""Run the complete required shared Ada inventory only at a release candidate."""
import argparse
import importlib.util
import os
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[3]
spec = importlib.util.spec_from_file_location('shared_session_cases', ROOT / 'libraries/cpp/fixtures/session_cases.py')
shared = importlib.util.module_from_spec(spec)
spec.loader.exec_module(shared)
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--package', type=Path, required=True)
parser.add_argument('--case', action='append', default=[])
args = parser.parse_args()
package = args.package.resolve(strict=True)
os.environ['THINKTHEN_TEST_PROFILE'] = 'full'
if args.case:
    os.environ['THINKTHEN_ADA_CASES'] = ','.join(args.case)


def invoke(step, settings, env, home, backend):
    fixture = home / 'fixture.json'
    fixture.write_text(shared.shared.compact(step))
    command = [sys.executable, str(Path(__file__).with_name('full_case.py')),
               str(package), str(fixture), shared.shared.compact(settings)]
    if not step.get('held_cancel'):
        return subprocess.run(command, cwd=home, env=env, capture_output=True, text=True, timeout=180)
    child = subprocess.Popen(command, cwd=home, env=env, stdin=subprocess.PIPE,
                             stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    try:
        assert backend.read('wait 1') == 'wait 1'
        child.stdin.write('!\n'); child.stdin.flush()
        assert child.stdout.readline() == 'cancel-fired\n'
        backend.process.stdin.write('release\n'); backend.process.stdin.flush()
        stdout, stderr = child.communicate(timeout=120)
        return subprocess.CompletedProcess(command, child.returncode, stdout, stderr)
    finally:
        backend.process.stdin.write('release\n'); backend.process.stdin.flush()
        if child.poll() is None:
            child.kill(); child.wait()


shared.native_cases(Path(__file__).with_name('full_case.py'), consumer='ada', invoke=invoke)
