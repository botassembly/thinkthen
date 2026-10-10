// Generated from the shared Rust graph. Do not edit.
import 'values.dart';

final class Annotation extends NativeObject {
  Annotation._(super.json);
  factory Annotation.read(Object? value) => Annotation._(readObject(value));
  String get answerId => json["answer_id"] as String;
  Map<String, AnnotationMember> get answers =>
      (json["answers"] as Map<String, Object?>)
          .map((k, v) => MapEntry(k, AnnotationMember.read(v)));
  Presence<String> get file => json.containsKey("file")
      ? Presence.present(json["file"] as String)
      : const Presence.absent();
  Presence<BigInt> get firstLine => json.containsKey("first_line")
      ? Presence.present(readInteger(json["first_line"]))
      : const Presence.absent();
  Presence<BigInt> get index => json.containsKey("index")
      ? Presence.present(readInteger(json["index"]))
      : const Presence.absent();
  Object? get input => json["input"];
  Presence<BigInt> get lastLine => json.containsKey("last_line")
      ? Presence.present(readInteger(json["last_line"]))
      : const Presence.absent();
  Meta get meta => Meta.read(json["meta"]);
  Presence<Position> get position => json.containsKey("position")
      ? Presence.present(Position.read(json["position"]))
      : const Presence.absent();
  String get schema => json["schema"] as String;
  Presence<PhysicalSource> get source => json.containsKey("source")
      ? Presence.present(PhysicalSource.read(json["source"]))
      : const Presence.absent();
  Map<String, AnnotatedField> get value =>
      (json["value"] as Map<String, Object?>)
          .map((k, v) => MapEntry(k, AnnotatedField.read(v)));
}

sealed class AnnotationMember extends NativeObject {
  AnnotationMember(super.json);
  factory AnnotationMember.read(Object? value) {
    final map = readObject(value);
    if (map.containsKey("answer_id")) return AnnotationMemberAnswerId.read(map);
    if (map.containsKey("failure_id"))
      return AnnotationMemberFailureId.read(map);
    throw FormatException("Unknown AnnotationMember alternative");
  }
}

final class AnnotationMemberAnswerId extends AnnotationMember {
  AnnotationMemberAnswerId._(super.json);
  factory AnnotationMemberAnswerId.read(Object? value) =>
      AnnotationMemberAnswerId._(readObject(value));
  Answer get answer => Answer.read(json["answer"]);
  String get answerId => json["answer_id"] as String;
  List<Observation> get observations => (json["observations"] as List)
      .map((v) => Observation.read(v))
      .toList(growable: false);
  ReadableQuestion get question => ReadableQuestion.read(json["question"]);
  List<QuestionSource> get questionSources => (json["question_sources"] as List)
      .map((v) => QuestionSource.read(v))
      .toList(growable: false);
  String get request => json["request"] as String;
  Object? get threshold => json["threshold"] == null ? null : json["threshold"];
  Presence<Usage> get usage => json.containsKey("usage")
      ? Presence.present(Usage.read(json["usage"]))
      : const Presence.absent();
  Value get value => Value.read(json["value"]);
}

final class AnnotationMemberFailureId extends AnnotationMember {
  AnnotationMemberFailureId._(super.json);
  factory AnnotationMemberFailureId.read(Object? value) =>
      AnnotationMemberFailureId._(readObject(value));
  Failure get failure => Failure.read(json["failure"]);
  String get failureId => json["failure_id"] as String;
  List<Observation> get observations => (json["observations"] as List)
      .map((v) => Observation.read(v))
      .toList(growable: false);
  ReadableQuestion get question => ReadableQuestion.read(json["question"]);
  List<QuestionSource> get questionSources => (json["question_sources"] as List)
      .map((v) => QuestionSource.read(v))
      .toList(growable: false);
  String get request => json["request"] as String;
  Object? get threshold => json["threshold"] == null ? null : json["threshold"];
  Presence<Usage> get usage => json.containsKey("usage")
      ? Presence.present(Usage.read(json["usage"]))
      : const Presence.absent();
}

sealed class AnnotationValue extends NativeObject {
  AnnotationValue(super.json);
  factory AnnotationValue.read(Object? value) {
    final map = readObject(value);
    if (map["kind"] == "decision") return AnnotationValueDecision.read(map);
    if (map["kind"] == "choice") return AnnotationValueChoice.read(map);
    if (map["kind"] == "score") return AnnotationValueScore.read(map);
    if (map["kind"] == "tags") return AnnotationValueTags.read(map);
    if (map["kind"] == "failed") return AnnotationValueFailed.read(map);
    throw FormatException("Unknown AnnotationValue alternative");
  }
}

final class AnnotationValueChoice extends AnnotationValue {
  AnnotationValueChoice._(super.json);
  factory AnnotationValueChoice.read(Object? value) =>
      AnnotationValueChoice._(readObject(value));
  String get kind => json["kind"] as String;
  String? get value => json["value"] == null ? null : json["value"] as String;
}

final class AnnotationValueDecision extends AnnotationValue {
  AnnotationValueDecision._(super.json);
  factory AnnotationValueDecision.read(Object? value) =>
      AnnotationValueDecision._(readObject(value));
  String get kind => json["kind"] as String;
  bool? get value => json["value"] == null ? null : json["value"] as bool;
}

final class AnnotationValueFailed extends AnnotationValue {
  AnnotationValueFailed._(super.json);
  factory AnnotationValueFailed.read(Object? value) =>
      AnnotationValueFailed._(readObject(value));
  String get kind => json["kind"] as String;
  Failure get value => Failure.read(json["value"]);
}

final class AnnotationValueScore extends AnnotationValue {
  AnnotationValueScore._(super.json);
  factory AnnotationValueScore.read(Object? value) =>
      AnnotationValueScore._(readObject(value));
  String get kind => json["kind"] as String;
  num get value => json["value"] as num;
}

final class AnnotationValueTags extends AnnotationValue {
  AnnotationValueTags._(super.json);
  factory AnnotationValueTags.read(Object? value) =>
      AnnotationValueTags._(readObject(value));
  String get kind => json["kind"] as String;
  List<String> get value =>
      (json["value"] as List).map((v) => v as String).toList(growable: false);
}

final class Answers extends NativeObject {
  Answers._(super.json);
  factory Answers.read(Object? value) => Answers._(readObject(value));
  List<RelationMember> get questions => (json["questions"] as List)
      .map((v) => RelationMember.read(v))
      .toList(growable: false);
}

