"""Shared routine cases through an installed Ruby gem's named typed calls."""
import os
from pathlib import Path
import subprocess
import sys
ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'libraries/cpp/fixtures'))
from session_cases import descriptor, native_cases
RUBY = os.environ['RUBY']
CONSUMER = Path(__file__).with_name('native_case.rb')
def invoke(step, settings, child, home, backend):
    child.update(GEM_PATH=os.environ['GEM_PATH'], LD_LIBRARY_PATH=os.environ.get('LD_LIBRARY_PATH', ''))
    args = [RUBY, str(CONSUMER), str(home / 'consumer-input.json'), __import__('json').dumps(settings)]
    if step.get('held_cancel'):
        process = subprocess.Popen(args, env=child, cwd=home, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        try:
            assert backend.read('wait 1') == 'wait 1'
            process.stdin.write('!\n'); process.stdin.flush()
            assert process.stdout.readline() == 'cancel-fired\n'
            backend.process.stdin.write('release\n'); backend.process.stdin.flush()
            stdout, stderr = process.communicate(timeout=60)
            return subprocess.CompletedProcess(args, process.returncode, stdout, stderr)
        finally:
            backend.process.stdin.write('release\n'); backend.process.stdin.flush()
            if process.poll() is None: process.kill(); process.wait()
    return subprocess.run(args, env=child, cwd=home, capture_output=True, text=True, timeout=60)
if __name__ == '__main__':
    native_cases(Path(RUBY), consumer='ruby', invoke=invoke)
