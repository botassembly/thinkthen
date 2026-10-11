"""Replay the seeded decision through the installed generated COBOL caller."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

REPO = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(REPO / 'conformance/children'))
from children import child_env

package = Path(os.environ['THINKTHEN_SESSION_PACKAGE'])
with tempfile.TemporaryDirectory(prefix='cobol-smoke-') as temporary:
    root = Path(temporary)
    for directory in ('src', 'copybooks', 'examples'):
        shutil.copytree(package / directory, root / directory)
    shutil.copy(os.environ['THINKTHEN_C_HEADER'], root / 'src/thinkthen.h')
    shutil.copy(os.environ['THINKTHEN_C_LIBRARY'], root / 'libthinkthen.so.0')
    question = root / 'question.json'
    question.write_text(json.dumps({'decide': os.environ['THINKTHEN_TEST_SMOKE_QUESTION']}))
    env = child_env(home=root / 'home', **{name: os.environ[name] for name in
        ('THINKTHEN_BASE_URL', 'THINKTHEN_API_KEY', 'THINKTHEN_CACHE')},
        TT_SESSION_QUESTION_FILE=str(question), TT_SESSION_EVIDENCE=os.environ['THINKTHEN_TEST_SMOKE_TEXT'],
        TT_SESSION_EXPECT_FAILURE='N')
    subprocess.run(['cobc', '-x', '-free', '-fstatic-call', '-fno-gen-c-decl-static-call',
        '-I', str(root / 'copybooks'), '-A', '-include ' + str(root / 'src/tt_session.h') + ' -Wno-incompatible-pointer-types',
        '-o', str(root / 'consumer'), str(root / 'examples/session.cob'), str(root / 'src/tt_session.c'),
        str(root / 'src/tt_requests_generated.c'), '-Q', str(root / 'libthinkthen.so.0') + ' -Wl,-rpath,$ORIGIN'],
        env=env, check=True, timeout=30)
    result = subprocess.run([str(root / 'consumer')], env=env, capture_output=True, text=True, timeout=10)
    assert result.returncode == 0 and 'SESSION_PASS' in result.stdout, (result.stdout, result.stderr)
    values = {'VALUE 0000000001': 'true', 'VALUE 0000000000': 'false', 'VALUE_NULL': 'null'}
    answer = next(values[line] for line in result.stdout.splitlines() if line in values)
    print('smoke: ' + answer)
