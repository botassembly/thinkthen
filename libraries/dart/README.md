# Dart

The development `thinkthen_dart` package provides one typed asynchronous caller. Import `package:thinkthen_dart/thinkthen_dart.dart`, open an `Engine`, await a named function, and close the engine in `finally`.

```dart
import 'package:thinkthen_dart/thinkthen_dart.dart';

final engine = Engine.open();
try {
  final refundDecision = await engine.decide(
    InputRequestQuestionText(text: 'Does this ask for a refund?'),
    InputRequestInputText(text: 'Refund me please.'),
  );
  final row = refundDecision.packets.whereType<SessionPacketDecideRow>().single;
  print(row.value.answer);
} finally {
  engine.close();
}
```

The ten named methods return `Future<OwnedCall>`. Generated classes retain typed answers, observations, row metadata and terminal facts. `Presence` distinguishes absence from present null. Wide counters and caller integers remain exact `BigInt`; unknown output fields and arbitrary authored JSON remain present. Rust admits requests and owns file reading, image admission, cache identity, recording and replay. Use generated `InputRequestInputSource` for files, `InputRequestImageFile` for image files, and `imageBytes` for `Uint8List` attachments.

`NativeFailure` reports admission errors. `SessionFailure` retains the owned call, completed rows and native terminal failure facts. An unresolved answer stays separate from a failed call. Reader failures use generated failure values; native diagnostics do not expose host exception text.

Pass `Cancellation` to stop a call explicitly. The native nonblocking session operations yield to Dart's event loop while the provider runs. A `feed` stream supplies bounded intake. Cancellation stops further intake and releases native owners before a held provider or asynchronous subscription cleanup finishes. Ordinary completion awaits subscription cleanup and reports `StreamCleanupFailure` if it fails. `OwnedSession` exposes `push`, `finish`, `read` and the typed `packets` stream for explicit session control. Close sessions before the engine. Native finalizers backstop forgotten cleanup; copied results survive close.

`usagePersistence()` observes count persistence without waiting. `finishUsageStatus()` finishes current deltas and returns an owned `UsagePersistenceStatus` with a header-derived state and optional safe advice. Persistence failures preserve answers and call facts. Only usage-lock acquisition has a deadline; other filesystem work can take longer. Closed engines reject these methods.

## Upgrade

| Old call | Typed call |
| --- | --- |
| `Door(path).create(settings)` | `Engine.open(settings: InputEngineSettings(...))` |
| `Door` judgment methods and `CompleteApi` methods | Await the same named `Engine` method with generated question and input values |
| `Door.ask` or generic `call` | Choose one of the ten named functions |
| Legacy batch pull | Use a named method with `feed`, or an owned session's typed `packets` stream |
| `engineFree` or `dispose` | `close()` in `finally` |
| Separate native path | SDK build hook and bundled native asset |

The old public names, complete readers and handwritten native layouts have been removed. The former session import now uses the main package import above.

## Native assets

Dart 3.10 or later is required. The build hook verifies the package definition's SHA-256 and bundles the selected native library. It compiles no Rust and downloads nothing at runtime. The current `native-assets.json` describes the reviewed Linux development asset and has no release URL. Distribution assembly supplies release pins; published installation and other platforms belong to candidate qualification.

For an offline development build, set `hooks.user_defines.thinkthen_dart.offline` to `true` and `asset_cache` to an owned cache directory in the application pubspec. Put the matching library at `<cache>/<sha256>/<file>` using `native-assets.json`. Missing or corrupt cached bytes fail during the build.

The routine installed check is `libraries/dart/check.sh --native-assets NATIVE_LIBRARY SCRATCH PUB_CACHE`. It extracts packages into unrelated consumers, exercises typed calls against a counted loopback backend, builds a standalone Dart bundle and Linux Flutter FFI application, and runs them after their source packages move away. It covers file reading, image admission, cache/replay, presence, failures, usage, cancellation and stream cleanup. It uses the installed toolchains and cached dependencies.
