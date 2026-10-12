import 'package:thinkthen_dart/thinkthen_dart.dart';

Future<void> main() async {
  final engine = Engine.open();
  const question = 'Which team owns this?';
  const teams = {
    'billing': 'Invoices, fees, and refunds.',
    'shipping': 'Parcels and delivery.',
    'account': 'Logins and passwords.',
  };
  const parcel = 'My parcel went to the wrong address.';
  final call = await engine.choose(
    InputRequestQuestionDefinition(
      value: InputRequestDefinition.read({
        'choose': question,
        'options': teams,
      }),
    ),
    InputRequestInputText(text: parcel),
  );
  final team = call.packets
      .whereType<SessionPacketChooseRow>()
      .single
      .value
      .value;
  assert(team == 'shipping');
  engine.close();
}
