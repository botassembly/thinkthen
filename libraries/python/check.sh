#!/bin/sh
# Check the package callers install. Full shared parity waits for a candidate.
unset THINKTHEN_API_KEY
set -eu
cd -- "$(dirname -- "$0")"
repo=$(cd ../.. && pwd)
. "$repo/sdlc/scripts/scratch.sh"
usage_home
profile=${THINKTHEN_TEST_PROFILE:-routine}
case "$profile" in routine|full|smoke) ;; *) echo 'unknown Python test profile' >&2; exit 2 ;; esac
command -v uv >/dev/null 2>&1 || { echo 'not run: no uv'; exit 77; }
command -v maturin >/dev/null 2>&1 || { echo 'not run: no maturin'; exit 77; }
. ./host.sh
host=$(python_host)
[ -n "$host" ] || { echo 'not run: no supported Python'; exit 77; }
scratch_dir scratch
if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
    artifact=$(realpath "$THINKTHEN_ARTIFACT")
else
    RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }--remap-path-prefix=$HOME=/build" PYO3_PYTHON=$host \
        maturin build --quiet --locked --offline -o "$scratch/wheel"
    artifact=$(find "$scratch/wheel" -name 'thinkthen-*.whl' -print)
fi
uv venv --quiet --offline --python "$host" "$scratch/venv"
uv pip install --quiet --offline --require-hashes --python "$scratch/venv/bin/python" -r requirements-dev.txt
uv pip install --quiet --offline --no-deps --python "$scratch/venv/bin/python" "$artifact"
cp -R tests "$scratch/tests"
python=$scratch/venv/bin/python
cd "$scratch"
unset PYTHONPATH
"$python" - "$scratch/venv" <<'PY'
import sys, thinkthen
from pathlib import Path
assert Path(thinkthen.__file__).is_relative_to(sys.argv[1])
for old in ('Judge','Call','Stream','Tally','Completion','complete','details'):
    assert not hasattr(thinkthen,old),old
for name in ('judge','stream','complete','_complete','frames'):
    assert not Path(thinkthen.__file__).with_name(name+'.py').exists(),name
PY
if [ "$profile" = smoke ]; then
    smoke_guard
    THINKTHEN_API_KEY=sk-smoke-loopback "$python" - <<'PY'
import os, thinkthen as tt
print(f"smoke: {tt.decide(os.environ['THINKTHEN_TEST_SMOKE_QUESTION'], os.environ['THINKTHEN_TEST_SMOKE_TEXT']).value}")
PY
    exit
fi
export THINKTHEN_TEST_REPO="$repo"
CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-$repo/target} "$python" -m pytest -q -p no:cacheprovider --tb=short "$scratch/tests" -m 'not stress'
"$python" -m mypy --strict tests/native_types.py
if [ "$profile" = routine ]; then
    "$python" - "$repo" "$scratch/ids" <<'PY'
import sys
from pathlib import Path
root=Path(sys.argv[1]);selected=set((root/'conformance/routine-ids.txt').read_text().splitlines())
selected.update(('complete-decide','images-decide','image-file-decide','images-choose','images-score','cache-hit-cannot-bypass-declaration','settings-replay-answers-from-the-folder-alone','settings-cache-off-sends-again','recognize-context-cache-replay'))
Path(sys.argv[2]).write_text('\n'.join(sorted(selected)))
PY
    THINKTHEN_CONFORMANCE_IDS=$scratch/ids
    export THINKTHEN_CONFORMANCE_IDS
else
    unset THINKTHEN_CONFORMANCE_IDS
fi
"$python" - "$repo" "$scratch/tests/native_case.py" <<'PY'
import sys
from pathlib import Path
root=Path(sys.argv[1]);sys.path[:0]=[str(root/'libraries/python/tests'),str(root/'conformance/children')]
from native_fixture import run
failed=run('python',[sys.executable,sys.argv[2]],root)
for consumer,library in [('pandas','pandas'),('python-polars','polars')]:
    failed+=run(consumer,[sys.executable,sys.argv[2]],root,extra_env={'THINKTHEN_FRAME_LIBRARY':library})
sys.exit(bool(failed))
PY
