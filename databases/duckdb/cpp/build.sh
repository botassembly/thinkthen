#!/bin/sh
# Build the selected source host and stage its matching C++ extension.
set -eu
HERE=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)
INPUTS_HERE=$HERE/../tools
. "$INPUTS_HERE/inputs.sh"
host_target=$(input_target)
input_select "${THINKTHEN_DUCKDB_VERSION:-}" "$host_target"
export THINKTHEN_DUCKDB_VERSION=$DUCKDB_VERSION
exec python3 "$INPUTS_HERE/source_hosts.py"
