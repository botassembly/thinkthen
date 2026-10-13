# ThinkThen for dart

ThinkThen answers typed questions about text. Use it to decide yes or no, choose a label, score an answer, or filter and rank records. Failed calls remain separate from answers.

## Install

```sh
dart pub add thinkthen_dart
```

## Example

Configure a model backend before running this example.

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

## Cost

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
