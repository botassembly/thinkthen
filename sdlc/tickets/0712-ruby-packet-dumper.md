# 0712: Replace the Ruby fixture consumers with a packet dumper

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0642, 0683, 0688

## Outcome

- Changes: `libraries/ruby/tests/`, `libraries/ruby/check.sh`.
- A Ruby dumper in `libraries/ruby/tests/` follows the 0642 contract: it reads each shared case's request, calls the installed gem's public API, and prints the packet JSON that Rust returned plus only the values, presence states and errors Ruby itself converted.
- `check.sh` compares the dumper's output against the packet oracle `conformance/fixtures/packets.json` through the 0642 comparison in `conformance/children`, in place of `tests/native_cases.py`.
- `tests/native_cases.py`, `tests/native_case.rb` and `tests/test_owned_session.rb` are deleted. Tests in `tests/test_owned_session.rb` that prove ownership, cancellation or Ruby-only conversion and that the dump cannot show move to the dumper's check first.

### Proof

- On the M5: `pm job run --machine m5 0712 SHA --memory 8 -- sh -c 'cargo build --locked --offline -p conformance-backend && sh libraries/ruby/check.sh'`. The installed gem's dump matches the oracle for every routine case.
- Done once by hand and not committed: changing one expected value in the oracle fails the Ruby comparison with a diff.

## Evidence

- Split from 0684 on 2026-10-19 so the dumper waits only on 0683 and 0642, not on the 0619 image.
- `libraries/ruby/check.sh` runs `tests/test_owned_session.rb`, `tests/test_examples.rb` and `python3 libraries/ruby/tests/native_cases.py` after `gem install --local`. `native_cases.py` drives `native_case.rb` once per step through `session_cases.native_cases` and `descriptor` in `conformance/children/session_cases.py`.
- 0642 makes `conformance/fixtures/packets.json` the one packet oracle and says each language adopts the dumper in its own phase 3 ticket.
