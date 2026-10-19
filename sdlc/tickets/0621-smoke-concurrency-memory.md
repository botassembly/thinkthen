# 0621: Measure and fix the Linux parity-consumer timeouts

Status: OPEN.

Milestone: 0.2.1

Phase: 1

Depends on: 0619

## Outcome

- Changes: `conformance/parity.py`, `conformance/test_parity.py`.
- The cause of each Linux timeout is measured in the capped x86_64 container on Yellow, with a memory and time sample per consumer, before any fix is called a fix.
- The parity pool in `run_consumers` schedules consumers by their measured peak memory against the container's limit. The measured peak memory and time of each consumer sit in that scheduling table, the one place the code reads them.
- The serial setting from 412d17372 stays only if the measurement supports it.

### Proof

- On Yellow, after 0619 lands: `pm job run --machine yellow 0621 SHA --memory 14 -- sdlc/scripts/linux-check test-full-cases` passes three times in a row with no timeout, each run's peak memory under the 14 GiB container cap. Hosted run 39102240049 shows the failure before the change.
- On the M5: `pm job run --machine m5 0621 SHA --memory 8 -- python3 conformance/test_parity.py` passes, with a case that the pool never runs two consumers whose measured peaks exceed the limit together.
- The hosted Linux pass on the candidate is part of 0687, not of this ticket.

## Evidence

The Linux full-case runs time out in C, Go, Zig, SQLite, C# and Rust Polars, while the same C image groups pass on the M5 in 16.10 seconds at about 4.06 GB peak. Run 39102240049 kept six failing consumer logs. C and Rust Polars exited 124 at about 902 seconds. Go, Zig, SQLite and C# exited 1. C, Go and SQLite show `image-admission-packing-overflow-decide` child timeouts at 120 seconds, and Zig also shows `body-excess-decide`. These cases include accepted 33554432-byte requests, not only refusals, so a failure label alone does not justify changing a contract or refusal assertion.

The binding smokes are not the overlap: `test-full-cases` waits for smoke before the full functional pass. The failed consumers ran in the runtime pool in `run_consumers` in `conformance/parity.py`, then four workers wide; the build preparation pool there still runs four at a time, and the logs show C overlapping Go and Zig overlapping SQLite. No Linux memory sample exists. Repair 412d17372 on main now runs the runtime lanes one at a time (`max_workers=1`). It is not proven on Linux, so the timeouts are not fixed. Duplicate descriptor serialization and extra Go fixture projections were seen, with no measured share of the time.

The failing hosted jobs ran on x86_64, so the measurement must run on x86_64. Yellow is the Linux x86_64 machine, and it runs the pinned container for ThinkThen through `sdlc/scripts/linux-check` (0619). The 0619 amendment books 14 GiB on Yellow, because it has 15 GiB of physical memory.
