# Shared functional checkpoint, 2026-09-28

Source: integrated main `86d5cff12935b2c549466c478cc9246f9c91f4a3` (0223, 0252, 0253 and 0254). This is a failed checkpoint, not a passing core gate. Runtime, tests, fixtures, public pages and plan were not edited. No test was excluded or retired.

## Reproduction boundary

I read `sdlc/scripts/test` and `sdlc/scripts/spec` at this SHA, then ran each complete rung **once**. The host had 16 CPUs, load about 1.18, 22 GiB available memory and 150 GiB free disk before the build. Rust/Cargo 1.95.0 and Node 22.22.3 were selected. Both invocations used the codex-7 `flock -o` lock, this worktree's `target/`, empty `RUSTC_WRAPPER` and `CARGO_BUILD_RUSTC_WRAPPER`, and no inherited key or backend URL. `sdlc/scripts/allow-list` removes `CARGO_NET_OFFLINE` and `CARGO_TARGET_DIR`; an isolated `CARGO_HOME` with `net.offline = true` and links to the retained local registry/git cache kept Cargo offline, while the rung's own default target stayed in this worktree. No provider, stress, full-case or host-surface campaign ran.

These were the rung commands after creating the isolated Cargo home by copying the existing Cargo configuration, appending `[net] offline = true`, and linking the retained registry/git cache into it:

```sh
unset THINKTHEN_API_KEY THINKTHEN_BASE_URL
export THINKTHEN_HEAVY_LOCK=/run/user/1000/thinkthen-codex-7.lock
export THINKTHEN_HEAVY_LOCK_HELD=/run/user/1000/thinkthen-codex-7.lock
export CARGO_HOME="$PWD/target/codex-builds/shared-functional-checkpoint-2026-09-28/cargo-home"
export CARGO_TARGET_DIR="$PWD/target" CARGO_NET_OFFLINE=true RUSTC_WRAPPER= CARGO_BUILD_RUSTC_WRAPPER=
export PATH="$HOME/.nvm/versions/node/v22.22.3/bin:$PATH"
set -o pipefail
flock -o "$THINKTHEN_HEAVY_LOCK" /usr/bin/time -p sdlc/scripts/test 2>&1 | tee target/codex-builds/shared-functional-checkpoint-2026-09-28/test.log
flock -o "$THINKTHEN_HEAVY_LOCK" /usr/bin/time -p sdlc/scripts/spec 2>&1 | tee target/codex-builds/shared-functional-checkpoint-2026-09-28/spec.log
```

`THINKTHEN_HEAVY_LOCK_HELD` made each sourced rung skip its nested lock. `target/codex-builds/shared-functional-checkpoint-2026-09-28/` retains source SHA, rung hashes, isolated Cargo configuration, raw logs, narrow replay output and artifact hashes. Rung SHA-256: test `956825aecef273bb70fff9792358ab771fe87467616bc59f1ff64d27d4c8b989`, spec `5bc775d76751ac85f6cdd1f8039dbc4afd8cc8a616bf472919c544bc8142cdfc`. The built CLI hash is `b80726d7f9864149acd8b06a0657c03c7771756886f537e1b7a0ab7856363a06`.

## Routine test rung

Exit 101 after 33.26 s wall. The first Cargo selection reported 21.98 s of build/selection time, followed by 0.88 s execution of its one test; the secrecy sweep ran 9.50 s. The command-wire test passed: 54 total cases, 31 selected, 25 passed, six deliberately not run by the command wire, 23 unselected, zero failures. The routine secrecy sweep passed 266 selected paths of 518 full paths. The third selected test, `secrecy::the_key_reaches_the_authorization_header_and_nothing_else`, failed on its first `decide` row: `written(&dir).len()` was 3 versus its expected 2. Later selections in `sdlc/scripts/test` did not run. These counts describe only work reached before the stop.

The retained test folder contains exactly the backend marker (124 bytes), one answer entry (397 bytes), and an empty `.locks/<digest>` file. `engine/cache_lock.rs::opened_lock` creates that digest lock; accepted `f9c971d5` stopped unlinking a completed lock so refresh waiters keep one inode. The exact two-file assertion in `tests/backend/secrecy.rs:432` predates that change. The test's `nothing_leaked` walk already visits all files, including `.locks`; the stale count stops it before that assertion. The smallest repair is to pin the actual three-file structure (including the empty lock) and retain the authorization-header and full secrecy checks, then run that single test before another whole rung. This is an inherited test expectation, not a new 0253 marker or observed key leak.

## Spec rung

Exit 1 after 9.10 s total wall. `cargo build --quiet` does not report its separate build duration; the 9.10 s includes build, settings checks and executable-page work. Settings self-test passed 10/10 and the settings table passed 51 rows, 56 flags, six environment names and 15 question-file keys. `mustmatch test spec` then reported 55 passed, one failed and one skipped, stopping at `spec/result.md`'s `Result shapes` bash block. Its demo 14 `annotate checks.json --jsonl --details --replay recording --input cases.jsonl` asked for packed records 1–6, group 1 with four members, digest `afeeaca00add0c41b92992a69c0d5fb20d3dd2f54685ed8446a933f81c99f818.json`; the retained recording has no such entry. No later spec pages, probes or demos ran.

Accepted annotate batching `a5197737` made the default `max` group records; demo 14's saved recording predates that request. The one narrow offline replay of the **same** demo command with `--batch 1` exited 0, produced six result rows and empty stderr against the existing fixture. Its output and stderr remain in the artifact folder. `spec/annotate.md` already pins `--batch 1` for its replay. The smallest documentation-owner repair is to make `spec/result.md` and demo 14's shown replay use the retained singleton identity, or obtain independently reviewed current packed responses if the default batch must be demonstrated. Do not copy a response onto a different request. This is a page/fixture identity gap, not evidence that the packed runtime failed.

## Next checkpoint

Root should claim the secrecy assertion for a focused exact test and route the replay wording/fixture decision to the public-documentation owner. After both repairs land, run the routine test and spec rungs once on that new pinned integrated SHA; the portions after these stops remain unmeasured. Root separately owns TypeScript, Ruby and DuckDB host checks. The pending policy parsing optimization was not awaited or included in this source revision.
