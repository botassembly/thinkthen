# ThinkThen for Dart

ThinkThen is ten functions on top of Jev, TypeSafe's decision model, or any model like it. It puts these decision models in your code, whatever model, language or database you use. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

```sh
dart pub add thinkthen_dart
```

Set your hosted backend key: `export THINKTHEN_API_KEY=...`. Backends include Jev, SystemOne, OpenAI and local models.

```dart
import 'package:thinkthen_dart/thinkthen_dart.dart';
Future<void> main() async {
  final engine = Engine.open();
  final teamQuestion = DefinedQuestion(value: ChooseQuestion(
    choose: const Wording.text('Which team owns this?'),
    options: const Presence.present(ChoiceOptions.fields({
      'billing': 'Invoices, fees, and refunds.', 'shipping': 'Parcels and delivery.',
      'account': 'Logins and passwords.'}))));
  try {
    for (final text in ['Please refund my order. It arrived broken.',
         'Thanks for the quick help yesterday!', 'I want to send this back.']) {
      final refund = (await engine.decide(QuestionText(text: 'Does the customer ask for a refund?'), InputText(text: text))).decideAnswer();
      print(refund);
    }
    for (final text in ['Please refund the extra fee on my invoice.',
         'My parcel went to the wrong address.', 'I cannot reset my password.']) {
      final team = (await engine.choose(teamQuestion, InputText(text: text))).chooseAnswer();
      print(team);
    }
  } finally { engine.close(); }
}
```

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
