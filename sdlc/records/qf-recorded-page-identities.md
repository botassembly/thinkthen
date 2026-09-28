# Recorded page identities, 2026-09-28

Base source: main `586aeb20c8d7daaa0256405a48ea178ea5ccb55d`. The product source and lockfile have no diff from the shared failed checkpoint at `86d5cff1` (`git diff --quiet 86d5cff1..HEAD -- crates/thinkthen/src Cargo.lock Cargo.toml` passed before this page edit). A narrow offline `cargo build --locked --offline -p thinkthen --bin thinkthen` reused the matching debug binary, SHA-256 `b80726d7f9864149acd8b06a0657c03c7771756886f537e1b7a0ab7856363a06`. Build output is in `target/codex-builds/qf-recorded-page-identities/build.log`. No provider call or response regeneration occurred.

## Correction

The checkpoint's `spec/result.md` and demos 14, 16 and 21 asked replay for the new default packed request. Their saved responses instead contain one record per request. Each executable replay invocation now passes `--batch 1`; demo 14's related dry run and demo 21's refusal examples use the same setting. The pages say why, without claiming the recordings prove default packing. `--details` remains where the result-shape and audit examples inspect it. Demo 16 forwards the option through its existing `run` and `triage` scripts; those scripts and the saved bytes are unchanged.

Demo 27's replay miss now compares the complete fixed diagnostic, including exit 5, the safe command context, exact missing digest and generic identity explanation. The observed command wrote zero stdout bytes and exited 5. Its raw stdout/stderr are in the same output folder. The page still clears the key and backend address, uses read-only replay and never treats the miss as a `false` answer.

## Executable proof

`target/codex-builds/qf-recorded-page-identities/run-pages.sh` pins Node 22.22.3, this worktree's compiled CLI and an unset key/address, then runs `mustmatch test` once for each changed page under the codex-7 lane lock. Script SHA-256: `4ab3b8d098e3c2f8cb421bd03eccdbbca2696e9608165bf9077419091404ca0e`. Each raw log and exit status is beside it.

The narrow build and page commands used the same isolated offline Cargo home and lock:

```sh
export THINKTHEN_HEAVY_LOCK=/run/user/1000/thinkthen-codex-7.lock
export THINKTHEN_HEAVY_LOCK_HELD=/run/user/1000/thinkthen-codex-7.lock
export CARGO_NET_OFFLINE=true RUSTC_WRAPPER= CARGO_BUILD_RUSTC_WRAPPER=
export CARGO_HOME="$PWD/target/codex-builds/shared-functional-checkpoint-2026-09-28/cargo-home"
flock -o "$THINKTHEN_HEAVY_LOCK" cargo build --locked --offline -p thinkthen --bin thinkthen
flock -o "$THINKTHEN_HEAVY_LOCK" bash target/codex-builds/qf-recorded-page-identities/run-pages.sh
```

| Page | Exit | Must-match result |
| --- | ---: | --- |
| `spec/result.md` | 0 | 1 passed |
| `demos/14-grade-a-batch/README.md` | 0 | 4 passed |
| `demos/16-triage-pipeline/README.md` | 0 | 3 passed, 1 nonexecuted block skipped |
| `demos/21-options-from-the-record/README.md` | 0 | 5 passed |
| `demos/27-test-with-no-network/README.md` | 0 | 6 passed, 1 nonexecuted block skipped |

Total: 19 executed blocks passed, zero failed, two nonexecuted blocks skipped. The prior failed checkpoint logs remain under `target/codex-builds/shared-functional-checkpoint-2026-09-28/`; this is a five-page repair, not a complete `spec` or `demos` rung claim.

An independent status-file audit found exactly five page statuses, all zero (`status-audit.log`). Offline policy checked 189 packages; `sdlc/scripts/pages` reported one coming and 22 green, `sdlc/scripts/tickets` reported zero evidence failures, and `git diff --check` passed. These static checks do not replace the executable page results.

The four claimed recording folders contain 34 files. Sorted per-file SHA-256 manifests before and after the page runs are byte-identical (`cmp` exit 0); each manifest's SHA-256 is `2a9de0480591dfeff21ac9f6205cc2e43ba2f9c6600a8571dfcdb46a3061b650`. No recording, source, site or root README file changed.

## What the build taught us

The saved answer's digest belongs to the complete request, including record grouping. When a default batch changes, make old replay examples state their original grouping and execute their expected values; syntax alone or copying a reply to the new digest would conceal a miss. A safe diagnostic can gain command context while retaining its local failure and digest. Pin its full observed line in an executable page rather than matching a stale substring.
