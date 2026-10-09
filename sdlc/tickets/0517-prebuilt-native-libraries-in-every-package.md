# 0517: Design how every package carries its native library

Status: OPEN.

Milestone: 0.2

Reviews: revision 4cd756859, reject

Reviews: revision c79e3f65e05eb4f12fa46d8f7ab003d477258c9c, accept

Reviews: revision 4d59e7c6a, accept

Reviews: revision a087f6dc3, accept

## Outcome

One reviewed package design says, for each language package, which platform and architecture pairs it supports, its runtime floor, its native dependencies, and how it carries and loads its prebuilt native library. Every migration builds its package to this design, so no caller sets a library path by hand.

## Evidence

- Starts from: [the 2026-10-09 binding decision](../decisions/2026-10-09-thin-first-class-bindings.md) and findings 5 and 7 of [the surface assessment](../records/0521-surface-contract-assessment.md). The C-interface packages ship without the library. Java builds with `--enable-preview --release 21` in `libraries/jvm/build.sh`. Flutter sets `publish_to: none` and bundles no platform libraries.
- Keeps: The existing release target definitions as the starting point. The release hold.
- Changes: Record the design once in the existing package definitions and package documentation. Do not infer that every language supports every operating system. Settle before the affected migration starts:
  - .NET runtime versions and NuGet runtime folders for 0516.
  - The stable JDK floor and Maven native-jar layout for 0504.
  - Flutter's actual plugin targets and pub packaging for 0522.
  - Swift's Apple binary target strategy for 0523 and the Apple package for 0518.
  - Go module and CMake package layouts for 0524 and 0525, and PHP's library location for 0526.
  - The C archive with header and prebuilt library, and the Zig package, for 0505.
  - The Ada and COBOL packages with their prebuilt library for 0528 and 0529.
  Name each file the design changes before editing it.
- Proof: A fresh review accepts the design against the existing target definitions and the guide. Each migration's installed package check later proves its part.
- Defers: Each package's implementation belongs to its migration: 0516 NuGet, 0504 Maven, 0522 Flutter and pub, 0523 SwiftPM, 0524 Go, 0525 CMake, 0526 PHP, 0518 Apple, 0505 C and Zig, 0528 Ada and 0529 COBOL. The shared inventory goes to 0501 and final distribution assembly to 0530. Native platform qualification goes to 0383–0385 at the first authorized candidate.

## Progress

- 2026-10-09 started
