# 0517: Ship the native library inside every package

Status: OPEN.

Milestone: 0.2

## Outcome

Every language package installs with its prebuilt native library for the supported platforms. No caller sets a library path by hand. Java runs on current JDKs without preview features. Flutter ships as a real FFI plugin.

## Evidence

- Starts from: [the 2026-10-09 binding decision](../decisions/2026-10-09-thin-first-class-bindings.md). The C-interface packages ship without the library. Java builds with `--enable-preview --release 21` (`libraries/jvm/build.sh`). Flutter sets `publish_to: none` and bundles no platform libraries.
- Keeps: The release hold. Hosted builds and publication wait for Ian's permission at the first candidate.
- Changes: NuGet runtime folders, Maven native jars, a pub native-assets or FFI plugin, a SwiftPM binary target, Go and CMake packages carrying the library. Move Java to stable foreign-function APIs. Claim packaging files per language.
- Proof: A clean container installs each package from local artifacts and runs one call with no library path set.
- Defers: Hosted and Windows qualification to the first authorized candidate.
