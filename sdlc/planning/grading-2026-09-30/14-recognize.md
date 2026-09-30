# Area 14: recognize

Commit `58014776c`. Reviewer: Sonnet 5.5, fresh, read only. Rubric: `README.md` in this folder.

## Scope

`recognize` finds every name in a text in three dependent steps (piece tags decoded by Viterbi, kind and edge questions for each found name, then optional relation pairs), prints each name with exact character offsets and a strength, and streams many-line input in order.

Paths below are under `crates/thinkthen/` unless they start with `specification/` or `sdlc/`. Relation pair planning is shared with area 15 and is counted there.

| Kind | Paths (nonblank lines, tests excluded) |
| --- | --- |
| Code, core | `src/core/recognize.rs` (226 before its test module), `recognize/{bilou 186, pieces 60, questions 223, categories 122}.rs`, `src/core/recognize_file.rs` (334 before tests) |
| Code, engine and surfaces | `src/engine/facade/recognize.rs` (433), `src/cli/recognize.rs` (214), `recognize/{dry_run 234, config 110}.rs`, `src/cli/failure/recognize.rs` (42), `src/public/recognize.rs` (426), `src/cli/schedule/ordered.rs` (221). About 2,770 together |
| Tests | Unit: `src/core/recognize/tables.rs` (5 edge tables, 294 lines), `recognize_file.rs` (2), `dry_run.rs` (1). Integration: `tests/backend/recognize.rs` (10), `recognize/rules.rs` (6), `recognize/stores.rs` (7), `recognize_refusals.rs` (5), `secrecy_recognize.rs` (4), `tests/public_batches/recognition.rs` (1), demo `demos/44-recognize-names` |
| Contract | `specification/recognize.md` (67 lines), `types.md` "Offsets", `result.md`, `settings.md`; ADR 0056, ADR 0111; `sdlc/planning/recognize-design.md` (status still "Proposed"); ticket 0147 |

## Complexity: 4 of 5

| Driver | Score | Measured fact |
| --- | ---: | --- |
| Code size | 4 | About 2,770 nonblank lines across 14 files |
| States and concurrency | 4 | Three dependent steps per text, with cancellation between them. Many-line input runs on a reader thread, scoped workers and channels, and hands rows on in input order (`src/cli/schedule/ordered.rs:1-30`, `:159`) |
| Rules and refusals | 4 | About 30. Pieces (4 rules), 40-piece step-1 requests with 6 pieces of context, Viterbi transitions, a one-in-a-million floor and an order tie rule, edge options, kind rules (0 to 20 kinds, 3 reserved names), the 600,000-byte guard, relation limits of 255 names and 4,000 questions, strength and cut, and about 6 fixed refusal sentences |
| Surfaces touched | 5 | 22 of 22. Every binding folder and all three SQL hosts mention recognize. Offsets differ by host and the page tabulates four forms (`specification/recognize.md`, "Names") |
| Settings | 4 | About 9 rows: Threshold, Relation threshold, Kinds, Recognize text limit, Relation rules, Request size, Model, Backend profile and Jobs (`specification/settings.md:48-56`). I counted names, not row numbers |
| Contract weight | 4 | Nine pages with a section (recognize, types, result, settings, audit, diff, question-file, backends, recording) and ADRs 0056 and 0111 |
| Churn and debt | 5 | 73 non-merge commits in 7 days on these paths (34 in the last 3 days). Moved onto the question cache on 2026-09-30 (`566e3e336`). The ordered runner was moved out of the engine the same day (`19166848a`). One open issue, deferred (`sdlc/issues/2026-09-25-recognize-and-relate-scale-and-shape.md`) |

Mean 4.3, rounded to 4.

## Grades

