# 0407: Supply separate per-record context everywhere

Status: ready. Planning only; implementation follows accepted ticket review.

Milestone: 0.2

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
