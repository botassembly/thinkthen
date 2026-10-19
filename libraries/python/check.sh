#!/bin/sh
# Check the package callers install. Full shared parity waits for a candidate.
unset THINKTHEN_API_KEY
set -eu
profile=${THINKTHEN_TEST_PROFILE:-routine}
case "$profile" in routine|full|smoke) ;; *) echo 'unknown Python test profile' >&2; exit 2 ;; esac
python3 "$(dirname -- "$0")/../../sdlc/scripts/check_verdict.py" platform libraries/python
tools_group=python
. "$(dirname -- "$0")/../../sdlc/scripts/resolved-tools"
cd -- "$(dirname -- "$0")"
repo=$(cd ../.. && pwd)
. "$repo/sdlc/scripts/scratch.sh"
usage_home
if [ -n "${THINKTHEN_PARITY_PREPARE:-}${THINKTHEN_PARITY_PREPARED:-}" ]; then
    [ "$profile" = full ] || exit 2
fi
command -v uv >/dev/null 2>&1 || { echo 'libraries/python: not run: no uv'; exit 1; }
command -v maturin >/dev/null 2>&1 || { echo 'libraries/python: not run: no maturin'; exit 1; }
python3 "$repo/build-tools/results/generate.py" --target python --check
python3 "$repo/build-tools/results/generate.py" --target python --inputs --check
. ./host.sh
host=$(python_host)
[ -n "$host" ] || { echo 'libraries/python: not run: no supported Python'; exit 1; }
python3 "$repo/conformance/children/privacy.py" --sources . --public --artifact-env
if [ -n "${THINKTHEN_COVERAGE_STAGE:-}" ]; then
    [ "$profile" = routine ] || exit 2
    scratch=$THINKTHEN_COVERAGE_STAGE
elif [ -n "${THINKTHEN_PARITY_PREPARED:-}" ]; then
    scratch=$THINKTHEN_PARITY_PREPARED
elif [ -n "${THINKTHEN_PARITY_PREPARE:-}" ]; then
    scratch=$THINKTHEN_PARITY_PREPARE
else
    scratch_dir scratch
fi
if [ -z "${THINKTHEN_PARITY_PREPARED:-}" ] && [ "${THINKTHEN_COVERAGE_PHASE:-prepare}" = prepare ]; then
mkdir -p "$scratch"
if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
    artifact=$(realpath "$THINKTHEN_ARTIFACT")
elif [ "$profile" = full ]; then
    PYO3_PYTHON=$host sh build-wheel.sh
    artifact=$(find "$PWD/target/release-wheel" -name 'thinkthen-*.whl' -print)
else
    build_flags=${RUSTFLAGS:-}
    [ "$profile" = full ] || [ "${THINKTHEN_FULL_COMPILER:-}" = 1 ] || build_flags="${build_flags:+$build_flags }--remap-path-prefix=$HOME=/build"
    RUSTFLAGS="$build_flags" PYO3_PYTHON=$host \
        maturin build --quiet --locked --offline -o "$scratch/wheel"
    artifact=$(find "$scratch/wheel" -name 'thinkthen-*.whl' -print)
fi
uv venv --quiet --offline --python "$host" "$scratch/venv"
uv pip install --quiet --offline --require-hashes --python "$scratch/venv/bin/python" -r requirements-dev.txt
uv pip install --quiet --offline --no-deps --python "$scratch/venv/bin/python" "$artifact"
cp -R tests "$scratch/tests"
fi
[ "${THINKTHEN_COVERAGE_PHASE:-}" != prepare ] || exit 0
venv=$scratch/venv
python=$venv/bin/python
if [ -n "${THINKTHEN_PARITY_PREPARED:-}" ]; then
    scratch_dir runtime
    cp -R "$scratch/tests" "$runtime/tests"
    scratch=$runtime
fi
export PYTHONDONTWRITEBYTECODE=1
cd "$scratch"
unset PYTHONPATH
"$python" - "$venv" <<'PY'
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
value = tt.decide(os.environ['THINKTHEN_TEST_SMOKE_QUESTION'], os.environ['THINKTHEN_TEST_SMOKE_TEXT']).value
assert isinstance(value, bool), value
print(f"smoke: {str(value).lower()}")
PY
    exit
fi
export THINKTHEN_TEST_REPO="$repo"
if [ -z "${THINKTHEN_PARITY_PREPARED:-}" ] && [ "${THINKTHEN_COVERAGE_PHASE:-shared}" = shared ]; then
    CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-$repo/target} "$python" -m pytest -q -p no:cacheprovider --tb=short "$scratch/tests" -m 'not stress'
    "$python" -m mypy --strict tests/native_types.py tests/options_types.py
fi
[ "${THINKTHEN_COVERAGE_PHASE:-}" != shared ] || exit 0
[ -z "${THINKTHEN_PARITY_PREPARE:-}" ] || exit 0
if [ "$profile" = routine ]; then
    "$python" - "$repo" "$scratch/ids" <<'PY'
import sys
from pathlib import Path
root=Path(sys.argv[1]);selected=set((root/'conformance/routine-ids.txt').read_text().splitlines())
selected.update(('library-call-result','complete-decide','images-decide','image-file-decide','images-choose','images-score','cache-hit-cannot-bypass-declaration','settings-replay-answers-from-the-folder-alone','settings-cache-off-sends-again','recognize-context-cache-replay'))
Path(sys.argv[2]).write_text('\n'.join(sorted(selected)))
PY
    THINKTHEN_CONFORMANCE_IDS=$scratch/ids
    export THINKTHEN_CONFORMANCE_IDS
else
    unset THINKTHEN_CONFORMANCE_IDS
fi
"$python" - "$repo" "$scratch/tests/native_case.py" <<'PY'
import os, sys
from pathlib import Path
root=Path(sys.argv[1]);sys.path[:0]=[str(root/'conformance/children')]
from native_fixture import run
selected=os.environ.get('THINKTHEN_PARITY_CONSUMER')
failed=0
for consumer,library in [('python',None),('pandas','pandas'),('python-polars','polars')]:
    if selected is None or selected==consumer:
        failed+=run(consumer,[sys.executable,sys.argv[2]],root,extra_env={'THINKTHEN_FRAME_LIBRARY':library} if library else {})
sys.exit(bool(failed))
PY
