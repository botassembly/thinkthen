import 'package:collection/collection.dart';
import 'package:thinkthen_dart/thinkthen_dart.dart';

typedef Named =
    RelationEndpointRelatedEntity;

Future<void> main() async {
  final engine = Engine.open();
  const names = [
    {'name': 'Apple', 'kind': 'company'},
    {'name': 'Amazon', 'kind': 'company'},
    {'name': 'iPhone', 'kind': 'product'},
    {'name': 'Kindle', 'kind': 'product'},
  ];
  final call = await engine.relate(
    DefinedQuestion(
      value: Definition.read({
        'version': 1,
        'relate': {
          'relations': [
            {
              'name': 'makes',
              'source': 'company',
              'target': 'product',
            },
          ],
        },
      }),
    ),
    InputEntities(items: [
      for (final name in names)
        Item(
          original: Presence.present(
            OriginalJson(value: name),
          ),
        ),
    ]),
  );
  final whoMakes = call.relateAnswer().value;
  final pairs = [
    for (final edge in whoMakes)
      (
        (edge.source as Named).name,
        (edge.target as Named).name,
      ),
  ];
  const expected = [
    ('Apple', 'iPhone'),
    ('Amazon', 'Kindle'),
  ];
  assert(const ListEquality<(String, String)>()
      .equals(pairs, expected));
  engine.close();
}
