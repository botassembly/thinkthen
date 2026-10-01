Status: closed 2026-10-01 by the quick fix "Land quick fix: DuckDB waits for answers without a channel, so a macOS forked child answers". Found on the M5 on 2026-09-30 while confirming Debt 026 at commit `91878c233`. The fork crash was a real macOS bug in the extension; the other three were test assumptions.

Kind: bug

Severity: medium

# The DuckDB check fails four tests on macOS

## The problem

On the M5, `databases/duckdb/check.sh` builds and loads the extension, and every suite passes except four tests. Each one also fails on an extension built before the Debt 026 fix, so none comes from that change.

| Suite | Test | Failure | Cause |
| --- | --- | --- | --- |
| `settings_suite.py` | `the_exit_flushes_a_call_while_the_usage_lock_is_held` | No such file `home/Library/Caches/thinkthen-usage` | The test expects the usage folder where macOS puts it, and the run left nothing there |
| `settings_suite.py` | `two_file_rows_share_one_open_and_a_refusal_opens_none` | No such file `strace` | `strace` exists only on Linux |
| `settings_suite.py` | `a_forked_child_answers_from_a_zero_total` | The child answers `None`, not `True` | Not yet known |
| `signal_suite.py` | `r3_13_an_siginfo_host_handler_gets_the_number_and_sender` | The host handler sees no siginfo number or sender | Not yet known |

The check also loaded the extension by a relative path, which the hardened macOS DuckDB CLI refuses. The quick fix that closed Debt 026 now passes the full path.

The M5's own `cargo-deny` is too old for the check's `--config` flag. That gap belongs to the M5, not the repository.

## A fix

Give `strace` a macOS skip with its reason, or a macOS tracer. Find where the usage folder lands on macOS under the test's `HOME`. Then look into the fork and siginfo cases, which may need macOS rules of their own.

## Resolution

Reproduced on the M5 at main `d3391ee40`. Each case had its own cause.

| Test | Cause | Fix |
| --- | --- | --- |
| `the_exit_flushes_a_call_while_the_usage_lock_is_held` | The test looked in the Linux usage folder | Already fixed by ticket 0360; it passed at `d3391ee40` |
| `two_file_rows_share_one_open_and_a_refusal_opens_none` | `strace` exists only on Linux | Off Linux, the suite skips both `strace` cases and prints why |
| `a_forked_child_answers_from_a_zero_total` | A real macOS bug in the extension, below | The bridge waits on a mutex and condition variable |
| `r3_13_an_siginfo_host_handler_gets_the_number_and_sender` | The test built `struct sigaction` and `siginfo_t` in Linux layouts, so `SA_SIGINFO` never reached macOS and the extension rightly called a one-argument handler | The test uses macOS layouts and `SA_SIGINFO` 0x40 on macOS |

The fork bug: the child crashed with SIGTRAP in 7 of 10 runs. The crash report reads "BUG IN CLIENT OF LIBDISPATCH: Use-after-free of dispatch_semaphore_t" and "crashed on child side of fork pre-exec", in `std::thread::park_timeout` under the bridge's `run_detached`. On macOS, Rust parks a thread on a libdispatch semaphore whose Mach port does not survive `fork()`. DuckDB's calling thread waited for its worker through a channel, so a host that asked once and then forked crashed on the child's first wait. The case's "the child timed out" message hid the crash. The case now reports the child's wait status.

`run_detached` now waits on a `Mutex` and `Condvar`, which use pthread calls and no Mach port. The worker still reports a defect if it ends without an answer. With the fix, the fork case passed 19 of 20 runs on the M5. The one failure came in the parent before the fork: the backend closed a pooled connection, the race Debt 020 records.

Proof at `f8c47856a`, before the review simplified the answer handoff and raised the ceilings: the full `databases/duckdb/check.sh` passed on Linux (128 cases) and on the M5 (127 cases and the `strace` skip). On the M5, `cargo deny` was stubbed because that host's cargo-deny 0.20.2 lacks `--config`; it ran on Linux. One earlier M5 run failed `sixteen_held_plans_refuse_without_eviction` with 14 of 16 sends held. That case then failed 1 of 40 runs on the fix and 0 of 40 on main. It has failed the same way on main before (see the 2026-09-30 progress log in `cleanup-2026-09-30.md`). Its held sends run on the worker threads, which the fix leaves unchanged.

At the reviewed `f1c59eb79`, the full check passed again on Linux (128 cases) and on the M5 (127 cases and the `strace` skip), and the fork case passed 20 of 20 runs on the M5. `lint` passed in a clean checkout.

Other surfaces wait on channels on the calling thread too. `2026-10-01-macos-forked-children-crash-on-a-channel-wait.md` owned them; ticket 0365 closed it.
