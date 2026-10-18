# 0670: Publish Dart and Flutter native assets for every platform

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0615, 0618, 0619, 0671, 0688

## Outcome

- The native-asset manifest is generated from the release inventory. Each supported platform has an HTTPS download address and a checksum.
- The checked-in hash and the development-only mode are deleted.
- The Flutter plugin declares every supported platform, at least Linux and macOS, and can be published.
- The Dart conformance runner becomes a packet dumper following the 0642 contract, and the old fixture consumer is deleted.

### Proof

- On the M5 through pm job, a clean Dart project resolves the package from a local pub server, fetches the native asset from a local HTTPS mirror of the candidate, and runs a named call with no library path.
- A clean Flutter app builds and runs a named call natively on the M5 and in the pinned arm64 container.

## Evidence

- **Development-only assets.** `libraries/dart/native-assets.json` marks itself development-only, lists only `linux_x64`, has `"url": null`, and keeps a checked-in sha256 that must change with every native build.
- **Flutter not publishable.** The Flutter plugin is `publish_to: none` and declares only Linux (`libraries/dart/flutter/pubspec.yaml`).
- **Path hack and silent skips.** `check.sh:8` falls back to a Flutter under `$HOME/.local/opt/flutter`, and `check.sh:10,13` exit 77. 0615 and 0618 fix both.
- **Packaging churn.** About 37 commit subjects since 2026-09-15 mention packaging or install (810d9573d, a11c38061, e72cbbba1, 29330cffd, 229dd3822).
