import 'package:thinkthen_dart/thinkthen_dart.dart';

Future<void> main() async {
  final engine = Engine.open();
  const question = 'How urgent is this?';
  const levels = ['Routine.', 'Soon.', 'Immediate.'];
  const outage = 'Our checkout page is down '
      'and customers cannot pay.\n';
  final call = await engine.score(
    DefinedQuestion(
      value: Definition.read({
        'score': question,
        'levels': levels,
      }),
    ),
    InputText(text: outage),
  );
  final urgency = call.scoreAnswer();
  assert(urgency == 2.0);
  engine.close();
}
