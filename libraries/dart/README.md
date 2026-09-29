# thinkthen_dart

This source package calls the ThinkThen C library through Dart FFI. It exposes the six named error kinds, a distinct unresolved `null` outcome, typed annotations and Unicode scalar offsets. `Door.ask` returns the C success envelope with separate `value` and per-call `facts`. `Door.decide`, `Door.many`, `Door.recognize`, and `Door.relate` each return `CallResult<T>`: `.value` is the former typed result and `.facts` is that operation's owned `CallFacts`. Required facts are `records`, `requestsSent`, `cacheAnswers`, and `seconds`; optional token counts and `model` are null when unreported. `DoorFailure.facts` copies the borrowed facts for a failed started call.

The native shared library is installed separately. This package does not build or download native code. The published Dart package name is reserved as `thinkthen_dart`; this source integration does not publish it. Match the native archive to the source release and platform. The checked host is Linux x86_64. Android, iOS, macOS, Windows, static linkage, and native asset packaging remain open.

## Build and use from source

Build the C library at the same source commit, or install its matching platform archive. From the repository root:

```sh
CARGO_NET_OFFLINE=true cargo build --offline --release --manifest-path libraries/c/Cargo.toml
cd libraries/dart
dart pub get --offline
dart analyze lib
```

The default Linux output is `libraries/c/target/release/libthinkthen_c.so`, unless `CARGO_TARGET_DIR` selects another location. Add a source path dependency to the consuming application's `pubspec.yaml`:

```yaml
dependencies:
  thinkthen_dart:
    path: /path/to/thinkthen/libraries/dart
```

Pass the installed native library path explicitly:

```dart
import 'package:thinkthen_dart/thinkthen_dart.dart';

final door = Door('/absolute/path/to/libthinkthen_c.so');
final engine = door.create();
try {
  final decisionEnvelope = door.ask(engine, {'decide': 'Is it?', 'evidence': 'Example'}) as Map;
  print(decisionEnvelope['value']);
  print(decisionEnvelope['facts']);
} finally {
  door.engineFree(engine);
}
```

Cancellation can fire from a second Dart isolate during a blocking native call. Join both isolates before freeing the cancel token or engine. Strings returned by C are freed with `thinkthen_free_string`; caller arguments use `package:ffi` allocation. [Flutter source](flutter/README.md) contains a Linux application that calls through this package.

`check.sh` builds the current C source offline, matches the header to all exported symbols, checks two independent installed Dart consumers, runs the shared J1 corpus through the public `Door` API, and runs the Flutter Linux host and app. Set `TT_DART` and `TT_FLUTTER` to installed executables when they are absent from `PATH`. The check needs a populated local pub cache and creates no runtime downloads.

The local Linux file pilot packs this Dart source beside a separately built matching C archive with `release-pack x86_64-unknown-linux-gnu OUT c php dart` from one clean commit. The Dart archive contains no Flutter wrapper or native library. After verifying the pair, set `THINKTHEN_ARTIFACT` to the absolute Dart archive path, `THINKTHEN_C_ARTIFACT` to the absolute C archive path, `TT_DART` to the Dart executable and `PUB_CACHE` to a local cache containing `ffi` 2.2.0, then run `sh libraries/dart/check.sh 0`. That installed-file mode resolves an unrelated consumer offline to the unpacked Dart source and loads the unpacked C library. It does not install or test the private Flutter wrapper or publish to pub.dev.
