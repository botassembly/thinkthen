# 0078 code review: release host signals

Reviewer: a fresh, read-only Claude session (Opus 5.5). Target: `origin/ticket/0078-fork-and-host-signals` at `fafbe009`, which contains main at `9f47bd18`. I read the ticket, the build record, the design review pointers in the ticket, and the repo `CLAUDE.md`. ADR 0047 does not exist. The ADRs stop at 0043, and 0047 is a ticket about monitor actions with no bearing here.

## Verdict

Findings. Three need fixing before landing. The rest of the build is sound.

## Findings to fix

1. **The `cargo tree` switch weakened a policy check** (`sdlc/scripts/policy.py`, `check_dependencies`). Main's `cargo metadata --no-default-features` graph covered every target platform and every dependency kind. The new `cargo tree -e normal` reads only the host platform and only normal edges. I planted two manifests in a scratch copy and ran `policy.py`:
   - `[target."cfg(windows)".dependencies] clap = "4.6.7"`: main fails it; the branch passes it.
   - `[build-dependencies] clap = "4.6.7"`: main fails it; the branch passes it.
   - `[target."cfg(windows)".dependencies] signal-hook = "0.3.18"`: the branch passes it.

   No other check catches these, because `check_member` pins only the `cfg(unix)` target tables and never reads `build-dependencies`. The fix: pass `-e normal,build --target all` to `cargo tree`. I applied that in scratch. The unchanged branch still passes, and all three plants fail with "the default-features-off graph excludes command dependencies". Leaving out dev edges is correct and was needed, because `signal-hook` is now a dev dependency. `CLAUDE.md` forbids weakening the policy tables, so this blocks landing.

2. **No test covers the `SIGXFSZ` exclusion from the worker mask** (`engine/workers.rs`). The ticket says "The mask excludes `SIGXFSZ`, so the host's disposition still governs a file-size signal." Planted bug A deleted `Signal::SIGXFSZ` from the exclusion list, and the full crate suite stayed green. The host proofs in `file_size_child` call `permit.finish` on the libtest thread, which is not an engine worker. The fix: run that `finish` inside `workers::scoped_observed(1, ...)`. A blocked `SIGXFSZ` on a worker then leaves the default-action child alive, and it leaves the handled child's flag unset. Both tests would turn red.

3. **No test covers "the calling thread keeps the host's mask."** Planted bug G moved `mask_host_signals()` from each worker onto the calling thread, before `thread::scope`. Workers inherit that mask, so R6-3 still passes, but the host's own thread loses every signal for good. The full crate suite stayed green. The fix: after `scoped_observed` returns in the held-send test, assert that `SigSet::thread_get_mask()` on the calling thread does not contain `SIGUSR1`. This is a one-line assertion.

## Minor finding

4. **The R6-3 test can miss the bug it guards under load.** It never goes red on correct code. I ran it 150 times at a load average of 14 to 27, beside three full library runs, with 0 failures. With the mask removed, though, it went red 30 of 30 times at rest and only 54 of 60 times at a load average of 30. Under load the signal sometimes lands before the worker enters its socket read. The handler then runs outside `read`, and no `EINTR` happens. A fix: send `SIGUSR1` several times across the 100 ms hold, for example five sends 20 ms apart, before the release. This does not block landing. Fix it together with finding 3, because both touch the same test.

## The builder's unforeseen choices

- **(a) nix `pthread` and `signal-hook` for tests only: accept.** `Cargo.lock` and `deny.toml` did not change, so no crate entered the tree. The workspace uses resolver 3, and `cargo tree -e normal,features --no-default-features -i nix` shows only `signal` (plus `process`, which `signal` pulls in) on the library. `pthread` does not reach the shipped graph. `policy.py` pins the Unix dev table and nix's exact feature list, and the ticket's own acceptance names nix's `pthread_kill`.
- **(b) The mask leaves `SIGXFSZ`, `SIGPIPE`, and the fault signals open: accept.** Blocking a synchronous fault is undefined, and the kernel kills the process anyway. `SIGSYS` covers seccomp, and `SIGTRAP` covers debuggers. `abort()` unblocks `SIGABRT` by itself. glibc strips its internal real-time signals from `pthread_sigmask`. One side effect is worth knowing: a host profiler driven by `SIGPROF` gets no samples from engine workers. That trade is acceptable. Ian or the queue owner can overturn the list, as the record says. The exclusion needs its test (finding 2).
- **(c) The claim runs for every command that reads input, but not for `status` or `cache prune`: accept.** This matches the ticket's allowance, and `transform` and `--version` return before the claim. Replay-only runs also claim it, which the ticket permits. Coverage comes from how the code is built. Planted bug E dropped the claim for `find`, and every test stayed green. The single predicate call site makes this acceptable.
- **(d) No test forces setup to fail: accept, and Ian can overturn it.** `signal_hook::flag::register` for `SIGXFSZ` cannot be made to fail without a seam built for tests. Planted bug D moved the claim after `Environment::read` and `interrupt::activate`, and everything stayed green, as the record admits. The claim is the first statement after argument parsing, so the order is easy to see in review.
- **(e) `cargo tree` in place of `cargo metadata`: reject as written.** See finding 1.

