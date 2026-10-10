# Flutter Linux FFI package

`thinkthen_flutter` declares a Linux FFI plugin and depends on the same released version of `thinkthen_dart`. Import `package:thinkthen_flutter/thinkthen_session_flutter.dart` and call `Engine.open(settings: ...)`. The ten named asynchronous methods, generated types, cancellation and cleanup share the Dart caller. The entry point fixes the native surface to `flutter`.

Flutter 3.38 or later and Dart 3.10 or later are required. The Dart package’s build hook owns acquisition, checksum verification and native-asset bundling. The Linux CMake integration adds no second engine or downloader. The installed application contains its native library and needs no caller path or runtime download.

The current native definition is development-only and requires the matching owned cache. The [Dart offline instructions](../README.md#development-owned-session-caller) describe that cache and the focused installed check. The check builds and launches an extracted-package Linux x86-64 application under Xvfb with a local fake provider. It claims no other Flutter target or published pub package. Publication remains held and `publish_to: none` remains set. Final distribution assembly must supply the approved release asset pin before ordinary pub installation can work.

The old entry points and example remain during the public API migration. They retain their explicit native path and are not the installed native-assets consumer.

## Complete typed Flutter calls

Import `package:thinkthen_flutter/thinkthen_complete_flutter.dart` and create `ThinkThenCompleteFlutter(absoluteLibrary, settingsJson: settingsJson)`. It implements the typed `CompleteApi` from Dart with the same ten named functions and six native lazy batch methods. Questions, records, images, file selections, controls and copied result views are re-exported. Close the owned facade after its batches and calls finish.

The facade selects the actual native `flutter` surface and retains one engine, route, reader, scheduler and storage. It preserves all released `ThinkThenFlutter` methods. Complete runtime cases execute this facade inside `flutter test`, separately from Dart's AOT consumer, using the same canonical fixtures and counted loopback sends. See the [Dart complete API](../README.md#complete-typed-calls) for the input, view, error and lifetime contracts.
