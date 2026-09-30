# Area 15: relate

Commit `58014776c`. Reviewer: Sonnet 5.5, fresh, read only. Rubric: `README.md` in this folder.

## Scope

`relate` reads one complete set of named entities and prints the relationships that reach a cut. It asks one yes/no question per allowed pair, or one menu per source for a rule marked `single`, through the shared relation planner that `recognize --relation` also uses.

Paths below are under `crates/thinkthen/` unless they start with `specification/` or `sdlc/`.

| Kind | Paths (nonblank lines, tests excluded) |
| --- | --- |
| Code, planner | `src/core/relation.rs` (169), `relation/{pairs 312, menu 199}.rs`, `src/core/relate_file.rs` (323 before tests) |
| Code, engine and surfaces | `src/engine/facade/relate.rs` (206), `src/cli/relate.rs` (91), `relate/{result 186, dry_run 119, config 115, input 85}.rs`, `src/cli/args/relate.rs` (27), `src/cli/failure/relate.rs` (39), `src/public/relate.rs` (309). About 2,080 together |
| Tests | Unit: `relation/tests.rs` (2), `relation/pairs.rs` (3), `relate_file/tests.rs` (6). Integration: `tests/backend/relate.rs` (10), `relate/{at_once 3, both_ways 1, ceiling 8, details 5, menu 7}.rs`, `relate_edge.rs` (1), `refusals/relate.rs` (124 lines), `question_file/relate.rs` (2), `secrecy_relate.rs` (1), demo `demos/45-map-relationships` |
| Contract | `specification/relate.md` (56 lines), `recognize.md`, `result.md`, `settings.md`, `records.md`; ADR 0057 with three amendments of 2026-09-30, ADR 0038, ADR 0111; `sdlc/planning/relate-design.md` (historical); tickets 0167, 0342, 0344, 0353 |

## Complexity: 4 of 5

| Driver | Score | Measured fact |
| --- | ---: | --- |
| Code size | 3 | About 2,080 nonblank lines across 14 files |
| States and concurrency | 2 | The area adds no thread of its own. It builds a plan and hands it to the shared pipeline. Modes: pair, menu, either, lines, plan, file |
| Rules and refusals | 4 | About 27. Nine input refusals (malformed rule, bad pointer value, blank name or kind, duplicate pair, absent kind, line rule, over 255 entities, `@FILE` mixed with rules, `single` with `either`), three fixed question forms, edge rules (no duplicates, inclusive cut, a menu tie or `none` makes no edge), exit 6 and exit 4 on partial failure, and the 400-question and request-byte splits |
| Surfaces touched | 5 | 22 of 22. Every binding folder and all three SQL hosts mention relate, and DuckDB runs it on the caller's database (ADR 0038) |
| Settings | 4 | About 10 rows: Threshold, Relation rules, Kind pointer, Field pointer, Request size, Relate query limits, Model, Backend profile, Jobs, Plan preview (`specification/settings.md:48`, `:56-57`, `:83`, `:89`). I counted names, not row numbers |
| Contract weight | 5 | About ten pages with a section, ADR 0057 and its three amendments, and ADRs 0038 and 0111 |
| Churn and debt | 5 | 70 non-merge commits in 7 days (22 in the last 3 days). The single-answer menu, the `either` edge member and the question-cache move all landed on 2026-09-30 (`5f059fcc4`, `e25f55610`, `566e3e336`, `22be506e2`). Two open issues, both deferred past 0.1 |

Mean 4.0, rounded to 4.

## Grades

| Dimension | Grade | Evidence |
| --- | :---: | --- |
| Quality | C | The code follows the page on questions and order. The three question forms match (`src/core/relation/pairs.rs:116-117`, `src/core/relation/menu.rs:120`), and a menu's exact tie or `none` makes no edge (`menu.rs:160-180`). The contract text would mislead on cost. `specification/relate.md:97` says a rerun "sends only the questions no earlier run answered", and `:54` says every request carries the same state of all entities. So adding one entity changes the state in every question, every key misses, and the whole run is billed again. No page says so, and the open issue lists saying it as its first step (`sdlc/issues/2026-09-25-recognize-and-relate-scale-and-shape.md`, item 4). This is inferred from the two page sentences and the issue. I did not run a cache test. The recommended form is also file-only, because inline rules have no `single` (`specification/relate.md:22`) |
| Reliability | B | Failure and limit paths have tests: the 254, 255 and 256 cases and the exact oversize sentence (`tests/backend/relate/ceiling.rs:165`, `:177`), the 400-question and profile splits (`:113`), a failed menu exits 6 and keeps its source (`tests/backend/relate/menu.rs:156`), and an unchanged rerun sends nothing (`:209`). `count_pairs` is tested against real plans (`src/core/relation/pairs.rs:222-245`). A host interrupt test failed under load and was fixed by ticket 0342 (`sdlc/issues/closed/2026-09-30-relate-host-interrupt-test-fails-under-load.md`). The menu, its either flag and its cache move all landed on 2026-09-30, so the window caps reliability at B. The only paid measurement is one run of one bench at one commit |
| Maintainability | B | One planner serves both commands, and the menu and the pair planner live in separate files (`src/core/relation/pairs.rs`, `menu.rs`). No lint suppression in the area. The 255-entity cap is written three times: `src/core/relate_file.rs:88`, `src/public/relate.rs:21` and `src/engine/facade/recognize.rs:20`, with the 4,000 cap beside it. `relate-design.md` keeps a large stale choice-planner section under a historical banner (`sdlc/planning/relate-design.md:3`, `:50-76`). `menu.rs` has no unit test, and its tie logic is proven only through command tests (`src/core/relation/menu.rs:160-180`) |

