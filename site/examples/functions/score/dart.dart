import 'package:thinkthen_dart/thinkthen_dart.dart';

Future<void> main() async {
  final engine = Engine.open();
  const question = 'How urgent is this?';
  const levels = ['Routine.', 'Soon.', 'Immediate.'];
  const outage = 'Our checkout page is down '
      'and customers cannot pay.\n';
  final call = await engine.score(
    InputRequestQuestionDefinition(
      value: InputRequestDefinition.read({
        'score': question,
        'levels': levels,
      }),
    ),
    InputRequestInputText(text: outage),
  );
  final urgency = call.packets
      .whereType<SessionPacketScoreRow>()
      .single
      .value
      .value;
  assert(urgency == 2.0);
  engine.close();
}
