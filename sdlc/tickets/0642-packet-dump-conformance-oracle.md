# 0642: Compare every library against one core packet dump

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Depends on: 0612, 0636

## Outcome

- The core writes one canonical, deterministic dump of each shared case's packets: rows, aggregates, terminal, answer, probability, pick, resolution, presence and facts. `conformance/fixtures/complete.json` is that dump. The core regenerates it, and a check in `lint` fails when it is not current. It is the one packet oracle.
- A library's dumper reads each case's request, calls the library's public API, and prints the packet JSON that Rust returned, plus only the values, presence states and errors the host itself converted. It does not re-serialize every public type in its own code, so a new core field touches no language dumper.
- `conformance/children` compares parsed JSON against the oracle, with numbers kept as decimal strings, not byte-for-byte text. Language-specific expectations in `native_fixture.py` are deleted.
- The dumper contract is documented in `libraries/BINDING-AUTHOR.md`.
- Each language adopts the dumper in its own phase 3 gate or packaging ticket, which deletes its old fixture consumer. The C dumper is 0640's.

### Proof

- On the M5 through pm job, the Rust library consumer's dump matches the oracle for every shared case.
- The currency check passes on main.
- Done once by hand and not committed: changing one expected value in the oracle fails the comparison with a diff.

## Evidence

Each language keeps a large fixture consumer that projects results its own way for `conformance/children/native_fixture.py` (389 lines):
- Go: `libraries/go/fixtures/owned_consumer.go` (678 lines) and `full_consumer.go` (278).
- C++: `libraries/cpp/fixtures/owned/main.cpp` (305) and `owned_calls.py` (256).
- Zig: `libraries/zig/Tests/request_fixture.zig` (220).
- Ada: `libraries/ada/checks/full_case.adb` (226).
- COBOL: `libraries/cobol/checks/blocking.py` (445).

Fixture changes fan out to every language. Since 2026-09-15, 97 commit subjects across the nine scoped libraries mention fixtures. Examples: 6ee099210 changed 54 files, and 9f04b5c7a and 1a4b68b02 touched C, C++, Zig, Ada and COBOL. The shared fixture `conformance/fixtures/complete.json` is read today only by C# and Objective-C.
