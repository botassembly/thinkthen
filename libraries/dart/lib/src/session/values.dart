import 'dart:convert';

/// Presence keeps omission separate from explicit null.
final class Presence<T> {
  final bool isPresent;
  final T? value;
  const Presence.absent()
      : isPresent = false,
        value = null;
  const Presence.present(this.value) : isPresent = true;
}

abstract class NativeObject {
  final Map<String, Object?> json;
  NativeObject(Map<String, Object?> value)
      : json = Map.unmodifiable(
            value.map((key, value) => MapEntry(key, _freeze(value))));
  Map<String, Object?> toJson() => json;
}

Object? _freeze(Object? value) {
  if (value is Map)
    return Map<String, Object?>.unmodifiable(
        value.map((key, value) => MapEntry(key as String, _freeze(value))));
  if (value is List) return List<Object?>.unmodifiable(value.map(_freeze));
  return value;
}

Map<String, Object?> readObject(Object? value) => value is NativeObject
    ? value.json
    : Map<String, Object?>.from(value as Map);
BigInt readInteger(Object? value) {
  if (value is BigInt) return value;
  if (value is int) return BigInt.from(value);
  throw const FormatException('Expected an integer');
}

/// Encode BigInt exactly, including integers inside caller-authored JSON.
String encodeNative(Object? value) {
  if (value is NativeObject) return encodeNative(value.toJson());
  if (value is BigInt) return value.toString();
  if (value is List) return '[${value.map(encodeNative).join(',')}]';
  if (value is Map) {
    return '{${value.entries.map((e) => '${jsonEncode(e.key)}:${encodeNative(e.value)}').join(',')}}';
  }
  if (value != null && value is! String && value is! num && value is! bool) {
    return encodeNative((value as dynamic).toJson());
  }
  return jsonEncode(value);
}

/// Native JSON retains wide i64/u64 values that Dart's JSON reader rounds.
Object? decodeNative(String source) {
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
