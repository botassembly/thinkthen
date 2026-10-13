import 'package:collection/collection.dart';
import 'package:thinkthen_dart/thinkthen_dart.dart';

Future<void> main() async {
  final engine = Engine.open();
  const question = 'Is this urgent?';
  const inbox = [
    'Newsletter: our autumn catalog is here. '
        'No reply needed.',
    'Our checkout page is down and customers cannot pay',
    'Reminder: your invoice is due in 30 days',
    'Please send the signed quote by 5 pm today',
  ];
  final call = await engine.rank(
    QuestionText(text: question),
    InputRecords(items: [
      for (final message in inbox)
        Item(
          original: Presence.present(
            OriginalText(text: message),
          ),
        ),
    ]),
  );
  final byUrgency = call.rankAnswer();
  final expected = [inbox[1], inbox[3], inbox[2], inbox[0]];
  assert(const ListEquality<Object?>()
      .equals(byUrgency, expected));
  engine.close();
}