## Strengths

- The pair planner counts questions by arithmetic before it builds any, with checked overflow, and a test proves the count equals the real plan (`src/core/relation/pairs.rs:57-88`, `:237`).
- The menu trades recall for precision and the page records the measured trade, not a claim: F1 0.523 against 0.635, precision 0.420 against 0.749 (`specification/relate.md`, "Precision"; `sdlc/planning/adr/0057-relate-asks-one-yes-no-question-per-pair.md:105-107`).
- ADR 0057 keeps each amendment dated with the evidence and the bar that decided it, and says Ian can overturn the default (`sdlc/planning/adr/0057-relate-asks-one-yes-no-question-per-pair.md:95-107`).
- The refusal for an oversized set names its count and the hypothetical pair arithmetic, and says the number is hypothetical (`specification/relate.md`, "Entities"; `tests/backend/relate/ceiling.rs:177`).
- `recognize` refuses `single`, so the menu never reaches the text-stated path (`tests/backend/relate/menu.rs:242`).

## Cleanup

1. **State in `relate.md` that adding an entity bills the whole run again.** Where: `specification/relate.md:54`, `:97`, and the database pages. Why: the page promises that a rerun sends only unanswered questions, and never says that a grown set changes every question's key. A user who adds ten songs to 184 pays for all 194. The open issue already names this as its first fix. Size: S. Blocks 0.1: yes.
2. **Let the command ask the recommended form.** Where: `specification/relate.md:22`, `src/cli/relate/config.rs`. Why: the page recommends `single` for single-answer relations, and only an `@FILE` can set it. Needs Ian or the coordinator to rule whether an inline form is worth an option. Size: S. Blocks 0.1: no.
3. **Keep the 255 cap in one place.** Where: `src/core/relate_file.rs:88`, `src/public/relate.rs:21`, `src/engine/facade/recognize.rs:20`. Why: three copies of one number and one sentence must change together. Size: S. Blocks 0.1: no.
4. **Retire the stale choice-planner text in `relate-design.md`.** Where: `sdlc/planning/relate-design.md:50-76`. Why: ADR 0057 superseded the planner, and the page still shows the old plan example with `method:"choice"`. A banner exists, but the text stays next to the current plan schema. Size: S. Blocks 0.1: no.
5. **Add a unit test for the menu tie rule.** Where: `src/core/relation/menu.rs:160-180`. Why: a tie between the top two labels, `none` on top, and a wrong label count each decide whether an edge prints. Only command tests reach them. Size: S. Blocks 0.1: no.
6. **Finish the decision run, with Ian's approval.** Where: `sdlc/planning/adr/0057-relate-asks-one-yes-no-question-per-pair.md:107`. Why: Liquid d1 was not run, and the result rests on one backend and one bench. A second paid run needs authorization under the live ledger. Size: M. Blocks 0.1: no.
7. **Block or group entities before pairing, after 0.1, as filed.** Where: `sdlc/issues/2026-09-25-recognize-and-relate-scale-and-shape.md`, item 5. Why: same-kind pairs grow with the square of the set. At 255 entities an `either` rule asks 32,385 questions, and a test plans 64,770 questions for a full line set (`tests/backend/relate/ceiling.rs:236-248`). Size: L. Blocks 0.1: no.

## Confidence: medium

What was read: `specification/relate.md` in full, the planner files `pairs.rs` (first 170 lines and the count test) and `menu.rs`, ADR 0057 and its amendments, the open issues for the area, the function lists of the ceiling and menu tests, the closed interrupt issue, and the git history of the area's paths.

Not checked: the bodies of `engine/facade/relate.rs`, `public/relate.rs`, `cli/relate/*` and `relate_file.rs`, the integration test bodies, and the question-key code that would prove the whole-run re-bill (cleanup 1 rests on the page and the issue, so it is unconfirmed by a test). The F1 and precision figures come from the page and ADR, and I ran no bench. No test was run. The 13 bindings' relate checks belong to bundle D, and the pair-state request splitting belongs to area 1.
