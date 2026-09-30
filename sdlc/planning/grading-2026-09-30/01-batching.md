# Area 1: Batching

Commit `58014776c`. Reviewer: Opus 5.5, fresh, read only. Rubric: `README.md` in this folder.

## Scope

Batching packs the questions the cache lacks into requests under a size ceiling and other limits, sends them on `jobs` workers, halves a request refused as too large, and hands each answer back to its input in input order.

All paths below are under `crates/thinkthen/src/` unless they start with `crates/`, `specification/` or `sdlc/`.

| Kind | Paths (nonblank lines) |
| --- | --- |
| Code, core | `core/pack.rs` (244), `core/pack/packer.rs` (270), `core/pack/split.rs` (62), `core/batch.rs` (142, now the quote form and the batch setting) |
| Code, engine | `engine/pipeline.rs` (269), `engine/pipeline/run.rs` (473), `engine/pipeline/run/slot.rs` (46), `engine/pipeline/send.rs` (209) |
| Code, askers | `public/asking.rs`, `cli/asking/judged.rs` (411), `cli/annotate/asker.rs`, `engine/facade/each.rs`, `public/bulk/annotation.rs` (317): 1,592 together |
| Code, dry-run planners | `cli/asking/plan.rs` (186), `cli/annotate/plan.rs`, `public/plan.rs`: 493 together |
| Tests | 136 tests in 7,900 lines. Unit: `core/pack/tests.rs` (5), `core/pack/packer_tests.rs` (5), `core/batch/tests.rs` (3), `engine/pipeline/tests.rs` (2). Integration: `crates/thinkthen/tests/backend/batching.rs` and `batching/` (54), `tests/public_batches.rs` and `public_batches/` (47, 2 ignored for `test-stress`), `tests/backend/question_cache*.rs`, `tests/backend/scheduling.rs` (4), `tests/backend/annotate/batching/` (7) |
| Contract | `specification/records.md` "Order and requests"; `specification/backends.md:26`; `specification/result.md` `requests_sent`; `specification/recording.md:72`; ADR 0111 (5,112 words, with a 2026-09-30 amendment), which amends 0048, 0053, 0055, 0092 and 0100; also ADRs 0040, 0051, 0085, 0087, 0090, 0094; ticket 0304 |

## Complexity: 4 of 5

| Driver | Score | Measured fact |
| --- | ---: | --- |
| Code size | 4 | About 3,800 nonblank lines: core 718, engine 997, askers 1,592, dry-run planners 493 |
| States and concurrency | 4 | One coordinator thread, `jobs` send workers, and a host reader thread. The coordinator tracks the unemitted window, keys in flight, the open request, busy workers, a stop, a halt and a 50 ms pause timer (`engine/pipeline/run.rs:48-71`) |
| Rules and refusals | 4 | About 30. Ten close reasons: request bytes, three profile limits, the inputs cap (`--batch N` or 4,096), relate's 400-question cap, a new state, the pause, a full window and the end. Four lone-question refusals, two context refusals, halving once on 413 or 400 `max_tokens_exceeded`, partial and whole refusals, coalescing of equal keys |
| Surfaces touched | 5 | All 22. Seven `ask_all` call sites serve the command, the Rust API, Polars, the C door and every host over it, and the three SQL extensions |
| Settings | 3 | Five rows: Batch (four tiers), Context, Request size, Backend profile, Throttle, plus Plan preview |
| Contract weight | 5 | Six spec pages with sections, eleven batching ADRs, and ADR 0111's amendment |
| Churn and debt | 5 | Rewritten on 2026-09-30 in eight slices of ticket 0304. 26 commits on the core paths since 2026-09-26. Four more commits touched the area after slice 5 landed, one of them a regression fix. 14 batching issues closed in ten days. No open issue names the core path |

Mean 4.3, rounded to 4.

## Grades

| Dimension | Grade | Evidence |
| --- | :---: | --- |
| Quality | B | The code follows ADR 0111 section 4 closely. The key hashes the exact bytes the body joins (`core/pack.rs:89-101`, `core/pack/packer.rs:158-170`). Tests prove that 100 records then 120 send only the 20 new questions (`crates/thinkthen/tests/backend/question_cache.rs:66`). The contract text has drifted in four places: `specification/records.md:106` and `:108` still give annotate one request per `on` group or group slice, `:113` says "equal request digests share one backend call", and `:139` says a run leaves "one entry for each batch". Since ADR 0111 all annotate groups share one state and pack together, and the store keeps one entry per question. `annotate --plan` passes `options: 0` to the packer (`cli/annotate/plan.rs:57`), so a plan can show a request that the live run refuses under a profile's `max_options`. That behavior was read from the code, not run |
| Reliability | B | Failure paths have send-counting tests: halving makes exactly three attempts (`crates/thinkthen/tests/backend/batching/too_large.rs:24`), a refused first half sends no second half (`:144`), a failed request sends nothing later (`crates/thinkthen/tests/backend/batching.rs:472`), the window bound holds (`crates/thinkthen/tests/backend/scheduling.rs:189`), and a spent deadline reads no input (`engine/pipeline/tests.rs:100`). The earliest failure wins whatever order replies arrive in (`engine/pipeline/run/slot.rs:249-271`). The code is less than a day old. Reviews during the build caught at least eight real defects: a 5-second busy wait that ignored stop, a failed request that kept sending, tag labels refused as repeats, the SQLite extension aborting on every call, DuckDB losing rows after a denied retry, and others (`sdlc/tickets/0304-question-cache-and-one-batching-path.md`, "What the build taught us"). One regression surfaced after landing, when audit and diff lost the batch setting (`e877e43df`). The packer has no property test |
| Maintainability | B | One pure packer and one engine path. The area has no lint suppression and no TODO. Its `Debug` output withholds text (`core/pack.rs:71-79`, `:148-156`). Four askers repeat the same answers-to-receipt block (`public/asking.rs:93-126`, `cli/asking/judged.rs:167-231`, `engine/facade/each.rs:206-228`, `engine/facade/annotate.rs:59-78`), and the copies disagree: `each.rs:296-304` keeps a partial usage sum when one share is missing and fails on overflow, while the other three drop the usage. Four dry-run planners rebuild the admit loop, and two of them build `PackLimits` by hand instead of calling `Engine::pack_limits` (`cli/asking/plan.rs:127-133`, `engine/facade/each.rs:157-167`). `engine/pipeline/run.rs` holds 473 of its 500-line cap |

