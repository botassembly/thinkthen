# 0085: Build the real engine facade

Status: built on `ticket/0085-real-engine-facade`; code review pending. Owner: Claude.

## Call-path inventory before the first code change

Tree: the branch after merging main `86f4012e`. Every owner below is private.

| Concern | Owner on this tree |
| --- | --- |
| Question files, typed values, plans | `core` (`question_file`, `question_set`, `plan`, `find`, `recognize`, `relation`, `relate_file`) |
| Answer rules, thresholds, ranking, the find selector | `core` (`answer`, `threshold`, `order`, `find`) |
| Recognition assembly and relation planning and edges | `core` (`recognize::assemble_names`, `relation::{plan_relation, plan_pairs, assemble_edges}`) |
| Request encoding, digests, the backend-limit split | `engine::prepared_request` (`PreparedRequest`, `PreparedRequests`, `SettledRelation`) |
| Replay, recording, cache locks | `engine::recorder`, `engine::cache_lock` |
| Transport, retries, the width gate | `engine::http::Client`, `engine::Widths` |
| One request through replay, transport, and recording | `engine::request::{ask_profile, ask_prepared}` |
| Ordered record scheduling and grouped scheduling | `engine::schedule`, `engine::annotate_schedule`, `engine::workers` |
| Counters | `engine::usage::Counters`, owned by the command's `Environment` |
| Cancellation and deadlines | `engine::Cancel` |

What the command held beside those owners, before this ticket:

| Function | Command path | Direct low-level use |
| --- | --- | --- |
| `decide`, `choose`, `score`, `tag`, `filter`, `rank` | `cli/judge.rs` to `cli/asking.rs` `run` and `Judging::finish_row` | builds `Client` and `Recorder`, calls `asking::ask` to `engine::request::ask_profile`; streams through `cli/schedule.rs` to `engine::schedule::run_cancelled`; the one-document call sends on the calling thread |
| `find` | `cli/find.rs` | builds `Client` and `Recorder`, `PreparedRequest` for the plan, `asking::ask`; sends on the calling thread |
| `annotate` | `cli/annotate.rs`, `cli/annotate_schedule.rs`, `cli/annotate/plan.rs` | builds `Client` and `Recorder`, `PreparedRequests` per group, `asking::ask_prepared`, `engine::annotate_schedule::run` |
| `recognize` | `cli/recognize.rs`, `cli/recognize/relation.rs`, `cli/recognize/dry_run.rs` | the whole staged pipeline (token questions, split, send, token answers, name assembly, relation settle and send, edge assembly) lives in the command; sends on the calling thread |
| `relate` | `cli/relate.rs`, `cli/relate/plan.rs` | relation settle and split, sequential sends, logical-answer bookkeeping in the command; sends on the calling thread |

`cli/asking/request.rs` held the command's two request adapters (`ask`, `ask_prepared`) and the `Asking` chunk loop that `recognize` and `relate` shared.

## Result

`crates/thinkthen/src/engine/facade.rs` holds one private `Engine`. `Engine::new` takes resolved `Settings`, registers an explicit width through `process_width`, opens the recorder, and builds the one HTTP pool. It reads no key and sends nothing. The retained pool, recorder, and counters sit in one `State` behind the private `Engine::state` accessor, where ticket 0096 puts its guard.

The ten functions reach the engine through these calls only:

| Function | Facade call |
| --- | --- |
| `decide`, `choose`, `score`, `tag`, `filter`, `rank` | `judge`, and `records` for a stream |
| `find` | `find` |
| `annotate` | `split`, `ask_chunks`, and `groups` for a stream |
| `recognize` | `recognize` (`engine/facade/recognize.rs`) |
| `relate` | `relations`, then `relate` (`engine/facade/relate.rs`) |

`request::ask_sent` sends every live attempt through `workers::on_worker`. A single call spawns one scoped worker and joins it. A scheduler worker sends on itself. A replay or cache hit never reaches `on_worker`. `Error::retryable` holds the one retry rule, and the transport's retry loop reads it.

The command keeps argument parsing, input framing and its detached stdin feeder, width registration through `--jobs`, rendering, exit codes, and its signal handlers. `sdlc/scripts/policy.py` now refuses a second `Client::new` outside the facade, any command file that names `engine::{schedule, annotate_schedule, request, prepared_request, recorder, http, workers}` or aliases or globs the engine, and a library-root re-export of those modules. Nine planted bypasses fail the check and six controls pass.

## Owners and duplication

