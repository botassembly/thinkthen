# 0417: Rank question sets on every SQL surface

Status: ready. Planning only; implementation follows accepted ticket review.

Milestone: 0.2

## Outcome

DuckDB, SQLite and PostgreSQL rank the same saved decide question set using landed 0401D’s turns merge, preserving keys, member names, combined facts and stable ties.

## Evidence

- Starts from: Existing ticket and 0405 audit; landed 0377/0401D/0420 are the current baseline, replacing the older audit-only matrix. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 4.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Add an explicit question-set route preserving existing single-question rows; one-member sets match ordinary rank. Use reviewed member/host-key ordering.
- Proof: Independent turns fixtures, duplicates, ties, top/LIMIT, one-member equivalence, per-member replay zero sends and invalid-kind/cut/on refusals.
- Defers: Proxy service/screens, images, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

0401D is landed; 0406 owns single-question richer rank; 0434 owns explicit PG question files; 0435 owns invocation facts.