## Strengths

- The packer is pure and lives in `core`. It checks every lone limit before it changes state, so a refused input packs nothing (`core/pack/packer.rs:105-150`).
- The body is joined from the same bytes each key hashes, so a request's size is a sum and replays never miss because of timing (`core/pack.rs:81-101`, `core/pack/packer.rs:281-286`).
- One `ask_all` entry serves every function and surface. Its workers are scoped and joined before return (`engine/pipeline.rs:221-261`).
- The key is read only when the first request is about to go, and a zero send limit refuses before the key is read (`engine/pipeline/send.rs:113-125`, `:186-198`).
- Equal keys in flight are sent once, and only the first waiter counts the send (`engine/pipeline/run.rs:296-300`, `:462-469`).

## Cleanup

1. **Correct the stale request and entry sentences in `records.md`.** Where: `specification/records.md:106`, `:108`, `:113`, `:139`. Why: the settled contract still describes per-group annotate requests, request digests and per-batch entries, all of which ADR 0111 retired. A user reading it expects the wrong request count and resume behavior. Size: S. Blocks 0.1: yes.
2. **Count options in the annotate dry run.** Where: `cli/annotate/plan.rs:57`. Why: every other planner passes `pipeline::options(&ask)`. Here a plan can pass a choose question that the live run refuses under `max_options`. Add a dry-run case beside `crates/thinkthen/tests/backend/annotate/splitting.rs:184`. Size: S. Blocks 0.1: no.
3. **Build a row's receipt from its answers in one place.** Where: `public/asking.rs:93-126`, `cli/asking/judged.rs:167-231`, `engine/facade/each.rs:206-228`, `engine/facade/annotate.rs:59-78`. Why: four copies of read, sum usage, name the model and collect keys. They already disagree on a missing usage share and on overflow (`engine/facade/each.rs:296-304`), against ADR 0111 section 7. One helper beside `pipeline::Answered` settles the rule. Size: M. Blocks 0.1: no. Check it with area 5, which owns spend accounting.
4. **Share one dry-run planner.** Where: `cli/asking/plan.rs:122-144`, `cli/annotate/plan.rs:28-70`, `public/plan.rs:146-180`, `engine/facade/each.rs:150-168`. Why: each planner repeats the admit loop (deduplicate keys, build entries, pack), and two build `PackLimits` by hand, so a new limit must be added in several places. A `pipeline::plan(asker, inputs, packing)` with no lookup would keep `--plan` equal to the live packing. Size: M. Blocks 0.1: no.
5. **Retire the content-cut history from the batching fixtures.** Where: `specification/fixtures/batching/grouping.txt` (nothing reads it), `specification/fixtures/batching/README.md:3-19`, and the `cut`, `members`, `closed` and `digests` fields of `portable-records.json`. Why: the README says the corpus makes three requests that close on content. The command now sends one request (`crates/thinkthen/tests/backend/batching/portable.rs`, `assert_eq!(sent.len(), 1)`). The three request files serve only as key oracles. About 13 binding checks read the corpus, so they change together. Size: M. Blocks 0.1: no.
6. **Add a property test for the packer and the key parser.** Where: `core/pack/packer_tests.rs`, `core/pack/tests.rs`. Why: `sdlc/planning/rust-standards.md` asks for property tests on parsers and total functions. None covers the packer's byte count, which must equal the joined body across the `q9` to `q10` name growth, or the rule that no closed request passes the ceiling unless it holds one question, or the `QuestionKey` hex round trip. Size: S. Blocks 0.1: no.
7. **Decode each wire answer once.** Where: `core/pack.rs:224-265`, `engine/pipeline/run.rs:342-351`, `core/pack/split.rs:340-343`. Why: a cached answer is wrapped in a made-up response body and decoded twice, once to check it and again in the asker's row. A live reply is parsed twice as well. The measured cost is about 50 microseconds per question (ticket 0304, slice 3c). This is headroom, not a defect. Size: M. Blocks 0.1: no.
8. **Split `run.rs` before it reaches its cap.** Where: `engine/pipeline/run.rs` (473 of 500 lines). Why: the next rule added to the coordinator will fail the file cap. Moving the store-facing `lookup`, `stored` and `answered` into their own module keeps the state machine readable. Size: S. Blocks 0.1: no.

## Confidence: medium-high

What was read: `core/pack*`, `core/batch.rs`, all of `engine/pipeline*`, the public and command askers, the three dry-run planners, ADR 0111, ticket 0304's slice evidence and lessons, the batching sections of `records.md` and `backends.md`, and the names of every batching test. Samples of `too_large.rs`, `portable.rs`, `scheduling.rs` and the memory test were read as well.

Not checked: no test was run, so the annotate dry-run divergence (item 2) is inferred from the code. `cli/annotate/asker.rs` and `public/bulk/annotation.rs` were only skimmed. The 13 bindings' own batching checks and the SQL hosts' batching paths were not read; bundles D and B cover them. The live effect of batching on answer quality is out of scope, because Ian ruled on it (ruling 9).
