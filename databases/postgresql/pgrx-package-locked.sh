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
# A copy of the proven lock: a step that rewrites it, or dies mid-write,
# leaves the tree as it found it (surfaces-review-5).
held=$(mktemp "${TMPDIR:-/tmp}/pgrx-lock.XXXXXX")
cp Cargo.lock "$held"
restore() {
  if ! cmp -s Cargo.lock "$held"; then
    cp "$held" Cargo.lock
    echo "pgrx-package-locked: the package step rewrote Cargo.lock; the proven lock is restored" >&2
    rm -f "$held"
    exit 1
  fi
  rm -f "$held"
}
trap restore EXIT
CARGO_NET_OFFLINE=true cargo pgrx package "$@"
