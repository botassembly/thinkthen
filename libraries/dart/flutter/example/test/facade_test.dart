import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:thinkthen_flutter/thinkthen_flutter.dart';

// Dart consumer bravo covers the strict door behaviors under plain Dart.
void main() {
  testWidgets('Flutter toolchain runs the facade call', (tester) async {
    final library = Platform.environment['TT_NATIVE_LIBRARY'];
    if (library == null) throw StateError('TT_NATIVE_LIBRARY required');
    final answer = ThinkThenFlutter(library).decide('Is it?', 'flutter-facade');
    expect(answer.value.outcome, Outcome.yes);
    expect(answer.value.probability, .9);
    expect(answer.facts['records'], 1);
    expect(answer.facts['requests_sent'], 1);
    print('FLUTTER_FACADE_PASS');
  });
}
