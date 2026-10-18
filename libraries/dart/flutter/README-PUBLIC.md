# ThinkThen for Flutter

ThinkThen is ten functions on top of Jev, TypeSafe's System One model, or any model like it. System One models return probabilities. Your code decides what to do with them. ThinkThen makes these models simple to call from any language or database. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

```sh
flutter pub add thinkthen_flutter
```
Set your hosted backend key: `export THINKTHEN_API_KEY=...`. Backends include Jev, SystemOne, OpenAI and local models. Judgment calls can send your question and text to the selected backend; the default is TypeSafe.
```dart
import 'package:thinkthen_flutter/thinkthen_flutter.dart';
Future<void> main() async {
  final engine = Engine.open();
  final teamQuestion = DefinedQuestion(value: ChooseQuestion(
    choose: const Wording.text('Which team owns this?'),
    options: const Presence.present(ChoiceOptions.fields({
      'billing': 'Invoices, fees, and refunds.', 'shipping': 'Parcels and delivery.',
      'account': 'Logins and passwords.'}))));
  try {
    for (final (index, text) in ['Please refund my order. It arrived broken.',
         'Thanks for the quick help yesterday!', 'I want to send this back.'].indexed) {
      final refund = (await engine.decide(QuestionText(text: 'Does the customer ask for a refund?'), InputText(text: text)));
      final isRefund = refund.value; final odds = (refund.probability as ScalarProbability).value;
      assert(isRefund == [true, false, true][index] && odds == [0.99, 0.01, 0.54][index]);
    }
    for (final (index, text) in ['Please refund the extra fee on my invoice.',
         'My parcel went to the wrong address.', 'I cannot reset my password.'].indexed) {
      final team = (await engine.choose(teamQuestion, InputText(text: text)));
      final owner = team.value; final odds = (team.probability as ScalarProbability).value;
      assert(owner == ['billing', 'shipping', 'account'][index] && odds == 1.0);
    }
  } finally { engine.close(); }
}
```

`value` is the answer. `probability` is null or a `ScalarProbability`, `ListProbability` or `GroupProbability` with typed numeric values. `results` keeps each record and `facts` keeps call observations. Not sure has a null probability. An authored JSON null answer can have a numeric probability.

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them. These examples were checked with saved answers; live answers and probabilities can differ. See the [package guide](https://thinkthen.dev/languages/flutter/) for results, failures, record inputs and offline replay.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
