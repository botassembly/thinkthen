import 'dart:io';

import 'package:thinkthen_dart/thinkthen_dart.dart';

void main() {
  final library = File('thinkthen-c/lib/libthinkthen.so');
  final tt = Door(library.absolute.path);
  final engine = tt.create();
  try {
    final urgency = tt.ask(engine, {
      'score': 'How urgent is this?',
      'levels': ['Routine.', 'Soon.', 'Immediate.'],
      'evidence': 'Our checkout page is down '
          'and customers cannot pay.\n',
    }) as Map;
    final level = urgency['value'];
    assert(level == 2.0);
  } finally {
    tt.engineFree(engine);
  }
}
