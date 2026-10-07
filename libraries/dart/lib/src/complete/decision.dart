import 'values.dart';
import 'read.dart';

/// The complete C view supplies this discriminator, including authored Boolean/null.
sealed class DecisionValue extends Carrier {
  const DecisionValue();
}

final class BooleanDecision extends DecisionValue {
  final bool value;
  const BooleanDecision(this.value);
  @override
  bool toJson() => value;
}

final class AuthoredDecision extends DecisionValue {
  final JsonContent meaning;
  const AuthoredDecision(this.meaning);
  @override
  Object? toJson() => meaning.toJson();
}

final class UnresolvedDecision extends DecisionValue {
  const UnresolvedDecision();
  @override
  Object? toJson() => null;
}

DecisionValue readDecision(Object? value, Object? question) {
  final q = readObject(question);
  if (q.containsKey('true') || q.containsKey('false')) {
    // JSON cannot identify authored null versus unresolved: never guess a reading.
    if (value == null) invalid();
    return AuthoredDecision(JsonContent.fromJson(value));
  }
  return value == null
      ? const UnresolvedDecision()
      : BooleanDecision(readBool(value));
}

ActionValue readAction(Object? value, Object? question) {
  final q = readObject(question);
  if (q['verb'] == 'decide' &&
      (q.containsKey('true') || q.containsKey('false'))) {
    if (value == null) invalid();
    return AuthoredValue(JsonContent.fromJson(value));
  }
  final valid = switch (q['verb']) {
    'decide' => value is bool || value == null,
    'choose' => value is String || value == null,
    'tag' => value is List,
    'score' => value is num,
    _ => false,
  };
  if (!valid) invalid();
  return ActionValue.fromJson(value);
}

Map<String, ActionValue> readAnnotationValues(Object? value, Object? answers) {
  final v = readObject(value), entries = readObject(answers);
  if (v.length != entries.length || v.keys.any((k) => !entries.containsKey(k)))
    invalid();
  return Map.unmodifiable(v.map((k, x) {
    final e = readObject(entries[k]);
    return MapEntry(
        k,
        e.containsKey('failure_id')
            ? ActionValue.fromJson(x)
            : readAction(x, e['question']));
  }));
}
