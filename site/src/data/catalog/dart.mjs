
export default {
    slug: 'dart', name: 'Dart', deckHeading: null,
    versions: { runtime: 'Dart 3.10 or later, below 4; Flutter 3.38 or later', platforms: 'Linux x86-64' },
    backends: 'code',
    lang: 'dart', tab: 'Dart',
    blurb: "Await the ten functions from Dart and Flutter.",
    call: 'Open an `Engine` with `Engine.open()`, `await engine.decide(question, input)`, and close the engine in `finally`.',
    unsureWord: 'a null answer with null probability; a resolved authored JSON null keeps its probability',
    facts: "`CallResult` has `value`, `probability`, `results` and `facts`; its owned packets retain presence and native terminal facts.",
    errors: "`NativeFailure` reports admission errors; `SessionFailure` retains the failed call.",
    settings: "`Engine.open(settings: InputEngineSettings(...))` loads the bundled native asset.",
    install: [
      ['dart pub add thinkthen_dart', null],
      ['thinkthen-dart-VERSION-TARGET.tar.gz', 'The package source, from each release.'],
    ],
    particular: ["Close engines and sessions in `finally`.", "Dart and Flutter share the native asset route."],
  };
