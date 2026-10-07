part of 'models.dart';

final class Failure extends Carrier {
  final String kind;
  final CauseKind cause;
  const Failure({
    required this.kind,
    required this.cause,
  });
  factory Failure.fromJson(Object? value) {
    final v = readObject(value);
    verify("Failure", v, ["kind", "cause"], ["kind", "cause"]);
    return Failure(
      kind: readLiteral(v["kind"], "backend"),
      cause: readEnum(v["cause"], CauseKind.values),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "kind": project(kind),
        "cause": project(cause),
      };
}

final class FailedField extends Carrier {
  final Failure failed;
  const FailedField({
    required this.failed,
  });
  factory FailedField.fromJson(Object? value) {
    final v = readObject(value);
    verify("FailedField", v, ["failed"], ["failed"]);
    return FailedField(
      failed: Failure.fromJson(v["failed"]),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "failed": project(failed),
      };
}

final class AnnotationSuccess extends Carrier implements AnnotationEntry {
  final AnswerId answer_id;
  final ActionValue value;
  final AtomicQuestion question;
  final AtomicAnswer answer;
  final Threshold threshold;
  final Digest request;
  const AnnotationSuccess({
    required this.answer_id,
    required this.value,
    required this.question,
    required this.answer,
    required this.threshold,
    required this.request,
  });
  factory AnnotationSuccess.fromJson(Object? value) {
    final v = readObject(value);
    verify(
        "AnnotationSuccess",
        v,
        ["answer_id", "value", "question", "answer", "threshold", "request"],
        ["answer_id", "value", "question", "answer", "threshold", "request"]);
    return AnnotationSuccess(
      answer_id: AnswerId(readString(v["answer_id"])),
      value: readAction(v["value"], v["question"]),
      question: readVariant<AtomicQuestion>(v["question"], [
        (v) => DecideQuestion.fromJson(v),
        (v) => ChooseQuestion.fromJson(v),
        (v) => TagQuestion.fromJson(v),
        (v) => ScoreQuestion.fromJson(v),
        (v) => FindQuestion.fromJson(v)
      ]),
      answer: readVariant<AtomicAnswer>(v["answer"], [
        (v) => YesNo.fromJson(v),
        (v) => Choice.fromJson(v),
        (v) => Tags.fromJson(v),
        (v) => Score.fromJson(v),
        (v) => FindAnswer.fromJson(v)
      ]),
      threshold: Threshold.fromJson(v["threshold"]),
      request: Digest(readString(v["request"])),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "answer_id": project(answer_id),
        "value": project(value),
        "question": project(question),
        "answer": project(answer),
        "threshold": project(threshold),
        "request": project(request),
      };
}

final class AnnotationFailure extends Carrier implements AnnotationEntry {
  final FailureId failure_id;
  final AtomicQuestion question;
  final Failure failure;
  final Digest request;
  const AnnotationFailure({
    required this.failure_id,
    required this.question,
    required this.failure,
    required this.request,
  });
  factory AnnotationFailure.fromJson(Object? value) {
    final v = readObject(value);
    verify(
        "AnnotationFailure",
        v,
        ["failure_id", "question", "failure", "request"],
        ["failure_id", "question", "failure", "request"]);
    return AnnotationFailure(
      failure_id: FailureId(readString(v["failure_id"])),
      question: readVariant<AtomicQuestion>(v["question"], [
        (v) => DecideQuestion.fromJson(v),
        (v) => ChooseQuestion.fromJson(v),
        (v) => TagQuestion.fromJson(v),
        (v) => ScoreQuestion.fromJson(v),
        (v) => FindQuestion.fromJson(v)
      ]),
      failure: Failure.fromJson(v["failure"]),
      request: Digest(readString(v["request"])),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "failure_id": project(failure_id),
        "question": project(question),
        "failure": project(failure),
        "request": project(request),
      };
}

final class RelationSuccess extends Carrier implements RelationEntry {
  final String relation;
  final String reads;
  final MethodKind method;
  final DirectionKind direction;
  final Endpoint source;
  final Endpoint? target;
  final Digest request;
  final AnswerId answer_id;
  final double probability;
  final bool accepted;
  const RelationSuccess({
    required this.relation,
    required this.reads,
    required this.method,
    required this.direction,
    required this.source,
    required this.target,
    required this.request,
    required this.answer_id,
    required this.probability,
    required this.accepted,
  });
  factory RelationSuccess.fromJson(Object? value) {
    final v = readObject(value);
    verify("RelationSuccess", v, [
      "relation",
      "reads",
      "method",
      "direction",
      "source",
      "target",
      "request",
      "answer_id",
      "probability",
      "accepted"
    ], [
      "relation",
      "reads",
      "method",
      "direction",
      "source",
      "target",
      "request",
      "answer_id",
      "probability",
      "accepted"
    ]);
    return RelationSuccess(
      relation: readString(v["relation"]),
      reads: readString(v["reads"]),
      method: readEnum(v["method"], MethodKind.values),
      direction: readEnum(v["direction"], DirectionKind.values),
      source: Endpoint.fromJson(v["source"]),
      target: (v["target"] == null ? null : Endpoint.fromJson(v["target"])),
      request: Digest(readString(v["request"])),
      answer_id: AnswerId(readString(v["answer_id"])),
      probability: readNumber(v["probability"], probability: true),
      accepted: readBool(v["accepted"]),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "relation": project(relation),
        "reads": project(reads),
        "method": project(method),
        "direction": project(direction),
        "source": project(source),
        "target": project(target),
        "request": project(request),
        "answer_id": project(answer_id),
        "probability": project(probability),
        "accepted": project(accepted),
      };
}

final class RelationFailure extends Carrier implements RelationEntry {
  final String relation;
  final String reads;
  final MethodKind method;
  final DirectionKind direction;
  final Endpoint source;
  final Endpoint? target;
  final Digest request;
  final FailureId failure_id;
  final Failure failure;
  const RelationFailure({
    required this.relation,
    required this.reads,
    required this.method,
    required this.direction,
    required this.source,
    required this.target,
    required this.request,
    required this.failure_id,
    required this.failure,
  });
  factory RelationFailure.fromJson(Object? value) {
    final v = readObject(value);
    verify("RelationFailure", v, [
      "relation",
      "reads",
      "method",
      "direction",
      "source",
      "target",
      "request",
      "failure_id",
      "failure"
    ], [
      "relation",
      "reads",
      "method",
      "direction",
      "source",
      "target",
      "request",
      "failure_id",
      "failure"
    ]);
    return RelationFailure(
      relation: readString(v["relation"]),
      reads: readString(v["reads"]),
      method: readEnum(v["method"], MethodKind.values),
      direction: readEnum(v["direction"], DirectionKind.values),
      source: Endpoint.fromJson(v["source"]),
      target: (v["target"] == null ? null : Endpoint.fromJson(v["target"])),
      request: Digest(readString(v["request"])),
      failure_id: FailureId(readString(v["failure_id"])),
      failure: Failure.fromJson(v["failure"]),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "relation": project(relation),
        "reads": project(reads),
        "method": project(method),
        "direction": project(direction),
        "source": project(source),
        "target": project(target),
        "request": project(request),
        "failure_id": project(failure_id),
        "failure": project(failure),
      };
}

final class RelationAnswer extends Carrier {
  final List<RelationEntry> questions;
  const RelationAnswer({
    required this.questions,
  });
  factory RelationAnswer.fromJson(Object? value) {
    final v = readObject(value);
    verify("RelationAnswer", v, ["questions"], ["questions"]);
    return RelationAnswer(
      questions: readList(
          v["questions"],
          (v) => readVariant<RelationEntry>(v, [
                (v) => RelationSuccess.fromJson(v),
                (v) => RelationFailure.fromJson(v)
              ])),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "questions": project(questions),
      };
}
