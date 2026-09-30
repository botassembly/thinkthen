# After 0304 slice 3: preparation

Status: preparation for builders, 2026-09-30. Read against `origin/main` at `4e2ec93e6`, after slice 3a landed and while 3b, 3c and 3d build. This page changes no ticket. Line numbers are from that commit. Ian can overturn each recommendation.

Read first: `sdlc/planning/cleanup-2026-09-30.md` "Lessons for builders", ADR 0111 build steps 4 and 5 (`sdlc/planning/adr/0111-question-cache-and-one-batching-path.md:241-242`), and `sdlc/planning/0304-slices-3b-3d-prep.md`.

"3d part 2" below means 3d's SQL-host follow-up: PostgreSQL `call.rs` and `call/settings.rs`, and DuckDB `engines.rs` (`0304-slices-3b-3d-prep.md:148,153`).

## 1. Ticket 0304 slices 4 and 5

### Slice 4: `find`, `recognize` and `relate` on the question cache

Ruling 4 says their backend questions are cached by context, question and model. ADR 0111 section 5 keeps their wire form (`adr/0111…:166-178`), so their fixtures convert without loss.

Where the old path lives today:

| Function | Old path |
| --- | --- |
| `find` | `engine/facade.rs:317-326` calls `ask` (`:405-416`), which calls `request::ask_profile` with the old `Recorder`. Callers: `cli/find.rs:107`, `public/bulk.rs:396`. Dry run: `cli/find.rs:130` calls `facade::split` |
| `relate` | `engine/facade/relate.rs:44-98` plans pairs and splits them with `pair_chunks` (`engine/prepared_request.rs:75-97`, the 400-question cap). `relate_observed` (`relate.rs:113-160`) sends through `ask_chunks_with_plan` (`engine/facade.rs:343-376`). Callers: `cli/relate.rs:43,75`, `public/relate.rs:284-290`. Dry run: `cli/relate/dry_run.rs:78-79` reads `questions_per_rule` and `requests_per_rule` |
| `recognize` | `engine/facade/recognize.rs:120` (step 1), `:248` (`pair_chunks` for step 3), `:294` (`ask_chunks_with_plan`). Callers: `cli/recognize.rs:185`, `public/recognize.rs:429`. The command's many-line form runs each line through the old record scheduler: `cli/recognize.rs:131,157` call `schedule::over_records` (`cli/schedule.rs:296-322`), which calls `Engine::records` (`engine/facade.rs:379-403`) and `engine/schedule.rs` |

What changes:

1. Each step becomes an `Asker` (`engine/pipeline.rs:29-48`) and calls `ask_all`. `find` asks one `choice` over its unit set. `relate` and recognize step 3 ask one `noul` per pair and relation.
2. The 400-question cap moves into the packer. `PackLimits.questions` exists (`core/pack/packer.rs:25-26`), but `Engine::pack_limits` always sets `None` (`engine/pipeline.rs:219`). Relate and recognize step 3 pass `Some(400)`, or the profile's smaller `max_questions`, as `pair_chunks` does today.
3. `pair_chunks`, `ask_chunks` and `ask_chunks_with_plan` go, per ADR 0111 step 4, and `engine/prepared_request.rs` goes by ADR section 10. Other callers need a home first. `thinkthen check` sends its probe through `split` and `ask_chunks` (`cli/check.rs:66,116`). The `find` dry run calls `facade::split` (`cli/find.rs:130`). The command's conformance runner and the facade tests call `ask_chunks` (`cli/conformance_tests/runner.rs:50`, `engine/facade_tests.rs:136`). `check` must not read or write the cache (`specification/check.md:19`), so it needs a send-only call on the new send stage. Dry runs take the pure packer.
4. The command's many-line `recognize` needs an outer record loop that is not `engine/schedule.rs`, or slice 5 cannot delete that file. Today `over_records` answers lines in parallel across the engine's width. An `Asker` returns all of one input's questions at once, so one input cannot hold three dependent steps. Options: (a) run each step across all lines, one `ask_all` call per step, which keeps parallel sends but holds every line until step 3; (b) move the small ordered runner into `cli`, which keeps today's streaming and width; (c) compose the steps per line on the host thread, which runs lines one after another and slows `--jobs`. The slice 4 ticket picks one and proves its effect on sends in flight. Recommendation: (b), because it keeps today's behavior with the least new code.
5. Relate and recognize details list question keys where they listed request digests (`cli/relate/result.rs:49,114,166`, `engine/facade/relate.rs:26,202`), as slice 2 did for the record functions. `requests_per_rule` in the dry run comes from the pure packer.
6. The old-store tests lose their last driver. Slice 2 moved the digest-lock and prune tests onto `find` (`sdlc/tickets/0304-question-cache-and-one-batching-path.md:82`). `tests/backend/cache_locking.rs`, `cache_locking/retained.rs` and `cache_prune_locking.rs` use `find` against the old recorder. `recording_conflicts.rs` and `recording_durability.rs` mix: their `find` tests use the old recorder, but `recording_conflicts.rs:108,205` and `recording_durability.rs:39,81,116` test `decide` and every command on the new store and must stay. Once `find` moves, the old-recorder tests test nothing the product runs. Recommendation: slice 4 deletes only those tests, and each deletion names its replacement in `engine/store/tests.rs` or `tests/backend/question_cache.rs`.
7. 0314 slice 2 deferred typed `annotate`, record-array and `relate` rows in the C door "for 0304's single batching path" (`sdlc/tickets/0314-rust-result-schema.md:50`). No ticket owns that now. Recommendation: slice 4 takes the `relate` rows, since it rewrites that path, and 0314 slice 4's C bridge family takes `annotate` and record arrays.

