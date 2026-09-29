import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:thinkthen_flutter/thinkthen_flutter.dart';

import '../lib/strict.dart' as strict;

void main() {
  testWidgets('Flutter toolchain runs strict Dart FFI consumer', (
    tester,
  ) async {
    final library = Platform.environment['TT_NATIVE_LIBRARY'];
    if (library == null) throw StateError('TT_NATIVE_LIBRARY required');
    final answer = ThinkThenFlutter(library).decide('Is it?', 'flutter-facade');
    expect(answer.outcome, Outcome.yes);
    expect(answer.probability, .9);
    print('FLUTTER_FACADE_PASS');
    await tester.runAsync(() async => strict.main([library]));
  });
}
