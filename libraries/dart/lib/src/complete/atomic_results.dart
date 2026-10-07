part of 'models.dart';

final class DecideResult extends Carrier implements Result {
  final String schema;
  final AnswerId answer_id;
  final Meta meta;
  final DecisionValue value;
  final DecideQuestion question;
  final YesNo answer;
  final Threshold threshold;
  final Optional<Object?> input;
  final Optional<Position> position;
  final Optional<String> input_file;
  const DecideResult({
    required this.schema,
    required this.answer_id,
    required this.meta,
    required this.value,
    required this.question,
    required this.answer,
    required this.threshold,
    this.input = const Optional.absent(),
    this.position = const Optional.absent(),
    this.input_file = const Optional.absent(),
  });
  factory DecideResult.fromJson(Object? value) {
    final v = readObject(value);
    verify("DecideResult", v, [
      "schema",
      "answer_id",
      "meta",
      "value",
      "question",
      "answer",
      "threshold",
      "input",
      "position",
      "input_file"
    ], [
      "schema",
      "answer_id",
      "meta",
      "value",
      "question",
      "answer",
      "threshold"
    ]);
    return DecideResult(
      schema: readLiteral(v["schema"], "thinkthen.result/2"),
      answer_id: AnswerId(readString(v["answer_id"])),
      meta: Meta.fromJson(v["meta"]),
      value: readDecision(v["value"], v["question"]),
      question: DecideQuestion.fromJson(v["question"]),
      answer: YesNo.fromJson(v["answer"]),
      threshold: Threshold.fromJson(v["threshold"]),
      input: v.containsKey("input")
          ? Optional.present(readJson(v["input"]))
          : const Optional.absent(),
      position: v.containsKey("position")
          ? Optional.present(Position.fromJson(v["position"]))
          : const Optional.absent(),
      input_file: v.containsKey("input_file")
          ? Optional.present(readString(v["input_file"]))
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
        if (input.present) "input": project(input.value),
        if (position.present) "position": project(position.value),
        if (input_file.present) "input_file": project(input_file.value),
      };
}

final class ChooseResult extends Carrier implements Result {
  final String schema;
  final AnswerId answer_id;
  final Meta meta;
  final String? value;
  final ChooseQuestion question;
  final Choice answer;
  final Threshold threshold;
  final Optional<Object?> input;
  final Optional<Position> position;
  final Optional<String> input_file;
  const ChooseResult({
    required this.schema,
    required this.answer_id,
    required this.meta,
    required this.value,
    required this.question,
    required this.answer,
    required this.threshold,
    this.input = const Optional.absent(),
    this.position = const Optional.absent(),
    this.input_file = const Optional.absent(),
  });
  factory ChooseResult.fromJson(Object? value) {
    final v = readObject(value);
    verify("ChooseResult", v, [
      "schema",
      "answer_id",
      "meta",
      "value",
      "question",
      "answer",
      "threshold",
      "input",
      "position",
      "input_file"
    ], [
      "schema",
      "answer_id",
      "meta",
      "value",
      "question",
      "answer",
      "threshold"
    ]);
    return ChooseResult(
      schema: readLiteral(v["schema"], "thinkthen.result/2"),
      answer_id: AnswerId(readString(v["answer_id"])),
      meta: Meta.fromJson(v["meta"]),
      value: (v["value"] == null ? null : readString(v["value"])),
      question: ChooseQuestion.fromJson(v["question"]),
      answer: Choice.fromJson(v["answer"]),
      threshold: Threshold.fromJson(v["threshold"]),
      input: v.containsKey("input")
          ? Optional.present(readJson(v["input"]))
          : const Optional.absent(),
      position: v.containsKey("position")
          ? Optional.present(Position.fromJson(v["position"]))
          : const Optional.absent(),
      input_file: v.containsKey("input_file")
          ? Optional.present(readString(v["input_file"]))
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
        if (input.present) "input": project(input.value),
        if (position.present) "position": project(position.value),
        if (input_file.present) "input_file": project(input_file.value),
      };
}

final class TagResult extends Carrier implements Result {
  final String schema;
  final AnswerId answer_id;
  final Meta meta;
  final List<String> value;
  final TagQuestion question;
  final Tags answer;
  final Threshold threshold;
  final Optional<Object?> input;
  final Optional<Position> position;
  final Optional<String> input_file;
  const TagResult({
    required this.schema,
    required this.answer_id,
    required this.meta,
    required this.value,
    required this.question,
    required this.answer,
    required this.threshold,
    this.input = const Optional.absent(),
    this.position = const Optional.absent(),
    this.input_file = const Optional.absent(),
  });
  factory TagResult.fromJson(Object? value) {
    final v = readObject(value);
    verify("TagResult", v, [
      "schema",
      "answer_id",
      "meta",
      "value",
      "question",
      "answer",
      "threshold",
      "input",
      "position",
      "input_file"
    ], [
      "schema",
      "answer_id",
      "meta",
      "value",
      "question",
      "answer",
      "threshold"
    ]);
    return TagResult(
      schema: readLiteral(v["schema"], "thinkthen.result/2"),
      answer_id: AnswerId(readString(v["answer_id"])),
      meta: Meta.fromJson(v["meta"]),
      value: readList(v["value"], (v) => readString(v)),
      question: TagQuestion.fromJson(v["question"]),
      answer: Tags.fromJson(v["answer"]),
      threshold: Threshold.fromJson(v["threshold"]),
      input: v.containsKey("input")
          ? Optional.present(readJson(v["input"]))
          : const Optional.absent(),
      position: v.containsKey("position")
          ? Optional.present(Position.fromJson(v["position"]))
          : const Optional.absent(),
      input_file: v.containsKey("input_file")
          ? Optional.present(readString(v["input_file"]))
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
        if (input.present) "input": project(input.value),
        if (position.present) "position": project(position.value),
        if (input_file.present) "input_file": project(input_file.value),
      };
}

final class ScoreResult extends Carrier implements Result {
  final String schema;
  final AnswerId answer_id;
  final Meta meta;
  final double value;
  final ScoreQuestion question;
  final Score answer;
  final Null threshold;
  final Optional<Object?> input;
  final Optional<Position> position;
  final Optional<String> input_file;
  const ScoreResult({
    required this.schema,
    required this.answer_id,
    required this.meta,
    required this.value,
    required this.question,
    required this.answer,
    required this.threshold,
    this.input = const Optional.absent(),
    this.position = const Optional.absent(),
    this.input_file = const Optional.absent(),
  });
  factory ScoreResult.fromJson(Object? value) {
    final v = readObject(value);
    verify("ScoreResult", v, [
      "schema",
      "answer_id",
      "meta",
      "value",
      "question",
      "answer",
      "threshold",
      "input",
      "position",
      "input_file"
    ], [
      "schema",
      "answer_id",
      "meta",
      "value",
      "question",
      "answer",
      "threshold"
    ]);
    return ScoreResult(
      schema: readLiteral(v["schema"], "thinkthen.result/2"),
      answer_id: AnswerId(readString(v["answer_id"])),
      meta: Meta.fromJson(v["meta"]),
      value: readNumber(v["value"], probability: false),
      question: ScoreQuestion.fromJson(v["question"]),
      answer: Score.fromJson(v["answer"]),
      threshold: readNull(v["threshold"]),
      input: v.containsKey("input")
          ? Optional.present(readJson(v["input"]))
          : const Optional.absent(),
      position: v.containsKey("position")
          ? Optional.present(Position.fromJson(v["position"]))
          : const Optional.absent(),
      input_file: v.containsKey("input_file")
          ? Optional.present(readString(v["input_file"]))
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
        if (input.present) "input": project(input.value),
        if (position.present) "position": project(position.value),
        if (input_file.present) "input_file": project(input_file.value),
      };
}

final class FilterResult extends Carrier implements Result {
  final String schema;
  final AnswerId answer_id;
  final Meta meta;
  final bool value;
  final Object? input;
  final DecideQuestion question;
  final YesNo answer;
  final Threshold threshold;
  final Optional<Position> position;
  const FilterResult({
    required this.schema,
    required this.answer_id,
    required this.meta,
    required this.value,
    required this.input,
    required this.question,
    required this.answer,
    required this.threshold,
    this.position = const Optional.absent(),
  });
  factory FilterResult.fromJson(Object? value) {
    final v = readObject(value);
    verify("FilterResult", v, [
      "schema",
      "answer_id",
      "meta",
      "value",
      "input",
      "question",
      "answer",
      "threshold",
      "position"
    ], [
      "schema",
      "answer_id",
      "meta",
      "value",
      "input",
      "question",
      "answer",
      "threshold"
    ]);
    return FilterResult(
      schema: readLiteral(v["schema"], "thinkthen.result/2"),
      answer_id: AnswerId(readString(v["answer_id"])),
      meta: Meta.fromJson(v["meta"]),
      value: readBool(v["value"]),
      input: readJson(v["input"]),
      question: DecideQuestion.fromJson(v["question"]),
      answer: YesNo.fromJson(v["answer"]),
      threshold: Threshold.fromJson(v["threshold"]),
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
        "input": project(input),
        "question": project(question),
        "answer": project(answer),
        "threshold": project(threshold),
        if (position.present) "position": project(position.value),
      };
}

final class RankResult extends Carrier implements Result {
  final String schema;
  final AnswerId answer_id;
  final Meta meta;
  final int value;
  final Object? input;
  final AtomicQuestion question;
  final AtomicAnswer answer;
  final Null threshold;
  final Optional<String> question_name;
  final Optional<Position> position;
  const RankResult({
    required this.schema,
    required this.answer_id,
    required this.meta,
    required this.value,
    required this.input,
    required this.question,
    required this.answer,
    required this.threshold,
    this.question_name = const Optional.absent(),
    this.position = const Optional.absent(),
  });
  factory RankResult.fromJson(Object? value) {
    final v = readObject(value);
    verify("RankResult", v, [
      "schema",
      "answer_id",
      "meta",
      "value",
      "input",
      "question",
      "answer",
      "threshold",
      "question_name",
      "position"
    ], [
      "schema",
      "answer_id",
      "meta",
      "value",
      "input",
      "question",
      "answer",
      "threshold"
    ]);
    return RankResult(
      schema: readLiteral(v["schema"], "thinkthen.result/2"),
      answer_id: AnswerId(readString(v["answer_id"])),
      meta: Meta.fromJson(v["meta"]),
      value: readInt(v["value"], 1, maxInteger),
      input: readJson(v["input"]),
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
      threshold: readNull(v["threshold"]),
      question_name: v.containsKey("question_name")
          ? Optional.present(readString(v["question_name"]))
          : const Optional.absent(),
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
        "input": project(input),
        "question": project(question),
        "answer": project(answer),
        "threshold": project(threshold),
        if (question_name.present)
          "question_name": project(question_name.value),
        if (position.present) "position": project(position.value),
      };
}
