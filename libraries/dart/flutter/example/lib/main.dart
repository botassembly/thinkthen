import 'dart:io';

import 'package:flutter/material.dart';
import 'package:thinkthen_flutter/thinkthen_flutter.dart';
import 'package:thinkthen_flutter/thinkthen_complete_flutter.dart' as c;

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  final library = Platform.environment['TT_NATIVE_LIBRARY'];
  if (library == null) throw StateError('TT_NATIVE_LIBRARY required');
  final answer = ThinkThenFlutter(library).decide('Is it?', 'flutter-embedder');
  if (answer.value.outcome != Outcome.yes ||
      answer.value.probability != .9 ||
      answer.facts['records'] != 1 ||
      answer.facts['requests_sent'] != 1) {
    throw StateError('FLUTTER_EMBEDDER_RESULT_MISMATCH: $answer');
  }
  final native =
      c.ThinkThenCompleteFlutter(library, settingsJson: '{"cache":false}');
  late c.CompleteResult<c.DecideView> complete;
  try {
    final question = c.Question.spec(c.QuestionSpec(c.FunctionKind.decide,
        text: const c.Content.text('Is it?')));
    final source =
        c.Records([c.Record(const c.Content.text('flutter-embedder'))]);
    complete = native.decide(question, source);
    if (complete.rows.single.value.data.boolean != 1 ||
        complete.rows.single.common.answer.value!.data.probability != .9 ||
        complete.summary.facts.value!.requests_sent != BigInt.one)
      throw StateError('FLUTTER_COMPLETE_MISMATCH');
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
  if (complete.rows.single.common.answer_id.data.length != 64)
    throw StateError('installed complete copy expired');
  try {
    c.ThinkThenCompleteFlutter(library, settingsJson: '{"unknown":true}');
    throw StateError('invalid settings accepted');
  } on c.CompleteFailure catch (failure) {
    if (failure.kind != ErrorKind.usage || failure.summary.facts.present != 0)
      rethrow;
  }
  print(
    'FLUTTER_EMBEDDER_PASS outcome=${answer.value.outcome} probability=${answer.value.probability}',
  );
  runApp(
    const MaterialApp(
      home: Scaffold(body: Center(child: Text('ThinkThen Flutter FFI pass'))),
    ),
  );
}
