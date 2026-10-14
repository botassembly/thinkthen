import 'package:thinkthen_dart/thinkthen_dart.dart';

Future<void> main() async {
  final engine = Engine.open();
  const report = 'Steps: click Log in. Nobody gets in.';
  final call = await engine.annotate(
    QuestionFile(path: 'form.json'),
    InputRecords(
      items: [Item(original: Presence.present(OriginalText(text: report)))],
    ),
  );
  final triage = (call.value as List).single;
  assert(triage['steps'] == true);
  assert(triage['area'] == 'login');
  assert(triage['impact'] == 1.98);
  engine.close();
}
