# 0057: Correct help and the CLI contract

Date: 2026-09-21

Status: landed

## Result

The help now names the threshold and width defaults, shows the backend address on the first screen under ADR 0031, and explains record-mode exits. `find` says that a `none` result prints nothing and exits 3. The exit tables and command specifications agree with partial question failures, empty inputs, exact ties, and detailed rows.

A zero timeout now exits 2 with a fixed sentence before the command reads its key or input and before it opens a connection. Both ordinary commands and `find` share that check. Request bytes, successful output, record framing, answer exit codes, scheduling, and paid behavior did not change.

The source ratchet rose from 26,009 to 26,179 nonblank Rust lines. Compiled boundary tests account for the growth. The command enum moved into its own module to keep each source file below 500 lines. No dependency or duplicate production path was added.

## Review and proof

Independent design review rejected two drafts. The final ticket qualifies record completion against partial and whole-run failures, keeps `--jobs` in long help, records the `--url` exception in ADR 0031, and proves zero-timeout validation in both argument forms before input and network access. The same reviewer accepted the corrected design.

Independent code review rejected the first implementation because the warning was absent from the first `decide -h` screen and the how-to overstated exit-zero behavior. The repair pins the first screen and qualifies the example. The same reviewer accepted the final implementation with no remaining findings.

With the key and base-address variables unset, `install`, `lint`, `test`, and `spec` all exited zero. The test rung passed 172 library tests, 212 backend tests, the command edge suites, two doctests, and the shell checks. The spec rung passed 26 command examples, seven transform examples, all replay checks, and 19 green how-tos. `git diff --check` passed. No paid call ran.
