# 0515: Keep one public API per language

Status: OPEN.

Milestone: 0.2

Depends on: 0511

Reviews: revision 4cd756859, reject

## Outcome

The review amendment below preserves frozen C compatibility independently of whether a current caller is visible.

Each language has one obvious entry point. The 0.1 JSON-string calls, duplicate APIs, unused code and demo programs are removed.

## Evidence

- Starts from: [the 2026-10-09 binding decision](../decisions/2026-10-09-thin-first-class-bindings.md). Python has five entry points. The C header has 90 functions across five generations. Unused or unshipped code includes PHP `src/complete/`, C++ `complete.hpp`, Swift `Complete.swift`, Zig `complete.zig`, most of Dart `complete/` and the probe code in `door.dart`, and demo `main` functions in the Kotlin and Scala jars.
- Keeps: The frozen 0.1 C symbols only where a C caller needs them, documented as deprecated.
- Changes: Remove the old calls and dead code per language, and update READMEs and examples. Claim each language folder per slice, coordinated with its migration ticket.
- Proof: Each package's public symbol list matches the guide. Installed examples pass. Line counts before and after go in the record.
- Defers: None.

## Review amendment

Preserve every frozen 0.1 C symbol, signature, layout, error code and accepted legacy JSON-door behavior under ADRs 0101 and 0125. Document compatibility exports separately from the recommended 0.2 API. Remove legacy host-language APIs under Ian's ruling only after replacement installed parity. C preservation does not depend on finding a current caller.

The first slice removes shipped demo entry points from `libraries/jvm/kotlin/KotlinCaller.kt` and `libraries/jvm/scala/ScalaCaller.scala`, updates `libraries/jvm/tests/package_check.py`, and names any affected installed example before editing. Later API-removal slices follow their language migration and name exact files. Public symbol checks compare against the declared contract, not a list derived only from the implementation.
