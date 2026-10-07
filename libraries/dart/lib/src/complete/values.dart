// Private typed values. No credentials, native handles or fabricated identities.
import 'models.dart';
import 'read.dart';

abstract class Carrier {
  const Carrier();
  Object? toJson();
  @override
  String toString() => '<$runtimeType: content withheld>';
}

final class Optional<T> {
  final bool present;
  final T? _value;
  const Optional.absent()
      : present = false,
        _value = null;
  const Optional.present(T value)
      : present = true,
        _value = value;
  T get value {
    if (!present) throw StateError('absent field');
    return _value as T;
  }

  @override
  String toString() => '<Optional: present=$present>';
}

abstract class Identity extends Carrier {
  final String value;
  Identity(this.value) {
    if (!RegExp(r'^[0-9a-f]{64}$').hasMatch(value) || value.length != 64)
      invalid();
  }
  @override
  String toJson() => value;
}

final class CallId extends Identity {
  CallId(super.value);
}

final class SdkRequestId extends Identity {
  SdkRequestId(super.value);
}

final class ObservationId extends Identity {
  ObservationId(super.value);
}

final class FailureId extends Identity {
  FailureId(super.value);
}

final class AnswerId extends Identity {
  AnswerId(super.value);
}

final class Digest extends Identity {
  Digest(super.value);
}

final class Authored extends Carrier {
  final Object? value;
  const Authored._(this.value);
  factory Authored.fromJson(Object? v) {
    if (v != null && v is! String && v is! List && v is! Map) invalid();
    return Authored._(readJson(v));
  }
  @override
  Object? toJson() => value;
}

final class Threshold extends Carrier {
  final double? cut, low, high;
  final Object? _original;
  const Threshold._(this.cut, this.low, this.high, this._original);
  factory Threshold.fromJson(Object? v) {
    if (v == null) return const Threshold._(null, null, null, null);
    if (v is num && v.isFinite && v > 0 && v <= 1) {
      return Threshold._(v.toDouble(), null, null, v);
    }
    if (v is String) {
      final match =
          RegExp(r'^(0(?:\.[0-9]+)?|1(?:\.0+)?):(0(?:\.[0-9]+)?|1(?:\.0+)?)$')
              .firstMatch(v);
      if (match != null && match.group(0) == v) {
        final low = double.parse(match.group(1)!),
            high = double.parse(match.group(2)!);
        if (low < high) return Threshold._(null, low, high, v);
      }
    }
    return invalid();
  }
  @override
  Object? toJson() => _original;
}

final class Batch extends Carrier {
  final int? records;
  const Batch._(this.records);
  factory Batch.fromJson(Object? v) =>
      v == 'max' ? const Batch._(null) : Batch._(readInt(v, 1, maxInteger));
  @override
  Object toJson() => records ?? 'max';
}

final class Pointers extends Carrier {
  final List<String> values;
  final bool single;
  const Pointers._(this.values, this.single);
  factory Pointers.fromJson(Object? v) => Pointers._(
      v is String ? List.unmodifiable([v]) : readList(v, readString),
      v is String);
  @override
  Object toJson() => single ? values.first : values;
}

final class Labels extends Carrier {
  final List<String>? names;
  final Map<String, Authored>? descriptions;
  const Labels._(this.names, this.descriptions);
  factory Labels.fromJson(Object? v) => v is List
      ? Labels._(readList(v, readString), null)
      : Labels._(null, readMap(v, Authored.fromJson));
  @override
  Object toJson() => names ?? project(descriptions)!;
}

sealed class ActionValue extends Carrier {
  const ActionValue();
  factory ActionValue.fromJson(Object? v) {
    if (v == null) return const Unresolved();
    if (v is bool) return BooleanValue(v);
    if (v is String) return LabelValue(v);
    if (v is num) return NumberValue(readNumber(v));
    if (v is List) return LabelValues(readList(v, readString));
    return FailedValue(FailedField.fromJson(v).failed);
  }
}

final class Unresolved extends ActionValue {
  const Unresolved();
  @override
  Object? toJson() => null;
}

final class BooleanValue extends ActionValue {
  final bool value;
  const BooleanValue(this.value);
  @override
  bool toJson() => value;
}

final class LabelValue extends ActionValue {
  final String value;
  const LabelValue(this.value);
  @override
  String toJson() => value;
}

final class NumberValue extends ActionValue {
  final double value;
  const NumberValue(this.value);
  @override
  double toJson() => value;
}

final class LabelValues extends ActionValue {
  final List<String> value;
  const LabelValues(this.value);
  @override
  List<String> toJson() => value;
}

final class FailedValue extends ActionValue {
  final Failure failed;
  const FailedValue(this.failed);
  @override
  Map<String, Object?> toJson() => {'failed': failed.toJson()};
}

/// Caller-authored successful content, distinct from a failed member marker.
final class AuthoredValue extends ActionValue {
  final JsonContent meaning;
  const AuthoredValue(this.meaning);
  @override
  Object? toJson() => meaning.toJson();
}

/// Arbitrary original/author JSON, including Boolean and null.
final class JsonContent extends Carrier {
  final Object? value;
  const JsonContent._(this.value);
  factory JsonContent.fromJson(Object? value) => JsonContent._(readJson(value));
  @override
  Object? toJson() => value;
}
