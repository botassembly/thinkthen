# 0692: Add a runtime version check and stop the C interface special-casing COBOL

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Depends on: 0638

Reviews: revision 1f5b049, accept

## Outcome

- An exported function returns the library's version and interface number. The header documents how a host checks it once at load and refuses a mismatch with a clear error.
- The run call from 0638 and every function added in phase 2 take no caller language identity and change no behavior by language. Usage-total surface labels, if kept, change nothing else.
- COBOL enforces its own 8192-byte limit in 0651. 0690 then deletes the cap and `THINKTHEN_COBOL_OVERFLOW` from the shared interface.

### Proof

- On the M5 through pm job, loading a library whose interface number differs from the header's makes the C consumer report the mismatch before any call.
- `sdlc/scripts/check-c-exports.py` passes with the new export.

## Evidence

There is no runtime version check. The header defines `THINKTHEN_VERSION_MAJOR`, `MINOR` and `PATCH` only as compile-time macros. Go pins them at compile time (`libraries/go/thinkthen.go:11`). A host that loads a mismatched library finds out only through misbehavior.

The shared interface also special-cases one language. `libraries/c/src/ffi/native_request/ffi.rs:75` caps each text value at 8192 bytes when the caller says it is COBOL. `THINKTHEN_COBOL_OVERFLOW` (`libraries/c/src/ffi/values.rs:143-144`, `libraries/c/src/failures.rs:56-57`) is defined in the header every language reads.

## Progress

- 2026-10-19 started