Tests that pin shapes, and must change or be named as unchanged:

- `tests/backend/relate/ceiling.rs:103,117` pins request sizes of 400 questions. Check `relate/at_once.rs`, `recognize/stores.rs`, `find.rs`, `secrecy_find.rs`, `secrecy_recognize.rs`, `secrecy_relate.rs` and `refusals/relate.rs` for old-store entries and request digests.
- `tests/backend/public_json.rs:114-132` runs `recognize` and `relate` through the public API.
- `tests/find_edge.rs` replays `probes/find-0040/recording`. `tests/relate_edge.rs` keeps the name-versus-text regression.
- `libraries/c/tests/door/golden.rs:39-42` pins exact door bytes for `find`, `recognize` and `relate`. A changed byte must be named with its reason.
- Host tests that call these functions through the public API: the DuckDB `relate_suite.py`, `verbs_suite.py` and `conformance.py`; the SQLite and PostgreSQL conformance runners; Python frame `recognize`; R `tests/recognize.R` and case 41's positions `11..20`.

Retained regressions: every row of `0304-slices-3b-3d-prep.md:157-164` that these paths reach; the 255 refusal; the recognize text-size refusal; relate's partial failure count (`engine/facade/relate.rs:179-181`); no key read on replay; "sends nothing" counted on loopback.

Proof list gap (lesson 2): ADR 0111 step 4 names conformance cases 41 to 50 only. `conformance/cases.json` also holds `18-find-second` (`:1770`), `19-find-none`, `51-same-kind-alerts` and `52-cross-kind-staff`, the two relate cases. Add them. Add `tests/backend/public_json.rs`. Also run the C door tests, the SQL host checks and the Python, R, Ruby and TypeScript checks, because each reaches these functions.

### How the relate precision issue interacts

`sdlc/issues/closed/2026-09-30-relate-pair-planner-loses-precision-on-the-beatles-bench.md` measures edge precision 0.420 under the pair planner against 0.867 under the old choice planner. Slice 4 keeps the pair planner's wire form, so it neither fixes nor worsens this. Its proof requires identical results from converted fixtures.

- Keep slice 4 free of planner changes. The issue's second option, one `choice` per song with a none-of-these answer, changes the question kind and the wire form. That breaks slice 4's identical-results proof and needs new recordings, so it belongs in its own ticket after slice 4.
- The first option, tuning the default cut (`cli/relate.rs:74`, `unwrap_or(0.5)`), is independent of slice 4. The threshold never enters the request, and `--details` already prints every pair's probability (`specification/relate.md:62`), so the bench's saved runs already give every cut. The issue reports that an audit-tuned cut helps little (held-half F1 0.689, 0.373 and 0.596), so a cut alone is unlikely to settle it.
- The same files serve issue priorities rank 14 item 3, the unordered both-ways edge shape, which the coordinator defaulted to "before 0.1, after slice 4" (`issue-priorities-2026-09-30.md:117`). Recommendation: one relate ticket after slice 4 covers the precision decision and the edge shape together. The issue is not yet ranked in `issue-priorities-2026-09-30.md`, because it was filed after that page.

