#!/usr/bin/env bash
# The DuckDB surface's check: the build, the null suite, the conformance
# slice, the slide as drawn, and the wire suite when the stub is up on
# this surface's port (8217). The wire stub is the in-repo
# tools/wire-stub, which scripts/check_surfaces.sh builds and starts.
set -euo pipefail
cd "$(dirname "$0")"

# The one DuckDB version pin (tools/version.env).
source "$(dirname "$0")/tools/version.env"

# Experimental on macOS: the library spelling below takes the Darwin form
# (dylib); Linux is the gate's platform.
case "$(uname -s)" in
  Darwin) LIB_EXT=dylib ;;
  *)      LIB_EXT=so ;;
esac

echo "== duckdb surface: build the extension"
make release >/dev/null
test -s build/release/thinkthen.duckdb_extension

echo "== duckdb surface: the fixture-armed extension for the failed-marker case"
# The stand-in's partial-failure opt-in is a compile-time door, so the one
# conformance case that replays the failed marker (74) runs against a
# second build that carries it, exactly the way the C surface builds its
# door test. The default extension stays the one every other step loads.
DUCKDB_EXTENSION_NAME=thinkthen DUCKDB_EXTENSION_MIN_DUCKDB_VERSION="$DUCKDB_VERSION" \
  cargo build --locked --release --quiet --features synthetic-partial
./configure/venv/bin/python extension-ci-tools/scripts/append_extension_metadata.py \
  -o build/fixture/thinkthen.duckdb_extension \
  -l target/release/libthinkthen.$LIB_EXT \
  -n thinkthen -dv "$DUCKDB_VERSION" \
  -evf configure/extension_version.txt -pf configure/platform.txt --abi-type C_STRUCT_UNSTABLE >/dev/null
test -s build/fixture/thinkthen.duckdb_extension

export ENGINE_FIXTURE_EXTENSION="$PWD/build/fixture/thinkthen.duckdb_extension"

echo "== duckdb surface: the error-mapping test"
cargo test --locked --release --quiet --lib

echo "== duckdb surface: null suite"
tools/null_suite.sh

echo "== duckdb surface: the row-mapping suite"
tools/mapping_suite.sh

echo "== duckdb surface: the security and boundary suite"
tools/security_suite.sh

echo "== duckdb surface: the relate guard suite"
tools/relate_guard_suite.sh

echo "== duckdb surface: the @file open count"
tools/atfile_suite.sh

echo "== duckdb surface: the stand-in is named only at the connector line"
standin_extra=$(grep -rn "thinkthen_standin" src/*.rs | grep -v "use thinkthen_standin::StandinConnector;" || true)
if [[ -n "$standin_extra" ]]; then
  echo "FAILED   stand-in references beyond the connector line:"
  printf '%s\n' "$standin_extra"
  exit 1
fi
echo "ok       the only stand-in reference is the connector import"

echo "== duckdb surface: the conformance driver can fail"
tools/conformance_selftest.sh

echo "== duckdb surface: one interrupt, one query (the re-arm proof)"
./configure/venv/bin/python tools/rearm_suite.py

echo "== duckdb surface: two databases, each on its own connection"
./configure/venv/bin/python tools/two_databases.py

echo "== duckdb surface: the third review's probes"
./configure/venv/bin/python tools/review3_duckdb.py

echo "== duckdb surface: the panic guards contain a callback panic"
tools/panic_suite.sh

echo "== duckdb surface: fast-backend cancel, the poll-bug shape"
tools/cancel_fast.sh

echo "== duckdb surface: the host's SIGINT coexistence, the job-2 shape"
# Punch-list item 5's proof: a host handler installed after LOAD fires and
# the extension keeps answering; a handler installed before LOAD is chained
# to and a running query stops. Runs through the build's own venv Python
# (duckdb 1.5.5, the CLI's version) on the null backend.
./configure/venv/bin/python tools/host_signal.py

echo "== duckdb surface: the function examples"
python3 tools/examples.py

echo "== duckdb surface: the fixture-armed extension for the failed-marker case"
# The stand-in's partial-failure opt-in is a compile-time door, so the one
# conformance case that replays the failed marker (74) runs against a
# second build that carries it, exactly the way the C surface builds its
# door test. The default extension stays the one every other step loads.
DUCKDB_EXTENSION_NAME=thinkthen DUCKDB_EXTENSION_MIN_DUCKDB_VERSION="$DUCKDB_VERSION" \
  cargo build --locked --release --quiet --features synthetic-partial
./configure/venv/bin/python extension-ci-tools/scripts/append_extension_metadata.py \
  -l target/release/libthinkthen.$LIB_EXT \
  -o build/fixture/thinkthen.duckdb_extension \
  -n thinkthen -dv "$DUCKDB_VERSION" \
  -evf configure/extension_version.txt -pf configure/platform.txt --abi-type C_STRUCT_UNSTABLE >/dev/null
test -s build/fixture/thinkthen.duckdb_extension

echo "== duckdb surface: conformance slice"
python3 tools/conformance.py

echo "== duckdb surface: slide sample, as drawn"
ENGINE_NULL=1 tools/run_slide.sh >/dev/null
echo "ok       the slide runs as drawn (output above its run in NOTES)"

echo "== duckdb surface: recognize and relate acceptance, calls as drawn"
ENGINE_NULL=1 tools/run_recognize.sh >/dev/null
if grep -q "Binder Error: Table function cannot contain subqueries" tools/recognize-run/run.log \
  && grep -q "thinkthen usage: relate takes at most 255 records" tools/recognize-run/run.log \
  && [ "$(grep -c 'based_in │ Northwind Freight │ Denver' tools/recognize-run/run.log)" -ge 1 ] \
  && grep -q "│ Northwind Freight │ Dana    │        2 │" tools/recognize-run/run.log \
  && [ "$(grep -c 'usage after the join │ requests      │     0' tools/recognize-run/run.log)" -ge 1 ]; then
    echo "ok       the recognize call runs as drawn; the relate subquery line is the pinned divergence"
    echo "ok       the working replacements ran with their evidence: the edges, the relations row, the mentions join, and the join's zero requests"
    echo "ok       the 255-record refusal is in the log"
else
    echo "FAILED   the recognize acceptance's pinned lines did not appear; see tools/recognize-run/run.log"
    exit 1
fi

if curl -sf --max-time 1 http://127.0.0.1:8217/v1/stats >/dev/null 2>&1; then
  echo "== duckdb surface: wire suite against the stub on 8217"
  ENGINE_BASE_URL=http://127.0.0.1:8217/v1 tools/wire_suite.sh
else
  echo "== duckdb surface: wire suite skipped, no stub on 8217"
fi
