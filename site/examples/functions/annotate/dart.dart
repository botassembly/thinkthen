import 'dart:convert';
import 'dart:io';

import 'package:thinkthen_dart/thinkthen_dart.dart';

void main() {
  final library = File('thinkthen-c/lib/libthinkthen.so');
  final tt = Door(library.absolute.path);
  final engine = tt.create();
  try {
    final form = File('form.json').readAsStringSync();
    final triage = tt.ask(engine, {
      'annotate': jsonDecode(form),
      'records': [
        'Steps: click Export. It is very slow.',
        'Steps: click Log in. Nobody gets in.',
        'The Pay button on billing is too blue.',
      ],
    }) as Map;
    final rows = triage['value'] as List;
    assert(jsonEncode(rows[0]) ==
        '{"steps":true,"area":"export","impact":1.04}');
    assert(jsonEncode(rows[1]) ==
        '{"steps":true,"area":"login","impact":1.98}');
    assert(jsonEncode(rows[2]) ==
        '{"steps":false,"area":"billing","impact":0.09}');
  } finally {
    tt.engineFree(engine);
  }
}
