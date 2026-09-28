# Ticket 0238: Refresh cached answers for a mutable model alias and refuse mixed-version runs

Status: **Proposed design, awaiting fresh review.** Severity 2, public 0.1 correctness. This joins remaining register 09 and the cache portion of register 23. ADR [0100](../planning/adr/0100-cache-model-freshness.md) gives the proposed policy; [preflight](../records/0238-cache-model-preflight.md) pins the current source. No runtime work or issue closure is authorized by this draft alone.

## Outcome

A user who selects `jev-latest` through the command line, question file or configuration with an answer cache receives an actual answer to every planned exchange and a visible cost warning before the first possible send. Complete new answers replace held cache answers atomically; a refresh that fails before rename preserves the old complete entry without serving it in that run. A user can explicitly refresh a pinned-model cache with `--refresh-cache`. Plain pinned cache hits still save requests. Strict `--replay` stays offline history. A refreshed `annotate` question set can combine old held groups and a newly added group without an old-cache/new-live artifact; genuinely different live versions still stop. `filter` and `rank` refuse before using a second model version in one run.

## Evidence

- Starts from: Experiment 273 review 08 item 1, review 03 finding 2-5, experiment 284 findings 09 and 23, issue [architect-review-08-cache](../issues/2026-09-26-architect-review-08-cache.md), issue [mixed-model-cache](../issues/2026-09-26-a-mixed-model-cache-fails-every-annotate-record.md), and the current-source [cache refresh](../records/2026-09-28-cache-correctness-refresh.md) at main `2b388453`.
- Keeps: The pinned default, exact adapter/URL/request digest, version-one entry and marker formats, strict offline replay, immutable explicit recording, complete-answer cache rule, existing per-record mixed-model refusal, bounded send controls, and actual-attempt accounting.
- Changes: Explicit `jev-latest` cache use refreshes each planned exchange; `--refresh-cache` makes that action available for another cached model; valid cache entries can be atomically replaced only on that cache-refresh path; record runs check one model version across answered batches before using a later batch.
- Proof: One loopback cache case seeds old-version alias entries, refreshes an unchanged group plus an edited group, checks actual request bodies/counts, new-version output, complete replacement and offline replay. One failed-refresh branch compares exact old bytes. A small two-batch listener table checks `filter` prefix, `rank` empty output and genuine mixed-live `annotate` refusal. The real pinned old reader reads a refreshed version-one entry in a temporary folder; no provider or production cache is used.
- Defers: Register 105's recording vintage/cadence, register 40's named writable-folder trust warning, unknown alias names without an explicit refresh request, library and SQL force-refresh knobs, and changes to tuned-profile calibration. Old installed binaries cannot acquire the new refresh policy.

## Contract and boundaries

Implement ADR 0100 exactly. The `--refresh-cache` option applies to the answer cache selected by `--cache DIR` or the platform default; reject it with `--record`, `--replay`, or `--no-cache` before touching storage or sending. A dry run sends and replaces nothing. For the recognized mutable `jev-latest` alias, refresh is automatic only in write-capable cache mode. It does not affect `--record`, `--replay`, `--no-cache`, or a pinned cache. The warning is fixed text emitted once before the first possible send, even on a cold alias cache; no user evidence, backend URL, key, response body, untrusted model string or folder content enters it. A cache hit under the alias is not counted as a replayed answer. Every retry or already dispatched sibling remains part of actual usage.

Take the per-digest lock before reading or replacing a valid entry. Preserve valid bytes on key, cancellation, deadline, transport, parse, model and incomplete-answer errors before rename; after a rename but before directory sync, the durable old-or-new complete result is uncertain. Install only a complete decoded response. Do not delete an existing valid entry first. Explicit recordings retain the ADR 0020 compare-or-conflict rule. No entry, marker or request schema changes; no directory-wide migration. The current marker cannot accept new fields under its version-one schema because installed old readers deny unknown members.

At the ordered `filter`/`rank` output boundary, compare one answer model per completed batch before accepting any of that batch's rows. Use the first answered batch in input order as the anchor. Check filtered-out batches too. Stop at exit 4 on a mismatch, with a fixed safe message and pin/rerun advice. No mismatched batch row is printed or ranked. Preserve previously printed `filter` rows; `rank` emits no ranking on refusal. Do not undo sends or make optimistic usage counts. Keep the existing per-record annotate check for true mixed-live group versions.

| Boundary | Expected result |
| --- | --- |
| Pinned default cache hit | Replay complete held answer; zero sends; no refresh warning. |
| Explicit `jev-latest` cache, old held answer | Warn once, send the requested exchange, replace complete entry and use live reported model. |
| Explicit `jev-latest` cache, failed send or incomplete reply | Keep old complete entry; return the existing failure, with no stale fallback. |
| Explicit `--refresh-cache` on pinned cache | Warn once and send/replace actual planned exchanges; no extra probe. |
| Exact `--replay` of an alias entry | Return historical answer offline, no key or freshness claim and no write. |
| Old cached annotate groups plus new group, all live replies same version | Succeed under refresh; all planned groups sent. |
| Live annotate groups report different versions | Retain exit 4 refusal; no row for that record. |
| Second batch reports another version during `filter` or `rank` | Exit 4 before using second batch; filter retains printed prefix; rank prints no ranking. |

## Prospective source, tests and checks

Claim after design review: `crates/thinkthen/src/engine/{recorder,request,facade}.rs` and a coherent private recorder child if needed for replacement; `crates/thinkthen/src/core/adapters/systemone.rs` for the known alias predicate; `crates/thinkthen/src/cli/{args,asking,schedule,failure}.rs`, `cli/asking/{folders,row}.rs` and the exact small children carrying cache selection and batched rows; and `specification/{recording,result,settings}.md` plus focused CLI help. The shared CLI cache-option carriers for annotate, find, recognize and relate must all consume or reject the flag consistently. Inspect the library and SQL constructors when threading the shared alias rule; do not claim their option surfaces for the CLI force-refresh flag without a separate contract. Preserve existing cache-lock, identity, temporary-write and completeness helpers. No broad facade or public-result refactor is intended.

Extend `crates/thinkthen/tests/backend/annotate/cache_versions.rs` and the nearest `crates/thinkthen/tests/backend/{cache_identity,recordings}.rs` listener case for the old-hit/new-live and failed-replace proof. Add one focused stream listener table in `crates/thinkthen/tests/backend/keeping.rs` or its focused child rather than a new matrix. Use a temporary HOME/XDG and a real pinned old executable or reader against the same version-one refreshed file. Verify exact old and new entry bytes, request counts, model and stdout/exit; include a duplicate-live model case so removing the existing safety check cannot pass. Run focused tests, format, strict lint and affected page checks; no provider call or production-cache mutation.

## Routing

Fresh design reviewer returns ACCEPT or concrete findings before source claims or implementation. The coordinator then assigns the accepted source slice. Fresh code review, focused verification, record, landing and push remain required. The original issues stay open until their complete relevant criteria are verified; register 105 and 40 retain separate owners and outcomes.
