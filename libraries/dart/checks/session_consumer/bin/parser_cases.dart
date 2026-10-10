import 'dart:convert';
import 'dart:io';
import 'package:thinkthen_dart/thinkthen_dart.dart';
import 'package:thinkthen_dart/src/session/values.dart' show decodeNative;

void main(List<String> args) {
  final cases = (jsonDecode(File(args.single).readAsStringSync())
      as Map)['cases'] as List;
  final named = {for (final c in cases.cast<Map>()) c['name']: c['response']};
  final row = ((named['17-annotate-partial'] as List).single as Map)
      .map((name, member) => MapEntry(name, AnnotatedField.read(member)));
  if (row['refund']!.value != null ||
      row['team']!.asObject.failed.kind != 'backend' ||
      row['team']!.asObject.failed.cause != 'missing_probability' ||
      row['severity']!.asNumber is! num ||
      row['topics']!.asArray is! List<String>) {
    throw StateError('null and failed collapsed');
  }
  final later = AnnotatedField.read({
    'failed': {'kind': 'backend', 'cause': 'wrong_kind', 'later': 1}
  });
  if (later.asObject.failed.cause != 'wrong_kind')
    throw StateError('unknown failure field lost');
  final entity = ((named['41-offsets-past-an-accent-and-an-emoji']
          as Map)['entities'] as List)
      .single as Map;
  final scalars = 'Le café 😀 Maria Chen arrived.'.runes.toList();
  if (String.fromCharCodes(
          scalars.sublist(entity['start'] as int, entity['end'] as int)) !=
      entity['text']) {
    throw StateError('non-BMP scalar offset mismatch');
  }
  for (final malformed in ['true false', '{"x":}', '[1,]', '"unterminated']) {
    try {
      decodeNative(malformed);
      throw StateError('malformed native JSON accepted');
    } on FormatException {}
  }
  final parsed =
      decodeNative('{"wide":18446744073709551615,"null":null,"false":false}')
          as Map;
  if (parsed['wide'] != BigInt.parse('18446744073709551615') ||
      parsed['null'] != null ||
      parsed['false'] != false) {
    throw StateError('native integer or null parsing changed');
  }
  print(
      'PASS: shared annotation failure/null and Unicode cases; strict native JSON and exact integers');
}
