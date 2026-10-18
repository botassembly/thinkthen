# 0640: Test the C library through its installed archive and the packet dump

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Depends on: 0612, 0618, 0638, 0642

## Outcome

- examples/session.c calls the run-call entry point and reads the packet's answer, probability and failure fields directly.
- A small C dumper implements the 0642 packet-dump contract over the installed archive and replaces C-specific fixture expectations. This ticket owns the C dumper; 0642 proves the oracle with the Rust consumer.
- Door tests that only repeat what the dumper proves through the installed archive are deleted. Tests that prove memory ownership, threading and cancellation at the ABI stay.
- The gate has no exit-77 path except platforms the 0615 manifest marks unsupported.

### Proof

On the M5 through pm job, the gate builds the archive, unpacks it, compiles the example and dumper from outside the repository, and matches the 0642 oracle for every shared case.

## Evidence

- The C library's main checks are Rust integration tests inside the crate (libraries/c/tests/door/*.rs, about 1,300 lines) plus C files compiled against the source tree (libraries/c/check.sh:41-58).
- The installed archive is exercised only with THINKTHEN_ARTIFACT set (check.sh:36-39) and by tests/installed-session.py.
- examples/session.c reads answers by walking packet views. When the core adds the answer and probability to the packet and a single run-call entry point, the example and tests should use them.
