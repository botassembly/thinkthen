# 0713: Replace the R fixture consumers with a packet dumper

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0681, 0682, 0688

## Outcome

- Changes: `libraries/r/tests/`, `libraries/r/check.sh`.
- An installed R consumer in `libraries/r/tests/` calls the installed package's public functions. The installed consumer runs each shared case through the language's run call over 0689's value tree and compares the converted answer, probability, presence and error fields against `conformance/fixtures/packets.json`. It does not print Rust's Call JSON back.
- `check.sh` runs that consumer in place of the `native_fixture.run('r', ...)` call over `tests/native_case.R`.
- `tests/native_case.R` and the R fixture scripts whose cases the consumer now covers are deleted. Scripts that prove R-only behavior the consumer cannot show stay: interrupts, forked children, hooks, startup, named backends and feeds.

### Proof

- On the M5: `pm job run --machine m5 0713 SHA --memory 8 -- sh -c 'cargo build --locked --offline -p conformance-backend && sh libraries/r/check.sh'`. The installed consumer's comparison passes for every routine case.
- Done once by hand and not committed: changing one expected value in the oracle fails the R comparison with a diff.

## Evidence

- Split from 0681 on 2026-10-19 so the R packaging work does not wait on 0682.
- `libraries/r/check.sh` runs `native_fixture.run('r', ...)` from `conformance/children/native_fixture.py` over `tests/native_case.R`, then about 20 R scripts through `tests/with-backend.sh`, then `tests/failed_receipt.py`, `tests/request_surface.py`, `tests/record_feeds.py` and `tests/feed_cases.py`. `native_fixture.py` carries R-only branches for `01-decide-yes-captured`, `19-find-none` and `files-relate`.
- 0642 makes `conformance/fixtures/packets.json` the one packet oracle and says each language adopts the dumper in its own phase 3 ticket.
