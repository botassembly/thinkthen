// Generated from the shared Rust graph. Do not edit.
part of thinkthen_session;

final class Annotation extends _NativeValue {
  Annotation._(super.fields);
  factory Annotation.read(Object? value) => Annotation._(_readObject(value));
  String get answerId => _fields["answer_id"] as String;
  Map<String, AnnotationMember> get answers =>
      (_fields["answers"] as Map<String, Object?>)
          .map((k, v) => MapEntry(k, AnnotationMember.read(v)));
  Presence<String> get file => _fields.containsKey("file")
      ? Presence.present(_fields["file"] as String)
      : const Presence.absent();
  Presence<BigInt> get firstLine => _fields.containsKey("first_line")
      ? Presence.present(_readInteger(_fields["first_line"]))
      : const Presence.absent();
  Presence<BigInt> get index => _fields.containsKey("index")
      ? Presence.present(_readInteger(_fields["index"]))
      : const Presence.absent();
  Object? get input => _fields["input"];
  Presence<BigInt> get lastLine => _fields.containsKey("last_line")
      ? Presence.present(_readInteger(_fields["last_line"]))
      : const Presence.absent();
  Meta get meta => Meta.read(_fields["meta"]);
  Presence<Position> get position => _fields.containsKey("position")
      ? Presence.present(Position.read(_fields["position"]))
      : const Presence.absent();
  String get schema => _fields["schema"] as String;
  Presence<PhysicalSource> get source => _fields.containsKey("source")
      ? Presence.present(PhysicalSource.read(_fields["source"]))
      : const Presence.absent();
  Map<String, AnnotatedField> get value =>
      (_fields["value"] as Map<String, Object?>)
          .map((k, v) => MapEntry(k, AnnotatedField.read(v)));
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "answer_id",
            "answers",
            "file",
            "first_line",
            "index",
            "input",
            "last_line",
            "meta",
            "position",
            "schema",
            "source",
            "value"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

sealed class AnnotationMember extends _NativeValue {
  AnnotationMember(super.fields);
  factory AnnotationMember.read(Object? value) {
    final map = _readObject(value);
    if (map.containsKey("answer_id")) return AnnotationMemberAnswerId.read(map);
    if (map.containsKey("failure_id"))
      return AnnotationMemberFailureId.read(map);
    throw FormatException("Unknown AnnotationMember alternative");
  }
}

final class AnnotationMemberAnswerId extends AnnotationMember {
  AnnotationMemberAnswerId._(super.fields);
  factory AnnotationMemberAnswerId.read(Object? value) =>
      AnnotationMemberAnswerId._(_readObject(value));
  Answer get answer => Answer.read(_fields["answer"]);
  String get answerId => _fields["answer_id"] as String;
  List<Observation> get observations => (_fields["observations"] as List)
      .map((v) => Observation.read(v))
      .toList(growable: false);
  ReadableQuestion get question => ReadableQuestion.read(_fields["question"]);
  List<QuestionSource> get questionSources =>
      (_fields["question_sources"] as List)
          .map((v) => QuestionSource.read(v))
          .toList(growable: false);
  String get request => _fields["request"] as String;
  Object? get threshold =>
      _fields["threshold"] == null ? null : _fields["threshold"];
  Presence<Usage> get usage => _fields.containsKey("usage")
      ? Presence.present(Usage.read(_fields["usage"]))
      : const Presence.absent();
  Value get value => Value.read(_fields["value"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "answer",
            "answer_id",
            "observations",
            "question",
            "question_sources",
            "request",
            "threshold",
            "usage",
            "value"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class AnnotationMemberFailureId extends AnnotationMember {
  AnnotationMemberFailureId._(super.fields);
  factory AnnotationMemberFailureId.read(Object? value) =>
      AnnotationMemberFailureId._(_readObject(value));
  Failure get failure => Failure.read(_fields["failure"]);
  String get failureId => _fields["failure_id"] as String;
  List<Observation> get observations => (_fields["observations"] as List)
      .map((v) => Observation.read(v))
      .toList(growable: false);
  ReadableQuestion get question => ReadableQuestion.read(_fields["question"]);
  List<QuestionSource> get questionSources =>
      (_fields["question_sources"] as List)
          .map((v) => QuestionSource.read(v))
          .toList(growable: false);
  String get request => _fields["request"] as String;
  Object? get threshold =>
      _fields["threshold"] == null ? null : _fields["threshold"];
  Presence<Usage> get usage => _fields.containsKey("usage")
      ? Presence.present(Usage.read(_fields["usage"]))
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "failure",
            "failure_id",
            "observations",
            "question",
            "question_sources",
            "request",
            "threshold",
            "usage"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

sealed class AnnotationValue extends _NativeValue {
  AnnotationValue(super.fields);
  factory AnnotationValue.read(Object? value) {
    final map = _readObject(value);
    if (map["kind"] == "decision") return AnnotationValueDecision.read(map);
    if (map["kind"] == "choice") return AnnotationValueChoice.read(map);
    if (map["kind"] == "score") return AnnotationValueScore.read(map);
    if (map["kind"] == "tags") return AnnotationValueTags.read(map);
    if (map["kind"] == "failed") return AnnotationValueFailed.read(map);
    throw FormatException("Unknown AnnotationValue alternative");
  }
}

final class AnnotationValueChoice extends AnnotationValue {
  AnnotationValueChoice._(super.fields);
  factory AnnotationValueChoice.read(Object? value) =>
      AnnotationValueChoice._(_readObject(value));
  String get kind => _fields["kind"] as String;
  String? get value =>
      _fields["value"] == null ? null : _fields["value"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "value"].contains(entry.key)) entry.key: entry.value
      });
}

final class AnnotationValueDecision extends AnnotationValue {
  AnnotationValueDecision._(super.fields);
  factory AnnotationValueDecision.read(Object? value) =>
      AnnotationValueDecision._(_readObject(value));
  String get kind => _fields["kind"] as String;
  bool? get value => _fields["value"] == null ? null : _fields["value"] as bool;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "value"].contains(entry.key)) entry.key: entry.value
      });
}

final class AnnotationValueFailed extends AnnotationValue {
  AnnotationValueFailed._(super.fields);
  factory AnnotationValueFailed.read(Object? value) =>
      AnnotationValueFailed._(_readObject(value));
  String get kind => _fields["kind"] as String;
  Failure get value => Failure.read(_fields["value"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "value"].contains(entry.key)) entry.key: entry.value
      });
}

final class AnnotationValueScore extends AnnotationValue {
  AnnotationValueScore._(super.fields);
  factory AnnotationValueScore.read(Object? value) =>
      AnnotationValueScore._(_readObject(value));
  String get kind => _fields["kind"] as String;
  num get value => _fields["value"] as num;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "value"].contains(entry.key)) entry.key: entry.value
      });
}

final class AnnotationValueTags extends AnnotationValue {
  AnnotationValueTags._(super.fields);
  factory AnnotationValueTags.read(Object? value) =>
      AnnotationValueTags._(_readObject(value));
  String get kind => _fields["kind"] as String;
  List<String> get value => (_fields["value"] as List)
      .map((v) => v as String)
      .toList(growable: false);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "value"].contains(entry.key)) entry.key: entry.value
      });
}

final class Answers extends _NativeValue {
  Answers._(super.fields);
  factory Answers.read(Object? value) => Answers._(_readObject(value));
  List<RelationMember> get questions => (_fields["questions"] as List)
      .map((v) => RelationMember.read(v))
      .toList(growable: false);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["questions"].contains(entry.key)) entry.key: entry.value
      });
}

