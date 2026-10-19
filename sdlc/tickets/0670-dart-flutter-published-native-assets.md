# 0670: Publish Dart and Flutter native assets for every platform

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0615, 0618, 0619, 0688, 0728

## Outcome

- Changes: `libraries/dart/native-assets.json`, `libraries/dart/hook/build.dart`, `libraries/dart/pubspec.yaml`, `libraries/dart/flutter/`, `libraries/dart/check.sh`, `libraries/dart/checks/`, `libraries/dart/README.md`, `sdlc/tools/dart.json`, `sdlc/scripts/release-pack.d/dart`, `sdlc/scripts/release-pack.d/flutter`.
- `release-pack` generates the native-asset manifest from the `dart` entry under `libraries` in `sdlc/tools.json`. Each supported platform has an HTTPS download address and a checksum.
- The checked-in hash and the development-only mode in `hook/build.dart` are deleted.
- The Flutter plugin declares every supported platform, at least Linux and macOS, and can be published. `sdlc/tools.json` declares Flutter for macOS and adds `darwin-arm64` to the Dart `check_platforms`.

### Proof

- On the M5 through `pm job run --machine m5 0670 SHA --memory 8 -- sh libraries/dart/check.sh`, a clean Dart project resolves the package from a local pub server, fetches the native asset from a local HTTPS mirror of the candidate, and runs a named call with no library path. A clean Flutter app builds and runs a named call on macOS. Before the change the check exits 77 on the M5 because only `linux-x86_64` is declared.
- The Flutter app in the arm64 container runs at the 0619 container checkpoint.

## Evidence

- Order (coordinator, 2026-10-19): the packaging ticket 0670 lands before the call ticket 0671, so 0671 can prove on the M5. The packet dumper belongs to 0671, written once against the run call.
- **Development-only assets.** `libraries/dart/native-assets.json` marks itself `development-only`, lists only `linux_x64`, has `"url": null`, and keeps a checked-in sha256 that must change with every native build. `libraries/dart/hook/build.dart` refuses an asset without a URL and points at the development cache.
- **Flutter not publishable.** `libraries/dart/flutter/pubspec.yaml` is `publish_to: none` and declares only the `linux` plugin platform; only `flutter/linux/CMakeLists.txt` exists.
- **One platform.** `sdlc/tools.json` declares Dart for `linux-x86_64` only and Flutter for Linux only. `release-pack` copies `native-assets.json` into the Dart archive unchanged.
- **Packaging churn.** About 37 commit subjects since 2026-09-15 mention packaging or install (810d9573d, a11c38061, e72cbbba1, 29330cffd, 229dd3822).
- The `version` fields in `native-assets.json` and the pubspecs are package release numbers and stay under `sdlc/decisions/2026-10-19-no-version-numbers-before-1-0.md`.
