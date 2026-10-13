part of thinkthen_session;

/// Presence keeps omission separate from explicit null.
final class Presence<T> {
  final bool isPresent;
  final T? value;
  const Presence.absent()
      : isPresent = false,
        value = null;
  const Presence.present(this.value) : isPresent = true;
}

abstract interface class _EncodedValue {
  Object? _toValue();
}

abstract class _NativeValue implements _EncodedValue {
  final Map<String, Object?> _fields;
  _NativeValue(Map<String, Object?> value)
      : _fields = Map.unmodifiable(
            value.map((key, value) => MapEntry(key, _freeze(value))));
  Map<String, Object?> _toValue() => _fields;
}

Object? _freeze(Object? value) {
  if (value is Map)
    return Map<String, Object?>.unmodifiable(
        value.map((key, value) => MapEntry(key as String, _freeze(value))));
  if (value is List) return List<Object?>.unmodifiable(value.map(_freeze));
  return value;
}

Map<String, Object?> _readObject(Object? value) => value is _NativeValue
    ? value._fields
    : Map<String, Object?>.from(value as Map);
BigInt _readInteger(Object? value) {
  if (value is BigInt) return value;
  if (value is int) return BigInt.from(value);
  throw const FormatException('Expected an integer');
}

/// Encode BigInt exactly, including integers inside caller-authored JSON.
String _encodeNative(Object? value) {
  if (value is _EncodedValue) return _encodeNative(value._toValue());
  if (value is BigInt) return value.toString();
  if (value is List) return '[${value.map(_encodeNative).join(',')}]';
  if (value is Map) {
    return '{${value.entries.map((e) => '${jsonEncode(e.key)}:${_encodeNative(e.value)}').join(',')}}';
  }
  if (value != null && value is! String && value is! num && value is! bool) {
    return _encodeNative((value as dynamic).toJson());
  }
  return jsonEncode(value);
}
