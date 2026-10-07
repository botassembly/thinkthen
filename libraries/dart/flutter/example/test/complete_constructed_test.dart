import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:thinkthen_flutter/thinkthen_complete_flutter.dart';
import '../../../checks/consumers/alpha/bin/complete_constructed.dart'
    show constructed;

void main() {
  test(
      'Flutter executes counted typed question constructors for every function',
      () {
    final env = Platform.environment;
    final engine = ThinkThenCompleteFlutter(env['TT_NATIVE_LIBRARY']!,
        settingsJson: env['TT_SETTINGS']!);
    try {
      final results = constructed(engine);
      engine.close();
      expect(results.length, 10);
      for (final result in results) {
        expect(result.summary.schema.data, 'thinkthen.result/2');
      }
    } finally {
      engine.close();
    }
  });
}
