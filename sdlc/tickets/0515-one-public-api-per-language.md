# 0515: Remove dead binding code and mark the frozen C exports

Status: OPEN.

Milestone: 0.2

Reviews: revision 4cd756859, reject

Reviews: revision c79e3f65e05eb4f12fa46d8f7ab003d477258c9c, accept

Reviews: revision ec618605c3dc8027da68bd7da9a200224702b8b3, accept

Reviews: revision 4d59e7c6a, accept

## Outcome

Packages ship no dead, unshipped or demo code. The frozen 0.1 C symbols stay and are documented as compatibility exports, separate from the recommended 0.2 API.

## Evidence

- Starts from: [the 2026-10-09 binding decision](../decisions/2026-10-09-thin-first-class-bindings.md). The audits found unused or unshipped code in PHP `src/complete/`, C++ `include/thinkthen/complete.hpp`, Swift `Sources/ThinkThen/Complete.swift`, Zig `src/complete.zig`, most of Dart `lib/src/complete/` and the probe code in `lib/src/door.dart`, and demo `main` functions in the Kotlin and Scala jars. The C header has 90 functions across five generations.
- Keeps: Every frozen 0.1 C symbol, signature, layout, error code and accepted legacy JSON-door behavior under ADRs 0101 and 0125. C preservation does not depend on finding a current caller. Every installed example and consumer that uses live code.
- Changes: Slices:
  - Kotlin and Scala demos: landed. The demos left `libraries/jvm/kotlin/KotlinCaller.kt` and `libraries/jvm/scala/ScalaCaller.scala` and now live only in installed tests.
  - Dead code: remove the unshipped or unused files above. Confirm each is unreachable from the shipped package and its installed checks before deleting it, and claim its exact files per slice.
  - Frozen C exports: document the compatibility exports apart from the recommended API in `libraries/c/README.md` and the generated header comments.
- Proof: Each affected package's installed checks and examples pass after removal. `sdlc/scripts/check-c-exports.py` still finds every frozen symbol. Public symbol checks compare against the declared contract, not a list derived from the implementation. Record handwritten code removed in the landing record.
- Defers: Removing each language's old public names after replacement parity moves to that language's migration ticket (0494–0498, 0504, 0516, 0518, 0522–0529), with a short old-to-new mapping in its README. The cross-surface upgrade guide goes to 0467.

## Progress

- 2026-10-09 started
- 2026-10-09 landed 2ca061f5172e01e924f6aeec0c73649b4e6a4e1f; next: Kotlin and Scala demos now live only in installed tests; facade APIs remain. Affected package, installed consumer and cancellation checks pass. Full JVM Matrix exposed a separate frozen legacy annotation admission defect now being repaired; broader API removals follow language migrations.
