#!/usr/bin/env bash
# cargo-pgrx 0.17 carries no --locked, and CARGO_NET_OFFLINE is not the
# lock: offline cargo still rewrites a drifted Cargo.lock from the local
# cache (surfaces-review-5). This wrapper is the one place `cargo pgrx
# package` runs: it refuses a lock that would change before the step, and
# fails if the step changed it after. Run from the crate's folder; the
# arguments pass through to `cargo pgrx package`.
# Usage: scripts/pgrx-package-locked.sh --pg-config PATH [--features F]
set -euo pipefail
cargo metadata --locked --offline --format-version 1 >/dev/null
before=$(cksum < Cargo.lock)
CARGO_NET_OFFLINE=true cargo pgrx package "$@"
after=$(cksum < Cargo.lock)
if [ "$before" != "$after" ]; then
  echo "pgrx-package-locked: the package step rewrote Cargo.lock" >&2
  exit 1
fi
