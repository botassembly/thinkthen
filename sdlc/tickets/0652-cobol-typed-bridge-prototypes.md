# 0652: Compile the COBOL bridge without suppressing pointer warnings

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0650, 0651, 0688

## Outcome

- Changes: `build-tools/results/templates/cobol*.py`, `libraries/cobol/src/`, `libraries/cobol/checks/`, `libraries/cobol/check.sh`, `libraries/cobol/README.md`, `libraries/cobol/README-PUBLIC.md`, `conformance/children/usage_installed.py`, `conformance/library-surface-test.py`, `sdlc/tools.json`, `site/src/data/build-lines.mjs`.
- The generated bridge declares parameters with the types cobc's static-call prototypes use, or callers use `-fno-gen-c-decl-static-call` with matching declarations from the bridge header. No `-Wno-incompatible-pointer-types` remains in the README, the checks or the site's build line.
- The forced `-include` of `src/tt_session.h` becomes a normal include path.
- The supported GnuCOBOL range is chosen from released versions and stated in the README. Its lowest and newest versions are both in `sdlc/tools.json`.
- The COBOL build line in `site/src/data/build-lines.mjs` matches the README's command, and the site's COBOL samples replay with it.

### Proof

- On the M5 through `pm job run --machine m5 0652 SHA --memory 8 -- sh libraries/cobol/check.sh`, the README's compile command builds the example and the dumper with `-Werror`, and they pass. Before the change, the same command without `-Wno-incompatible-pointer-types` fails with pointer-type errors.
- At the 0619 container checkpoint, the release suite runs the COBOL gate with both the lowest and newest GnuCOBOL versions.

## Evidence

- `libraries/cobol/README.md` ("Compile the new caller") passes `-A '-include src/tt_session.h -Wno-incompatible-pointer-types'`. The checks and `conformance/children/usage_installed.py` do the same, and `site/src/data/build-lines.mjs` prints the same flags for every COBOL sample.
- The generated C bridge (`src/tt_requests_generated.c`, `src/tt_session.c`) and the prototypes cobc emits disagree on pointer types. The warning switch hides that, along with any real mismatch.
- `sdlc/tools.json` pins `cobc` at `4.0-early-dev.0`, a development snapshot; GnuCOBOL 4 has no release. The newest GnuCOBOL release is in the 3.x line, so the range comes from 3.x releases unless the bridge needs a 4.0 feature, which the developer then names.
- `conformance/library-surface-test.py` plants changes in `src/tt_session.h`.
