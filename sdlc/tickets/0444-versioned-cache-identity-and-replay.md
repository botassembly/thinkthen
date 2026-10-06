# 0444: Version cache keys and preserve offline replay

Status: ready. Planning only; implementation follows accepted ticket review.

Milestone: 0.2

Owner: builder.
Ticket review: accepted 2026-10-06; blocking findings corrected.

## Outcome

Saved answers carry enough validated identity to rebuild their key without a call. Equivalent endpoint spellings share keys; changing concrete group targets never serves a stale online answer.

## Evidence

- Starts from: Current main c64b71859; 0405 audit is historical and predates 0377/0420. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 10.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Implement 0442’s versioned pure key, canonical posting URL, requested/concrete model identity and validated offline store conversion. Preserve original rows on failed atomic migration. Read-only replay indexes in memory. Unknown/damaged/incomplete records refuse locally before any send.
- Proof: Two processes share keys for trailing-slash/host-case variants; distinct paths differ. Rebuild saved keys offline. Pin valid v1 conversion/idempotence/read-only bytes and tampered/missing-field zero-send refusal. Pinned models cache; group selector returning model1 then model2 goes live again and cannot reuse model1. Multiple historical concrete versions refuse ambiguous replay with zero sends. Keep jev-latest refresh and no key/address leakage.
- Defers: Proxy service/screens, images, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

0442 defines the deliberate storage compatibility change. 0443 supplies cache-control policy and provenance. No automatic repair send, group-to-model guessing, migration command or silent downgrade promise.
