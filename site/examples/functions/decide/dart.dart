import 'package:thinkthen_dart/thinkthen_dart.dart';

Future<void> main() async {
  final engine = Engine.open();
  const question = 'Does the customer ask for a refund?';
  const broken =
      'Please refund my order. It arrived broken.';
  final call = await engine.decide(
    QuestionText(text: question),
    InputText(text: broken),
  );
  final isRefund = call.value;
  assert(isRefund == true);
  engine.close();
}
