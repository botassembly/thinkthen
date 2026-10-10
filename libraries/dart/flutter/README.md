# Flutter Linux FFI package

Import `package:thinkthen_flutter/thinkthen_flutter.dart` and use `Engine.open(settings: ...)`. The ten named `Future` methods, generated types, owned sessions, typed streams, cancellation and cleanup share the Dart implementation. The entry point fixes native attribution to Flutter. See the [Dart caller guide](../README.md) for calls, results, errors and lifetime rules.

Flutter 3.38 or later and Dart 3.10 or later are required. The Linux FFI plugin uses the Dart package's build hook for checksum verification and native asset bundling. Its installed application contains the native library. The current development definition needs the matching owned cache; final distribution assembly supplies approved release pins. The package remains unpublished. Other platform qualification belongs to the candidate.

The old `ThinkThenFlutter(path)` and `ThinkThenCompleteFlutter(path)` facades map to `Engine.open()`. Await the same named function with generated question and input values, then call `close()` in `finally`. The old facade imports and handwritten readers have been removed.

The [routine installed check](../README.md#native-assets) launches an extracted-package Linux application under Xvfb against a counted fake provider and checks bundled native ownership and held cancellation.
