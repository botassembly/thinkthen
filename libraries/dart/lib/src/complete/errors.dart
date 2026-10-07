part of 'models.dart';

final class CallError extends Carrier {
  final FailureKind kind;
  final String message;
  final bool retryable;
  final Optional<Facts> facts;
  final Optional<List<Attempt>> attempts;
  final Optional<Stopped> stopped;
  const CallError({
    required this.kind,
    required this.message,
    required this.retryable,
    this.facts = const Optional.absent(),
    this.attempts = const Optional.absent(),
    this.stopped = const Optional.absent(),
  });
  factory CallError.fromJson(Object? value) {
    final v = readObject(value);
    verify(
        "CallError",
        v,
        ["kind", "message", "retryable", "facts", "attempts", "stopped"],
        ["kind", "message", "retryable"]);
    return CallError(
      kind: readEnum(v["kind"], FailureKind.values),
      message: readString(v["message"]),
      retryable: readBool(v["retryable"]),
      facts: v.containsKey("facts")
          ? Optional.present(Facts.fromJson(v["facts"]))
          : const Optional.absent(),
      attempts: v.containsKey("attempts")
          ? Optional.present(
              readList(v["attempts"], (v) => Attempt.fromJson(v)))
          : const Optional.absent(),
      stopped: v.containsKey("stopped")
          ? Optional.present(Stopped.fromJson(v["stopped"]))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "kind": project(kind),
        "message": project(message),
        "retryable": project(retryable),
        if (facts.present) "facts": project(facts.value),
        if (attempts.present) "attempts": project(attempts.value),
        if (stopped.present) "stopped": project(stopped.value),
      };
}

final class Stopped extends Carrier {
  final StopCauseKind cause;
  final bool retryable;
  final Optional<int> at;
  final Optional<int> status;
  const Stopped({
    required this.cause,
    required this.retryable,
    this.at = const Optional.absent(),
    this.status = const Optional.absent(),
  });
  factory Stopped.fromJson(Object? value) {
    final v = readObject(value);
    verify("Stopped", v, ["cause", "retryable", "at", "status"],
        ["cause", "retryable"]);
    return Stopped(
      cause: readEnum(v["cause"], StopCauseKind.values),
      retryable: readBool(v["retryable"]),
      at: v.containsKey("at")
          ? Optional.present(readInt(v["at"], 1, maxInteger))
          : const Optional.absent(),
      status: v.containsKey("status")
          ? Optional.present(readInt(v["status"], 100, 599))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "cause": project(cause),
        "retryable": project(retryable),
        if (at.present) "at": project(at.value),
        if (status.present) "status": project(status.value),
      };
}
