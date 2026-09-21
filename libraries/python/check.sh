#!/usr/bin/env bash
# The Python surface's check: build the wheel into the folder-local venv,
# run the surface's own tests and its slice of the conformance file
# offline, then the slide sample and the cancel proof when the stub is up.
# The port this surface owns is 8211.
set -euo pipefail
cd "$(dirname "$0")"

stub_url="http://127.0.0.1:8211/v1"
wire=no
if curl -sf --max-time 1 "$stub_url/stats" >/dev/null 2>&1; then
  wire=yes
fi

if [ ! -x .venv/bin/python ]; then
  uv venv .venv
  uv pip install --python .venv/bin/python pytest pandas
fi
# maturin develop installs into the active virtualenv
source .venv/bin/activate
maturin develop --release

echo "== surface tests, null backend"
ENGINE_NULL=1 .venv/bin/python -m pytest tests/test_surface.py -q

echo "== conformance slice, offline"
ENGINE_NULL=1 .venv/bin/python tests/conformance.py

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
fi
