Status: open. Found on the M5 on 2026-09-30 while confirming Debt 026 at commit `91878c233`. Owner: the queue owner. The release smoke runs none of these tests, so the rehearsal does not meet them.

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
