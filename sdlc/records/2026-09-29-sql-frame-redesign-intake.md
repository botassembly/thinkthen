# Accepted SQL and DataFrame redesign intake

Ian handed off workspace experiment2038 and asked Codex to number, prepare and prioritize its sixteen ticket drafts. The source contract is `design/2038-sql-dataframe-interface` at `f9579c5cd7744cbd3b528f6646510d290a36be1b`. Its two ADRs were already accepted by Ian after ten review rounds. The experiment's `HANDOFF.md` maps the evidence; `questions.md` and `review.md` retain the ruling and review trail. This record imports decisions, not prototype code or new runtime proof.

## Number reconciliation

Main already uses [ADR0106](../planning/adr/0106-typed-wrapper-call-facts.md) for owned typed wrapper facts. Preserve that decision. The incoming functional/lazy ADR0106 is imported as [ADR0107](../planning/adr/0107-functional-and-lazy-interfaces-in-0-1.md), and references between the two incoming ADRs use0107. [ADR0105](../planning/adr/0105-one-call-shape-for-sql-and-data-frames.md) retains its number. One stale sentence asking for review before acceptance was removed from0105's accepted status paragraph. Every other source byte is retained. These mechanical corrections reopen no ruling.

| Draft | Ticket | Initial ownership |
| --- | --- | --- |
| T1 | 0283 | Core settings parser and command plan foundation; first build |
| T2 | 0284 | SQLite keyed forms and settings; database lane after0283 |
| T3 | 0285 | PostgreSQL named and keyed forms; separate database lane |
| T4 | 0286 | DuckDB macro/settings/keyed forms; separate database lane |
| T5 | 0287 | Python keywords and probability; Python lane |
| T6 | 0288 | R keywords and probability; R lane |
| T7 | 0289 | Core request cap, plan, tally and Rust Polars; core lane |
| T8 | 0290 | Final shared examples and contract pages; lands last |
| T9 | 0291 | Remaining language doors; divide into bounded file families |
| F1 | 0292 | Python judges and input shape |
| F2 | 0293 | Native lazy stream |
| F3 | 0294 | Python tally wrapper |
| F4 | 0295 | Python Polars namespace |
| F5 | 0296 | pandas accessor |
| F6 | 0297 | R judges and plan |
| F7 | 0298 | Rust Polars lazy expressions |

The [work plan](../planning/work-plan-2026-09-27.md) owns active file claims and the landing sequence. Codex2 prepares the sixteen tickets and common evidence on a pushed branch. The coordinator assigns implementation files only after review. Numbered drafts remain on their branch until the corresponding code lands. These are subdivisions of the accepted program, not sixteen completed issues or automatic closure of the existing SQL/frame criteria.

## Facts corrected before preparation

- The handoff lists eight spike artifacts even though its heading says six. Reuse the artifacts that reduce a named ticket risk. The T1/T9 prototype branches stack; none is product code merely because its spike passed.
- Demo16's guarded cleanup already landed at reviewed `da126032`. The coordinator rechecked `scratch_lint` on `demos/16-triage-pipeline/review-jsonl` at main `dbf952ca5`; it passes. Its issue still retains the separate full-lint criterion. `children` currently reports exactly one finding at `databases/duckdb/bridge/src/ffi/tests.rs:76`; schedule that narrow prerequisite under0276's accepted method.
- The collision forecast names older source. Refresh file claims against current main and retain the completed0259 fixes, recent wrapper facts migrations and Python test-runner renames.
- T drafts need the repository's five-part Evidence section; F drafts need full outcome, code, proof and dependency sections. The accepted ADR bodies resolve stale summary phrases: SQLite reuse is connection-scoped, R judge plan lands withF6, and expression deadlines are per morsel. Record these as preparation corrections.
- Read `spikes/BOUNDED-RUNS.md` before any later experiment command. PostgreSQL requires server `statement_timeout = '30s'`; shell timeout alone leaves its query running. SQLite uses a bounded timeout. Repeated correlated/EXISTS shapes use1,000 rows. Aggregate once before a100,000-row join. Do not rerun an identical timeout. Routine proof remains a small functional set; RSS/load campaigns remain explicit opt-in work.
- E7–E9 recordings and installed-host proofs remain build work. Saved spike timing or a signature stub does not close those criteria. No new spike, SQL build, provider call or full package campaign ran during intake.

## Checks

The import was compared to the exact accepted branch with only the number substitutions and stale status sentence removal above. Focused `pages`, `tickets` and diff checks passed. The imported contract is accepted; the numbered implementation tickets still need current-source preparation and review.
