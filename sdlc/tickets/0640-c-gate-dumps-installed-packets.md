# 0640: Test the C library through its installed archive and the packet dump

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Depends on: 0612, 0618, 0638, 0642

## Outcome

- Changes: `libraries/c/check.sh`, `libraries/c/examples/session.c`, `libraries/c/README.md`, `libraries/c/tests/`.
- `examples/session.c` keeps calling `thinkthen_run_call` and prints the call's answer and probability read from the result, not the whole result JSON.
- A small C dumper, `libraries/c/tests/c/dump.c`, implements the 0642 packet-dump contract over the installed archive and replaces the C-specific fixture expectations. This ticket owns the C dumper; 0642 proves the oracle with the Rust consumer.
- Door tests in `libraries/c/tests/door/` and C programs in `libraries/c/tests/c/` that only repeat what the dumper proves through the installed archive are deleted. Tests that prove memory ownership, threading and cancellation at the ABI stay.
- The C gate has no exit-77 path except platforms the `sdlc/tools.json` manifest marks unsupported.

### Proof

- On the M5 natively: `pm job run --machine m5 0640 SHA --memory 8 -- sh -c 'out=$(mktemp -d) && sh sdlc/scripts/release-pack aarch64-apple-darwin "$out" c && THINKTHEN_ARTIFACT="$(ls "$out"/thinkthen-c-*.tar.gz)" sh libraries/c/check.sh'`. The installed check unpacks the archive, compiles the example and the dumper from outside the repository, and matches the 0642 oracle for every shared case. Before the change no C program prints packets for the oracle, so a changed oracle value passes the C gate.
- Done once by hand and not committed: changing one expected value in the oracle `conformance/fixtures/packets.json` makes the same run fail with a diff.
- Linux runs at the 0619 container checkpoint.

## Evidence

- `libraries/c/check.sh` runs `tests/test_installed_consumers.py`, then with `THINKTHEN_ARTIFACT` set runs `tests/installed-session.py` on the archive and stops. Without it, it runs `generate-c-header.py --check`, `check-c-exports.py`, the crate's `cargo test` (the Rust door tests in `libraries/c/tests/door/*.rs`, about 1,400 lines), and compiles `examples/session.c` against the debug build.
- `tests/installed-session.py` and `tests/installed_cases.py` run the JSON session and plan cases against an extracted archive with the loopback `Backend`. `tests/c/` holds C programs such as `session.c`, `session_cases.c`, `run_call.c` and `error_views.c`.
- 0638 landed `thinkthen_run_call` and the `thinkthen_call_result` and `thinkthen_call_error` owners, and `examples/session.c` already calls `thinkthen_run_call`; it prints `thinkthen_call_result_json` whole.
- `sdlc/scripts/release-pack` `part_c` packs `thinkthen-c-VERSION-TARGET.tar.gz` with the header, both libraries and a `.pc` file.
- The Darwin linking defect in the source check, where `check.sh` copies `libthinkthen_c.so`, belongs to 0641, which also claims `libraries/c/check.sh`.
