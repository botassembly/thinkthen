# thinkthen_dart

This source package calls the ThinkThen C library through Dart FFI. It exposes the six named error kinds, a distinct unresolved `null` outcome, typed annotations and Unicode scalar offsets. `Door.ask` returns the C success envelope with separate `value` and per-call `facts`. `DoorFailure.facts` copies the borrowed facts for a failed started call. The older `Door.decide`, `Door.many`, `Door.recognize`, and `Door.relate` convenience methods return bare typed values; use `Door.ask` when the caller needs success facts.

The native shared library is installed separately. This package does not build or download native code. The published Dart package name is reserved as `thinkthen_dart`; this source integration does not publish it. Match the native archive to the source release and platform. The checked host is Linux x86_64. Android, iOS, macOS, Windows, static linkage, and native asset packaging remain open.

## Build and use from source

Build the C library at the same source commit, or install its matching platform archive. From the repository root:

```sh
CARGO_NET_OFFLINE=true cargo build --offline --release --manifest-path libraries/c/Cargo.toml
cd libraries/dart
dart pub get --offline
dart analyze lib
```

The default Linux output is `target/release/libthinkthen_c.so`, unless `CARGO_TARGET_DIR` selects another location. Add a source path dependency to the consuming application's `pubspec.yaml`:

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
  final result = door.ask(engine, {'decide': 'Is it?', 'evidence': 'Example'}) as Map;
  print(result['value']);
  print(result['facts']);
} finally {
  door.engineFree(engine);
}
```

Cancellation can fire from a second Dart isolate during a blocking native call. Join both isolates before freeing the cancel token or engine. Strings returned by C are freed with `thinkthen_free_string`; caller arguments use `package:ffi` allocation. [Flutter source](flutter/README.md) contains a Linux application that calls through this package.

`check.sh` builds the current C source offline, matches the header to all exported symbols, checks two independent installed Dart consumers, runs the shared J1 corpus through the public `Door` API, and runs the Flutter Linux host and app. Set `TT_DART` and `TT_FLUTTER` to installed executables when they are absent from `PATH`. The check needs a populated local pub cache and creates no runtime downloads.
