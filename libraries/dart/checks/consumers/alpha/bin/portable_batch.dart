import 'dart:convert';
import 'dart:io';

import 'package:thinkthen_dart/thinkthen_dart.dart';

void main(List<String> args) {
  if (args.length != 2)
    throw ArgumentError('native library and corpus required');
  final corpus =
      jsonDecode(File(args[1]).readAsStringSync()) as Map<String, dynamic>;
  final door = Door(args[0]);
  final engine = door.create();
  try {
    final answers = door.many(
      engine,
      corpus['question'] as String,
      (corpus['texts'] as List<dynamic>).cast<String>(),
    );
    if (answers.value.length != 5 ||
        answers.facts['records'] != 5 ||
        answers.facts['requests_sent'] != 1 ||
        answers.value.any(
          (answer) => answer.outcome != Outcome.yes || answer.probability != .9,
        )) {
      throw StateError('portable bulk answers changed');
    }
    print('DART_PORTABLE_BATCH_PASS');
  } finally {
    door.engineFree(engine);
  }
}
