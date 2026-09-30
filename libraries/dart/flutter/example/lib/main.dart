import 'dart:io';

import 'package:flutter/material.dart';
import 'package:thinkthen_flutter/thinkthen_flutter.dart';

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
  print(
    'FLUTTER_EMBEDDER_PASS outcome=${answer.value.outcome} probability=${answer.value.probability}',
  );
  runApp(
    const MaterialApp(
      home: Scaffold(body: Center(child: Text('ThinkThen Flutter FFI pass'))),
    ),
  );
}
