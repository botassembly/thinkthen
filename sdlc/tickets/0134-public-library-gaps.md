---
flow: build
priority: 134
opens: crates/thinkthen/src/public/relate.rs crates/thinkthen/tests/public_members.rs sdlc/ratchet.json sdlc/issues sdlc/records sdlc/tickets
---

# 0134: Withhold a relate entity's kind, and sort the public library gaps for 0.1

Status: built, awaiting code review (`sdlc/records/0134-build-public-library-gaps.md`). Design accepted 2026-09-26 by a fresh read-only Claude review on its second pass. Owner: Claude.

Lane: thinkthen-lane-2

Review route: a fresh read-only Claude session reviewed the design. A public type's `Debug` line changes, so the coordinator's code review is the second review the repo `CLAUDE.md` asks for, and it names what it checked.

## Scope change on 2026-09-26

The accepted design also took item 8 of `sdlc/issues/2026-09-25-public-library-api-gaps.md`: Python Polars writes the ADR 0047 column table. It was built on this branch, with plants red, before the ladder ran. The coordinator then said ticket 0136 settles item 8 and told this ticket not to build it. The item 8 code and its two tests came out of the branch unmerged. This page keeps the rest of the accepted design. Its record, `sdlc/records/0134-build-public-library-gaps.md`, gives what the withdrawn build measured, for 0136 to compare.

## Outcome and authority

A library user who formats a public `thinkthen::Entity` with `{:?}` sees neither its name nor its kind. Today the kind prints in clear (`sdlc/issues/2026-09-25-the-public-entity-debug-prints-its-kind.md`). The repo `CLAUDE.md` requires a secrecy test over every `Debug` line.

The coordinator assigned the entity issue and the API gaps issue to this ticket, and asked it to take what 0.1 needs and name a reason for each gap it defers.

## Prior evidence

- The core `RelationEntity` `Debug` (`crates/thinkthen/src/core/relation.rs:19`) withholds name and kind through `Withheld`. The public `Entity` `Debug` (`crates/thinkthen/src/public/relate.rs:145`) withholds only the name.
- No test formats a public `Entity`. The command's secrecy sweep never meets one, so nothing fails today.
- `crates/thinkthen/tests/public_members.rs` holds the public `Debug` secrecy test for builders and its `shown` helper, which returns the plain and pretty lines.

## Decision

`Entity`'s `Debug` prints `kind` as `Withheld(self.kind.len())`, as it already prints `name`. The line becomes `Entity { name: <N bytes withheld>, kind: <M bytes withheld> }`. The accessors `name()` and `kind()` do not change. Any value that holds an `Entity`, such as a relate edge, prints the same withheld line.

## Edge cases

| Input | Result |
| --- | --- |
| `Entity::new("sentinel-name", "sentinel-kind-x")`, plain `{:?}` | `Entity { name: <13 bytes withheld>, kind: <15 bytes withheld> }` |
| The same, pretty `{:#?}` | the same two fields, one per line |
| `Entity::new` with a blank name or kind | `Usage`, unchanged |

## Proof

| Test and where it runs | What it proves | Planted fault that turns it red |
| --- | --- | --- |
| `public_members.rs` `a_relate_entitys_debug_line_withholds_its_name_and_kind`, `test` rung | The plain and pretty lines of an entity built with marked text equal the pinned lines exactly | Print `kind` in clear again |

The four questions. It protects the promise that a public `Debug` line prints no caller text. A credible regression is the pre-fix line, or a new field printed in clear. No test formats a public `Entity` today. It needs no test-only hook: it builds the entity through `Entity::new`.

## Budgets, in nonblank lines

- `relate.rs`: 1 changed line, 0 added.
- `public_members.rs`: at most 12 added.
- The root ceiling rises by at most 12, equal to the measured count. The record and issue edits are outside the budget.

## Stop rules

