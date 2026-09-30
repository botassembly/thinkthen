// C outcome codes stay inside the binding. A not-sure answer is not a failure.
enum Outcome { no, yes, notSure }

/// The six named failures. `code` is the C error code, 1 to 6.
enum ErrorKind {
  usage,
  backend,
  deadline,
  local,
  cancelled,
  defect;

  int get code => index + 1;
}

/// One member of an annotate row's value or answers (ADR 0112 section 4).
sealed class AnnotatedField {
  const AnnotatedField();
}

/// JSON null: the question was not sure.
final class UnresolvedField extends AnnotatedField {
  const UnresolvedField();
}

final class AnswerField extends AnnotatedField {
  final Object value;
  const AnswerField(this.value);
}

/// The one-member object `{"failed": {...}}`.
final class FailedField extends AnnotatedField {
  final String kind, cause;
  const FailedField(this.kind, this.cause);
}

/// Reads one member as `jsonDecode` gives it. No answered value is an
/// object, so an object that is not a failure is a [FormatException].
AnnotatedField readField(Object? member) {
  if (member == null) return const UnresolvedField();
  if (member is! Map) return AnswerField(member);
  final failed = member['failed'];
  if (failed is! Map) {
    throw FormatException('annotate member is an object but not a failure');
  }
  final kind = failed['kind'], cause = failed['cause'];
  return FailedField(kind is String ? kind : '', cause is String ? cause : '');
}
