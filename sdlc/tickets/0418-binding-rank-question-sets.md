# 0418: Rank question sets through typed language and frame APIs

Status: ready. Planning only; implementation follows accepted ticket review.

Milestone: 0.2

## Outcome

C, every SDK and supported dataframe variant exposes saved decide question-set rank with the landed shared turns merge and full member/final facts.

## Evidence

- Starts from: Existing ticket and 0405 audit; landed 0377/0401D/0420 are the current baseline, replacing the older audit-only matrix. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 4.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Define additive typed input/output; retain original indexes/member names and single-question behavior. Remove the old foreign-later deferral.
- Proof: Same independent turns cases as 0417 through each named public method; stable ties, duplicates, one-member equivalence, invalid inputs and zero-send replay.
- Defers: Proxy service/screens, images, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

0401D is landed; 0426–0431 own host carriers; 0410 owns frames; 0432 enforces full coverage.
