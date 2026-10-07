import 'dart:convert';
import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:thinkthen_flutter/thinkthen_complete_flutter.dart';
import '../../../checks/consumers/alpha/bin/complete_native.dart' show consume;
import '../../../checks/consumers/alpha/bin/native_encode.dart';

void main() {
  test(
      'Flutter executes named complete native functions and retains typed copies',
      () async {
    final env = Platform.environment;
    Directory.current = env['TT_CASE_HOME']!;
    ThinkThenCompleteFlutter? engine;
    Map<String, Object?> output;
    try {
      engine = ThinkThenCompleteFlutter(env['TT_NATIVE_LIBRARY']!,
          settingsJson: env['TT_SETTINGS']!);
      final v = jsonDecode(File(env['TT_INPUT']!).readAsStringSync())
          as Map<String, dynamic>;
      output = await consume(v, engine, env['TT_NATIVE_LIBRARY']!);
    } on CompleteFailure catch (e) {
      output = {'failure': encodeNative(e.summary)};
    } finally {
      engine?.close();
    }
    File(env['TT_OUTPUT']!).writeAsStringSync(jsonEncode(output));
  });
}
