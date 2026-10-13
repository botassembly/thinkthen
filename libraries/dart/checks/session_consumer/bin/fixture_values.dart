import 'dart:convert';
import 'fixture_values_generated.dart';
export 'fixture_values_generated.dart';
/// Native JSON retains wide i64/u64 values that Dart's JSON reader rounds.
Object? decodeFixture(String source) {
  var offset = 0;
  void space() {
    while (offset < source.length && ' \r\n\t'.contains(source[offset]))
      offset++;
  }

  String string() {
    final start = offset++;
    while (offset < source.length) {
      final char = source[offset++];
      if (char == '\\') offset++;
      if (char == '"')
        return jsonDecode(source.substring(start, offset)) as String;
    }
    throw const FormatException('Unterminated JSON string');
  }

  Object? value() {
    space();
    if (offset >= source.length)
      throw const FormatException('Missing JSON value');
    final char = source[offset];
    if (char == '"') return string();
    if (char == '[' || char == '{') {
      final object = char == '{';
      final list = <Object?>[];
      final map = <String, Object?>{};
      offset++;
      space();
      final end = object ? '}' : ']';
      if (source[offset] != end) {
        while (true) {
          space();
          if (object) {
            final key = string();
            space();
            if (source[offset++] != ':')
              throw const FormatException('Missing colon');
            map[key] = value();
          } else {
            list.add(value());
          }
          space();
          if (source[offset] != ',') break;
          offset++;
        }
      }
      if (source[offset++] != end)
        throw const FormatException('Missing JSON end');
      return object ? map : list;
    }
    final start = offset;
    while (offset < source.length && !' ,]}\r\n\t'.contains(source[offset]))
      offset++;
    final token = source.substring(start, offset);
    if (RegExp(r'^-?[0-9]+$').hasMatch(token))
      return int.tryParse(token) ?? BigInt.parse(token);
    return jsonDecode(token);
  }

  final result = value();
  space();
  if (offset != source.length) throw const FormatException('Trailing JSON');
  return result;
}

String encodeFixture(Object? value) {
  value = fixtureValue(value);
  if (value is BigInt) return value.toString();
  if (value is List) return '[${value.map(encodeFixture).join(',')}]';
  if (value is Map) return '{${value.entries.map((e) => '${jsonEncode(e.key)}:${encodeFixture(e.value)}').join(',')}}';
  return jsonEncode(value);
}
