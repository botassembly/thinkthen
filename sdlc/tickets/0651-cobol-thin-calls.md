# 0651: Thin COBOL calls onto run-call and packet answers

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0688

## Outcome

- One family remains, calling the run-call entry point. Blocking answer records copy the packet's answer fields instead of projecting them.
- The streaming `TT_SESSION_*` family, including the `TT_SESSION_NEXT` loop and its private closed, reader and producer error text, is deleted.
- The COBOL binding enforces its own 8192-byte text limit and stops passing a COBOL identity to the C library, so 0690 can delete the shared cap.
- The projection code in cobol_blocking.py and the COBOL-only constructor dependency are deleted.
- Generated copybooks shrink accordingly. The README describes one family.
- Its files leave the 0637 answer-rule check's list of exceptions.

### Proof

On the M5 through pm job, the unpacked archive compiles the remaining examples from outside the repository. Their packet dump matches the 0642 oracle.

## Evidence

- COBOL generates two call families: blocking `TT_DECIDE`… and streaming `TT_SESSION_*` (libraries/cobol/native-helpers.json:3-26). Both are generated from cobol.py, cobol_native.py and cobol_blocking.py (about 900 template lines).
- The blocking family builds answer records with their own per-function projection (cobol_blocking.py, 463 lines). That is another copy of the rules.
- It depends on a COBOL-specific typed constructor, `thinkthen_session_new_typed_with_surface` (native-helpers.json:52), which the phase 2 C interface work removes.
