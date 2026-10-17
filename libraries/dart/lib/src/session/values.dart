part of thinkthen_session;

/// Presence keeps omission separate from explicit null.
final class Presence<T> {
  final bool isPresent;
  final T? value;
  const Presence.absent() : isPresent = false, value = null;
  const Presence.present(this.value) : isPresent = true;
}

abstract interface class _EncodedValue {
  Object? _toValue();
}

abstract class _NativeValue implements _EncodedValue {
  final Map<String, Object?> _fields;
  _NativeValue(Map<String, Object?> value)
    : _fields = Map.unmodifiable(
        value.map((key, value) => MapEntry(key, _freeze(value))),
      );
  Map<String, Object?> _toValue() => _fields;
}

Object? _freeze(Object? value) {
  if (value is Map)
    return Map<String, Object?>.unmodifiable(
      value.map((key, value) => MapEntry(key as String, _freeze(value))),
    );
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

/// One completed call retains its answer, odds, native records and terminal facts.
final class CallResult<V, R> {
  final V value;
  final CallProbability? probability;
  final List<R> results;
  final Facts facts;
  final OwnedCall _call;
  CallResult._(this.value, this.probability, List<R> results, this._call)
    : results = List.unmodifiable(results),
      facts =
          _call.terminal.facts.value ??
          (throw StateError('Completed call has no facts'));
  List<SessionPacket> get packets => _call.packets;
  SessionPacketTerminal get terminal => _call.terminal;
}

sealed class CallProbability {
  const CallProbability();
}

final class ScalarProbability extends CallProbability {
  final double value;
  const ScalarProbability(this.value);
}

final class ListProbability extends CallProbability {
  final List<double?> value;
  ListProbability(List<double?> values) : value = List.unmodifiable(values);
}

final class GroupProbability extends CallProbability {
  final List<List<double?>> value;
  GroupProbability(List<List<double?>> values)
    : value = List.unmodifiable(
        values.map((v) => List<double?>.unmodifiable(v)),
      );
}

Object? _atPath(Object? value, List<String> path) {
  for (final key in path) {
    if (value is! Map) return null;
    value = value[key];
  }
  return value;
}

double? _numberProbability(Object? value) => (value as num?)?.toDouble();
ScalarProbability? _scalarProbability(double? value) =>
    value == null ? null : ScalarProbability(value);

List<double?> _recognitionProbability(Recognition row) {
  final value = row.value;
  if (value is RecognizeModeProposals)
    return value.proposals
        .map((proposal) => proposal.probability.toDouble())
        .toList(growable: false);
  final entities = (value as RecognizeEntities).entities;
  final answer = row.answer as RecognitionOddsRecognizeAnswer;
  return entities
      .map((entity) {
        for (final proposal in answer.proposals) {
          final selected = proposal.selected.value;
          if (proposal.kept &&
              selected != null &&
              selected.start == entity.start &&
              selected.end == entity.end &&
              proposal.kind.value == entity.kind)
            return proposal.spanProbability.toDouble();
        }
        return null;
      })
      .toList(growable: false);
}