### Slice 5: remove the old store and recordings

ADR 0111 step 5 lists deletions. On main, more than deletion is left:

1. **`cache prune`, `cache unused` and `status` still read only old files.** `cli/cache.rs:42,97-132` calls `engine/cache_prune`. `cli/status.rs:161-202` counts old entries and the folder binding. None reads `thinkthen.sqlite`. So since slice 2 the default cache's size target never shrinks the question store. Slice 5 writes these as SQL over `taken_at`, `answered_by` and keys, then `PRAGMA incremental_vacuum` (`adr/0111…:232`), and moves `status` to `thinkthen.status/2` (`cli/status.rs:134`).
2. **Pieces still imported from files on the list.**
   - `engine/schedule.rs`: `engine/pipeline.rs:17` and `engine/pipeline/run.rs:15` import `Input`. `engine/facade.rs:33` re-exports its types, and `cli/schedule.rs:12` imports them; `cli/schedule.rs` survives under the ADR, but `RecordFlow` is on the delete list. The conformance runner (`cli/conformance_tests/runner.rs:181`) and `engine/facade_tests.rs:152` call `records` with `RecordFlow::Streaming`.
   - `core/batch.rs`: `quoted_plan` and `quoted_plan_of` are used by `cli/asking/judged.rs:86-95`, `cli/annotate.rs:357`, `public/asking.rs:80`, `public/bulk/annotation.rs:74` and the test-only `engine/facade.rs:294`. The file also defines `Setting`, `BatchError` and `BatchRecord`, re-exported at `core/mod.rs:62` and used in about 26 places outside tests.
   - `engine/recorder.rs`: `engine/store.rs:139,228` call `recorder::require_private`, the duplicate check the slice 2 defers name.
   - Each moves before its file goes.
3. **Committed old files.** 67 tracked folders hold `DIGEST.json` files and 42 hold `thinkthen.jsonl`. Outside `probes/`, these have no fixture: `site/recordings` and 14 folders under `site/examples/` (nine Beatles bench examples, one bench results run, the relate, recognize and score-bands pages, and two Bash how-tos). `site/scripts/smoke.mjs:101` replays `site/recordings`. The store's replay reads only `thinkthen.jsonl` or `thinkthen.sqlite` (`engine/store.rs:127-133`), so the site smoke's record-function calls likely miss today. This was not run. Slice 5 converts these folders with `cache convert`, as slice 1 requoted them by script (ticket 0304 line 139), and tells marketing. Only then are the old files deleted.
4. The pages: `specification/recording.md` and the size and splitting sections of `specification/backends.md`.

Proof list gap (lesson 2): add the site smoke, `sdlc/scripts/spec` and demos, `lint` in a clean checkout, the PostgreSQL check (it runs `status`), and every surface check, because the recorder sits under every engine (`engine/facade.rs:108,192`).

## 2. Other tickets waiting on slice 3

| Ticket | Needs from slice 3 | Files |
| --- | --- | --- |
| 0314 slice 4 with 0291 | 3b's C door and archive helper; ADR 0112 section 5 starts the port pass after slice 3 (`adr/0112…:110`) | First family: new `libraries/c/src/plan.rs` and `ffi/plan.rs` (`ffi.rs` holds 499 nonblank lines), `libraries/c/include/thinkthen.h`, a new `libraries/BINDING-AUTHOR.md`, `libraries/c/tests/`. TypeScript's hand-built detail JSON also moves here (`tickets/0314…:62`). Then each family's public wrapper, typed result layer and `check.sh`, in the order of `adr/0112…:108` |
| 0322 slice 2 | 3b for the SQL hosts; 3d part 2 for the DuckDB and PostgreSQL engine registries that the exit flush reads | `tests/public_env/usage_totals.rs`, `libraries/c/tests/door/usage.rs`, `databases/sqlite/tests/test_usage.py`, `databases/duckdb/tools/settings_suite.py`, `databases/postgresql/check.sh` |
| 0334 | 3d part 1, which edits `cli/asking.rs` (`0304-slices-3b-3d-prep.md:147`); 0334 edits `cli/asking.rs:206` | `config.rs` and a new `config/backends.rs`, a child of `core/adapters/systemone/`, `cli/edge.rs` and a new module, `cli/edge/key.rs`, `cli/args*`, `cli/status.rs`, `cli/check.rs:36`, the `Backend::resolve` call in each asking command (`cli/asking.rs:206`, `cli/annotate.rs:76`, `cli/find.rs:46`, `cli/relate.rs:27`, `cli/recognize.rs:81`), `public/settings.rs`, `public/settings/environment.rs`, `core/backend.rs`, and the pages |
| 0335 slice 2 | All of 3a to 3d, including part 2, because one merge row waits for 3d (`test-split-2026-09-30.md:67`) and the smokes run every binding | A `THINKTHEN_TEST_PROFILE=smoke` branch in every `check.sh`, new `sdlc/scripts/smoke`, `sdlc/scripts/test`, a shared replay fixture in `conformance/`, and the six merge rows' test files |
| 0335 slice 3 | Nothing in slice 3; see stale premises | `tests/public_batches.rs:64`, `tests/public_controls.rs:34`, `engine/deadline_tests`, `cli/schedule/width_tests.rs` |

