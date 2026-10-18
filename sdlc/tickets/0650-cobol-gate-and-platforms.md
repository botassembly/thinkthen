# 0650: Replace COBOL fixture drivers and cover its supported platforms

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0615, 0618, 0619, 0688

## Outcome

- One COBOL packet dumper program over the public copybooks, following the 0642 contract, replaces the per-case drivers, which are deleted. Python only feeds cases and compares dumps.
- COBOL adds darwin-arm64 and linux-arm64 to its supported targets, beside linux-x86_64, in the 0615 manifest. The archive carries the native library for each, and the gate runs on each.

### Proof

Through pm job: natively on the M5 (darwin-arm64), in the pinned arm64 container on the M5 (linux-arm64), and in the pinned x86_64 container on Yellow (linux-x86_64). On each, the gate unpacks the archive, compiles the dumper outside the repository, and matches the 0642 oracle for every shared case.

## Evidence

- libraries/cobol/check.sh:5 exits 77 off Linux x86-64, and :7-9 exits 77 when cobc, cc, clang, nm, python3, tar or node is missing.
- Conformance runs through Python drivers that compile and run COBOL per case: checks/blocking.py 445 lines, session.py 227, session_records.py 162, native_parity.py 120, owned_reader.py 133.
- Fixture repairs recur: 62ce94b1a, 89c57312f, fb1aff74f.