final class TagResult extends _NativeValue {
  TagResult._(super.fields);
  factory TagResult.read(Object? value) => TagResult._(_readObject(value));
  Answer get answer => Answer.read(_fields["answer"]);
  String get answerId => _fields["answer_id"] as String;
  Presence<List<Image>> get images => _fields.containsKey("images")
      ? Presence.present((_fields["images"] as List)
          .map((v) => Image.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<BigInt> get index => _fields.containsKey("index")
      ? Presence.present(_readInteger(_fields["index"]))
      : const Presence.absent();
  Presence<Object?> get input => _fields.containsKey("input")
      ? Presence.present(_fields["input"])
      : const Presence.absent();
  Presence<List<RankMember>> get members => _fields.containsKey("members")
      ? Presence.present((_fields["members"] as List)
          .map((v) => RankMember.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Meta get meta => Meta.read(_fields["meta"]);
  ReadableQuestion get question => ReadableQuestion.read(_fields["question"]);
  Presence<String> get questionName => _fields.containsKey("question_name")
      ? Presence.present(_fields["question_name"] as String)
      : const Presence.absent();
  String get schema => _fields["schema"] as String;
  Presence<PhysicalSource> get source => _fields.containsKey("source")
      ? Presence.present(PhysicalSource.read(_fields["source"]))
      : const Presence.absent();
  Object? get threshold =>
      _fields["threshold"] == null ? null : _fields["threshold"];
  List<String> get value => (_fields["value"] as List)
      .map((v) => v as String)
      .toList(growable: false);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "answer",
            "answer_id",
            "images",
            "index",
            "input",
            "members",
            "meta",
            "question",
            "question_name",
            "schema",
            "source",
            "threshold",
            "value"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class DecisionResult extends _NativeValue {
  DecisionResult._(super.fields);
  factory DecisionResult.read(Object? value) =>
      DecisionResult._(_readObject(value));
  Answer get answer => Answer.read(_fields["answer"]);
  String get answerId => _fields["answer_id"] as String;
  Presence<List<Image>> get images => _fields.containsKey("images")
      ? Presence.present((_fields["images"] as List)
          .map((v) => Image.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<BigInt> get index => _fields.containsKey("index")
      ? Presence.present(_readInteger(_fields["index"]))
      : const Presence.absent();
  Presence<Object?> get input => _fields.containsKey("input")
      ? Presence.present(_fields["input"])
      : const Presence.absent();
  Presence<List<RankMember>> get members => _fields.containsKey("members")
      ? Presence.present((_fields["members"] as List)
          .map((v) => RankMember.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Meta get meta => Meta.read(_fields["meta"]);
  ReadableQuestion get question => ReadableQuestion.read(_fields["question"]);
  Presence<String> get questionName => _fields.containsKey("question_name")
      ? Presence.present(_fields["question_name"] as String)
      : const Presence.absent();
  String get schema => _fields["schema"] as String;
  Presence<PhysicalSource> get source => _fields.containsKey("source")
      ? Presence.present(PhysicalSource.read(_fields["source"]))
      : const Presence.absent();
  Object? get threshold =>
      _fields["threshold"] == null ? null : _fields["threshold"];
  DecideValue get value => DecideValue.read(_fields["value"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "answer",
            "answer_id",
            "images",
            "index",
            "input",
            "members",
            "meta",
            "question",
            "question_name",
            "schema",
            "source",
            "threshold",
            "value"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class RankResult extends _NativeValue {
  RankResult._(super.fields);
  factory RankResult.read(Object? value) => RankResult._(_readObject(value));
  Answer get answer => Answer.read(_fields["answer"]);
  String get answerId => _fields["answer_id"] as String;
  Presence<List<Image>> get images => _fields.containsKey("images")
      ? Presence.present((_fields["images"] as List)
          .map((v) => Image.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<BigInt> get index => _fields.containsKey("index")
      ? Presence.present(_readInteger(_fields["index"]))
      : const Presence.absent();
  Presence<Object?> get input => _fields.containsKey("input")
      ? Presence.present(_fields["input"])
      : const Presence.absent();
  Presence<List<RankMember>> get members => _fields.containsKey("members")
      ? Presence.present((_fields["members"] as List)
          .map((v) => RankMember.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Meta get meta => Meta.read(_fields["meta"]);
  ReadableQuestion get question => ReadableQuestion.read(_fields["question"]);
  Presence<String> get questionName => _fields.containsKey("question_name")
      ? Presence.present(_fields["question_name"] as String)
      : const Presence.absent();
  String get schema => _fields["schema"] as String;
  Presence<PhysicalSource> get source => _fields.containsKey("source")
      ? Presence.present(PhysicalSource.read(_fields["source"]))
      : const Presence.absent();
  Object? get threshold =>
      _fields["threshold"] == null ? null : _fields["threshold"];
  BigInt get value => _readInteger(_fields["value"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "answer",
            "answer_id",
            "images",
            "index",
            "input",
            "members",
            "meta",
            "question",
            "question_name",
            "schema",
            "source",
            "threshold",
            "value"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class ChoiceResult extends _NativeValue {
  ChoiceResult._(super.fields);
  factory ChoiceResult.read(Object? value) =>
      ChoiceResult._(_readObject(value));
  Answer get answer => Answer.read(_fields["answer"]);
  String get answerId => _fields["answer_id"] as String;
  Presence<List<Image>> get images => _fields.containsKey("images")
      ? Presence.present((_fields["images"] as List)
          .map((v) => Image.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<BigInt> get index => _fields.containsKey("index")
      ? Presence.present(_readInteger(_fields["index"]))
      : const Presence.absent();
  Presence<Object?> get input => _fields.containsKey("input")
      ? Presence.present(_fields["input"])
      : const Presence.absent();
  Presence<List<RankMember>> get members => _fields.containsKey("members")
      ? Presence.present((_fields["members"] as List)
          .map((v) => RankMember.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Meta get meta => Meta.read(_fields["meta"]);
  ReadableQuestion get question => ReadableQuestion.read(_fields["question"]);
  Presence<String> get questionName => _fields.containsKey("question_name")
      ? Presence.present(_fields["question_name"] as String)
      : const Presence.absent();
  String get schema => _fields["schema"] as String;
  Presence<PhysicalSource> get source => _fields.containsKey("source")
      ? Presence.present(PhysicalSource.read(_fields["source"]))
      : const Presence.absent();
  Object? get threshold =>
      _fields["threshold"] == null ? null : _fields["threshold"];
  String? get value =>
      _fields["value"] == null ? null : _fields["value"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "answer",
            "answer_id",
            "images",
            "index",
            "input",
            "members",
            "meta",
            "question",
            "question_name",
            "schema",
            "source",
            "threshold",
            "value"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class FilterResult extends _NativeValue {
  FilterResult._(super.fields);
  factory FilterResult.read(Object? value) =>
      FilterResult._(_readObject(value));
  Answer get answer => Answer.read(_fields["answer"]);
  String get answerId => _fields["answer_id"] as String;
  Presence<List<Image>> get images => _fields.containsKey("images")
      ? Presence.present((_fields["images"] as List)
          .map((v) => Image.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<BigInt> get index => _fields.containsKey("index")
      ? Presence.present(_readInteger(_fields["index"]))
      : const Presence.absent();
  Presence<Object?> get input => _fields.containsKey("input")
      ? Presence.present(_fields["input"])
      : const Presence.absent();
  Presence<List<RankMember>> get members => _fields.containsKey("members")
      ? Presence.present((_fields["members"] as List)
          .map((v) => RankMember.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Meta get meta => Meta.read(_fields["meta"]);
  ReadableQuestion get question => ReadableQuestion.read(_fields["question"]);
  Presence<String> get questionName => _fields.containsKey("question_name")
      ? Presence.present(_fields["question_name"] as String)
      : const Presence.absent();
  String get schema => _fields["schema"] as String;
  Presence<PhysicalSource> get source => _fields.containsKey("source")
      ? Presence.present(PhysicalSource.read(_fields["source"]))
      : const Presence.absent();
  Object? get threshold =>
      _fields["threshold"] == null ? null : _fields["threshold"];
  bool get value => _fields["value"] as bool;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "answer",
            "answer_id",
            "images",
            "index",
            "input",
            "members",
            "meta",
            "question",
            "question_name",
            "schema",
            "source",
            "threshold",
            "value"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class ScoreResult extends _NativeValue {
  ScoreResult._(super.fields);
  factory ScoreResult.read(Object? value) => ScoreResult._(_readObject(value));
  Answer get answer => Answer.read(_fields["answer"]);
  String get answerId => _fields["answer_id"] as String;
  Presence<List<Image>> get images => _fields.containsKey("images")
      ? Presence.present((_fields["images"] as List)
          .map((v) => Image.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<BigInt> get index => _fields.containsKey("index")
      ? Presence.present(_readInteger(_fields["index"]))
      : const Presence.absent();
  Presence<Object?> get input => _fields.containsKey("input")
      ? Presence.present(_fields["input"])
      : const Presence.absent();
  Presence<List<RankMember>> get members => _fields.containsKey("members")
      ? Presence.present((_fields["members"] as List)
          .map((v) => RankMember.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Meta get meta => Meta.read(_fields["meta"]);
  ReadableQuestion get question => ReadableQuestion.read(_fields["question"]);
  Presence<String> get questionName => _fields.containsKey("question_name")
      ? Presence.present(_fields["question_name"] as String)
      : const Presence.absent();
  String get schema => _fields["schema"] as String;
  Presence<PhysicalSource> get source => _fields.containsKey("source")
      ? Presence.present(PhysicalSource.read(_fields["source"]))
      : const Presence.absent();
  Object? get threshold =>
      _fields["threshold"] == null ? null : _fields["threshold"];
  num get value => _fields["value"] as num;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "answer",
            "answer_id",
            "images",
            "index",
            "input",
            "members",
            "meta",
            "question",
            "question_name",
            "schema",
            "source",
            "threshold",
            "value"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class Attempt extends _NativeValue {
  Attempt._(super.fields);
  factory Attempt.read(Object? value) => Attempt._(_readObject(value));
  BigInt get ordinal => _readInteger(_fields["ordinal"]);
  String get outcome => _fields["outcome"] as String;
  Presence<String> get requestId => _fields.containsKey("request_id")
      ? Presence.present(_fields["request_id"] as String)
      : const Presence.absent();
  String get requestSha256 => _fields["request_sha256"] as String;
  String get sdkRequestId => _fields["sdk_request_id"] as String;
  Presence<BigInt> get serverMs => _fields.containsKey("server_ms")
      ? Presence.present(_readInteger(_fields["server_ms"]))
      : const Presence.absent();
  Presence<BigInt> get status => _fields.containsKey("status")
      ? Presence.present(_readInteger(_fields["status"]))
      : const Presence.absent();
  BigInt get wallMs => _readInteger(_fields["wall_ms"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "ordinal",
            "outcome",
            "request_id",
            "request_sha256",
            "sdk_request_id",
            "server_ms",
            "status",
            "wall_ms"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class Batch implements _EncodedValue {
  final Object? value;
  const Batch._(this.value);
  factory Batch.read(Object? value) => value is Batch ? value : Batch._(value);
  Object? _toValue() => value;
  const Batch.integer(BigInt this.value);
  BigInt get asInteger => _readInteger(value);
  const Batch.text(String this.value);
  String get asText => value as String;
}

final class BoundaryOdds extends _NativeValue {
  BoundaryOdds._(super.fields);
  factory BoundaryOdds.read(Object? value) =>
      BoundaryOdds._(_readObject(value));
  List<PieceOdds> get pieces => (_fields["pieces"] as List)
      .map((v) => PieceOdds.read(v))
      .toList(growable: false);
  List<BoundaryProposal> get proposals => (_fields["proposals"] as List)
      .map((v) => BoundaryProposal.read(v))
      .toList(growable: false);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["pieces", "proposals"].contains(entry.key))
            entry.key: entry.value
      });
}

final class BoundaryProposal extends _NativeValue {
  BoundaryProposal._(super.fields);
  factory BoundaryProposal.read(Object? value) =>
      BoundaryProposal._(_readObject(value));
  BigInt get end => _readInteger(_fields["end"]);
  BigInt get length => _readInteger(_fields["length"]);
  num get probability => _fields["probability"] as num;
  BigInt get start => _readInteger(_fields["start"]);
  String get text => _fields["text"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["end", "length", "probability", "start", "text"]
              .contains(entry.key))
            entry.key: entry.value
      });
}

final class CallError extends _NativeValue {
  CallError._(super.fields);
  factory CallError.read(Object? value) => CallError._(_readObject(value));
  Error get error => Error.read(_fields["error"]);
  Presence<Facts> get facts => _fields.containsKey("facts")
      ? Presence.present(Facts.read(_fields["facts"]))
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["error", "facts"].contains(entry.key)) entry.key: entry.value
      });
}

final class DecideValue implements _EncodedValue {
  final Object? value;
  const DecideValue._(this.value);
  factory DecideValue.read(Object? value) =>
      value is DecideValue ? value : DecideValue._(value);
  Object? _toValue() => value;
  const DecideValue.boolean(bool? this.value);
  bool? get asBoolean => value == null ? null : value as bool;
  const DecideValue.reading(Object? this.value);
  Object? get asReading => value;
}

final class EntityDocument extends _NativeValue {
  EntityDocument._(super.fields);
  factory EntityDocument.read(Object? value) =>
      EntityDocument._(_readObject(value));
  String get kind => _fields["kind"] as String;
  String get name => _fields["name"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "name"].contains(entry.key)) entry.key: entry.value
      });
}

final class Error extends _NativeValue {
  Error._(super.fields);
  factory Error.read(Object? value) => Error._(_readObject(value));
  Presence<EstimatedInputDenial> get estimatedInputDenial =>
      _fields.containsKey("estimated_input_denial")
          ? Presence.present(
              EstimatedInputDenial.read(_fields["estimated_input_denial"]))
          : const Presence.absent();
  String get kind => _fields["kind"] as String;
  String get message => _fields["message"] as String;
  bool get retryable => _fields["retryable"] as bool;
  Presence<SendBudgetDenial> get sendBudgetDenial => _fields
          .containsKey("send_budget_denial")
      ? Presence.present(SendBudgetDenial.read(_fields["send_budget_denial"]))
      : const Presence.absent();
  Stopped get stopped => Stopped.read(_fields["stopped"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "estimated_input_denial",
            "kind",
            "message",
            "retryable",
            "send_budget_denial",
            "stopped"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

sealed class EstimatedInputDenial extends _NativeValue {
  EstimatedInputDenial(super.fields);
  factory EstimatedInputDenial.read(Object? value) {
    final map = _readObject(value);
    if (map["kind"] == "initial_request")
      return EstimatedInputDenialInitialRequest.read(map);
    if (map["kind"] == "additional_request")
      return EstimatedInputDenialAdditionalRequest.read(map);
    if (map["kind"] == "retry") return EstimatedInputDenialRetry.read(map);
    throw FormatException("Unknown EstimatedInputDenial alternative");
  }
}

final class EstimatedInputDenialAdditionalRequest extends EstimatedInputDenial {
  EstimatedInputDenialAdditionalRequest._(super.fields);
  factory EstimatedInputDenialAdditionalRequest.read(Object? value) =>
      EstimatedInputDenialAdditionalRequest._(_readObject(value));
  String get kind => _fields["kind"] as String;
  BigInt get limit => _readInteger(_fields["limit"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "limit"].contains(entry.key)) entry.key: entry.value
      });
}

final class EstimatedInputDenialInitialRequest extends EstimatedInputDenial {
  EstimatedInputDenialInitialRequest._(super.fields);
  factory EstimatedInputDenialInitialRequest.read(Object? value) =>
      EstimatedInputDenialInitialRequest._(_readObject(value));
  String get kind => _fields["kind"] as String;
  BigInt get limit => _readInteger(_fields["limit"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "limit"].contains(entry.key)) entry.key: entry.value
      });
}

final class EstimatedInputDenialRetry extends EstimatedInputDenial {
  EstimatedInputDenialRetry._(super.fields);
  factory EstimatedInputDenialRetry.read(Object? value) =>
      EstimatedInputDenialRetry._(_readObject(value));
  String get kind => _fields["kind"] as String;
  BigInt get lastStatus => _readInteger(_fields["last_status"]);
  BigInt get limit => _readInteger(_fields["limit"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "last_status", "limit"].contains(entry.key))
            entry.key: entry.value
      });
}

final class Facts extends _NativeValue {
  Facts._(super.fields);
  factory Facts.read(Object? value) => Facts._(_readObject(value));
  Presence<List<Attempt>> get attempts => _fields.containsKey("attempts")
      ? Presence.present((_fields["attempts"] as List)
          .map((v) => Attempt.read(v))
          .toList(growable: false))
      : const Presence.absent();
  BigInt get cacheAnswers => _readInteger(_fields["cache_answers"]);
  String get callId => _fields["call_id"] as String;
  Presence<String> get estimatedCostUsd =>
      _fields.containsKey("estimated_cost_usd")
          ? Presence.present(_fields["estimated_cost_usd"] as String)
          : const Presence.absent();
  Presence<bool> get heldModelMismatch =>
      _fields.containsKey("held_model_mismatch")
          ? Presence.present(_fields["held_model_mismatch"] as bool)
          : const Presence.absent();
  Presence<BigInt> get inputTokens => _fields.containsKey("input_tokens")
      ? Presence.present(_readInteger(_fields["input_tokens"]))
      : const Presence.absent();
  BigInt get largestRequestBytes =>
      _readInteger(_fields["largest_request_bytes"]);
  BigInt? get largestRequestEstimatedInputTokens =>
      _fields["largest_request_estimated_input_tokens"] == null
          ? null
          : _readInteger(_fields["largest_request_estimated_input_tokens"]);
  Presence<String> get model => _fields.containsKey("model")
      ? Presence.present(_fields["model"] as String)
      : const Presence.absent();
  Presence<BigInt> get outputTokens => _fields.containsKey("output_tokens")
      ? Presence.present(_readInteger(_fields["output_tokens"]))
      : const Presence.absent();
  BigInt get records => _readInteger(_fields["records"]);
  BigInt get requestsSent => _readInteger(_fields["requests_sent"]);
  num get seconds => _fields["seconds"] as num;
  String get tokenEstimateMethod => _fields["token_estimate_method"] as String;
  Presence<PersistenceObservation> get usagePersistence =>
      _fields.containsKey("usage_persistence")
          ? Presence.present(
              PersistenceObservation.read(_fields["usage_persistence"]))
          : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "attempts",
            "cache_answers",
            "call_id",
            "estimated_cost_usd",
            "held_model_mismatch",
            "input_tokens",
            "largest_request_bytes",
            "largest_request_estimated_input_tokens",
            "model",
            "output_tokens",
            "records",
            "requests_sent",
            "seconds",
            "token_estimate_method",
            "usage_persistence"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class Find extends _NativeValue {
  Find._(super.fields);
  factory Find.read(Object? value) => Find._(_readObject(value));
  FindAnswer get answer => FindAnswer.read(_fields["answer"]);
  String get answerId => _fields["answer_id"] as String;
  Presence<List<FindCandidate>> get candidates =>
      _fields.containsKey("candidates")
          ? Presence.present((_fields["candidates"] as List)
              .map((v) => FindCandidate.read(v))
              .toList(growable: false))
          : const Presence.absent();
  Presence<String> get file => _fields.containsKey("file")
      ? Presence.present(_fields["file"] as String)
      : const Presence.absent();
  Presence<BigInt> get firstLine => _fields.containsKey("first_line")
      ? Presence.present(_readInteger(_fields["first_line"]))
      : const Presence.absent();
  BigInt? get index =>
      _fields["index"] == null ? null : _readInteger(_fields["index"]);
  Presence<BigInt> get lastLine => _fields.containsKey("last_line")
      ? Presence.present(_readInteger(_fields["last_line"]))
      : const Presence.absent();
  Meta get meta => Meta.read(_fields["meta"]);
  Presence<Position> get position => _fields.containsKey("position")
      ? Presence.present(Position.read(_fields["position"]))
      : const Presence.absent();
  FindQuestionInfo get question => FindQuestionInfo.read(_fields["question"]);
  String get schema => _fields["schema"] as String;
  Object? get threshold => _fields["threshold"];
  Object? get value => _fields["value"];
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "answer",
            "answer_id",
            "candidates",
            "file",
            "first_line",
            "index",
            "last_line",
            "meta",
            "position",
            "question",
            "schema",
            "threshold",
            "value"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class FindCandidate extends _NativeValue {
  FindCandidate._(super.fields);
  factory FindCandidate.read(Object? value) =>
      FindCandidate._(_readObject(value));
  BigInt? get index =>
      _fields["index"] == null ? null : _readInteger(_fields["index"]);
  Object? get input => _fields["input"];
  num get probability => _fields["probability"] as num;
  Presence<PhysicalSource> get source => _fields.containsKey("source")
      ? Presence.present(PhysicalSource.read(_fields["source"]))
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["index", "input", "probability", "source"].contains(entry.key))
            entry.key: entry.value
      });
}

final class Image extends _NativeValue {
  Image._(super.fields);
  factory Image.read(Object? value) => Image._(_readObject(value));
  String get base64 => _fields["base64"] as String;
  BigInt get height => _readInteger(_fields["height"]);
  String get media => _fields["media"] as String;
  BigInt get width => _readInteger(_fields["width"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["base64", "height", "media", "width"].contains(entry.key))
            entry.key: entry.value
      });
}

final class FailureFacts extends _NativeValue {
  FailureFacts._(super.fields);
  factory FailureFacts.read(Object? value) =>
      FailureFacts._(_readObject(value));
  Presence<List<Attempt>> get attempts => _fields.containsKey("attempts")
      ? Presence.present((_fields["attempts"] as List)
          .map((v) => Attempt.read(v))
          .toList(growable: false))
      : const Presence.absent();
  BigInt get cacheAnswers => _readInteger(_fields["cache_answers"]);
  Presence<String> get callId => _fields.containsKey("call_id")
      ? Presence.present(_fields["call_id"] as String)
      : const Presence.absent();
  Presence<String> get estimatedCostUsd =>
      _fields.containsKey("estimated_cost_usd")
          ? Presence.present(_fields["estimated_cost_usd"] as String)
          : const Presence.absent();
  Presence<bool> get heldModelMismatch =>
      _fields.containsKey("held_model_mismatch")
          ? Presence.present(_fields["held_model_mismatch"] as bool)
          : const Presence.absent();
  Presence<BigInt> get inputTokens => _fields.containsKey("input_tokens")
      ? Presence.present(_readInteger(_fields["input_tokens"]))
      : const Presence.absent();
  BigInt get largestRequestBytes =>
      _readInteger(_fields["largest_request_bytes"]);
  BigInt? get largestRequestEstimatedInputTokens =>
      _fields["largest_request_estimated_input_tokens"] == null
          ? null
          : _readInteger(_fields["largest_request_estimated_input_tokens"]);
  Presence<String> get model => _fields.containsKey("model")
      ? Presence.present(_fields["model"] as String)
      : const Presence.absent();
  Presence<BigInt> get outputTokens => _fields.containsKey("output_tokens")
      ? Presence.present(_readInteger(_fields["output_tokens"]))
      : const Presence.absent();
  BigInt get records => _readInteger(_fields["records"]);
  BigInt get requestsSent => _readInteger(_fields["requests_sent"]);
  num get seconds => _fields["seconds"] as num;
  String get tokenEstimateMethod => _fields["token_estimate_method"] as String;
  Presence<PersistenceObservation> get usagePersistence =>
      _fields.containsKey("usage_persistence")
          ? Presence.present(
              PersistenceObservation.read(_fields["usage_persistence"]))
          : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "attempts",
            "cache_answers",
            "call_id",
            "estimated_cost_usd",
            "held_model_mismatch",
            "input_tokens",
            "largest_request_bytes",
            "largest_request_estimated_input_tokens",
            "model",
            "output_tokens",
            "records",
            "requests_sent",
            "seconds",
            "token_estimate_method",
            "usage_persistence"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

sealed class InputDeclaration extends _NativeValue {
  InputDeclaration(super.fields);
  factory InputDeclaration.read(Object? value) {
    final map = _readObject(value);
    if (map["type"] == "string") return InputDeclarationStringRoot.read(map);
    if (map["type"] == "object") return InputDeclarationObjectRoot.read(map);
    throw FormatException("Unknown InputDeclaration alternative");
  }
}

final class InputDeclarationObjectRoot extends InputDeclaration {
  InputDeclarationObjectRoot._(super.fields);
  factory InputDeclarationObjectRoot.read(Object? value) =>
      InputDeclarationObjectRoot._(_readObject(value));
  Map<String, InputPropertyType> get properties =>
      (_fields["properties"] as Map<String, Object?>)
          .map((k, v) => MapEntry(k, InputPropertyType.read(v)));
  Presence<List<String>> get required => _fields.containsKey("required")
      ? Presence.present((_fields["required"] as List)
          .map((v) => v as String)
          .toList(growable: false))
      : const Presence.absent();
  String get type => _fields["type"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["properties", "required", "type"].contains(entry.key))
            entry.key: entry.value
      });
}

final class InputDeclarationStringRoot extends InputDeclaration {
  InputDeclarationStringRoot._(super.fields);
  factory InputDeclarationStringRoot.read(Object? value) =>
      InputDeclarationStringRoot._(_readObject(value));
  String get type => _fields["type"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["type"].contains(entry.key)) entry.key: entry.value
      });
}

sealed class InputPropertyType extends _NativeValue {
  InputPropertyType(super.fields);
  factory InputPropertyType.read(Object? value) {
    final map = _readObject(value);
    if (map["type"] == "string") return InputPropertyTypeString.read(map);
    if (map["type"] == "number") return InputPropertyTypeNumber.read(map);
    if (map["type"] == "boolean") return InputPropertyTypeBoolean.read(map);
    if (map["type"] == "array") return InputPropertyTypeArray.read(map);
    throw FormatException("Unknown InputPropertyType alternative");
  }
}

final class InputPropertyTypeArray extends InputPropertyType {
  InputPropertyTypeArray._(super.fields);
  factory InputPropertyTypeArray.read(Object? value) =>
      InputPropertyTypeArray._(_readObject(value));
  StringRoot get items => StringRoot.read(_fields["items"]);
  String get type => _fields["type"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["items", "type"].contains(entry.key)) entry.key: entry.value
      });
}

final class InputPropertyTypeBoolean extends InputPropertyType {
  InputPropertyTypeBoolean._(super.fields);
  factory InputPropertyTypeBoolean.read(Object? value) =>
      InputPropertyTypeBoolean._(_readObject(value));
  String get type => _fields["type"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["type"].contains(entry.key)) entry.key: entry.value
      });
}

final class InputPropertyTypeNumber extends InputPropertyType {
  InputPropertyTypeNumber._(super.fields);
  factory InputPropertyTypeNumber.read(Object? value) =>
      InputPropertyTypeNumber._(_readObject(value));
  String get type => _fields["type"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["type"].contains(entry.key)) entry.key: entry.value
      });
}

final class InputPropertyTypeString extends InputPropertyType {
  InputPropertyTypeString._(super.fields);
  factory InputPropertyTypeString.read(Object? value) =>
      InputPropertyTypeString._(_readObject(value));
  String get type => _fields["type"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["type"].contains(entry.key)) entry.key: entry.value
      });
}

final class Label extends _NativeValue {
  Label._(super.fields);
  factory Label.read(Object? value) => Label._(_readObject(value));
  Presence<Object?> get description => _fields.containsKey("description")
      ? Presence.present(_fields["description"])
      : const Presence.absent();
  String get name => _fields["name"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["description", "name"].contains(entry.key))
            entry.key: entry.value
      });
}

final class Meta extends _NativeValue {
  Meta._(super.fields);
  factory Meta.read(Object? value) => Meta._(_readObject(value));
  Presence<String> get answeredBy => _fields.containsKey("answered_by")
      ? Presence.present(_fields["answered_by"] as String)
      : const Presence.absent();
  Presence<List<Attempt>> get attempts => _fields.containsKey("attempts")
      ? Presence.present((_fields["attempts"] as List)
          .map((v) => Attempt.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<BatchSetting> get batchSetting =>
      _fields.containsKey("batch_setting")
          ? Presence.present(BatchSetting.read(_fields["batch_setting"]))
          : const Presence.absent();
  Presence<BatchWarning> get batchWarning =>
      _fields.containsKey("batch_warning")
          ? Presence.present(BatchWarning.read(_fields["batch_warning"]))
          : const Presence.absent();
  bool get cached => _fields["cached"] as bool;
  Presence<String> get contextSha256 => _fields.containsKey("context_sha256")
      ? Presence.present(_fields["context_sha256"] as String)
      : const Presence.absent();
  BigInt get failedQuestions => _readInteger(_fields["failed_questions"]);
  String get model => _fields["model"] as String;
  List<Observation> get observations => (_fields["observations"] as List)
      .map((v) => Observation.read(v))
      .toList(growable: false);
  String? get origin =>
      _fields["origin"] == null ? null : _fields["origin"] as String;
  Presence<ProfileWarning> get profileWarning =>
      _fields.containsKey("profile_warning")
          ? Presence.present(ProfileWarning.read(_fields["profile_warning"]))
          : const Presence.absent();
  Presence<String> get questionSha256 => _fields.containsKey("question_sha256")
      ? Presence.present(_fields["question_sha256"] as String)
      : const Presence.absent();
  List<QuestionSource> get questionSources =>
      (_fields["question_sources"] as List)
          .map((v) => QuestionSource.read(v))
          .toList(growable: false);
  Presence<String> get questionsSha256 =>
      _fields.containsKey("questions_sha256")
          ? Presence.present(_fields["questions_sha256"] as String)
          : const Presence.absent();
  List<String> get requests => (_fields["requests"] as List)
      .map((v) => v as String)
      .toList(growable: false);
  BigInt get requestsSent => _readInteger(_fields["requests_sent"]);
  String get tool => _fields["tool"] as String;
  String get url => _fields["url"] as String;
  Presence<Usage> get usage => _fields.containsKey("usage")
      ? Presence.present(Usage.read(_fields["usage"]))
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "answered_by",
            "attempts",
            "batch_setting",
            "batch_warning",
            "cached",
            "context_sha256",
            "failed_questions",
            "model",
            "observations",
            "origin",
            "profile_warning",
            "question_sha256",
            "question_sources",
            "questions_sha256",
            "requests",
            "requests_sent",
            "tool",
            "url",
            "usage"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class ObjectRoot extends _NativeValue {
  ObjectRoot._(super.fields);
  factory ObjectRoot.read(Object? value) => ObjectRoot._(_readObject(value));
  Map<String, InputPropertyType> get properties =>
      (_fields["properties"] as Map<String, Object?>)
          .map((k, v) => MapEntry(k, InputPropertyType.read(v)));
  Presence<List<String>> get required => _fields.containsKey("required")
      ? Presence.present((_fields["required"] as List)
          .map((v) => v as String)
          .toList(growable: false))
      : const Presence.absent();
  String get type => _fields["type"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["properties", "required", "type"].contains(entry.key))
            entry.key: entry.value
      });
}

sealed class Observation extends _NativeValue {
  Observation(super.fields);
  factory Observation.read(Object? value) {
    final map = _readObject(value);
    if (map.containsKey("observation_id"))
      return ObservationObservationId.read(map);
    if (map.containsKey("failure_id")) return ObservationFailureId.read(map);
    throw FormatException("Unknown Observation alternative");
  }
}

final class ObservationFailureId extends Observation {
  ObservationFailureId._(super.fields);
  factory ObservationFailureId.read(Object? value) =>
      ObservationFailureId._(_readObject(value));
  String get failureId => _fields["failure_id"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["failure_id"].contains(entry.key)) entry.key: entry.value
      });
}

final class ObservationObservationId extends Observation {
  ObservationObservationId._(super.fields);
  factory ObservationObservationId.read(Object? value) =>
      ObservationObservationId._(_readObject(value));
  String get observationId => _fields["observation_id"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["observation_id"].contains(entry.key)) entry.key: entry.value
      });
}

final class PersistenceObservation extends _NativeValue {
  PersistenceObservation._(super.fields);
  factory PersistenceObservation.read(Object? value) =>
      PersistenceObservation._(_readObject(value));
  Presence<String> get advice => _fields.containsKey("advice")
      ? Presence.present(_fields["advice"] as String)
      : const Presence.absent();
  String get observedAt => _fields["observed_at"] as String;
  String get state => _fields["state"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["advice", "observed_at", "state"].contains(entry.key))
            entry.key: entry.value
      });
}

final class PhysicalSource extends _NativeValue {
  PhysicalSource._(super.fields);
  factory PhysicalSource.read(Object? value) =>
      PhysicalSource._(_readObject(value));
  String get file => _fields["file"] as String;
  Presence<BigInt> get firstLine => _fields.containsKey("first_line")
      ? Presence.present(_readInteger(_fields["first_line"]))
      : const Presence.absent();
  Presence<BigInt> get lastLine => _fields.containsKey("last_line")
      ? Presence.present(_readInteger(_fields["last_line"]))
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["file", "first_line", "last_line"].contains(entry.key))
            entry.key: entry.value
      });
}

final class Position extends _NativeValue {
  Position._(super.fields);
  factory Position.read(Object? value) => Position._(_readObject(value));
  String? get file =>
      _fields["file"] == null ? null : _fields["file"] as String;
  Presence<BigInt> get first => _fields.containsKey("first")
      ? Presence.present(_readInteger(_fields["first"]))
      : const Presence.absent();
  Presence<List<String>> get images => _fields.containsKey("images")
      ? Presence.present((_fields["images"] as List)
          .map((v) => v as String)
          .toList(growable: false))
      : const Presence.absent();
  Presence<BigInt> get last => _fields.containsKey("last")
      ? Presence.present(_readInteger(_fields["last"]))
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["file", "first", "images", "last"].contains(entry.key))
            entry.key: entry.value
      });
}

