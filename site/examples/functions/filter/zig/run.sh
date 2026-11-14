#!/bin/sh
# Reuse the installed zig prepare/execute launcher with this cell's shown source.
# --prepare builds the retained executable; other invocations run it.
set -eu
root=$(CDPATH= cd "$(dirname "$0")/../../../../.." && pwd)
exec sh "$root/site/examples/lang-zig/run.sh" "$@"
