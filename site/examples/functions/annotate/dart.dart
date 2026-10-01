import 'dart:convert';
import 'dart:io';

import 'package:thinkthen_dart/thinkthen_dart.dart';

void main() {
  final library = File('thinkthen-c/lib/libthinkthen.so');
  final tt = Door(library.absolute.path);
  final engine = tt.create();
  try {
    final form = File('form.json').readAsStringSync();
    const report = 'Steps: click Log in. Nobody gets in.';
    final rows = tt.ask(engine, {
      'annotate': jsonDecode(form),
      'records': [report],
    }) as Map;
    final triage = rows['value'] as List;
    assert(triage.length == 1);
    assert(jsonEncode(triage[0]) ==
        '{"steps":true,"area":"login","impact":1.98}');
  } finally {
    tt.engineFree(engine);
  }
}