final class AtomicArrayOfString extends NativeObject {
  AtomicArrayOfString._(super.json);
  factory AtomicArrayOfString.read(Object? value) =>
      AtomicArrayOfString._(readObject(value));
  Answer get answer => Answer.read(json["answer"]);
  String get answerId => json["answer_id"] as String;
  Presence<List<Image>> get images => json.containsKey("images")
      ? Presence.present((json["images"] as List)
          .map((v) => Image.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<BigInt> get index => json.containsKey("index")
      ? Presence.present(readInteger(json["index"]))
      : const Presence.absent();
  Presence<Object?> get input => json.containsKey("input")
      ? Presence.present(json["input"])
      : const Presence.absent();
  Presence<List<RankMember>> get members => json.containsKey("members")
      ? Presence.present((json["members"] as List)
          .map((v) => RankMember.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Meta get meta => Meta.read(json["meta"]);
  ReadableQuestion get question => ReadableQuestion.read(json["question"]);
  Presence<String> get questionName => json.containsKey("question_name")
      ? Presence.present(json["question_name"] as String)
      : const Presence.absent();
  String get schema => json["schema"] as String;
  Presence<PhysicalSource> get source => json.containsKey("source")
      ? Presence.present(PhysicalSource.read(json["source"]))
      : const Presence.absent();
  Object? get threshold => json["threshold"] == null ? null : json["threshold"];
  List<String> get value =>
      (json["value"] as List).map((v) => v as String).toList(growable: false);
}

final class AtomicDecideValue extends NativeObject {
  AtomicDecideValue._(super.json);
  factory AtomicDecideValue.read(Object? value) =>
      AtomicDecideValue._(readObject(value));
  Answer get answer => Answer.read(json["answer"]);
  String get answerId => json["answer_id"] as String;
  Presence<List<Image>> get images => json.containsKey("images")
      ? Presence.present((json["images"] as List)
          .map((v) => Image.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<BigInt> get index => json.containsKey("index")
      ? Presence.present(readInteger(json["index"]))
      : const Presence.absent();
  Presence<Object?> get input => json.containsKey("input")
      ? Presence.present(json["input"])
      : const Presence.absent();
  Presence<List<RankMember>> get members => json.containsKey("members")
      ? Presence.present((json["members"] as List)
          .map((v) => RankMember.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Meta get meta => Meta.read(json["meta"]);
  ReadableQuestion get question => ReadableQuestion.read(json["question"]);
  Presence<String> get questionName => json.containsKey("question_name")
      ? Presence.present(json["question_name"] as String)
      : const Presence.absent();
  String get schema => json["schema"] as String;
  Presence<PhysicalSource> get source => json.containsKey("source")
      ? Presence.present(PhysicalSource.read(json["source"]))
      : const Presence.absent();
  Object? get threshold => json["threshold"] == null ? null : json["threshold"];
  DecideValue get value => DecideValue.read(json["value"]);
}

final class AtomicNonZeroUsize extends NativeObject {
  AtomicNonZeroUsize._(super.json);
  factory AtomicNonZeroUsize.read(Object? value) =>
      AtomicNonZeroUsize._(readObject(value));
  Answer get answer => Answer.read(json["answer"]);
  String get answerId => json["answer_id"] as String;
  Presence<List<Image>> get images => json.containsKey("images")
      ? Presence.present((json["images"] as List)
          .map((v) => Image.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<BigInt> get index => json.containsKey("index")
      ? Presence.present(readInteger(json["index"]))
      : const Presence.absent();
  Presence<Object?> get input => json.containsKey("input")
      ? Presence.present(json["input"])
      : const Presence.absent();
  Presence<List<RankMember>> get members => json.containsKey("members")
      ? Presence.present((json["members"] as List)
          .map((v) => RankMember.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Meta get meta => Meta.read(json["meta"]);
  ReadableQuestion get question => ReadableQuestion.read(json["question"]);
  Presence<String> get questionName => json.containsKey("question_name")
      ? Presence.present(json["question_name"] as String)
      : const Presence.absent();
  String get schema => json["schema"] as String;
  Presence<PhysicalSource> get source => json.containsKey("source")
      ? Presence.present(PhysicalSource.read(json["source"]))
      : const Presence.absent();
  Object? get threshold => json["threshold"] == null ? null : json["threshold"];
  BigInt get value => readInteger(json["value"]);
}

final class AtomicNullableString extends NativeObject {
  AtomicNullableString._(super.json);
  factory AtomicNullableString.read(Object? value) =>
      AtomicNullableString._(readObject(value));
  Answer get answer => Answer.read(json["answer"]);
  String get answerId => json["answer_id"] as String;
  Presence<List<Image>> get images => json.containsKey("images")
      ? Presence.present((json["images"] as List)
          .map((v) => Image.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<BigInt> get index => json.containsKey("index")
      ? Presence.present(readInteger(json["index"]))
      : const Presence.absent();
  Presence<Object?> get input => json.containsKey("input")
      ? Presence.present(json["input"])
      : const Presence.absent();
  Presence<List<RankMember>> get members => json.containsKey("members")
      ? Presence.present((json["members"] as List)
          .map((v) => RankMember.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Meta get meta => Meta.read(json["meta"]);
  ReadableQuestion get question => ReadableQuestion.read(json["question"]);
  Presence<String> get questionName => json.containsKey("question_name")
      ? Presence.present(json["question_name"] as String)
      : const Presence.absent();
  String get schema => json["schema"] as String;
  Presence<PhysicalSource> get source => json.containsKey("source")
      ? Presence.present(PhysicalSource.read(json["source"]))
      : const Presence.absent();
  Object? get threshold => json["threshold"] == null ? null : json["threshold"];
  String? get value => json["value"] == null ? null : json["value"] as String;
}

final class AtomicBoolean extends NativeObject {
  AtomicBoolean._(super.json);
  factory AtomicBoolean.read(Object? value) =>
      AtomicBoolean._(readObject(value));
  Answer get answer => Answer.read(json["answer"]);
  String get answerId => json["answer_id"] as String;
  Presence<List<Image>> get images => json.containsKey("images")
      ? Presence.present((json["images"] as List)
          .map((v) => Image.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<BigInt> get index => json.containsKey("index")
      ? Presence.present(readInteger(json["index"]))
      : const Presence.absent();
  Presence<Object?> get input => json.containsKey("input")
      ? Presence.present(json["input"])
      : const Presence.absent();
  Presence<List<RankMember>> get members => json.containsKey("members")
      ? Presence.present((json["members"] as List)
          .map((v) => RankMember.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Meta get meta => Meta.read(json["meta"]);
  ReadableQuestion get question => ReadableQuestion.read(json["question"]);
  Presence<String> get questionName => json.containsKey("question_name")
      ? Presence.present(json["question_name"] as String)
      : const Presence.absent();
  String get schema => json["schema"] as String;
  Presence<PhysicalSource> get source => json.containsKey("source")
      ? Presence.present(PhysicalSource.read(json["source"]))
      : const Presence.absent();
  Object? get threshold => json["threshold"] == null ? null : json["threshold"];
  bool get value => json["value"] as bool;
}

final class AtomicDouble extends NativeObject {
  AtomicDouble._(super.json);
  factory AtomicDouble.read(Object? value) => AtomicDouble._(readObject(value));
  Answer get answer => Answer.read(json["answer"]);
  String get answerId => json["answer_id"] as String;
  Presence<List<Image>> get images => json.containsKey("images")
      ? Presence.present((json["images"] as List)
          .map((v) => Image.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<BigInt> get index => json.containsKey("index")
      ? Presence.present(readInteger(json["index"]))
      : const Presence.absent();
  Presence<Object?> get input => json.containsKey("input")
      ? Presence.present(json["input"])
      : const Presence.absent();
  Presence<List<RankMember>> get members => json.containsKey("members")
      ? Presence.present((json["members"] as List)
          .map((v) => RankMember.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Meta get meta => Meta.read(json["meta"]);
  ReadableQuestion get question => ReadableQuestion.read(json["question"]);
  Presence<String> get questionName => json.containsKey("question_name")
      ? Presence.present(json["question_name"] as String)
      : const Presence.absent();
  String get schema => json["schema"] as String;
  Presence<PhysicalSource> get source => json.containsKey("source")
      ? Presence.present(PhysicalSource.read(json["source"]))
      : const Presence.absent();
  Object? get threshold => json["threshold"] == null ? null : json["threshold"];
  num get value => json["value"] as num;
}

final class Attempt extends NativeObject {
  Attempt._(super.json);
  factory Attempt.read(Object? value) => Attempt._(readObject(value));
  BigInt get ordinal => readInteger(json["ordinal"]);
  String get outcome => json["outcome"] as String;
  Presence<String> get requestId => json.containsKey("request_id")
      ? Presence.present(json["request_id"] as String)
      : const Presence.absent();
  String get requestSha256 => json["request_sha256"] as String;
  String get sdkRequestId => json["sdk_request_id"] as String;
  Presence<BigInt> get serverMs => json.containsKey("server_ms")
      ? Presence.present(readInteger(json["server_ms"]))
      : const Presence.absent();
  Presence<BigInt> get status => json.containsKey("status")
      ? Presence.present(readInteger(json["status"]))
      : const Presence.absent();
  BigInt get wallMs => readInteger(json["wall_ms"]);
}

final class Batch {
  final Object? value;
  const Batch._(this.value);
  factory Batch.read(Object? value) => value is Batch ? value : Batch._(value);
  Object? toJson() => value;
  const Batch.alternative0(BigInt this.value);
  BigInt get asAlternative0 => readInteger(value);
  const Batch.alternative1(String this.value);
  String get asAlternative1 => value as String;
}

final class BoundaryOdds extends NativeObject {
  BoundaryOdds._(super.json);
  factory BoundaryOdds.read(Object? value) => BoundaryOdds._(readObject(value));
  List<PieceOdds> get pieces => (json["pieces"] as List)
      .map((v) => PieceOdds.read(v))
      .toList(growable: false);
  List<BoundaryProposal> get proposals => (json["proposals"] as List)
      .map((v) => BoundaryProposal.read(v))
      .toList(growable: false);
}

final class BoundaryProposal extends NativeObject {
  BoundaryProposal._(super.json);
  factory BoundaryProposal.read(Object? value) =>
      BoundaryProposal._(readObject(value));
  BigInt get end => readInteger(json["end"]);
  BigInt get length => readInteger(json["length"]);
  num get probability => json["probability"] as num;
  BigInt get start => readInteger(json["start"]);
  String get text => json["text"] as String;
}

final class CallError extends NativeObject {
  CallError._(super.json);
  factory CallError.read(Object? value) => CallError._(readObject(value));
  Error get error => Error.read(json["error"]);
  Presence<Facts> get facts => json.containsKey("facts")
      ? Presence.present(Facts.read(json["facts"]))
      : const Presence.absent();
}

final class DecideValue {
  final Object? value;
  const DecideValue._(this.value);
  factory DecideValue.read(Object? value) =>
      value is DecideValue ? value : DecideValue._(value);
  Object? toJson() => value;
  const DecideValue.alternative0(bool? this.value);
  bool? get asAlternative0 => value == null ? null : value as bool;
  const DecideValue.alternative1(Object? this.value);
  Object? get asAlternative1 => value;
}

final class EntityDocument extends NativeObject {
  EntityDocument._(super.json);
  factory EntityDocument.read(Object? value) =>
      EntityDocument._(readObject(value));
  String get kind => json["kind"] as String;
  String get name => json["name"] as String;
}

final class Error extends NativeObject {
  Error._(super.json);
  factory Error.read(Object? value) => Error._(readObject(value));
  Presence<EstimatedInputDenial> get estimatedInputDenial =>
      json.containsKey("estimated_input_denial")
          ? Presence.present(
              EstimatedInputDenial.read(json["estimated_input_denial"]))
          : const Presence.absent();
  String get kind => json["kind"] as String;
  String get message => json["message"] as String;
  bool get retryable => json["retryable"] as bool;
  Presence<SendBudgetDenial> get sendBudgetDenial =>
      json.containsKey("send_budget_denial")
          ? Presence.present(SendBudgetDenial.read(json["send_budget_denial"]))
          : const Presence.absent();
  Stopped get stopped => Stopped.read(json["stopped"]);
}

sealed class EstimatedInputDenial extends NativeObject {
  EstimatedInputDenial(super.json);
  factory EstimatedInputDenial.read(Object? value) {
    final map = readObject(value);
    if (map["kind"] == "initial_request")
      return EstimatedInputDenialInitialRequest.read(map);
    if (map["kind"] == "additional_request")
      return EstimatedInputDenialAdditionalRequest.read(map);
    if (map["kind"] == "retry") return EstimatedInputDenialRetry.read(map);
    throw FormatException("Unknown EstimatedInputDenial alternative");
  }
}

final class EstimatedInputDenialAdditionalRequest extends EstimatedInputDenial {
  EstimatedInputDenialAdditionalRequest._(super.json);
  factory EstimatedInputDenialAdditionalRequest.read(Object? value) =>
      EstimatedInputDenialAdditionalRequest._(readObject(value));
  String get kind => json["kind"] as String;
  BigInt get limit => readInteger(json["limit"]);
}

final class EstimatedInputDenialInitialRequest extends EstimatedInputDenial {
  EstimatedInputDenialInitialRequest._(super.json);
  factory EstimatedInputDenialInitialRequest.read(Object? value) =>
      EstimatedInputDenialInitialRequest._(readObject(value));
  String get kind => json["kind"] as String;
  BigInt get limit => readInteger(json["limit"]);
}

final class EstimatedInputDenialRetry extends EstimatedInputDenial {
  EstimatedInputDenialRetry._(super.json);
  factory EstimatedInputDenialRetry.read(Object? value) =>
      EstimatedInputDenialRetry._(readObject(value));
  String get kind => json["kind"] as String;
  BigInt get lastStatus => readInteger(json["last_status"]);
  BigInt get limit => readInteger(json["limit"]);
}

final class Facts extends NativeObject {
  Facts._(super.json);
  factory Facts.read(Object? value) => Facts._(readObject(value));
  Presence<List<Attempt>> get attempts => json.containsKey("attempts")
      ? Presence.present((json["attempts"] as List)
          .map((v) => Attempt.read(v))
          .toList(growable: false))
      : const Presence.absent();
  BigInt get cacheAnswers => readInteger(json["cache_answers"]);
  String get callId => json["call_id"] as String;
  Presence<String> get estimatedCostUsd =>
      json.containsKey("estimated_cost_usd")
          ? Presence.present(json["estimated_cost_usd"] as String)
          : const Presence.absent();
  Presence<bool> get heldModelMismatch =>
      json.containsKey("held_model_mismatch")
          ? Presence.present(json["held_model_mismatch"] as bool)
          : const Presence.absent();
  Presence<BigInt> get inputTokens => json.containsKey("input_tokens")
      ? Presence.present(readInteger(json["input_tokens"]))
      : const Presence.absent();
  BigInt get largestRequestBytes => readInteger(json["largest_request_bytes"]);
  BigInt? get largestRequestEstimatedInputTokens =>
      json["largest_request_estimated_input_tokens"] == null
          ? null
          : readInteger(json["largest_request_estimated_input_tokens"]);
  Presence<String> get model => json.containsKey("model")
      ? Presence.present(json["model"] as String)
      : const Presence.absent();
  Presence<BigInt> get outputTokens => json.containsKey("output_tokens")
      ? Presence.present(readInteger(json["output_tokens"]))
      : const Presence.absent();
  BigInt get records => readInteger(json["records"]);
  BigInt get requestsSent => readInteger(json["requests_sent"]);
  num get seconds => json["seconds"] as num;
  String get tokenEstimateMethod => json["token_estimate_method"] as String;
  Presence<PersistenceObservation> get usagePersistence => json
          .containsKey("usage_persistence")
      ? Presence.present(PersistenceObservation.read(json["usage_persistence"]))
      : const Presence.absent();
}

final class Find extends NativeObject {
  Find._(super.json);
  factory Find.read(Object? value) => Find._(readObject(value));
  FindAnswer get answer => FindAnswer.read(json["answer"]);
  String get answerId => json["answer_id"] as String;
  Presence<List<FindCandidate>> get candidates => json.containsKey("candidates")
      ? Presence.present((json["candidates"] as List)
          .map((v) => FindCandidate.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<String> get file => json.containsKey("file")
      ? Presence.present(json["file"] as String)
      : const Presence.absent();
  Presence<BigInt> get firstLine => json.containsKey("first_line")
      ? Presence.present(readInteger(json["first_line"]))
      : const Presence.absent();
  BigInt? get index =>
      json["index"] == null ? null : readInteger(json["index"]);
  Presence<BigInt> get lastLine => json.containsKey("last_line")
      ? Presence.present(readInteger(json["last_line"]))
      : const Presence.absent();
  Meta get meta => Meta.read(json["meta"]);
  Presence<Position> get position => json.containsKey("position")
      ? Presence.present(Position.read(json["position"]))
      : const Presence.absent();
  ReadableQuestion2 get question => ReadableQuestion2.read(json["question"]);
  String get schema => json["schema"] as String;
  Object? get threshold => json["threshold"];
  Object? get value => json["value"];
}

final class FindCandidate extends NativeObject {
  FindCandidate._(super.json);
  factory FindCandidate.read(Object? value) =>
      FindCandidate._(readObject(value));
  BigInt? get index =>
      json["index"] == null ? null : readInteger(json["index"]);
  Object? get input => json["input"];
  num get probability => json["probability"] as num;
  Presence<PhysicalSource> get source => json.containsKey("source")
      ? Presence.present(PhysicalSource.read(json["source"]))
      : const Presence.absent();
}

final class Image extends NativeObject {
  Image._(super.json);
  factory Image.read(Object? value) => Image._(readObject(value));
  String get base64 => json["base64"] as String;
  BigInt get height => readInteger(json["height"]);
  String get media => json["media"] as String;
  BigInt get width => readInteger(json["width"]);
}

sealed class InputDeclaration extends NativeObject {
  InputDeclaration(super.json);
  factory InputDeclaration.read(Object? value) {
    final map = readObject(value);
    if (map["type"] == "string") return InputDeclarationString.read(map);
    if (map["type"] == "object") return InputDeclarationObject.read(map);
    throw FormatException("Unknown InputDeclaration alternative");
  }
}

final class InputDeclarationObject extends InputDeclaration {
  InputDeclarationObject._(super.json);
  factory InputDeclarationObject.read(Object? value) =>
      InputDeclarationObject._(readObject(value));
  Map<String, InputPropertyType> get properties =>
      (json["properties"] as Map<String, Object?>)
          .map((k, v) => MapEntry(k, InputPropertyType.read(v)));
  Presence<List<String>> get required => json.containsKey("required")
      ? Presence.present((json["required"] as List)
          .map((v) => v as String)
          .toList(growable: false))
      : const Presence.absent();
  String get type => json["type"] as String;
}

final class InputDeclarationString extends InputDeclaration {
  InputDeclarationString._(super.json);
  factory InputDeclarationString.read(Object? value) =>
      InputDeclarationString._(readObject(value));
  String get type => json["type"] as String;
}

sealed class InputPropertyType extends NativeObject {
  InputPropertyType(super.json);
  factory InputPropertyType.read(Object? value) {
    final map = readObject(value);
    if (map["type"] == "string") return InputPropertyTypeString.read(map);
    if (map["type"] == "number") return InputPropertyTypeNumber.read(map);
    if (map["type"] == "boolean") return InputPropertyTypeBoolean.read(map);
    if (map["type"] == "array") return InputPropertyTypeArray.read(map);
    throw FormatException("Unknown InputPropertyType alternative");
  }
}

final class InputPropertyTypeArray extends InputPropertyType {
  InputPropertyTypeArray._(super.json);
  factory InputPropertyTypeArray.read(Object? value) =>
      InputPropertyTypeArray._(readObject(value));
  StringRoot get items => StringRoot.read(json["items"]);
  String get type => json["type"] as String;
}

final class InputPropertyTypeBoolean extends InputPropertyType {
  InputPropertyTypeBoolean._(super.json);
  factory InputPropertyTypeBoolean.read(Object? value) =>
      InputPropertyTypeBoolean._(readObject(value));
  String get type => json["type"] as String;
}

final class InputPropertyTypeNumber extends InputPropertyType {
  InputPropertyTypeNumber._(super.json);
  factory InputPropertyTypeNumber.read(Object? value) =>
      InputPropertyTypeNumber._(readObject(value));
  String get type => json["type"] as String;
}

final class InputPropertyTypeString extends InputPropertyType {
  InputPropertyTypeString._(super.json);
  factory InputPropertyTypeString.read(Object? value) =>
      InputPropertyTypeString._(readObject(value));
  String get type => json["type"] as String;
}

final class Label extends NativeObject {
  Label._(super.json);
  factory Label.read(Object? value) => Label._(readObject(value));
  Presence<Object?> get description => json.containsKey("description")
      ? Presence.present(json["description"])
      : const Presence.absent();
  String get name => json["name"] as String;
}

final class Meta extends NativeObject {
  Meta._(super.json);
  factory Meta.read(Object? value) => Meta._(readObject(value));
  Presence<String> get answeredBy => json.containsKey("answered_by")
      ? Presence.present(json["answered_by"] as String)
      : const Presence.absent();
  Presence<List<Attempt>> get attempts => json.containsKey("attempts")
      ? Presence.present((json["attempts"] as List)
          .map((v) => Attempt.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<BatchSetting> get batchSetting => json.containsKey("batch_setting")
      ? Presence.present(BatchSetting.read(json["batch_setting"]))
      : const Presence.absent();
  Presence<BatchWarning> get batchWarning => json.containsKey("batch_warning")
      ? Presence.present(BatchWarning.read(json["batch_warning"]))
      : const Presence.absent();
  bool get cached => json["cached"] as bool;
  Presence<String> get contextSha256 => json.containsKey("context_sha256")
      ? Presence.present(json["context_sha256"] as String)
      : const Presence.absent();
  BigInt get failedQuestions => readInteger(json["failed_questions"]);
  String get model => json["model"] as String;
  List<Observation> get observations => (json["observations"] as List)
      .map((v) => Observation.read(v))
      .toList(growable: false);
  String? get origin =>
      json["origin"] == null ? null : json["origin"] as String;
  Presence<ProfileWarning> get profileWarning =>
      json.containsKey("profile_warning")
          ? Presence.present(ProfileWarning.read(json["profile_warning"]))
          : const Presence.absent();
  Presence<String> get questionSha256 => json.containsKey("question_sha256")
      ? Presence.present(json["question_sha256"] as String)
      : const Presence.absent();
  List<QuestionSource> get questionSources => (json["question_sources"] as List)
      .map((v) => QuestionSource.read(v))
      .toList(growable: false);
  Presence<String> get questionsSha256 => json.containsKey("questions_sha256")
      ? Presence.present(json["questions_sha256"] as String)
      : const Presence.absent();
  List<String> get requests => (json["requests"] as List)
      .map((v) => v as String)
      .toList(growable: false);
  BigInt get requestsSent => readInteger(json["requests_sent"]);
  String get tool => json["tool"] as String;
  String get url => json["url"] as String;
  Presence<Usage> get usage => json.containsKey("usage")
      ? Presence.present(Usage.read(json["usage"]))
      : const Presence.absent();
}

final class ObjectRoot extends NativeObject {
  ObjectRoot._(super.json);
  factory ObjectRoot.read(Object? value) => ObjectRoot._(readObject(value));
  Map<String, InputPropertyType> get properties =>
      (json["properties"] as Map<String, Object?>)
          .map((k, v) => MapEntry(k, InputPropertyType.read(v)));
  Presence<List<String>> get required => json.containsKey("required")
      ? Presence.present((json["required"] as List)
          .map((v) => v as String)
          .toList(growable: false))
      : const Presence.absent();
  String get type => json["type"] as String;
}

sealed class Observation extends NativeObject {
  Observation(super.json);
  factory Observation.read(Object? value) {
    final map = readObject(value);
    if (map.containsKey("observation_id"))
      return ObservationObservationId.read(map);
    if (map.containsKey("failure_id")) return ObservationFailureId.read(map);
    throw FormatException("Unknown Observation alternative");
  }
}

final class ObservationFailureId extends Observation {
  ObservationFailureId._(super.json);
  factory ObservationFailureId.read(Object? value) =>
      ObservationFailureId._(readObject(value));
  String get failureId => json["failure_id"] as String;
}

final class ObservationObservationId extends Observation {
  ObservationObservationId._(super.json);
  factory ObservationObservationId.read(Object? value) =>
      ObservationObservationId._(readObject(value));
  String get observationId => json["observation_id"] as String;
}

final class PersistenceObservation extends NativeObject {
  PersistenceObservation._(super.json);
  factory PersistenceObservation.read(Object? value) =>
      PersistenceObservation._(readObject(value));
  Presence<String> get advice => json.containsKey("advice")
      ? Presence.present(json["advice"] as String)
      : const Presence.absent();
  String get observedAt => json["observed_at"] as String;
  String get state => json["state"] as String;
}

final class PhysicalSource extends NativeObject {
  PhysicalSource._(super.json);
  factory PhysicalSource.read(Object? value) =>
      PhysicalSource._(readObject(value));
  String get file => json["file"] as String;
  Presence<BigInt> get firstLine => json.containsKey("first_line")
      ? Presence.present(readInteger(json["first_line"]))
      : const Presence.absent();
  Presence<BigInt> get lastLine => json.containsKey("last_line")
      ? Presence.present(readInteger(json["last_line"]))
      : const Presence.absent();
}

final class Position extends NativeObject {
  Position._(super.json);
  factory Position.read(Object? value) => Position._(readObject(value));
  String? get file => json["file"] == null ? null : json["file"] as String;
  Presence<BigInt> get first => json.containsKey("first")
      ? Presence.present(readInteger(json["first"]))
      : const Presence.absent();
  Presence<List<String>> get images => json.containsKey("images")
      ? Presence.present((json["images"] as List)
          .map((v) => v as String)
          .toList(growable: false))
      : const Presence.absent();
  Presence<BigInt> get last => json.containsKey("last")
      ? Presence.present(readInteger(json["last"]))
      : const Presence.absent();
}

final class QuestionSource extends NativeObject {
  QuestionSource._(super.json);
  factory QuestionSource.read(Object? value) =>
      QuestionSource._(readObject(value));
  String get answeredBy => json["answered_by"] as String;
  Presence<BigInt> get batchSize => json.containsKey("batch_size")
      ? Presence.present(readInteger(json["batch_size"]))
      : const Presence.absent();
  String get origin => json["origin"] as String;
}

final class RankMember extends NativeObject {
  RankMember._(super.json);
  factory RankMember.read(Object? value) => RankMember._(readObject(value));
  String get name => json["name"] as String;
  RankMemberResult get result => RankMemberResult.read(json["result"]);
}

final class RankMemberResult extends NativeObject {
  RankMemberResult._(super.json);
  factory RankMemberResult.read(Object? value) =>
      RankMemberResult._(readObject(value));
  Answer get answer => Answer.read(json["answer"]);
  String get answerId => json["answer_id"] as String;
  Presence<List<Image>> get images => json.containsKey("images")
      ? Presence.present((json["images"] as List)
          .map((v) => Image.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Meta get meta => Meta.read(json["meta"]);
  ReadableQuestion get question => ReadableQuestion.read(json["question"]);
  String get schema => json["schema"] as String;
  Presence<PhysicalSource> get source => json.containsKey("source")
      ? Presence.present(PhysicalSource.read(json["source"]))
      : const Presence.absent();
  Object? get threshold => json["threshold"];
  BigInt get value => readInteger(json["value"]);
}

sealed class ReadableQuestion extends NativeObject {
  ReadableQuestion(super.json);
  factory ReadableQuestion.read(Object? value) {
    final map = readObject(value);
    if (map["verb"] == "decide") return ReadableQuestionDecide.read(map);
    if (map["verb"] == "choose") return ReadableQuestionChoose.read(map);
    if (map["verb"] == "tag") return ReadableQuestionTag.read(map);
    if (map["verb"] == "score") return ReadableQuestionScore.read(map);
    throw FormatException("Unknown ReadableQuestion alternative");
  }
}

final class ReadableQuestion2 extends NativeObject {
  ReadableQuestion2._(super.json);
  factory ReadableQuestion2.read(Object? value) =>
      ReadableQuestion2._(readObject(value));
  Presence<Batch> get batch => json.containsKey("batch")
      ? Presence.present(Batch.read(json["batch"]))
      : const Presence.absent();
  Presence<InputDeclaration> get contextSchema =>
      json.containsKey("context_schema")
          ? Presence.present(InputDeclaration.read(json["context_schema"]))
          : const Presence.absent();
  Presence<InputDeclaration> get itemSchema => json.containsKey("item_schema")
      ? Presence.present(InputDeclaration.read(json["item_schema"]))
      : const Presence.absent();
  Presence<List<Label>> get labelDetails => json.containsKey("label_details")
      ? Presence.present((json["label_details"] as List)
          .map((v) => Label.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<String> get model => json.containsKey("model")
      ? Presence.present(json["model"] as String)
      : const Presence.absent();
  Presence<String> get name => json.containsKey("name")
      ? Presence.present(json["name"] as String)
      : const Presence.absent();
  bool get none => json["none"] as bool;
  Presence<List<String>> get on => json.containsKey("on")
      ? Presence.present(
          (json["on"] as List).map((v) => v as String).toList(growable: false))
      : const Presence.absent();
  Presence<String> get profile => json.containsKey("profile")
      ? Presence.present(json["profile"] as String)
      : const Presence.absent();
  Object? get text => json["text"];
  String get verb => json["verb"] as String;
  Presence<BigInt> get wordingVersion => json.containsKey("wording_version")
      ? Presence.present(readInteger(json["wording_version"]))
      : const Presence.absent();
}

final class ReadableQuestion3 extends NativeObject {
  ReadableQuestion3._(super.json);
  factory ReadableQuestion3.read(Object? value) =>
      ReadableQuestion3._(readObject(value));
  Presence<Batch> get batch => json.containsKey("batch")
      ? Presence.present(Batch.read(json["batch"]))
      : const Presence.absent();
  Presence<InputDeclaration> get contextSchema =>
      json.containsKey("context_schema")
          ? Presence.present(InputDeclaration.read(json["context_schema"]))
          : const Presence.absent();
  Presence<Object?> get entityDefinition =>
      json.containsKey("entity_definition")
          ? Presence.present(json["entity_definition"])
          : const Presence.absent();
  Presence<Object?> get instructions => json.containsKey("instructions")
      ? Presence.present(json["instructions"])
      : const Presence.absent();
  Presence<InputDeclaration> get itemSchema => json.containsKey("item_schema")
      ? Presence.present(InputDeclaration.read(json["item_schema"]))
      : const Presence.absent();
  Map<String, Object?> get kinds =>
      (json["kinds"] as Map<String, Object?>).map((k, v) => MapEntry(k, v));
  Presence<List<Label>> get labelDetails => json.containsKey("label_details")
      ? Presence.present((json["label_details"] as List)
          .map((v) => Label.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<String> get mode => json.containsKey("mode")
      ? Presence.present(json["mode"] as String)
      : const Presence.absent();
  Presence<String> get model => json.containsKey("model")
      ? Presence.present(json["model"] as String)
      : const Presence.absent();
  Presence<String> get name => json.containsKey("name")
      ? Presence.present(json["name"] as String)
      : const Presence.absent();
  Presence<List<String>> get on => json.containsKey("on")
      ? Presence.present(
          (json["on"] as List).map((v) => v as String).toList(growable: false))
      : const Presence.absent();
  Presence<String> get profile => json.containsKey("profile")
      ? Presence.present(json["profile"] as String)
      : const Presence.absent();
  Presence<Object?> get relationThreshold =>
      json.containsKey("relation_threshold")
          ? Presence.present(json["relation_threshold"])
          : const Presence.absent();
  Presence<List<RelationRule>> get relations => json.containsKey("relations")
      ? Presence.present((json["relations"] as List)
          .map((v) => RelationRule.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<BigInt> get snippetPieces => json.containsKey("snippet_pieces")
      ? Presence.present(readInteger(json["snippet_pieces"]))
      : const Presence.absent();
  Presence<RecognitionStageContext> get stageContext => json
          .containsKey("stage_context")
      ? Presence.present(RecognitionStageContext.read(json["stage_context"]))
      : const Presence.absent();
  Object? get threshold => json["threshold"];
  String get verb => json["verb"] as String;
  Presence<BigInt> get wordingVersion => json.containsKey("wording_version")
      ? Presence.present(readInteger(json["wording_version"]))
      : const Presence.absent();
}

final class ReadableQuestion4 extends NativeObject {
  ReadableQuestion4._(super.json);
  factory ReadableQuestion4.read(Object? value) =>
      ReadableQuestion4._(readObject(value));
  Presence<Batch> get batch => json.containsKey("batch")
      ? Presence.present(Batch.read(json["batch"]))
      : const Presence.absent();
  Presence<InputDeclaration> get contextSchema =>
      json.containsKey("context_schema")
          ? Presence.present(InputDeclaration.read(json["context_schema"]))
          : const Presence.absent();
  RelateFields? get fields =>
      json["fields"] == null ? null : RelateFields.read(json["fields"]);
  Presence<InputDeclaration> get itemSchema => json.containsKey("item_schema")
      ? Presence.present(InputDeclaration.read(json["item_schema"]))
      : const Presence.absent();
  Presence<List<Label>> get labelDetails => json.containsKey("label_details")
      ? Presence.present((json["label_details"] as List)
          .map((v) => Label.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<String> get model => json.containsKey("model")
      ? Presence.present(json["model"] as String)
      : const Presence.absent();
  Presence<String> get name => json.containsKey("name")
      ? Presence.present(json["name"] as String)
      : const Presence.absent();
  Presence<List<String>> get on => json.containsKey("on")
      ? Presence.present(
          (json["on"] as List).map((v) => v as String).toList(growable: false))
      : const Presence.absent();
  Presence<String> get profile => json.containsKey("profile")
      ? Presence.present(json["profile"] as String)
      : const Presence.absent();
  List<RelationRule> get relations => (json["relations"] as List)
      .map((v) => RelationRule.read(v))
      .toList(growable: false);
  Object? get threshold => json["threshold"];
  String get verb => json["verb"] as String;
  Presence<BigInt> get wordingVersion => json.containsKey("wording_version")
      ? Presence.present(readInteger(json["wording_version"]))
      : const Presence.absent();
}

final class ReadableQuestionChoose extends ReadableQuestion {
  ReadableQuestionChoose._(super.json);
  factory ReadableQuestionChoose.read(Object? value) =>
      ReadableQuestionChoose._(readObject(value));
  Presence<Batch> get batch => json.containsKey("batch")
      ? Presence.present(Batch.read(json["batch"]))
      : const Presence.absent();
  Presence<InputDeclaration> get contextSchema =>
      json.containsKey("context_schema")
          ? Presence.present(InputDeclaration.read(json["context_schema"]))
          : const Presence.absent();
  Presence<InputDeclaration> get itemSchema => json.containsKey("item_schema")
      ? Presence.present(InputDeclaration.read(json["item_schema"]))
      : const Presence.absent();
  Presence<List<Label>> get labelDetails => json.containsKey("label_details")
      ? Presence.present((json["label_details"] as List)
          .map((v) => Label.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<String> get model => json.containsKey("model")
      ? Presence.present(json["model"] as String)
      : const Presence.absent();
  Presence<String> get name => json.containsKey("name")
      ? Presence.present(json["name"] as String)
      : const Presence.absent();
  Presence<List<String>> get on => json.containsKey("on")
      ? Presence.present(
          (json["on"] as List).map((v) => v as String).toList(growable: false))
      : const Presence.absent();
  Presence<String> get profile => json.containsKey("profile")
      ? Presence.present(json["profile"] as String)
      : const Presence.absent();
  Presence<BigInt> get wordingVersion => json.containsKey("wording_version")
      ? Presence.present(readInteger(json["wording_version"]))
      : const Presence.absent();
  List<String> get options =>
      (json["options"] as List).map((v) => v as String).toList(growable: false);
  Object? get text => json["text"];
  String get verb => json["verb"] as String;
}

final class ReadableQuestionDecide extends ReadableQuestion {
  ReadableQuestionDecide._(super.json);
  factory ReadableQuestionDecide.read(Object? value) =>
      ReadableQuestionDecide._(readObject(value));
  Presence<Batch> get batch => json.containsKey("batch")
      ? Presence.present(Batch.read(json["batch"]))
      : const Presence.absent();
  Presence<InputDeclaration> get contextSchema =>
      json.containsKey("context_schema")
          ? Presence.present(InputDeclaration.read(json["context_schema"]))
          : const Presence.absent();
  Presence<InputDeclaration> get itemSchema => json.containsKey("item_schema")
      ? Presence.present(InputDeclaration.read(json["item_schema"]))
      : const Presence.absent();
  Presence<List<Label>> get labelDetails => json.containsKey("label_details")
      ? Presence.present((json["label_details"] as List)
          .map((v) => Label.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<String> get model => json.containsKey("model")
      ? Presence.present(json["model"] as String)
      : const Presence.absent();
  Presence<String> get name => json.containsKey("name")
      ? Presence.present(json["name"] as String)
      : const Presence.absent();
  Presence<List<String>> get on => json.containsKey("on")
      ? Presence.present(
          (json["on"] as List).map((v) => v as String).toList(growable: false))
      : const Presence.absent();
  Presence<String> get profile => json.containsKey("profile")
      ? Presence.present(json["profile"] as String)
      : const Presence.absent();
  Presence<BigInt> get wordingVersion => json.containsKey("wording_version")
      ? Presence.present(readInteger(json["wording_version"]))
      : const Presence.absent();
  Presence<Object?> get falseValue => json.containsKey("false")
      ? Presence.present(json["false"])
      : const Presence.absent();
  Object? get text => json["text"];
  Presence<Object?> get trueValue => json.containsKey("true")
      ? Presence.present(json["true"])
      : const Presence.absent();
  String get verb => json["verb"] as String;
}

final class ReadableQuestionScore extends ReadableQuestion {
  ReadableQuestionScore._(super.json);
  factory ReadableQuestionScore.read(Object? value) =>
      ReadableQuestionScore._(readObject(value));
  Presence<Batch> get batch => json.containsKey("batch")
      ? Presence.present(Batch.read(json["batch"]))
      : const Presence.absent();
  Presence<InputDeclaration> get contextSchema =>
      json.containsKey("context_schema")
          ? Presence.present(InputDeclaration.read(json["context_schema"]))
          : const Presence.absent();
  Presence<InputDeclaration> get itemSchema => json.containsKey("item_schema")
      ? Presence.present(InputDeclaration.read(json["item_schema"]))
      : const Presence.absent();
  Presence<List<Label>> get labelDetails => json.containsKey("label_details")
      ? Presence.present((json["label_details"] as List)
          .map((v) => Label.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<String> get model => json.containsKey("model")
      ? Presence.present(json["model"] as String)
      : const Presence.absent();
  Presence<String> get name => json.containsKey("name")
      ? Presence.present(json["name"] as String)
      : const Presence.absent();
  Presence<List<String>> get on => json.containsKey("on")
      ? Presence.present(
          (json["on"] as List).map((v) => v as String).toList(growable: false))
      : const Presence.absent();
  Presence<String> get profile => json.containsKey("profile")
      ? Presence.present(json["profile"] as String)
      : const Presence.absent();
  Presence<BigInt> get wordingVersion => json.containsKey("wording_version")
      ? Presence.present(readInteger(json["wording_version"]))
      : const Presence.absent();
  List<String> get levels =>
      (json["levels"] as List).map((v) => v as String).toList(growable: false);
  Object? get text => json["text"];
  String get verb => json["verb"] as String;
}

final class ReadableQuestionTag extends ReadableQuestion {
  ReadableQuestionTag._(super.json);
  factory ReadableQuestionTag.read(Object? value) =>
      ReadableQuestionTag._(readObject(value));
  Presence<Batch> get batch => json.containsKey("batch")
      ? Presence.present(Batch.read(json["batch"]))
      : const Presence.absent();
  Presence<InputDeclaration> get contextSchema =>
      json.containsKey("context_schema")
          ? Presence.present(InputDeclaration.read(json["context_schema"]))
          : const Presence.absent();
  Presence<InputDeclaration> get itemSchema => json.containsKey("item_schema")
      ? Presence.present(InputDeclaration.read(json["item_schema"]))
      : const Presence.absent();
  Presence<List<Label>> get labelDetails => json.containsKey("label_details")
      ? Presence.present((json["label_details"] as List)
          .map((v) => Label.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<String> get model => json.containsKey("model")
      ? Presence.present(json["model"] as String)
      : const Presence.absent();
  Presence<String> get name => json.containsKey("name")
      ? Presence.present(json["name"] as String)
      : const Presence.absent();
  Presence<List<String>> get on => json.containsKey("on")
      ? Presence.present(
          (json["on"] as List).map((v) => v as String).toList(growable: false))
      : const Presence.absent();
  Presence<String> get profile => json.containsKey("profile")
      ? Presence.present(json["profile"] as String)
      : const Presence.absent();
  Presence<BigInt> get wordingVersion => json.containsKey("wording_version")
      ? Presence.present(readInteger(json["wording_version"]))
      : const Presence.absent();
  List<String> get labels =>
      (json["labels"] as List).map((v) => v as String).toList(growable: false);
  Object? get text => json["text"];
  String get verb => json["verb"] as String;
}

final class Recognition extends NativeObject {
  Recognition._(super.json);
  factory Recognition.read(Object? value) => Recognition._(readObject(value));
  RecognitionOdds get answer => RecognitionOdds.read(json["answer"]);
  String get answerId => json["answer_id"] as String;
  Presence<String> get file => json.containsKey("file")
      ? Presence.present(json["file"] as String)
      : const Presence.absent();
  Presence<BigInt> get firstLine => json.containsKey("first_line")
      ? Presence.present(readInteger(json["first_line"]))
      : const Presence.absent();
  Presence<BigInt> get index => json.containsKey("index")
      ? Presence.present(readInteger(json["index"]))
      : const Presence.absent();
  Presence<Object?> get input => json.containsKey("input")
      ? Presence.present(json["input"])
      : const Presence.absent();
  Presence<BigInt> get lastLine => json.containsKey("last_line")
      ? Presence.present(readInteger(json["last_line"]))
      : const Presence.absent();
  Meta get meta => Meta.read(json["meta"]);
  Presence<Position> get position => json.containsKey("position")
      ? Presence.present(Position.read(json["position"]))
      : const Presence.absent();
  ReadableQuestion3 get question => ReadableQuestion3.read(json["question"]);
  String get schema => json["schema"] as String;
  Presence<PhysicalSource> get source => json.containsKey("source")
      ? Presence.present(PhysicalSource.read(json["source"]))
      : const Presence.absent();
  Recognize get value => Recognize.read(json["value"]);
}

final class RecognitionEdgeDocument extends NativeObject {
  RecognitionEdgeDocument._(super.json);
  factory RecognitionEdgeDocument.read(Object? value) =>
      RecognitionEdgeDocument._(readObject(value));
  bool get either => json["either"] as bool;
  num get probability => json["probability"] as num;
  String get relation => json["relation"] as String;
  Entity get source => Entity.read(json["source"]);
  Entity get target => Entity.read(json["target"]);
}

sealed class RecognitionOdds extends NativeObject {
  RecognitionOdds(super.json);
  factory RecognitionOdds.read(Object? value) {
    final map = readObject(value);
    if (map.containsKey("names") &&
        map.containsKey("pairs") &&
        map.containsKey("pieces") &&
        map.containsKey("proposals"))
      return RecognitionOddsFieldsNamesPairsPiecesProposals.read(map);
    if (map.containsKey("pieces") &&
        map.containsKey("proposals") &&
        !map.containsKey("names") &&
        !map.containsKey("pairs"))
      return RecognitionOddsFieldsPiecesProposals.read(map);
    throw FormatException("Unknown RecognitionOdds alternative");
  }
}

final class RecognitionOddsFieldsNamesPairsPiecesProposals
    extends RecognitionOdds {
  RecognitionOddsFieldsNamesPairsPiecesProposals._(super.json);
  factory RecognitionOddsFieldsNamesPairsPiecesProposals.read(Object? value) =>
      RecognitionOddsFieldsNamesPairsPiecesProposals._(readObject(value));
  List<NameOdds> get names => (json["names"] as List)
      .map((v) => NameOdds.read(v))
      .toList(growable: false);
  List<PairOdds> get pairs => (json["pairs"] as List)
      .map((v) => PairOdds.read(v))
      .toList(growable: false);
  List<PieceOdds> get pieces => (json["pieces"] as List)
      .map((v) => PieceOdds.read(v))
      .toList(growable: false);
  List<RecognitionProposal> get proposals => (json["proposals"] as List)
      .map((v) => RecognitionProposal.read(v))
      .toList(growable: false);
}

final class RecognitionOddsFieldsPiecesProposals extends RecognitionOdds {
  RecognitionOddsFieldsPiecesProposals._(super.json);
  factory RecognitionOddsFieldsPiecesProposals.read(Object? value) =>
      RecognitionOddsFieldsPiecesProposals._(readObject(value));
  List<PieceOdds> get pieces => (json["pieces"] as List)
      .map((v) => PieceOdds.read(v))
      .toList(growable: false);
  List<BoundaryProposal> get proposals => (json["proposals"] as List)
      .map((v) => BoundaryProposal.read(v))
      .toList(growable: false);
}

final class RecognitionProposal extends NativeObject {
  RecognitionProposal._(super.json);
  factory RecognitionProposal.read(Object? value) =>
      RecognitionProposal._(readObject(value));
  BigInt get end => readInteger(json["end"]);
  bool get kept => json["kept"] as bool;
  Presence<String> get kind => json.containsKey("kind")
      ? Presence.present(json["kind"] as String)
      : const Presence.absent();
  Presence<Place> get selected => json.containsKey("selected")
      ? Presence.present(Place.read(json["selected"]))
      : const Presence.absent();
  num get spanProbability => json["span_probability"] as num;
  BigInt get start => readInteger(json["start"]);
  Presence<num> get strength => json.containsKey("strength")
      ? Presence.present(json["strength"] as num)
      : const Presence.absent();
}

final class RecognitionStageContext extends NativeObject {
  RecognitionStageContext._(super.json);
  factory RecognitionStageContext.read(Object? value) =>
      RecognitionStageContext._(readObject(value));
  Presence<String> get boundary => json.containsKey("boundary")
      ? Presence.present(json["boundary"] as String)
      : const Presence.absent();
  Presence<String> get kindEdge => json.containsKey("kind_edge")
      ? Presence.present(json["kind_edge"] as String)
      : const Presence.absent();
  Presence<String> get relation => json.containsKey("relation")
      ? Presence.present(json["relation"] as String)
      : const Presence.absent();
}

final class Relation extends NativeObject {
  Relation._(super.json);
  factory Relation.read(Object? value) => Relation._(readObject(value));
  Answers get answer => Answers.read(json["answer"]);
  String get answerId => json["answer_id"] as String;
  Presence<String> get file => json.containsKey("file")
      ? Presence.present(json["file"] as String)
      : const Presence.absent();
  Presence<BigInt> get firstLine => json.containsKey("first_line")
      ? Presence.present(readInteger(json["first_line"]))
      : const Presence.absent();
  Presence<BigInt> get index => json.containsKey("index")
      ? Presence.present(readInteger(json["index"]))
      : const Presence.absent();
  Presence<Object?> get input => json.containsKey("input")
      ? Presence.present(json["input"])
      : const Presence.absent();
  Presence<List<SessionInputSource>> get inputSources =>
      json.containsKey("input_sources")
          ? Presence.present((json["input_sources"] as List)
              .map((v) => SessionInputSource.read(v))
              .toList(growable: false))
          : const Presence.absent();
  Presence<BigInt> get lastLine => json.containsKey("last_line")
      ? Presence.present(readInteger(json["last_line"]))
      : const Presence.absent();
  Meta get meta => Meta.read(json["meta"]);
  Presence<Position> get position => json.containsKey("position")
      ? Presence.present(Position.read(json["position"]))
      : const Presence.absent();
  ReadableQuestion4 get question => ReadableQuestion4.read(json["question"]);
  String get schema => json["schema"] as String;
  List<RelatedEntityEdge> get value => (json["value"] as List)
      .map((v) => RelatedEntityEdge.read(v))
      .toList(growable: false);
}

sealed class RelationMember extends NativeObject {
  RelationMember(super.json);
  factory RelationMember.read(Object? value) {
    final map = readObject(value);
    if (map.containsKey("answer_id")) return RelationMemberAnswerId.read(map);
    if (map.containsKey("failure_id")) return RelationMemberFailureId.read(map);
    throw FormatException("Unknown RelationMember alternative");
  }
}

final class RelationMemberAnswerId extends RelationMember {
  RelationMemberAnswerId._(super.json);
  factory RelationMemberAnswerId.read(Object? value) =>
      RelationMemberAnswerId._(readObject(value));
  String get direction => json["direction"] as String;
  String get method => json["method"] as String;
  List<Observation> get observations => (json["observations"] as List)
      .map((v) => Observation.read(v))
      .toList(growable: false);
  ReadableQuestion get question => ReadableQuestion.read(json["question"]);
  List<QuestionSource> get questionSources => (json["question_sources"] as List)
      .map((v) => QuestionSource.read(v))
      .toList(growable: false);
  String get reads => json["reads"] as String;
  String get relation => json["relation"] as String;
  String get request => json["request"] as String;
  RelatedEntity get source => RelatedEntity.read(json["source"]);
  RelatedEntity? get target =>
      json["target"] == null ? null : RelatedEntity.read(json["target"]);
  Object? get threshold => json["threshold"];
  Presence<Usage> get usage => json.containsKey("usage")
      ? Presence.present(Usage.read(json["usage"]))
      : const Presence.absent();
  bool get accepted => json["accepted"] as bool;
  Answer get answer => Answer.read(json["answer"]);
  String get answerId => json["answer_id"] as String;
  num get probability => json["probability"] as num;
}

final class RelationMemberFailureId extends RelationMember {
  RelationMemberFailureId._(super.json);
  factory RelationMemberFailureId.read(Object? value) =>
      RelationMemberFailureId._(readObject(value));
  String get direction => json["direction"] as String;
  String get method => json["method"] as String;
  List<Observation> get observations => (json["observations"] as List)
      .map((v) => Observation.read(v))
      .toList(growable: false);
  ReadableQuestion get question => ReadableQuestion.read(json["question"]);
  List<QuestionSource> get questionSources => (json["question_sources"] as List)
      .map((v) => QuestionSource.read(v))
      .toList(growable: false);
  String get reads => json["reads"] as String;
  String get relation => json["relation"] as String;
  String get request => json["request"] as String;
  RelatedEntity get source => RelatedEntity.read(json["source"]);
  RelatedEntity? get target =>
      json["target"] == null ? null : RelatedEntity.read(json["target"]);
  Object? get threshold => json["threshold"];
  Presence<Usage> get usage => json.containsKey("usage")
      ? Presence.present(Usage.read(json["usage"]))
      : const Presence.absent();
  Failure get failure => Failure.read(json["failure"]);
  String get failureId => json["failure_id"] as String;
}

sealed class SendBudgetDenial extends NativeObject {
  SendBudgetDenial(super.json);
  factory SendBudgetDenial.read(Object? value) {
    final map = readObject(value);
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
  SendBudgetDenialBeforeAdditionalSend._(super.json);
  factory SendBudgetDenialBeforeAdditionalSend.read(Object? value) =>
      SendBudgetDenialBeforeAdditionalSend._(readObject(value));
  String get kind => json["kind"] as String;
}

final class SendBudgetDenialBeforeFirstSend extends SendBudgetDenial {
  SendBudgetDenialBeforeFirstSend._(super.json);
  factory SendBudgetDenialBeforeFirstSend.read(Object? value) =>
      SendBudgetDenialBeforeFirstSend._(readObject(value));
  String get kind => json["kind"] as String;
}

final class SendBudgetDenialBeforeRetry extends SendBudgetDenial {
  SendBudgetDenialBeforeRetry._(super.json);
  factory SendBudgetDenialBeforeRetry.read(Object? value) =>
      SendBudgetDenialBeforeRetry._(readObject(value));
  String get kind => json["kind"] as String;
  BigInt get lastStatus => readInteger(json["last_status"]);
}

final class Stopped extends NativeObject {
  Stopped._(super.json);
  factory Stopped.read(Object? value) => Stopped._(readObject(value));
  Presence<BigInt> get at => json.containsKey("at")
      ? Presence.present(readInteger(json["at"]))
      : const Presence.absent();
  String get cause => json["cause"] as String;
  bool get retryable => json["retryable"] as bool;
  Presence<BigInt> get status => json.containsKey("status")
      ? Presence.present(readInteger(json["status"]))
      : const Presence.absent();
}

final class StringRoot extends NativeObject {
  StringRoot._(super.json);
  factory StringRoot.read(Object? value) => StringRoot._(readObject(value));
  String get type => json["type"] as String;
}

final class Usage extends NativeObject {
  Usage._(super.json);
  factory Usage.read(Object? value) => Usage._(readObject(value));
  Presence<BigInt> get inputTokens => json.containsKey("input_tokens")
      ? Presence.present(readInteger(json["input_tokens"]))
      : const Presence.absent();
  Presence<BigInt> get outputTokens => json.containsKey("output_tokens")
      ? Presence.present(readInteger(json["output_tokens"]))
      : const Presence.absent();
}

final class AnnotatedField {
  final Object? value;
  const AnnotatedField(this.value);
  factory AnnotatedField.read(Object? value) =>
      value is AnnotatedField ? value : AnnotatedField(value);
  Object? toJson() => value;
  bool get asBoolean => value as bool;
  String get asString => value as String;
  List<String> get asArray =>
      (value as List).map((v) => v as String).toList(growable: false);
  num get asNumber => value as num;
  Failed get asObject => Failed.read(value);
}

sealed class Answer extends NativeObject {
  Answer(super.json);
  factory Answer.read(Object? value) {
    final map = readObject(value);
    if (map["kind"] == "yes_no") return AnswerYesNo.read(map);
    if (map["kind"] == "choice") return AnswerChoice.read(map);
    if (map["kind"] == "tag") return AnswerTag.read(map);
    if (map["kind"] == "score") return AnswerScore.read(map);
    throw FormatException("Unknown Answer alternative");
  }
}

final class AnswerChoice extends Answer {
  AnswerChoice._(super.json);
  factory AnswerChoice.read(Object? value) => AnswerChoice._(readObject(value));
  Presence<num> get confidence => json.containsKey("confidence")
      ? Presence.present(json["confidence"] as num)
      : const Presence.absent();
  String get kind => json["kind"] as String;
  String get pick => json["pick"] as String;
  Map<String, num> get probabilities =>
      (json["probabilities"] as Map<String, Object?>)
          .map((k, v) => MapEntry(k, v as num));
}

final class AnswerScore extends Answer {
  AnswerScore._(super.json);
  factory AnswerScore.read(Object? value) => AnswerScore._(readObject(value));
  Presence<num> get confidence => json.containsKey("confidence")
      ? Presence.present(json["confidence"] as num)
      : const Presence.absent();
  String get kind => json["kind"] as String;
  String get level => json["level"] as String;
  Map<String, num> get probabilities =>
      (json["probabilities"] as Map<String, Object?>)
          .map((k, v) => MapEntry(k, v as num));
}

final class AnswerTag extends Answer {
  AnswerTag._(super.json);
  factory AnswerTag.read(Object? value) => AnswerTag._(readObject(value));
  String get kind => json["kind"] as String;
  Map<String, num> get probabilities =>
      (json["probabilities"] as Map<String, Object?>)
          .map((k, v) => MapEntry(k, v as num));
}

final class AnswerYesNo extends Answer {
  AnswerYesNo._(super.json);
  factory AnswerYesNo.read(Object? value) => AnswerYesNo._(readObject(value));
  String get kind => json["kind"] as String;
  num get probability => json["probability"] as num;
}

final class BatchSetting {
  final Object? value;
  const BatchSetting._(this.value);
  factory BatchSetting.read(Object? value) =>
      value is BatchSetting ? value : BatchSetting._(value);
  Object? toJson() => value;
  const BatchSetting.alternative0(BigInt this.value);
  BigInt get asAlternative0 => readInteger(value);
  const BatchSetting.alternative1(String this.value);
  String get asAlternative1 => value as String;
}

final class BatchWarning extends NativeObject {
  BatchWarning._(super.json);
  factory BatchWarning.read(Object? value) => BatchWarning._(readObject(value));
  BatchSetting get running => BatchSetting.read(json["running"]);
  BatchSetting get tunedFor => BatchSetting.read(json["tuned_for"]);
}

final class Entity extends NativeObject {
  Entity._(super.json);
  factory Entity.read(Object? value) => Entity._(readObject(value));
  BigInt get end => readInteger(json["end"]);
  Presence<String> get file => json.containsKey("file")
      ? Presence.present(json["file"] as String)
      : const Presence.absent();
  Presence<BigInt> get firstLine => json.containsKey("first_line")
      ? Presence.present(readInteger(json["first_line"]))
      : const Presence.absent();
  String get kind => json["kind"] as String;
  Presence<BigInt> get lastLine => json.containsKey("last_line")
      ? Presence.present(readInteger(json["last_line"]))
      : const Presence.absent();
  BigInt get length => readInteger(json["length"]);
  BigInt get start => readInteger(json["start"]);
  num get strength => json["strength"] as num;
  String get text => json["text"] as String;
}

final class EntityEdge extends NativeObject {
  EntityEdge._(super.json);
  factory EntityEdge.read(Object? value) => EntityEdge._(readObject(value));
  Presence<bool> get either => json.containsKey("either")
      ? Presence.present(json["either"] as bool)
      : const Presence.absent();
  num get probability => json["probability"] as num;
  String get relation => json["relation"] as String;
  Entity get source => Entity.read(json["source"]);
  Entity get target => Entity.read(json["target"]);
}

final class Failed extends NativeObject {
  Failed._(super.json);
  factory Failed.read(Object? value) => Failed._(readObject(value));
  Failure get failed => Failure.read(json["failed"]);
}

final class Failure extends NativeObject {
  Failure._(super.json);
  factory Failure.read(Object? value) => Failure._(readObject(value));
  String get cause => json["cause"] as String;
  String get kind => json["kind"] as String;
}

final class FindAnswer extends NativeObject {
  FindAnswer._(super.json);
  factory FindAnswer.read(Object? value) => FindAnswer._(readObject(value));
  Presence<num> get confidence => json.containsKey("confidence")
      ? Presence.present(json["confidence"] as num)
      : const Presence.absent();
  String get kind => json["kind"] as String;
  String get pick => json["pick"] as String;
  Map<String, num> get probabilities =>
      (json["probabilities"] as Map<String, Object?>)
          .map((k, v) => MapEntry(k, v as num));
}

final class NameOdds extends NativeObject {
  NameOdds._(super.json);
  factory NameOdds.read(Object? value) => NameOdds._(readObject(value));
  Map<String, num>? get edges => json["edges"] == null
      ? null
      : (json["edges"] as Map<String, Object?>)
          .map((k, v) => MapEntry(k, v as num));
  BigInt get end => readInteger(json["end"]);
  Map<String, num>? get kinds => json["kinds"] == null
      ? null
      : (json["kinds"] as Map<String, Object?>)
          .map((k, v) => MapEntry(k, v as num));
  BigInt get start => readInteger(json["start"]);
}

final class PairOdds extends NativeObject {
  PairOdds._(super.json);
  factory PairOdds.read(Object? value) => PairOdds._(readObject(value));
  num get probability => json["probability"] as num;
  String get relation => json["relation"] as String;
  Place get source => Place.read(json["source"]);
  Place get target => Place.read(json["target"]);
}

final class PieceOdds extends NativeObject {
  PieceOdds._(super.json);
  factory PieceOdds.read(Object? value) => PieceOdds._(readObject(value));
  BigInt get end => readInteger(json["end"]);
  BigInt get start => readInteger(json["start"]);
  Map<String, num> get tags => (json["tags"] as Map<String, Object?>)
      .map((k, v) => MapEntry(k, v as num));
}

final class Place extends NativeObject {
  Place._(super.json);
  factory Place.read(Object? value) => Place._(readObject(value));
  BigInt get end => readInteger(json["end"]);
  BigInt get start => readInteger(json["start"]);
}

final class ProfileWarning extends NativeObject {
  ProfileWarning._(super.json);
  factory ProfileWarning.read(Object? value) =>
      ProfileWarning._(readObject(value));
  String get running => json["running"] as String;
  String get tunedFor => json["tuned_for"] as String;
}

sealed class Recognize extends NativeObject {
  Recognize(super.json);
  factory Recognize.read(Object? value) {
    final map = readObject(value);
    if (map.containsKey("entities") &&
        !map.containsKey("mode") &&
        !map.containsKey("proposals")) return RecognizeFieldsEntities.read(map);
    if (map.containsKey("mode") &&
        map.containsKey("proposals") &&
        !map.containsKey("entities"))
      return RecognizeFieldsModeProposals.read(map);
    throw FormatException("Unknown Recognize alternative");
  }
}

final class RecognizeAnswer extends NativeObject {
  RecognizeAnswer._(super.json);
  factory RecognizeAnswer.read(Object? value) =>
      RecognizeAnswer._(readObject(value));
  List<NameOdds> get names => (json["names"] as List)
      .map((v) => NameOdds.read(v))
      .toList(growable: false);
  List<PairOdds> get pairs => (json["pairs"] as List)
      .map((v) => PairOdds.read(v))
      .toList(growable: false);
  List<PieceOdds> get pieces => (json["pieces"] as List)
      .map((v) => PieceOdds.read(v))
      .toList(growable: false);
  List<RecognitionProposal> get proposals => (json["proposals"] as List)
      .map((v) => RecognitionProposal.read(v))
      .toList(growable: false);
}

final class RecognizeFieldsEntities extends Recognize {
  RecognizeFieldsEntities._(super.json);
  factory RecognizeFieldsEntities.read(Object? value) =>
      RecognizeFieldsEntities._(readObject(value));
  List<Entity> get entities => (json["entities"] as List)
      .map((v) => Entity.read(v))
      .toList(growable: false);
  Presence<List<EntityEdge>> get relations => json.containsKey("relations")
      ? Presence.present((json["relations"] as List)
          .map((v) => EntityEdge.read(v))
          .toList(growable: false))
      : const Presence.absent();
}

final class RecognizeFieldsModeProposals extends Recognize {
  RecognizeFieldsModeProposals._(super.json);
  factory RecognizeFieldsModeProposals.read(Object? value) =>
      RecognizeFieldsModeProposals._(readObject(value));
  String get mode => json["mode"] as String;
  List<BoundaryProposal> get proposals => (json["proposals"] as List)
      .map((v) => BoundaryProposal.read(v))
      .toList(growable: false);
}

final class RelateFields extends NativeObject {
  RelateFields._(super.json);
  factory RelateFields.read(Object? value) => RelateFields._(readObject(value));
  String get kind => json["kind"] as String;
  String get name => json["name"] as String;
}

final class RelatedEntity extends NativeObject {
  RelatedEntity._(super.json);
  factory RelatedEntity.read(Object? value) =>
      RelatedEntity._(readObject(value));
  String get kind => json["kind"] as String;
  String get name => json["name"] as String;
}

final class RelatedEntityEdge extends NativeObject {
  RelatedEntityEdge._(super.json);
  factory RelatedEntityEdge.read(Object? value) =>
      RelatedEntityEdge._(readObject(value));
  Presence<bool> get either => json.containsKey("either")
      ? Presence.present(json["either"] as bool)
      : const Presence.absent();
  num get probability => json["probability"] as num;
  String get relation => json["relation"] as String;
  RelatedEntityEdgePropertiesSource get source =>
      RelatedEntityEdgePropertiesSource.read(json["source"]);
  RelatedEntityEdgePropertiesSource get target =>
      RelatedEntityEdgePropertiesSource.read(json["target"]);
}

sealed class RelatedEntityEdgePropertiesSource extends NativeObject {
  RelatedEntityEdgePropertiesSource(super.json);
  factory RelatedEntityEdgePropertiesSource.read(Object? value) {
    final map = readObject(value);
    if (map.containsKey("kind") &&
        map.containsKey("name") &&
        !map.containsKey("file") &&
        !map.containsKey("ordinal") &&
        !map.containsKey("record"))
      return RelatedEntityEdgePropertiesSourceFieldsKindName.read(map);
    if (map.containsKey("file") &&
        map.containsKey("kind") &&
        map.containsKey("name") &&
        map.containsKey("ordinal") &&
        map.containsKey("record"))
      return RelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord
          .read(map);
    throw FormatException(
        "Unknown RelatedEntityEdgePropertiesSource alternative");
  }
}

final class RelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord
    extends RelatedEntityEdgePropertiesSource {
  RelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord._(
      super.json);
  factory RelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord.read(
          Object? value) =>
      RelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord._(
          readObject(value));
  String? get file => json["file"] == null ? null : json["file"] as String;
  Presence<BigInt> get firstLine => json.containsKey("first_line")
      ? Presence.present(readInteger(json["first_line"]))
      : const Presence.absent();
  String get kind => json["kind"] as String;
  Presence<BigInt> get lastLine => json.containsKey("last_line")
      ? Presence.present(readInteger(json["last_line"]))
      : const Presence.absent();
  String get name => json["name"] as String;
  BigInt get ordinal => readInteger(json["ordinal"]);
  Object? get record => json["record"];
}

final class RelatedEntityEdgePropertiesSourceFieldsKindName
    extends RelatedEntityEdgePropertiesSource {
  RelatedEntityEdgePropertiesSourceFieldsKindName._(super.json);
  factory RelatedEntityEdgePropertiesSourceFieldsKindName.read(Object? value) =>
      RelatedEntityEdgePropertiesSourceFieldsKindName._(readObject(value));
  String get kind => json["kind"] as String;
  String get name => json["name"] as String;
}

final class RelationRule extends NativeObject {
  RelationRule._(super.json);
  factory RelationRule.read(Object? value) => RelationRule._(readObject(value));
  bool get either => json["either"] as bool;
  String get name => json["name"] as String;
  String get reads => json["reads"] as String;
  Presence<bool> get single => json.containsKey("single")
      ? Presence.present(json["single"] as bool)
      : const Presence.absent();
  String get source => json["source"] as String;
  String get target => json["target"] as String;
}

final class SessionAnnotation extends NativeObject {
  SessionAnnotation._(super.json);
  factory SessionAnnotation.read(Object? value) =>
      SessionAnnotation._(readObject(value));
  String get name => json["name"] as String;
  AnnotationValue get value => AnnotationValue.read(json["value"]);
}

final class SessionInputSource extends NativeObject {
  SessionInputSource._(super.json);
  factory SessionInputSource.read(Object? value) =>
      SessionInputSource._(readObject(value));
  BigInt get index => readInteger(json["index"]);
  PhysicalSource get source => PhysicalSource.read(json["source"]);
}

sealed class SessionJudgment extends NativeObject {
  SessionJudgment(super.json);
  factory SessionJudgment.read(Object? value) {
    final map = readObject(value);
    if (map["kind"] == "decision") return SessionJudgmentDecision.read(map);
    if (map["kind"] == "choice") return SessionJudgmentChoice.read(map);
    if (map["kind"] == "score") return SessionJudgmentScore.read(map);
    if (map["kind"] == "tags") return SessionJudgmentTags.read(map);
    throw FormatException("Unknown SessionJudgment alternative");
  }
}

final class SessionJudgmentChoice extends SessionJudgment {
  SessionJudgmentChoice._(super.json);
  factory SessionJudgmentChoice.read(Object? value) =>
      SessionJudgmentChoice._(readObject(value));
  String get kind => json["kind"] as String;
  String? get value => json["value"] == null ? null : json["value"] as String;
}

final class SessionJudgmentDecision extends SessionJudgment {
  SessionJudgmentDecision._(super.json);
  factory SessionJudgmentDecision.read(Object? value) =>
      SessionJudgmentDecision._(readObject(value));
  String get kind => json["kind"] as String;
  bool? get value => json["value"] == null ? null : json["value"] as bool;
}

final class SessionJudgmentScore extends SessionJudgment {
  SessionJudgmentScore._(super.json);
  factory SessionJudgmentScore.read(Object? value) =>
      SessionJudgmentScore._(readObject(value));
  String get kind => json["kind"] as String;
  num get value => json["value"] as num;
}

final class SessionJudgmentTags extends SessionJudgment {
  SessionJudgmentTags._(super.json);
  factory SessionJudgmentTags.read(Object? value) =>
      SessionJudgmentTags._(readObject(value));
  String get kind => json["kind"] as String;
  List<String> get value =>
      (json["value"] as List).map((v) => v as String).toList(growable: false);
}

final class SessionNamedProbability extends NativeObject {
  SessionNamedProbability._(super.json);
  factory SessionNamedProbability.read(Object? value) =>
      SessionNamedProbability._(readObject(value));
  String get name => json["name"] as String;
  num get probability => json["probability"] as num;
}

sealed class SessionObservation extends NativeObject {
  SessionObservation(super.json);
  factory SessionObservation.read(Object? value) {
    final map = readObject(value);
    if (map["kind"] == "question") return SessionObservationQuestion.read(map);
    if (map["kind"] == "row") return SessionObservationRow.read(map);
    throw FormatException("Unknown SessionObservation alternative");
  }
}

final class SessionObservationQuestion extends SessionObservation {
  SessionObservationQuestion._(super.json);
  factory SessionObservationQuestion.read(Object? value) =>
      SessionObservationQuestion._(readObject(value));
  SessionQuestionDetail get detail =>
      SessionQuestionDetail.read(json["detail"]);
  BigInt get index => readInteger(json["index"]);
  String get kind => json["kind"] as String;
  Presence<String> get member => json.containsKey("member")
      ? Presence.present(json["member"] as String)
      : const Presence.absent();
  BigInt get position => readInteger(json["position"]);
  Presence<String> get stage => json.containsKey("stage")
      ? Presence.present(json["stage"] as String)
      : const Presence.absent();
}

final class SessionObservationRow extends SessionObservation {
  SessionObservationRow._(super.json);
  factory SessionObservationRow.read(Object? value) =>
      SessionObservationRow._(readObject(value));
  BigInt get index => readInteger(json["index"]);
  String get kind => json["kind"] as String;
  SessionObservedRow get value => SessionObservedRow.read(json["value"]);
}

sealed class SessionObservedRow extends NativeObject {
  SessionObservedRow(super.json);
  factory SessionObservedRow.read(Object? value) {
    final map = readObject(value);
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
  SessionObservedRowAnnotated._(super.json);
  factory SessionObservedRowAnnotated.read(Object? value) =>
      SessionObservedRowAnnotated._(readObject(value));
  String get kind => json["kind"] as String;
  List<SessionAnnotation> get value => (json["value"] as List)
      .map((v) => SessionAnnotation.read(v))
      .toList(growable: false);
}

final class SessionObservedRowFind extends SessionObservedRow {
  SessionObservedRowFind._(super.json);
  factory SessionObservedRowFind.read(Object? value) =>
      SessionObservedRowFind._(readObject(value));
  String get kind => json["kind"] as String;
  BigInt? get value =>
      json["value"] == null ? null : readInteger(json["value"]);
}

final class SessionObservedRowJudgment extends SessionObservedRow {
  SessionObservedRowJudgment._(super.json);
  factory SessionObservedRowJudgment.read(Object? value) =>
      SessionObservedRowJudgment._(readObject(value));
  String get kind => json["kind"] as String;
  SessionJudgment get value => SessionJudgment.read(json["value"]);
}

final class SessionObservedRowRecognized extends SessionObservedRow {
  SessionObservedRowRecognized._(super.json);
  factory SessionObservedRowRecognized.read(Object? value) =>
      SessionObservedRowRecognized._(readObject(value));
  String get kind => json["kind"] as String;
  SessionRecognition get value => SessionRecognition.read(json["value"]);
}

final class SessionObservedRowRelations extends SessionObservedRow {
  SessionObservedRowRelations._(super.json);
  factory SessionObservedRowRelations.read(Object? value) =>
      SessionObservedRowRelations._(readObject(value));
  String get kind => json["kind"] as String;
  List<SessionRelationEdge> get value => (json["value"] as List)
      .map((v) => SessionRelationEdge.read(v))
      .toList(growable: false);
}

sealed class SessionPacket extends NativeObject {
  SessionPacket(super.json);
  factory SessionPacket.read(Object? value) {
    final map = readObject(value);
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
  SessionPacketAnnotateAggregate._(super.json);
  factory SessionPacketAnnotateAggregate.read(Object? value) =>
      SessionPacketAnnotateAggregate._(readObject(value));
  String get function => json["function"] as String;
  String get kind => json["kind"] as String;
  List<Annotation> get value => (json["value"] as List)
      .map((v) => Annotation.read(v))
      .toList(growable: false);
}

final class SessionPacketAnnotateRow extends SessionPacket {
  SessionPacketAnnotateRow._(super.json);
  factory SessionPacketAnnotateRow.read(Object? value) =>
      SessionPacketAnnotateRow._(readObject(value));
  String get function => json["function"] as String;
  String get kind => json["kind"] as String;
  Annotation get value => Annotation.read(json["value"]);
}

final class SessionPacketChooseAggregate extends SessionPacket {
  SessionPacketChooseAggregate._(super.json);
  factory SessionPacketChooseAggregate.read(Object? value) =>
      SessionPacketChooseAggregate._(readObject(value));
  String get function => json["function"] as String;
  String get kind => json["kind"] as String;
  List<AtomicNullableString> get value => (json["value"] as List)
      .map((v) => AtomicNullableString.read(v))
      .toList(growable: false);
}

final class SessionPacketChooseRow extends SessionPacket {
  SessionPacketChooseRow._(super.json);
  factory SessionPacketChooseRow.read(Object? value) =>
      SessionPacketChooseRow._(readObject(value));
  String get function => json["function"] as String;
  String get kind => json["kind"] as String;
  AtomicNullableString get value => AtomicNullableString.read(json["value"]);
}

final class SessionPacketDecideAggregate extends SessionPacket {
  SessionPacketDecideAggregate._(super.json);
  factory SessionPacketDecideAggregate.read(Object? value) =>
      SessionPacketDecideAggregate._(readObject(value));
  String get function => json["function"] as String;
  String get kind => json["kind"] as String;
  List<AtomicDecideValue> get value => (json["value"] as List)
      .map((v) => AtomicDecideValue.read(v))
      .toList(growable: false);
}

final class SessionPacketDecideRow extends SessionPacket {
  SessionPacketDecideRow._(super.json);
  factory SessionPacketDecideRow.read(Object? value) =>
      SessionPacketDecideRow._(readObject(value));
  String get function => json["function"] as String;
  String get kind => json["kind"] as String;
  AtomicDecideValue get value => AtomicDecideValue.read(json["value"]);
}

final class SessionPacketFilterAggregate extends SessionPacket {
  SessionPacketFilterAggregate._(super.json);
  factory SessionPacketFilterAggregate.read(Object? value) =>
      SessionPacketFilterAggregate._(readObject(value));
  String get function => json["function"] as String;
  String get kind => json["kind"] as String;
  List<AtomicBoolean> get value => (json["value"] as List)
      .map((v) => AtomicBoolean.read(v))
      .toList(growable: false);
}

final class SessionPacketFilterRow extends SessionPacket {
  SessionPacketFilterRow._(super.json);
  factory SessionPacketFilterRow.read(Object? value) =>
      SessionPacketFilterRow._(readObject(value));
  String get function => json["function"] as String;
  String get kind => json["kind"] as String;
  AtomicBoolean get value => AtomicBoolean.read(json["value"]);
}

final class SessionPacketFindAggregate extends SessionPacket {
  SessionPacketFindAggregate._(super.json);
  factory SessionPacketFindAggregate.read(Object? value) =>
      SessionPacketFindAggregate._(readObject(value));
  String get function => json["function"] as String;
  String get kind => json["kind"] as String;
  Find get value => Find.read(json["value"]);
}

final class SessionPacketObservation extends SessionPacket {
  SessionPacketObservation._(super.json);
  factory SessionPacketObservation.read(Object? value) =>
      SessionPacketObservation._(readObject(value));
  String get function => json["function"] as String;
  String get kind => json["kind"] as String;
  SessionObservation get value => SessionObservation.read(json["value"]);
}

final class SessionPacketRankAggregate extends SessionPacket {
  SessionPacketRankAggregate._(super.json);
  factory SessionPacketRankAggregate.read(Object? value) =>
      SessionPacketRankAggregate._(readObject(value));
  String get function => json["function"] as String;
  String get kind => json["kind"] as String;
  List<AtomicNonZeroUsize> get value => (json["value"] as List)
      .map((v) => AtomicNonZeroUsize.read(v))
      .toList(growable: false);
}

final class SessionPacketRecognizeAggregate extends SessionPacket {
  SessionPacketRecognizeAggregate._(super.json);
  factory SessionPacketRecognizeAggregate.read(Object? value) =>
      SessionPacketRecognizeAggregate._(readObject(value));
  String get function => json["function"] as String;
  String get kind => json["kind"] as String;
  List<Recognition> get value => (json["value"] as List)
      .map((v) => Recognition.read(v))
      .toList(growable: false);
}

final class SessionPacketRelateAggregate extends SessionPacket {
  SessionPacketRelateAggregate._(super.json);
  factory SessionPacketRelateAggregate.read(Object? value) =>
      SessionPacketRelateAggregate._(readObject(value));
  String get function => json["function"] as String;
  String get kind => json["kind"] as String;
  Relation get value => Relation.read(json["value"]);
}

final class SessionPacketScoreAggregate extends SessionPacket {
  SessionPacketScoreAggregate._(super.json);
  factory SessionPacketScoreAggregate.read(Object? value) =>
      SessionPacketScoreAggregate._(readObject(value));
  String get function => json["function"] as String;
  String get kind => json["kind"] as String;
  List<AtomicDouble> get value => (json["value"] as List)
      .map((v) => AtomicDouble.read(v))
      .toList(growable: false);
}

final class SessionPacketScoreRow extends SessionPacket {
  SessionPacketScoreRow._(super.json);
  factory SessionPacketScoreRow.read(Object? value) =>
      SessionPacketScoreRow._(readObject(value));
  String get function => json["function"] as String;
  String get kind => json["kind"] as String;
  AtomicDouble get value => AtomicDouble.read(json["value"]);
}

final class SessionPacketTagAggregate extends SessionPacket {
  SessionPacketTagAggregate._(super.json);
  factory SessionPacketTagAggregate.read(Object? value) =>
      SessionPacketTagAggregate._(readObject(value));
  String get function => json["function"] as String;
  String get kind => json["kind"] as String;
  List<AtomicArrayOfString> get value => (json["value"] as List)
      .map((v) => AtomicArrayOfString.read(v))
      .toList(growable: false);
}

final class SessionPacketTagRow extends SessionPacket {
  SessionPacketTagRow._(super.json);
  factory SessionPacketTagRow.read(Object? value) =>
      SessionPacketTagRow._(readObject(value));
  String get function => json["function"] as String;
  String get kind => json["kind"] as String;
  AtomicArrayOfString get value => AtomicArrayOfString.read(json["value"]);
}

final class SessionPacketTerminal extends SessionPacket {
  SessionPacketTerminal._(super.json);
  factory SessionPacketTerminal.read(Object? value) =>
      SessionPacketTerminal._(readObject(value));
  Presence<Facts> get facts => json.containsKey("facts")
      ? Presence.present(Facts.read(json["facts"]))
      : const Presence.absent();
  Presence<CallError> get failure => json.containsKey("failure")
      ? Presence.present(CallError.read(json["failure"]))
      : const Presence.absent();
  String get kind => json["kind"] as String;
}

sealed class SessionProbabilities extends NativeObject {
  SessionProbabilities(super.json);
  factory SessionProbabilities.read(Object? value) {
    final map = readObject(value);
    if (map["kind"] == "yes_no") return SessionProbabilitiesYesNo.read(map);
    if (map["kind"] == "named") return SessionProbabilitiesNamed.read(map);
    throw FormatException("Unknown SessionProbabilities alternative");
  }
}

final class SessionProbabilitiesNamed extends SessionProbabilities {
  SessionProbabilitiesNamed._(super.json);
  factory SessionProbabilitiesNamed.read(Object? value) =>
      SessionProbabilitiesNamed._(readObject(value));
  String get kind => json["kind"] as String;
  List<SessionNamedProbability> get value => (json["value"] as List)
      .map((v) => SessionNamedProbability.read(v))
      .toList(growable: false);
}

final class SessionProbabilitiesYesNo extends SessionProbabilities {
  SessionProbabilitiesYesNo._(super.json);
  factory SessionProbabilitiesYesNo.read(Object? value) =>
      SessionProbabilitiesYesNo._(readObject(value));
  String get kind => json["kind"] as String;
  num get value => json["value"] as num;
}

final class SessionQuestionDetail extends NativeObject {
  SessionQuestionDetail._(super.json);
  factory SessionQuestionDetail.read(Object? value) =>
      SessionQuestionDetail._(readObject(value));
  Presence<String> get answerId => json.containsKey("answer_id")
      ? Presence.present(json["answer_id"] as String)
      : const Presence.absent();
  bool get cached => json["cached"] as bool;
  Presence<num> get confidence => json.containsKey("confidence")
      ? Presence.present(json["confidence"] as num)
      : const Presence.absent();
  BigInt get failedQuestions => readInteger(json["failed_questions"]);
  Presence<Failure> get failure => json.containsKey("failure")
      ? Presence.present(Failure.read(json["failure"]))
      : const Presence.absent();
  Presence<String> get failureId => json.containsKey("failure_id")
      ? Presence.present(json["failure_id"] as String)
      : const Presence.absent();
  Presence<Object?> get input => json.containsKey("input")
      ? Presence.present(json["input"])
      : const Presence.absent();
  Presence<PhysicalSource> get inputSource => json.containsKey("input_source")
      ? Presence.present(PhysicalSource.read(json["input_source"]))
      : const Presence.absent();
  List<SessionInputSource> get inputSources => (json["input_sources"] as List)
      .map((v) => SessionInputSource.read(v))
      .toList(growable: false);
  List<Object?> get inputs =>
      (json["inputs"] as List).map((v) => v).toList(growable: false);
  String get model => json["model"] as String;
  List<Observation> get observations => (json["observations"] as List)
      .map((v) => Observation.read(v))
      .toList(growable: false);
  Presence<SessionProbabilities> get probabilities =>
      json.containsKey("probabilities")
          ? Presence.present(SessionProbabilities.read(json["probabilities"]))
          : const Presence.absent();
  ReadableQuestion get question => ReadableQuestion.read(json["question"]);
  String get questionSha256 => json["question_sha256"] as String;
  List<QuestionSource> get questionSources => (json["question_sources"] as List)
      .map((v) => QuestionSource.read(v))
      .toList(growable: false);
  Presence<String> get rawPick => json.containsKey("raw_pick")
      ? Presence.present(json["raw_pick"] as String)
      : const Presence.absent();
  Presence<Usage> get reportedUsage => json.containsKey("reported_usage")
      ? Presence.present(Usage.read(json["reported_usage"]))
      : const Presence.absent();
  List<String> get requests => (json["requests"] as List)
      .map((v) => v as String)
      .toList(growable: false);
  BigInt get requestsSent => readInteger(json["requests_sent"]);
  Presence<Object?> get threshold => json.containsKey("threshold")
      ? Presence.present(json["threshold"])
      : const Presence.absent();
  String get url => json["url"] as String;
  Presence<TokenUsage> get usage => json.containsKey("usage")
      ? Presence.present(TokenUsage.read(json["usage"]))
      : const Presence.absent();
  Presence<Value> get value => json.containsKey("value")
      ? Presence.present(Value.read(json["value"]))
      : const Presence.absent();
}

final class SessionRecognition extends NativeObject {
  SessionRecognition._(super.json);
  factory SessionRecognition.read(Object? value) =>
      SessionRecognition._(readObject(value));
  List<Entity> get entities => (json["entities"] as List)
      .map((v) => Entity.read(v))
      .toList(growable: false);
  String get mode => json["mode"] as String;
  Presence<List<BoundaryProposal>> get proposals =>
      json.containsKey("proposals")
          ? Presence.present((json["proposals"] as List)
              .map((v) => BoundaryProposal.read(v))
              .toList(growable: false))
          : const Presence.absent();
  Presence<List<RecognitionEdgeDocument>> get relations =>
      json.containsKey("relations")
          ? Presence.present((json["relations"] as List)
              .map((v) => RecognitionEdgeDocument.read(v))
              .toList(growable: false))
          : const Presence.absent();
}

final class SessionRelationEdge extends NativeObject {
  SessionRelationEdge._(super.json);
  factory SessionRelationEdge.read(Object? value) =>
      SessionRelationEdge._(readObject(value));
  bool get either => json["either"] as bool;
  num get probability => json["probability"] as num;
  String get relation => json["relation"] as String;
  EntityDocument get source => EntityDocument.read(json["source"]);
  EntityDocument get target => EntityDocument.read(json["target"]);
}

final class SourceRelationEndpoint extends NativeObject {
  SourceRelationEndpoint._(super.json);
  factory SourceRelationEndpoint.read(Object? value) =>
      SourceRelationEndpoint._(readObject(value));
  String? get file => json["file"] == null ? null : json["file"] as String;
  Presence<BigInt> get firstLine => json.containsKey("first_line")
      ? Presence.present(readInteger(json["first_line"]))
      : const Presence.absent();
  String get kind => json["kind"] as String;
  Presence<BigInt> get lastLine => json.containsKey("last_line")
      ? Presence.present(readInteger(json["last_line"]))
      : const Presence.absent();
  String get name => json["name"] as String;
  BigInt get ordinal => readInteger(json["ordinal"]);
  Object? get record => json["record"];
}

final class TokenUsage extends NativeObject {
  TokenUsage._(super.json);
  factory TokenUsage.read(Object? value) => TokenUsage._(readObject(value));
  BigInt get inputTokens => readInteger(json["input_tokens"]);
  BigInt get outputTokens => readInteger(json["output_tokens"]);
}

final class Value {
  final Object? value;
  const Value(this.value);
  factory Value.read(Object? value) => value is Value ? value : Value(value);
  Object? toJson() => value;
  bool get asBoolean => value as bool;
  String get asString => value as String;
  List<String> get asArray =>
      (value as List).map((v) => v as String).toList(growable: false);
  num get asNumber => value as num;
}
