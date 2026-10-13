# ThinkThen for Dart

ThinkThen puts ten functions on top of any model: decide, choose, tag, score, filter, rank, find, annotate, recognize and relate. Your code branches on typed answers, gets not sure instead of a guess, packs many records into few requests, and caches each question so you pay once. Tests replay saved answers with no key or network. Use Jev, OpenAI or a local model from every language and database.

```sh
dart pub add thinkthen_dart
```

Set your hosted backend key: `export THINKTHEN_API_KEY=...`.

```dart
import 'package:thinkthen_dart/thinkthen_dart.dart';

Future<void> main() async {
  final engine = Engine.open();
  try {
    final call = await engine.decide(
      QuestionText(text: 'Does this ask for a refund?'),
      InputText(text: 'Please refund my order.'),
    );
    print(call.decideAnswer());
  } finally { engine.close(); }
}
```

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
