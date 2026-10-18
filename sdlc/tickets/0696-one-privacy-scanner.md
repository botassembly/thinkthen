# 0696: Share one privacy scanner across the library checks

Status: OPEN.

Milestone: 0.2.1

Phase: 1

Reviews: revision 9dd3407, accept

Reviews: revision cbec1bf, accept

## Outcome

- One privacy scanner lives in the shared test tools, and every library and database check calls it.
- Every copy is deleted. Any rule that only one copy had is kept in the shared scanner if it still guards something real, and dropped otherwise.

### Proof

- On the M5 through pm job, `sdlc/scripts/test` and each library gate that used a copy pass with the shared scanner.
- `git grep` finds one privacy scanner under `libraries/`, `databases/`, `conformance/` and `sdlc/scripts/`.

## Evidence

0564's review counted nine near-identical copies of a privacy scanner that checks public sources and packages for private text. Three are files named `checks/privacy.py` in `libraries/ada`, `libraries/cobol` and `libraries/dart`, and they already differ. The lane confirms the rest, starting with the fixture guards such as `libraries/cpp/fixtures/guard.py` and `libraries/go/fixtures/guard.py`. A fix to one copy does not reach the others. 0564 found the copies and planned one shared scanner with the first language lane; with the 0.2.1 phases, it belongs in the test machinery before any language work.

## Progress

- 2026-10-18 started
- 2026-10-18 next: The shared scanner is held outside the clean closing batch pending a narrow ruling on synthetic paths and public TLS test keys shipped in the Rust crate; no scanner exception or package membership change has been made.
- 2026-10-18 next: The exact path-and-rule fixture allowances and archive coverage repairs are accepted at cbec1bfc4, with nine privacy tests and the real crate plus 1041 unpacked files passing; integrated routine gates remain.
