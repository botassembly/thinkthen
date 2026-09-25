# 0120: Build the Rust Polars surface

Status: built on `ticket/0120-port-rust-polars` in the surface batch; code review pending. Owner: Claude.

## Result

`thinkthen-polars` lives at `libraries/polars` as its own Cargo workspace. It adds the `PolarsEngine` trait to `thinkthen::Engine` with `decide_series`, `choose_series`, `score_series`, `tag_series`, and `annotate_frame`, and it adds one `Error` type. It uses only the public API and holds no `unsafe`. `sdlc/surfaces.txt` names it as the tenth surface, landed.

`libraries/polars/check.sh` passed on beelink on 2026-09-25: fmt, Clippy with warnings denied, and 11 tests (9 integration tests in 4 files, and the README doctest), with 0 failures. It ran offline, with a fake key and a closed loopback address.

## Pins, lock, and licenses

- `polars` and `polars-core` are pinned at `=0.55.2`, the newest 0.55 release on crates.io on 2026-09-25. The five methods need no Polars feature. `polars-core` is a dev-dependency with `dtype-categorical` and `dtype-struct`, for the R1-4 test only. `policy.py` checks that the two pins are equal.
- The lock was seeded from the root `Cargo.lock` and resolved offline. It holds 163 packages, under the budget of 200. It matches spike 257's changes: `getrandom` 0.4.3, `r-efi` 6.0.0, `signal-hook` 0.4.4, and `rand` 0.10 join the lock. serde_json's `preserve_order` stays off (`cargo tree -e features -i serde_json`), so the sorted-map plant keeps its form.
- `cargo deny` with `libraries/polars/deny.toml` passes advisories, bans, and licenses. The file equals the root file plus four exceptions, 81 nonblank lines. `cargo tree -i` at 0.55.2 confirmed each chain: `foldhash` through `hashbrown` 0.17, `slotmap` through `polars-async` and `polars-utils`, `xxhash-rust` directly under `polars-core`, and `ar_archive_writer` as a build dependency of `psm` under `stacker` under `polars-utils`. Zlib, BSL-1.0, and Apache-2.0 WITH LLVM-exception are each permissive and carry no copyleft term. Each lets anyone use, change, and ship the code with a notice kept, and none requires shared source.

## Budgets

- Production Rust: 3 files, 406 nonblank lines (limit 4 and 450).
- Rust tests: 5 files, 746 nonblank lines (limit 8 and 900).
- `check.sh`: 21 nonblank lines (limit 80).
- Gate changes: 29 nonblank lines in `policy.py` (limit 30).
- Documentation: about 72 net nonblank lines across the README, the ADR 0047 amendment, `rust.md`, and `polars-plan.md` (limit 180).
- `libraries/polars/ratchet.json` equals the measured 1152. The root ratchet does not change.

## Planted failures

Each plant ran alone, turned its test red, and was removed. The test then passed again.

| Row | Plant | Red result |
| --- | --- | --- |
| R1-24 | `score_series` passes `CallOptions::new()` in place of the caller's options | `deadline`: the call returned `Ok` after 4.03 s |
| R1-3 | Read the first chunk for every chunk | `door`: the sliced three-chunk column gave the wrong answers |
| R1-4 | Cast the caller's `Categorical` column to `String` | `door`: the dtype check failed |
| R2-28 | Run the probe with an empty `CARGO_HOME` | `check.sh` printed "not run" and exited 77 |
| R2-28 | Add a failing test | `check.sh` exited 101 and printed no "not run" |
| Throttle | Loop `decide` per row in `decide_series` | `throttle_equality`: the series took about 20 s against the slice's 4 s |
| Throttle | Loop one `annotate_with` per row in `score_series` | `throttle_equality`: the held score column did not reach 8 in flight |
| Caller's engine | Call `default_engine()` in `decide_series` | `throttle_equality`: the call failed on the closed address |
| Shared cases | Map not sure to `false` | `cases`: the band cases differed from the slice form |
| Failed marker | Write `null` for a failed cell | `door`: the team cell was null |
| Failed marker | Parse the member and write it back through a sorted map | `door`: the marker read `cause` before `kind` |
| Failed decide row | Write a failed row as null and keep going | `door`: the call returned `Ok` |
| Refusals | Drop the null check | `door`: the null column reached the engine |
| Refusals | Drop the kind check | `door`: the decide question reached the engine |
| Paid backend | Delete the `unset` and fake-key lines, run with a sentinel key | all 9 engine tests failed in the helper before any engine was built |
| Policy | A fifth license exception in `deny.toml` | `policy.py`: the deny difference failed |
| Policy | Unequal `polars` and `polars-core` pins | `policy.py`: the pin check failed |

## Findings

1. **A failed row ends a choose, score, or tag series.** The ticket expected `score_series` over `/arm/malformed/missing_probability` to widen to `String` with the marker in each cell. The engine's reply reader refuses a reply whose every answer failed (`core/adapters/systemone/response.rs`), so a one-question set never yields `Annotated::Failed`. A failed row therefore ends every series call with the engine's `Backend` error, as it ends `decide_series`. The test pins that error. Frames still widen, because a frame's other questions answer. This affects 0106's bulk `choose`, `score`, and `tag` the same way. The door changed nothing in the engine.
2. **`policy.py`'s lock-tree check fails for this binding.** Polars turns on `getrandom` 0.2's `js` feature for WebAssembly targets. The lock then records `js-sys`, `wasm-bindgen`, and 10 more packages under `getrandom` 0.2.17, which sits in `thinkthen`'s tree through `ring`. Every `thinkthen` dependency keeps its root version, but the check counts these new wasm-only packages as drift. The fix is a gate change the ticket did not authorize: compare only the packages the root tree also holds by name. The landing agent or the owner decides it.
3. **`surfaces --registry` runs deny with the root file.** It will fail this binding's licenses. The ticket says the binding's deny call uses `libraries/polars/deny.toml`. The one-line fix passes `--config "$surface/deny.toml"` when that file exists. This build did not change the ladder script.
4. **A column takes about 4 s, not 2.5 s.** On the 100 ms delay arm at throttle 8, 200 rows took about 4.05 s through both the Series and the slice. The two stayed within 5 percent of each other. The overhead sits in the engine or the backend, not the door. The test bounds the time at 2.5 to 6 s.
5. **Question sets with `on` pointers need no issue.** `QuestionSet::from_json` refuses a member with `on`, and the builder has no `on` setter. The engine already refuses such a set.

## Not measured

The cold build time on Linux and on the Mac was not measured. Spike 257 measured 59 s on Linux at 0.55.2.