## Tests can fail

The builder's three plants are recorded in the build record. I added four more plants (A, D, E, G) and one policy plant, and ran each against the whole crate (`cargo test -p thinkthen --all-targets --all-features`):

| Plant | Result |
| --- | --- |
| `signal-hook` made a normal library dependency again | Red: `policy.py` fails three dependency rules |
| A: `SIGXFSZ` blocked on workers | Green: finding 2 |
| G: mask set on the calling thread | Green: finding 3 |
| E: `find` skips the claim | Green: accepted under (c) |
| D: claim moved after environment and interrupt setup | Green: accepted under (d) |
| Mask removed (the builder's plant), under load | Red 54 of 60: finding 4 |

I also confirmed the manifest plants from finding 1. After each plant I restored the scratch copy and checked it against `git archive fafbe009`.

## Flake check at a load of 10 or more

I held 20 busy loops on 16 cores, which put the load average at 14 to 33. The three signal tests ran 150 times with 0 failures, and three full library runs passed at the same time. The child file-size tests have no timing: `SIGXFSZ` arrives on the writing thread before the write returns. The held-send test waits at most 2 s for the worker ID and 2 s for the hold, with a 5 s client timeout. The process-wide no-op `SIGUSR1` handler affects no other test, because no other test uses `SIGUSR1`.

## No paid backend

The only URLs added are `127.0.0.1` loopback. The child test's `127.0.0.1:1` URL never sends. Keys are the literal `sk-test-value`, and the command test clears the environment. The diff does not touch `sdlc/scripts/live` or `sdlc/live-test`. I did not run either one.

## Second-agent dependency review (`CLAUDE.md` rule)

I checked these points:
- The manifest diff.
- `Cargo.lock` and `deny.toml` are unchanged.
- No crate entered or left the tree. `policy.py` resolved 98 packages and passed.
- The shipped library graph with default features off holds `nix` with `signal` only, and it holds no `signal-hook`, `clap`, or `csv-core`.
- Dev-only features stay out of the shipped graph under resolver 3.
- `policy.py` pins the new tables exactly.
- The `signal_hook` ban on engine files is in place, with its planted recorder line and its two controls.
- The graph-check rewrite has a gap (finding 1).

The ceiling rose by 229 lines, from 48140 to 48369, and `ratchet.mjs` reads 48369 of 48369. That is 189 test lines and 39 command lines, less the 25-line recorder removal, which is inside the ticket's scope. On the branch, formatting, Clippy with `-D warnings`, and `git diff --check` pass. The full crate suite passes 779 tests with 0 failures.

## Re-review at fa5d836f (main c19771a8 merged)

Verdict: ACCEPT. Every finding is fixed, and each fix goes red under a plant. I worked read-only from a `git archive` scratch copy, which I deleted afterwards.

- **Finding 1 (policy graph): fixed.** `check_library_graph` now runs `cargo tree -e normal,build --target all`. The unchanged branch passes `policy.py`, and the ratchet reads 48801 of 48801. I planted three manifests: `signal-hook` under a Windows target, `clap` under a Windows target, and `clap` as a build dependency. `policy.py` failed each one with "the default-features-off graph excludes command dependencies". The new built-in plants test the check itself. I reverted the flags to `-e normal` without `--target all`, and `policy.py` failed with "the planted graph ... is refused" for both the Windows plant and the build plant. That reversion does not get through silently. `Cargo.lock` and `deny.toml` are still unchanged.
- **Finding 2 (SIGXFSZ left unblocked on workers): fixed.** `file_size_child` now writes through `workers::scoped_observed`. Plant A deleted `SIGXFSZ` from the exclusion list, and both `a_host_sigxfsz_action_stays_installed_through_a_recording` ("the host action ran") and `a_host_with_the_default_sigxfsz_action_is_killed_by_it` went red.
- **Finding 3 (calling-thread mask): fixed.** Plant G moved the mask onto the calling thread, and `a_host_signal_during_a_held_send_on_a_worker_leaves_the_call_whole` went red.
- **Finding 4 (R6-3 deterministic): fixed.** The new check asserts that the host's `SIGUSR1` handler never ran. It does not depend on timing, because a blocked signal stays pending on the worker and ends with it. With the mask removed, the test fails with "the worker kept SIGUSR1 blocked". Under load it went red 40 of 40 times. The correct code passed all three signal tests 40 of 40 times. That run had a load average of 10.97 to 16.90, with 12 busy loops beside three parallel full-crate suites run with `--test-threads 16`. All three suites passed.
- Formatting, Clippy with `-D warnings`, and `git diff --check` pass on the branch.
