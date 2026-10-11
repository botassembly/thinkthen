# 0537: Remove the remaining old names and stale wording

Status: OPEN.

Milestone: 0.2

Reviews: revision fe9bf662f0234e8177eecd2a0f0adea96b36e5d0, accept

## Outcome

No shipped package, script or document uses the retired "door" names. The JVM jar, package and classes carry current names. Every migrated language's README maps its old calls to the new ones. The public docs describe current behavior only.

## Evidence

- Starts from: gap 3 of [the 0.2 closure review](../records/2026-10-11-0-2-closure-review.md) and the done rule in [the 2026-10-10 ruling](../decisions/2026-10-10-drive-0-2-to-done.md).
- Keeps: the frozen 0.1 C exports and their documentation. Every current public call.
- Changes:
  - JVM: rename the `door` module and `thinkthen-door.jar`. Delete `jvm/door/thinkthen/Json.java` if the generated readers cover it, or move it under the session package. Update `jvm/pom.xml`, `jvm/ratchet.java.json` and `sdlc/scripts/release-managed-pair-self-test.py:127` to the current classes.
  - COBOL: add an old-to-new table to `libraries/cobol/README.md` for the removed calls and carriers, and remove "in this slice".
  - Wording: replace "door" in `swift/Sources/CThinkThen/include/thinkthen.h`, `go/thinkthen.go`, `cpp/include/thinkthen/json.hpp` and any other shipped file a search finds.
  - Docs: rewrite the Foundation sentence in `libraries/UPGRADING-0.2.md` to describe the API and drop its test status. Each language entry on the site links its package README, which holds the checked example. This replaces the 15 site examples that 0467 deleted.
- Proof: a search of shipped source, package metadata and docs finds no retired name outside the frozen C section. The routine installed JVM checks and the release script self-tests pass. The site build and its link check pass.
- Defers: nothing.
