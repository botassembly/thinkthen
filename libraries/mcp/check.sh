#!/bin/sh
# Execute the complete required MCP row, without a case selector.
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
[ "$#" -eq 1 ] || { echo 'usage: libraries/mcp/check.sh LOOPBACK_PORT' >&2; exit 2; }
command=${THINKTHEN_COMMAND:-$root/target/debug/thinkthen}
[ -z "${THINKTHEN_ARTIFACT:-}" ] || [ -n "${THINKTHEN_COMMAND:-}" ] || { echo 'mcp: installed mode needs the extracted command' >&2; exit 1; }
[ -f "$command" ] && [ -x "$command" ] || { echo 'mcp: command is missing or not executable' >&2; exit 1; }
python3 "$root/libraries/mcp/test_client.py"
sh "$root/sdlc/scripts/time-limit" 30 python3 "$root/libraries/mcp/installed.py" "$command"
sh "$root/sdlc/scripts/time-limit" 1800 python3 "$root/libraries/mcp/conformance.py" "$1" "$command"
sh "$root/sdlc/scripts/time-limit" 120 python3 "$root/libraries/mcp/test_installed.py" "$command" "$root/target/debug/conformance-backend"
