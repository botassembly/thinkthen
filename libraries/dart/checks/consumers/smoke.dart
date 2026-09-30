// The replay smoke (ticket 0335): one decide through the environment-reading
// engine, with the question and text sdlc/scripts/smoke names.
import 'dart:io';

import 'package:thinkthen_dart/thinkthen_dart.dart';

void main(List<String> args) {
  final door = Door(args[0]);
  final engine = door.create();
  final env = Platform.environment;
  final answer = door.decide(
    engine,
    env['THINKTHEN_SMOKE_QUESTION']!,
    env['THINKTHEN_SMOKE_TEXT']!,
  );
  door.engineFree(engine);
  final outcome = answer.value.outcome;
  print(
    'smoke: ${outcome == Outcome.yes ? 'true' : outcome == Outcome.no ? 'false' : 'null'}',
  );
}
