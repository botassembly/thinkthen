"""Run the complete required shared Ada inventory only at a release candidate."""
import argparse
import importlib.util
import os
from pathlib import Path
import subprocess
import sys
import tempfile

import full_case

ROOT = Path(__file__).resolve().parents[3]
spec = importlib.util.spec_from_file_location('shared_session_cases', ROOT / 'conformance/children/session_cases.py')
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
    env = dict(env, THINKTHEN_API_KEY='sk-conformance-loopback', LIQUIDAI_API_KEY='sk-conformance-loopback', OPENROUTER_API_KEY='sk-conformance-loopback', PERPLEXITY_API_KEY='sk-conformance-loopback')
    fixture = full_case.fixture(step, home)
    command = [str(binary), str(fixture), shared.shared.compact(settings)]
    if not step.get('held_cancel'):
        return subprocess.run(command, cwd=home, env=env, capture_output=True, text=True, timeout=180)
    child = subprocess.Popen(command, cwd=home, env=env, stdin=subprocess.PIPE,
                             stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    try:
        assert backend.read('wait 1') == 'wait 1'
        child.stdin.write('!\n'); child.stdin.flush()
        assert shared.read_process_line(child.stdout) == 'cancel-fired\n'
        backend.process.stdin.write('release\n'); backend.process.stdin.flush()
        stdout, stderr = child.communicate(timeout=120)
        return subprocess.CompletedProcess(command, child.returncode, stdout, stderr)
    finally:
        backend.process.stdin.write('release\n'); backend.process.stdin.flush()
        if child.poll() is None:
            child.kill(); child.wait()


with tempfile.TemporaryDirectory(prefix='thinkthen-ada-objects-') as temporary:
    binary = full_case.build(package, Path(temporary))
    shared.native_cases(Path(__file__).with_name('full_case.py'), consumer='ada', invoke=invoke)
    subprocess.run([sys.executable, str(ROOT / 'conformance/children/usage_installed.py'), '--family', 'ada', '--ada-package', str(package), '--ada-objects', str(binary.parent / 'ada-objects')], check=True)