final class QuestionSource extends _NativeValue {
  QuestionSource._(super.fields);
  factory QuestionSource.read(Object? value) =>
      QuestionSource._(_readObject(value));
  String get answeredBy => _fields["answered_by"] as String;
  Presence<BigInt> get batchSize => _fields.containsKey("batch_size")
      ? Presence.present(_readInteger(_fields["batch_size"]))
      : const Presence.absent();
  String get origin => _fields["origin"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["answered_by", "batch_size", "origin"].contains(entry.key))
            entry.key: entry.value
      });
}

final class RankMember extends _NativeValue {
  RankMember._(super.fields);
  factory RankMember.read(Object? value) => RankMember._(_readObject(value));
  String get name => _fields["name"] as String;
  RankMemberResult get result => RankMemberResult.read(_fields["result"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["name", "result"].contains(entry.key)) entry.key: entry.value
      });
}

final class RankMemberResult extends _NativeValue {
  RankMemberResult._(super.fields);
  factory RankMemberResult.read(Object? value) =>
      RankMemberResult._(_readObject(value));
  Answer get answer => Answer.read(_fields["answer"]);
  String get answerId => _fields["answer_id"] as String;
  Presence<List<Image>> get images => _fields.containsKey("images")
      ? Presence.present((_fields["images"] as List)
          .map((v) => Image.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Meta get meta => Meta.read(_fields["meta"]);
  ReadableQuestion get question => ReadableQuestion.read(_fields["question"]);
  String get schema => _fields["schema"] as String;
  Presence<PhysicalSource> get source => _fields.containsKey("source")
      ? Presence.present(PhysicalSource.read(_fields["source"]))
      : const Presence.absent();
  Object? get threshold => _fields["threshold"];
  BigInt get value => _readInteger(_fields["value"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "answer",
            "answer_id",
            "images",
            "meta",
            "question",
            "schema",
            "source",
            "threshold",
            "value"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

sealed class ReadableQuestion extends _NativeValue {
  ReadableQuestion(super.fields);
  factory ReadableQuestion.read(Object? value) {
    final map = _readObject(value);
    if (map["verb"] == "decide") return ReadableQuestionDecide.read(map);
    if (map["verb"] == "choose") return ReadableQuestionChoose.read(map);
    if (map["verb"] == "tag") return ReadableQuestionTag.read(map);
    if (map["verb"] == "score") return ReadableQuestionScore.read(map);
    throw FormatException("Unknown ReadableQuestion alternative");
  }
}

final class FindQuestionInfo extends _NativeValue {
  FindQuestionInfo._(super.fields);
  factory FindQuestionInfo.read(Object? value) =>
      FindQuestionInfo._(_readObject(value));
  Presence<Batch> get batch => _fields.containsKey("batch")
      ? Presence.present(Batch.read(_fields["batch"]))
      : const Presence.absent();
  Presence<InputDeclaration> get contextSchema =>
      _fields.containsKey("context_schema")
          ? Presence.present(InputDeclaration.read(_fields["context_schema"]))
          : const Presence.absent();
  Presence<InputDeclaration> get itemSchema =>
      _fields.containsKey("item_schema")
          ? Presence.present(InputDeclaration.read(_fields["item_schema"]))
          : const Presence.absent();
  Presence<List<Label>> get labelDetails => _fields.containsKey("label_details")
      ? Presence.present((_fields["label_details"] as List)
          .map((v) => Label.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<String> get model => _fields.containsKey("model")
      ? Presence.present(_fields["model"] as String)
      : const Presence.absent();
  Presence<String> get name => _fields.containsKey("name")
      ? Presence.present(_fields["name"] as String)
      : const Presence.absent();
  bool get none => _fields["none"] as bool;
  Presence<List<String>> get on => _fields.containsKey("on")
      ? Presence.present((_fields["on"] as List)
          .map((v) => v as String)
          .toList(growable: false))
      : const Presence.absent();
  Presence<String> get profile => _fields.containsKey("profile")
      ? Presence.present(_fields["profile"] as String)
      : const Presence.absent();
  Object? get text => _fields["text"];
  String get verb => _fields["verb"] as String;
  Presence<BigInt> get wordingVersion => _fields.containsKey("wording_version")
      ? Presence.present(_readInteger(_fields["wording_version"]))
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "batch",
            "context_schema",
            "item_schema",
            "label_details",
            "model",
            "name",
            "none",
            "on",
            "profile",
            "text",
            "verb",
            "wording_version"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class RecognitionQuestionInfo extends _NativeValue {
  RecognitionQuestionInfo._(super.fields);
  factory RecognitionQuestionInfo.read(Object? value) =>
      RecognitionQuestionInfo._(_readObject(value));
  Presence<Batch> get batch => _fields.containsKey("batch")
      ? Presence.present(Batch.read(_fields["batch"]))
      : const Presence.absent();
  Presence<InputDeclaration> get contextSchema =>
      _fields.containsKey("context_schema")
          ? Presence.present(InputDeclaration.read(_fields["context_schema"]))
          : const Presence.absent();
  Presence<Object?> get entityDefinition =>
      _fields.containsKey("entity_definition")
          ? Presence.present(_fields["entity_definition"])
          : const Presence.absent();
  Presence<Object?> get instructions => _fields.containsKey("instructions")
      ? Presence.present(_fields["instructions"])
      : const Presence.absent();
  Presence<InputDeclaration> get itemSchema =>
      _fields.containsKey("item_schema")
          ? Presence.present(InputDeclaration.read(_fields["item_schema"]))
          : const Presence.absent();
  Map<String, Object?> get kinds =>
      (_fields["kinds"] as Map<String, Object?>).map((k, v) => MapEntry(k, v));
  Presence<List<Label>> get labelDetails => _fields.containsKey("label_details")
      ? Presence.present((_fields["label_details"] as List)
          .map((v) => Label.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<String> get mode => _fields.containsKey("mode")
      ? Presence.present(_fields["mode"] as String)
      : const Presence.absent();
  Presence<String> get model => _fields.containsKey("model")
      ? Presence.present(_fields["model"] as String)
      : const Presence.absent();
  Presence<String> get name => _fields.containsKey("name")
      ? Presence.present(_fields["name"] as String)
      : const Presence.absent();
  Presence<List<String>> get on => _fields.containsKey("on")
      ? Presence.present((_fields["on"] as List)
          .map((v) => v as String)
          .toList(growable: false))
      : const Presence.absent();
  Presence<String> get profile => _fields.containsKey("profile")
      ? Presence.present(_fields["profile"] as String)
      : const Presence.absent();
  Presence<Object?> get relationThreshold =>
      _fields.containsKey("relation_threshold")
          ? Presence.present(_fields["relation_threshold"])
          : const Presence.absent();
  Presence<List<RelationRule>> get relations => _fields.containsKey("relations")
      ? Presence.present((_fields["relations"] as List)
          .map((v) => RelationRule.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<BigInt> get snippetPieces => _fields.containsKey("snippet_pieces")
      ? Presence.present(_readInteger(_fields["snippet_pieces"]))
      : const Presence.absent();
  Presence<RecognitionStageContext> get stageContext => _fields
          .containsKey("stage_context")
      ? Presence.present(RecognitionStageContext.read(_fields["stage_context"]))
      : const Presence.absent();
  Object? get threshold => _fields["threshold"];
  String get verb => _fields["verb"] as String;
  Presence<BigInt> get wordingVersion => _fields.containsKey("wording_version")
      ? Presence.present(_readInteger(_fields["wording_version"]))
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "batch",
            "context_schema",
            "entity_definition",
            "instructions",
            "item_schema",
            "kinds",
            "label_details",
            "mode",
            "model",
            "name",
            "on",
            "profile",
            "relation_threshold",
            "relations",
            "snippet_pieces",
            "stage_context",
            "threshold",
            "verb",
            "wording_version"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class RelationQuestionInfo extends _NativeValue {
  RelationQuestionInfo._(super.fields);
  factory RelationQuestionInfo.read(Object? value) =>
      RelationQuestionInfo._(_readObject(value));
  Presence<Batch> get batch => _fields.containsKey("batch")
      ? Presence.present(Batch.read(_fields["batch"]))
      : const Presence.absent();
  Presence<InputDeclaration> get contextSchema =>
      _fields.containsKey("context_schema")
          ? Presence.present(InputDeclaration.read(_fields["context_schema"]))
          : const Presence.absent();
  RelateFields? get fields =>
      _fields["fields"] == null ? null : RelateFields.read(_fields["fields"]);
  Presence<InputDeclaration> get itemSchema =>
      _fields.containsKey("item_schema")
          ? Presence.present(InputDeclaration.read(_fields["item_schema"]))
          : const Presence.absent();
  Presence<List<Label>> get labelDetails => _fields.containsKey("label_details")
      ? Presence.present((_fields["label_details"] as List)
          .map((v) => Label.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<String> get model => _fields.containsKey("model")
      ? Presence.present(_fields["model"] as String)
      : const Presence.absent();
  Presence<String> get name => _fields.containsKey("name")
      ? Presence.present(_fields["name"] as String)
      : const Presence.absent();
  Presence<List<String>> get on => _fields.containsKey("on")
      ? Presence.present((_fields["on"] as List)
          .map((v) => v as String)
          .toList(growable: false))
      : const Presence.absent();
  Presence<String> get profile => _fields.containsKey("profile")
      ? Presence.present(_fields["profile"] as String)
      : const Presence.absent();
  List<RelationRule> get relations => (_fields["relations"] as List)
      .map((v) => RelationRule.read(v))
      .toList(growable: false);
  Object? get threshold => _fields["threshold"];
  String get verb => _fields["verb"] as String;
  Presence<BigInt> get wordingVersion => _fields.containsKey("wording_version")
      ? Presence.present(_readInteger(_fields["wording_version"]))
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "batch",
            "context_schema",
            "fields",
            "item_schema",
            "label_details",
            "model",
            "name",
            "on",
            "profile",
            "relations",
            "threshold",
            "verb",
            "wording_version"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class ReadableQuestionChoose extends ReadableQuestion {
  ReadableQuestionChoose._(super.fields);
  factory ReadableQuestionChoose.read(Object? value) =>
      ReadableQuestionChoose._(_readObject(value));
  Presence<Batch> get batch => _fields.containsKey("batch")
      ? Presence.present(Batch.read(_fields["batch"]))
      : const Presence.absent();
  Presence<InputDeclaration> get contextSchema =>
      _fields.containsKey("context_schema")
          ? Presence.present(InputDeclaration.read(_fields["context_schema"]))
          : const Presence.absent();
  Presence<InputDeclaration> get itemSchema =>
      _fields.containsKey("item_schema")
          ? Presence.present(InputDeclaration.read(_fields["item_schema"]))
          : const Presence.absent();
  Presence<List<Label>> get labelDetails => _fields.containsKey("label_details")
      ? Presence.present((_fields["label_details"] as List)
          .map((v) => Label.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<String> get model => _fields.containsKey("model")
      ? Presence.present(_fields["model"] as String)
      : const Presence.absent();
  Presence<String> get name => _fields.containsKey("name")
      ? Presence.present(_fields["name"] as String)
      : const Presence.absent();
  Presence<List<String>> get on => _fields.containsKey("on")
      ? Presence.present((_fields["on"] as List)
          .map((v) => v as String)
          .toList(growable: false))
      : const Presence.absent();
  Presence<String> get profile => _fields.containsKey("profile")
      ? Presence.present(_fields["profile"] as String)
      : const Presence.absent();
  Presence<BigInt> get wordingVersion => _fields.containsKey("wording_version")
      ? Presence.present(_readInteger(_fields["wording_version"]))
      : const Presence.absent();
  List<String> get options => (_fields["options"] as List)
      .map((v) => v as String)
      .toList(growable: false);
  Object? get text => _fields["text"];
  String get verb => _fields["verb"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "batch",
            "context_schema",
            "item_schema",
            "label_details",
            "model",
            "name",
            "on",
            "profile",
            "wording_version",
            "options",
            "text",
            "verb"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class ReadableQuestionDecide extends ReadableQuestion {
  ReadableQuestionDecide._(super.fields);
  factory ReadableQuestionDecide.read(Object? value) =>
      ReadableQuestionDecide._(_readObject(value));
  Presence<Batch> get batch => _fields.containsKey("batch")
      ? Presence.present(Batch.read(_fields["batch"]))
      : const Presence.absent();
  Presence<InputDeclaration> get contextSchema =>
      _fields.containsKey("context_schema")
          ? Presence.present(InputDeclaration.read(_fields["context_schema"]))
          : const Presence.absent();
  Presence<InputDeclaration> get itemSchema =>
      _fields.containsKey("item_schema")
          ? Presence.present(InputDeclaration.read(_fields["item_schema"]))
          : const Presence.absent();
  Presence<List<Label>> get labelDetails => _fields.containsKey("label_details")
      ? Presence.present((_fields["label_details"] as List)
          .map((v) => Label.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<String> get model => _fields.containsKey("model")
      ? Presence.present(_fields["model"] as String)
      : const Presence.absent();
  Presence<String> get name => _fields.containsKey("name")
      ? Presence.present(_fields["name"] as String)
      : const Presence.absent();
  Presence<List<String>> get on => _fields.containsKey("on")
      ? Presence.present((_fields["on"] as List)
          .map((v) => v as String)
          .toList(growable: false))
      : const Presence.absent();
  Presence<String> get profile => _fields.containsKey("profile")
      ? Presence.present(_fields["profile"] as String)
      : const Presence.absent();
  Presence<BigInt> get wordingVersion => _fields.containsKey("wording_version")
      ? Presence.present(_readInteger(_fields["wording_version"]))
      : const Presence.absent();
  Presence<Object?> get falseValue => _fields.containsKey("false")
      ? Presence.present(_fields["false"])
      : const Presence.absent();
  Object? get text => _fields["text"];
  Presence<Object?> get trueValue => _fields.containsKey("true")
      ? Presence.present(_fields["true"])
      : const Presence.absent();
  String get verb => _fields["verb"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "batch",
            "context_schema",
            "item_schema",
            "label_details",
            "model",
            "name",
            "on",
            "profile",
            "wording_version",
            "false",
            "text",
            "true",
            "verb"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class ReadableQuestionScore extends ReadableQuestion {
  ReadableQuestionScore._(super.fields);
  factory ReadableQuestionScore.read(Object? value) =>
      ReadableQuestionScore._(_readObject(value));
  Presence<Batch> get batch => _fields.containsKey("batch")
      ? Presence.present(Batch.read(_fields["batch"]))
      : const Presence.absent();
  Presence<InputDeclaration> get contextSchema =>
      _fields.containsKey("context_schema")
          ? Presence.present(InputDeclaration.read(_fields["context_schema"]))
          : const Presence.absent();
  Presence<InputDeclaration> get itemSchema =>
      _fields.containsKey("item_schema")
          ? Presence.present(InputDeclaration.read(_fields["item_schema"]))
          : const Presence.absent();
  Presence<List<Label>> get labelDetails => _fields.containsKey("label_details")
      ? Presence.present((_fields["label_details"] as List)
          .map((v) => Label.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<String> get model => _fields.containsKey("model")
      ? Presence.present(_fields["model"] as String)
      : const Presence.absent();
  Presence<String> get name => _fields.containsKey("name")
      ? Presence.present(_fields["name"] as String)
      : const Presence.absent();
  Presence<List<String>> get on => _fields.containsKey("on")
      ? Presence.present((_fields["on"] as List)
          .map((v) => v as String)
          .toList(growable: false))
      : const Presence.absent();
  Presence<String> get profile => _fields.containsKey("profile")
      ? Presence.present(_fields["profile"] as String)
      : const Presence.absent();
  Presence<BigInt> get wordingVersion => _fields.containsKey("wording_version")
      ? Presence.present(_readInteger(_fields["wording_version"]))
      : const Presence.absent();
  List<String> get levels => (_fields["levels"] as List)
      .map((v) => v as String)
      .toList(growable: false);
  Object? get text => _fields["text"];
  String get verb => _fields["verb"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "batch",
            "context_schema",
            "item_schema",
            "label_details",
            "model",
            "name",
            "on",
            "profile",
            "wording_version",
            "levels",
            "text",
            "verb"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class ReadableQuestionTag extends ReadableQuestion {
  ReadableQuestionTag._(super.fields);
  factory ReadableQuestionTag.read(Object? value) =>
      ReadableQuestionTag._(_readObject(value));
  Presence<Batch> get batch => _fields.containsKey("batch")
      ? Presence.present(Batch.read(_fields["batch"]))
      : const Presence.absent();
  Presence<InputDeclaration> get contextSchema =>
      _fields.containsKey("context_schema")
          ? Presence.present(InputDeclaration.read(_fields["context_schema"]))
          : const Presence.absent();
  Presence<InputDeclaration> get itemSchema =>
      _fields.containsKey("item_schema")
          ? Presence.present(InputDeclaration.read(_fields["item_schema"]))
          : const Presence.absent();
  Presence<List<Label>> get labelDetails => _fields.containsKey("label_details")
      ? Presence.present((_fields["label_details"] as List)
          .map((v) => Label.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<String> get model => _fields.containsKey("model")
      ? Presence.present(_fields["model"] as String)
      : const Presence.absent();
  Presence<String> get name => _fields.containsKey("name")
      ? Presence.present(_fields["name"] as String)
      : const Presence.absent();
  Presence<List<String>> get on => _fields.containsKey("on")
      ? Presence.present((_fields["on"] as List)
          .map((v) => v as String)
          .toList(growable: false))
      : const Presence.absent();
  Presence<String> get profile => _fields.containsKey("profile")
      ? Presence.present(_fields["profile"] as String)
      : const Presence.absent();
  Presence<BigInt> get wordingVersion => _fields.containsKey("wording_version")
      ? Presence.present(_readInteger(_fields["wording_version"]))
      : const Presence.absent();
  List<String> get labels => (_fields["labels"] as List)
      .map((v) => v as String)
      .toList(growable: false);
  Object? get text => _fields["text"];
  String get verb => _fields["verb"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "batch",
            "context_schema",
            "item_schema",
            "label_details",
            "model",
            "name",
            "on",
            "profile",
            "wording_version",
            "labels",
            "text",
            "verb"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class Recognition extends _NativeValue {
  Recognition._(super.fields);
  factory Recognition.read(Object? value) => Recognition._(_readObject(value));
  RecognitionOdds get answer => RecognitionOdds.read(_fields["answer"]);
  String get answerId => _fields["answer_id"] as String;
  Presence<String> get file => _fields.containsKey("file")
      ? Presence.present(_fields["file"] as String)
      : const Presence.absent();
  Presence<BigInt> get firstLine => _fields.containsKey("first_line")
      ? Presence.present(_readInteger(_fields["first_line"]))
      : const Presence.absent();
  Presence<BigInt> get index => _fields.containsKey("index")
      ? Presence.present(_readInteger(_fields["index"]))
      : const Presence.absent();
  Presence<Object?> get input => _fields.containsKey("input")
      ? Presence.present(_fields["input"])
      : const Presence.absent();
  Presence<BigInt> get lastLine => _fields.containsKey("last_line")
      ? Presence.present(_readInteger(_fields["last_line"]))
      : const Presence.absent();
  Meta get meta => Meta.read(_fields["meta"]);
  Presence<Position> get position => _fields.containsKey("position")
      ? Presence.present(Position.read(_fields["position"]))
      : const Presence.absent();
  RecognitionQuestionInfo get question =>
      RecognitionQuestionInfo.read(_fields["question"]);
  String get schema => _fields["schema"] as String;
  Presence<PhysicalSource> get source => _fields.containsKey("source")
      ? Presence.present(PhysicalSource.read(_fields["source"]))
      : const Presence.absent();
  Recognize get value => Recognize.read(_fields["value"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "answer",
            "answer_id",
            "file",
            "first_line",
            "index",
            "input",
            "last_line",
            "meta",
            "position",
            "question",
            "schema",
            "source",
            "value"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class RecognitionEdgeDocument extends _NativeValue {
  RecognitionEdgeDocument._(super.fields);
  factory RecognitionEdgeDocument.read(Object? value) =>
      RecognitionEdgeDocument._(_readObject(value));
  bool get either => _fields["either"] as bool;
  num get probability => _fields["probability"] as num;
  String get relation => _fields["relation"] as String;
  Entity get source => Entity.read(_fields["source"]);
  Entity get target => Entity.read(_fields["target"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["either", "probability", "relation", "source", "target"]
              .contains(entry.key))
            entry.key: entry.value
      });
}

sealed class RecognitionOdds extends _NativeValue {
  RecognitionOdds(super.fields);
  factory RecognitionOdds.read(Object? value) {
    final map = _readObject(value);
    if (map.containsKey("names") &&
        map.containsKey("pairs") &&
        map.containsKey("pieces") &&
        map.containsKey("proposals"))
      return RecognitionOddsRecognizeAnswer.read(map);
    if (map.containsKey("pieces") &&
        map.containsKey("proposals") &&
        !map.containsKey("names") &&
        !map.containsKey("pairs")) return RecognitionOddsBoundaryOdds.read(map);
    throw FormatException("Unknown RecognitionOdds alternative");
  }
}

final class RecognitionOddsBoundaryOdds extends RecognitionOdds {
  RecognitionOddsBoundaryOdds._(super.fields);
  factory RecognitionOddsBoundaryOdds.read(Object? value) =>
      RecognitionOddsBoundaryOdds._(_readObject(value));
  List<PieceOdds> get pieces => (_fields["pieces"] as List)
      .map((v) => PieceOdds.read(v))
      .toList(growable: false);
  List<BoundaryProposal> get proposals => (_fields["proposals"] as List)
      .map((v) => BoundaryProposal.read(v))
      .toList(growable: false);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["pieces", "proposals"].contains(entry.key))
            entry.key: entry.value
      });
}

final class RecognitionOddsRecognizeAnswer extends RecognitionOdds {
  RecognitionOddsRecognizeAnswer._(super.fields);
  factory RecognitionOddsRecognizeAnswer.read(Object? value) =>
      RecognitionOddsRecognizeAnswer._(_readObject(value));
  List<NameOdds> get names => (_fields["names"] as List)
      .map((v) => NameOdds.read(v))
      .toList(growable: false);
  List<PairOdds> get pairs => (_fields["pairs"] as List)
      .map((v) => PairOdds.read(v))
      .toList(growable: false);
  List<PieceOdds> get pieces => (_fields["pieces"] as List)
      .map((v) => PieceOdds.read(v))
      .toList(growable: false);
  List<RecognitionProposal> get proposals => (_fields["proposals"] as List)
      .map((v) => RecognitionProposal.read(v))
      .toList(growable: false);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["names", "pairs", "pieces", "proposals"].contains(entry.key))
            entry.key: entry.value
      });
}

final class RecognitionProposal extends _NativeValue {
  RecognitionProposal._(super.fields);
  factory RecognitionProposal.read(Object? value) =>
      RecognitionProposal._(_readObject(value));
  BigInt get end => _readInteger(_fields["end"]);
  bool get kept => _fields["kept"] as bool;
  Presence<String> get kind => _fields.containsKey("kind")
      ? Presence.present(_fields["kind"] as String)
      : const Presence.absent();
  Presence<Place> get selected => _fields.containsKey("selected")
      ? Presence.present(Place.read(_fields["selected"]))
      : const Presence.absent();
  num get spanProbability => _fields["span_probability"] as num;
  BigInt get start => _readInteger(_fields["start"]);
  Presence<num> get strength => _fields.containsKey("strength")
      ? Presence.present(_fields["strength"] as num)
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "end",
            "kept",
            "kind",
            "selected",
            "span_probability",
            "start",
            "strength"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class RecognitionStageContext extends _NativeValue {
  RecognitionStageContext._(super.fields);
  factory RecognitionStageContext.read(Object? value) =>
      RecognitionStageContext._(_readObject(value));
  Presence<String> get boundary => _fields.containsKey("boundary")
      ? Presence.present(_fields["boundary"] as String)
      : const Presence.absent();
  Presence<String> get kindEdge => _fields.containsKey("kind_edge")
      ? Presence.present(_fields["kind_edge"] as String)
      : const Presence.absent();
  Presence<String> get relation => _fields.containsKey("relation")
      ? Presence.present(_fields["relation"] as String)
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["boundary", "kind_edge", "relation"].contains(entry.key))
            entry.key: entry.value
      });
}

final class Relation extends _NativeValue {
  Relation._(super.fields);
  factory Relation.read(Object? value) => Relation._(_readObject(value));
  Answers get answer => Answers.read(_fields["answer"]);
  String get answerId => _fields["answer_id"] as String;
  Presence<String> get file => _fields.containsKey("file")
      ? Presence.present(_fields["file"] as String)
      : const Presence.absent();
  Presence<BigInt> get firstLine => _fields.containsKey("first_line")
      ? Presence.present(_readInteger(_fields["first_line"]))
      : const Presence.absent();
  Presence<BigInt> get index => _fields.containsKey("index")
      ? Presence.present(_readInteger(_fields["index"]))
      : const Presence.absent();
  Presence<Object?> get input => _fields.containsKey("input")
      ? Presence.present(_fields["input"])
      : const Presence.absent();
  Presence<List<SessionInputSource>> get inputSources =>
      _fields.containsKey("input_sources")
          ? Presence.present((_fields["input_sources"] as List)
              .map((v) => SessionInputSource.read(v))
              .toList(growable: false))
          : const Presence.absent();
  Presence<BigInt> get lastLine => _fields.containsKey("last_line")
      ? Presence.present(_readInteger(_fields["last_line"]))
      : const Presence.absent();
  Meta get meta => Meta.read(_fields["meta"]);
  Presence<Position> get position => _fields.containsKey("position")
      ? Presence.present(Position.read(_fields["position"]))
      : const Presence.absent();
  RelationQuestionInfo get question =>
      RelationQuestionInfo.read(_fields["question"]);
  String get schema => _fields["schema"] as String;
  List<RelatedEntityEdge> get value => (_fields["value"] as List)
      .map((v) => RelatedEntityEdge.read(v))
      .toList(growable: false);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "answer",
            "answer_id",
            "file",
            "first_line",
            "index",
            "input",
            "input_sources",
            "last_line",
            "meta",
            "position",
            "question",
            "schema",
            "value"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

sealed class RelationEndpoint extends _NativeValue {
  RelationEndpoint(super.fields);
  String get name;
  factory RelationEndpoint.read(Object? value) {
    final map = _readObject(value);
    if (map.containsKey("kind") &&
        map.containsKey("name") &&
        !map.containsKey("file") &&
        !map.containsKey("ordinal") &&
        !map.containsKey("record"))
      return RelationEndpointRelatedEntity.read(map);
    if (map.containsKey("file") &&
        map.containsKey("kind") &&
        map.containsKey("name") &&
        map.containsKey("ordinal") &&
        map.containsKey("record")) return RelationEndpointSource.read(map);
    throw FormatException("Unknown RelationEndpoint alternative");
  }
}

final class RelationEndpointRelatedEntity extends RelationEndpoint {
  RelationEndpointRelatedEntity._(super.fields);
  factory RelationEndpointRelatedEntity.read(Object? value) =>
      RelationEndpointRelatedEntity._(_readObject(value));
  String get kind => _fields["kind"] as String;
  String get name => _fields["name"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "name"].contains(entry.key)) entry.key: entry.value
      });
}

final class RelationEndpointSource extends RelationEndpoint {
  RelationEndpointSource._(super.fields);
  factory RelationEndpointSource.read(Object? value) =>
      RelationEndpointSource._(_readObject(value));
  String? get file =>
      _fields["file"] == null ? null : _fields["file"] as String;
  Presence<BigInt> get firstLine => _fields.containsKey("first_line")
      ? Presence.present(_readInteger(_fields["first_line"]))
      : const Presence.absent();
  String get kind => _fields["kind"] as String;
  Presence<BigInt> get lastLine => _fields.containsKey("last_line")
      ? Presence.present(_readInteger(_fields["last_line"]))
      : const Presence.absent();
  String get name => _fields["name"] as String;
  BigInt get ordinal => _readInteger(_fields["ordinal"]);
  Object? get record => _fields["record"];
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "file",
            "first_line",
            "kind",
            "last_line",
            "name",
            "ordinal",
            "record"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

sealed class RelationMember extends _NativeValue {
  RelationMember(super.fields);
  factory RelationMember.read(Object? value) {
    final map = _readObject(value);
    if (map.containsKey("answer_id")) return RelationMemberAnswerId.read(map);
    if (map.containsKey("failure_id")) return RelationMemberFailureId.read(map);
    throw FormatException("Unknown RelationMember alternative");
  }
}

final class RelationMemberAnswerId extends RelationMember {
  RelationMemberAnswerId._(super.fields);
  factory RelationMemberAnswerId.read(Object? value) =>
      RelationMemberAnswerId._(_readObject(value));
  String get direction => _fields["direction"] as String;
  String get method => _fields["method"] as String;
  List<Observation> get observations => (_fields["observations"] as List)
      .map((v) => Observation.read(v))
      .toList(growable: false);
  ReadableQuestion get question => ReadableQuestion.read(_fields["question"]);
  List<QuestionSource> get questionSources =>
      (_fields["question_sources"] as List)
          .map((v) => QuestionSource.read(v))
          .toList(growable: false);
  String get reads => _fields["reads"] as String;
  String get relation => _fields["relation"] as String;
  String get request => _fields["request"] as String;
  RelatedEntity get source => RelatedEntity.read(_fields["source"]);
  RelatedEntity? get target =>
      _fields["target"] == null ? null : RelatedEntity.read(_fields["target"]);
  Object? get threshold => _fields["threshold"];
  Presence<Usage> get usage => _fields.containsKey("usage")
      ? Presence.present(Usage.read(_fields["usage"]))
      : const Presence.absent();
  bool get accepted => _fields["accepted"] as bool;
  Answer get answer => Answer.read(_fields["answer"]);
  String get answerId => _fields["answer_id"] as String;
  num get probability => _fields["probability"] as num;
  String get sourceName => source.name;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "direction",
            "method",
            "observations",
            "question",
            "question_sources",
            "reads",
            "relation",
            "request",
            "source",
            "target",
            "threshold",
            "usage",
            "accepted",
            "answer",
            "answer_id",
            "probability"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class RelationMemberFailureId extends RelationMember {
  RelationMemberFailureId._(super.fields);
  factory RelationMemberFailureId.read(Object? value) =>
      RelationMemberFailureId._(_readObject(value));
  String get direction => _fields["direction"] as String;
  String get method => _fields["method"] as String;
  List<Observation> get observations => (_fields["observations"] as List)
      .map((v) => Observation.read(v))
      .toList(growable: false);
  ReadableQuestion get question => ReadableQuestion.read(_fields["question"]);
  List<QuestionSource> get questionSources =>
      (_fields["question_sources"] as List)
          .map((v) => QuestionSource.read(v))
          .toList(growable: false);
  String get reads => _fields["reads"] as String;
  String get relation => _fields["relation"] as String;
  String get request => _fields["request"] as String;
  RelatedEntity get source => RelatedEntity.read(_fields["source"]);
  RelatedEntity? get target =>
      _fields["target"] == null ? null : RelatedEntity.read(_fields["target"]);
  Object? get threshold => _fields["threshold"];
  Presence<Usage> get usage => _fields.containsKey("usage")
      ? Presence.present(Usage.read(_fields["usage"]))
      : const Presence.absent();
  Failure get failure => Failure.read(_fields["failure"]);
  String get failureId => _fields["failure_id"] as String;
  String get sourceName => source.name;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "direction",
            "method",
            "observations",
            "question",
            "question_sources",
            "reads",
            "relation",
            "request",
            "source",
            "target",
            "threshold",
            "usage",
            "failure",
            "failure_id"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

sealed class SendBudgetDenial extends _NativeValue {
  SendBudgetDenial(super.fields);
  factory SendBudgetDenial.read(Object? value) {
    final map = _readObject(value);
    if (map["kind"] == "before_first_send")
      return SendBudgetDenialBeforeFirstSend.read(map);
    if (map["kind"] == "before_additional_send")
      return SendBudgetDenialBeforeAdditionalSend.read(map);
    if (map["kind"] == "before_retry")
      return SendBudgetDenialBeforeRetry.read(map);
    throw FormatException("Unknown SendBudgetDenial alternative");
  }
}

final class SendBudgetDenialBeforeAdditionalSend extends SendBudgetDenial {
  SendBudgetDenialBeforeAdditionalSend._(super.fields);
  factory SendBudgetDenialBeforeAdditionalSend.read(Object? value) =>
      SendBudgetDenialBeforeAdditionalSend._(_readObject(value));
  String get kind => _fields["kind"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind"].contains(entry.key)) entry.key: entry.value
      });
}

final class SendBudgetDenialBeforeFirstSend extends SendBudgetDenial {
  SendBudgetDenialBeforeFirstSend._(super.fields);
  factory SendBudgetDenialBeforeFirstSend.read(Object? value) =>
      SendBudgetDenialBeforeFirstSend._(_readObject(value));
  String get kind => _fields["kind"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind"].contains(entry.key)) entry.key: entry.value
      });
}

final class SendBudgetDenialBeforeRetry extends SendBudgetDenial {
  SendBudgetDenialBeforeRetry._(super.fields);
  factory SendBudgetDenialBeforeRetry.read(Object? value) =>
      SendBudgetDenialBeforeRetry._(_readObject(value));
  String get kind => _fields["kind"] as String;
  BigInt get lastStatus => _readInteger(_fields["last_status"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "last_status"].contains(entry.key))
            entry.key: entry.value
      });
}

final class Stopped extends _NativeValue {
  Stopped._(super.fields);
  factory Stopped.read(Object? value) => Stopped._(_readObject(value));
  Presence<BigInt> get at => _fields.containsKey("at")
      ? Presence.present(_readInteger(_fields["at"]))
      : const Presence.absent();
  String get cause => _fields["cause"] as String;
  bool get retryable => _fields["retryable"] as bool;
  Presence<BigInt> get status => _fields.containsKey("status")
      ? Presence.present(_readInteger(_fields["status"]))
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["at", "cause", "retryable", "status"].contains(entry.key))
            entry.key: entry.value
      });
}

final class StringRoot extends _NativeValue {
  StringRoot._(super.fields);
  factory StringRoot.read(Object? value) => StringRoot._(_readObject(value));
  String get type => _fields["type"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["type"].contains(entry.key)) entry.key: entry.value
      });
}

final class Usage extends _NativeValue {
  Usage._(super.fields);
  factory Usage.read(Object? value) => Usage._(_readObject(value));
  Presence<BigInt> get inputTokens => _fields.containsKey("input_tokens")
      ? Presence.present(_readInteger(_fields["input_tokens"]))
      : const Presence.absent();
  Presence<BigInt> get outputTokens => _fields.containsKey("output_tokens")
      ? Presence.present(_readInteger(_fields["output_tokens"]))
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["input_tokens", "output_tokens"].contains(entry.key))
            entry.key: entry.value
      });
}

final class AnnotatedField implements _EncodedValue {
  final Object? value;
  const AnnotatedField(this.value);
  factory AnnotatedField.read(Object? value) =>
      value is AnnotatedField ? value : AnnotatedField(value);
  Object? _toValue() => value;
  bool get asBoolean => value as bool;
  String get asString => value as String;
  List<String> get asArray =>
      (value as List).map((v) => v as String).toList(growable: false);
  num get asNumber => value as num;
  Failed get asObject => Failed.read(value);
}

sealed class Answer extends _NativeValue {
  Answer(super.fields);
  factory Answer.read(Object? value) {
    final map = _readObject(value);
    if (map["kind"] == "yes_no") return AnswerYesNo.read(map);
    if (map["kind"] == "choice") return AnswerChoice.read(map);
    if (map["kind"] == "tag") return AnswerTag.read(map);
    if (map["kind"] == "score") return AnswerScore.read(map);
    throw FormatException("Unknown Answer alternative");
  }
}

final class AnswerChoice extends Answer {
  AnswerChoice._(super.fields);
  factory AnswerChoice.read(Object? value) =>
      AnswerChoice._(_readObject(value));
  Presence<num> get confidence => _fields.containsKey("confidence")
      ? Presence.present(_fields["confidence"] as num)
      : const Presence.absent();
  String get kind => _fields["kind"] as String;
  String get pick => _fields["pick"] as String;
  Map<String, num> get probabilities =>
      (_fields["probabilities"] as Map<String, Object?>)
          .map((k, v) => MapEntry(k, v as num));
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["confidence", "kind", "pick", "probabilities"]
              .contains(entry.key))
            entry.key: entry.value
      });
}

final class AnswerScore extends Answer {
  AnswerScore._(super.fields);
  factory AnswerScore.read(Object? value) => AnswerScore._(_readObject(value));
  Presence<num> get confidence => _fields.containsKey("confidence")
      ? Presence.present(_fields["confidence"] as num)
      : const Presence.absent();
  String get kind => _fields["kind"] as String;
  String get level => _fields["level"] as String;
  Map<String, num> get probabilities =>
      (_fields["probabilities"] as Map<String, Object?>)
          .map((k, v) => MapEntry(k, v as num));
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["confidence", "kind", "level", "probabilities"]
              .contains(entry.key))
            entry.key: entry.value
      });
}

final class AnswerTag extends Answer {
  AnswerTag._(super.fields);
  factory AnswerTag.read(Object? value) => AnswerTag._(_readObject(value));
  String get kind => _fields["kind"] as String;
  Map<String, num> get probabilities =>
      (_fields["probabilities"] as Map<String, Object?>)
          .map((k, v) => MapEntry(k, v as num));
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "probabilities"].contains(entry.key))
            entry.key: entry.value
      });
}

final class AnswerYesNo extends Answer {
  AnswerYesNo._(super.fields);
  factory AnswerYesNo.read(Object? value) => AnswerYesNo._(_readObject(value));
  String get kind => _fields["kind"] as String;
  num get probability => _fields["probability"] as num;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "probability"].contains(entry.key))
            entry.key: entry.value
      });
}

final class BatchSetting implements _EncodedValue {
  final Object? value;
  const BatchSetting._(this.value);
  factory BatchSetting.read(Object? value) =>
      value is BatchSetting ? value : BatchSetting._(value);
  Object? _toValue() => value;
  const BatchSetting.integer(BigInt this.value);
  BigInt get asInteger => _readInteger(value);
  const BatchSetting.text(String this.value);
  String get asText => value as String;
}

final class BatchWarning extends _NativeValue {
  BatchWarning._(super.fields);
  factory BatchWarning.read(Object? value) =>
      BatchWarning._(_readObject(value));
  BatchSetting get running => BatchSetting.read(_fields["running"]);
  BatchSetting get tunedFor => BatchSetting.read(_fields["tuned_for"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["running", "tuned_for"].contains(entry.key))
            entry.key: entry.value
      });
}

