# 0063: Report status and persist numeric usage counts

Date: 2026-09-22

Status: landed

## Result

`thinkthen status` now reports resolved configuration, backend and cache provenance, cache size, key presence, and local usage counts without contacting a backend or changing local state. `status --json` returns the same facts as one closed `thinkthen.status/1` object. Machines without an absolute platform home report unavailable paths and counts.

Successful command runs persist only request, input-token, output-token, and cache-answer counts in private monthly aggregates beside the platform cache. The files contain no requests, answers, addresses, models, prices, or keys. Every HTTP attempt is precharged before it is sent. Valid provider token counts survive later answer refusal. Explicit replay never adds tokens or cache answers. Bookkeeping failure leaves the judgment and exit meaning intact and prints one fixed warning.

The monthly writer uses one stable private lock and atomic replacement. The first lock holder makes the lock durable before publishing the first month. Later writers serialize checked additions across processes. Status takes a shared lock and strictly validates recognized monthly files without repairing them. Cache inspection shares the existing folder gate and allocated-byte rules.

The Rust ceiling rose from 30,856 to 32,464 nonblank lines. The status command and closed serializers, independent platform path and provenance resolution, private counters, durable monthly store, observed adapter result, strict readers, failure seams, concurrency tests, accounting tests, and documentation account for the increase. The change reused the existing backend decoder, request scheduler, recording cache, folder gate, configuration, and failure reporting.

## Review and proof

Independent design review rejected the first draft because append-only rows could leave a corrupt tail, updates became slower as the ledger grew, lock and privacy rules were incomplete, and unavailable output was underspecified. The accepted rewrite uses one atomically replaced aggregate per UTC month, a stable private lock, strict opened-handle checks, closed status shapes, and honest best-effort limits.

Independent code review rejected the first implementation because fully refused answers lost valid token counts, the accepted durability and accounting matrix was incomplete, a test speed workaround weakened filesystem proof and leaked temporary directories, and status used inaccurate words. Remediation preserved validated usage across answer refusal, added focused failure, race, retry, replay, cache, packed-annotation, rollover, overflow, warning, and secrecy proof, removed and cleaned the workaround, and added a status-specific safe error.

Re-review found one first-creation durability race. A second process could overtake the lock creator before the lock name was synced. The final implementation makes whichever process first acquires the lock and sees no monthly state sync the lock and directory before publishing. A deterministic overtaking test proves the ordering. The reviewer accepted the result with no remaining material finding.

The final install, lint, test, and specification rungs passed with the key and outside address variables unset. The lint rung measured the exact 32,464-line ratchet. The specification rung passed all 19 green demos. `git diff --check` passed. No paid or outside request ran.
