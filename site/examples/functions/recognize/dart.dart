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
          'PER': "Part of a person's name.",
          'ORG': 'Part of the name of an organization: '
              'a company, band, team, agency, government '
              'body, or media outlet.',
          'LOC': 'Part of the name of a place: a country, '
              'region, city, or geographic feature.',
          'MISC': 'Part of another named entity: a '
              'nationality, an event, a product, or the '
              'name of a creative work.',
        },
      },
    });
    final names = tt.recognize(
      engine,
      spec,
      'Maria Chen joined Northwind Freight in Chicago '
          'last spring.',
    );
    final spans = [
      for (final one in (names.value as Map)['entities'])
        '${one['text']} ${one['kind']}',
    ];
    assert(spans[0] == 'Maria Chen PER');
    assert(spans[1] == 'Northwind Freight ORG');
    assert(spans[2] == 'Chicago LOC');
  } finally {
    tt.engineFree(engine);
  }
}