final class Entity extends _NativeValue {
  Entity._(super.fields);
  factory Entity.read(Object? value) => Entity._(_readObject(value));
  BigInt get end => _readInteger(_fields["end"]);
  Presence<String> get file => _fields.containsKey("file")
      ? Presence.present(_fields["file"] as String)
      : const Presence.absent();
  Presence<BigInt> get firstLine => _fields.containsKey("first_line")
      ? Presence.present(_readInteger(_fields["first_line"]))
      : const Presence.absent();
  String get kind => _fields["kind"] as String;
  Presence<BigInt> get lastLine => _fields.containsKey("last_line")
      ? Presence.present(_readInteger(_fields["last_line"]))
      : const Presence.absent();
  BigInt get length => _readInteger(_fields["length"]);
  BigInt get start => _readInteger(_fields["start"]);
  num get strength => _fields["strength"] as num;
  String get text => _fields["text"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "end",
            "file",
            "first_line",
            "kind",
            "last_line",
            "length",
            "start",
            "strength",
            "text"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class EntityEdge extends _NativeValue {
  EntityEdge._(super.fields);
  factory EntityEdge.read(Object? value) => EntityEdge._(_readObject(value));
  Presence<bool> get either => _fields.containsKey("either")
      ? Presence.present(_fields["either"] as bool)
      : const Presence.absent();
  num get probability => _fields["probability"] as num;
  String get relation => _fields["relation"] as String;
  Entity get source => Entity.read(_fields["source"]);
  Entity get target => Entity.read(_fields["target"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["either", "probability", "relation", "source", "target"]
              .contains(entry.key))
            entry.key: entry.value
      });
}

final class Failed extends _NativeValue {
  Failed._(super.fields);
  factory Failed.read(Object? value) => Failed._(_readObject(value));
  Failure get failed => Failure.read(_fields["failed"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["failed"].contains(entry.key)) entry.key: entry.value
      });
}

