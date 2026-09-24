# Quick Fix qf-model-mismatch-test: run the model-mismatch check at one job

Status: landed. It closes `sdlc/issues/2026-09-24-the-model-mismatch-cancel-check-fails-under-load.md`, which was filed on the 0085 branch at `a4474607` and copied to main here, because 0085 had not landed. A fresh read-only Opus review is in `sdlc/records/qf-model-mismatch-test-review.md`.

## Result

- `annotate::scheduling::a_model_mismatch_cancels_groups_that_have_not_started` runs at `--jobs 1` with no reply delays. The listener answers group 1 with `jev-1.3` and every other group with `jev-1.2`. The test asserts exit 4 and exactly 2 requests.
- At one job the engine reads group 1's other model and calls `stop()` before it can send group 2. The count depends on no timing.
- The first commit, `8cfd6c5f`, kept `--jobs 2` and ordered replies with an arrivals helper. The review showed a correct run could still fail if the engine took more than its 3-second window to handle group 1's reply. `37c06950` replaced it.
- No production code changed.

## The test gate of `2026-09-24-tests-earn-their-place.md`

- What it protects: a model mismatch stops groups that have not been sent.
- The regression that fails it: a mismatch that records its failure but still dispatches pending groups.
- Why no other test catches it: `different_safe_model_versions_fail_one_record_and_name_both` in `annotate.rs` and `different_models_across_chunks_keep_the_safe_failure` in `splitting.rs` put the mismatch on the last group, so nothing is left to cancel.
- Test-only hooks: none. It drives the compiled binary against the loopback listener.

The review suggested pinning the mismatch message too. The two tests above already pin that message, so this test counts requests only.

## Planted bug and load rounds

The plant removes `self.pending.clear()` from `stop()` and the `!state.halted` check from `dispatch` in `crates/thinkthen/src/engine/annotate_schedule.rs`. Load came from `stress-ng --cpu 16` plus a loop that ran the other backend tests. Each run sat under `timeout 120`.

| Round | Test | Tool | Load | Result |
| --- | --- | --- | --- | --- |
| 5 runs | fixed | planted | 6.9 | 0 pass, 5 fail, each `left: 4, right: 2` in about 0.26 s |
| 5 runs | fixed | correct | 6.9 | 5 pass |
| 20 runs, loaded | old, `origin/main` | correct | 19.3 to 20.1 | 19 pass, 1 fail with `left: 4, right: 3` |
| 20 runs, loaded | fixed | correct | 20.1 to 20.5 | 20 pass, 0.13 to 0.20 s each |
| 20 runs, loaded | fixed | planted | 20.5 to 20.8 | 0 pass, 20 fail, each `left: 4, right: 2` |

The review's first plant recorded the failure without calling `stop()`. The run never ended, because the row never finished. That hang comes from the harness `spawn`, which has no deadline. It is filed as `sdlc/issues/2026-09-24-a-spawned-test-run-has-no-time-limit.md`.

## Ratchet

The ceiling falls from 48812 to 48806.

## Checks

The whole ladder ran with `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset and each rung under `timeout`, at the commit that adds this record, which is the commit that lands. The merge commit on main states its result. `sdlc/scripts/live` did not run.
