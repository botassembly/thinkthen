import 'package:collection/collection.dart';
import 'package:thinkthen_dart/thinkthen_dart.dart';

Future<void> main() async {
  final engine = Engine.open();
  const question = 'Is this a complaint?';
  const reviews = [
    'Arrived a day early. Thank you!',
    'The zipper broke the first time I used it.',
    'Does this come in blue?',
    'The strap snapped on day two.',
  ];
  final call = await engine.filter(
    InputRequestQuestionText(text: question),
    InputRequestInputRecords(items: [
      for (final review in reviews)
        InputRequestItem(
          original: Presence.present(
            InputRequestOriginalText(text: review),
          ),
        ),
    ]),
  );
  final complaints = [
    for (final row
        in call.packets.whereType<SessionPacketFilterRow>())
      row.value.input.value,
  ];
  assert(const ListEquality<Object?>().equals(
    complaints,
    [reviews[1], reviews[3]],
  ));
  engine.close();
}