| Dimension | Grade | Evidence |
| --- | :---: | --- |
| Quality | B | The constants and sentences on the page match the code: 40 pieces and 6 of context (`src/core/recognize/questions.rs:11`, `:14`), the 600,000-byte guard and its exact sentence (`src/engine/facade/recognize.rs:19`, `src/cli/failure/recognize.rs:39`), the 255 and 4,000 relation limits (`facade/recognize.rs:20-21`), and the kinds refusals (`src/core/recognize_file.rs:56-58`). Strength is P(kind) times P(span) at four places, and the cut reads the printed value (`src/core/recognize.rs:177-181`, `:223`). The Viterbi floor and tie rule match (`src/core/recognize/bilou.rs:12`, `:34`). Contract text has one stale page: `sdlc/planning/recognize-design.md:3` still says "Proposed" and "authorizes no build", and its method lines point at `experiments/` files that are not in the repo. Recognition quality figures on the page come from single runs, and the page says so (`specification/recognize.md`, "Measured quality") |
| Reliability | B | The guard is tested at 600,000 and 600,001 bytes with no send before refusal (`tests/backend/recognize.rs:315`). A failed step-2 request fails the text (`:302`). The edge tables pin pieces, decode, strength, edge options and the 40 or 41 piece boundary (`src/core/recognize/tables.rs:23`, `:47`, `:144`, `:230`, `:281`). Secrecy has its own file of 4 tests (`tests/backend/secrecy_recognize.rs`). Code moved onto a new cache and scheduler on 2026-09-30 and was fixed after review (`469354f5e`, `5bd407521`, `4c24a6257`), so the window caps reliability at B. No wall-clock wait appears in the recognize tests |
| Maintainability | B | Pure core, with `core` touching no file or clock. The pieces, decode, question and file parsing split cleanly into four small files (`src/core/recognize/`). Two files sit near the cap: `src/engine/facade/recognize.rs` (433) and `src/public/recognize.rs` (426). The Unicode category tables were generated from Unicode 15.0.0 by a Python script that is not in the repo, and no test checks them (`src/core/recognize/categories.rs:3`). The `too_many_arguments` suppression on the ordered runner has a stated reason (`src/cli/schedule/ordered.rs:159`). `decode` and `SpanOdds` have table tests only, with no property test |

## Strengths

- The splitter and the decode are pure, small and table-tested with exact outputs (`src/core/recognize/pieces.rs:25`, `src/core/recognize/tables.rs:23`).
- The guard refuses before any request and names its count and limit without echoing text (`src/cli/failure/recognize.rs:39`, `tests/backend/recognize.rs:315`).
- Offsets are character counts, not bytes, and each piece keeps both units (`src/core/recognize/pieces.rs:7-12`).
- `Debug` withholds every recognized name and label (`src/core/recognize.rs:40-51`, `:116-127`).
- The ordered runner says why a many-line `recognize` cannot be one pipeline input and scopes its workers (`src/cli/schedule/ordered.rs:1-9`).

## Cleanup

1. **Mark `recognize-design.md` historical and fix its references.** Where: `sdlc/planning/recognize-design.md:3`, `:5`. Why: the page is listed as a design source, says it authorizes no build, and cites workspace files outside the repo. A new maintainer cannot tell what is still current. The settled contract is `specification/recognize.md`. Size: S. Blocks 0.1: no.
2. **Keep the Unicode tables reproducible.** Where: `src/core/recognize/categories.rs:3`. Why: the generator is not in the repo, and the category tables are fixed at Unicode 15.0.0 while `char::is_whitespace` follows the compiler. Add the generator under `sdlc/scripts` or a test that compares the tables with a pinned reference. Size: S. Blocks 0.1: no.
3. **Add property tests for the decode and the span odds.** Where: `src/core/recognize/bilou.rs:34`, `:123`. Why: `rust-standards.md:60` asks for property tests on total functions. The decode must always return valid, ordered, non-overlapping stretches, and each span odd must lie in 0 to 1. Size: S. Blocks 0.1: no.
4. **Split the two files near the cap.** Where: `src/engine/facade/recognize.rs` (433), `src/public/recognize.rs` (426). Why: the next rule added to a step fails the file cap. Size: S. Blocks 0.1: no.
5. **Bound relation pairs by distance after 0.1, as filed.** Where: `sdlc/issues/2026-09-26-relation-pairs-span-every-mention-and-the-whole-text.md`. Why: the pair count grows with the square of the names, and a limit changes the contract, so it needs a ruling. Size: L. Blocks 0.1: no.
6. **Add the 254, 255 and 256 relation cases to the shared conformance set.** Where: `sdlc/issues/2026-09-25-recognize-and-relate-scale-and-shape.md`, item 6. Why: only the command proves the limit and its count. Size: M. Blocks 0.1: no.

## Confidence: medium

What was read: `specification/recognize.md` in full, `core/recognize.rs`, `pieces.rs`, most of `bilou.rs`, the head of `categories.rs`, the constants and function list of `questions.rs` and `engine/facade/recognize.rs`, the head of `ordered.rs`, the failure sentences, the test function lists, the open issue and the git history of the area's paths.

Not checked: `public/recognize.rs`, `cli/recognize.rs` and `dry_run.rs` bodies, the second half of `bilou.rs` (the span odds arithmetic), the question wording against the page, and whether every offset row of case 41 holds on the 13 bindings (bundle D covers that). The windows across chunk edges were checked only through the 40 and 41 piece table. The two-step request grouping in `step_two` was not traced. No test was run, so the recognition figures are quoted from the page and unconfirmed.
