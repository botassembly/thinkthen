# 0621: Measure and fix the Linux parity-consumer timeouts

Status: OPEN.

Milestone: 0.2.1

Phase: 1

Depends on: 0619

## Outcome

- The cause of each Linux timeout is measured in the capped x86_64 container on Yellow, with a memory and time sample per consumer, before any fix is called a fix.
- The measured peak memory and time of each smoke and image group is recorded beside the case list.
- The parity pool schedules consumers by measured memory against the machine's limit, and the serial setting from 412d17372 stays only if the measurement supports it.

### Proof

- On Yellow through pm job with `--memory 16`, `test-full-cases` passes three times in a row in the capped x86_64 container with no timeout.
- The recorded peak memory of each run stays under the container's cap.
- The hosted Linux pass on the candidate is part of 0687, not of this ticket.

## Evidence

The Linux full-case runs time out in C, Go, Zig, SQLite, C# and Rust Polars, while the same C image groups pass on the M5 in 16.10 seconds at about 4.06 GB peak. Run 39102240049 kept six failing consumer logs. C and Rust Polars exited 124 at about 902 seconds. Go, Zig, SQLite and C# exited 1. C, Go and SQLite show `image-admission-packing-overflow-decide` child timeouts at 120 seconds, and Zig also shows `body-excess-decide`. These cases include accepted 33554432-byte requests, not only refusals, so a failure label alone does not justify changing a contract or refusal assertion.

The binding smokes are not the overlap: `test-full-cases` waits for smoke before the full functional pass. The failed consumers ran in `conformance/parity.py`'s pool (`ThreadPoolExecutor(max_workers=4)`), and the logs show C overlapping Go and Zig overlapping SQLite. No Linux memory sample exists. Repair 412d17372 on main now runs parity consumers one at a time. It is not proven on Linux, so the timeouts are not fixed. Duplicate descriptor serialization and extra Go fixture projections were seen, with no measured share of the time.

The failing hosted jobs ran on x86_64, so the measurement must run on x86_64. Yellow is the Linux x86_64 machine, and it runs the pinned container for ThinkThen (0619).
