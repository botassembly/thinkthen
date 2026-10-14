import 'package:thinkthen_dart/thinkthen_dart.dart';

Future<void> main() async {
  final engine = Engine.open();
  const question = 'Which line gives the refund deadline?';
  const policy = [
    'Returns need the original receipt.',
    'Refunds are issued within 30 days of purchase.',
    'Shipping is free on orders over \$50.',
    'Gift cards cannot be exchanged for cash.',
  ];
  final call = await engine.find(
    QuestionText(text: question),
    InputUnits(
      items: [
        for (final line in policy)
          Item(original: Presence.present(
            OriginalText(text: line),
          )),
      ],
    ),
  );
  final refundDeadline = call.value;
  assert(refundDeadline == policy[1]);
  engine.close();
}
