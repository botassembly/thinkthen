import 'package:collection/collection.dart';
import 'package:thinkthen_dart/thinkthen_dart.dart';

Future<void> main() async {
  final engine = Engine.open();
  const question = 'Which labels fit this message?';
  const labels = ['praise', 'bug', 'billing'];
  const message =
      'Love the new dashboard, '
      'but export crashes the app,\n'
      'and I was charged twice.\n';
  final call = await engine.tag(
    DefinedQuestion(
      value: Definition.read(
        {'tag': question, 'labels': labels},
      ),
    ),
    InputText(text: message),
  );
  final fittingLabels = call.value as List<String>;
  assert(
    const ListEquality<String>().equals(fittingLabels, [
      'praise',
      'bug',
      'billing',
    ]),
  );
  engine.close();
}
