import 'package:collection/collection.dart';
import 'package:thinkthen_dart/thinkthen_dart.dart';

Future<void> main() async {
  final engine = Engine.open();
  const question = 'Which labels fit this message?';
  const labels = ['praise', 'bug', 'billing'];
  const message = 'Love the new dashboard, '
      'but export crashes the app,\n'
      'and I was charged twice.\n';
  final call = await engine.tag(
    InputRequestQuestionDefinition(
      value: InputRequestDefinition.read({
        'tag': question,
        'labels': labels,
      }),
    ),
    InputRequestInputText(text: message),
  );
  final fittingLabels = call.packets
      .whereType<SessionPacketTagRow>()
      .single
      .value
      .value;
  assert(const ListEquality<String>().equals(
    fittingLabels,
    ['praise', 'bug', 'billing'],
  ));
  engine.close();
}
