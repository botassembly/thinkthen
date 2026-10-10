import 'package:thinkthen_flutter/thinkthen_flutter.dart';

Future<void> main() async {
  final engine = Engine.open();
  try {
    final result = await engine.decide(
      InputRequestQuestionText(text: 'Does this ask for a refund?'),
      InputRequestInputText(text: 'Refund me please.'),
    );
    print(result.terminal.facts.value);
  } finally {
    engine.close();
  }
}
