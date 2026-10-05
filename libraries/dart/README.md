# thinkthen_dart

This source package calls the ThinkThen C library through Dart FFI. It exposes the six named error kinds with their C codes (`ErrorKind.code`), a distinct unresolved `null` outcome, and Unicode scalar offsets. Results are plain JSON: `Door.ask` returns the C success envelope with separate `value` and per-call `facts`, and `Door.decide`, `Door.many`, `Door.recognize`, and `Door.relate` each return a record whose `.facts` is that call's facts map, as `specification/result.schema.json` describes it. A reader ignores members it does not know. `readField` reads one annotate member as `UnresolvedField` (JSON null), `AnswerField` with its value, or `FailedField` with its kind and cause. `Door.plan(engine, verb, question, input, settings)` previews a decide, choose, score or tag call through `thinkthen_plan_json` and returns the plan map; it needs no key and sends nothing. `call`, `ask`, `recognize` and `relate` take `deadline` and `token` like `decide` and `many`, and `max_requests_total` passes through `Door.create(settings)`. `DoorFailure.facts` copies the borrowed facts for a failed started call.

The native shared library is installed separately. This package does not build or download native code. pub.dev publishes the package as `thinkthen_dart`. Each GitHub release ships the matching C archive. Match the native archive to the source release and platform. The checked host is Linux x86_64. Android, iOS, macOS, Windows, static linkage, and native asset packaging remain open.

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

`Door.create(settings)` accepts `{"backend":"local"}` to select the `local` entry in the read-only ThinkThen configuration. Use `{"base_url":"http://localhost:11434/v1"}` for a direct address instead. A named backend supplies its address, model, wire settings and key environment variable; explicit constructor settings take precedence. Omitting `backend` preserves ordinary environment/default selection. A missing or invalid name fails before sending.

Explicit files and folders use the [library reader contract](../files.md), with line, window or whole-file units and located results. Existing text, record and column methods retain their arguments.
