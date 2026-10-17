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
    QuestionText(text: question),
    InputRecords(
      items: [
        for (final review in reviews)
          Item(original: Presence.present(
            OriginalText(text: review),
          )),
      ],
    ),
  );
  final complaints = call.value;
  assert(
    const ListEquality<Object?>().equals(
      complaints, [reviews[1], reviews[3]],
    ),
  );
  engine.close();
}
