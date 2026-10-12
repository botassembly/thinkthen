import 'package:collection/collection.dart';
import 'package:thinkthen_dart/thinkthen_dart.dart';

typedef Named =
    RelatedEntityEdgePropertiesSourceFieldsKindName;

Future<void> main() async {
  final engine = Engine.open();
  const names = [
    {'name': 'Paul McCartney', 'kind': 'singer'},
    {'name': 'Ringo Starr', 'kind': 'singer'},
    {'name': 'Yesterday', 'kind': 'song'},
    {'name': "Octopus's Garden", 'kind': 'song'},
  ];
  final call = await engine.relate(
    InputRequestQuestionDefinition(
      value: InputRequestDefinition.read({
        'version': 1,
        'relate': {
          'relations': [
            {
              'name': 'sings',
              'source': 'singer',
              'target': 'song',
            },
          ],
        },
      }),
    ),
    InputRequestInputRecords(items: [
      for (final name in names)
        InputRequestItem(
          original: Presence.present(
            InputRequestOriginalJson(value: name),
          ),
        ),
    ]),
  );
  final whoSings = call.packets
      .whereType<SessionPacketRelateAggregate>()
      .single
      .value
      .value;
  final pairs = [
    for (final edge in whoSings)
      (
        (edge.source as Named).name,
        (edge.target as Named).name,
      ),
  ];
  const expected = [
    ('Paul McCartney', 'Yesterday'),
    ('Ringo Starr', "Octopus's Garden"),
  ];
  assert(const ListEquality<(String, String)>()
      .equals(pairs, expected));
  engine.close();
}
