import 'models.dart';
import 'values.dart';
import 'read.dart';

/// Private envelope reader; callers choose the actual single or bulk result type.
final class CompleteCall<T> extends Carrier {
  final T value;
  final Facts facts;
  final Optional<List<Attempt>> attempts;
  const CompleteCall(this.value, this.facts, this.attempts);
  factory CompleteCall.fromJson(Object? json, T Function(Object?) readValue) {
    final v = readObject(json);
    if (!v.containsKey('value') || !v.containsKey('facts')) invalid();
    return CompleteCall(
        readValue(v['value']),
        Facts.fromJson(v['facts']),
        v.containsKey('attempts')
            ? Optional.present(readList(v['attempts'], Attempt.fromJson))
            : const Optional.absent());
  }
  @override
  Map<String, Object?> toJson() => {
        'value': project(value),
        'facts': facts.toJson(),
        if (attempts.present) 'attempts': project(attempts.value)
      };
}

/// Copied image view from reviewed C ABI. Native alone determines media/dimensions.
final class ImageView extends Carrier {
  final MediaType media;
  final List<int> bytes;
  final int width, height;
  final Optional<String> filename;
  ImageView(this.media, List<int> bytes, int width, int height, this.filename)
      : bytes = readBytes(bytes),
        width = readInt(width, 1, 4294967295),
        height = readInt(height, 1, 4294967295);
  @override
  Object? toJson() =>
      throw StateError('image views require the native complete adapter');
}
