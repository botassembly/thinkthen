# 0417: Rank question sets on every SQL surface

Status: in progress. Complete SQL rank-set results and ordered member identities passed all 248 required cases through each source and installed extension. Final family checks passed. Landing remains.

Milestone: 0.2

Owner: builder.
Ticket review: accepted 2026-10-06; blocking findings corrected.

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

## SQL family build contract (2026-10-06)

Risk: High: keyed originals and authored names cross DuckDB's counted FFI;
all hosts must admit the complete input before transport and retain secrecy,
cancellation and strict replay. No new parser, ordering algorithm, scheduler,
cache or dependency is introduced. Root owns one fresh whole-family review
and landing gates. Ian can overturn the additive SQL spelling below.

Before implementation: `thinkthen_rank_set(questions, keyed_json, settings :=
NULL)` returns `key`, one-based `rank`, selecting `probability`,
`question_name`, and `facts`. DuckDB/SQLite take JSON text and return facts
as JSON text; PostgreSQL takes questions text, keyed jsonb, settings json,
and returns facts jsonb. Facts are serialized directly from the same native
Call, combined across members and repeated on each output row. LIMIT applies
after the native merge, judging every record/member. Empty keyed input returns
no rows. NULL questions/keyed input retain the host's existing rank behavior:
DuckDB no rows; SQLite required-argument Usage; PostgreSQL NULL questions
Usage and NULL input no rows. NULL settings means defaults.

Questions use native `RankSet::from_json`; DuckDB/SQLite retain their existing
explicit @file reader and PostgreSQL retains its existing confined/privileged
@file route or `thinkthen_question_file(path)`. Raw JSON preserves order and
duplicate detection. Settings admit batch, context and deadline_ms only;
model/backend selection uses the existing host engine settings/environment.
Per-call model override needs a native RankSet model/control API, absent in
this baseline; it is refused rather than silently ignored.

0406 dependency: native `Engine::rank_with` currently requires a plain
`Question::rank` (Kind::Rank), while `Question::from_json` produces Decide or
Score. There is no public saved-question-to-rank conversion. Described decide
members already execute through RankSet and get SQL coverage here; independent
single described/saved-score adoption awaits the native owner's rank API.
The SQL family must not sort details locally to replace that missing route.

0435/native 0442/0445/0450 own finalized complete results, observations,
call/answer IDs and started-failure facts. This slice exposes existing native
combined count facts, not those pending carriers. 0456's design is accepted;
native name/declaration/loading implementation is awaited. No local schema or
@NAME parser is added. These dependencies remain open, with no parity claim.

Focused family checks execute the public set route on SQLite 3.50,
PostgreSQL 16.15 and DuckDB v1.5.5/v1.5.4: independent saved exchanges and
turns orders, retained duplicate-text keys, names/order, host-specific ties,
LIMIT, current combined facts, one-member equivalence, individual-member
recording replay from empty caches, rename/reorder replay, strict misses,
described criteria/context/model selection, NULL/empty and invalid admission,
held cancellation and backend-error secrecy. Refusals and replays count zero
sends at owned loopback listeners. Legacy rank and SQLite 0433 regression
checks are retained and executed. This is builder evidence for root's whole
review, not a landing record, complete-result matrix or completion claim.

Slice A code review: ACCEPT, 2026-10-06; fresh read-only High whole-family review of0066b0639. Full tests and lint run on the landing commit. Complete native result and declaration adoption stay open.

## What the build taught us

Recording replay must start with empty caches to prove the recordings supply the answers. The shared native turns merge preserves duplicate keyed records without a SQL-specific sorting implementation.

Complete family adoption is qualified under 0435. Its [single family record](../records/0435-sql-call-facts-and-prices.md) carries the source and installed results, review corrections and remaining integration gates.
