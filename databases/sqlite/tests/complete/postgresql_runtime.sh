#!/usr/bin/env bash
# Install the existing packaged extension into an owned pinned runtime, then run SQL cases.
set -euo pipefail
here=$(cd -- "$(dirname -- "$0")/../../../postgresql" && pwd)
cd -- "$here"
. ../../sdlc/scripts/scratch.sh
PG_MAJOR=16
. ./runtime.sh
runtime_ready
runtime_target
REPO=$(cd ../.. && pwd)
LIMIT=$REPO/sdlc/scripts/time-limit
EXT_VERSION=$(sed -n "s/^default_version = '\(.*\)'$/\1/p" thinkthen.control)
runtime_open
cleanup() { pg_stop; runtime_remove; scratch_clean; rm -f .runtime/last-run; }
trap cleanup EXIT INT TERM
PG_CONFIG=${PG_CONFIG:-/usr/bin/pg_config}
EXT=${CARGO_TARGET_DIR:-target}/release/thinkthen-pg16
runtime_install "$EXT$("$PG_CONFIG" --pkglibdir)" "$EXT$("$PG_CONFIG" --sharedir)/extension"
export THINKTHEN_POSTGRESQL_BIN=$BIN THINKTHEN_POSTGRESQL_DATA=$DATA THINKTHEN_POSTGRESQL_SOCKET=$SOCK
[ "$#" -eq 0 ] || unset THINKTHEN_CONFORMANCE_IDS
python3 ../sqlite/tests/complete/parity.py postgresql "$@"
python3 ../sqlite/tests/complete/facts.py postgresql