1. A fix needs a file another running builder owns: ticket 0132's `engine/http.rs`, backend tests, `specification/backends.md`, `specification/check.md`, or `public_controls.rs`; the flaky-test Quick Fix's `libraries/c/tests/door`, `databases/postgresql/check.sh`, or `crates/thinkthen/tests/polars/throttle_equality.rs`; ticket 0133's rung scripts, children check, or interrupt handler test switch; or ticket 0136's Python Polars files.
2. A budget would be crossed.
3. The test needs a test-only hook.

## Overlaps

| Owner | Shared | Note |
| --- | --- | --- |
| 0132, 0133, 0136 | `sdlc/ratchet.json` | Re-measure the ceiling at merge |
| 0136 | `sdlc/issues/2026-09-25-public-library-api-gaps.md` | This ticket marks item 8 settled by 0136, on the coordinator's word |

## Deferred gaps

Items of `sdlc/issues/2026-09-25-public-library-api-gaps.md`, each left open in that issue:

- **Item 8, Python Polars and the column table.** Ticket 0136 settles it. The issue marks it so.
- **Item 4, `cache_bytes`, and the settings sweep.** Ian ruled that the setter must leave before 0.1. It is 0.1 work, but it changes `databases/postgresql/check.sh` lines 488 and 489, which the flaky-test Quick Fix owns (stop rule 1). It also spans the public API, 0084, and every binding with the setter: Python, Ruby, R, TypeScript, SQLite, DuckDB, and PostgreSQL. It gets its own ticket once the Quick Fix lands.
- **Item 1, counters per engine or per process.** Backlog row A5 pairs it with the status issue's option 1, which builds on the place item 1 picks. Both go in one ticket, and that ticket is 0.1 work in section A.
- **Item 2, a public inline rule parser; item 3, public error constructors; item 9, `LoadedQuestion` as a decision question.** Each adds to the public API without breaking it, so each can ship after 0.1. The backlog puts items 2 and 3 in section C. It does not place item 9.
- **Item 5, find's none option and annotate parts.** Each needs a new spelling in 0084 and adds to the API. Section C.
- **Item 6, the engine reuse test and a library `warm`.** The product reading of 2026-09-22 set it low. A library `warm` is a new method, and the counting test needs every surface's harness.
- **Item 7, typed descriptions and annotate forms outside Rust.** A cross-surface design over Python and TypeScript with a digest test. Section C.
- **Python's own `Entity` and `Edge` reprs.** `libraries/python/src/asked.rs:268` prints `Entity(name=…, kind=…)` in clear, and `:301` does the same for `Edge`. This is the entity issue's defect on the Python surface, and the Rust fix does not reach it. The build files it as `sdlc/issues/2026-09-26-python-entity-and-edge-reprs-print-caller-text.md` for its own Quick Fix.

## Closes

- `sdlc/issues/2026-09-25-the-public-entity-debug-prints-its-kind.md`, moved to `closed/` at landing.
- The API gaps issue stays open. This branch marks its item 8 settled by 0136.

## What Ian can overturn

- Item 4 deferred to its own ticket because of file ownership, although his ruling puts it in 0.1.
- Item 8 left to 0136, on the coordinator's word.
- The Python repr defect filed for a Quick Fix, not taken here.

## Evidence

- Starts from: the entity issue, the core `RelationEntity` `Debug`, the builder secrecy test in `public_members.rs`, the backlog of 2026-09-25, and the API gaps issue as checked against main on 2026-09-26.
- Keeps: the public API and its frozen inventory, `Entity`'s accessors and refusals, and every surface.
- Changes: `Entity`'s `Debug` withholds `kind`. The API gaps issue marks item 8 settled by 0136. A new issue files the Python repr defect.
- Proof: the one test above with its plant, and the `install`, `lint`, `test`, `spec`, and `surfaces` rungs once each after a merge of `origin/main`.
- Defers: API gaps items 1 to 7 and 9, item 4 for file ownership, item 8 to 0136, and the Python reprs to a Quick Fix.

## Complexity

Contract 1; state and timing 0; reach 1; proof 1; cost of error 1; total 4. One `Debug` line and one test.
