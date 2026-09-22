# The settle-everything wave

Ian approved settling every open detail across all languages, 2026-09-21:
"approve the polars decisions. i want all details settled across all languages."
The wave runs the contract lane first; the surface lanes then wire what it lands.

## The contract lane

Every settlement below is in `contract/`, `standin/`, or `conformance/` unless the
row names the branch's top-level records; each was approved by Ian's instruction
above, and each is his to overturn.

1. **`Details.nearest`** (ADR 0017 pick 6). The field is `Option<String>`,
   `Some(level)` on a score question and `None` (serialized null) on every
   other verb. Rationale: the audit found the level unreachable on five
   surfaces while two surfaces claimed it in their docs; one field at the
   contract closes all five, and no surface carries a private field for it.
   The stand-in fills it in `details_opts` from the same serialized level the
   `score` path reads, in one shared helper. On the score path the trail's
   `probability` is the nearest level's own probability — a number the
   backend reported, legal under the one-rule vocabulary — and `answer`
   reads as `Yes` because the core's own read of a resolved score is
   `Outcome::Yes`; a score has no threshold to apply. The decide path is
   unchanged byte for byte. The full-details alternative — widening
   `probability` and `answer` to `Option` to match DuckDB's TrailRow
   convention — was left undone on purpose: the ruling asked for one field.
   Overturnable in one small commit if the product side prefers nulls.
2. **find and rank return the ruled pair.** Verified: `Ranked {index,
   probability}` and `Found {index, probability}` already carried the pair.
   The trait docs now say the pair is the one shape and no surface returns
   the bare unit alone, and `the_find_and_rank_pair_is_the_door_json` pins
   the door JSON exactly.
3. **A spent deadline is legal.** The `Options` docs and `deadline_in` state
   it: a zero budget, or any deadline already past, sends nothing and
   returns the deadline kind naming the budget. `a_zero_budget_is_spent_immediately`
   proves it. The surface lanes align TypeScript's refusal to this.
4. **A built question plus members refuses as ambiguous.** The `Question`
   docs state the one rule: the question carries its members once; handing
   over a built question and members together refuses with a usage error
   naming both. The surface lanes enforce it; Python's silent ignore and
   Ruby's rebuild both align to TypeScript's refusal.
5. **The record row is a host-side rendering.** One doc paragraph where
   `Row` is defined: library verbs return their own values; a host renders
   `{"input","value"}` when it pairs answers with records; no surface must
   emit the row for the engine's sake.
6. **DuckDB's interrupt channel** is named where the interrupt shape is
   described: `FINDINGS.md`'s interrupt paragraph and `MERGE-NOTE.md` §6
   both now name DuckDB's chained SIGINT-at-LOAD handler beside SQLite's
   progress handler and PostgreSQL's interrupt check, with the measured
   0.11 s stop. The task named `MERGE-NOTE-INPUT.md`, but the interrupt
   text lives in `MERGE-NOTE.md` §6 — the input file is the nine-item
   cross-side list and carries no interrupt paragraph; the shape text was
   updated where it actually is.

## The conformance side

`conformance/tools/validate_conformance.py` now asserts the score case's
details carry `nearest_level` at all and that it is one of the question's
own levels, beside the existing value check. Case 13 already carried it, and
the case-schema name `nearest_level` stays: four surface runners read that
name, and the surface lanes own their files. The contract's field is
`nearest`; the case's expectation name is its own vocabulary.

## What this lane broke and who fixes it

Adding a field to `Details` breaks every exhaustive destructure in the
surface shims (`let Details { ... } = ...`) — Python, TypeScript, Ruby, R,
Rust, C, and the three databases. That is the designed handoff: the surface
lanes wire `nearest` into their details records and docs. `contract/`,
`standin/`, and `conformance/` are green by command; the surfaces read red
until their lanes land, exactly as in the previous shape waves.

## Green by command

```
contract: cargo test --release -> 15 passed; 0 failed
standin:  cargo test --release -> 8 + 1 + 7 + 1 + 3 passed; 0 failed
conformance: validate_conformance.py conformance.json -> OK: 74 cases validated
```
