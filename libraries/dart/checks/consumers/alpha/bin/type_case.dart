import 'dart:convert';
import 'dart:ffi';
import 'dart:io';
import 'package:thinkthen_dart/thinkthen_dart.dart';

// With one argument, it is one JSON-door request and stdout its reply.
// "fields REQUEST" reads each annotate row member through readField; "plan
// INPUT" reads one thinkthen.plan-input/1 object and prints Door.plan's
// object; "limits" checks ticket 0291's zero budgets and zero cap.
void main(List<String> args) {
  final library = Platform.environment['TT_NATIVE_LIBRARY'];
  if (library == null) throw StateError('TT_NATIVE_LIBRARY required');
  final mode = args.length == 2 || args.first == 'limits' ? args.first : '';
  final door = Door(library);
  final engine = door.create();
  try {
    try {
      stdout.writeln(jsonEncode(switch (mode) {
        'plan' => plan(door, engine, jsonDecode(args.last) as Map),
        'fields' => fields(door.call(engine, args.last) as Map),
        'limits' => limits(door, engine),
        _ => door.ask(engine, jsonDecode(args.single) as Map<String, Object?>),
      }));
    } on DoorFailure catch (error) {
      stdout.writeln(jsonEncode({
        'failed': {'kind': error.kind.name, 'code': error.kind.code},
      }));
    }
  } finally {
    door.engineFree(engine);
  }
}

Object? plan(Door door, Pointer<Void> engine, Map input) => door.plan(
      engine,
      input['verb'] as String,
      input['question'] as Object,
      (input['input'] as List).cast<String>(),
      (input['settings'] as Map?)?.cast<String, Object?>(),
    );

List<Map<String, String>> fields(Map reply) => [
      for (final row in (reply['value'] as List).cast<Map>())
        row.map((name, member) => MapEntry(
            name as String,
            switch (readField(member)) {
              UnresolvedField() => 'unresolved',
              AnswerField() => 'answered',
              FailedField(:final kind, :final cause) => 'failed $kind $cause',
            })),
    ];

// A zero cap and a zero budget each refuse before sending. Relate gets two
// entities, since one entity has no pair to ask.
Map<String, String> limits(Door door, Pointer<Void> engine) {
  DoorFailure refused(void Function() call) {
    try {
      call();
    } on DoorFailure catch (failure) {
      return failure;
    }
    throw StateError('a limited call succeeded');
  }

  final capped = door.create('{"max_requests_total":0,"cache":false}');
  try {
    final cap = refused(() => door.decide(capped, 'Is it?', 'capped'));
    if (cap.kind != ErrorKind.usage ||
        cap.kind.code != 1 ||
        !cap.message.contains('process send budget')) {
      throw StateError('cap: $cap');
    }
  } finally {
    door.engineFree(capped);
  }
  for (final call in <void Function()>[
    () => door.call(engine, '{"decide":"Is it?","evidence":"zero-call"}',
        deadline: 0),
    () => door.recognize(
        engine,
        '{"version":1,"recognize":{"kinds":{"person":"A person name."}}}',
        'zero-recognize',
        deadline: 0),
    () => door.relate(
        engine,
        '{"version":1,"relate":{"relations":[{"name":"caused_by","source":"alert","target":"alert"}]}}',
        ['{"name":"A","kind":"alert"}', '{"name":"B","kind":"alert"}'],
        deadline: 0),
  ]) {
    final zero = refused(call);
    if (zero.kind != ErrorKind.deadline || zero.kind.code != 3) {
      throw StateError('zero budget: $zero');
    }
  }
  return {'limits': 'pass'};
}