final class Failure extends _NativeValue {
  Failure._(super.fields);
  factory Failure.read(Object? value) => Failure._(_readObject(value));
  String get cause => _fields["cause"] as String;
  String get kind => _fields["kind"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["cause", "kind"].contains(entry.key)) entry.key: entry.value
      });
}

final class FindAnswer extends _NativeValue {
  FindAnswer._(super.fields);
  factory FindAnswer.read(Object? value) => FindAnswer._(_readObject(value));
  Presence<num> get confidence => _fields.containsKey("confidence")
      ? Presence.present(_fields["confidence"] as num)
      : const Presence.absent();
  String get kind => _fields["kind"] as String;
  String get pick => _fields["pick"] as String;
  Map<String, num> get probabilities =>
      (_fields["probabilities"] as Map<String, Object?>)
          .map((k, v) => MapEntry(k, v as num));
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["confidence", "kind", "pick", "probabilities"]
              .contains(entry.key))
            entry.key: entry.value
      });
}

final class NameOdds extends _NativeValue {
  NameOdds._(super.fields);
  factory NameOdds.read(Object? value) => NameOdds._(_readObject(value));
  Map<String, num>? get edges => _fields["edges"] == null
      ? null
      : (_fields["edges"] as Map<String, Object?>)
          .map((k, v) => MapEntry(k, v as num));
  BigInt get end => _readInteger(_fields["end"]);
  Map<String, num>? get kinds => _fields["kinds"] == null
      ? null
      : (_fields["kinds"] as Map<String, Object?>)
          .map((k, v) => MapEntry(k, v as num));
  BigInt get start => _readInteger(_fields["start"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["edges", "end", "kinds", "start"].contains(entry.key))
            entry.key: entry.value
      });
}

