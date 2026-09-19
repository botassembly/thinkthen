---
flow: build
priority: 40
opens: crates specification/recording.md Cargo.toml Cargo.lock sdlc/ratchet.json sdlc/scripts/policy.py sdlc/scripts/spec demos
---

# 0004: Record and replay backend exchanges

Status: ready

## Outcome

`--record DIR` and `--replay DIR` work on `decide if` as `specification/recording.md` describes. The `spec` rung runs every demo marked green. Demo 01 is recorded once against the live backend and turns green.

## Current Facts

Ticket 0003 delivers `decide if` end to end. ADR 0005 makes demos replay recordings. `demos/01-refund-gate` uses only `decide if` and is red. The live testing cap is in `sdlc/planning/plan.md`. The core is pure, so the digest is computed there and the files are touched only by the binary.

## Scope

- The entry type and its digest in the core. The digest is SHA-256 through the `sha2` crate, over the adapter name, the URL, and the request bytes, as the page says.
- Reading and writing entries in the binary, with the temporary name and the rename.
- The three modes: record, replay, and both as a cache. Two different folders is a usage error. Replay opens no connection and reads no key.
- `meta.replayed` in the result.
- `sdlc/scripts/spec` runs `spec/` and then each `demos/NN-name/README.md` whose status line reads `Status: green`.
- Two small fixes from the review and the live call of ticket 0003, each with its test first: a profile named only by `THINKTHEN_BACKEND` yields to an ad-hoc backend given by flags, and a failure message adds the fixed phrase `backends.md` lists for its status. The first doc lines of `edge.rs` and `failure.rs` lose their "and".
- Dependency added: `sha2`. Update `policy.py` in the same commit.

Excluded: a token ledger, a cache eviction rule, recording of failures, and every other verb.

## Acceptance

- A unit test pins the digest of the `if-urgent` fixture request to a known value, so the file name never drifts.
- Integration tests cover: record then replay gives the same result with `replayed` true, replay with a closed port as the URL succeeds, a replay miss exits 5 and names the entry, cache mode calls the listener once across two runs, two folders exit 2, and no entry ever holds the bearer header.
- The steering agent records demo 01 live, under the cap, and commits the recording. Demo 01 then passes in the `spec` rung with no network. This bullet waits on vendor credits and does not block landing. Until then a test proves the demo runner with a small green demo page of its own under `crates/thinkthen/tests/`.
- The whole ladder is green, and a second agent reviews the dependency addition.
