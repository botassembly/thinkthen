#!/bin/sh
# Qualify the supplied installed command or build this checkout's public command.
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
if [ "$#" -eq 1 ]; then
    cargo build --locked --offline --manifest-path "$root/Cargo.toml" --package thinkthen-cli --bin thinkthen
    set -- "$1" "${CARGO_TARGET_DIR:-$root/target}/debug/thinkthen"
fi
exec python3 "$root/conformance/cli_parity.py" "$@"
