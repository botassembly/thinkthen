import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import '../lib/parity_consumer.dart' as caller;

void main() {
  test('shared typed caller case', () async {
    await caller.main([
      Platform.environment['TT_INPUT']!,
      '',
      Platform.environment['TT_SETTINGS']!
    ]);
  });
}
