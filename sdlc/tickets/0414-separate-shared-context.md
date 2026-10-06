# 0414: Separate context from aggregate evidence everywhere

Status: ready. Planning only; implementation follows accepted ticket review.

Milestone: 0.2

Owner: builder.
Ticket review: accepted 2026-10-06; blocking findings corrected.

## Outcome

Annotate, find, recognize and relate accept separate shared context across CLI/Rust/C/SDK/SQL/frames without changing evidence identity or offsets.

## Evidence

- Starts from: Existing ticket and 0405 audit; landed 0377/0401D/0420 are the current baseline, replacing the older audit-only matrix. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 4.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Settle the additive context contract and stage packing first; preserve context-free bytes and source metadata rules.
- Proof: Pin candidates/entities/offsets/edges/groups and context-specific cache misses; strict replay sends zero. Context never becomes evidence or shifts spans.
- Defers: Proxy service/screens, images, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

0407 owns per-record context; family/SQL/frame owners expose this shared contract.

## Context delivery acceptance

The native semantic tests assert independently declared request bodies: shared context reaches every applicable stage and every packed/split request of annotate, find, recognize and relate, separately from evidence. A context-only cache-key change cannot pass. Preserve context-free bytes and evidence/offset invariants. Family, SQL and dataframe consumers reuse these expected exchanges through their named methods in 0432.
