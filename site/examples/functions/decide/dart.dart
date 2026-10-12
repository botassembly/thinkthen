import 'package:thinkthen_dart/thinkthen_dart.dart';

Future<void> main() async {
  final engine = Engine.open();
  const question = 'Does the customer ask for a refund?';
  const broken =
      'Please refund my order. It arrived broken.';
  final call = await engine.decide(
    InputRequestQuestionText(text: question),
    InputRequestInputText(text: broken),
  );
  final isRefund = call.packets
      .whereType<SessionPacketDecideRow>()
      .single
      .value
      .value
      .asAlternative0;
  assert(isRefund == true);
  engine.close();
}
