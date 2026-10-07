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

## Complete typed calls

Import `package:thinkthen_dart/thinkthen_complete.dart` for the native-backed `Engine`, typed questions/inputs and copied complete views. Existing `Door` methods retain their behavior.

```dart
import 'package:thinkthen_dart/thinkthen_complete.dart';

final engine = Engine(absoluteLibrary, settingsJson: settingsJson);
try {
  final question = Question.spec(QuestionSpec(FunctionKind.decide,
      text: const Content.text('Does this ask for a refund?')));
  final result = engine.decide(question,
      Records([Record(const Content.text('Refund me please.'))]));
  final answerId = result.rows.first.common.answer_id.data;
  final probability = result.rows.first.common.answer.value!.data.probability;
} finally { engine.close(); }
```

The ten named methods return `CompleteResult<DecideView>`, `ChooseView`, `TagView`, `ScoreView`, `FilterView`, `RankView`, `FindView`, `AnnotateView`, `RecognizeView` and `RelateView`, respectively. `CompleteApi` fixes their compile-time signatures for Dart and the separately executed Flutter facade.

`QuestionSpec` carries typed choices/descriptions, meanings, reading rules, model/profile/batch, pointers, members, recognition kinds and relation rules. `Author`, `Declaration` and `Property` carry names, wording versions and item/context schemas. `Question.file`, `named` and `reference` use native loaders. `Question.saved(LoaderRole.…, json)` explicitly imports saved grammar under one native parser. Text never implies a path.

`Records` preserves ordered `Record` originals, contexts, candidate replacements and `Image` attachments. `Content.text` is literal; `Content.json` is arbitrary caller JSON, including explicit null. `Files` selects native line/window/file/image/JSONL reading. Images remain ordered original JPEG/PNG bytes; native alone decodes and admits their route/limits. Only decide/choose/score support them on admitted routes. Other functions refuse before sending.

Results copy summaries/facts/attempts, every observation and its details/author, full row views/details/authors, annotation member authors, rank member views and located recognition/relation views. Known fields have static property types; arbitrary authored/original JSON retains explicit content bytes. Optional views keep `present` and nullable `value`. Decide's native discriminator preserves uncertainty, ordinary Boolean and authored meanings, including Boolean/null. Find's named-answer reader accepts declared C tag 5 and current native tag 7 and preserves the emitted tag. Unsigned counters are exact `BigInt`; wording version admission stays native.

`CompleteFailure` exposes `ErrorKind`, message, retryability and copied final native summary. Prestart failures have absent facts/attempts. `Controls` supplies deadlines, cancellation, context, packing and attempts. Cancellation owns a native token; `nativeHandle` is borrowed until close and can be passed to a cooperating native thread/isolate. Free tokens after using calls finish.

`decideBatch`, `chooseBatch`, `tagBatch`, `scoreBatch`, `filterBatch` and `annotateBatch` own native lazy batches. Pull `next()` until exhaustion, read `facts()`, and always close the batch before closing its engine. The native engine owns all reading, packing, scheduling and storage. Returned views survive engine/batch close. Cache/record/refresh/replay settings retain native identities and support changed-reading strict replay without sends.

The older private JSON carrier readers retain their own boundary restrictions; complete views above do not use them. The canonical consumer is AOT-compiled and statically calls the named public methods. `complete_parity.py dart` reuses the shared suite and counted owned loopback backend. Flutter runs its own facade separately.
