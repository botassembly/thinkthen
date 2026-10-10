// Generated from the Rust result graph. Do not edit.
package thinkthen.kotlin
import thinkthen.Values
import thinkthen.Presence
import java.math.*
object Results {
abstract class View(value: Any?) {
val json: Any? = Values.freeze(value)
fun presence(member: String): Presence.State { val objectValue = Values.`object`(json); return if (!objectValue.containsKey(member)) Presence.State.MISSING else if (objectValue[member] == null) Presence.State.NULL else Presence.State.VALUE }
protected fun required(member: String): Any? { val objectValue = Values.`object`(json); check(objectValue.containsKey(member)) { "Missing native member: $member" }; return objectValue[member] }
protected fun optional(member: String): Any? = Values.`object`(json)[member]
}
class Annotation(rawValue: Any?) : View(rawValue) {
val `answerId`: AnswerId get() = required("answer_id").let { value -> AnswerId.read(value) }
val `answers`: Map<String, AnnotationMember> get() = required("answers").let { value -> Values.map(value) { item0 -> AnnotationMember.read(item0) } }
val `file`: String? get() = optional("file")?.let { value -> value as String }
val `firstLine`: BigInteger? get() = optional("first_line")?.let { value -> Values.integer(value) }
val `index`: BigInteger? get() = optional("index")?.let { value -> Values.integer(value) }
val `input`: Any? get() = required("input")?.let { value -> Values.freeze(value) }
val `lastLine`: BigInteger? get() = optional("last_line")?.let { value -> Values.integer(value) }
val `meta`: Meta get() = required("meta").let { value -> Meta.read(value) }
val `position`: Position? get() = optional("position")?.let { value -> Position.read(value) }
val `schema`: Version get() = required("schema").let { value -> Version.read(value) }
val `source`: PhysicalSource? get() = optional("source")?.let { value -> PhysicalSource.read(value) }
val `value`: AnnotatedRow get() = required("value").let { value -> AnnotatedRow.read(value) }
companion object { fun read(value: Any?): Annotation = Annotation(value) }
}
sealed interface AnnotationMember { val json: Any?
companion object {
fun read(value: Any?): AnnotationMember {
val objectValue = Values.`object`(value)
if (objectValue.containsKey("answer_id")) return AnnotationMemberAnswerId.read(value)
if (objectValue.containsKey("failure_id")) return AnnotationMemberFailureId.read(value)
throw IllegalStateException("Unknown native result alternative")
}
}
}
class AnnotationMemberAnswerId(rawValue: Any?) : View(rawValue), AnnotationMember {
val `answer`: Answer get() = required("answer").let { value -> Answer.read(value) }
val `answerId`: AnswerId get() = required("answer_id").let { value -> AnswerId.read(value) }
val `observations`: List<Observation> get() = required("observations").let { value -> Values.list(value) { item0 -> Observation.read(item0) } }
val `question`: ReadableQuestion get() = required("question").let { value -> ReadableQuestion.read(value) }
val `questionSources`: List<QuestionSource> get() = required("question_sources").let { value -> Values.list(value) { item0 -> QuestionSource.read(item0) } }
val `request`: String get() = required("request").let { value -> value as String }
val `threshold`: Threshold? get() = required("threshold")?.let { value -> Threshold.read(value) }
val `usage`: Usage? get() = optional("usage")?.let { value -> Usage.read(value) }
val `value`: Value? get() = required("value")?.let { value -> Value.read(value) }
companion object { fun read(value: Any?): AnnotationMemberAnswerId = AnnotationMemberAnswerId(value) }
}
class AnnotationMemberFailureId(rawValue: Any?) : View(rawValue), AnnotationMember {
val `failure`: Failure get() = required("failure").let { value -> Failure.read(value) }
val `failureId`: FailureId get() = required("failure_id").let { value -> FailureId.read(value) }
val `observations`: List<Observation> get() = required("observations").let { value -> Values.list(value) { item0 -> Observation.read(item0) } }
val `question`: ReadableQuestion get() = required("question").let { value -> ReadableQuestion.read(value) }
val `questionSources`: List<QuestionSource> get() = required("question_sources").let { value -> Values.list(value) { item0 -> QuestionSource.read(item0) } }
val `request`: String get() = required("request").let { value -> value as String }
val `threshold`: Threshold? get() = required("threshold")?.let { value -> Threshold.read(value) }
val `usage`: Usage? get() = optional("usage")?.let { value -> Usage.read(value) }
companion object { fun read(value: Any?): AnnotationMemberFailureId = AnnotationMemberFailureId(value) }
}
sealed interface AnnotationValue { val json: Any?
companion object {
fun read(value: Any?): AnnotationValue {
val objectValue = Values.`object`(value)
if (objectValue["kind"] == "decision") return AnnotationValueDecision.read(value)
if (objectValue["kind"] == "choice") return AnnotationValueChoice.read(value)
if (objectValue["kind"] == "score") return AnnotationValueScore.read(value)
if (objectValue["kind"] == "tags") return AnnotationValueTags.read(value)
if (objectValue["kind"] == "failed") return AnnotationValueFailed.read(value)
throw IllegalStateException("Unknown native result alternative")
}
}
}
class AnnotationValueChoice(rawValue: Any?) : View(rawValue), AnnotationValue {
val `kind`: String get() = required("kind").let { value -> value as String }
val `value`: String? get() = required("value")?.let { value -> value as String }
companion object { fun read(value: Any?): AnnotationValueChoice = AnnotationValueChoice(value) }
}
class AnnotationValueDecision(rawValue: Any?) : View(rawValue), AnnotationValue {
val `kind`: String get() = required("kind").let { value -> value as String }
val `value`: Boolean? get() = required("value")?.let { value -> value as Boolean }
companion object { fun read(value: Any?): AnnotationValueDecision = AnnotationValueDecision(value) }
}
class AnnotationValueFailed(rawValue: Any?) : View(rawValue), AnnotationValue {
val `kind`: String get() = required("kind").let { value -> value as String }
val `value`: Failure get() = required("value").let { value -> Failure.read(value) }
companion object { fun read(value: Any?): AnnotationValueFailed = AnnotationValueFailed(value) }
}
class AnnotationValueScore(rawValue: Any?) : View(rawValue), AnnotationValue {
val `kind`: String get() = required("kind").let { value -> value as String }
val `value`: BigDecimal get() = required("value").let { value -> value as BigDecimal }
companion object { fun read(value: Any?): AnnotationValueScore = AnnotationValueScore(value) }
}
class AnnotationValueTags(rawValue: Any?) : View(rawValue), AnnotationValue {
val `kind`: String get() = required("kind").let { value -> value as String }
val `value`: List<String> get() = required("value").let { value -> Values.list(value) { item0 -> item0 as String } }
companion object { fun read(value: Any?): AnnotationValueTags = AnnotationValueTags(value) }
}
class AnswerId(rawValue: Any?) : View(rawValue) {
val value: String get() = json as String
companion object { fun read(value: Any?): AnswerId = AnswerId(value) }
}
class Answers(rawValue: Any?) : View(rawValue) {
val `questions`: List<RelationMember> get() = required("questions").let { value -> Values.list(value) { item0 -> RelationMember.read(item0) } }
companion object { fun read(value: Any?): Answers = Answers(value) }
}
class AtomicArrayOfString(rawValue: Any?) : View(rawValue) {
val `answer`: Answer get() = required("answer").let { value -> Answer.read(value) }
val `answerId`: AnswerId get() = required("answer_id").let { value -> AnswerId.read(value) }
val `images`: List<Image>? get() = optional("images")?.let { value -> Values.list(value) { item0 -> Image.read(item0) } }
val `index`: BigInteger? get() = optional("index")?.let { value -> Values.integer(value) }
val `input`: Any? get() = optional("input")?.let { value -> Values.freeze(value) }
val `members`: List<RankMember>? get() = optional("members")?.let { value -> Values.list(value) { item0 -> RankMember.read(item0) } }
val `meta`: Meta get() = required("meta").let { value -> Meta.read(value) }
val `question`: ReadableQuestion get() = required("question").let { value -> ReadableQuestion.read(value) }
val `questionName`: String? get() = optional("question_name")?.let { value -> value as String }
val `schema`: Version get() = required("schema").let { value -> Version.read(value) }
val `source`: PhysicalSource? get() = optional("source")?.let { value -> PhysicalSource.read(value) }
val `threshold`: Threshold? get() = required("threshold")?.let { value -> Threshold.read(value) }
val `value`: List<String> get() = required("value").let { value -> Values.list(value) { item0 -> item0 as String } }
companion object { fun read(value: Any?): AtomicArrayOfString = AtomicArrayOfString(value) }
}
class AtomicDecideValue(rawValue: Any?) : View(rawValue) {
val `answer`: Answer get() = required("answer").let { value -> Answer.read(value) }
val `answerId`: AnswerId get() = required("answer_id").let { value -> AnswerId.read(value) }
val `images`: List<Image>? get() = optional("images")?.let { value -> Values.list(value) { item0 -> Image.read(item0) } }
val `index`: BigInteger? get() = optional("index")?.let { value -> Values.integer(value) }
val `input`: Any? get() = optional("input")?.let { value -> Values.freeze(value) }
val `members`: List<RankMember>? get() = optional("members")?.let { value -> Values.list(value) { item0 -> RankMember.read(item0) } }
val `meta`: Meta get() = required("meta").let { value -> Meta.read(value) }
val `question`: ReadableQuestion get() = required("question").let { value -> ReadableQuestion.read(value) }
val `questionName`: String? get() = optional("question_name")?.let { value -> value as String }
val `schema`: Version get() = required("schema").let { value -> Version.read(value) }
val `source`: PhysicalSource? get() = optional("source")?.let { value -> PhysicalSource.read(value) }
val `threshold`: Threshold? get() = required("threshold")?.let { value -> Threshold.read(value) }
val `value`: DecideValue get() = required("value").let { value -> DecideValue.read(value) }
companion object { fun read(value: Any?): AtomicDecideValue = AtomicDecideValue(value) }
}
class AtomicNonZeroUsize(rawValue: Any?) : View(rawValue) {
val `answer`: Answer get() = required("answer").let { value -> Answer.read(value) }
val `answerId`: AnswerId get() = required("answer_id").let { value -> AnswerId.read(value) }
val `images`: List<Image>? get() = optional("images")?.let { value -> Values.list(value) { item0 -> Image.read(item0) } }
val `index`: BigInteger? get() = optional("index")?.let { value -> Values.integer(value) }
val `input`: Any? get() = optional("input")?.let { value -> Values.freeze(value) }
val `members`: List<RankMember>? get() = optional("members")?.let { value -> Values.list(value) { item0 -> RankMember.read(item0) } }
val `meta`: Meta get() = required("meta").let { value -> Meta.read(value) }
val `question`: ReadableQuestion get() = required("question").let { value -> ReadableQuestion.read(value) }
val `questionName`: String? get() = optional("question_name")?.let { value -> value as String }
val `schema`: Version get() = required("schema").let { value -> Version.read(value) }
val `source`: PhysicalSource? get() = optional("source")?.let { value -> PhysicalSource.read(value) }
val `threshold`: Threshold? get() = required("threshold")?.let { value -> Threshold.read(value) }
val `value`: BigInteger get() = required("value").let { value -> Values.integer(value) }
companion object { fun read(value: Any?): AtomicNonZeroUsize = AtomicNonZeroUsize(value) }
}
class AtomicNullableString(rawValue: Any?) : View(rawValue) {
val `answer`: Answer get() = required("answer").let { value -> Answer.read(value) }
val `answerId`: AnswerId get() = required("answer_id").let { value -> AnswerId.read(value) }
val `images`: List<Image>? get() = optional("images")?.let { value -> Values.list(value) { item0 -> Image.read(item0) } }
val `index`: BigInteger? get() = optional("index")?.let { value -> Values.integer(value) }
val `input`: Any? get() = optional("input")?.let { value -> Values.freeze(value) }
val `members`: List<RankMember>? get() = optional("members")?.let { value -> Values.list(value) { item0 -> RankMember.read(item0) } }
val `meta`: Meta get() = required("meta").let { value -> Meta.read(value) }
val `question`: ReadableQuestion get() = required("question").let { value -> ReadableQuestion.read(value) }
val `questionName`: String? get() = optional("question_name")?.let { value -> value as String }
val `schema`: Version get() = required("schema").let { value -> Version.read(value) }
val `source`: PhysicalSource? get() = optional("source")?.let { value -> PhysicalSource.read(value) }
val `threshold`: Threshold? get() = required("threshold")?.let { value -> Threshold.read(value) }
val `value`: String? get() = required("value")?.let { value -> value as String }
companion object { fun read(value: Any?): AtomicNullableString = AtomicNullableString(value) }
}
class AtomicBoolean(rawValue: Any?) : View(rawValue) {
val `answer`: Answer get() = required("answer").let { value -> Answer.read(value) }
val `answerId`: AnswerId get() = required("answer_id").let { value -> AnswerId.read(value) }
val `images`: List<Image>? get() = optional("images")?.let { value -> Values.list(value) { item0 -> Image.read(item0) } }
val `index`: BigInteger? get() = optional("index")?.let { value -> Values.integer(value) }
val `input`: Any? get() = optional("input")?.let { value -> Values.freeze(value) }
val `members`: List<RankMember>? get() = optional("members")?.let { value -> Values.list(value) { item0 -> RankMember.read(item0) } }
val `meta`: Meta get() = required("meta").let { value -> Meta.read(value) }
val `question`: ReadableQuestion get() = required("question").let { value -> ReadableQuestion.read(value) }
val `questionName`: String? get() = optional("question_name")?.let { value -> value as String }
val `schema`: Version get() = required("schema").let { value -> Version.read(value) }
val `source`: PhysicalSource? get() = optional("source")?.let { value -> PhysicalSource.read(value) }
val `threshold`: Threshold? get() = required("threshold")?.let { value -> Threshold.read(value) }
val `value`: Boolean get() = required("value").let { value -> value as Boolean }
companion object { fun read(value: Any?): AtomicBoolean = AtomicBoolean(value) }
}
class AtomicDouble(rawValue: Any?) : View(rawValue) {
val `answer`: Answer get() = required("answer").let { value -> Answer.read(value) }
val `answerId`: AnswerId get() = required("answer_id").let { value -> AnswerId.read(value) }
val `images`: List<Image>? get() = optional("images")?.let { value -> Values.list(value) { item0 -> Image.read(item0) } }
val `index`: BigInteger? get() = optional("index")?.let { value -> Values.integer(value) }
val `input`: Any? get() = optional("input")?.let { value -> Values.freeze(value) }
val `members`: List<RankMember>? get() = optional("members")?.let { value -> Values.list(value) { item0 -> RankMember.read(item0) } }
val `meta`: Meta get() = required("meta").let { value -> Meta.read(value) }
val `question`: ReadableQuestion get() = required("question").let { value -> ReadableQuestion.read(value) }
val `questionName`: String? get() = optional("question_name")?.let { value -> value as String }
val `schema`: Version get() = required("schema").let { value -> Version.read(value) }
val `source`: PhysicalSource? get() = optional("source")?.let { value -> PhysicalSource.read(value) }
val `threshold`: Threshold? get() = required("threshold")?.let { value -> Threshold.read(value) }
val `value`: BigDecimal get() = required("value").let { value -> value as BigDecimal }
companion object { fun read(value: Any?): AtomicDouble = AtomicDouble(value) }
}
class Attempt(rawValue: Any?) : View(rawValue) {
val `ordinal`: BigInteger get() = required("ordinal").let { value -> Values.integer(value) }
val `outcome`: AttemptOutcome get() = required("outcome").let { value -> AttemptOutcome.read(value) }
val `requestId`: String? get() = optional("request_id")?.let { value -> value as String }
val `requestSha256`: String get() = required("request_sha256").let { value -> value as String }
val `sdkRequestId`: SdkRequestId get() = required("sdk_request_id").let { value -> SdkRequestId.read(value) }
val `serverMs`: BigInteger? get() = optional("server_ms")?.let { value -> Values.integer(value) }
val `status`: BigInteger? get() = optional("status")?.let { value -> Values.integer(value) }
val `wallMs`: BigInteger get() = required("wall_ms").let { value -> Values.integer(value) }
companion object { fun read(value: Any?): Attempt = Attempt(value) }
}
sealed interface Batch { val json: Any?
companion object {
fun read(value: Any?): Batch {
if (value is java.math.BigDecimal) return BatchInteger.read(value)
if (value is String) return BatchString.read(value)
throw IllegalStateException("Unknown native result alternative")
}
}
}
class BatchInteger(override val json: Any?) : Batch { val value: BigInteger get() = Values.integer(json)
companion object { fun read(value: Any?): BatchInteger = BatchInteger(Values.freeze(value)) }
}
class BatchString(override val json: Any?) : Batch { val value: String get() = json as String
companion object { fun read(value: Any?): BatchString = BatchString(Values.freeze(value)) }
}
class BoundaryMode(rawValue: Any?) : View(rawValue) {
val value: String get() = json as String
companion object { fun read(value: Any?): BoundaryMode = BoundaryMode(value) }
}
class BoundaryOdds(rawValue: Any?) : View(rawValue) {
val `pieces`: List<PieceOdds> get() = required("pieces").let { value -> Values.list(value) { item0 -> PieceOdds.read(item0) } }
val `proposals`: List<BoundaryProposal> get() = required("proposals").let { value -> Values.list(value) { item0 -> BoundaryProposal.read(item0) } }
companion object { fun read(value: Any?): BoundaryOdds = BoundaryOdds(value) }
}
class BoundaryProposal(rawValue: Any?) : View(rawValue) {
val `end`: BigInteger get() = required("end").let { value -> Values.integer(value) }
val `length`: BigInteger get() = required("length").let { value -> Values.integer(value) }
val `probability`: BigDecimal get() = required("probability").let { value -> value as BigDecimal }
val `start`: BigInteger get() = required("start").let { value -> Values.integer(value) }
val `text`: String get() = required("text").let { value -> value as String }
companion object { fun read(value: Any?): BoundaryProposal = BoundaryProposal(value) }
}
class CallError(rawValue: Any?) : View(rawValue) {
val `error`: Error get() = required("error").let { value -> Error.read(value) }
val `facts`: Facts? get() = optional("facts")?.let { value -> Facts.read(value) }
companion object { fun read(value: Any?): CallError = CallError(value) }
}
class CallId(rawValue: Any?) : View(rawValue) {
val value: String get() = json as String
companion object { fun read(value: Any?): CallId = CallId(value) }
}
class DecideValue(rawValue: Any?) : View(rawValue) {
val value: Any? get() = Values.freeze(json)
companion object { fun read(value: Any?): DecideValue = DecideValue(value) }
}
class EntityDocument(rawValue: Any?) : View(rawValue) {
val `kind`: String get() = required("kind").let { value -> value as String }
val `name`: String get() = required("name").let { value -> value as String }
companion object { fun read(value: Any?): EntityDocument = EntityDocument(value) }
}
class Error(rawValue: Any?) : View(rawValue) {
val `estimatedInputDenial`: EstimatedInputDenial? get() = optional("estimated_input_denial")?.let { value -> EstimatedInputDenial.read(value) }
val `kind`: FailureKind get() = required("kind").let { value -> FailureKind.read(value) }
val `message`: String get() = required("message").let { value -> value as String }
val `retryable`: Boolean get() = required("retryable").let { value -> value as Boolean }
val `sendBudgetDenial`: SendBudgetDenial? get() = optional("send_budget_denial")?.let { value -> SendBudgetDenial.read(value) }
val `stopped`: Stopped get() = required("stopped").let { value -> Stopped.read(value) }
companion object { fun read(value: Any?): Error = Error(value) }
}
sealed interface EstimatedInputDenial { val json: Any?
companion object {
fun read(value: Any?): EstimatedInputDenial {
val objectValue = Values.`object`(value)
if (objectValue["kind"] == "initial_request") return EstimatedInputDenialInitialRequest.read(value)
if (objectValue["kind"] == "additional_request") return EstimatedInputDenialAdditionalRequest.read(value)
if (objectValue["kind"] == "retry") return EstimatedInputDenialRetry.read(value)
throw IllegalStateException("Unknown native result alternative")
}
}
}
class EstimatedInputDenialAdditionalRequest(rawValue: Any?) : View(rawValue), EstimatedInputDenial {
val `kind`: String get() = required("kind").let { value -> value as String }
val `limit`: BigInteger get() = required("limit").let { value -> Values.integer(value) }
companion object { fun read(value: Any?): EstimatedInputDenialAdditionalRequest = EstimatedInputDenialAdditionalRequest(value) }
}
class EstimatedInputDenialInitialRequest(rawValue: Any?) : View(rawValue), EstimatedInputDenial {
val `kind`: String get() = required("kind").let { value -> value as String }
val `limit`: BigInteger get() = required("limit").let { value -> Values.integer(value) }
companion object { fun read(value: Any?): EstimatedInputDenialInitialRequest = EstimatedInputDenialInitialRequest(value) }
}
class EstimatedInputDenialRetry(rawValue: Any?) : View(rawValue), EstimatedInputDenial {
val `kind`: String get() = required("kind").let { value -> value as String }
val `lastStatus`: BigInteger get() = required("last_status").let { value -> Values.integer(value) }
val `limit`: BigInteger get() = required("limit").let { value -> Values.integer(value) }
companion object { fun read(value: Any?): EstimatedInputDenialRetry = EstimatedInputDenialRetry(value) }
}
class Facts(rawValue: Any?) : View(rawValue) {
val `attempts`: List<Attempt>? get() = optional("attempts")?.let { value -> Values.list(value) { item0 -> Attempt.read(item0) } }
val `cacheAnswers`: BigInteger get() = required("cache_answers").let { value -> Values.integer(value) }
val `callId`: CallId get() = required("call_id").let { value -> CallId.read(value) }
val `estimatedCostUsd`: String? get() = optional("estimated_cost_usd")?.let { value -> value as String }
val `heldModelMismatch`: Boolean? get() = optional("held_model_mismatch")?.let { value -> value as Boolean }
val `inputTokens`: BigInteger? get() = optional("input_tokens")?.let { value -> Values.integer(value) }
val `largestRequestBytes`: BigInteger get() = required("largest_request_bytes").let { value -> Values.integer(value) }
val `largestRequestEstimatedInputTokens`: BigInteger? get() = required("largest_request_estimated_input_tokens")?.let { value -> Values.integer(value) }
val `model`: String? get() = optional("model")?.let { value -> value as String }
val `outputTokens`: BigInteger? get() = optional("output_tokens")?.let { value -> Values.integer(value) }
val `records`: BigInteger get() = required("records").let { value -> Values.integer(value) }
val `requestsSent`: BigInteger get() = required("requests_sent").let { value -> Values.integer(value) }
val `seconds`: BigDecimal get() = required("seconds").let { value -> value as BigDecimal }
val `tokenEstimateMethod`: String get() = required("token_estimate_method").let { value -> value as String }
val `usagePersistence`: PersistenceObservation? get() = optional("usage_persistence")?.let { value -> PersistenceObservation.read(value) }
companion object { fun read(value: Any?): Facts = Facts(value) }
}
class FailureId(rawValue: Any?) : View(rawValue) {
val value: String get() = json as String
companion object { fun read(value: Any?): FailureId = FailureId(value) }
}
class Find(rawValue: Any?) : View(rawValue) {
val `answer`: FindAnswer get() = required("answer").let { value -> FindAnswer.read(value) }
val `answerId`: AnswerId get() = required("answer_id").let { value -> AnswerId.read(value) }
val `candidates`: List<FindCandidate>? get() = optional("candidates")?.let { value -> Values.list(value) { item0 -> FindCandidate.read(item0) } }
val `file`: String? get() = optional("file")?.let { value -> value as String }
val `firstLine`: BigInteger? get() = optional("first_line")?.let { value -> Values.integer(value) }
val `index`: BigInteger? get() = required("index")?.let { value -> Values.integer(value) }
val `lastLine`: BigInteger? get() = optional("last_line")?.let { value -> Values.integer(value) }
val `meta`: Meta get() = required("meta").let { value -> Meta.read(value) }
val `position`: Position? get() = optional("position")?.let { value -> Position.read(value) }
val `question`: ReadableQuestion2 get() = required("question").let { value -> ReadableQuestion2.read(value) }
val `schema`: Version get() = required("schema").let { value -> Version.read(value) }
val `threshold`: Any? get() = required("threshold")?.let { value -> Values.freeze(value) }
val `value`: Any? get() = required("value")?.let { value -> Values.freeze(value) }
companion object { fun read(value: Any?): Find = Find(value) }
}
class FindCandidate(rawValue: Any?) : View(rawValue) {
val `index`: BigInteger? get() = required("index")?.let { value -> Values.integer(value) }
val `input`: Any? get() = required("input")?.let { value -> Values.freeze(value) }
val `probability`: BigDecimal get() = required("probability").let { value -> value as BigDecimal }
val `source`: PhysicalSource? get() = optional("source")?.let { value -> PhysicalSource.read(value) }
companion object { fun read(value: Any?): FindCandidate = FindCandidate(value) }
}
class Image(rawValue: Any?) : View(rawValue) {
val `base64`: String get() = required("base64").let { value -> value as String }
val `height`: BigInteger get() = required("height").let { value -> Values.integer(value) }
val `media`: ImageMedia get() = required("media").let { value -> ImageMedia.read(value) }
val `width`: BigInteger get() = required("width").let { value -> Values.integer(value) }
companion object { fun read(value: Any?): Image = Image(value) }
}
class ImageMedia(rawValue: Any?) : View(rawValue) {
val value: String get() = json as String
companion object { fun read(value: Any?): ImageMedia = ImageMedia(value) }
}
sealed interface InputDeclaration { val json: Any?
companion object {
fun read(value: Any?): InputDeclaration {
val objectValue = Values.`object`(value)
if (objectValue["type"] == "string") return InputDeclarationString.read(value)
if (objectValue["type"] == "object") return InputDeclarationObject.read(value)
throw IllegalStateException("Unknown native result alternative")
}
}
}
class InputDeclarationObject(rawValue: Any?) : View(rawValue), InputDeclaration {
val `properties`: Map<String, InputPropertyType> get() = required("properties").let { value -> Values.map(value) { item0 -> InputPropertyType.read(item0) } }
val `required`: List<String>? get() = optional("required")?.let { value -> Values.list(value) { item0 -> item0 as String } }
val `type`: ObjectType get() = required("type").let { value -> ObjectType.read(value) }
companion object { fun read(value: Any?): InputDeclarationObject = InputDeclarationObject(value) }
}
class InputDeclarationString(rawValue: Any?) : View(rawValue), InputDeclaration {
val `type`: StringType get() = required("type").let { value -> StringType.read(value) }
companion object { fun read(value: Any?): InputDeclarationString = InputDeclarationString(value) }
}
sealed interface InputPropertyType { val json: Any?
companion object {
fun read(value: Any?): InputPropertyType {
val objectValue = Values.`object`(value)
if (objectValue["type"] == "string") return InputPropertyTypeString.read(value)
if (objectValue["type"] == "number") return InputPropertyTypeNumber.read(value)
if (objectValue["type"] == "boolean") return InputPropertyTypeBoolean.read(value)
if (objectValue["type"] == "array") return InputPropertyTypeArray.read(value)
throw IllegalStateException("Unknown native result alternative")
}
}
}
class InputPropertyTypeArray(rawValue: Any?) : View(rawValue), InputPropertyType {
val `items`: StringRoot get() = required("items").let { value -> StringRoot.read(value) }
val `type`: String get() = required("type").let { value -> value as String }
companion object { fun read(value: Any?): InputPropertyTypeArray = InputPropertyTypeArray(value) }
}
class InputPropertyTypeBoolean(rawValue: Any?) : View(rawValue), InputPropertyType {
val `type`: String get() = required("type").let { value -> value as String }
companion object { fun read(value: Any?): InputPropertyTypeBoolean = InputPropertyTypeBoolean(value) }
}
class InputPropertyTypeNumber(rawValue: Any?) : View(rawValue), InputPropertyType {
val `type`: String get() = required("type").let { value -> value as String }
companion object { fun read(value: Any?): InputPropertyTypeNumber = InputPropertyTypeNumber(value) }
}
class InputPropertyTypeString(rawValue: Any?) : View(rawValue), InputPropertyType {
val `type`: String get() = required("type").let { value -> value as String }
companion object { fun read(value: Any?): InputPropertyTypeString = InputPropertyTypeString(value) }
}
class Label(rawValue: Any?) : View(rawValue) {
val `description`: Any? get() = optional("description")?.let { value -> Values.freeze(value) }
val `name`: String get() = required("name").let { value -> value as String }
companion object { fun read(value: Any?): Label = Label(value) }
}
class Meta(rawValue: Any?) : View(rawValue) {
val `answeredBy`: String? get() = optional("answered_by")?.let { value -> value as String }
val `attempts`: List<Attempt>? get() = optional("attempts")?.let { value -> Values.list(value) { item0 -> Attempt.read(item0) } }
val `batchSetting`: BatchSetting? get() = optional("batch_setting")?.let { value -> BatchSetting.read(value) }
val `batchWarning`: BatchWarning? get() = optional("batch_warning")?.let { value -> BatchWarning.read(value) }
val `cached`: Boolean get() = required("cached").let { value -> value as Boolean }
val `contextSha256`: String? get() = optional("context_sha256")?.let { value -> value as String }
val `failedQuestions`: BigInteger get() = required("failed_questions").let { value -> Values.integer(value) }
val `model`: String get() = required("model").let { value -> value as String }
val `observations`: List<Observation> get() = required("observations").let { value -> Values.list(value) { item0 -> Observation.read(item0) } }
val `origin`: Origin? get() = required("origin")?.let { value -> Origin.read(value) }
val `profileWarning`: ProfileWarning? get() = optional("profile_warning")?.let { value -> ProfileWarning.read(value) }
val `questionSha256`: String? get() = optional("question_sha256")?.let { value -> value as String }
val `questionSources`: List<QuestionSource> get() = required("question_sources").let { value -> Values.list(value) { item0 -> QuestionSource.read(item0) } }
val `questionsSha256`: String? get() = optional("questions_sha256")?.let { value -> value as String }
val `requests`: List<String> get() = required("requests").let { value -> Values.list(value) { item0 -> item0 as String } }
val `requestsSent`: BigInteger get() = required("requests_sent").let { value -> Values.integer(value) }
val `tool`: String get() = required("tool").let { value -> value as String }
val `url`: String get() = required("url").let { value -> value as String }
val `usage`: Usage? get() = optional("usage")?.let { value -> Usage.read(value) }
companion object { fun read(value: Any?): Meta = Meta(value) }
}
class ObjectRoot(rawValue: Any?) : View(rawValue) {
val `properties`: Map<String, InputPropertyType> get() = required("properties").let { value -> Values.map(value) { item0 -> InputPropertyType.read(item0) } }
val `required`: List<String>? get() = optional("required")?.let { value -> Values.list(value) { item0 -> item0 as String } }
val `type`: ObjectType get() = required("type").let { value -> ObjectType.read(value) }
companion object { fun read(value: Any?): ObjectRoot = ObjectRoot(value) }
}
class ObjectType(rawValue: Any?) : View(rawValue) {
val value: String get() = json as String
companion object { fun read(value: Any?): ObjectType = ObjectType(value) }
}
sealed interface Observation { val json: Any?
companion object {
fun read(value: Any?): Observation {
val objectValue = Values.`object`(value)
if (objectValue.containsKey("observation_id")) return ObservationObservationId.read(value)
if (objectValue.containsKey("failure_id")) return ObservationFailureId.read(value)
throw IllegalStateException("Unknown native result alternative")
}
}
}
class ObservationId(rawValue: Any?) : View(rawValue) {
val value: String get() = json as String
companion object { fun read(value: Any?): ObservationId = ObservationId(value) }
}
class ObservationFailureId(rawValue: Any?) : View(rawValue), Observation {
val `failureId`: FailureId get() = required("failure_id").let { value -> FailureId.read(value) }
companion object { fun read(value: Any?): ObservationFailureId = ObservationFailureId(value) }
}
class ObservationObservationId(rawValue: Any?) : View(rawValue), Observation {
val `observationId`: ObservationId get() = required("observation_id").let { value -> ObservationId.read(value) }
companion object { fun read(value: Any?): ObservationObservationId = ObservationObservationId(value) }
}
class Origin(rawValue: Any?) : View(rawValue) {
val value: String get() = json as String
companion object { fun read(value: Any?): Origin = Origin(value) }
}
class PersistenceObservation(rawValue: Any?) : View(rawValue) {
val `advice`: String? get() = optional("advice")?.let { value -> value as String }
val `observedAt`: String get() = required("observed_at").let { value -> value as String }
val `state`: UsagePersistence get() = required("state").let { value -> UsagePersistence.read(value) }
companion object { fun read(value: Any?): PersistenceObservation = PersistenceObservation(value) }
}
class PhysicalSource(rawValue: Any?) : View(rawValue) {
val `file`: String get() = required("file").let { value -> value as String }
val `firstLine`: BigInteger? get() = optional("first_line")?.let { value -> Values.integer(value) }
val `lastLine`: BigInteger? get() = optional("last_line")?.let { value -> Values.integer(value) }
companion object { fun read(value: Any?): PhysicalSource = PhysicalSource(value) }
}
class Position(rawValue: Any?) : View(rawValue) {
val `file`: String? get() = required("file")?.let { value -> value as String }
val `first`: BigInteger? get() = optional("first")?.let { value -> Values.integer(value) }
val `images`: List<String>? get() = optional("images")?.let { value -> Values.list(value) { item0 -> item0 as String } }
val `last`: BigInteger? get() = optional("last")?.let { value -> Values.integer(value) }
companion object { fun read(value: Any?): Position = Position(value) }
}
class QuestionName(rawValue: Any?) : View(rawValue) {
val value: String get() = json as String
companion object { fun read(value: Any?): QuestionName = QuestionName(value) }
}
class QuestionSource(rawValue: Any?) : View(rawValue) {
val `answeredBy`: String get() = required("answered_by").let { value -> value as String }
val `batchSize`: BigInteger? get() = optional("batch_size")?.let { value -> Values.integer(value) }
val `origin`: Origin get() = required("origin").let { value -> Origin.read(value) }
companion object { fun read(value: Any?): QuestionSource = QuestionSource(value) }
}
class RankMember(rawValue: Any?) : View(rawValue) {
val `name`: String get() = required("name").let { value -> value as String }
val `result`: RankMemberResult get() = required("result").let { value -> RankMemberResult.read(value) }
companion object { fun read(value: Any?): RankMember = RankMember(value) }
}
class RankMemberResult(rawValue: Any?) : View(rawValue) {
val `answer`: Answer get() = required("answer").let { value -> Answer.read(value) }
val `answerId`: AnswerId get() = required("answer_id").let { value -> AnswerId.read(value) }
val `images`: List<Image>? get() = optional("images")?.let { value -> Values.list(value) { item0 -> Image.read(item0) } }
val `meta`: Meta get() = required("meta").let { value -> Meta.read(value) }
val `question`: ReadableQuestion get() = required("question").let { value -> ReadableQuestion.read(value) }
val `schema`: Version get() = required("schema").let { value -> Version.read(value) }
val `source`: PhysicalSource? get() = optional("source")?.let { value -> PhysicalSource.read(value) }
val `threshold`: Any? get() = required("threshold")?.let { value -> Values.freeze(value) }
val `value`: BigInteger get() = required("value").let { value -> Values.integer(value) }
companion object { fun read(value: Any?): RankMemberResult = RankMemberResult(value) }
}
sealed interface ReadableQuestion { val json: Any?
companion object {
fun read(value: Any?): ReadableQuestion {
val objectValue = Values.`object`(value)
if (objectValue["verb"] == "decide") return ReadableQuestionDecide.read(value)
if (objectValue["verb"] == "choose") return ReadableQuestionChoose.read(value)
if (objectValue["verb"] == "tag") return ReadableQuestionTag.read(value)
if (objectValue["verb"] == "score") return ReadableQuestionScore.read(value)
throw IllegalStateException("Unknown native result alternative")
}
}
}
class ReadableQuestion2(rawValue: Any?) : View(rawValue) {
val `batch`: Batch? get() = optional("batch")?.let { value -> Batch.read(value) }
val `contextSchema`: InputDeclaration? get() = optional("context_schema")?.let { value -> InputDeclaration.read(value) }
val `itemSchema`: InputDeclaration? get() = optional("item_schema")?.let { value -> InputDeclaration.read(value) }
val `labelDetails`: List<Label>? get() = optional("label_details")?.let { value -> Values.list(value) { item0 -> Label.read(item0) } }
val `model`: String? get() = optional("model")?.let { value -> value as String }
val `name`: QuestionName? get() = optional("name")?.let { value -> QuestionName.read(value) }
val `none`: Boolean get() = required("none").let { value -> value as Boolean }
val `on`: List<String>? get() = optional("on")?.let { value -> Values.list(value) { item0 -> item0 as String } }
val `profile`: String? get() = optional("profile")?.let { value -> value as String }
val `text`: Any? get() = required("text").let { value -> Values.freeze(value) }
val `verb`: String get() = required("verb").let { value -> value as String }
val `wordingVersion`: WordingVersion? get() = optional("wording_version")?.let { value -> WordingVersion.read(value) }
companion object { fun read(value: Any?): ReadableQuestion2 = ReadableQuestion2(value) }
}
class ReadableQuestion3(rawValue: Any?) : View(rawValue) {
val `batch`: Batch? get() = optional("batch")?.let { value -> Batch.read(value) }
val `contextSchema`: InputDeclaration? get() = optional("context_schema")?.let { value -> InputDeclaration.read(value) }
val `entityDefinition`: Any? get() = optional("entity_definition")?.let { value -> Values.freeze(value) }
val `instructions`: Any? get() = optional("instructions")?.let { value -> Values.freeze(value) }
val `itemSchema`: InputDeclaration? get() = optional("item_schema")?.let { value -> InputDeclaration.read(value) }
val `kinds`: Map<String, Any?> get() = required("kinds").let { value -> Values.map(value) { item0 -> Values.freeze(item0) } }
val `labelDetails`: List<Label>? get() = optional("label_details")?.let { value -> Values.list(value) { item0 -> Label.read(item0) } }
val `mode`: RecognitionMode? get() = optional("mode")?.let { value -> RecognitionMode.read(value) }
val `model`: String? get() = optional("model")?.let { value -> value as String }
val `name`: QuestionName? get() = optional("name")?.let { value -> QuestionName.read(value) }
val `on`: List<String>? get() = optional("on")?.let { value -> Values.list(value) { item0 -> item0 as String } }
val `profile`: String? get() = optional("profile")?.let { value -> value as String }
val `relationThreshold`: Threshold? get() = optional("relation_threshold")?.let { value -> Threshold.read(value) }
val `relations`: List<RelationRule>? get() = optional("relations")?.let { value -> Values.list(value) { item0 -> RelationRule.read(item0) } }
val `snippetPieces`: BigInteger? get() = optional("snippet_pieces")?.let { value -> Values.integer(value) }
val `stageContext`: RecognitionStageContext? get() = optional("stage_context")?.let { value -> RecognitionStageContext.read(value) }
val `threshold`: Threshold get() = required("threshold").let { value -> Threshold.read(value) }
val `verb`: Verb get() = required("verb").let { value -> Verb.read(value) }
val `wordingVersion`: WordingVersion? get() = optional("wording_version")?.let { value -> WordingVersion.read(value) }
companion object { fun read(value: Any?): ReadableQuestion3 = ReadableQuestion3(value) }
}
class ReadableQuestion4(rawValue: Any?) : View(rawValue) {
val `batch`: Batch? get() = optional("batch")?.let { value -> Batch.read(value) }
val `contextSchema`: InputDeclaration? get() = optional("context_schema")?.let { value -> InputDeclaration.read(value) }
val `fields`: RelateFields? get() = required("fields")?.let { value -> RelateFields.read(value) }
val `itemSchema`: InputDeclaration? get() = optional("item_schema")?.let { value -> InputDeclaration.read(value) }
val `labelDetails`: List<Label>? get() = optional("label_details")?.let { value -> Values.list(value) { item0 -> Label.read(item0) } }
val `model`: String? get() = optional("model")?.let { value -> value as String }
val `name`: QuestionName? get() = optional("name")?.let { value -> QuestionName.read(value) }
val `on`: List<String>? get() = optional("on")?.let { value -> Values.list(value) { item0 -> item0 as String } }
val `profile`: String? get() = optional("profile")?.let { value -> value as String }
val `relations`: List<RelationRule> get() = required("relations").let { value -> Values.list(value) { item0 -> RelationRule.read(item0) } }
val `threshold`: Threshold get() = required("threshold").let { value -> Threshold.read(value) }
val `verb`: String get() = required("verb").let { value -> value as String }
val `wordingVersion`: WordingVersion? get() = optional("wording_version")?.let { value -> WordingVersion.read(value) }
companion object { fun read(value: Any?): ReadableQuestion4 = ReadableQuestion4(value) }
}
class ReadableQuestionChoose(rawValue: Any?) : View(rawValue), ReadableQuestion {
val `batch`: Batch? get() = optional("batch")?.let { value -> Batch.read(value) }
val `contextSchema`: InputDeclaration? get() = optional("context_schema")?.let { value -> InputDeclaration.read(value) }
val `itemSchema`: InputDeclaration? get() = optional("item_schema")?.let { value -> InputDeclaration.read(value) }
val `labelDetails`: List<Label>? get() = optional("label_details")?.let { value -> Values.list(value) { item0 -> Label.read(item0) } }
val `model`: String? get() = optional("model")?.let { value -> value as String }
val `name`: QuestionName? get() = optional("name")?.let { value -> QuestionName.read(value) }
val `on`: List<String>? get() = optional("on")?.let { value -> Values.list(value) { item0 -> item0 as String } }
val `profile`: String? get() = optional("profile")?.let { value -> value as String }
val `wordingVersion`: WordingVersion? get() = optional("wording_version")?.let { value -> WordingVersion.read(value) }
val `options`: List<String> get() = required("options").let { value -> Values.list(value) { item0 -> item0 as String } }
val `text`: Any? get() = required("text").let { value -> Values.freeze(value) }
val `verb`: String get() = required("verb").let { value -> value as String }
companion object { fun read(value: Any?): ReadableQuestionChoose = ReadableQuestionChoose(value) }
}
class ReadableQuestionDecide(rawValue: Any?) : View(rawValue), ReadableQuestion {
val `batch`: Batch? get() = optional("batch")?.let { value -> Batch.read(value) }
val `contextSchema`: InputDeclaration? get() = optional("context_schema")?.let { value -> InputDeclaration.read(value) }
val `itemSchema`: InputDeclaration? get() = optional("item_schema")?.let { value -> InputDeclaration.read(value) }
val `labelDetails`: List<Label>? get() = optional("label_details")?.let { value -> Values.list(value) { item0 -> Label.read(item0) } }
val `model`: String? get() = optional("model")?.let { value -> value as String }
val `name`: QuestionName? get() = optional("name")?.let { value -> QuestionName.read(value) }
val `on`: List<String>? get() = optional("on")?.let { value -> Values.list(value) { item0 -> item0 as String } }
val `profile`: String? get() = optional("profile")?.let { value -> value as String }
val `wordingVersion`: WordingVersion? get() = optional("wording_version")?.let { value -> WordingVersion.read(value) }
val `falseValue`: Any? get() = optional("false")?.let { value -> Values.freeze(value) }
val `text`: Any? get() = required("text").let { value -> Values.freeze(value) }
val `trueValue`: Any? get() = optional("true")?.let { value -> Values.freeze(value) }
val `verb`: String get() = required("verb").let { value -> value as String }
companion object { fun read(value: Any?): ReadableQuestionDecide = ReadableQuestionDecide(value) }
}
class ReadableQuestionScore(rawValue: Any?) : View(rawValue), ReadableQuestion {
val `batch`: Batch? get() = optional("batch")?.let { value -> Batch.read(value) }
val `contextSchema`: InputDeclaration? get() = optional("context_schema")?.let { value -> InputDeclaration.read(value) }
val `itemSchema`: InputDeclaration? get() = optional("item_schema")?.let { value -> InputDeclaration.read(value) }
val `labelDetails`: List<Label>? get() = optional("label_details")?.let { value -> Values.list(value) { item0 -> Label.read(item0) } }
val `model`: String? get() = optional("model")?.let { value -> value as String }
val `name`: QuestionName? get() = optional("name")?.let { value -> QuestionName.read(value) }
val `on`: List<String>? get() = optional("on")?.let { value -> Values.list(value) { item0 -> item0 as String } }
val `profile`: String? get() = optional("profile")?.let { value -> value as String }
val `wordingVersion`: WordingVersion? get() = optional("wording_version")?.let { value -> WordingVersion.read(value) }
val `levels`: List<String> get() = required("levels").let { value -> Values.list(value) { item0 -> item0 as String } }
val `text`: Any? get() = required("text").let { value -> Values.freeze(value) }
val `verb`: String get() = required("verb").let { value -> value as String }
companion object { fun read(value: Any?): ReadableQuestionScore = ReadableQuestionScore(value) }
}
class ReadableQuestionTag(rawValue: Any?) : View(rawValue), ReadableQuestion {
val `batch`: Batch? get() = optional("batch")?.let { value -> Batch.read(value) }
val `contextSchema`: InputDeclaration? get() = optional("context_schema")?.let { value -> InputDeclaration.read(value) }
val `itemSchema`: InputDeclaration? get() = optional("item_schema")?.let { value -> InputDeclaration.read(value) }
val `labelDetails`: List<Label>? get() = optional("label_details")?.let { value -> Values.list(value) { item0 -> Label.read(item0) } }
val `model`: String? get() = optional("model")?.let { value -> value as String }
val `name`: QuestionName? get() = optional("name")?.let { value -> QuestionName.read(value) }
val `on`: List<String>? get() = optional("on")?.let { value -> Values.list(value) { item0 -> item0 as String } }
val `profile`: String? get() = optional("profile")?.let { value -> value as String }
val `wordingVersion`: WordingVersion? get() = optional("wording_version")?.let { value -> WordingVersion.read(value) }
val `labels`: List<String> get() = required("labels").let { value -> Values.list(value) { item0 -> item0 as String } }
val `text`: Any? get() = required("text").let { value -> Values.freeze(value) }
val `verb`: String get() = required("verb").let { value -> value as String }
companion object { fun read(value: Any?): ReadableQuestionTag = ReadableQuestionTag(value) }
}
class Recognition(rawValue: Any?) : View(rawValue) {
val `answer`: RecognitionOdds get() = required("answer").let { value -> RecognitionOdds.read(value) }
val `answerId`: AnswerId get() = required("answer_id").let { value -> AnswerId.read(value) }
val `file`: String? get() = optional("file")?.let { value -> value as String }
val `firstLine`: BigInteger? get() = optional("first_line")?.let { value -> Values.integer(value) }
val `index`: BigInteger? get() = optional("index")?.let { value -> Values.integer(value) }
val `input`: Any? get() = optional("input")?.let { value -> Values.freeze(value) }
val `lastLine`: BigInteger? get() = optional("last_line")?.let { value -> Values.integer(value) }
val `meta`: Meta get() = required("meta").let { value -> Meta.read(value) }
val `position`: Position? get() = optional("position")?.let { value -> Position.read(value) }
val `question`: ReadableQuestion3 get() = required("question").let { value -> ReadableQuestion3.read(value) }
val `schema`: Version get() = required("schema").let { value -> Version.read(value) }
val `source`: PhysicalSource? get() = optional("source")?.let { value -> PhysicalSource.read(value) }
val `value`: Recognize get() = required("value").let { value -> Recognize.read(value) }
companion object { fun read(value: Any?): Recognition = Recognition(value) }
}
class RecognitionEdgeDocument(rawValue: Any?) : View(rawValue) {
val `either`: Boolean get() = required("either").let { value -> value as Boolean }
val `probability`: BigDecimal get() = required("probability").let { value -> value as BigDecimal }
val `relation`: String get() = required("relation").let { value -> value as String }
val `source`: Entity get() = required("source").let { value -> Entity.read(value) }
val `target`: Entity get() = required("target").let { value -> Entity.read(value) }
companion object { fun read(value: Any?): RecognitionEdgeDocument = RecognitionEdgeDocument(value) }
}
class RecognitionMode(rawValue: Any?) : View(rawValue) {
val value: String get() = json as String
companion object { fun read(value: Any?): RecognitionMode = RecognitionMode(value) }
}
sealed interface RecognitionOdds { val json: Any?
companion object {
fun read(value: Any?): RecognitionOdds {
val objectValue = Values.`object`(value)
if (objectValue.containsKey("names") && objectValue.containsKey("pairs") && objectValue.containsKey("pieces") && objectValue.containsKey("proposals")) return RecognitionOddsFieldsNamesPairsPiecesProposals.read(value)
if (objectValue.containsKey("pieces") && objectValue.containsKey("proposals") && !objectValue.containsKey("names") && !objectValue.containsKey("pairs")) return RecognitionOddsFieldsPiecesProposals.read(value)
throw IllegalStateException("Unknown native result alternative")
}
}
}
class RecognitionOddsFieldsNamesPairsPiecesProposals(rawValue: Any?) : View(rawValue), RecognitionOdds {
val `names`: List<NameOdds> get() = required("names").let { value -> Values.list(value) { item0 -> NameOdds.read(item0) } }
val `pairs`: List<PairOdds> get() = required("pairs").let { value -> Values.list(value) { item0 -> PairOdds.read(item0) } }
val `pieces`: List<PieceOdds> get() = required("pieces").let { value -> Values.list(value) { item0 -> PieceOdds.read(item0) } }
val `proposals`: List<RecognitionProposal> get() = required("proposals").let { value -> Values.list(value) { item0 -> RecognitionProposal.read(item0) } }
companion object { fun read(value: Any?): RecognitionOddsFieldsNamesPairsPiecesProposals = RecognitionOddsFieldsNamesPairsPiecesProposals(value) }
}
class RecognitionOddsFieldsPiecesProposals(rawValue: Any?) : View(rawValue), RecognitionOdds {
val `pieces`: List<PieceOdds> get() = required("pieces").let { value -> Values.list(value) { item0 -> PieceOdds.read(item0) } }
val `proposals`: List<BoundaryProposal> get() = required("proposals").let { value -> Values.list(value) { item0 -> BoundaryProposal.read(item0) } }
companion object { fun read(value: Any?): RecognitionOddsFieldsPiecesProposals = RecognitionOddsFieldsPiecesProposals(value) }
}
class RecognitionProposal(rawValue: Any?) : View(rawValue) {
val `end`: BigInteger get() = required("end").let { value -> Values.integer(value) }
val `kept`: Boolean get() = required("kept").let { value -> value as Boolean }
val `kind`: String? get() = optional("kind")?.let { value -> value as String }
val `selected`: Place? get() = optional("selected")?.let { value -> Place.read(value) }
val `spanProbability`: BigDecimal get() = required("span_probability").let { value -> value as BigDecimal }
val `start`: BigInteger get() = required("start").let { value -> Values.integer(value) }
val `strength`: BigDecimal? get() = optional("strength")?.let { value -> value as BigDecimal }
companion object { fun read(value: Any?): RecognitionProposal = RecognitionProposal(value) }
}
class RecognitionStageContext(rawValue: Any?) : View(rawValue) {
val `boundary`: String? get() = optional("boundary")?.let { value -> value as String }
val `kindEdge`: String? get() = optional("kind_edge")?.let { value -> value as String }
val `relation`: String? get() = optional("relation")?.let { value -> value as String }
companion object { fun read(value: Any?): RecognitionStageContext = RecognitionStageContext(value) }
}
class Relation(rawValue: Any?) : View(rawValue) {
val `answer`: Answers get() = required("answer").let { value -> Answers.read(value) }
val `answerId`: AnswerId get() = required("answer_id").let { value -> AnswerId.read(value) }
val `file`: String? get() = optional("file")?.let { value -> value as String }
val `firstLine`: BigInteger? get() = optional("first_line")?.let { value -> Values.integer(value) }
val `index`: BigInteger? get() = optional("index")?.let { value -> Values.integer(value) }
val `input`: Any? get() = optional("input")?.let { value -> Values.freeze(value) }
val `inputSources`: List<SessionInputSource>? get() = optional("input_sources")?.let { value -> Values.list(value) { item0 -> SessionInputSource.read(item0) } }
val `lastLine`: BigInteger? get() = optional("last_line")?.let { value -> Values.integer(value) }
val `meta`: Meta get() = required("meta").let { value -> Meta.read(value) }
val `position`: Position? get() = optional("position")?.let { value -> Position.read(value) }
val `question`: ReadableQuestion4 get() = required("question").let { value -> ReadableQuestion4.read(value) }
val `schema`: Version get() = required("schema").let { value -> Version.read(value) }
val `value`: List<RelatedEntityEdge> get() = required("value").let { value -> Values.list(value) { item0 -> RelatedEntityEdge.read(item0) } }
companion object { fun read(value: Any?): Relation = Relation(value) }
}
class RelationDirection(rawValue: Any?) : View(rawValue) {
val value: String get() = json as String
companion object { fun read(value: Any?): RelationDirection = RelationDirection(value) }
}
sealed interface RelationMember { val json: Any?
companion object {
fun read(value: Any?): RelationMember {
val objectValue = Values.`object`(value)
if (objectValue.containsKey("answer_id")) return RelationMemberAnswerId.read(value)
if (objectValue.containsKey("failure_id")) return RelationMemberFailureId.read(value)
throw IllegalStateException("Unknown native result alternative")
}
}
}
class RelationMemberAnswerId(rawValue: Any?) : View(rawValue), RelationMember {
val `direction`: RelationDirection get() = required("direction").let { value -> RelationDirection.read(value) }
val `method`: RelationMethod get() = required("method").let { value -> RelationMethod.read(value) }
val `observations`: List<Observation> get() = required("observations").let { value -> Values.list(value) { item0 -> Observation.read(item0) } }
val `question`: ReadableQuestion get() = required("question").let { value -> ReadableQuestion.read(value) }
val `questionSources`: List<QuestionSource> get() = required("question_sources").let { value -> Values.list(value) { item0 -> QuestionSource.read(item0) } }
val `reads`: String get() = required("reads").let { value -> value as String }
val `relation`: String get() = required("relation").let { value -> value as String }
val `request`: String get() = required("request").let { value -> value as String }
val `source`: RelatedEntity get() = required("source").let { value -> RelatedEntity.read(value) }
val `target`: RelatedEntity? get() = required("target")?.let { value -> RelatedEntity.read(value) }
val `threshold`: Threshold get() = required("threshold").let { value -> Threshold.read(value) }
val `usage`: Usage? get() = optional("usage")?.let { value -> Usage.read(value) }
val `accepted`: Boolean get() = required("accepted").let { value -> value as Boolean }
val `answer`: Answer get() = required("answer").let { value -> Answer.read(value) }
val `answerId`: AnswerId get() = required("answer_id").let { value -> AnswerId.read(value) }
val `probability`: BigDecimal get() = required("probability").let { value -> value as BigDecimal }
companion object { fun read(value: Any?): RelationMemberAnswerId = RelationMemberAnswerId(value) }
}
class RelationMemberFailureId(rawValue: Any?) : View(rawValue), RelationMember {
val `direction`: RelationDirection get() = required("direction").let { value -> RelationDirection.read(value) }
val `method`: RelationMethod get() = required("method").let { value -> RelationMethod.read(value) }
val `observations`: List<Observation> get() = required("observations").let { value -> Values.list(value) { item0 -> Observation.read(item0) } }
val `question`: ReadableQuestion get() = required("question").let { value -> ReadableQuestion.read(value) }
val `questionSources`: List<QuestionSource> get() = required("question_sources").let { value -> Values.list(value) { item0 -> QuestionSource.read(item0) } }
val `reads`: String get() = required("reads").let { value -> value as String }
val `relation`: String get() = required("relation").let { value -> value as String }
val `request`: String get() = required("request").let { value -> value as String }
val `source`: RelatedEntity get() = required("source").let { value -> RelatedEntity.read(value) }
val `target`: RelatedEntity? get() = required("target")?.let { value -> RelatedEntity.read(value) }
val `threshold`: Threshold get() = required("threshold").let { value -> Threshold.read(value) }
val `usage`: Usage? get() = optional("usage")?.let { value -> Usage.read(value) }
val `failure`: Failure get() = required("failure").let { value -> Failure.read(value) }
val `failureId`: FailureId get() = required("failure_id").let { value -> FailureId.read(value) }
companion object { fun read(value: Any?): RelationMemberFailureId = RelationMemberFailureId(value) }
}
class RelationMethod(rawValue: Any?) : View(rawValue) {
val value: String get() = json as String
companion object { fun read(value: Any?): RelationMethod = RelationMethod(value) }
}
class RequestFunction(rawValue: Any?) : View(rawValue) {
val value: String get() = json as String
companion object { fun read(value: Any?): RequestFunction = RequestFunction(value) }
}
class SdkRequestId(rawValue: Any?) : View(rawValue) {
val value: String get() = json as String
companion object { fun read(value: Any?): SdkRequestId = SdkRequestId(value) }
}
sealed interface SendBudgetDenial { val json: Any?
companion object {
fun read(value: Any?): SendBudgetDenial {
val objectValue = Values.`object`(value)
if (objectValue["kind"] == "before_first_send") return SendBudgetDenialBeforeFirstSend.read(value)
if (objectValue["kind"] == "before_additional_send") return SendBudgetDenialBeforeAdditionalSend.read(value)
if (objectValue["kind"] == "before_retry") return SendBudgetDenialBeforeRetry.read(value)
throw IllegalStateException("Unknown native result alternative")
}
}
}
class SendBudgetDenialBeforeAdditionalSend(rawValue: Any?) : View(rawValue), SendBudgetDenial {
val `kind`: String get() = required("kind").let { value -> value as String }
companion object { fun read(value: Any?): SendBudgetDenialBeforeAdditionalSend = SendBudgetDenialBeforeAdditionalSend(value) }
}
class SendBudgetDenialBeforeFirstSend(rawValue: Any?) : View(rawValue), SendBudgetDenial {
val `kind`: String get() = required("kind").let { value -> value as String }
companion object { fun read(value: Any?): SendBudgetDenialBeforeFirstSend = SendBudgetDenialBeforeFirstSend(value) }
}
class SendBudgetDenialBeforeRetry(rawValue: Any?) : View(rawValue), SendBudgetDenial {
val `kind`: String get() = required("kind").let { value -> value as String }
val `lastStatus`: BigInteger get() = required("last_status").let { value -> Values.integer(value) }
companion object { fun read(value: Any?): SendBudgetDenialBeforeRetry = SendBudgetDenialBeforeRetry(value) }
}
class StopCause(rawValue: Any?) : View(rawValue) {
val value: String get() = json as String
companion object { fun read(value: Any?): StopCause = StopCause(value) }
}
class Stopped(rawValue: Any?) : View(rawValue) {
val `at`: BigInteger? get() = optional("at")?.let { value -> Values.integer(value) }
val `cause`: StopCause get() = required("cause").let { value -> StopCause.read(value) }
val `retryable`: Boolean get() = required("retryable").let { value -> value as Boolean }
val `status`: BigInteger? get() = optional("status")?.let { value -> Values.integer(value) }
companion object { fun read(value: Any?): Stopped = Stopped(value) }
}
class StringRoot(rawValue: Any?) : View(rawValue) {
val `type`: StringType get() = required("type").let { value -> StringType.read(value) }
companion object { fun read(value: Any?): StringRoot = StringRoot(value) }
}
class StringType(rawValue: Any?) : View(rawValue) {
val value: String get() = json as String
companion object { fun read(value: Any?): StringType = StringType(value) }
}
class Usage(rawValue: Any?) : View(rawValue) {
val `inputTokens`: BigInteger? get() = optional("input_tokens")?.let { value -> Values.integer(value) }
val `outputTokens`: BigInteger? get() = optional("output_tokens")?.let { value -> Values.integer(value) }
companion object { fun read(value: Any?): Usage = Usage(value) }
}
class UsagePersistence(rawValue: Any?) : View(rawValue) {
val value: String get() = json as String
companion object { fun read(value: Any?): UsagePersistence = UsagePersistence(value) }
}
class Verb(rawValue: Any?) : View(rawValue) {
val value: String get() = json as String
companion object { fun read(value: Any?): Verb = Verb(value) }
}
class Version(rawValue: Any?) : View(rawValue) {
val value: String get() = json as String
companion object { fun read(value: Any?): Version = Version(value) }
}
class WordingVersion(rawValue: Any?) : View(rawValue) {
val value: BigInteger get() = Values.integer(json)
companion object { fun read(value: Any?): WordingVersion = WordingVersion(value) }
}
sealed interface AnnotatedField { val json: Any?
companion object {
fun read(value: Any?): AnnotatedField {
if (value is Boolean) return AnnotatedFieldBoolean.read(value)
if (value == null) return AnnotatedFieldNull.read(value)
if (value is String) return AnnotatedFieldString.read(value)
if (value is List<*>) return AnnotatedFieldArray.read(value)
if (value is java.math.BigDecimal) return AnnotatedFieldNumber.read(value)
if (value is Map<*, *>) return AnnotatedFieldObject.read(value)
throw IllegalStateException("Unknown native result alternative")
}
}
}
class AnnotatedFieldBoolean(override val json: Any?) : AnnotatedField { val value: Boolean get() = json as Boolean
companion object { fun read(value: Any?): AnnotatedFieldBoolean = AnnotatedFieldBoolean(Values.freeze(value)) }
}
class AnnotatedFieldNull(override val json: Any?) : AnnotatedField { val value: Any? get() = Values.freeze(json)
companion object { fun read(value: Any?): AnnotatedFieldNull = AnnotatedFieldNull(Values.freeze(value)) }
}
class AnnotatedFieldString(override val json: Any?) : AnnotatedField { val value: String get() = json as String
companion object { fun read(value: Any?): AnnotatedFieldString = AnnotatedFieldString(Values.freeze(value)) }
}
class AnnotatedFieldArray(override val json: Any?) : AnnotatedField { val value: List<String> get() = Values.list(json) { item0 -> item0 as String }
companion object { fun read(value: Any?): AnnotatedFieldArray = AnnotatedFieldArray(Values.freeze(value)) }
}
class AnnotatedFieldNumber(override val json: Any?) : AnnotatedField { val value: BigDecimal get() = json as BigDecimal
companion object { fun read(value: Any?): AnnotatedFieldNumber = AnnotatedFieldNumber(Values.freeze(value)) }
}
class AnnotatedFieldObject(override val json: Any?) : AnnotatedField { val value: Failed get() = Failed.read(json)
companion object { fun read(value: Any?): AnnotatedFieldObject = AnnotatedFieldObject(Values.freeze(value)) }
}
class AnnotatedRow(rawValue: Any?) : View(rawValue) {
val value: Map<String, AnnotatedField> get() = Values.map(json) { item0 -> AnnotatedField.read(item0) }
companion object { fun read(value: Any?): AnnotatedRow = AnnotatedRow(value) }
}
sealed interface Answer { val json: Any?
companion object {
fun read(value: Any?): Answer {
val objectValue = Values.`object`(value)
if (objectValue["kind"] == "yes_no") return AnswerYesNo.read(value)
if (objectValue["kind"] == "choice") return AnswerChoice.read(value)
if (objectValue["kind"] == "tag") return AnswerTag.read(value)
if (objectValue["kind"] == "score") return AnswerScore.read(value)
throw IllegalStateException("Unknown native result alternative")
}
}
}
class AnswerChoice(rawValue: Any?) : View(rawValue), Answer {
val `confidence`: BigDecimal? get() = optional("confidence")?.let { value -> value as BigDecimal }
val `kind`: String get() = required("kind").let { value -> value as String }
val `pick`: String get() = required("pick").let { value -> value as String }
val `probabilities`: Map<String, BigDecimal> get() = required("probabilities").let { value -> Values.map(value) { item0 -> item0 as BigDecimal } }
companion object { fun read(value: Any?): AnswerChoice = AnswerChoice(value) }
}
class AnswerScore(rawValue: Any?) : View(rawValue), Answer {
val `confidence`: BigDecimal? get() = optional("confidence")?.let { value -> value as BigDecimal }
val `kind`: String get() = required("kind").let { value -> value as String }
val `level`: String get() = required("level").let { value -> value as String }
val `probabilities`: Map<String, BigDecimal> get() = required("probabilities").let { value -> Values.map(value) { item0 -> item0 as BigDecimal } }
companion object { fun read(value: Any?): AnswerScore = AnswerScore(value) }
}
class AnswerTag(rawValue: Any?) : View(rawValue), Answer {
val `kind`: String get() = required("kind").let { value -> value as String }
val `probabilities`: Map<String, BigDecimal> get() = required("probabilities").let { value -> Values.map(value) { item0 -> item0 as BigDecimal } }
companion object { fun read(value: Any?): AnswerTag = AnswerTag(value) }
}
class AnswerYesNo(rawValue: Any?) : View(rawValue), Answer {
val `kind`: String get() = required("kind").let { value -> value as String }
val `probability`: BigDecimal get() = required("probability").let { value -> value as BigDecimal }
companion object { fun read(value: Any?): AnswerYesNo = AnswerYesNo(value) }
}
class AttemptOutcome(rawValue: Any?) : View(rawValue) {
val value: String get() = json as String
companion object { fun read(value: Any?): AttemptOutcome = AttemptOutcome(value) }
}
sealed interface BatchSetting { val json: Any?
companion object {
fun read(value: Any?): BatchSetting {
if (value is java.math.BigDecimal) return BatchSettingInteger.read(value)
if (value is String) return BatchSettingString.read(value)
throw IllegalStateException("Unknown native result alternative")
}
}
}
class BatchSettingInteger(override val json: Any?) : BatchSetting { val value: BigInteger get() = Values.integer(json)
companion object { fun read(value: Any?): BatchSettingInteger = BatchSettingInteger(Values.freeze(value)) }
}
class BatchSettingString(override val json: Any?) : BatchSetting { val value: String get() = json as String
companion object { fun read(value: Any?): BatchSettingString = BatchSettingString(Values.freeze(value)) }
}
class BatchWarning(rawValue: Any?) : View(rawValue) {
val `running`: BatchSetting get() = required("running").let { value -> BatchSetting.read(value) }
val `tunedFor`: BatchSetting get() = required("tuned_for").let { value -> BatchSetting.read(value) }
companion object { fun read(value: Any?): BatchWarning = BatchWarning(value) }
}
class Entity(rawValue: Any?) : View(rawValue) {
val `end`: BigInteger get() = required("end").let { value -> Values.integer(value) }
val `file`: String? get() = optional("file")?.let { value -> value as String }
val `firstLine`: BigInteger? get() = optional("first_line")?.let { value -> Values.integer(value) }
val `kind`: String get() = required("kind").let { value -> value as String }
val `lastLine`: BigInteger? get() = optional("last_line")?.let { value -> Values.integer(value) }
val `length`: BigInteger get() = required("length").let { value -> Values.integer(value) }
val `start`: BigInteger get() = required("start").let { value -> Values.integer(value) }
val `strength`: BigDecimal get() = required("strength").let { value -> value as BigDecimal }
val `text`: String get() = required("text").let { value -> value as String }
companion object { fun read(value: Any?): Entity = Entity(value) }
}
class EntityEdge(rawValue: Any?) : View(rawValue) {
val `either`: Boolean? get() = optional("either")?.let { value -> value as Boolean }
val `probability`: BigDecimal get() = required("probability").let { value -> value as BigDecimal }
val `relation`: String get() = required("relation").let { value -> value as String }
val `source`: Entity get() = required("source").let { value -> Entity.read(value) }
val `target`: Entity get() = required("target").let { value -> Entity.read(value) }
companion object { fun read(value: Any?): EntityEdge = EntityEdge(value) }
}
class Failed(rawValue: Any?) : View(rawValue) {
val `failed`: Failure get() = required("failed").let { value -> Failure.read(value) }
companion object { fun read(value: Any?): Failed = Failed(value) }
}
class Failure(rawValue: Any?) : View(rawValue) {
val `cause`: FailureCause get() = required("cause").let { value -> FailureCause.read(value) }
val `kind`: String get() = required("kind").let { value -> value as String }
companion object { fun read(value: Any?): Failure = Failure(value) }
}
class FailureCause(rawValue: Any?) : View(rawValue) {
val value: String get() = json as String
companion object { fun read(value: Any?): FailureCause = FailureCause(value) }
}
class FailureKind(rawValue: Any?) : View(rawValue) {
val value: String get() = json as String
companion object { fun read(value: Any?): FailureKind = FailureKind(value) }
}
class FindAnswer(rawValue: Any?) : View(rawValue) {
val `confidence`: BigDecimal? get() = optional("confidence")?.let { value -> value as BigDecimal }
val `kind`: String get() = required("kind").let { value -> value as String }
val `pick`: String get() = required("pick").let { value -> value as String }
val `probabilities`: Map<String, BigDecimal> get() = required("probabilities").let { value -> Values.map(value) { item0 -> item0 as BigDecimal } }
companion object { fun read(value: Any?): FindAnswer = FindAnswer(value) }
}
class NameOdds(rawValue: Any?) : View(rawValue) {
val `edges`: Map<String, BigDecimal>? get() = required("edges")?.let { value -> Values.map(value) { item0 -> item0 as BigDecimal } }
val `end`: BigInteger get() = required("end").let { value -> Values.integer(value) }
val `kinds`: Map<String, BigDecimal>? get() = required("kinds")?.let { value -> Values.map(value) { item0 -> item0 as BigDecimal } }
val `start`: BigInteger get() = required("start").let { value -> Values.integer(value) }
companion object { fun read(value: Any?): NameOdds = NameOdds(value) }
}
class PairOdds(rawValue: Any?) : View(rawValue) {
val `probability`: BigDecimal get() = required("probability").let { value -> value as BigDecimal }
val `relation`: String get() = required("relation").let { value -> value as String }
val `source`: Place get() = required("source").let { value -> Place.read(value) }
val `target`: Place get() = required("target").let { value -> Place.read(value) }
companion object { fun read(value: Any?): PairOdds = PairOdds(value) }
}
class PieceOdds(rawValue: Any?) : View(rawValue) {
val `end`: BigInteger get() = required("end").let { value -> Values.integer(value) }
val `start`: BigInteger get() = required("start").let { value -> Values.integer(value) }
val `tags`: Map<String, BigDecimal> get() = required("tags").let { value -> Values.map(value) { item0 -> item0 as BigDecimal } }
companion object { fun read(value: Any?): PieceOdds = PieceOdds(value) }
}
class Place(rawValue: Any?) : View(rawValue) {
val `end`: BigInteger get() = required("end").let { value -> Values.integer(value) }
val `start`: BigInteger get() = required("start").let { value -> Values.integer(value) }
companion object { fun read(value: Any?): Place = Place(value) }
}
class Plan(rawValue: Any?) : View(rawValue) {
val `estimatedBytes`: BigInteger get() = required("estimated_bytes").let { value -> Values.integer(value) }
val `estimatedInputTokens`: TokenBand get() = required("estimated_input_tokens").let { value -> TokenBand.read(value) }
val `firstBodyUtf8`: String? get() = required("first_body_utf8")?.let { value -> value as String }
val `largestRequestBytes`: BigInteger get() = required("largest_request_bytes").let { value -> Values.integer(value) }
val `largestRequestEstimatedInputTokens`: BigInteger get() = required("largest_request_estimated_input_tokens").let { value -> Values.integer(value) }
val `records`: BigInteger get() = required("records").let { value -> Values.integer(value) }
val `requests`: BigInteger get() = required("requests").let { value -> Values.integer(value) }
val `tokenEstimateMethod`: String get() = required("token_estimate_method").let { value -> value as String }
val `upperBound`: Boolean get() = required("upper_bound").let { value -> value as Boolean }
companion object { fun read(value: Any?): Plan = Plan(value) }
}
class ProfileWarning(rawValue: Any?) : View(rawValue) {
val `running`: String get() = required("running").let { value -> value as String }
val `tunedFor`: String get() = required("tuned_for").let { value -> value as String }
companion object { fun read(value: Any?): ProfileWarning = ProfileWarning(value) }
}
sealed interface Recognize { val json: Any?
companion object {
fun read(value: Any?): Recognize {
val objectValue = Values.`object`(value)
if (objectValue.containsKey("entities") && !objectValue.containsKey("mode") && !objectValue.containsKey("proposals")) return RecognizeFieldsEntities.read(value)
if (objectValue.containsKey("mode") && objectValue.containsKey("proposals") && !objectValue.containsKey("entities")) return RecognizeFieldsModeProposals.read(value)
throw IllegalStateException("Unknown native result alternative")
}
}
}
class RecognizeAnswer(rawValue: Any?) : View(rawValue) {
val `names`: List<NameOdds> get() = required("names").let { value -> Values.list(value) { item0 -> NameOdds.read(item0) } }
val `pairs`: List<PairOdds> get() = required("pairs").let { value -> Values.list(value) { item0 -> PairOdds.read(item0) } }
val `pieces`: List<PieceOdds> get() = required("pieces").let { value -> Values.list(value) { item0 -> PieceOdds.read(item0) } }
val `proposals`: List<RecognitionProposal> get() = required("proposals").let { value -> Values.list(value) { item0 -> RecognitionProposal.read(item0) } }
companion object { fun read(value: Any?): RecognizeAnswer = RecognizeAnswer(value) }
}
class RecognizeFieldsEntities(rawValue: Any?) : View(rawValue), Recognize {
val `entities`: List<Entity> get() = required("entities").let { value -> Values.list(value) { item0 -> Entity.read(item0) } }
val `relations`: List<EntityEdge>? get() = optional("relations")?.let { value -> Values.list(value) { item0 -> EntityEdge.read(item0) } }
companion object { fun read(value: Any?): RecognizeFieldsEntities = RecognizeFieldsEntities(value) }
}
class RecognizeFieldsModeProposals(rawValue: Any?) : View(rawValue), Recognize {
val `mode`: BoundaryMode get() = required("mode").let { value -> BoundaryMode.read(value) }
val `proposals`: List<BoundaryProposal> get() = required("proposals").let { value -> Values.list(value) { item0 -> BoundaryProposal.read(item0) } }
companion object { fun read(value: Any?): RecognizeFieldsModeProposals = RecognizeFieldsModeProposals(value) }
}
class RelateFields(rawValue: Any?) : View(rawValue) {
val `kind`: String get() = required("kind").let { value -> value as String }
val `name`: String get() = required("name").let { value -> value as String }
companion object { fun read(value: Any?): RelateFields = RelateFields(value) }
}
class RelatedEntity(rawValue: Any?) : View(rawValue) {
val `kind`: String get() = required("kind").let { value -> value as String }
val `name`: String get() = required("name").let { value -> value as String }
companion object { fun read(value: Any?): RelatedEntity = RelatedEntity(value) }
}
class RelatedEntityEdge(rawValue: Any?) : View(rawValue) {
val `either`: Boolean? get() = optional("either")?.let { value -> value as Boolean }
val `probability`: BigDecimal get() = required("probability").let { value -> value as BigDecimal }
val `relation`: String get() = required("relation").let { value -> value as String }
val `source`: RelatedEntityEdgePropertiesSource get() = required("source").let { value -> RelatedEntityEdgePropertiesSource.read(value) }
val `target`: RelatedEntityEdgePropertiesSource get() = required("target").let { value -> RelatedEntityEdgePropertiesSource.read(value) }
companion object { fun read(value: Any?): RelatedEntityEdge = RelatedEntityEdge(value) }
}
sealed interface RelatedEntityEdgePropertiesSource { val json: Any?
companion object {
fun read(value: Any?): RelatedEntityEdgePropertiesSource {
val objectValue = Values.`object`(value)
if (objectValue.containsKey("kind") && objectValue.containsKey("name") && !objectValue.containsKey("file") && !objectValue.containsKey("ordinal") && !objectValue.containsKey("record")) return RelatedEntityEdgePropertiesSourceFieldsKindName.read(value)
if (objectValue.containsKey("file") && objectValue.containsKey("kind") && objectValue.containsKey("name") && objectValue.containsKey("ordinal") && objectValue.containsKey("record")) return RelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord.read(value)
throw IllegalStateException("Unknown native result alternative")
}
}
}
class RelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord(rawValue: Any?) : View(rawValue), RelatedEntityEdgePropertiesSource {
val `file`: String? get() = required("file")?.let { value -> value as String }
val `firstLine`: BigInteger? get() = optional("first_line")?.let { value -> Values.integer(value) }
val `kind`: String get() = required("kind").let { value -> value as String }
val `lastLine`: BigInteger? get() = optional("last_line")?.let { value -> Values.integer(value) }
val `name`: String get() = required("name").let { value -> value as String }
val `ordinal`: BigInteger get() = required("ordinal").let { value -> Values.integer(value) }
val `record`: Any? get() = required("record").let { value -> Values.freeze(value) }
companion object { fun read(value: Any?): RelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord = RelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord(value) }
}
class RelatedEntityEdgePropertiesSourceFieldsKindName(rawValue: Any?) : View(rawValue), RelatedEntityEdgePropertiesSource {
val `kind`: String get() = required("kind").let { value -> value as String }
val `name`: String get() = required("name").let { value -> value as String }
companion object { fun read(value: Any?): RelatedEntityEdgePropertiesSourceFieldsKindName = RelatedEntityEdgePropertiesSourceFieldsKindName(value) }
}
class RelationRule(rawValue: Any?) : View(rawValue) {
val `either`: Boolean get() = required("either").let { value -> value as Boolean }
val `name`: String get() = required("name").let { value -> value as String }
val `reads`: String get() = required("reads").let { value -> value as String }
val `single`: Boolean? get() = optional("single")?.let { value -> value as Boolean }
val `source`: String get() = required("source").let { value -> value as String }
val `target`: String get() = required("target").let { value -> value as String }
companion object { fun read(value: Any?): RelationRule = RelationRule(value) }
}
class SessionAnnotation(rawValue: Any?) : View(rawValue) {
val `name`: String get() = required("name").let { value -> value as String }
val `value`: AnnotationValue get() = required("value").let { value -> AnnotationValue.read(value) }
companion object { fun read(value: Any?): SessionAnnotation = SessionAnnotation(value) }
}
class SessionInputSource(rawValue: Any?) : View(rawValue) {
val `index`: BigInteger get() = required("index").let { value -> Values.integer(value) }
val `source`: PhysicalSource get() = required("source").let { value -> PhysicalSource.read(value) }
companion object { fun read(value: Any?): SessionInputSource = SessionInputSource(value) }
}
sealed interface SessionJudgment { val json: Any?
companion object {
fun read(value: Any?): SessionJudgment {
val objectValue = Values.`object`(value)
if (objectValue["kind"] == "decision") return SessionJudgmentDecision.read(value)
if (objectValue["kind"] == "choice") return SessionJudgmentChoice.read(value)
if (objectValue["kind"] == "score") return SessionJudgmentScore.read(value)
if (objectValue["kind"] == "tags") return SessionJudgmentTags.read(value)
throw IllegalStateException("Unknown native result alternative")
}
}
}
class SessionJudgmentChoice(rawValue: Any?) : View(rawValue), SessionJudgment {
val `kind`: String get() = required("kind").let { value -> value as String }
val `value`: String? get() = required("value")?.let { value -> value as String }
companion object { fun read(value: Any?): SessionJudgmentChoice = SessionJudgmentChoice(value) }
}
class SessionJudgmentDecision(rawValue: Any?) : View(rawValue), SessionJudgment {
val `kind`: String get() = required("kind").let { value -> value as String }
val `value`: Boolean? get() = required("value")?.let { value -> value as Boolean }
companion object { fun read(value: Any?): SessionJudgmentDecision = SessionJudgmentDecision(value) }
}
class SessionJudgmentScore(rawValue: Any?) : View(rawValue), SessionJudgment {
val `kind`: String get() = required("kind").let { value -> value as String }
val `value`: BigDecimal get() = required("value").let { value -> value as BigDecimal }
companion object { fun read(value: Any?): SessionJudgmentScore = SessionJudgmentScore(value) }
}
class SessionJudgmentTags(rawValue: Any?) : View(rawValue), SessionJudgment {
val `kind`: String get() = required("kind").let { value -> value as String }
val `value`: List<String> get() = required("value").let { value -> Values.list(value) { item0 -> item0 as String } }
companion object { fun read(value: Any?): SessionJudgmentTags = SessionJudgmentTags(value) }
}
class SessionNamedProbability(rawValue: Any?) : View(rawValue) {
val `name`: String get() = required("name").let { value -> value as String }
val `probability`: BigDecimal get() = required("probability").let { value -> value as BigDecimal }
companion object { fun read(value: Any?): SessionNamedProbability = SessionNamedProbability(value) }
}
sealed interface SessionObservation { val json: Any?
companion object {
fun read(value: Any?): SessionObservation {
val objectValue = Values.`object`(value)
if (objectValue["kind"] == "question") return SessionObservationQuestion.read(value)
if (objectValue["kind"] == "row") return SessionObservationRow.read(value)
throw IllegalStateException("Unknown native result alternative")
}
}
}
class SessionObservationQuestion(rawValue: Any?) : View(rawValue), SessionObservation {
val `detail`: SessionQuestionDetail get() = required("detail").let { value -> SessionQuestionDetail.read(value) }
val `index`: BigInteger get() = required("index").let { value -> Values.integer(value) }
val `kind`: String get() = required("kind").let { value -> value as String }
val `member`: String? get() = optional("member")?.let { value -> value as String }
val `position`: BigInteger get() = required("position").let { value -> Values.integer(value) }
val `stage`: String? get() = optional("stage")?.let { value -> value as String }
companion object { fun read(value: Any?): SessionObservationQuestion = SessionObservationQuestion(value) }
}
class SessionObservationRow(rawValue: Any?) : View(rawValue), SessionObservation {
val `index`: BigInteger get() = required("index").let { value -> Values.integer(value) }
val `kind`: String get() = required("kind").let { value -> value as String }
val `value`: SessionObservedRow get() = required("value").let { value -> SessionObservedRow.read(value) }
companion object { fun read(value: Any?): SessionObservationRow = SessionObservationRow(value) }
}
sealed interface SessionObservedRow { val json: Any?
companion object {
fun read(value: Any?): SessionObservedRow {
val objectValue = Values.`object`(value)
if (objectValue["kind"] == "judgment") return SessionObservedRowJudgment.read(value)
if (objectValue["kind"] == "annotated") return SessionObservedRowAnnotated.read(value)
if (objectValue["kind"] == "recognized") return SessionObservedRowRecognized.read(value)
if (objectValue["kind"] == "find") return SessionObservedRowFind.read(value)
if (objectValue["kind"] == "relations") return SessionObservedRowRelations.read(value)
throw IllegalStateException("Unknown native result alternative")
}
}
}
class SessionObservedRowAnnotated(rawValue: Any?) : View(rawValue), SessionObservedRow {
val `kind`: String get() = required("kind").let { value -> value as String }
val `value`: List<SessionAnnotation> get() = required("value").let { value -> Values.list(value) { item0 -> SessionAnnotation.read(item0) } }
companion object { fun read(value: Any?): SessionObservedRowAnnotated = SessionObservedRowAnnotated(value) }
}
class SessionObservedRowFind(rawValue: Any?) : View(rawValue), SessionObservedRow {
val `kind`: String get() = required("kind").let { value -> value as String }
val `value`: BigInteger? get() = required("value")?.let { value -> Values.integer(value) }
companion object { fun read(value: Any?): SessionObservedRowFind = SessionObservedRowFind(value) }
}
class SessionObservedRowJudgment(rawValue: Any?) : View(rawValue), SessionObservedRow {
val `kind`: String get() = required("kind").let { value -> value as String }
val `value`: SessionJudgment get() = required("value").let { value -> SessionJudgment.read(value) }
companion object { fun read(value: Any?): SessionObservedRowJudgment = SessionObservedRowJudgment(value) }
}
class SessionObservedRowRecognized(rawValue: Any?) : View(rawValue), SessionObservedRow {
val `kind`: String get() = required("kind").let { value -> value as String }
val `value`: SessionRecognition get() = required("value").let { value -> SessionRecognition.read(value) }
companion object { fun read(value: Any?): SessionObservedRowRecognized = SessionObservedRowRecognized(value) }
}
class SessionObservedRowRelations(rawValue: Any?) : View(rawValue), SessionObservedRow {
val `kind`: String get() = required("kind").let { value -> value as String }
val `value`: List<SessionRelationEdge> get() = required("value").let { value -> Values.list(value) { item0 -> SessionRelationEdge.read(item0) } }
companion object { fun read(value: Any?): SessionObservedRowRelations = SessionObservedRowRelations(value) }
}
sealed interface SessionPacket { val json: Any?
companion object {
fun read(value: Any?): SessionPacket {
val objectValue = Values.`object`(value)
if (objectValue["function"] == "decide" && objectValue["kind"] == "row") return SessionPacketDecideRow.read(value)
if (objectValue["function"] == "choose" && objectValue["kind"] == "row") return SessionPacketChooseRow.read(value)
if (objectValue["function"] == "tag" && objectValue["kind"] == "row") return SessionPacketTagRow.read(value)
if (objectValue["function"] == "score" && objectValue["kind"] == "row") return SessionPacketScoreRow.read(value)
if (objectValue["function"] == "filter" && objectValue["kind"] == "row") return SessionPacketFilterRow.read(value)
if (objectValue["function"] == "annotate" && objectValue["kind"] == "row") return SessionPacketAnnotateRow.read(value)
if (objectValue["function"] == "decide" && objectValue["kind"] == "aggregate") return SessionPacketDecideAggregate.read(value)
if (objectValue["function"] == "choose" && objectValue["kind"] == "aggregate") return SessionPacketChooseAggregate.read(value)
if (objectValue["function"] == "tag" && objectValue["kind"] == "aggregate") return SessionPacketTagAggregate.read(value)
if (objectValue["function"] == "score" && objectValue["kind"] == "aggregate") return SessionPacketScoreAggregate.read(value)
if (objectValue["function"] == "filter" && objectValue["kind"] == "aggregate") return SessionPacketFilterAggregate.read(value)
if (objectValue["function"] == "rank" && objectValue["kind"] == "aggregate") return SessionPacketRankAggregate.read(value)
if (objectValue["function"] == "find" && objectValue["kind"] == "aggregate") return SessionPacketFindAggregate.read(value)
if (objectValue["function"] == "annotate" && objectValue["kind"] == "aggregate") return SessionPacketAnnotateAggregate.read(value)
if (objectValue["function"] == "recognize" && objectValue["kind"] == "aggregate") return SessionPacketRecognizeAggregate.read(value)
if (objectValue["function"] == "relate" && objectValue["kind"] == "aggregate") return SessionPacketRelateAggregate.read(value)
if (objectValue["kind"] == "observation") return SessionPacketObservation.read(value)
if (objectValue["kind"] == "terminal") return SessionPacketTerminal.read(value)
throw IllegalStateException("Unknown native result alternative")
}
}
}
class SessionPacketAnnotateAggregate(rawValue: Any?) : View(rawValue), SessionPacket {
val `function`: String get() = required("function").let { value -> value as String }
val `kind`: String get() = required("kind").let { value -> value as String }
val `value`: List<Annotation> get() = required("value").let { value -> Values.list(value) { item0 -> Annotation.read(item0) } }
companion object { fun read(value: Any?): SessionPacketAnnotateAggregate = SessionPacketAnnotateAggregate(value) }
}
class SessionPacketAnnotateRow(rawValue: Any?) : View(rawValue), SessionPacket {
val `function`: String get() = required("function").let { value -> value as String }
val `kind`: String get() = required("kind").let { value -> value as String }
val `value`: Annotation get() = required("value").let { value -> Annotation.read(value) }
companion object { fun read(value: Any?): SessionPacketAnnotateRow = SessionPacketAnnotateRow(value) }
}
class SessionPacketChooseAggregate(rawValue: Any?) : View(rawValue), SessionPacket {
val `function`: String get() = required("function").let { value -> value as String }
val `kind`: String get() = required("kind").let { value -> value as String }
val `value`: List<AtomicNullableString> get() = required("value").let { value -> Values.list(value) { item0 -> AtomicNullableString.read(item0) } }
companion object { fun read(value: Any?): SessionPacketChooseAggregate = SessionPacketChooseAggregate(value) }
}
class SessionPacketChooseRow(rawValue: Any?) : View(rawValue), SessionPacket {
val `function`: String get() = required("function").let { value -> value as String }
val `kind`: String get() = required("kind").let { value -> value as String }
val `value`: AtomicNullableString get() = required("value").let { value -> AtomicNullableString.read(value) }
companion object { fun read(value: Any?): SessionPacketChooseRow = SessionPacketChooseRow(value) }
}
class SessionPacketDecideAggregate(rawValue: Any?) : View(rawValue), SessionPacket {
val `function`: String get() = required("function").let { value -> value as String }
val `kind`: String get() = required("kind").let { value -> value as String }
val `value`: List<AtomicDecideValue> get() = required("value").let { value -> Values.list(value) { item0 -> AtomicDecideValue.read(item0) } }
companion object { fun read(value: Any?): SessionPacketDecideAggregate = SessionPacketDecideAggregate(value) }
}
class SessionPacketDecideRow(rawValue: Any?) : View(rawValue), SessionPacket {
val `function`: String get() = required("function").let { value -> value as String }
val `kind`: String get() = required("kind").let { value -> value as String }
val `value`: AtomicDecideValue get() = required("value").let { value -> AtomicDecideValue.read(value) }
companion object { fun read(value: Any?): SessionPacketDecideRow = SessionPacketDecideRow(value) }
}
class SessionPacketFilterAggregate(rawValue: Any?) : View(rawValue), SessionPacket {
val `function`: String get() = required("function").let { value -> value as String }
val `kind`: String get() = required("kind").let { value -> value as String }
val `value`: List<AtomicBoolean> get() = required("value").let { value -> Values.list(value) { item0 -> AtomicBoolean.read(item0) } }
companion object { fun read(value: Any?): SessionPacketFilterAggregate = SessionPacketFilterAggregate(value) }
}
class SessionPacketFilterRow(rawValue: Any?) : View(rawValue), SessionPacket {
val `function`: String get() = required("function").let { value -> value as String }
val `kind`: String get() = required("kind").let { value -> value as String }
val `value`: AtomicBoolean get() = required("value").let { value -> AtomicBoolean.read(value) }
companion object { fun read(value: Any?): SessionPacketFilterRow = SessionPacketFilterRow(value) }
}
class SessionPacketFindAggregate(rawValue: Any?) : View(rawValue), SessionPacket {
val `function`: String get() = required("function").let { value -> value as String }
val `kind`: String get() = required("kind").let { value -> value as String }
val `value`: Find get() = required("value").let { value -> Find.read(value) }
companion object { fun read(value: Any?): SessionPacketFindAggregate = SessionPacketFindAggregate(value) }
}
class SessionPacketObservation(rawValue: Any?) : View(rawValue), SessionPacket {
val `function`: RequestFunction get() = required("function").let { value -> RequestFunction.read(value) }
val `kind`: String get() = required("kind").let { value -> value as String }
val `value`: SessionObservation get() = required("value").let { value -> SessionObservation.read(value) }
companion object { fun read(value: Any?): SessionPacketObservation = SessionPacketObservation(value) }
}
class SessionPacketRankAggregate(rawValue: Any?) : View(rawValue), SessionPacket {
val `function`: String get() = required("function").let { value -> value as String }
val `kind`: String get() = required("kind").let { value -> value as String }
val `value`: List<AtomicNonZeroUsize> get() = required("value").let { value -> Values.list(value) { item0 -> AtomicNonZeroUsize.read(item0) } }
companion object { fun read(value: Any?): SessionPacketRankAggregate = SessionPacketRankAggregate(value) }
}
class SessionPacketRecognizeAggregate(rawValue: Any?) : View(rawValue), SessionPacket {
val `function`: String get() = required("function").let { value -> value as String }
val `kind`: String get() = required("kind").let { value -> value as String }
val `value`: List<Recognition> get() = required("value").let { value -> Values.list(value) { item0 -> Recognition.read(item0) } }
companion object { fun read(value: Any?): SessionPacketRecognizeAggregate = SessionPacketRecognizeAggregate(value) }
}
class SessionPacketRelateAggregate(rawValue: Any?) : View(rawValue), SessionPacket {
val `function`: String get() = required("function").let { value -> value as String }
val `kind`: String get() = required("kind").let { value -> value as String }
val `value`: Relation get() = required("value").let { value -> Relation.read(value) }
companion object { fun read(value: Any?): SessionPacketRelateAggregate = SessionPacketRelateAggregate(value) }
}
class SessionPacketScoreAggregate(rawValue: Any?) : View(rawValue), SessionPacket {
val `function`: String get() = required("function").let { value -> value as String }
val `kind`: String get() = required("kind").let { value -> value as String }
val `value`: List<AtomicDouble> get() = required("value").let { value -> Values.list(value) { item0 -> AtomicDouble.read(item0) } }
companion object { fun read(value: Any?): SessionPacketScoreAggregate = SessionPacketScoreAggregate(value) }
}
class SessionPacketScoreRow(rawValue: Any?) : View(rawValue), SessionPacket {
val `function`: String get() = required("function").let { value -> value as String }
val `kind`: String get() = required("kind").let { value -> value as String }
val `value`: AtomicDouble get() = required("value").let { value -> AtomicDouble.read(value) }
companion object { fun read(value: Any?): SessionPacketScoreRow = SessionPacketScoreRow(value) }
}
class SessionPacketTagAggregate(rawValue: Any?) : View(rawValue), SessionPacket {
val `function`: String get() = required("function").let { value -> value as String }
val `kind`: String get() = required("kind").let { value -> value as String }
val `value`: List<AtomicArrayOfString> get() = required("value").let { value -> Values.list(value) { item0 -> AtomicArrayOfString.read(item0) } }
companion object { fun read(value: Any?): SessionPacketTagAggregate = SessionPacketTagAggregate(value) }
}
class SessionPacketTagRow(rawValue: Any?) : View(rawValue), SessionPacket {
val `function`: String get() = required("function").let { value -> value as String }
val `kind`: String get() = required("kind").let { value -> value as String }
val `value`: AtomicArrayOfString get() = required("value").let { value -> AtomicArrayOfString.read(value) }
companion object { fun read(value: Any?): SessionPacketTagRow = SessionPacketTagRow(value) }
}
class SessionPacketTerminal(rawValue: Any?) : View(rawValue), SessionPacket {
val `facts`: Facts? get() = optional("facts")?.let { value -> Facts.read(value) }
val `failure`: CallError? get() = optional("failure")?.let { value -> CallError.read(value) }
val `kind`: String get() = required("kind").let { value -> value as String }
companion object { fun read(value: Any?): SessionPacketTerminal = SessionPacketTerminal(value) }
}
sealed interface SessionProbabilities { val json: Any?
companion object {
fun read(value: Any?): SessionProbabilities {
val objectValue = Values.`object`(value)
if (objectValue["kind"] == "yes_no") return SessionProbabilitiesYesNo.read(value)
if (objectValue["kind"] == "named") return SessionProbabilitiesNamed.read(value)
throw IllegalStateException("Unknown native result alternative")
}
}
}
class SessionProbabilitiesNamed(rawValue: Any?) : View(rawValue), SessionProbabilities {
val `kind`: String get() = required("kind").let { value -> value as String }
val `value`: List<SessionNamedProbability> get() = required("value").let { value -> Values.list(value) { item0 -> SessionNamedProbability.read(item0) } }
companion object { fun read(value: Any?): SessionProbabilitiesNamed = SessionProbabilitiesNamed(value) }
}
class SessionProbabilitiesYesNo(rawValue: Any?) : View(rawValue), SessionProbabilities {
val `kind`: String get() = required("kind").let { value -> value as String }
val `value`: BigDecimal get() = required("value").let { value -> value as BigDecimal }
companion object { fun read(value: Any?): SessionProbabilitiesYesNo = SessionProbabilitiesYesNo(value) }
}
class SessionQuestionDetail(rawValue: Any?) : View(rawValue) {
val `answerId`: AnswerId? get() = optional("answer_id")?.let { value -> AnswerId.read(value) }
val `cached`: Boolean get() = required("cached").let { value -> value as Boolean }
val `confidence`: BigDecimal? get() = optional("confidence")?.let { value -> value as BigDecimal }
val `failedQuestions`: BigInteger get() = required("failed_questions").let { value -> Values.integer(value) }
val `failure`: Failure? get() = optional("failure")?.let { value -> Failure.read(value) }
val `failureId`: FailureId? get() = optional("failure_id")?.let { value -> FailureId.read(value) }
val `input`: Any? get() = optional("input")?.let { value -> Values.freeze(value) }
val `inputSource`: PhysicalSource? get() = optional("input_source")?.let { value -> PhysicalSource.read(value) }
val `inputSources`: List<SessionInputSource> get() = required("input_sources").let { value -> Values.list(value) { item0 -> SessionInputSource.read(item0) } }
val `inputs`: List<Any?> get() = required("inputs").let { value -> Values.list(value) { item0 -> Values.freeze(item0) } }
val `model`: String get() = required("model").let { value -> value as String }
val `observations`: List<Observation> get() = required("observations").let { value -> Values.list(value) { item0 -> Observation.read(item0) } }
val `probabilities`: SessionProbabilities? get() = optional("probabilities")?.let { value -> SessionProbabilities.read(value) }
val `question`: ReadableQuestion get() = required("question").let { value -> ReadableQuestion.read(value) }
val `questionSha256`: String get() = required("question_sha256").let { value -> value as String }
val `questionSources`: List<QuestionSource> get() = required("question_sources").let { value -> Values.list(value) { item0 -> QuestionSource.read(item0) } }
val `rawPick`: String? get() = optional("raw_pick")?.let { value -> value as String }
val `reportedUsage`: Usage? get() = optional("reported_usage")?.let { value -> Usage.read(value) }
val `requests`: List<String> get() = required("requests").let { value -> Values.list(value) { item0 -> item0 as String } }
val `requestsSent`: BigInteger get() = required("requests_sent").let { value -> Values.integer(value) }
val `threshold`: Threshold? get() = optional("threshold")?.let { value -> Threshold.read(value) }
val `url`: String get() = required("url").let { value -> value as String }
val `usage`: TokenUsage? get() = optional("usage")?.let { value -> TokenUsage.read(value) }
val `value`: Value? get() = optional("value")?.let { value -> Value.read(value) }
companion object { fun read(value: Any?): SessionQuestionDetail = SessionQuestionDetail(value) }
}
class SessionRecognition(rawValue: Any?) : View(rawValue) {
val `entities`: List<Entity> get() = required("entities").let { value -> Values.list(value) { item0 -> Entity.read(item0) } }
val `mode`: RecognitionMode get() = required("mode").let { value -> RecognitionMode.read(value) }
val `proposals`: List<BoundaryProposal>? get() = optional("proposals")?.let { value -> Values.list(value) { item0 -> BoundaryProposal.read(item0) } }
val `relations`: List<RecognitionEdgeDocument>? get() = optional("relations")?.let { value -> Values.list(value) { item0 -> RecognitionEdgeDocument.read(item0) } }
companion object { fun read(value: Any?): SessionRecognition = SessionRecognition(value) }
}
class SessionRelationEdge(rawValue: Any?) : View(rawValue) {
val `either`: Boolean get() = required("either").let { value -> value as Boolean }
val `probability`: BigDecimal get() = required("probability").let { value -> value as BigDecimal }
val `relation`: String get() = required("relation").let { value -> value as String }
val `source`: EntityDocument get() = required("source").let { value -> EntityDocument.read(value) }
val `target`: EntityDocument get() = required("target").let { value -> EntityDocument.read(value) }
companion object { fun read(value: Any?): SessionRelationEdge = SessionRelationEdge(value) }
}
class SourceRelationEndpoint(rawValue: Any?) : View(rawValue) {
val `file`: String? get() = required("file")?.let { value -> value as String }
val `firstLine`: BigInteger? get() = optional("first_line")?.let { value -> Values.integer(value) }
val `kind`: String get() = required("kind").let { value -> value as String }
val `lastLine`: BigInteger? get() = optional("last_line")?.let { value -> Values.integer(value) }
val `name`: String get() = required("name").let { value -> value as String }
val `ordinal`: BigInteger get() = required("ordinal").let { value -> Values.integer(value) }
val `record`: Any? get() = required("record").let { value -> Values.freeze(value) }
companion object { fun read(value: Any?): SourceRelationEndpoint = SourceRelationEndpoint(value) }
}
sealed interface Threshold { val json: Any?
companion object {
fun read(value: Any?): Threshold {
if (value is java.math.BigDecimal) return ThresholdNumber.read(value)
if (value is String) return ThresholdString.read(value)
throw IllegalStateException("Unknown native result alternative")
}
}
}
class ThresholdNumber(override val json: Any?) : Threshold { val value: BigDecimal get() = json as BigDecimal
companion object { fun read(value: Any?): ThresholdNumber = ThresholdNumber(Values.freeze(value)) }
}
class ThresholdString(override val json: Any?) : Threshold { val value: String get() = json as String
companion object { fun read(value: Any?): ThresholdString = ThresholdString(Values.freeze(value)) }
}
class TokenBand(rawValue: Any?) : View(rawValue) {
val `lower`: BigInteger get() = required("lower").let { value -> Values.integer(value) }
val `upper`: BigInteger get() = required("upper").let { value -> Values.integer(value) }
companion object { fun read(value: Any?): TokenBand = TokenBand(value) }
}
class TokenUsage(rawValue: Any?) : View(rawValue) {
val `inputTokens`: BigInteger get() = required("input_tokens").let { value -> Values.integer(value) }
val `outputTokens`: BigInteger get() = required("output_tokens").let { value -> Values.integer(value) }
companion object { fun read(value: Any?): TokenUsage = TokenUsage(value) }
}
sealed interface Value { val json: Any?
companion object {
fun read(value: Any?): Value {
if (value is Boolean) return ValueBoolean.read(value)
if (value == null) return ValueNull.read(value)
if (value is String) return ValueString.read(value)
if (value is List<*>) return ValueArray.read(value)
if (value is java.math.BigDecimal) return ValueNumber.read(value)
throw IllegalStateException("Unknown native result alternative")
}
}
}
class ValueBoolean(override val json: Any?) : Value { val value: Boolean get() = json as Boolean
companion object { fun read(value: Any?): ValueBoolean = ValueBoolean(Values.freeze(value)) }
}
class ValueNull(override val json: Any?) : Value { val value: Any? get() = Values.freeze(json)
companion object { fun read(value: Any?): ValueNull = ValueNull(Values.freeze(value)) }
}
class ValueString(override val json: Any?) : Value { val value: String get() = json as String
companion object { fun read(value: Any?): ValueString = ValueString(Values.freeze(value)) }
}
class ValueArray(override val json: Any?) : Value { val value: List<String> get() = Values.list(json) { item0 -> item0 as String }
companion object { fun read(value: Any?): ValueArray = ValueArray(Values.freeze(value)) }
}
class ValueNumber(override val json: Any?) : Value { val value: BigDecimal get() = json as BigDecimal
companion object { fun read(value: Any?): ValueNumber = ValueNumber(Values.freeze(value)) }
}
}
