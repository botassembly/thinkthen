# 0134 build: withhold the relate entity's kind

Builder: Claude (Opus subagent), 2026-09-26, on `ticket/0134-public-library-gaps` in lane `thinkthen-lane-2`. The ticket was accepted on its second review pass by a fresh read-only Claude session. `origin/main` was merged after ticket 0136 landed (`152c6477`), and the ladder ran on that merge head `defd34a1`. Main then moved (ticket 0133 and others), so `origin/main` was merged again and the ladder ran a second time on `74f1bd6b`. After that run, ticket 0137 reached main. The branch does not carry it, and the lander merges it. Ian can overturn every decision below.

## Outcome

A public `thinkthen::Entity` prints `Entity { name: <N bytes withheld>, kind: <M bytes withheld> }` under `{:?}` and `{:#?}`. The kind no longer prints in clear. The accessors, the refusals, and the public API are unchanged.

## Scope change

The accepted design also took item 8 of `sdlc/issues/2026-09-25-public-library-api-gaps.md`, the Python Polars column table. It was built and its plants ran red. Before the ladder finished, the coordinator said ticket 0136 settles item 8 and told this ticket not to build it. The item 8 code and tests left the branch unmerged, and the issue marks item 8 "Settled on 2026-09-26 by ticket 0136". The ticket page was narrowed in the same commit.

What the withdrawn build found, for 0136's owner to compare:

- Python's widened score of 1.0 printed `1`. A Polars frame's tag column was `List(String)`. The engine's `value_json` gives `1.0` and the array text `["bill"]`.
- A per-record loopback listener proved the whole widened table through both doors: decide `true` and null, score `1.0`, tag `["bill"]`, choose `billing`, and the marker `{"failed":{"kind":"backend","cause":"missing_answer"}}`. The Rust door already wrote every cell of that table correctly.
- Plants run: Python's own score text turned the Python test red (`'urgency': '1'`); a list tag column in a Polars frame turned it red (`List(String)`); `f64` display in the Rust door's `column.rs` turned the Rust test red.

## Proof

| Test and plant | Result |
| --- | --- |
| `public_members.rs` `a_relate_entitys_debug_line_withholds_its_name_and_kind`, with `kind` printed in clear | RED: `kind: "sentinel-kind-x"` in both lines. GREEN after restore |

The plant was applied alone, the test run under the heavy lock, the file restored byte for byte, and its modification time touched. The plant script and log sit outside the repository. A grep of the diff for the plant marker found none.

The four questions. It protects the rule that a public `Debug` line prints no caller text. A credible regression is the pre-fix line or a new field printed in clear. No other test formats a public `Entity`. It needs no test-only hook.

A change to a public type's `Debug` output asks for a second review under the repo `CLAUDE.md`. The coordinator's code review is that review.

## Ladder

Each rung ran once per run, directly, never wrapped in `flock`. Each rung started when the heavy lock was free, so no time below includes a wait for another builder.

| Rung | Run 1 on `defd34a1` | Run 2 on `74f1bd6b` |
| --- | --- | --- |
| `install` | pass, 1 s | pass, 1 s |
| `lint` | pass, 105 s | pass, 160 s |
| `test` | pass, 123 s | pass, 128 s |
| `spec` | pass, demos 21 green and 0 red, 22 s | pass, demos 21 green and 0 red, 19 s |
| `surfaces` | pass on all ten surfaces, 884 s | pass on all ten surfaces, 581 s |
| Total | 1,135 s | 889 s |

Run 2 rebuilt what main's merge changed: the interrupt handler, the recording code, and the PostgreSQL and DuckDB bindings among them.

## Lane trial

- The lane started cold: 29 MB, no `target` folder.
- The withdrawn item 8 work built the Python extension, the Rust Polars lane, and the `public_members` test in the lane, so the lane was 1.6 GB before any rung ran.
- An earlier ladder run on the item 8 code, stopped by the scope change, took `install` 0 s and `lint` 250 s from that partly warm lane. It was stopped before `test`.
- The lane measured 2.9 GB before run 1 and 9.3 GB after it. It measured 9.3 GB after run 2.
- Run 1 is the first full ladder in this lane. It ran on a lane warmed by the stopped `lint` and the development builds, so it is not a clean cold run. Run 2 is the warm run, 246 s faster in total.

## Ratchet

`node sdlc/scripts/ratchet.mjs` reads `crates + conformance 66547/66547` on `74f1bd6b`. Main at that merge measured 66,536. The rise of 11 is the entity test. The entity fix changes one line and adds none. The Python ceilings are unchanged from main.

## Line counts against the budget

| Budget | Measured |
| --- | --- |
| `relate.rs`: 1 changed line, 0 added | 1 changed, 0 added |
| `public_members.rs`: at most 12 added | 11 nonblank added |
| Root ceiling: at most 12 more | 11 more |

## Issues

- The entity issue closes at landing. The lander moves it to `closed/`.
- The API gaps issue stays open. Item 8 is marked settled by 0136. Items 1 to 7 and 9 stay open with the reasons in the ticket.
- New: `sdlc/issues/2026-09-26-python-entity-and-edge-reprs-print-caller-text.md`. Python's `Entity` and `Edge` reprs print name and kind in clear.

## Stop rules

No stop rule fired. Item 4 goes to a later dead-settings ticket, because removing `cache_bytes` spans the public API, 0084, seven bindings, and `databases/postgresql/check.sh` lines 498 and 499, past this ticket's budget. The flaky-test Quick Fix that once owned that file landed as `a12afc58`.
