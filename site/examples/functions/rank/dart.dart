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
    InputRequestQuestionText(text: question),
    InputRequestInputRecords(items: [
      for (final message in inbox)
        InputRequestItem(
          original: Presence.present(
            InputRequestOriginalText(text: message),
          ),
        ),
    ]),
  );
  final byUrgency = call.packets
      .whereType<SessionPacketRankAggregate>()
      .single
      .value;
  final order = [
    for (final one in byUrgency) one.index.value!.toInt(),
  ];
  const expected = [1, 3, 2, 0];
  assert(const ListEquality<int>().equals(order, expected));
  engine.close();
}
