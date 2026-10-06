# 0406: Preserve rank criteria and score ordering on every surface

Status: ready. Planning only; implementation follows accepted ticket review.

Milestone: 0.2

Owner: builder.
Ticket review: accepted 2026-10-06; blocking findings corrected.

## Outcome

CLI, Rust, C, every language, all SQL and dataframe variants rank over the same described decide criteria or saved score question while retaining stable order and input identity.

## Evidence

- Starts from: Existing ticket and 0405 audit; landed 0377/0401D/0420 are the current baseline, replacing the older audit-only matrix. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 4.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Use the existing score/criteria grammar and shared rank implementation; define any additive typed carrier before code. Single-question equivalence stays here; question sets belong to 0417/0418.
- Proof: Saved described decide/score cases pin weighted ordering, ties, ordinary bytes, model override, invalid cuts and zero-send replay.
- Defers: Proxy service/screens, images, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

0426–0431 own foreign carriers; 0409 owns TypeScript declarations.
