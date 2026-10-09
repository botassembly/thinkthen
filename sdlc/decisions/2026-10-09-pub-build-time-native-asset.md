# Dart and Flutter fetch the prebuilt library at build time

## Ruling

The PM approved the pub route in the [native package design](2026-10-09-native-package-design.md) on 2026-10-09, under Ian's instruction to make all the right decisions for 0.2. Ian can overturn it.

For Dart and Flutter on pub, "each package carries its prebuilt native library" permits the package's build hook to fetch the same-version, checksum-pinned prebuilt library during the application build and bundle it into the application automatically. A download at first call and a consumer-built Rust engine stay prohibited.

## Reason

Flutter's guidance says not to upload a plugin containing binary code to pub.dev. The build-hook route is the standard Dart native-assets mechanism, keeps ordinary `dart pub add` installation, and needs no manual library path. The installed application still contains the native library. The cost is higher floors (Dart 3.10, Flutter 3.38), hook dependencies and a checksum-pinned build-time fetch with an owned cache.

## Options considered

- The pub build-hook route. Chosen.
- A trusted Git or release plugin that bundles the binary and keeps Dart 3.3 and Flutter 3.24. It avoids the build-time fetch but fails ordinary pub.dev installation.

## Replaces

The open exception in the native package design. Publication and removing `publish_to: none` still wait for Ian's release permission.
