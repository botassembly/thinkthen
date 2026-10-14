# Dart

The development `thinkthen_dart` package provides one typed asynchronous caller. Import `package:thinkthen_dart/thinkthen_dart.dart`, open an `Engine`, await a named function, and close the engine in `finally`.

```dart
import 'package:thinkthen_dart/thinkthen_dart.dart';

Future<void> main() async {
  final engine = Engine.open();
  try {
    final refund = await engine.decide(
      QuestionText(text: 'Does the customer ask for a refund?'),
      InputText(text: 'Please refund my order. It arrived broken.'),
    );
    final isRefund = refund.value;
    final probability = refund.probability;
    assert(isRefund == true);
    assert(probability is ScalarProbability);
  } finally { engine.close(); }
}
```

The ten named methods return typed `Future<CallResult<V, R>>` objects with `value`, `probability`, `results` and `facts`. `probability` is null or a closed numeric carrier: `ScalarProbability.value` is a double, `ListProbability.value` is a list of nullable doubles and `GroupProbability.value` is a list of those lists. Generated classes retain typed answers, observations, row metadata and terminal facts. `Presence` distinguishes absence from present null. Rank's `value` returns input indexes in rank order as `List<BigInt>`; Filter's `value` returns accepted original values. Wide counters and caller integers remain exact `BigInt`; unknown output fields and arbitrary authored JSON remain present. Rust admits requests and owns file reading, image admission, cache identity, recording and replay. Use generated `InputSource` for files, `ImageFile` for image files, and `imageBytes` for `Uint8List` attachments.

`CallFailure` carries a generated error kind, retryability and owned typed facts. Terminal failures retain the owned call and its completed rows. An unresolved answer stays separate from a failed call. Reader failures use generated failure values; native diagnostics do not expose host exception text.

Pass `Cancellation` to stop a call explicitly. A persistent helper isolate performs native blocking waits while Dart's calling isolate remains responsive. A `feed` stream supplies bounded intake. Cancellation stops further intake and returns before a held provider or asynchronous subscription cleanup finishes; in-flight helpers retain native owners until their waits settle. Ordinary completion awaits subscription cleanup and reports `StreamCleanupFailure` if it fails. `OwnedSession` exposes `push`, `finish`, `read` and the typed `packets` stream for explicit session control. Close sessions before the engine. Native finalizers backstop forgotten cleanup; copied results survive close.

`plan(function, question, input, options: ...)` returns a typed `Plan` through native admission without sending. Atomic functions and rank support previews; other functions refuse. Estimates retain exact `BigInt` counts and an optional first request body.

`usagePersistence()` observes count persistence without waiting. `finishUsageStatus()` finishes current deltas and returns an owned `UsagePersistenceStatus` with a header-derived state and optional safe advice. Persistence failures preserve answers and call facts. Only usage-lock acquisition has a deadline; other filesystem work can take longer. Closed engines reject these methods.

## Native assets

Dart 3.10 or later is required. The build hook verifies the package definition's SHA-256 and bundles the selected native library. It compiles no Rust and downloads nothing at runtime. The current `native-assets.json` describes the reviewed Linux development asset and has no release URL. Distribution assembly supplies release pins; published installation and other platforms belong to candidate qualification.

For an offline development build, set `hooks.user_defines.thinkthen_dart.offline` to `true` and `asset_cache` to an owned cache directory in the application pubspec. Put the matching library at `<cache>/<sha256>/<file>` using `native-assets.json`. Missing or corrupt cached bytes fail during the build.

The routine installed check is `libraries/dart/check.sh --native-assets NATIVE_LIBRARY SCRATCH PUB_CACHE`. It extracts packages into unrelated consumers, exercises typed calls against a counted loopback backend, builds a standalone Dart bundle and Linux Flutter FFI application, and runs them after their source packages move away. It covers file reading, image admission, cache/replay, presence, failures, usage, cancellation and stream cleanup. It uses the installed toolchains and cached dependencies.
