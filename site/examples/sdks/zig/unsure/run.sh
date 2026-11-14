#!/bin/sh
# Reuse the installed Zig prepare/execute launcher with this shown source.
# --prepare builds the retained executable; other invocations run it.
set -eu
root=$(CDPATH= cd "$(dirname "$0")/../../../../.." && pwd)
exec sh "$root/site/examples/lang-zig/run.sh" "$@"
