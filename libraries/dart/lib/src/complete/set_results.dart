part of 'models.dart';

final class FindResult extends Carrier implements Result {
  final String schema;
  final AnswerId answer_id;
  final Meta meta;
  final Object? value;
  final FindQuestion question;
  final FindAnswer answer;
  final Null threshold;
  final Optional<Position> position;
  const FindResult({
    required this.schema,
    required this.answer_id,
    required this.meta,
    required this.value,
    required this.question,
    required this.answer,
    required this.threshold,
    this.position = const Optional.absent(),
  });
  factory FindResult.fromJson(Object? value) {
    final v = readObject(value);
    verify("FindResult", v, [
      "schema",
      "answer_id",
      "meta",
      "value",
      "question",
      "answer",
      "threshold",
      "position"
    ], [
      "schema",
      "answer_id",
      "meta",
      "value",
      "question",
      "answer",
      "threshold"
    ]);
    return FindResult(
      schema: readLiteral(v["schema"], "thinkthen.result/2"),
      answer_id: AnswerId(readString(v["answer_id"])),
      meta: Meta.fromJson(v["meta"]),
      value: readJson(v["value"]),
      question: FindQuestion.fromJson(v["question"]),
      answer: FindAnswer.fromJson(v["answer"]),
      threshold: readNull(v["threshold"]),
      position: v.containsKey("position")
          ? Optional.present(Position.fromJson(v["position"]))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "schema": project(schema),
        "answer_id": project(answer_id),
        "meta": project(meta),
        "value": project(value),
        "question": project(question),
        "answer": project(answer),
        "threshold": project(threshold),
        if (position.present) "position": project(position.value),
      };
}

final class AnnotateResult extends Carrier implements Result {
  final String schema;
  final AnswerId answer_id;
  final Meta meta;
  final Object? input;
  final Map<String, ActionValue> value;
  final Map<String, AnnotationEntry> answers;
  final Optional<Position> position;
  const AnnotateResult({
    required this.schema,
    required this.answer_id,
    required this.meta,
    required this.input,
    required this.value,
    required this.answers,
    this.position = const Optional.absent(),
  });
  factory AnnotateResult.fromJson(Object? value) {
    final v = readObject(value);
    verify("AnnotateResult", v, [
      "schema",
      "answer_id",
      "meta",
      "input",
      "value",
      "answers",
      "position"
    ], [
      "schema",
      "answer_id",
      "meta",
      "input",
      "value",
      "answers"
    ]);
    return AnnotateResult(
      schema: readLiteral(v["schema"], "thinkthen.result/2"),
      answer_id: AnswerId(readString(v["answer_id"])),
      meta: Meta.fromJson(v["meta"]),
      input: readJson(v["input"]),
      value: readAnnotationValues(v["value"], v["answers"]),
      answers: readMap(
          v["answers"],
          (v) => readVariant<AnnotationEntry>(v, [
                (v) => AnnotationSuccess.fromJson(v),
                (v) => AnnotationFailure.fromJson(v)
              ])),
      position: v.containsKey("position")
          ? Optional.present(Position.fromJson(v["position"]))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "schema": project(schema),
        "answer_id": project(answer_id),
        "meta": project(meta),
        "input": project(input),
        "value": project(value),
        "answers": project(answers),
        if (position.present) "position": project(position.value),
      };
}

final class RecognizeResult extends Carrier implements Result {
  final String schema;
  final AnswerId answer_id;
  final Meta meta;
  final Recognition value;
  final RecognizeQuestion question;
  final RecognitionAnswer answer;
  final Optional<Object?> input;
  const RecognizeResult({
    required this.schema,
    required this.answer_id,
    required this.meta,
    required this.value,
    required this.question,
    required this.answer,
    this.input = const Optional.absent(),
  });
  factory RecognizeResult.fromJson(Object? value) {
    final v = readObject(value);
    verify(
        "RecognizeResult",
        v,
        ["schema", "answer_id", "meta", "value", "question", "answer", "input"],
        ["schema", "answer_id", "meta", "value", "question", "answer"]);
    return RecognizeResult(
      schema: readLiteral(v["schema"], "thinkthen.result/2"),
      answer_id: AnswerId(readString(v["answer_id"])),
      meta: Meta.fromJson(v["meta"]),
      value: Recognition.fromJson(v["value"]),
      question: RecognizeQuestion.fromJson(v["question"]),
      answer: RecognitionAnswer.fromJson(v["answer"]),
      input: v.containsKey("input")
          ? Optional.present(readJson(v["input"]))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "schema": project(schema),
        "answer_id": project(answer_id),
        "meta": project(meta),
        "value": project(value),
        "question": project(question),
        "answer": project(answer),
        if (input.present) "input": project(input.value),
      };
}

final class RelateResult extends Carrier implements Result {
  final String schema;
  final AnswerId answer_id;
  final Meta meta;
  final List<Edge> value;
  final RelateQuestion question;
  final RelationAnswer answer;
  const RelateResult({
    required this.schema,
    required this.answer_id,
    required this.meta,
    required this.value,
    required this.question,
    required this.answer,
  });
  factory RelateResult.fromJson(Object? value) {
    final v = readObject(value);
    verify(
        "RelateResult",
        v,
        ["schema", "answer_id", "meta", "value", "question", "answer"],
        ["schema", "answer_id", "meta", "value", "question", "answer"]);
    return RelateResult(
      schema: readLiteral(v["schema"], "thinkthen.result/2"),
      answer_id: AnswerId(readString(v["answer_id"])),
      meta: Meta.fromJson(v["meta"]),
      value: readList(v["value"], (v) => Edge.fromJson(v)),
      question: RelateQuestion.fromJson(v["question"]),
      answer: RelationAnswer.fromJson(v["answer"]),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "schema": project(schema),
        "answer_id": project(answer_id),
        "meta": project(meta),
        "value": project(value),
        "question": project(question),
        "answer": project(answer),
      };
}
