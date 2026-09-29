import 'dart:convert';
import 'dart:io';

import 'package:thinkthen_dart/thinkthen_dart.dart';

void main(List<String> args) {
  final cases = (jsonDecode(File(args.single).readAsStringSync())
      as Map)['cases'] as List;
  var seen = 0;
  for (final raw in cases) {
    final c = raw as Map;
    final definition = c['definition'];
    final response = c['response'];
    if (!c.containsKey('response') ||
        ![
          'annotate',
          'recognize',
          'relate',
          'annotatedRow',
        ].contains(definition)) continue;
    final valid = c['response_valid'] != false;
    try {
      switch (definition) {
        case 'annotate':
          final a = Annotation.parse(response);
          if (c['name'] == '17-annotate-partial') {
            if ((a.rows.single['refund'] as AnswerField).value != null ||
                (a.rows.single['team'] as FailedField).cause !=
                    'missing_probability') {
              throw StateError('null and failed collapsed');
            }
          }
        case 'recognize':
          final r = Recognition.parse(response);
          if (c['name'] == '41-offsets-past-an-accent-and-an-emoji') {
            final text = 'Le café 😀 Maria Chen arrived.';
            final e = r.entities.single;
            final scalars = text.runes.toList();
            if (String.fromCharCodes(scalars.sublist(e.start, e.end)) !=
                    e.text ||
                e.start != 10 ||
                e.end != 20 ||
                e.length != 10 ||
                text.codeUnits.indexOf(0xD83D) != 8 ||
                text.codeUnits.length != scalars.length + 1) {
              throw StateError('non-BMP scalar offset mismatch');
            }
            print('NON_BMP_SCALAR_OFFSET_PASS case 41 [10,20)');
          }
        case 'relate':
          Relations.parse(response);
        case 'annotatedRow':
          Annotation.parse([response]);
      }
      if (!valid)
        throw StateError('invalid $definition shape accepted ${c['name']}');
    } on FormatException {
      if (valid) rethrow;
    }
    seen++;
  }
  if (seen != 5) throw StateError('typed corpus coverage $seen');
  print('DART_STRUCTURAL_PARITY_PASS $seen typed samples');
}
