# ThinkThen for Dart

ThinkThen answers typed questions about text. Use it to decide yes or no, choose a label, score an answer, or filter and rank records. ThinkThen gives code typed answers to branch on, caches each question so you pay once, and replays recorded answers offline for tests. Backends include Jev, SystemOne, OpenAI and local models.

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