final class PairOdds extends _NativeValue {
  PairOdds._(super.fields);
  factory PairOdds.read(Object? value) => PairOdds._(_readObject(value));
  num get probability => _fields["probability"] as num;
  String get relation => _fields["relation"] as String;
  Place get source => Place.read(_fields["source"]);
  Place get target => Place.read(_fields["target"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["probability", "relation", "source", "target"]
              .contains(entry.key))
            entry.key: entry.value
      });
}

final class PieceOdds extends _NativeValue {
  PieceOdds._(super.fields);
  factory PieceOdds.read(Object? value) => PieceOdds._(_readObject(value));
  BigInt get end => _readInteger(_fields["end"]);
  BigInt get start => _readInteger(_fields["start"]);
  Map<String, num> get tags => (_fields["tags"] as Map<String, Object?>)
      .map((k, v) => MapEntry(k, v as num));
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["end", "start", "tags"].contains(entry.key))
            entry.key: entry.value
      });
}

final class Place extends _NativeValue {
  Place._(super.fields);
  factory Place.read(Object? value) => Place._(_readObject(value));
  BigInt get end => _readInteger(_fields["end"]);
  BigInt get start => _readInteger(_fields["start"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["end", "start"].contains(entry.key)) entry.key: entry.value
      });
}

final class ProfileWarning extends _NativeValue {
  ProfileWarning._(super.fields);
  factory ProfileWarning.read(Object? value) =>
      ProfileWarning._(_readObject(value));
  String get running => _fields["running"] as String;
  String get tunedFor => _fields["tuned_for"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["running", "tuned_for"].contains(entry.key))
            entry.key: entry.value
      });
}

sealed class Recognize extends _NativeValue {
  Recognize(super.fields);
  factory Recognize.read(Object? value) {
    final map = _readObject(value);
    if (map.containsKey("entities") &&
        !map.containsKey("mode") &&
        !map.containsKey("proposals")) return RecognizeEntities.read(map);
    if (map.containsKey("mode") &&
        map.containsKey("proposals") &&
        !map.containsKey("entities")) return RecognizeModeProposals.read(map);
    throw FormatException("Unknown Recognize alternative");
  }
}

final class RecognizeAnswer extends _NativeValue {
  RecognizeAnswer._(super.fields);
  factory RecognizeAnswer.read(Object? value) =>
      RecognizeAnswer._(_readObject(value));
  List<NameOdds> get names => (_fields["names"] as List)
      .map((v) => NameOdds.read(v))
      .toList(growable: false);
  List<PairOdds> get pairs => (_fields["pairs"] as List)
      .map((v) => PairOdds.read(v))
      .toList(growable: false);
  List<PieceOdds> get pieces => (_fields["pieces"] as List)
      .map((v) => PieceOdds.read(v))
      .toList(growable: false);
  List<RecognitionProposal> get proposals => (_fields["proposals"] as List)
      .map((v) => RecognitionProposal.read(v))
      .toList(growable: false);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["names", "pairs", "pieces", "proposals"].contains(entry.key))
            entry.key: entry.value
      });
}

final class RecognizeEntities extends Recognize {
  RecognizeEntities._(super.fields);
  factory RecognizeEntities.read(Object? value) =>
      RecognizeEntities._(_readObject(value));
  List<Entity> get entities => (_fields["entities"] as List)
      .map((v) => Entity.read(v))
      .toList(growable: false);
  Presence<List<EntityEdge>> get relations => _fields.containsKey("relations")
      ? Presence.present((_fields["relations"] as List)
          .map((v) => EntityEdge.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["entities", "relations"].contains(entry.key))
            entry.key: entry.value
      });
}

final class RecognizeModeProposals extends Recognize {
  RecognizeModeProposals._(super.fields);
  factory RecognizeModeProposals.read(Object? value) =>
      RecognizeModeProposals._(_readObject(value));
  String get mode => _fields["mode"] as String;
  List<BoundaryProposal> get proposals => (_fields["proposals"] as List)
      .map((v) => BoundaryProposal.read(v))
      .toList(growable: false);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["mode", "proposals"].contains(entry.key)) entry.key: entry.value
      });
}

