import 'dart:convert';
import 'dart:io';

import 'package:thinkthen_dart/thinkthen_dart.dart';

void main() {
  final library = File('thinkthen-c/lib/libthinkthen.so');
  final tt = Door(library.absolute.path);
  final engine = tt.create();
  try {
    final spec = jsonEncode({
      'version': 1,
      'recognize': {
        'kinds': {
          'person': null,
          'organization': null,
          'place': null,
        },
      },
    });
    const text = 'Maria Chen joined Northwind Freight, '
        'a company in Chicago.';
    final facts = tt.recognize(engine, spec, text);
    final names = [
      for (final one in (facts.value as Map)['entities'])
        '${one['text']} ${one['kind']}',
    ];
    assert(names[0] == 'Maria Chen person');
    assert(names[1] == 'Northwind Freight organization');
    assert(names[2] == 'Chicago place');
  } finally {
    tt.engineFree(engine);
  }
}
