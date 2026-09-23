#!/usr/bin/env bash
# The Python surface's check: build the wheel into the folder-local venv,
# run the surface's own tests and its slice of the conformance file
# offline, then the slide sample and the cancel proof when the stub is up.
# The port this surface owns is 8211.
# Experimental on macOS: the library spellings below take the Darwin forms
# (dylib, DYLD_LIBRARY_PATH); Linux is the gate's platform.
set -euo pipefail
cd "$(dirname "$0")"

case "$(uname -s)" in
  Darwin) LIB_EXT=dylib; LIB_PATH_VAR=DYLD_LIBRARY_PATH ;;
  *)      LIB_EXT=so;    LIB_PATH_VAR=LD_LIBRARY_PATH ;;
esac

stub_url="http://127.0.0.1:8211/v1"
wire=no
skip_note='wire suites skipped: no stub on 8211'
if curl -sf --max-time 1 "$stub_url/stats" >/dev/null 2>&1; then
  wire=yes
fi

if [ ! -x .venv/bin/python ]; then
  uv venv .venv
  # Pinned through requirements-dev.txt; the first provision needs the
  # network (or uv's cache), every later run is offline.
  uv pip install --python .venv/bin/python -r requirements-dev.txt
fi
# maturin develop installs into the active virtualenv. The wheel builds
# with the stand-in's compile-time `synthetic-partial` feature so the
# annotate partial-failure fixture (finding 7) is present for the cases
# that pin its marker; a shipped wheel is a default build, and the
# stand-in's own default-build test proves the door is compile-time only
# (no environment variable arms it).
#
# Cargo embeds absolute source paths (the workspace, the registry) in the
# extension's panic locations, so a shipped .so would otherwise name the
# builder's home directory 165 times. Remap every path under $HOME to a
# neutral prefix at build time; the two checks after the build prove the
# remap took, and the same remap rides in build-wheel.sh for wheels.
export RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }--remap-path-prefix=$HOME=/build"
source .venv/bin/activate
maturin develop --locked --release --features synthetic-partial

echo "== the built extensions carry no builder home paths"
for artifact in target/release/lib_thinkthen.$LIB_EXT thinkthen/_thinkthen.abi3.so; do
  if strings "$artifact" | grep -qF -- "$HOME"; then
    echo "the remap did not take: $artifact still carries $HOME" >&2
    exit 1
  fi
done
echo "clean: neither built extension carries $HOME"

echo "== the defect kind maps to the host's error (shim unit test)"
# The unit tests link libpython and construct a contract Error with kind
# `defect`; the wheel build keeps `extension-module` through the crate's
# default feature, so this step builds with --no-default-features.
LIBDIR=$(.venv/bin/python -c 'import sysconfig; print(sysconfig.get_config_var("LIBDIR"))')
BASEP=$(.venv/bin/python -c 'import sys; print(sys.base_prefix)')
PYO3_PYTHON="$PWD/.venv/bin/python" \
  env "$LIB_PATH_VAR=${LIBDIR}${!LIB_PATH_VAR:+:${!LIB_PATH_VAR}}" \
  PYTHONHOME="$BASEP" \
  cargo test --quiet --no-default-features --lib --locked

echo "== surface tests, null backend"
ENGINE_NULL=1 .venv/bin/python -m pytest tests/test_surface.py -q

echo "== the Polars door, null backend"
ENGINE_NULL=1 .venv/bin/python -m pytest tests/test_polars_door.py -q

echo "== the pandas checks, null backend"
ENGINE_NULL=1 .venv/bin/python -m pytest tests/test_pandas_checks.py -q

echo "== one deadline for a whole column (this file's own loopback server)"
.venv/bin/python -m pytest tests/test_deadline_column.py -q

echo "== the second review's findings, offline"
ENGINE_NULL=1 .venv/bin/python -m pytest tests/test_review2_findings.py -q

echo "== the second review's signal children"
ENGINE_NULL=1 .venv/bin/python -m pytest tests/test_review2_signals.py -q

echo "== the second review's wire findings (this file's own counting stub)"
.venv/bin/python -m pytest tests/test_review2_wire.py -q

echo "== the third review's offline findings, null backend"
ENGINE_NULL=1 .venv/bin/python -m pytest tests/test_review3_offline.py -q

echo "== the third review's wire findings (own loopback server)"
.venv/bin/python -m pytest tests/test_review3_wire.py -q

echo "== the fourth review's signal findings (own counting stub)"
.venv/bin/python -m pytest tests/test_review4_signals.py -q

echo "== recognize and relate, null backend"
ENGINE_NULL=1 .venv/bin/python -m pytest tests/test_recognize_relate.py -q

echo "== canonical results and ownership (punch-list item 3)"
ENGINE_NULL=1 .venv/bin/python -m pytest tests/test_ownership.py -q

echo "== cancel on a fast backend"
ENGINE_NULL=1 .venv/bin/python tests/test_cancel_fast.py

echo "== recognize and relate at the scale the recordings carry"
ENGINE_NULL=1 .venv/bin/python tests/bench_recognize_scale.py

echo "== per-record cost through both containers, null backend"
ENGINE_NULL=1 .venv/bin/python tests/bench_cost_polars.py
ENGINE_NULL=1 .venv/bin/python tests/bench_cost_pandas.py

echo "== the function examples, run as one test"
ENGINE_NULL=1 .venv/bin/python tests/examples.py

echo "== conformance slice, offline"
THINKTHEN_NULL=1 .venv/bin/python tests/conformance.py

echo "== slide sample, as drawn, on the stand-in's offline backend"
# The loopback stub answers one noul shape a request, so the choose, score,
# and annotate wire shapes cannot be served by it; job 3 marked those five
# exchanges shaped-to-contract for the same reason. The sample runs as
# drawn against the stand-in offline, and the wire section below runs what
# the stub can serve.
scratch=$(mktemp -d)
cp tests/fixture/form.json "$scratch/form.json"
cd "$scratch"
ENGINE_NULL=1 "$OLDPWD/.venv/bin/python" "$OLDPWD/tests/slide_sample.py"
cd - >/dev/null
rm -rf "$scratch"

if [ "$wire" = yes ]; then
  echo "== cancel on the wire"
  THINKTHEN_BASE_URL="$stub_url" ENGINE_WIDTH=8 \
    .venv/bin/python tests/test_cancel.py

  # The width benches hold the equality proof (same gate, same request
  # count, same in-flight high-water mark through every container). The
  # gate runs them at a 200-record fixture to keep the stub window short;
  # the recorded 1,000-record runs are the manual's numbers.
  echo "== width equality on the wire, small fixture"
  BENCH_RECORDS=200 ENGINE_BASE_URL="$stub_url" ENGINE_WIDTH=32 \
    .venv/bin/python tests/bench_width_polars.py
  BENCH_RECORDS=200 ENGINE_BASE_URL="$stub_url" ENGINE_WIDTH=32 \
    .venv/bin/python tests/bench_width_pandas.py
else
  echo "== width equality on the wire, small fixture"
  echo "$skip_note"
fi
