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
      'relate': {
        'relations': [
          {
            'name': 'sings',
            'source': 'singer',
            'target': 'song',
          },
        ],
      },
    });
    final sings = tt.relate(engine, spec, [
      '{"name": "Paul McCartney", "kind": "singer"}',
      '{"name": "Ringo Starr", "kind": "singer"}',
      '{"name": "Yesterday", "kind": "song"}',
      '{"name": "Octopus\'s Garden", "kind": "song"}',
    ]);
    final pairs = [
      for (final edge in (sings.value as Map)['edges'])
        [edge['source']['name'], edge['target']['name']],
    ];
    assert(jsonEncode(pairs) ==
        '[["Paul McCartney","Yesterday"],'
            '["Ringo Starr","Octopus\'s Garden"]]');
  } finally {
    tt.engineFree(engine);
  }
}
