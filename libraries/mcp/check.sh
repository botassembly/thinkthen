#!/bin/sh
# Execute the complete required MCP row, without a case selector.
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
[ "$#" -eq 1 ] || { echo 'usage: libraries/mcp/check.sh LOOPBACK_PORT' >&2; exit 2; }
python3 "$root/libraries/mcp/test_client.py"
sh "$root/sdlc/scripts/time-limit" 30 python3 "$root/libraries/mcp/installed.py" "$root/target/debug/thinkthen"
sh "$root/sdlc/scripts/time-limit" 1800 python3 "$root/libraries/mcp/conformance.py" "$1" "$root/target/debug/thinkthen"
sh "$root/sdlc/scripts/time-limit" 120 python3 "$root/libraries/mcp/test_installed.py" "$root/target/debug/thinkthen" "$root/target/debug/conformance-backend"
