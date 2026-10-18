# 0652: Compile the COBOL bridge without suppressing pointer warnings

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0650, 0651, 0688

## Outcome

- The generated bridge declares parameters with the types cobc's static-call prototypes use, or callers use `-fno-gen-c-decl-static-call` with matching declarations from the bridge header. No `-Wno-incompatible-pointer-types` remains in the README or checks.
- The forced `-include` of tt_session.h is replaced by a normal include path.
- The supported GnuCOBOL release range is chosen from released versions and stated. Its lowest and newest versions are both in the tool manifest and the pinned image.

### Proof

On the M5 through pm job, natively and in the pinned arm64 container, the README's compile command builds both examples with `-Werror` on each supported GnuCOBOL version, and the examples pass.

## Evidence

- The documented COBOL build passes `-A '-include src/tt_session.h -Wno-incompatible-pointer-types'` (libraries/cobol/README.md, "Build an installed caller"). The checks do the same (checks/blocking.py:109, :429; checks/native_parity.py:87).
- The generated C bridge and the prototypes cobc emits disagree on pointer types. The warning switch hides that, along with any real mismatch.
- The README targets "GnuCOBOL 4", which may still be a development branch; this needs confirming.
