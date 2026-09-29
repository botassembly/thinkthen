# Flutter Linux consumer

This source folder holds `thinkthen_flutter`, a small Flutter-facing wrapper around `thinkthen_dart`, and a real Linux Flutter application in `example/`. The example builds a Linux engine app, runs under Xvfb, calls the native library, and checks its counted backend arrival. The wrapper does not bundle a native library and has no Flutter platform-channel plugin declaration. Install the matching native archive separately and supply its path as `TT_NATIVE_LIBRARY` for the example or as the `ThinkThenFlutter` constructor argument in an application.

The Flutter package is private (`publish_to: none`) and its source path dependency points at the sibling Dart package. Linux x86_64 is the checked host. Other Flutter targets need their own native artifact and installation proof. No binary is downloaded at runtime.

From the repository root, run `libraries/dart/check.sh` with a populated offline pub cache. It checks the Dart binding, Flutter host test, planted negatives, and real Linux embedder. `TT_FLUTTER` can name the installed Flutter executable.
