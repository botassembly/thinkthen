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
- Kept on purpose, three copies that sum request metadata (model check, usage, replayed flag, send count, digests): `Aggregate::add_answered` and `Aggregate::add` in `engine/facade/recognize.rs`, `add_meta` in `engine/facade/relate.rs`, and `cli/annotate/aggregation.rs`. All three predate this ticket, and their rules differ. Annotate drops usage when any reply lacks it, and the other two keep the part they know. Recognize names both models on a mismatch, and relate names neither. This ticket moved two of them without changing behavior.
- For 0086: annotate's per-record assembly (model check, usage sum, digests, failed members) still lives in `cli/annotate/aggregation.rs`, and `filter` and `rank` still assemble in the command's closures over `records`. 0086's public `annotate` must move that assembly below the facade. It must not copy it.

## Tests

Six red-green tests first failed because no facade module existed. They are deleted: the shared-case runner covers what they showed. The code review then removed three more tests and both test-only hooks under the tests-earn-their-place rule. The facade width test repeated `every_live_path_and_every_engine_share_one_cap`. The refused-connection test repeated three transport tests. The thread-ID list in `workers` and `Engine::gated` are gone. Nine facade tests remain in `engine/facade_tests.rs` and `engine/facade_tests/contract_tests.rs`, and one child-process test sits in `cli/schedule/width_tests/facade_tests.rs`:

- a host `SIGUSR1` on the calling thread while a single judgment is held, and again while a `find` is held: both answer with one send each;
- G2: a close after the body sends once on `judge`, `find`, `annotate`, `recognize`, `relate`, and `records`;
- relate: when every answer fails, the call fails with the `backend` kind, and a partial failure keeps its good edge and counts the failed answer;
- pre-fired cancel, spent deadline, and an impossible profile limit on five calls: no request, no count, no file;
- input order under reversed completion, with a good, an unsure, and a failed answer, the stop position, and no request from reading results;
- G4: bulk rows of case 27 carry the details probability, and scalar calls on cases 37 to 39 return the one-question `annotate` values;
- only `Engine::state` reads the state field;
- the key reaches no result, error, `Debug` line, count, or recording;
- in the width child process: the facade registers an explicit width 1, refuses a conflicting width 2 with the `usage` kind before any send, and then runs R5-19. One reply is held across a batch cancel. After the release, the count stays at one, one row is kept, and the next call adds one send.

`cli/conformance_tests/runner.rs` now runs every case in `conformance/cases.json` through the facade from a replay folder with no key. It uses `judge`, `find`, or `split` with `ask_chunks` by verb, and fault cases stop `records`. The runner keeps each call's reply and applies the answer rules itself, so it checks request bytes, digests, and replies at the facade. It does not compare the facade's bare values. Eight command tests and one demo cover a threshold the facade drops. The staged `recognize` and `relate` cases run the command, which calls the facade. `backend-profiles.json` crosses `facade::split`. `record-values.json` holds rendering cases with no request. The core test keeps them, and no facade call applies.

## Planted bugs

Each plant ran alone on a committed tree and was reverted.

| Row or rule | Plant | Result |
| --- | --- | --- |
| R2-21 | `Error::retryable` also accepts `Transport(Refused)` | red in `http::tests::no_transport_failure_is_sent_again`, `http::tests::a_refused_attempt_is_observed_once_and_returned_without_a_retry`, and `exchange::a_refused_port_fails_before_the_first_default_retry_wait` |
| R5-19 | the ticket's plant: a scheduler worker skips its stop check | red in `engine::schedule::tests::cancellation_with_work_in_flight_ignores_later_input_and_joins` |
| R5-19 | `Engine::records` hands the scheduler a fresh token in place of the call's | red in the width child: 3 sends where 1 was expected |
| G2 | `is_retried` also accepts `Transport(PrematureClose)` | red: `judge` sent 4 times |
| sending thread | `on_worker` sends on the calling thread | red, 3 of 3: `Transport(Other)` |
| width | `Engine::new` ignores the explicit width | red in the width child |
| relate | `relate` drops its no-usable-answer check | stayed green (see below) |
| state accessor | `recording` reads `self.state` directly | red: 2 reads |

## Budgets

Measured with `git diff -U0 86f4012e HEAD -- '*.rs'`, nonblank lines:

- Production: 29 files changed. The owner lifted the file budget from 16 to 29 after the code review: only 8 files carry new logic, 2 hold moved code, and the rest are deletions and call sites. 1,137 lines added and 929 deleted, net 208. The budget is 1,200 added.
- Tests: 848 added and 115 deleted. The budget is 2,100.
- Combined: 1,985 added. The budget is 3,300.
- Largest touched file: `cli/asking.rs` at 485 nonblank lines.

## Known limits

- Worker joining has no new test. `on_worker` uses a scoped thread, and `records` and `groups` call the schedulers whose lifetime tests already prove the join.
- Malformed typed input and a request budget have no facade test. The facade takes typed values that the core has already checked, and the private engine holds no request budget.
- Split requests, recognition stages, and relation questions run in order inside one call, so worker completion cannot reorder them.

## What the ticket did not foresee

- The file budget. See Budgets.
- The ticket's R5-19 plant cannot send at the facade. The scheduler never queues an item beyond its free workers, and the gate wait and pre-attempt check stop a dispatched item. The scheduler's own test catches that plant, and the fresh-token plant proves the facade passes the call's token.
- `relate`'s check that fails a call with no usable answer cannot be reached. The decoder refuses a reply in which every answer failed, and it returns `Reply`, so `relate` fails before the check runs. A partial reply always holds one good answer. The reviewer's plant that drops the check stays green for that reason. The command behaves the same as before this ticket. Ian can decide whether to delete the check and its `RelateLogical` error.
- The command's stdin feeder stays detached and command-owned, because a blocked read cannot be joined. `records` and `groups` take the reader as a closure.
- The ticket asked for a test hook that records the sending thread. The tests-earn-their-place rule removed it, because the signal alone turns the calling-thread plant red. Ian can overturn that.
- `sdlc/issues/2026-09-24-reconcile-signal-dependencies-after-0078.md` stays open. This build adds no dependency.

