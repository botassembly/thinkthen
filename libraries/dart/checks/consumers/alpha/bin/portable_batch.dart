import 'dart:convert';
import 'dart:io';

import 'package:thinkthen_dart/thinkthen_dart.dart';
import 'package:thinkthen_dart/thinkthen_complete.dart' as c;

void main(List<String> args) {
  if (args.length != 2)
    throw ArgumentError('native library and corpus required');
  final corpus =
      jsonDecode(File(args[1]).readAsStringSync()) as Map<String, dynamic>;
  final door = Door(args[0]);
  final engine = door.create(Platform.environment['TT_PORTABLE_SETTINGS']);
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
  final native = c.Engine(args[0],
      settingsJson: jsonEncode({
        'batch': 'max',
        'cache': false,
        ...jsonDecode(Platform.environment['TT_PORTABLE_SETTINGS'] ?? '{}')
            as Map<String, dynamic>
      }));
  late c.CompleteResult<c.DecideView> result;
  try {
    final question = c.Question.spec(c.QuestionSpec(c.FunctionKind.decide,
        text: c.Content.text(corpus['question'] as String)));
    final source = c.Records((corpus['texts'] as List)
        .cast<String>()
        .map((text) => c.Record(c.Content.text(text)))
        .toList());
    result = native.decide(question, source);
    if (result.rows.length != 5 ||
        result.summary.facts.value!.requests_sent != BigInt.one ||
        result.rows.any((row) =>
            row.value.data.boolean != 1 ||
            row.common.answer.value!.data.probability != .9))
      throw StateError('installed complete answers/facts changed');
    final token = native.cancellation();
    try {
      token.fire();
      try {
        native.decide(question, source, controls: c.Controls(cancel: token));
        throw StateError('spent token accepted');
      } on c.CompleteFailure catch (failure) {
        if (failure.kind != ErrorKind.cancelled ||
            failure.summary.facts.present != 0) rethrow;
      }
    } finally {
      token.close();
    }
  } finally {
    native.close();
  }
  if (result.rows.first.common.answer_id.data.length != 64)
    throw StateError('installed complete copy expired');
  try {
    c.Engine(args[0], settingsJson: '{"unknown":true}');
    throw StateError('invalid settings accepted');
  } on c.CompleteFailure catch (failure) {
    if (failure.kind != ErrorKind.usage || failure.summary.facts.present != 0)
      rethrow;
  }
  print(
      'DART_INSTALLED_COMPLETE_PASS five typed rows, cancellation and usage refuse before sending');
}
