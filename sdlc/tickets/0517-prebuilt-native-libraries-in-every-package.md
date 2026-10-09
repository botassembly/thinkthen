# 0517: Ship the native library inside every package

Status: OPEN.

Milestone: 0.2

Reviews: revision 4cd756859, reject

Reviews: revision c79e3f65e05eb4f12fa46d8f7ab003d477258c9c, accept

Reviews: revision 4d59e7c6a, accept

## Outcome

The review amendment below distinguishes local packaging from native platform qualification and release actions.

Every language package installs with its prebuilt native library for the supported platforms. No caller sets a library path by hand. Java runs on current JDKs without preview features. Flutter ships as a real FFI plugin.

## Evidence

- Starts from: [the 2026-10-09 binding decision](../decisions/2026-10-09-thin-first-class-bindings.md). The C-interface packages ship without the library. Java builds with `--enable-preview --release 21` (`libraries/jvm/build.sh`). Flutter sets `publish_to: none` and bundles no platform libraries.
- Keeps: The release hold. Hosted builds and publication wait for Ian's permission at the first candidate.
- Changes: NuGet runtime folders, Maven native jars, a pub native-assets or FFI plugin, a SwiftPM binary target, Go and CMake packages carrying the library. Move Java to stable foreign-function APIs. Claim packaging files per language.
- Proof: A clean container installs each package from local artifacts and runs one call with no library path set.
- Defers: Hosted and Windows qualification to the first authorized candidate.

## Review amendment

Install local artifacts in clean environments on each supported platform and architecture, without user library-path settings, source-checkout fallback or a separately installed ThinkThen library. Verify loader selection and inventory. Linux containers do not prove Windows or macOS behavior; native qualification remains owed under Ian's release hold. Separate local assembly from hosted builds and publication. Declare the supported target matrix and minimum stable JDK in the reviewed design; SwiftPM needs an explicit Apple binary target strategy.

The first NuGet slice claims `libraries/csharp/ThinkThen.csproj`, `libraries/csharp/Botassembly.ThinkThen.nuspec`, `libraries/csharp/README.md`, `libraries/csharp/tests/package_check.py` and `libraries/csharp/tests/source/Installed.cs`; name the loader file before implementation. Link 0501 as the single package-inventory owner. Each later slice names its manifests, loader and installed tests.

## Surface assessment amendment

Do the bounded package design before the affected host migration, using the existing release target definitions as its starting point. Record supported platform/architecture pairs, runtime floors, native dependency requirements and loader/artifact choices once in the repository's existing package definitions and documentation. Do not infer that every language supports every operating system. Settle stable JVM requirements, Swift's Apple binary strategy and Flutter's actual plugin targets before those implementations. 0504 owns JVM foreign-function code; this ticket owns its distribution.

0516 now owns the initial local NuGet assets, loader and installed pilot. Reuse that result here. This supersedes the NuGet implementation claim above and avoids making the pilot wait for all-language packaging. Remaining package slices follow their corresponding reviewed host implementation; design work may proceed earlier. 0501 owns the shared product inventory consumed by builders, collectors and checks.

Own the final artifact path as well as family archives. Name the actual changes to `sdlc/scripts/release-registry.py`, `sdlc/scripts/release-workflow`, their existing self-tests, artifact collection in `.github/workflows/release.yml`, and each affected installed-consumer route before implementation. Current registry assembly selects only the existing JVM jars, and current workflow validation permits Objective-C artifacts only on Linux; both must follow the new native inventory and Apple-only ruling. A passing development archive cannot substitute for an install of the final registry-format artifact with its declared dependencies and native assets. Prove supported target selection without a checkout, warm loader state or manual library paths. Check clear unsupported-target refusal and absent/wrong native assets in the existing installed checks.

0383–0385 consume these final packages for Windows qualification. Local workflow edits and assembly tests do not authorize dispatch, a candidate or publication. Native platform evidence remains required before the corresponding support claim is made.
