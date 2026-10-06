# 0407: Supply separate per-record context everywhere

Status: ready. Planning only; implementation follows accepted ticket review.

Milestone: 0.2

Owner: builder.
Ticket review: accepted 2026-10-06; blocking findings corrected.

## Outcome

Eligible record judgments, filter, rank and annotate accept independent per-record context separately from original evidence across CLI, Rust, C, SDKs, keyed SQL batches and frames.

## Evidence

- Starts from: Existing ticket and 0405 audit; landed 0377/0401D/0420 are the current baseline, replacing the older audit-only matrix. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 4.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Settle packing/identity and typed record/context carriers; retain shared context and absence semantics. Whole-set context belongs to 0414.
- Proof: Different/repeated/empty contexts preserve original items; counted cache misses and strict replay preserve identities, pointer disclosure and split behavior.
- Defers: Proxy service/screens, images, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

Coordinate 0413 per-record options and family carrier ownership.

## Record association and identity

Per-record context is associated with the stable input record/key before sorting, filtering, deduplication or splitting. Missing per-record context falls back to shared context; an explicit empty string supplies empty context and suppresses that fallback. Never concatenate contexts implicitly. Null/non-string record contexts are Usage before sends. Keep the original evidence separately recoverable.

Coordinate 0442/0444: the effective context is part of canonical shared state and therefore question identity. Identical evidence/question with different effective contexts cannot deduplicate or hit each other’s cache/replay entry; equal effective pairs may reuse. Tests use distinguishable record/context pairs, including duplicate evidence with different contexts and reordered keyed/frame inputs, and pin independently expected associations/answers through splitting. Count distinct-context misses, equal-context reuse and offline strict replay.

Assert expected wire bodies as well as results: each record’s effective context reaches its judging request separately from evidence, including packed/split requests. Cache identity alone is insufficient proof of context delivery.
