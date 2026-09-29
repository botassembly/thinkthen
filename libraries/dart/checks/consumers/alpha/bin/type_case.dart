import 'dart:convert';
import 'dart:io';
import 'package:thinkthen_dart/thinkthen_dart.dart';

void main(List<String> args) {
  if (args.length != 1) throw ArgumentError('one request JSON required');
  final library = Platform.environment['TT_NATIVE_LIBRARY'];
  if (library == null) throw StateError('TT_NATIVE_LIBRARY required');
  final door = Door(library);
  final engine = door.create();
  try {
    try {
      final request = jsonDecode(args.single) as Map<String, Object?>;
      stdout.writeln(jsonEncode(door.ask(engine, request)));
    } on DoorFailure catch (error) {
      stdout.writeln(jsonEncode({'error': error.kind.name}));
    }
  } finally {
    door.engineFree(engine);
  }
}
