# test-stress writes the real usage totals

Status: open. Filed 2026-09-30 by the records Quick Fix that investigated `2026-09-30-live-batching-flake-and-unexplained-usage-calls.md`. Found by reading source on main `e8e2833e9`; nothing was run. Owner: the queue owner, as a Quick Fix. It touches only `sdlc/scripts/test-stress`, which no running lane edits.

Severity 2: a broken guarantee. ADR 0113 "Test isolation" says no test writes the real usage folder. Severity 1 means a wrong answer, data loss, a security problem or a hang. Severity 2 means a broken guarantee or a misleading document. Severity 3 means a sharp edge or a missing feature an integrator needs.

No request leaves the machine, and no real key is read. The sends go to a loopback backend under the fake key `sk-polars-loopback`.

## What happens

1. `sdlc/scripts/test-stress` sources neither `scratch.sh` nor `usage_home`, so it keeps the caller's `XDG_CACHE_HOME` and `HOME`. `sdlc/scripts/test` and `surfaces` both take `usage_guard`; `test-stress` does not.
2. It exports `THINKTHEN_API_KEY=sk-polars-loopback` and `THINKTHEN_BASE_URL=http://127.0.0.1:9/v1`, then runs the `polars_throttle` target in-process with `cargo test`.
3. That target builds every engine through `crates/thinkthen/tests/polars/common/mod.rs::builder`, which calls `EngineBuilder::from_env()`. Under ADR 0113 section 2, `from_env` seeds the counters with `config::usage_path()`, the real `thinkthen-usage` folder. `cache_at` and the `base_url` override move neither the totals nor the count.
4. Each engine then adds its loopback sends, and the tokens the loopback replies report, to the real month file.

Per run, from the test source:

| Mode | Test | Loopback sends added to the real totals |
| --- | --- | ---: |
| `--check` | `a_series_runs_at_the_throttle_as_a_slice_does` | about 15 (3 + 3 on the delay arm, 3 held arms of 3) |
| `--run` | `two_hundred_series_records_match_the_slice` | about 400 (200 series + 200 slice) |

Two or three `--run` campaigns give about 1,000 sends. That matches the unexplained count on the Beatles Bench issue. The other `test-stress` selections are clean: `public_batches` builds with `Engine::builder()`, which writes no usage, and `surfaces --stress` takes `usage_guard`.

A builder who exports the fake key and runs a Polars test target directly, outside `libraries/polars/check.sh`, writes the real totals the same way. The helper's `guarded()` check admits the fake key and a loopback base and never checks the usage folder.

## Fix

- `test-stress` sources `scratch.sh` and takes `usage_guard` before its first `cargo test`, as `test` does. Then the Polars target writes the decoy, and the run fails. So the Polars cases also need their own scratch folder: take `usage_home` around the `polars_throttle` calls, as `libraries/polars/check.sh` does, and keep the guard for everything else.
- Optionally, `polars/common/mod.rs::guarded()` also refuses when the usage folder `from_env` would pick is the real one, for example when neither `XDG_CACHE_HOME` nor `HOME` sits under a scratch folder. That covers the direct `cargo test` path.
- Proof: one `test-stress --check` run leaves the real usage month file unchanged, and a planted run without the step trips the guard.
