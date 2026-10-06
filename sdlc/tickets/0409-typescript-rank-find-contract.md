# 0409: Match TypeScript rank and find types to runtime

Status: ready. Planning only; implementation follows accepted ticket review.

Milestone: 0.2

Owner: builder.
Ticket review: accepted 2026-10-06; blocking findings corrected.

## Outcome

TypeScript declarations match actual supported rank/find inputs and refuse unsupported combinations before sends.

## Evidence

- Starts from: Existing ticket and 0405 audit; landed 0377/0401D/0420 are the current baseline, replacing the older audit-only matrix. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 4.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Adopt 0406 richer rank contract while retaining literal text; find must not inherit unsupported score/criteria forms.
- Proof: Compile valid/invalid consumers and run matching JS calls; test thresholds, criteria, model/profile, literal @/{ and explicit file variants.
- Defers: Proxy service/screens, images, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

0431 owns located overloads; 0418 owns rank sets.
