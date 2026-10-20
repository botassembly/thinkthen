# 0716: Replace the Zig fixture consumers with one packet dumper

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0666, 0667, 0688, 0740

## Outcome

- Changes: `libraries/zig/Tests/request_fixture.zig`, `libraries/zig/Tests/session_consumer.zig`, `libraries/zig/Tests/complete_parity.py`, `libraries/zig/Tests/dump.zig`, `libraries/zig/Tests/installed.py`, `libraries/zig/check.sh`, `sdlc/scripts/children`, `sdlc/scripts/coverage_installed_c_hosts.py`.
- One installed Zig consumer over the public API replaces `Tests/request_fixture.zig` and `Tests/session_consumer.zig`, which are deleted. The installed consumer runs each shared case through the language's run call and compares the answer, probability, presence and owned error fields it converted against `conformance/fixtures/packets.json`, as 0640 does for C. It does not print Rust's Call JSON back. `Tests/complete_parity.py` builds the consumer outside the repository and runs it.
- The `zig fmt` list in check.sh, `sdlc/scripts/children` and the Zig consumer names in `sdlc/scripts/coverage_installed_c_hosts.py` follow the deletion.

### Proof

- On the M5 through `pm job run --machine m5 0716 SHA --memory 8 -- sdlc/scripts/job sh libraries/zig/check.sh`, the installed consumer's comparison passes for every shared case. Before the change the routine check compiles the two fixture consumers and compares their own projections.
- Done once by hand and not committed: changing one expected value in the oracle fails the Zig comparison with a diff.
- Linux arm64: `pm job run --machine m5 0716 SHA --memory 16 -- sdlc/scripts/linux-check surface libraries/zig`; Linux x86_64: the same command with `--machine gator --memory 7`.

## Evidence

- `libraries/zig/Tests/complete_parity.py` copies `session_consumer.zig` and `request_fixture.zig` (220 lines) into a scratch folder and builds them with `zig build-exe`. `Tests/installed.py` runs it after `Tests/installed-session.py`.
- `libraries/zig/check.sh` runs `zig fmt --check` over both files. `sdlc/scripts/children` lists `libraries/zig/Tests/complete_parity.py`. `sdlc/scripts/coverage_installed_c_hosts.py` names the Zig consumer binaries `session`, `example` and `consumer`.
- 0642 defines the dumper contract and the oracle `conformance/fixtures/packets.json`. The installed consumer is written once against the thinned calls from 0667, on the darwin-arm64 gate that 0666 adds.
- Split from 0666 on 2026-10-19 so packaging and the test move land separately.
