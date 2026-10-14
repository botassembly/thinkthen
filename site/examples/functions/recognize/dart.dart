import 'package:collection/collection.dart';
import 'package:thinkthen_dart/thinkthen_dart.dart';

Future<void> main() async {
  final engine = Engine.open();
  const kinds = ['person', 'product', 'place'];
  const text =
      'Sam Patel says his Kindle arrived broken '
      'in Denver.';
  final call = await engine.recognize(
    DefinedQuestion(
      value: Definition.read({
        'version': 1,
        'recognize': {
          'kinds': {for (final kind in kinds) kind: null},
        },
      }),
    ),
    InputText(text: text),
  );
  final facts = call.value as RecognizeEntities;
  final names = [for (final one in facts.entities) (one.text, one.kind)];
  const expected = [
    ('Sam Patel', 'person'),
    ('Kindle', 'product'),
    ('Denver', 'place'),
  ];
  assert(const ListEquality<(String, String)>().equals(names, expected));
  engine.close();
}