Collisions:

- **0304 slice 4 and 0334.** Both edit `cli/find.rs`, `cli/relate.rs`, `cli/recognize.rs` and `cli/check.rs`. The overlap is small: 0334 changes the `Backend::resolve` lines, and slice 4 changes the engine calls. The second lander rebases.
- **0304 slice 5 and 0334.** Both move `status --json` to `thinkthen.status/2` in `cli/status.rs`. ADR 0114 says whichever lands second adds its fields (`adr/0114…:111`).
- **0304 slice 4 and 0314 slice 4.** Both can change `libraries/c/tests/door/golden.rs` and the host tests that pin `relate` and `find` output.
- **0304 slice 4 and 0335 slice 2.** Slice 4 must run `tests/backend/public_json.rs`, and 0335 slice 2's merge row deletes it in favour of the consumer's `cases.rs` (`test-split-2026-09-30.md:68`). Land slice 4 first, or move the recognize and relate rows into the consumer before the delete.
- **0304 slice 4 and 3d part 2.** Both edit `databases/duckdb/tools/relate_suite.py`; it pins the throttle sentence at `:260`.
- **0314 slice 4, 0322 slice 2 and 0335 slice 2.** All three edit `check.sh` files. 0335 slice 2 edits every one. 0322 slice 2 edits `databases/postgresql/check.sh`. Each 0314 family edits its own.
- **Everything.** Each landing changes `sdlc/ratchet.json` and `CHANGELOG.md`. Landings go one at a time, and the lander remeasures with `node sdlc/scripts/ratchet.mjs`.

## 3. Lanes after slice 3 lands

| Lane | Work, in order |
| --- | --- |
| claude-1 | 0304 slice 4, then slice 5. One owner for ticket 0304 |
| claude-2 | 0314 slice 4 with 0291: the C bridge family first, then the families in ADR 0112 section 5's order. Ticket 0314 still says lane claude-1; the lane changes here |
| claude-3 | 3d part 2, then 0322 slice 2, then 0335 slice 2 |
| claude-4 | 0334, then the relate ticket from section 1 once slice 4 lands |

Can start before 3d part 2 lands:

- 0304 slice 4's crate work. Its DuckDB `relate_suite.py` repoint rebases over 3d part 2.
- 0314 slice 4's C bridge family and the families that hold no SQL host.
- 0334. Its slice 1 touches no SQL host.
- 0322 slice 2's Rust and C door rows.

Waits for 3d part 2: 0322 slice 2's SQL rows, 0335 slice 2, and 0304 slice 5, whose surface sweep must see the final SQL hosts.

Land 0334 before slice 5 if both are ready, so slice 5 adds its cache fields to an existing `status/2` rather than two tickets drafting it.

## 4. Stale premises and missing checks

Stale on main (lesson 1):

