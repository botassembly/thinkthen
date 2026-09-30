import 'dart:convert';
import 'dart:io';

import 'package:thinkthen_dart/thinkthen_dart.dart';

// Results are host JSON; schema.py holds each case's verdict. This reads the
// shared annotate case through readField and case 41's offsets as Unicode
// scalars.
void main(List<String> args) {
  final cases = (jsonDecode(File(args.single).readAsStringSync())
      as Map)['cases'] as List;
  final named = {for (final c in cases.cast<Map>()) c['name']: c['response']};
  final row = ((named['17-annotate-partial'] as List).single as Map)
      .map((name, member) => MapEntry(name, readField(member)));
  final team = row['team'];
  if (row['refund'] is! UnresolvedField ||
      team is! FailedField ||
      team.kind != 'backend' ||
      team.cause != 'missing_probability' ||
      row['severity'] is! AnswerField ||
      row['topics'] is! AnswerField) {
    throw StateError('null and failed collapsed');
  }
  final later = readField({
    'failed': {'kind': 'backend', 'cause': 'wrong_kind', 'later': 1},
  });
  if (later is! FailedField || later.cause != 'wrong_kind') {
    throw StateError('a failure with an unknown member');
  }
  try {
    readField({'team': 'billing'});
    throw StateError('an object read as an answer');
  } on FormatException {/* expected */}
  print('ANNOTATE_NULL_AND_FAILED_PASS case 17 and an unknown member');
  final entity = ((named['41-offsets-past-an-accent-and-an-emoji']
          as Map)['entities'] as List)
      .single as Map;
  final text = 'Le café 😀 Maria Chen arrived.';
  final scalars = text.runes.toList();
  final start = entity['start'] as int, end = entity['end'] as int;
  if (String.fromCharCodes(scalars.sublist(start, end)) != entity['text'] ||
      start != 10 ||
      end != 20 ||
      entity['length'] != 10 ||
      text.codeUnits.indexOf(0xD83D) != 8 ||
      text.codeUnits.length != scalars.length + 1) {
    throw StateError('non-BMP scalar offset mismatch');
  }
  print('NON_BMP_SCALAR_OFFSET_PASS case 41 [10,20)');
}
