# The recording page still describes the retired backend marker

Status: Closed on 2026-09-30. Merged into `../2026-09-30-old-batching-files-still-have-live-callers.md`, since 0304 slice 5 rewrites the recording page and `status`.

## The problem

ADR 0111, section 3, removed the backend marker and the folder gate: "This removes per-digest lock files, the folder gate, the backend marker and its admission probe." The address now sits inside every cache key, so a folder pointed at a new address simply misses and resends.

`specification/recording.md` still describes the old rules:

- Line 50: the first write-capable use of a folder binds it to the backend address, and a later mismatch exits 5 naming the resolved endpoint.
- Lines 51 to 54: the marker's schema, its write order and the unmarked-folder rules.
- Line 119: "Prune ignores and preserves `.thinkthen-backend.json`."

The build follows the ADR. It writes no marker. A hand-written marker naming another address does not stop `--cache` from sending. But `thinkthen status --json` still reports `cache.binding` as `unbound`, `matching` or `mismatched`, a field the specification does not list and the ADR retired.

## What should happen

The recording page states the ADR 0111 rule: answers from two addresses never mix because the address is in every key, and a cache at a new address misses and resends. The marker paragraphs and the prune sentence go. `status` drops `cache.binding`, or the specification names it and says what it means now.

## Evidence

Release QA suite, edge E-088, part B of its first edge-specs ticket.
