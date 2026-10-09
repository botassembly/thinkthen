# 0517: Ship the native library inside every package

Status: OPEN.

Milestone: 0.2

Reviews: revision 4cd756859, reject

Reviews: revision c79e3f65e05eb4f12fa46d8f7ab003d477258c9c, accept

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