final class RelateFields extends _NativeValue {
  RelateFields._(super.fields);
  factory RelateFields.read(Object? value) =>
      RelateFields._(_readObject(value));
  String get kind => _fields["kind"] as String;
  String get name => _fields["name"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "name"].contains(entry.key)) entry.key: entry.value
      });
}

final class RelatedEntity extends _NativeValue {
  RelatedEntity._(super.fields);
  factory RelatedEntity.read(Object? value) =>
      RelatedEntity._(_readObject(value));
  String get kind => _fields["kind"] as String;
  String get name => _fields["name"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "name"].contains(entry.key)) entry.key: entry.value
      });
}

final class RelatedEntityEdge extends _NativeValue {
  RelatedEntityEdge._(super.fields);
  factory RelatedEntityEdge.read(Object? value) =>
      RelatedEntityEdge._(_readObject(value));
  Presence<bool> get either => _fields.containsKey("either")
      ? Presence.present(_fields["either"] as bool)
      : const Presence.absent();
  num get probability => _fields["probability"] as num;
  String get relation => _fields["relation"] as String;
  RelationEndpoint get source => RelationEndpoint.read(_fields["source"]);
  RelationEndpoint get target => RelationEndpoint.read(_fields["target"]);
  String get sourceName => source.name;
  String get targetName => target.name;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["either", "probability", "relation", "source", "target"]
              .contains(entry.key))
            entry.key: entry.value
      });
}

final class RelationRule extends _NativeValue {
  RelationRule._(super.fields);
  factory RelationRule.read(Object? value) =>
      RelationRule._(_readObject(value));
  bool get either => _fields["either"] as bool;
  String get name => _fields["name"] as String;
  String get reads => _fields["reads"] as String;
  Presence<bool> get single => _fields.containsKey("single")
      ? Presence.present(_fields["single"] as bool)
      : const Presence.absent();
  String get source => _fields["source"] as String;
  String get target => _fields["target"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["either", "name", "reads", "single", "source", "target"]
              .contains(entry.key))
            entry.key: entry.value
      });
}

final class SessionAnnotation extends _NativeValue {
  SessionAnnotation._(super.fields);
  factory SessionAnnotation.read(Object? value) =>
      SessionAnnotation._(_readObject(value));
  String get name => _fields["name"] as String;
  AnnotationValue get value => AnnotationValue.read(_fields["value"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["name", "value"].contains(entry.key)) entry.key: entry.value
      });
}

final class SessionInputSource extends _NativeValue {
  SessionInputSource._(super.fields);
  factory SessionInputSource.read(Object? value) =>
      SessionInputSource._(_readObject(value));
  BigInt get index => _readInteger(_fields["index"]);
  PhysicalSource get source => PhysicalSource.read(_fields["source"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["index", "source"].contains(entry.key)) entry.key: entry.value
      });
}

sealed class SessionJudgment extends _NativeValue {
  SessionJudgment(super.fields);
  factory SessionJudgment.read(Object? value) {
    final map = _readObject(value);
    if (map["kind"] == "decision") return SessionJudgmentDecision.read(map);
    if (map["kind"] == "choice") return SessionJudgmentChoice.read(map);
    if (map["kind"] == "score") return SessionJudgmentScore.read(map);
    if (map["kind"] == "tags") return SessionJudgmentTags.read(map);
    throw FormatException("Unknown SessionJudgment alternative");
  }
}

final class SessionJudgmentChoice extends SessionJudgment {
  SessionJudgmentChoice._(super.fields);
  factory SessionJudgmentChoice.read(Object? value) =>
      SessionJudgmentChoice._(_readObject(value));
  String get kind => _fields["kind"] as String;
  String? get value =>
      _fields["value"] == null ? null : _fields["value"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "value"].contains(entry.key)) entry.key: entry.value
      });
}

final class SessionJudgmentDecision extends SessionJudgment {
  SessionJudgmentDecision._(super.fields);
  factory SessionJudgmentDecision.read(Object? value) =>
      SessionJudgmentDecision._(_readObject(value));
  String get kind => _fields["kind"] as String;
  bool? get value => _fields["value"] == null ? null : _fields["value"] as bool;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "value"].contains(entry.key)) entry.key: entry.value
      });
}

final class SessionJudgmentScore extends SessionJudgment {
  SessionJudgmentScore._(super.fields);
  factory SessionJudgmentScore.read(Object? value) =>
      SessionJudgmentScore._(_readObject(value));
  String get kind => _fields["kind"] as String;
  num get value => _fields["value"] as num;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "value"].contains(entry.key)) entry.key: entry.value
      });
}

final class SessionJudgmentTags extends SessionJudgment {
  SessionJudgmentTags._(super.fields);
  factory SessionJudgmentTags.read(Object? value) =>
      SessionJudgmentTags._(_readObject(value));
  String get kind => _fields["kind"] as String;
  List<String> get value => (_fields["value"] as List)
      .map((v) => v as String)
      .toList(growable: false);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "value"].contains(entry.key)) entry.key: entry.value
      });
}

final class SessionNamedProbability extends _NativeValue {
  SessionNamedProbability._(super.fields);
  factory SessionNamedProbability.read(Object? value) =>
      SessionNamedProbability._(_readObject(value));
  String get name => _fields["name"] as String;
  num get probability => _fields["probability"] as num;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["name", "probability"].contains(entry.key))
            entry.key: entry.value
      });
}

sealed class SessionObservation extends _NativeValue {
  SessionObservation(super.fields);
  factory SessionObservation.read(Object? value) {
    final map = _readObject(value);
    if (map["kind"] == "question") return SessionObservationQuestion.read(map);
    if (map["kind"] == "row") return SessionObservationRow.read(map);
    throw FormatException("Unknown SessionObservation alternative");
  }
}

final class SessionObservationQuestion extends SessionObservation {
  SessionObservationQuestion._(super.fields);
  factory SessionObservationQuestion.read(Object? value) =>
      SessionObservationQuestion._(_readObject(value));
  SessionQuestionDetail get detail =>
      SessionQuestionDetail.read(_fields["detail"]);
  BigInt get index => _readInteger(_fields["index"]);
  String get kind => _fields["kind"] as String;
  Presence<String> get member => _fields.containsKey("member")
      ? Presence.present(_fields["member"] as String)
      : const Presence.absent();
  BigInt get position => _readInteger(_fields["position"]);
  Presence<String> get stage => _fields.containsKey("stage")
      ? Presence.present(_fields["stage"] as String)
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["detail", "index", "kind", "member", "position", "stage"]
              .contains(entry.key))
            entry.key: entry.value
      });
}

final class SessionObservationRow extends SessionObservation {
  SessionObservationRow._(super.fields);
  factory SessionObservationRow.read(Object? value) =>
      SessionObservationRow._(_readObject(value));
  BigInt get index => _readInteger(_fields["index"]);
  String get kind => _fields["kind"] as String;
  SessionObservedRow get value => SessionObservedRow.read(_fields["value"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["index", "kind", "value"].contains(entry.key))
            entry.key: entry.value
      });
}

sealed class SessionObservedRow extends _NativeValue {
  SessionObservedRow(super.fields);
  factory SessionObservedRow.read(Object? value) {
    final map = _readObject(value);
    if (map["kind"] == "judgment") return SessionObservedRowJudgment.read(map);
    if (map["kind"] == "annotated")
      return SessionObservedRowAnnotated.read(map);
    if (map["kind"] == "recognized")
      return SessionObservedRowRecognized.read(map);
    if (map["kind"] == "find") return SessionObservedRowFind.read(map);
    if (map["kind"] == "relations")
      return SessionObservedRowRelations.read(map);
    throw FormatException("Unknown SessionObservedRow alternative");
  }
}

final class SessionObservedRowAnnotated extends SessionObservedRow {
  SessionObservedRowAnnotated._(super.fields);
  factory SessionObservedRowAnnotated.read(Object? value) =>
      SessionObservedRowAnnotated._(_readObject(value));
  String get kind => _fields["kind"] as String;
  List<SessionAnnotation> get value => (_fields["value"] as List)
      .map((v) => SessionAnnotation.read(v))
      .toList(growable: false);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "value"].contains(entry.key)) entry.key: entry.value
      });
}

final class SessionObservedRowFind extends SessionObservedRow {
  SessionObservedRowFind._(super.fields);
  factory SessionObservedRowFind.read(Object? value) =>
      SessionObservedRowFind._(_readObject(value));
  String get kind => _fields["kind"] as String;
  BigInt? get value =>
      _fields["value"] == null ? null : _readInteger(_fields["value"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "value"].contains(entry.key)) entry.key: entry.value
      });
}

final class SessionObservedRowJudgment extends SessionObservedRow {
  SessionObservedRowJudgment._(super.fields);
  factory SessionObservedRowJudgment.read(Object? value) =>
      SessionObservedRowJudgment._(_readObject(value));
  String get kind => _fields["kind"] as String;
  SessionJudgment get value => SessionJudgment.read(_fields["value"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "value"].contains(entry.key)) entry.key: entry.value
      });
}

final class SessionObservedRowRecognized extends SessionObservedRow {
  SessionObservedRowRecognized._(super.fields);
  factory SessionObservedRowRecognized.read(Object? value) =>
      SessionObservedRowRecognized._(_readObject(value));
  String get kind => _fields["kind"] as String;
  SessionRecognition get value => SessionRecognition.read(_fields["value"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "value"].contains(entry.key)) entry.key: entry.value
      });
}

final class SessionObservedRowRelations extends SessionObservedRow {
  SessionObservedRowRelations._(super.fields);
  factory SessionObservedRowRelations.read(Object? value) =>
      SessionObservedRowRelations._(_readObject(value));
  String get kind => _fields["kind"] as String;
  List<SessionRelationEdge> get value => (_fields["value"] as List)
      .map((v) => SessionRelationEdge.read(v))
      .toList(growable: false);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "value"].contains(entry.key)) entry.key: entry.value
      });
}

sealed class SessionPacket extends _NativeValue {
  SessionPacket(super.fields);
  factory SessionPacket.read(Object? value) {
    final map = _readObject(value);
    if (map["function"] == "decide" && map["kind"] == "row")
      return SessionPacketDecideRow.read(map);
    if (map["function"] == "choose" && map["kind"] == "row")
      return SessionPacketChooseRow.read(map);
    if (map["function"] == "tag" && map["kind"] == "row")
      return SessionPacketTagRow.read(map);
    if (map["function"] == "score" && map["kind"] == "row")
      return SessionPacketScoreRow.read(map);
    if (map["function"] == "filter" && map["kind"] == "row")
      return SessionPacketFilterRow.read(map);
    if (map["function"] == "annotate" && map["kind"] == "row")
      return SessionPacketAnnotateRow.read(map);
    if (map["function"] == "decide" && map["kind"] == "aggregate")
      return SessionPacketDecideAggregate.read(map);
    if (map["function"] == "choose" && map["kind"] == "aggregate")
      return SessionPacketChooseAggregate.read(map);
    if (map["function"] == "tag" && map["kind"] == "aggregate")
      return SessionPacketTagAggregate.read(map);
    if (map["function"] == "score" && map["kind"] == "aggregate")
      return SessionPacketScoreAggregate.read(map);
    if (map["function"] == "filter" && map["kind"] == "aggregate")
      return SessionPacketFilterAggregate.read(map);
    if (map["function"] == "rank" && map["kind"] == "aggregate")
      return SessionPacketRankAggregate.read(map);
    if (map["function"] == "find" && map["kind"] == "aggregate")
      return SessionPacketFindAggregate.read(map);
    if (map["function"] == "annotate" && map["kind"] == "aggregate")
      return SessionPacketAnnotateAggregate.read(map);
    if (map["function"] == "recognize" && map["kind"] == "aggregate")
      return SessionPacketRecognizeAggregate.read(map);
    if (map["function"] == "relate" && map["kind"] == "aggregate")
      return SessionPacketRelateAggregate.read(map);
    if (map["kind"] == "observation") return SessionPacketObservation.read(map);
    if (map["kind"] == "terminal") return SessionPacketTerminal.read(map);
    throw FormatException("Unknown SessionPacket alternative");
  }
}

final class SessionPacketAnnotateAggregate extends SessionPacket {
  SessionPacketAnnotateAggregate._(super.fields);
  factory SessionPacketAnnotateAggregate.read(Object? value) =>
      SessionPacketAnnotateAggregate._(_readObject(value));
  String get function => _fields["function"] as String;
  String get kind => _fields["kind"] as String;
  List<Annotation> get value => (_fields["value"] as List)
      .map((v) => Annotation.read(v))
      .toList(growable: false);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["function", "kind", "value"].contains(entry.key))
            entry.key: entry.value
      });
}

final class SessionPacketAnnotateRow extends SessionPacket {
  SessionPacketAnnotateRow._(super.fields);
  factory SessionPacketAnnotateRow.read(Object? value) =>
      SessionPacketAnnotateRow._(_readObject(value));
  String get function => _fields["function"] as String;
  String get kind => _fields["kind"] as String;
  Annotation get value => Annotation.read(_fields["value"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["function", "kind", "value"].contains(entry.key))
            entry.key: entry.value
      });
}

final class SessionPacketChooseAggregate extends SessionPacket {
  SessionPacketChooseAggregate._(super.fields);
  factory SessionPacketChooseAggregate.read(Object? value) =>
      SessionPacketChooseAggregate._(_readObject(value));
  String get function => _fields["function"] as String;
  String get kind => _fields["kind"] as String;
  List<ChoiceResult> get value => (_fields["value"] as List)
      .map((v) => ChoiceResult.read(v))
      .toList(growable: false);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["function", "kind", "value"].contains(entry.key))
            entry.key: entry.value
      });
}

final class SessionPacketChooseRow extends SessionPacket {
  SessionPacketChooseRow._(super.fields);
  factory SessionPacketChooseRow.read(Object? value) =>
      SessionPacketChooseRow._(_readObject(value));
  String get function => _fields["function"] as String;
  String get kind => _fields["kind"] as String;
  ChoiceResult get value => ChoiceResult.read(_fields["value"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["function", "kind", "value"].contains(entry.key))
            entry.key: entry.value
      });
}

final class SessionPacketDecideAggregate extends SessionPacket {
  SessionPacketDecideAggregate._(super.fields);
  factory SessionPacketDecideAggregate.read(Object? value) =>
      SessionPacketDecideAggregate._(_readObject(value));
  String get function => _fields["function"] as String;
  String get kind => _fields["kind"] as String;
  List<DecisionResult> get value => (_fields["value"] as List)
      .map((v) => DecisionResult.read(v))
      .toList(growable: false);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["function", "kind", "value"].contains(entry.key))
            entry.key: entry.value
      });
}