1. **0291's P1 proof** (`sdlc/tickets/0291-remaining-language-doors.md:27`) expects 120 body bytes and a 61 to 109 token band. That body is the unquoted single-record form that slice 1 removed. DuckDB's plan suite now pins the quoted body (`databases/duckdb/tools/plan_suite.py:9`) at 182 bytes with a 93 to 166 token band (`:15,19-20`). The C bridge proof takes those figures.
2. **0291 keeps typed facts** (`0291…:13,19`: "Preserve typed owned-facts routes", "Keep 0281/0282 typed-facts methods"). ADR 0112 section 4 deletes the ports' typed facts structs (`adr/0112…:87-99`). The ADR is later and accepted, so it wins. The C door's typed facts exports stay, because its ABI is frozen.
3. **0291's copied headers** (`0291…:17`) name `libraries/swift/Sources/CThinkThen/include/thinkthen.h` and `libraries/objective-c/Sources/thinkthen.h`. Ticket 0332 removed the tracked copies, and ticket 0337 stopped shipping the Objective-C one.
4. **0291 calls all fourteen bindings C doors.** Ruby and TypeScript are native Rust bindings. Ruby's result layer already moved in 0314 slice 3, so Ruby needs only 0291's plan, deadline, cap and refusal methods. TypeScript also moves its hand-built detail JSON to the crate's `Serialize`, as 0314 slice 3 deferred.
5. **0335 slice 2's merge table** (`sdlc/planning/test-split-2026-09-30.md:63-68,77`) names tests that 3a renamed or removed. `public_bulk_keeps_portable_max_bodies_and_row_identities` is now `public_bulk_keeps_portable_question_bytes_and_keys_in_one_request` (`tests/public_batches/portable.rs:17`). `portable_max_cuts_cross_public_series_and_frame_calls` is now `portable_questions_ride_one_request_across_series_and_frame_calls` (`tests/polars/batching.rs:122`). The kept tests `named_group_refused_parent_and_halves_keep_request_identity` and `split_record_details_name_the_refused_parent_and_the_answering_half` are gone; `a_refused_request_and_both_halves_keep_exact_send_shares` (`tests/public_batches/splits.rs:351`) and `split_record_details_keep_one_question_key_each` (`tests/public_batches/details.rs:100`) took their place. Remap each row before its mutation run. The outcome's 82.5 s figure (`0335…:7`) predates slice 1's 29 s.
6. **0335 slice 3** waits for "the 0305 cleanup" to remove the process-wide statics (`0335…:30`). Ticket 0305 only measured the suite; the cleanup it points at is an unfiled follow-up (`cleanup-2026-09-30.md:158`). 3d also keeps one throttle per process (ticket 0304 line 107). The `SERIAL` locks guard that process state and stay. Rescope slice 3 to the long waits and binary merges, or close it.
7. **ADR 0111 step 4** says `ask_chunks` goes. `thinkthen check` and the conformance runner still use it (section 1, item 3).
8. **ADR 0111 step 5** lists `engine/schedule.rs` and `core/batch.rs` for deletion. The pipeline, the command's `recognize`, and three plan builders still import from them (section 1, slice 5, item 2).
9. **0304 slice 3a deferred `default_engine`** (`public/mod.rs:73-74`) to 0314's binding pass (ticket 0304 line 126). 0314 slice 4 does not list it. 0322 also defers its exit flush.
10. **0334's evidence** starts from `f57650111` (`sdlc/tickets/0334-named-backends.md:11`). Its line counts still hold: `cli/edge.rs` 499 and `config.rs` 457 nonblank lines. Rebase over 3d part 1 first.

Missing reachable checks (lesson 2):

- 0304 slice 4: conformance cases 18, 19, 51 and 52; `tests/backend/public_json.rs`; the C door golden test; the SQL, Python, R, Ruby and TypeScript checks (section 1).
- 0304 slice 5: the site smoke, `spec`, demos, a clean-checkout `lint`, and every surface check (section 1).
- 0334: it changes `EngineBuilder::from_env`, which every binding and SQL host builds from. Add a surfaces checkpoint with no backend named, to prove the byte-for-byte keep.
- 0322 slice 2: SQL extensions cache only in a named folder since 0318. The cached rerun must name one, or it proves nothing.
- 0335 slice 2: each smoke must take ADR 0113's scratch usage step, or the decoy guard fails `test`.
- 0314 slice 4: the header export check (`sdlc/scripts/check-c-exports.py`), `release-pack` and `release-go-cpp-pair` member lists, the type corpus self-test, and 3b's localized archive in place of Cargo's raw one.

## Open risks

- The site smoke is probably red since slice 2. Marketing owns `site/`, so slice 5 must coordinate before it deletes files.
- Since slice 2, `cache prune` does not shrink `thinkthen.sqlite`, so the default cache can grow past its size target until slice 5 lands.
- The relate precision decision is open for 0.1. It needs a queue-owner ruling and probably a paid bench run under `sdlc/scripts/live`.