- Deleted from the command: `cli/asking/request.rs` (the two request adapters and the `Asking` chunk loop), `cli/recognize/relation.rs`, `cli/relate/plan.rs`, the recognition pipeline and its result types in `cli/recognize.rs`, the relation executor in `cli/relate.rs`, and the `Execution` and `Logical` copies in `cli/relate/result.rs`.
- Moved, not copied: recognition staging into `engine/facade/recognize.rs` and relation execution into `engine/facade/relate.rs`. Both still call the core tokenizer, splitter, name assembler, relation planner, and edge assembler.
- Deleted from the engine: the test-only `request::ask_with` and `schedule::run`. The shared-case runner used them and now crosses the facade.
- Retained on purpose: `cli/schedule.rs::jobs_of` and `width` stay, because the command registers `--jobs` before it reads input. `Engine::new` selects the same width again, which the width state accepts.
- Touched owners: `engine::{error, http, request, schedule, usage, workers, mod}`, `core::mod` (the `FindAnswer` re-export), and the command modules above.

## Tests

Six red-green tests first failed because no facade module existed. They are deleted: the shared-case runner covers what they showed. Ten facade tests in `engine/facade_tests.rs` and `engine/facade_tests/contract_tests.rs` remain. Each is a contract check against a counted loopback listener:

- sending thread and a host `SIGUSR1` during a held single send;
- R2-21: a refused connection fails at once, is not retryable, and counts one attempt;
- G2: a close after the body sends once on `judge`, `find`, `annotate`, `recognize`, `relate`, and `records`;
- R5-19: one reply held across a batch cancel; after release the count stays at one, and the next call adds one;
- width 2 across six concurrent calls of every kind: peak 2;
- pre-fired cancel, spent deadline, and an impossible profile limit on five calls: no request, no count, no file;
- input order under reversed completion, with a good, an unsure, and a failed answer, the stop position, and no request from reading results;
- G4: bulk rows of case 27 carry the details probability, and scalar calls on cases 37 to 39 return the one-question `annotate` values;
- only `Engine::state` reads the state field;
- the key reaches no result, error, `Debug` line, count, or recording.

`cli/conformance_tests/runner.rs` now runs every case in `conformance/cases.json` through the facade from a replay folder with no key. It uses `judge`, `find`, or `split` with `ask_chunks` by verb, and fault cases stop `records`. The staged `recognize` and `relate` cases run the command, which calls the facade. `backend-profiles.json` crosses `facade::split`. `record-values.json` holds rendering cases with no request. The core test keeps them, and no facade call applies.

## Planted bugs

Each plant ran alone on a committed tree and was reverted.

| Row | Plant | Result |
| --- | --- | --- |
| R2-21 | `Error::retryable` also accepts `Transport(Refused)` | red: `!error.retryable()` failed |
| R5-19 | the ticket's plant: a scheduler worker skips its stop check once | stayed green (see below) |
| R5-19 | `Engine::records` hands the scheduler a fresh token in place of the call's | red: 3 sends where 1 was expected |
| G2 | `is_retried` also accepts `Transport(PrematureClose)` | red: `judge` sent 4 times |
| sending thread | `on_worker` sends on the calling thread | red: the calling thread sent |
| state accessor | `recording` reads `self.state` directly | red: 2 reads |

## Budgets

Measured with `git diff -U0 86f4012e HEAD -- '*.rs'`, nonblank lines:

- Production: 29 files changed. The budget is 16. 1,166 lines added and 929 deleted, net 237. The budget is 1,200 added.
- Tests: 778 added and 115 deleted. The budget is 2,100.
- Combined: 1,944 added. The budget is 3,300.
- Largest touched file: `cli/asking.rs` at 485 nonblank lines.

## Known limits

- Worker joining has no new test. `on_worker` uses a scoped thread, and `records` and `groups` call the schedulers whose lifetime tests already prove the join.
- A conflicting explicit width is not tested at the facade. The width state is one per process, and a unit test that selects it would change every later test in the binary. 0077's command tests cover the refusal. `Engine::new` refuses before it builds the pool.
- Malformed typed input and a request budget have no facade test. The facade takes typed values that the core has already checked, and the private engine holds no request budget.
- Split requests, recognition stages, and relation questions run in order inside one call, so worker completion cannot reorder them.

## What the ticket did not foresee

- The file budget. Routing ten functions and deleting the command's copies touches 29 production files, three of them deletions. The stop condition names the budget. Ian can overturn the budget or ask for the change split.
- The ticket's R5-19 plant cannot send here. The scheduler never queues an item beyond its free workers, and the gate wait and pre-attempt check stop a dispatched item. The record uses the plant above instead.
- The command's stdin feeder stays detached and command-owned, because a blocked read cannot be joined. `records` and `groups` take the reader as a closure.
- Two test-only hooks: `workers::SENDS`, which the ticket asks for, and `Engine::gated`, which gives a test its own width gate inside one test binary.
- `sdlc/issues/2026-09-24-reconcile-signal-dependencies-after-0078.md` stays open. This build adds no dependency.
