# Flutter Linux consumer

This source folder holds `thinkthen_flutter`, a small Flutter-facing wrapper around `thinkthen_dart`, and a real Linux Flutter application in `example/`. The example builds a Linux engine app, runs under Xvfb, calls the native library, and checks its counted backend arrival. The wrapper does not bundle a native library and has no Flutter platform-channel plugin declaration. Install the matching native archive separately and supply its path as `TT_NATIVE_LIBRARY` for the example or as the `ThinkThenFlutter` constructor argument in an application.

The Flutter package is private (`publish_to: none`) and its source path dependency points at the sibling Dart package. Linux x86_64 is the checked host. Other Flutter targets need their own native artifact and installation proof. No binary is downloaded at runtime.

`ThinkThenFlutter.decide` returns a record with the judgment in `.value` and that call's facts object in `.facts`, a JSON map; the facade closes its temporary engine before returning. The underlying Dart `Door` returns the same record shape from `many`, `recognize`, and `relate`.

From the repository root, run `libraries/dart/check.sh` with a populated offline pub cache. It checks the Dart binding, Flutter host test, planted negatives, and real Linux embedder. `TT_FLUTTER` can name the installed Flutter executable.

For a local Linux release-file check, pack `c dart flutter` together with `sdlc/scripts/release-pack`. Validate the resulting three files with `sdlc/scripts/release-go-cpp-pair OUT dart-flutter`. The Flutter archive keeps this private package and example; install it beside the matching unpacked Dart archive so the relative dependencies still resolve. Pass the Flutter, Dart, and C archive paths as `THINKTHEN_ARTIFACT`, `THINKTHEN_DART_ARTIFACT`, and `THINKTHEN_C_ARTIFACT` to `libraries/dart/check.sh`. This is a local Linux consumer proof, not a published Flutter package or proof for another platform.

0429's private `lib/src/complete.dart` facade independently exposes the ten typed
request builders to the Flutter consumer test. It remains outside the public
`thinkthen_flutter.dart` export. The facade test compiles and executes carriers
and independently expected requests separately from Dart. Actual Flutter
complete calls, its `flutter` surface token and counted runtime parity await
the reviewed complete C/native adapter; the legacy facade above stays intact.