final class SessionPacketDecideRow extends SessionPacket {
  SessionPacketDecideRow._(super.fields);
  factory SessionPacketDecideRow.read(Object? value) =>
      SessionPacketDecideRow._(_readObject(value));
  String get function => _fields["function"] as String;
  String get kind => _fields["kind"] as String;
  DecisionResult get value => DecisionResult.read(_fields["value"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["function", "kind", "value"].contains(entry.key))
            entry.key: entry.value
      });
}

final class SessionPacketFilterAggregate extends SessionPacket {
  SessionPacketFilterAggregate._(super.fields);
  factory SessionPacketFilterAggregate.read(Object? value) =>
      SessionPacketFilterAggregate._(_readObject(value));
  String get function => _fields["function"] as String;
  String get kind => _fields["kind"] as String;
  List<FilterResult> get value => (_fields["value"] as List)
      .map((v) => FilterResult.read(v))
      .toList(growable: false);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["function", "kind", "value"].contains(entry.key))
            entry.key: entry.value
      });
}

final class SessionPacketFilterRow extends SessionPacket {
  SessionPacketFilterRow._(super.fields);
  factory SessionPacketFilterRow.read(Object? value) =>
      SessionPacketFilterRow._(_readObject(value));
  String get function => _fields["function"] as String;
  String get kind => _fields["kind"] as String;
  FilterResult get value => FilterResult.read(_fields["value"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["function", "kind", "value"].contains(entry.key))
            entry.key: entry.value
      });
}

final class SessionPacketFindAggregate extends SessionPacket {
  SessionPacketFindAggregate._(super.fields);
  factory SessionPacketFindAggregate.read(Object? value) =>
      SessionPacketFindAggregate._(_readObject(value));
  String get function => _fields["function"] as String;
  String get kind => _fields["kind"] as String;
  Find get value => Find.read(_fields["value"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["function", "kind", "value"].contains(entry.key))
            entry.key: entry.value
      });
}

final class SessionPacketObservation extends SessionPacket {
  SessionPacketObservation._(super.fields);
  factory SessionPacketObservation.read(Object? value) =>
      SessionPacketObservation._(_readObject(value));
  String get function => _fields["function"] as String;
  String get kind => _fields["kind"] as String;
  SessionObservation get value => SessionObservation.read(_fields["value"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["function", "kind", "value"].contains(entry.key))
            entry.key: entry.value
      });
}

final class SessionPacketRankAggregate extends SessionPacket {
  SessionPacketRankAggregate._(super.fields);
  factory SessionPacketRankAggregate.read(Object? value) =>
      SessionPacketRankAggregate._(_readObject(value));
  String get function => _fields["function"] as String;
  String get kind => _fields["kind"] as String;
  List<RankResult> get value => (_fields["value"] as List)
      .map((v) => RankResult.read(v))
      .toList(growable: false);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["function", "kind", "value"].contains(entry.key))
            entry.key: entry.value
      });
}

final class SessionPacketRecognizeAggregate extends SessionPacket {
  SessionPacketRecognizeAggregate._(super.fields);
  factory SessionPacketRecognizeAggregate.read(Object? value) =>
      SessionPacketRecognizeAggregate._(_readObject(value));
  String get function => _fields["function"] as String;
  String get kind => _fields["kind"] as String;
  List<Recognition> get value => (_fields["value"] as List)
      .map((v) => Recognition.read(v))
      .toList(growable: false);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["function", "kind", "value"].contains(entry.key))
            entry.key: entry.value
      });
}

final class SessionPacketRelateAggregate extends SessionPacket {
  SessionPacketRelateAggregate._(super.fields);
  factory SessionPacketRelateAggregate.read(Object? value) =>
      SessionPacketRelateAggregate._(_readObject(value));
  String get function => _fields["function"] as String;
  String get kind => _fields["kind"] as String;
  Relation get value => Relation.read(_fields["value"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["function", "kind", "value"].contains(entry.key))
            entry.key: entry.value
      });
}

final class SessionPacketScoreAggregate extends SessionPacket {
  SessionPacketScoreAggregate._(super.fields);
  factory SessionPacketScoreAggregate.read(Object? value) =>
      SessionPacketScoreAggregate._(_readObject(value));
  String get function => _fields["function"] as String;
  String get kind => _fields["kind"] as String;
  List<ScoreResult> get value => (_fields["value"] as List)
      .map((v) => ScoreResult.read(v))
      .toList(growable: false);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["function", "kind", "value"].contains(entry.key))
            entry.key: entry.value
      });
}

final class SessionPacketScoreRow extends SessionPacket {
  SessionPacketScoreRow._(super.fields);
  factory SessionPacketScoreRow.read(Object? value) =>
      SessionPacketScoreRow._(_readObject(value));
  String get function => _fields["function"] as String;
  String get kind => _fields["kind"] as String;
  ScoreResult get value => ScoreResult.read(_fields["value"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["function", "kind", "value"].contains(entry.key))
            entry.key: entry.value
      });
}

final class SessionPacketTagAggregate extends SessionPacket {
  SessionPacketTagAggregate._(super.fields);
  factory SessionPacketTagAggregate.read(Object? value) =>
      SessionPacketTagAggregate._(_readObject(value));
  String get function => _fields["function"] as String;
  String get kind => _fields["kind"] as String;
  List<TagResult> get value => (_fields["value"] as List)
      .map((v) => TagResult.read(v))
      .toList(growable: false);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["function", "kind", "value"].contains(entry.key))
            entry.key: entry.value
      });
}

final class SessionPacketTagRow extends SessionPacket {
  SessionPacketTagRow._(super.fields);
  factory SessionPacketTagRow.read(Object? value) =>
      SessionPacketTagRow._(_readObject(value));
  String get function => _fields["function"] as String;
  String get kind => _fields["kind"] as String;
  TagResult get value => TagResult.read(_fields["value"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["function", "kind", "value"].contains(entry.key))
            entry.key: entry.value
      });
}

final class SessionPacketTerminal extends SessionPacket {
  SessionPacketTerminal._(super.fields);
  factory SessionPacketTerminal.read(Object? value) =>
      SessionPacketTerminal._(_readObject(value));
  Presence<Facts> get facts => _fields.containsKey("facts")
      ? Presence.present(Facts.read(_fields["facts"]))
      : const Presence.absent();
  Presence<CallError> get failure => _fields.containsKey("failure")
      ? Presence.present(CallError.read(_fields["failure"]))
      : const Presence.absent();
  String get kind => _fields["kind"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["facts", "failure", "kind"].contains(entry.key))
            entry.key: entry.value
      });
}

sealed class SessionProbabilities extends _NativeValue {
  SessionProbabilities(super.fields);
  factory SessionProbabilities.read(Object? value) {
    final map = _readObject(value);
    if (map["kind"] == "yes_no") return SessionProbabilitiesYesNo.read(map);
    if (map["kind"] == "named") return SessionProbabilitiesNamed.read(map);
    throw FormatException("Unknown SessionProbabilities alternative");
  }
}

final class SessionProbabilitiesNamed extends SessionProbabilities {
  SessionProbabilitiesNamed._(super.fields);
  factory SessionProbabilitiesNamed.read(Object? value) =>
      SessionProbabilitiesNamed._(_readObject(value));
  String get kind => _fields["kind"] as String;
  List<SessionNamedProbability> get value => (_fields["value"] as List)
      .map((v) => SessionNamedProbability.read(v))
      .toList(growable: false);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "value"].contains(entry.key)) entry.key: entry.value
      });
}

final class SessionProbabilitiesYesNo extends SessionProbabilities {
  SessionProbabilitiesYesNo._(super.fields);
  factory SessionProbabilitiesYesNo.read(Object? value) =>
      SessionProbabilitiesYesNo._(_readObject(value));
  String get kind => _fields["kind"] as String;
  num get value => _fields["value"] as num;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "value"].contains(entry.key)) entry.key: entry.value
      });
}

final class SessionQuestionDetail extends _NativeValue {
  SessionQuestionDetail._(super.fields);
  factory SessionQuestionDetail.read(Object? value) =>
      SessionQuestionDetail._(_readObject(value));
  Presence<String> get answerId => _fields.containsKey("answer_id")
      ? Presence.present(_fields["answer_id"] as String)
      : const Presence.absent();
  bool get cached => _fields["cached"] as bool;
  Presence<num> get confidence => _fields.containsKey("confidence")
      ? Presence.present(_fields["confidence"] as num)
      : const Presence.absent();
  BigInt get failedQuestions => _readInteger(_fields["failed_questions"]);
  Presence<Failure> get failure => _fields.containsKey("failure")
      ? Presence.present(Failure.read(_fields["failure"]))
      : const Presence.absent();
  Presence<String> get failureId => _fields.containsKey("failure_id")
      ? Presence.present(_fields["failure_id"] as String)
      : const Presence.absent();
  Presence<Object?> get input => _fields.containsKey("input")
      ? Presence.present(_fields["input"])
      : const Presence.absent();
  Presence<PhysicalSource> get inputSource =>
      _fields.containsKey("input_source")
          ? Presence.present(PhysicalSource.read(_fields["input_source"]))
          : const Presence.absent();
  List<SessionInputSource> get inputSources =>
      (_fields["input_sources"] as List)
          .map((v) => SessionInputSource.read(v))
          .toList(growable: false);
  List<Object?> get inputs =>
      (_fields["inputs"] as List).map((v) => v).toList(growable: false);
  String get model => _fields["model"] as String;
  List<Observation> get observations => (_fields["observations"] as List)
      .map((v) => Observation.read(v))
      .toList(growable: false);
  Presence<SessionProbabilities> get probabilities => _fields
          .containsKey("probabilities")
      ? Presence.present(SessionProbabilities.read(_fields["probabilities"]))
      : const Presence.absent();
  ReadableQuestion get question => ReadableQuestion.read(_fields["question"]);
  String get questionSha256 => _fields["question_sha256"] as String;
  List<QuestionSource> get questionSources =>
      (_fields["question_sources"] as List)
          .map((v) => QuestionSource.read(v))
          .toList(growable: false);
  Presence<String> get rawPick => _fields.containsKey("raw_pick")
      ? Presence.present(_fields["raw_pick"] as String)
      : const Presence.absent();
  Presence<Usage> get reportedUsage => _fields.containsKey("reported_usage")
      ? Presence.present(Usage.read(_fields["reported_usage"]))
      : const Presence.absent();
  List<String> get requests => (_fields["requests"] as List)
      .map((v) => v as String)
      .toList(growable: false);
  BigInt get requestsSent => _readInteger(_fields["requests_sent"]);
  Presence<Object?> get threshold => _fields.containsKey("threshold")
      ? Presence.present(_fields["threshold"])
      : const Presence.absent();
  String get url => _fields["url"] as String;
  Presence<TokenUsage> get usage => _fields.containsKey("usage")
      ? Presence.present(TokenUsage.read(_fields["usage"]))
      : const Presence.absent();
  Presence<Value> get value => _fields.containsKey("value")
      ? Presence.present(Value.read(_fields["value"]))
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "answer_id",
            "cached",
            "confidence",
            "failed_questions",
            "failure",
            "failure_id",
            "input",
            "input_source",
            "input_sources",
            "inputs",
            "model",
            "observations",
            "probabilities",
            "question",
            "question_sha256",
            "question_sources",
            "raw_pick",
            "reported_usage",
            "requests",
            "requests_sent",
            "threshold",
            "url",
            "usage",
            "value"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class SessionRecognition extends _NativeValue {
  SessionRecognition._(super.fields);
  factory SessionRecognition.read(Object? value) =>
      SessionRecognition._(_readObject(value));
  List<Entity> get entities => (_fields["entities"] as List)
      .map((v) => Entity.read(v))
      .toList(growable: false);
  String get mode => _fields["mode"] as String;
  Presence<List<BoundaryProposal>> get proposals => _fields
          .containsKey("proposals")
      ? Presence.present((_fields["proposals"] as List)
          .map((v) => BoundaryProposal.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<List<RecognitionEdgeDocument>> get relations =>
      _fields.containsKey("relations")
          ? Presence.present((_fields["relations"] as List)
              .map((v) => RecognitionEdgeDocument.read(v))
              .toList(growable: false))
          : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["entities", "mode", "proposals", "relations"]
              .contains(entry.key))
            entry.key: entry.value
      });
}

final class SessionRelationEdge extends _NativeValue {
  SessionRelationEdge._(super.fields);
  factory SessionRelationEdge.read(Object? value) =>
      SessionRelationEdge._(_readObject(value));
  bool get either => _fields["either"] as bool;
  num get probability => _fields["probability"] as num;
  String get relation => _fields["relation"] as String;
  EntityDocument get source => EntityDocument.read(_fields["source"]);
  EntityDocument get target => EntityDocument.read(_fields["target"]);
  String get sourceName => source.name;
  String get targetName => target.name;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["either", "probability", "relation", "source", "target"]
              .contains(entry.key))
            entry.key: entry.value
      });
}

final class SourceRelationEndpoint extends _NativeValue {
  SourceRelationEndpoint._(super.fields);
  factory SourceRelationEndpoint.read(Object? value) =>
      SourceRelationEndpoint._(_readObject(value));
  String? get file =>
      _fields["file"] == null ? null : _fields["file"] as String;
  Presence<BigInt> get firstLine => _fields.containsKey("first_line")
      ? Presence.present(_readInteger(_fields["first_line"]))
      : const Presence.absent();
  String get kind => _fields["kind"] as String;
  Presence<BigInt> get lastLine => _fields.containsKey("last_line")
      ? Presence.present(_readInteger(_fields["last_line"]))
      : const Presence.absent();
  String get name => _fields["name"] as String;
  BigInt get ordinal => _readInteger(_fields["ordinal"]);
  Object? get record => _fields["record"];
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "file",
            "first_line",
            "kind",
            "last_line",
            "name",
            "ordinal",
            "record"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class TokenUsage extends _NativeValue {
  TokenUsage._(super.fields);
  factory TokenUsage.read(Object? value) => TokenUsage._(_readObject(value));
  BigInt get inputTokens => _readInteger(_fields["input_tokens"]);
  BigInt get outputTokens => _readInteger(_fields["output_tokens"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["input_tokens", "output_tokens"].contains(entry.key))
            entry.key: entry.value
      });
}

final class Value implements _EncodedValue {
  final Object? value;
  const Value(this.value);
  factory Value.read(Object? value) => value is Value ? value : Value(value);
  Object? _toValue() => value;
  bool get asBoolean => value as bool;
  String get asString => value as String;
  List<String> get asArray =>
      (value as List).map((v) => v as String).toList(growable: false);
  num get asNumber => value as num;
}
