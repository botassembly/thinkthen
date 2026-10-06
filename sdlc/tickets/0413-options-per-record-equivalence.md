# 0413: Supply per-record candidate options everywhere

Status: ready. Planning only; implementation follows accepted ticket review.

Milestone: 0.2

Owner: builder.
Ticket review: accepted 2026-10-06; blocking findings corrected.

## Outcome

Choose batches across every SDK, keyed SQL and frames carry independent ordered candidates and descriptions for each original record.

## Evidence

- Starts from: Existing ticket and 0405 audit; landed 0377/0401D/0420 are the current baseline, replacing the older audit-only matrix. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 4.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Define identity/order/description/refusal semantics before code; reuse CLI options projection and preserve fixed-question calls. Dynamic tag/annotate options remain their separately scoped issue.
- Proof: Two different shortlists, descriptions, duplicate/missing labels, missing pointer, none and excluded candidates pin wire bytes, originals and zero-send invalid/replay cases.
- Defers: Proxy service/screens, images, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

0407 owns per-record context; family tickets adopt shared carriers.
