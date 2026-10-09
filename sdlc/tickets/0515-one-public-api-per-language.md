# 0515: Keep one public API per language

Status: OPEN.

Milestone: 0.2

Depends on: 0511

## Outcome

Each language has one obvious entry point. The 0.1 JSON-string calls, duplicate APIs, unused code and demo programs are removed.

## Evidence

- Starts from: [the 2026-10-09 binding decision](../decisions/2026-10-09-thin-first-class-bindings.md). Python has five entry points. The C header has 90 functions across five generations. Unused or unshipped code includes PHP `src/complete/`, C++ `complete.hpp`, Swift `Complete.swift`, Zig `complete.zig`, most of Dart `complete/` and the probe code in `door.dart`, and demo `main` functions in the Kotlin and Scala jars.
- Keeps: The frozen 0.1 C symbols only where a C caller needs them, documented as deprecated.
- Changes: Remove the old calls and dead code per language, and update READMEs and examples. Claim each language folder per slice, coordinated with its migration ticket.
- Proof: Each package's public symbol list matches the guide. Installed examples pass. Line counts before and after go in the record.
- Defers: None.
