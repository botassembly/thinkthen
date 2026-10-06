# 0444: Version cache keys and preserve offline replay

Status: ready. Planning only; implementation follows accepted ticket review.

Milestone: 0.2

Owner: builder.
Ticket review: ACCEPT, 2026-10-06; fresh read-only review. Conflicting image deferrals corrected where found.

## Outcome

Saved answers carry enough validated identity to rebuild their key without a call. Equivalent endpoint spellings share keys; a changed reported model never serves an old online answer.

## Evidence

- Starts from: Current main c64b71859; 0405 audit is historical and predates 0377/0420. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 10.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Implement 0442’s versioned pure key, canonical posting URL, requested/concrete model identity and validated offline store conversion. Preserve original rows on failed atomic migration. Read-only replay indexes in memory. Unknown/damaged/incomplete records refuse locally before any send.
- Proof: Two processes share keys for trailing-slash/host-case variants; distinct paths differ. Rebuild saved keys offline. Pin valid v1 conversion/idempotence/read-only bytes and tampered/missing-field zero-send refusal. Pinned models cache; opaque selector returning model1 then model2 goes live again and cannot reuse model1. Multiple historical concrete versions refuse ambiguous replay with zero sends. Keep jev-latest refresh and no key/address leakage.
- Defers: Proxy service/screens, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

0442 defines the deliberate storage compatibility change. 0443 supplies cache-control policy and provenance. No automatic repair send, selector-to-model guessing, migration command or silent downgrade promise.

## Image and observation identity

0447 image evidence enters canonical shared state: actual media and ordered immutable image bytes under the reviewed serialization version, preserving the v2 key’s framed fields. Paths/labels/timestamps remain excluded. Pin byte/media/order changes, text/image separation, relocation equality and zero-send key rebuild/replay. 0450 observation IDs persist outside request identity; derive validated legacy IDs before rekeying and retain them atomically. Read-only replay changes no bytes. If normalization collapses distinct saved snapshots onto one key, refuse before committing rather than choose one silently.

0449 removes SDK group interpretation; retain conservative requested/reported-model comparison, known mutable-alias refresh and ambiguous offline replay refusal. Do not add discovery, target maps or routing. A mutable selector that echoes itself cannot prove freshness: caller no-cache/refresh or provider no-store is required, and future explicit proxy calls bypass local reuse until an admitted policy-version contract proves safety. Do not claim the key formula alone fixes echoed aliases.

When this change is pushed to main, notify the experiments team through pm with the commit, changed behavior and affected experiment 0035 steps. They rerun only affected steps without waiting for release. Note the notification in this ticket’s single landing record.
