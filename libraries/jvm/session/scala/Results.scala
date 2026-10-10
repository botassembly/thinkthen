// Generated from the Rust result graph. Do not edit.
package thinkthen.scala
import thinkthen.Values
import thinkthen.Presence
import scala.jdk.CollectionConverters.*
object Results {
abstract class View(value: Any) {
val json: Any = Values.freeze(value)
def presence(member: String): Presence.State = { val objectValue = Values.`object`(json); if (!objectValue.containsKey(member)) Presence.State.MISSING else if (objectValue.get(member) == null) Presence.State.NULL else Presence.State.VALUE }
protected def required(member: String): Any = { val objectValue = Values.`object`(json); require(objectValue.containsKey(member), "Missing native member: " + member); objectValue.get(member) }
protected def optional(member: String): Option[Any] = { val objectValue = Values.`object`(json); if (objectValue.containsKey(member)) Some(objectValue.get(member)) else None }
}
final class Annotation(rawValue: Any) extends View(rawValue) {
def `answerId`: AnswerId = { val value = required("answer_id"); require(value != null, "Null native member: " + "answer_id"); AnswerId.read(value) }
def `answers`: Map[String, AnnotationMember] = { val value = required("answers"); require(value != null, "Null native member: " + "answers"); Values.`object`(value).asScala.toMap.map { case (key, item0) => key -> (AnnotationMember.read(item0)) } }
def `file`: Option[String] = optional("file").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `firstLine`: Option[BigInt] = optional("first_line").flatMap(value => Option(value).map(value => BigInt(Values.integer(value))))
def `index`: Option[BigInt] = optional("index").flatMap(value => Option(value).map(value => BigInt(Values.integer(value))))
def `input`: Option[Any] = Option(required("input")).map(value => Values.freeze(value))
def `lastLine`: Option[BigInt] = optional("last_line").flatMap(value => Option(value).map(value => BigInt(Values.integer(value))))
def `meta`: Meta = { val value = required("meta"); require(value != null, "Null native member: " + "meta"); Meta.read(value) }
def `position`: Option[Position] = optional("position").flatMap(value => Option(value).map(value => Position.read(value)))
def `schema`: Version = { val value = required("schema"); require(value != null, "Null native member: " + "schema"); Version.read(value) }
def `source`: Option[PhysicalSource] = optional("source").flatMap(value => Option(value).map(value => PhysicalSource.read(value)))
def `value`: AnnotatedRow = { val value = required("value"); require(value != null, "Null native member: " + "value"); AnnotatedRow.read(value) }
}
object Annotation { def read(value: Any): Annotation = new Annotation(value) }
sealed trait AnnotationMember { def json: Any }
object AnnotationMember {
def read(value: Any): AnnotationMember = {
val objectValue = Values.`object`(value)
if (objectValue.containsKey("answer_id")) return AnnotationMemberAnswerId.read(value)
if (objectValue.containsKey("failure_id")) return AnnotationMemberFailureId.read(value)
throw new IllegalStateException("Unknown native result alternative")
}
}
final class AnnotationMemberAnswerId(rawValue: Any) extends View(rawValue) with AnnotationMember {
def `answer`: Answer = { val value = required("answer"); require(value != null, "Null native member: " + "answer"); Answer.read(value) }
def `answerId`: AnswerId = { val value = required("answer_id"); require(value != null, "Null native member: " + "answer_id"); AnswerId.read(value) }
def `observations`: List[Observation] = { val value = required("observations"); require(value != null, "Null native member: " + "observations"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => Observation.read(item0)) }
def `question`: ReadableQuestion = { val value = required("question"); require(value != null, "Null native member: " + "question"); ReadableQuestion.read(value) }
def `questionSources`: List[QuestionSource] = { val value = required("question_sources"); require(value != null, "Null native member: " + "question_sources"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => QuestionSource.read(item0)) }
def `request`: String = { val value = required("request"); require(value != null, "Null native member: " + "request"); value.asInstanceOf[String] }
def `threshold`: Option[Threshold] = Option(required("threshold")).map(value => Threshold.read(value))
def `usage`: Option[Usage] = optional("usage").flatMap(value => Option(value).map(value => Usage.read(value)))
def `value`: Option[Value] = Option(required("value")).map(value => Value.read(value))
}
object AnnotationMemberAnswerId { def read(value: Any): AnnotationMemberAnswerId = new AnnotationMemberAnswerId(value) }
final class AnnotationMemberFailureId(rawValue: Any) extends View(rawValue) with AnnotationMember {
def `failure`: Failure = { val value = required("failure"); require(value != null, "Null native member: " + "failure"); Failure.read(value) }
def `failureId`: FailureId = { val value = required("failure_id"); require(value != null, "Null native member: " + "failure_id"); FailureId.read(value) }
def `observations`: List[Observation] = { val value = required("observations"); require(value != null, "Null native member: " + "observations"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => Observation.read(item0)) }
def `question`: ReadableQuestion = { val value = required("question"); require(value != null, "Null native member: " + "question"); ReadableQuestion.read(value) }
def `questionSources`: List[QuestionSource] = { val value = required("question_sources"); require(value != null, "Null native member: " + "question_sources"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => QuestionSource.read(item0)) }
def `request`: String = { val value = required("request"); require(value != null, "Null native member: " + "request"); value.asInstanceOf[String] }
def `threshold`: Option[Threshold] = Option(required("threshold")).map(value => Threshold.read(value))
def `usage`: Option[Usage] = optional("usage").flatMap(value => Option(value).map(value => Usage.read(value)))
}
object AnnotationMemberFailureId { def read(value: Any): AnnotationMemberFailureId = new AnnotationMemberFailureId(value) }
sealed trait AnnotationValue { def json: Any }
object AnnotationValue {
def read(value: Any): AnnotationValue = {
val objectValue = Values.`object`(value)
if (objectValue.get("kind") == "decision") return AnnotationValueDecision.read(value)
if (objectValue.get("kind") == "choice") return AnnotationValueChoice.read(value)
if (objectValue.get("kind") == "score") return AnnotationValueScore.read(value)
if (objectValue.get("kind") == "tags") return AnnotationValueTags.read(value)
if (objectValue.get("kind") == "failed") return AnnotationValueFailed.read(value)
throw new IllegalStateException("Unknown native result alternative")
}
}
final class AnnotationValueChoice(rawValue: Any) extends View(rawValue) with AnnotationValue {
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `value`: Option[String] = Option(required("value")).map(value => value.asInstanceOf[String])
}
object AnnotationValueChoice { def read(value: Any): AnnotationValueChoice = new AnnotationValueChoice(value) }
final class AnnotationValueDecision(rawValue: Any) extends View(rawValue) with AnnotationValue {
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `value`: Option[Boolean] = Option(required("value")).map(value => value.asInstanceOf[Boolean])
}
object AnnotationValueDecision { def read(value: Any): AnnotationValueDecision = new AnnotationValueDecision(value) }
final class AnnotationValueFailed(rawValue: Any) extends View(rawValue) with AnnotationValue {
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `value`: Failure = { val value = required("value"); require(value != null, "Null native member: " + "value"); Failure.read(value) }
}
object AnnotationValueFailed { def read(value: Any): AnnotationValueFailed = new AnnotationValueFailed(value) }
final class AnnotationValueScore(rawValue: Any) extends View(rawValue) with AnnotationValue {
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `value`: BigDecimal = { val value = required("value"); require(value != null, "Null native member: " + "value"); BigDecimal(value.asInstanceOf[java.math.BigDecimal]) }
}
object AnnotationValueScore { def read(value: Any): AnnotationValueScore = new AnnotationValueScore(value) }
final class AnnotationValueTags(rawValue: Any) extends View(rawValue) with AnnotationValue {
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `value`: List[String] = { val value = required("value"); require(value != null, "Null native member: " + "value"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => item0.asInstanceOf[String]) }
}
object AnnotationValueTags { def read(value: Any): AnnotationValueTags = new AnnotationValueTags(value) }
final class AnswerId(rawValue: Any) extends View(rawValue) {
def value: String = json.asInstanceOf[String]
}
object AnswerId { def read(value: Any): AnswerId = new AnswerId(value) }
final class Answers(rawValue: Any) extends View(rawValue) {
def `questions`: List[RelationMember] = { val value = required("questions"); require(value != null, "Null native member: " + "questions"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => RelationMember.read(item0)) }
}
object Answers { def read(value: Any): Answers = new Answers(value) }
final class AtomicArrayOfString(rawValue: Any) extends View(rawValue) {
def `answer`: Answer = { val value = required("answer"); require(value != null, "Null native member: " + "answer"); Answer.read(value) }
def `answerId`: AnswerId = { val value = required("answer_id"); require(value != null, "Null native member: " + "answer_id"); AnswerId.read(value) }
def `images`: Option[List[Image]] = optional("images").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => Image.read(item0))))
def `index`: Option[BigInt] = optional("index").flatMap(value => Option(value).map(value => BigInt(Values.integer(value))))
def `input`: Option[Option[Any]] = optional("input").map(value => Option(value).map(value => Values.freeze(value)))
def `members`: Option[List[RankMember]] = optional("members").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => RankMember.read(item0))))
def `meta`: Meta = { val value = required("meta"); require(value != null, "Null native member: " + "meta"); Meta.read(value) }
def `question`: ReadableQuestion = { val value = required("question"); require(value != null, "Null native member: " + "question"); ReadableQuestion.read(value) }
def `questionName`: Option[String] = optional("question_name").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `schema`: Version = { val value = required("schema"); require(value != null, "Null native member: " + "schema"); Version.read(value) }
def `source`: Option[PhysicalSource] = optional("source").flatMap(value => Option(value).map(value => PhysicalSource.read(value)))
def `threshold`: Option[Threshold] = Option(required("threshold")).map(value => Threshold.read(value))
def `value`: List[String] = { val value = required("value"); require(value != null, "Null native member: " + "value"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => item0.asInstanceOf[String]) }
}
object AtomicArrayOfString { def read(value: Any): AtomicArrayOfString = new AtomicArrayOfString(value) }
final class AtomicDecideValue(rawValue: Any) extends View(rawValue) {
def `answer`: Answer = { val value = required("answer"); require(value != null, "Null native member: " + "answer"); Answer.read(value) }
def `answerId`: AnswerId = { val value = required("answer_id"); require(value != null, "Null native member: " + "answer_id"); AnswerId.read(value) }
def `images`: Option[List[Image]] = optional("images").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => Image.read(item0))))
def `index`: Option[BigInt] = optional("index").flatMap(value => Option(value).map(value => BigInt(Values.integer(value))))
def `input`: Option[Option[Any]] = optional("input").map(value => Option(value).map(value => Values.freeze(value)))
def `members`: Option[List[RankMember]] = optional("members").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => RankMember.read(item0))))
def `meta`: Meta = { val value = required("meta"); require(value != null, "Null native member: " + "meta"); Meta.read(value) }
def `question`: ReadableQuestion = { val value = required("question"); require(value != null, "Null native member: " + "question"); ReadableQuestion.read(value) }
def `questionName`: Option[String] = optional("question_name").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `schema`: Version = { val value = required("schema"); require(value != null, "Null native member: " + "schema"); Version.read(value) }
def `source`: Option[PhysicalSource] = optional("source").flatMap(value => Option(value).map(value => PhysicalSource.read(value)))
def `threshold`: Option[Threshold] = Option(required("threshold")).map(value => Threshold.read(value))
def `value`: DecideValue = { val value = required("value"); require(value != null, "Null native member: " + "value"); DecideValue.read(value) }
}
object AtomicDecideValue { def read(value: Any): AtomicDecideValue = new AtomicDecideValue(value) }
final class AtomicNonZeroUsize(rawValue: Any) extends View(rawValue) {
def `answer`: Answer = { val value = required("answer"); require(value != null, "Null native member: " + "answer"); Answer.read(value) }
def `answerId`: AnswerId = { val value = required("answer_id"); require(value != null, "Null native member: " + "answer_id"); AnswerId.read(value) }
def `images`: Option[List[Image]] = optional("images").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => Image.read(item0))))
def `index`: Option[BigInt] = optional("index").flatMap(value => Option(value).map(value => BigInt(Values.integer(value))))
def `input`: Option[Option[Any]] = optional("input").map(value => Option(value).map(value => Values.freeze(value)))
def `members`: Option[List[RankMember]] = optional("members").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => RankMember.read(item0))))
def `meta`: Meta = { val value = required("meta"); require(value != null, "Null native member: " + "meta"); Meta.read(value) }
def `question`: ReadableQuestion = { val value = required("question"); require(value != null, "Null native member: " + "question"); ReadableQuestion.read(value) }
def `questionName`: Option[String] = optional("question_name").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `schema`: Version = { val value = required("schema"); require(value != null, "Null native member: " + "schema"); Version.read(value) }
def `source`: Option[PhysicalSource] = optional("source").flatMap(value => Option(value).map(value => PhysicalSource.read(value)))
def `threshold`: Option[Threshold] = Option(required("threshold")).map(value => Threshold.read(value))
def `value`: BigInt = { val value = required("value"); require(value != null, "Null native member: " + "value"); BigInt(Values.integer(value)) }
}
object AtomicNonZeroUsize { def read(value: Any): AtomicNonZeroUsize = new AtomicNonZeroUsize(value) }
final class AtomicNullableString(rawValue: Any) extends View(rawValue) {
def `answer`: Answer = { val value = required("answer"); require(value != null, "Null native member: " + "answer"); Answer.read(value) }
def `answerId`: AnswerId = { val value = required("answer_id"); require(value != null, "Null native member: " + "answer_id"); AnswerId.read(value) }
def `images`: Option[List[Image]] = optional("images").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => Image.read(item0))))
def `index`: Option[BigInt] = optional("index").flatMap(value => Option(value).map(value => BigInt(Values.integer(value))))
def `input`: Option[Option[Any]] = optional("input").map(value => Option(value).map(value => Values.freeze(value)))
def `members`: Option[List[RankMember]] = optional("members").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => RankMember.read(item0))))
def `meta`: Meta = { val value = required("meta"); require(value != null, "Null native member: " + "meta"); Meta.read(value) }
def `question`: ReadableQuestion = { val value = required("question"); require(value != null, "Null native member: " + "question"); ReadableQuestion.read(value) }
def `questionName`: Option[String] = optional("question_name").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `schema`: Version = { val value = required("schema"); require(value != null, "Null native member: " + "schema"); Version.read(value) }
def `source`: Option[PhysicalSource] = optional("source").flatMap(value => Option(value).map(value => PhysicalSource.read(value)))
def `threshold`: Option[Threshold] = Option(required("threshold")).map(value => Threshold.read(value))
def `value`: Option[String] = Option(required("value")).map(value => value.asInstanceOf[String])
}
object AtomicNullableString { def read(value: Any): AtomicNullableString = new AtomicNullableString(value) }
final class AtomicBoolean(rawValue: Any) extends View(rawValue) {
def `answer`: Answer = { val value = required("answer"); require(value != null, "Null native member: " + "answer"); Answer.read(value) }
def `answerId`: AnswerId = { val value = required("answer_id"); require(value != null, "Null native member: " + "answer_id"); AnswerId.read(value) }
def `images`: Option[List[Image]] = optional("images").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => Image.read(item0))))
def `index`: Option[BigInt] = optional("index").flatMap(value => Option(value).map(value => BigInt(Values.integer(value))))
def `input`: Option[Option[Any]] = optional("input").map(value => Option(value).map(value => Values.freeze(value)))
def `members`: Option[List[RankMember]] = optional("members").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => RankMember.read(item0))))
def `meta`: Meta = { val value = required("meta"); require(value != null, "Null native member: " + "meta"); Meta.read(value) }
def `question`: ReadableQuestion = { val value = required("question"); require(value != null, "Null native member: " + "question"); ReadableQuestion.read(value) }
def `questionName`: Option[String] = optional("question_name").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `schema`: Version = { val value = required("schema"); require(value != null, "Null native member: " + "schema"); Version.read(value) }
def `source`: Option[PhysicalSource] = optional("source").flatMap(value => Option(value).map(value => PhysicalSource.read(value)))
def `threshold`: Option[Threshold] = Option(required("threshold")).map(value => Threshold.read(value))
def `value`: Boolean = { val value = required("value"); require(value != null, "Null native member: " + "value"); value.asInstanceOf[Boolean] }
}
object AtomicBoolean { def read(value: Any): AtomicBoolean = new AtomicBoolean(value) }
final class AtomicDouble(rawValue: Any) extends View(rawValue) {
def `answer`: Answer = { val value = required("answer"); require(value != null, "Null native member: " + "answer"); Answer.read(value) }
def `answerId`: AnswerId = { val value = required("answer_id"); require(value != null, "Null native member: " + "answer_id"); AnswerId.read(value) }
def `images`: Option[List[Image]] = optional("images").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => Image.read(item0))))
def `index`: Option[BigInt] = optional("index").flatMap(value => Option(value).map(value => BigInt(Values.integer(value))))
def `input`: Option[Option[Any]] = optional("input").map(value => Option(value).map(value => Values.freeze(value)))
def `members`: Option[List[RankMember]] = optional("members").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => RankMember.read(item0))))
def `meta`: Meta = { val value = required("meta"); require(value != null, "Null native member: " + "meta"); Meta.read(value) }
def `question`: ReadableQuestion = { val value = required("question"); require(value != null, "Null native member: " + "question"); ReadableQuestion.read(value) }
def `questionName`: Option[String] = optional("question_name").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `schema`: Version = { val value = required("schema"); require(value != null, "Null native member: " + "schema"); Version.read(value) }
def `source`: Option[PhysicalSource] = optional("source").flatMap(value => Option(value).map(value => PhysicalSource.read(value)))
def `threshold`: Option[Threshold] = Option(required("threshold")).map(value => Threshold.read(value))
def `value`: BigDecimal = { val value = required("value"); require(value != null, "Null native member: " + "value"); BigDecimal(value.asInstanceOf[java.math.BigDecimal]) }
}
object AtomicDouble { def read(value: Any): AtomicDouble = new AtomicDouble(value) }
final class Attempt(rawValue: Any) extends View(rawValue) {
def `ordinal`: BigInt = { val value = required("ordinal"); require(value != null, "Null native member: " + "ordinal"); BigInt(Values.integer(value)) }
def `outcome`: AttemptOutcome = { val value = required("outcome"); require(value != null, "Null native member: " + "outcome"); AttemptOutcome.read(value) }
def `requestId`: Option[String] = optional("request_id").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `requestSha256`: String = { val value = required("request_sha256"); require(value != null, "Null native member: " + "request_sha256"); value.asInstanceOf[String] }
def `sdkRequestId`: SdkRequestId = { val value = required("sdk_request_id"); require(value != null, "Null native member: " + "sdk_request_id"); SdkRequestId.read(value) }
def `serverMs`: Option[BigInt] = optional("server_ms").flatMap(value => Option(value).map(value => BigInt(Values.integer(value))))
def `status`: Option[BigInt] = optional("status").flatMap(value => Option(value).map(value => BigInt(Values.integer(value))))
def `wallMs`: BigInt = { val value = required("wall_ms"); require(value != null, "Null native member: " + "wall_ms"); BigInt(Values.integer(value)) }
}
object Attempt { def read(value: Any): Attempt = new Attempt(value) }
sealed trait Batch { def json: Any }
object Batch {
def read(value: Any): Batch = {
if (value.isInstanceOf[java.math.BigDecimal]) return BatchInteger.read(value)
if (value.isInstanceOf[String]) return BatchString.read(value)
throw new IllegalStateException("Unknown native result alternative")
}
}
final class BatchInteger(val json: Any) extends Batch { def value: BigInt = BigInt(Values.integer(json)) }
object BatchInteger { def read(value: Any): BatchInteger = new BatchInteger(Values.freeze(value)) }
final class BatchString(val json: Any) extends Batch { def value: String = json.asInstanceOf[String] }
object BatchString { def read(value: Any): BatchString = new BatchString(Values.freeze(value)) }
final class BoundaryMode(rawValue: Any) extends View(rawValue) {
def value: String = json.asInstanceOf[String]
}
object BoundaryMode { def read(value: Any): BoundaryMode = new BoundaryMode(value) }
final class BoundaryOdds(rawValue: Any) extends View(rawValue) {
def `pieces`: List[PieceOdds] = { val value = required("pieces"); require(value != null, "Null native member: " + "pieces"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => PieceOdds.read(item0)) }
def `proposals`: List[BoundaryProposal] = { val value = required("proposals"); require(value != null, "Null native member: " + "proposals"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => BoundaryProposal.read(item0)) }
}
object BoundaryOdds { def read(value: Any): BoundaryOdds = new BoundaryOdds(value) }
final class BoundaryProposal(rawValue: Any) extends View(rawValue) {
def `end`: BigInt = { val value = required("end"); require(value != null, "Null native member: " + "end"); BigInt(Values.integer(value)) }
def `length`: BigInt = { val value = required("length"); require(value != null, "Null native member: " + "length"); BigInt(Values.integer(value)) }
def `probability`: BigDecimal = { val value = required("probability"); require(value != null, "Null native member: " + "probability"); BigDecimal(value.asInstanceOf[java.math.BigDecimal]) }
def `start`: BigInt = { val value = required("start"); require(value != null, "Null native member: " + "start"); BigInt(Values.integer(value)) }
def `text`: String = { val value = required("text"); require(value != null, "Null native member: " + "text"); value.asInstanceOf[String] }
}
object BoundaryProposal { def read(value: Any): BoundaryProposal = new BoundaryProposal(value) }
final class CallError(rawValue: Any) extends View(rawValue) {
def `error`: Error = { val value = required("error"); require(value != null, "Null native member: " + "error"); Error.read(value) }
def `facts`: Option[Facts] = optional("facts").flatMap(value => Option(value).map(value => Facts.read(value)))
}
object CallError { def read(value: Any): CallError = new CallError(value) }
final class CallId(rawValue: Any) extends View(rawValue) {
def value: String = json.asInstanceOf[String]
}
object CallId { def read(value: Any): CallId = new CallId(value) }
final class DecideValue(rawValue: Any) extends View(rawValue) {
def value: Any = Values.freeze(json)
}
object DecideValue { def read(value: Any): DecideValue = new DecideValue(value) }
final class EntityDocument(rawValue: Any) extends View(rawValue) {
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `name`: String = { val value = required("name"); require(value != null, "Null native member: " + "name"); value.asInstanceOf[String] }
}
object EntityDocument { def read(value: Any): EntityDocument = new EntityDocument(value) }
final class Error(rawValue: Any) extends View(rawValue) {
def `estimatedInputDenial`: Option[EstimatedInputDenial] = optional("estimated_input_denial").flatMap(value => Option(value).map(value => EstimatedInputDenial.read(value)))
def `kind`: FailureKind = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); FailureKind.read(value) }
def `message`: String = { val value = required("message"); require(value != null, "Null native member: " + "message"); value.asInstanceOf[String] }
def `retryable`: Boolean = { val value = required("retryable"); require(value != null, "Null native member: " + "retryable"); value.asInstanceOf[Boolean] }
def `sendBudgetDenial`: Option[SendBudgetDenial] = optional("send_budget_denial").flatMap(value => Option(value).map(value => SendBudgetDenial.read(value)))
def `stopped`: Stopped = { val value = required("stopped"); require(value != null, "Null native member: " + "stopped"); Stopped.read(value) }
}
object Error { def read(value: Any): Error = new Error(value) }
sealed trait EstimatedInputDenial { def json: Any }
object EstimatedInputDenial {
def read(value: Any): EstimatedInputDenial = {
val objectValue = Values.`object`(value)
if (objectValue.get("kind") == "initial_request") return EstimatedInputDenialInitialRequest.read(value)
if (objectValue.get("kind") == "additional_request") return EstimatedInputDenialAdditionalRequest.read(value)
if (objectValue.get("kind") == "retry") return EstimatedInputDenialRetry.read(value)
throw new IllegalStateException("Unknown native result alternative")
}
}
final class EstimatedInputDenialAdditionalRequest(rawValue: Any) extends View(rawValue) with EstimatedInputDenial {
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `limit`: BigInt = { val value = required("limit"); require(value != null, "Null native member: " + "limit"); BigInt(Values.integer(value)) }
}
object EstimatedInputDenialAdditionalRequest { def read(value: Any): EstimatedInputDenialAdditionalRequest = new EstimatedInputDenialAdditionalRequest(value) }
final class EstimatedInputDenialInitialRequest(rawValue: Any) extends View(rawValue) with EstimatedInputDenial {
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `limit`: BigInt = { val value = required("limit"); require(value != null, "Null native member: " + "limit"); BigInt(Values.integer(value)) }
}
object EstimatedInputDenialInitialRequest { def read(value: Any): EstimatedInputDenialInitialRequest = new EstimatedInputDenialInitialRequest(value) }
final class EstimatedInputDenialRetry(rawValue: Any) extends View(rawValue) with EstimatedInputDenial {
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `lastStatus`: BigInt = { val value = required("last_status"); require(value != null, "Null native member: " + "last_status"); BigInt(Values.integer(value)) }
def `limit`: BigInt = { val value = required("limit"); require(value != null, "Null native member: " + "limit"); BigInt(Values.integer(value)) }
}
object EstimatedInputDenialRetry { def read(value: Any): EstimatedInputDenialRetry = new EstimatedInputDenialRetry(value) }
final class Facts(rawValue: Any) extends View(rawValue) {
def `attempts`: Option[List[Attempt]] = optional("attempts").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => Attempt.read(item0))))
def `cacheAnswers`: BigInt = { val value = required("cache_answers"); require(value != null, "Null native member: " + "cache_answers"); BigInt(Values.integer(value)) }
def `callId`: CallId = { val value = required("call_id"); require(value != null, "Null native member: " + "call_id"); CallId.read(value) }
def `estimatedCostUsd`: Option[String] = optional("estimated_cost_usd").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `heldModelMismatch`: Option[Boolean] = optional("held_model_mismatch").flatMap(value => Option(value).map(value => value.asInstanceOf[Boolean]))
def `inputTokens`: Option[BigInt] = optional("input_tokens").flatMap(value => Option(value).map(value => BigInt(Values.integer(value))))
def `largestRequestBytes`: BigInt = { val value = required("largest_request_bytes"); require(value != null, "Null native member: " + "largest_request_bytes"); BigInt(Values.integer(value)) }
def `largestRequestEstimatedInputTokens`: Option[BigInt] = Option(required("largest_request_estimated_input_tokens")).map(value => BigInt(Values.integer(value)))
def `model`: Option[String] = optional("model").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `outputTokens`: Option[BigInt] = optional("output_tokens").flatMap(value => Option(value).map(value => BigInt(Values.integer(value))))
def `records`: BigInt = { val value = required("records"); require(value != null, "Null native member: " + "records"); BigInt(Values.integer(value)) }
def `requestsSent`: BigInt = { val value = required("requests_sent"); require(value != null, "Null native member: " + "requests_sent"); BigInt(Values.integer(value)) }
def `seconds`: BigDecimal = { val value = required("seconds"); require(value != null, "Null native member: " + "seconds"); BigDecimal(value.asInstanceOf[java.math.BigDecimal]) }
def `tokenEstimateMethod`: String = { val value = required("token_estimate_method"); require(value != null, "Null native member: " + "token_estimate_method"); value.asInstanceOf[String] }
def `usagePersistence`: Option[PersistenceObservation] = optional("usage_persistence").flatMap(value => Option(value).map(value => PersistenceObservation.read(value)))
}
object Facts { def read(value: Any): Facts = new Facts(value) }
final class FailureId(rawValue: Any) extends View(rawValue) {
def value: String = json.asInstanceOf[String]
}
object FailureId { def read(value: Any): FailureId = new FailureId(value) }
final class Find(rawValue: Any) extends View(rawValue) {
def `answer`: FindAnswer = { val value = required("answer"); require(value != null, "Null native member: " + "answer"); FindAnswer.read(value) }
def `answerId`: AnswerId = { val value = required("answer_id"); require(value != null, "Null native member: " + "answer_id"); AnswerId.read(value) }
def `candidates`: Option[List[FindCandidate]] = optional("candidates").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => FindCandidate.read(item0))))
def `file`: Option[String] = optional("file").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `firstLine`: Option[BigInt] = optional("first_line").flatMap(value => Option(value).map(value => BigInt(Values.integer(value))))
def `index`: Option[BigInt] = Option(required("index")).map(value => BigInt(Values.integer(value)))
def `lastLine`: Option[BigInt] = optional("last_line").flatMap(value => Option(value).map(value => BigInt(Values.integer(value))))
def `meta`: Meta = { val value = required("meta"); require(value != null, "Null native member: " + "meta"); Meta.read(value) }
def `position`: Option[Position] = optional("position").flatMap(value => Option(value).map(value => Position.read(value)))
def `question`: ReadableQuestion2 = { val value = required("question"); require(value != null, "Null native member: " + "question"); ReadableQuestion2.read(value) }
def `schema`: Version = { val value = required("schema"); require(value != null, "Null native member: " + "schema"); Version.read(value) }
def `threshold`: Option[Any] = Option(required("threshold")).map(value => Values.freeze(value))
def `value`: Option[Any] = Option(required("value")).map(value => Values.freeze(value))
}
object Find { def read(value: Any): Find = new Find(value) }
final class FindCandidate(rawValue: Any) extends View(rawValue) {
def `index`: Option[BigInt] = Option(required("index")).map(value => BigInt(Values.integer(value)))
def `input`: Option[Any] = Option(required("input")).map(value => Values.freeze(value))
def `probability`: BigDecimal = { val value = required("probability"); require(value != null, "Null native member: " + "probability"); BigDecimal(value.asInstanceOf[java.math.BigDecimal]) }
def `source`: Option[PhysicalSource] = optional("source").flatMap(value => Option(value).map(value => PhysicalSource.read(value)))
}
object FindCandidate { def read(value: Any): FindCandidate = new FindCandidate(value) }
final class Image(rawValue: Any) extends View(rawValue) {
def `base64`: String = { val value = required("base64"); require(value != null, "Null native member: " + "base64"); value.asInstanceOf[String] }
def `height`: BigInt = { val value = required("height"); require(value != null, "Null native member: " + "height"); BigInt(Values.integer(value)) }
def `media`: ImageMedia = { val value = required("media"); require(value != null, "Null native member: " + "media"); ImageMedia.read(value) }
def `width`: BigInt = { val value = required("width"); require(value != null, "Null native member: " + "width"); BigInt(Values.integer(value)) }
}
object Image { def read(value: Any): Image = new Image(value) }
final class ImageMedia(rawValue: Any) extends View(rawValue) {
def value: String = json.asInstanceOf[String]
}
object ImageMedia { def read(value: Any): ImageMedia = new ImageMedia(value) }
sealed trait InputDeclaration { def json: Any }
object InputDeclaration {
def read(value: Any): InputDeclaration = {
val objectValue = Values.`object`(value)
if (objectValue.get("type") == "string") return InputDeclarationString.read(value)
if (objectValue.get("type") == "object") return InputDeclarationObject.read(value)
throw new IllegalStateException("Unknown native result alternative")
}
}
final class InputDeclarationObject(rawValue: Any) extends View(rawValue) with InputDeclaration {
def `properties`: Map[String, InputPropertyType] = { val value = required("properties"); require(value != null, "Null native member: " + "properties"); Values.`object`(value).asScala.toMap.map { case (key, item0) => key -> (InputPropertyType.read(item0)) } }
def `required`: Option[List[String]] = optional("required").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => item0.asInstanceOf[String])))
def `type`: ObjectType = { val value = required("type"); require(value != null, "Null native member: " + "type"); ObjectType.read(value) }
}
object InputDeclarationObject { def read(value: Any): InputDeclarationObject = new InputDeclarationObject(value) }
final class InputDeclarationString(rawValue: Any) extends View(rawValue) with InputDeclaration {
def `type`: StringType = { val value = required("type"); require(value != null, "Null native member: " + "type"); StringType.read(value) }
}
object InputDeclarationString { def read(value: Any): InputDeclarationString = new InputDeclarationString(value) }
sealed trait InputPropertyType { def json: Any }
object InputPropertyType {
def read(value: Any): InputPropertyType = {
val objectValue = Values.`object`(value)
if (objectValue.get("type") == "string") return InputPropertyTypeString.read(value)
if (objectValue.get("type") == "number") return InputPropertyTypeNumber.read(value)
if (objectValue.get("type") == "boolean") return InputPropertyTypeBoolean.read(value)
if (objectValue.get("type") == "array") return InputPropertyTypeArray.read(value)
throw new IllegalStateException("Unknown native result alternative")
}
}
final class InputPropertyTypeArray(rawValue: Any) extends View(rawValue) with InputPropertyType {
def `items`: StringRoot = { val value = required("items"); require(value != null, "Null native member: " + "items"); StringRoot.read(value) }
def `type`: String = { val value = required("type"); require(value != null, "Null native member: " + "type"); value.asInstanceOf[String] }
}
object InputPropertyTypeArray { def read(value: Any): InputPropertyTypeArray = new InputPropertyTypeArray(value) }
final class InputPropertyTypeBoolean(rawValue: Any) extends View(rawValue) with InputPropertyType {
def `type`: String = { val value = required("type"); require(value != null, "Null native member: " + "type"); value.asInstanceOf[String] }
}
object InputPropertyTypeBoolean { def read(value: Any): InputPropertyTypeBoolean = new InputPropertyTypeBoolean(value) }
final class InputPropertyTypeNumber(rawValue: Any) extends View(rawValue) with InputPropertyType {
def `type`: String = { val value = required("type"); require(value != null, "Null native member: " + "type"); value.asInstanceOf[String] }
}
object InputPropertyTypeNumber { def read(value: Any): InputPropertyTypeNumber = new InputPropertyTypeNumber(value) }
final class InputPropertyTypeString(rawValue: Any) extends View(rawValue) with InputPropertyType {
def `type`: String = { val value = required("type"); require(value != null, "Null native member: " + "type"); value.asInstanceOf[String] }
}
object InputPropertyTypeString { def read(value: Any): InputPropertyTypeString = new InputPropertyTypeString(value) }
final class Label(rawValue: Any) extends View(rawValue) {
def `description`: Option[Any] = optional("description").flatMap(value => Option(value).map(value => Values.freeze(value)))
def `name`: String = { val value = required("name"); require(value != null, "Null native member: " + "name"); value.asInstanceOf[String] }
}
object Label { def read(value: Any): Label = new Label(value) }
final class Meta(rawValue: Any) extends View(rawValue) {
def `answeredBy`: Option[String] = optional("answered_by").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `attempts`: Option[List[Attempt]] = optional("attempts").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => Attempt.read(item0))))
def `batchSetting`: Option[BatchSetting] = optional("batch_setting").flatMap(value => Option(value).map(value => BatchSetting.read(value)))
def `batchWarning`: Option[BatchWarning] = optional("batch_warning").flatMap(value => Option(value).map(value => BatchWarning.read(value)))
def `cached`: Boolean = { val value = required("cached"); require(value != null, "Null native member: " + "cached"); value.asInstanceOf[Boolean] }
def `contextSha256`: Option[String] = optional("context_sha256").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `failedQuestions`: BigInt = { val value = required("failed_questions"); require(value != null, "Null native member: " + "failed_questions"); BigInt(Values.integer(value)) }
def `model`: String = { val value = required("model"); require(value != null, "Null native member: " + "model"); value.asInstanceOf[String] }
def `observations`: List[Observation] = { val value = required("observations"); require(value != null, "Null native member: " + "observations"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => Observation.read(item0)) }
def `origin`: Option[Origin] = Option(required("origin")).map(value => Origin.read(value))
def `profileWarning`: Option[ProfileWarning] = optional("profile_warning").flatMap(value => Option(value).map(value => ProfileWarning.read(value)))
def `questionSha256`: Option[String] = optional("question_sha256").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `questionSources`: List[QuestionSource] = { val value = required("question_sources"); require(value != null, "Null native member: " + "question_sources"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => QuestionSource.read(item0)) }
def `questionsSha256`: Option[String] = optional("questions_sha256").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `requests`: List[String] = { val value = required("requests"); require(value != null, "Null native member: " + "requests"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => item0.asInstanceOf[String]) }
def `requestsSent`: BigInt = { val value = required("requests_sent"); require(value != null, "Null native member: " + "requests_sent"); BigInt(Values.integer(value)) }
def `tool`: String = { val value = required("tool"); require(value != null, "Null native member: " + "tool"); value.asInstanceOf[String] }
def `url`: String = { val value = required("url"); require(value != null, "Null native member: " + "url"); value.asInstanceOf[String] }
def `usage`: Option[Usage] = optional("usage").flatMap(value => Option(value).map(value => Usage.read(value)))
}
object Meta { def read(value: Any): Meta = new Meta(value) }
final class ObjectRoot(rawValue: Any) extends View(rawValue) {
def `properties`: Map[String, InputPropertyType] = { val value = required("properties"); require(value != null, "Null native member: " + "properties"); Values.`object`(value).asScala.toMap.map { case (key, item0) => key -> (InputPropertyType.read(item0)) } }
def `required`: Option[List[String]] = optional("required").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => item0.asInstanceOf[String])))
def `type`: ObjectType = { val value = required("type"); require(value != null, "Null native member: " + "type"); ObjectType.read(value) }
}
object ObjectRoot { def read(value: Any): ObjectRoot = new ObjectRoot(value) }
final class ObjectType(rawValue: Any) extends View(rawValue) {
def value: String = json.asInstanceOf[String]
}
object ObjectType { def read(value: Any): ObjectType = new ObjectType(value) }
sealed trait Observation { def json: Any }
object Observation {
def read(value: Any): Observation = {
val objectValue = Values.`object`(value)
if (objectValue.containsKey("observation_id")) return ObservationObservationId.read(value)
if (objectValue.containsKey("failure_id")) return ObservationFailureId.read(value)
throw new IllegalStateException("Unknown native result alternative")
}
}
final class ObservationId(rawValue: Any) extends View(rawValue) {
def value: String = json.asInstanceOf[String]
}
object ObservationId { def read(value: Any): ObservationId = new ObservationId(value) }
final class ObservationFailureId(rawValue: Any) extends View(rawValue) with Observation {
def `failureId`: FailureId = { val value = required("failure_id"); require(value != null, "Null native member: " + "failure_id"); FailureId.read(value) }
}
object ObservationFailureId { def read(value: Any): ObservationFailureId = new ObservationFailureId(value) }
final class ObservationObservationId(rawValue: Any) extends View(rawValue) with Observation {
def `observationId`: ObservationId = { val value = required("observation_id"); require(value != null, "Null native member: " + "observation_id"); ObservationId.read(value) }
}
object ObservationObservationId { def read(value: Any): ObservationObservationId = new ObservationObservationId(value) }
final class Origin(rawValue: Any) extends View(rawValue) {
def value: String = json.asInstanceOf[String]
}
object Origin { def read(value: Any): Origin = new Origin(value) }
final class PersistenceObservation(rawValue: Any) extends View(rawValue) {
def `advice`: Option[String] = optional("advice").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `observedAt`: String = { val value = required("observed_at"); require(value != null, "Null native member: " + "observed_at"); value.asInstanceOf[String] }
def `state`: UsagePersistence = { val value = required("state"); require(value != null, "Null native member: " + "state"); UsagePersistence.read(value) }
}
object PersistenceObservation { def read(value: Any): PersistenceObservation = new PersistenceObservation(value) }
final class PhysicalSource(rawValue: Any) extends View(rawValue) {
def `file`: String = { val value = required("file"); require(value != null, "Null native member: " + "file"); value.asInstanceOf[String] }
def `firstLine`: Option[BigInt] = optional("first_line").flatMap(value => Option(value).map(value => BigInt(Values.integer(value))))
def `lastLine`: Option[BigInt] = optional("last_line").flatMap(value => Option(value).map(value => BigInt(Values.integer(value))))
}
object PhysicalSource { def read(value: Any): PhysicalSource = new PhysicalSource(value) }
final class Position(rawValue: Any) extends View(rawValue) {
def `file`: Option[String] = Option(required("file")).map(value => value.asInstanceOf[String])
def `first`: Option[BigInt] = optional("first").flatMap(value => Option(value).map(value => BigInt(Values.integer(value))))
def `images`: Option[List[String]] = optional("images").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => item0.asInstanceOf[String])))
def `last`: Option[BigInt] = optional("last").flatMap(value => Option(value).map(value => BigInt(Values.integer(value))))
}
object Position { def read(value: Any): Position = new Position(value) }
final class QuestionName(rawValue: Any) extends View(rawValue) {
def value: String = json.asInstanceOf[String]
}
object QuestionName { def read(value: Any): QuestionName = new QuestionName(value) }
final class QuestionSource(rawValue: Any) extends View(rawValue) {
def `answeredBy`: String = { val value = required("answered_by"); require(value != null, "Null native member: " + "answered_by"); value.asInstanceOf[String] }
def `batchSize`: Option[BigInt] = optional("batch_size").flatMap(value => Option(value).map(value => BigInt(Values.integer(value))))
def `origin`: Origin = { val value = required("origin"); require(value != null, "Null native member: " + "origin"); Origin.read(value) }
}
object QuestionSource { def read(value: Any): QuestionSource = new QuestionSource(value) }
final class RankMember(rawValue: Any) extends View(rawValue) {
def `name`: String = { val value = required("name"); require(value != null, "Null native member: " + "name"); value.asInstanceOf[String] }
def `result`: RankMemberResult = { val value = required("result"); require(value != null, "Null native member: " + "result"); RankMemberResult.read(value) }
}
object RankMember { def read(value: Any): RankMember = new RankMember(value) }
final class RankMemberResult(rawValue: Any) extends View(rawValue) {
def `answer`: Answer = { val value = required("answer"); require(value != null, "Null native member: " + "answer"); Answer.read(value) }
def `answerId`: AnswerId = { val value = required("answer_id"); require(value != null, "Null native member: " + "answer_id"); AnswerId.read(value) }
def `images`: Option[List[Image]] = optional("images").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => Image.read(item0))))
def `meta`: Meta = { val value = required("meta"); require(value != null, "Null native member: " + "meta"); Meta.read(value) }
def `question`: ReadableQuestion = { val value = required("question"); require(value != null, "Null native member: " + "question"); ReadableQuestion.read(value) }
def `schema`: Version = { val value = required("schema"); require(value != null, "Null native member: " + "schema"); Version.read(value) }
def `source`: Option[PhysicalSource] = optional("source").flatMap(value => Option(value).map(value => PhysicalSource.read(value)))
def `threshold`: Option[Any] = Option(required("threshold")).map(value => Values.freeze(value))
def `value`: BigInt = { val value = required("value"); require(value != null, "Null native member: " + "value"); BigInt(Values.integer(value)) }
}
object RankMemberResult { def read(value: Any): RankMemberResult = new RankMemberResult(value) }
sealed trait ReadableQuestion { def json: Any }
object ReadableQuestion {
def read(value: Any): ReadableQuestion = {
val objectValue = Values.`object`(value)
if (objectValue.get("verb") == "decide") return ReadableQuestionDecide.read(value)
if (objectValue.get("verb") == "choose") return ReadableQuestionChoose.read(value)
if (objectValue.get("verb") == "tag") return ReadableQuestionTag.read(value)
if (objectValue.get("verb") == "score") return ReadableQuestionScore.read(value)
throw new IllegalStateException("Unknown native result alternative")
}
}
final class ReadableQuestion2(rawValue: Any) extends View(rawValue) {
def `batch`: Option[Batch] = optional("batch").flatMap(value => Option(value).map(value => Batch.read(value)))
def `contextSchema`: Option[InputDeclaration] = optional("context_schema").flatMap(value => Option(value).map(value => InputDeclaration.read(value)))
def `itemSchema`: Option[InputDeclaration] = optional("item_schema").flatMap(value => Option(value).map(value => InputDeclaration.read(value)))
def `labelDetails`: Option[List[Label]] = optional("label_details").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => Label.read(item0))))
def `model`: Option[String] = optional("model").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `name`: Option[QuestionName] = optional("name").flatMap(value => Option(value).map(value => QuestionName.read(value)))
def `none`: Boolean = { val value = required("none"); require(value != null, "Null native member: " + "none"); value.asInstanceOf[Boolean] }
def `on`: Option[List[String]] = optional("on").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => item0.asInstanceOf[String])))
def `profile`: Option[String] = optional("profile").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `text`: Any = { val value = required("text"); require(value != null, "Null native member: " + "text"); Values.freeze(value) }
def `verb`: String = { val value = required("verb"); require(value != null, "Null native member: " + "verb"); value.asInstanceOf[String] }
def `wordingVersion`: Option[WordingVersion] = optional("wording_version").flatMap(value => Option(value).map(value => WordingVersion.read(value)))
}
object ReadableQuestion2 { def read(value: Any): ReadableQuestion2 = new ReadableQuestion2(value) }
final class ReadableQuestion3(rawValue: Any) extends View(rawValue) {
def `batch`: Option[Batch] = optional("batch").flatMap(value => Option(value).map(value => Batch.read(value)))
def `contextSchema`: Option[InputDeclaration] = optional("context_schema").flatMap(value => Option(value).map(value => InputDeclaration.read(value)))
def `entityDefinition`: Option[Any] = optional("entity_definition").flatMap(value => Option(value).map(value => Values.freeze(value)))
def `instructions`: Option[Any] = optional("instructions").flatMap(value => Option(value).map(value => Values.freeze(value)))
def `itemSchema`: Option[InputDeclaration] = optional("item_schema").flatMap(value => Option(value).map(value => InputDeclaration.read(value)))
def `kinds`: Map[String, Any] = { val value = required("kinds"); require(value != null, "Null native member: " + "kinds"); Values.`object`(value).asScala.toMap.map { case (key, item0) => key -> (Values.freeze(item0)) } }
def `labelDetails`: Option[List[Label]] = optional("label_details").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => Label.read(item0))))
def `mode`: Option[RecognitionMode] = optional("mode").flatMap(value => Option(value).map(value => RecognitionMode.read(value)))
def `model`: Option[String] = optional("model").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `name`: Option[QuestionName] = optional("name").flatMap(value => Option(value).map(value => QuestionName.read(value)))
def `on`: Option[List[String]] = optional("on").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => item0.asInstanceOf[String])))
def `profile`: Option[String] = optional("profile").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `relationThreshold`: Option[Threshold] = optional("relation_threshold").flatMap(value => Option(value).map(value => Threshold.read(value)))
def `relations`: Option[List[RelationRule]] = optional("relations").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => RelationRule.read(item0))))
def `snippetPieces`: Option[BigInt] = optional("snippet_pieces").flatMap(value => Option(value).map(value => BigInt(Values.integer(value))))
def `stageContext`: Option[RecognitionStageContext] = optional("stage_context").flatMap(value => Option(value).map(value => RecognitionStageContext.read(value)))
def `threshold`: Threshold = { val value = required("threshold"); require(value != null, "Null native member: " + "threshold"); Threshold.read(value) }
def `verb`: Verb = { val value = required("verb"); require(value != null, "Null native member: " + "verb"); Verb.read(value) }
def `wordingVersion`: Option[WordingVersion] = optional("wording_version").flatMap(value => Option(value).map(value => WordingVersion.read(value)))
}
object ReadableQuestion3 { def read(value: Any): ReadableQuestion3 = new ReadableQuestion3(value) }
final class ReadableQuestion4(rawValue: Any) extends View(rawValue) {
def `batch`: Option[Batch] = optional("batch").flatMap(value => Option(value).map(value => Batch.read(value)))
def `contextSchema`: Option[InputDeclaration] = optional("context_schema").flatMap(value => Option(value).map(value => InputDeclaration.read(value)))
def `fields`: Option[RelateFields] = Option(required("fields")).map(value => RelateFields.read(value))
def `itemSchema`: Option[InputDeclaration] = optional("item_schema").flatMap(value => Option(value).map(value => InputDeclaration.read(value)))
def `labelDetails`: Option[List[Label]] = optional("label_details").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => Label.read(item0))))
def `model`: Option[String] = optional("model").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `name`: Option[QuestionName] = optional("name").flatMap(value => Option(value).map(value => QuestionName.read(value)))
def `on`: Option[List[String]] = optional("on").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => item0.asInstanceOf[String])))
def `profile`: Option[String] = optional("profile").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `relations`: List[RelationRule] = { val value = required("relations"); require(value != null, "Null native member: " + "relations"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => RelationRule.read(item0)) }
def `threshold`: Threshold = { val value = required("threshold"); require(value != null, "Null native member: " + "threshold"); Threshold.read(value) }
def `verb`: String = { val value = required("verb"); require(value != null, "Null native member: " + "verb"); value.asInstanceOf[String] }
def `wordingVersion`: Option[WordingVersion] = optional("wording_version").flatMap(value => Option(value).map(value => WordingVersion.read(value)))
}
object ReadableQuestion4 { def read(value: Any): ReadableQuestion4 = new ReadableQuestion4(value) }
final class ReadableQuestionChoose(rawValue: Any) extends View(rawValue) with ReadableQuestion {
def `batch`: Option[Batch] = optional("batch").flatMap(value => Option(value).map(value => Batch.read(value)))
def `contextSchema`: Option[InputDeclaration] = optional("context_schema").flatMap(value => Option(value).map(value => InputDeclaration.read(value)))
def `itemSchema`: Option[InputDeclaration] = optional("item_schema").flatMap(value => Option(value).map(value => InputDeclaration.read(value)))
def `labelDetails`: Option[List[Label]] = optional("label_details").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => Label.read(item0))))
def `model`: Option[String] = optional("model").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `name`: Option[QuestionName] = optional("name").flatMap(value => Option(value).map(value => QuestionName.read(value)))
def `on`: Option[List[String]] = optional("on").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => item0.asInstanceOf[String])))
def `profile`: Option[String] = optional("profile").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `wordingVersion`: Option[WordingVersion] = optional("wording_version").flatMap(value => Option(value).map(value => WordingVersion.read(value)))
def `options`: List[String] = { val value = required("options"); require(value != null, "Null native member: " + "options"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => item0.asInstanceOf[String]) }
def `text`: Any = { val value = required("text"); require(value != null, "Null native member: " + "text"); Values.freeze(value) }
def `verb`: String = { val value = required("verb"); require(value != null, "Null native member: " + "verb"); value.asInstanceOf[String] }
}
object ReadableQuestionChoose { def read(value: Any): ReadableQuestionChoose = new ReadableQuestionChoose(value) }
final class ReadableQuestionDecide(rawValue: Any) extends View(rawValue) with ReadableQuestion {
def `batch`: Option[Batch] = optional("batch").flatMap(value => Option(value).map(value => Batch.read(value)))
def `contextSchema`: Option[InputDeclaration] = optional("context_schema").flatMap(value => Option(value).map(value => InputDeclaration.read(value)))
def `itemSchema`: Option[InputDeclaration] = optional("item_schema").flatMap(value => Option(value).map(value => InputDeclaration.read(value)))
def `labelDetails`: Option[List[Label]] = optional("label_details").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => Label.read(item0))))
def `model`: Option[String] = optional("model").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `name`: Option[QuestionName] = optional("name").flatMap(value => Option(value).map(value => QuestionName.read(value)))
def `on`: Option[List[String]] = optional("on").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => item0.asInstanceOf[String])))
def `profile`: Option[String] = optional("profile").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `wordingVersion`: Option[WordingVersion] = optional("wording_version").flatMap(value => Option(value).map(value => WordingVersion.read(value)))
def `falseValue`: Option[Any] = optional("false").flatMap(value => Option(value).map(value => Values.freeze(value)))
def `text`: Any = { val value = required("text"); require(value != null, "Null native member: " + "text"); Values.freeze(value) }
def `trueValue`: Option[Any] = optional("true").flatMap(value => Option(value).map(value => Values.freeze(value)))
def `verb`: String = { val value = required("verb"); require(value != null, "Null native member: " + "verb"); value.asInstanceOf[String] }
}
object ReadableQuestionDecide { def read(value: Any): ReadableQuestionDecide = new ReadableQuestionDecide(value) }
final class ReadableQuestionScore(rawValue: Any) extends View(rawValue) with ReadableQuestion {
def `batch`: Option[Batch] = optional("batch").flatMap(value => Option(value).map(value => Batch.read(value)))
def `contextSchema`: Option[InputDeclaration] = optional("context_schema").flatMap(value => Option(value).map(value => InputDeclaration.read(value)))
def `itemSchema`: Option[InputDeclaration] = optional("item_schema").flatMap(value => Option(value).map(value => InputDeclaration.read(value)))
def `labelDetails`: Option[List[Label]] = optional("label_details").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => Label.read(item0))))
def `model`: Option[String] = optional("model").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `name`: Option[QuestionName] = optional("name").flatMap(value => Option(value).map(value => QuestionName.read(value)))
def `on`: Option[List[String]] = optional("on").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => item0.asInstanceOf[String])))
def `profile`: Option[String] = optional("profile").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `wordingVersion`: Option[WordingVersion] = optional("wording_version").flatMap(value => Option(value).map(value => WordingVersion.read(value)))
def `levels`: List[String] = { val value = required("levels"); require(value != null, "Null native member: " + "levels"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => item0.asInstanceOf[String]) }
def `text`: Any = { val value = required("text"); require(value != null, "Null native member: " + "text"); Values.freeze(value) }
def `verb`: String = { val value = required("verb"); require(value != null, "Null native member: " + "verb"); value.asInstanceOf[String] }
}
object ReadableQuestionScore { def read(value: Any): ReadableQuestionScore = new ReadableQuestionScore(value) }
final class ReadableQuestionTag(rawValue: Any) extends View(rawValue) with ReadableQuestion {
def `batch`: Option[Batch] = optional("batch").flatMap(value => Option(value).map(value => Batch.read(value)))
def `contextSchema`: Option[InputDeclaration] = optional("context_schema").flatMap(value => Option(value).map(value => InputDeclaration.read(value)))
def `itemSchema`: Option[InputDeclaration] = optional("item_schema").flatMap(value => Option(value).map(value => InputDeclaration.read(value)))
def `labelDetails`: Option[List[Label]] = optional("label_details").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => Label.read(item0))))
def `model`: Option[String] = optional("model").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `name`: Option[QuestionName] = optional("name").flatMap(value => Option(value).map(value => QuestionName.read(value)))
def `on`: Option[List[String]] = optional("on").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => item0.asInstanceOf[String])))
def `profile`: Option[String] = optional("profile").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `wordingVersion`: Option[WordingVersion] = optional("wording_version").flatMap(value => Option(value).map(value => WordingVersion.read(value)))
def `labels`: List[String] = { val value = required("labels"); require(value != null, "Null native member: " + "labels"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => item0.asInstanceOf[String]) }
def `text`: Any = { val value = required("text"); require(value != null, "Null native member: " + "text"); Values.freeze(value) }
def `verb`: String = { val value = required("verb"); require(value != null, "Null native member: " + "verb"); value.asInstanceOf[String] }
}
object ReadableQuestionTag { def read(value: Any): ReadableQuestionTag = new ReadableQuestionTag(value) }
final class Recognition(rawValue: Any) extends View(rawValue) {
def `answer`: RecognitionOdds = { val value = required("answer"); require(value != null, "Null native member: " + "answer"); RecognitionOdds.read(value) }
def `answerId`: AnswerId = { val value = required("answer_id"); require(value != null, "Null native member: " + "answer_id"); AnswerId.read(value) }
def `file`: Option[String] = optional("file").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `firstLine`: Option[BigInt] = optional("first_line").flatMap(value => Option(value).map(value => BigInt(Values.integer(value))))
def `index`: Option[BigInt] = optional("index").flatMap(value => Option(value).map(value => BigInt(Values.integer(value))))
def `input`: Option[Option[Any]] = optional("input").map(value => Option(value).map(value => Values.freeze(value)))
def `lastLine`: Option[BigInt] = optional("last_line").flatMap(value => Option(value).map(value => BigInt(Values.integer(value))))
def `meta`: Meta = { val value = required("meta"); require(value != null, "Null native member: " + "meta"); Meta.read(value) }
def `position`: Option[Position] = optional("position").flatMap(value => Option(value).map(value => Position.read(value)))
def `question`: ReadableQuestion3 = { val value = required("question"); require(value != null, "Null native member: " + "question"); ReadableQuestion3.read(value) }
def `schema`: Version = { val value = required("schema"); require(value != null, "Null native member: " + "schema"); Version.read(value) }
def `source`: Option[PhysicalSource] = optional("source").flatMap(value => Option(value).map(value => PhysicalSource.read(value)))
def `value`: Recognize = { val value = required("value"); require(value != null, "Null native member: " + "value"); Recognize.read(value) }
}
object Recognition { def read(value: Any): Recognition = new Recognition(value) }
final class RecognitionEdgeDocument(rawValue: Any) extends View(rawValue) {
def `either`: Boolean = { val value = required("either"); require(value != null, "Null native member: " + "either"); value.asInstanceOf[Boolean] }
def `probability`: BigDecimal = { val value = required("probability"); require(value != null, "Null native member: " + "probability"); BigDecimal(value.asInstanceOf[java.math.BigDecimal]) }
def `relation`: String = { val value = required("relation"); require(value != null, "Null native member: " + "relation"); value.asInstanceOf[String] }
def `source`: Entity = { val value = required("source"); require(value != null, "Null native member: " + "source"); Entity.read(value) }
def `target`: Entity = { val value = required("target"); require(value != null, "Null native member: " + "target"); Entity.read(value) }
}
object RecognitionEdgeDocument { def read(value: Any): RecognitionEdgeDocument = new RecognitionEdgeDocument(value) }
final class RecognitionMode(rawValue: Any) extends View(rawValue) {
def value: String = json.asInstanceOf[String]
}
object RecognitionMode { def read(value: Any): RecognitionMode = new RecognitionMode(value) }
sealed trait RecognitionOdds { def json: Any }
object RecognitionOdds {
def read(value: Any): RecognitionOdds = {
val objectValue = Values.`object`(value)
if (objectValue.containsKey("names") && objectValue.containsKey("pairs") && objectValue.containsKey("pieces") && objectValue.containsKey("proposals")) return RecognitionOddsFieldsNamesPairsPiecesProposals.read(value)
if (objectValue.containsKey("pieces") && objectValue.containsKey("proposals") && !objectValue.containsKey("names") && !objectValue.containsKey("pairs")) return RecognitionOddsFieldsPiecesProposals.read(value)
throw new IllegalStateException("Unknown native result alternative")
}
}
final class RecognitionOddsFieldsNamesPairsPiecesProposals(rawValue: Any) extends View(rawValue) with RecognitionOdds {
def `names`: List[NameOdds] = { val value = required("names"); require(value != null, "Null native member: " + "names"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => NameOdds.read(item0)) }
def `pairs`: List[PairOdds] = { val value = required("pairs"); require(value != null, "Null native member: " + "pairs"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => PairOdds.read(item0)) }
def `pieces`: List[PieceOdds] = { val value = required("pieces"); require(value != null, "Null native member: " + "pieces"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => PieceOdds.read(item0)) }
def `proposals`: List[RecognitionProposal] = { val value = required("proposals"); require(value != null, "Null native member: " + "proposals"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => RecognitionProposal.read(item0)) }
}
object RecognitionOddsFieldsNamesPairsPiecesProposals { def read(value: Any): RecognitionOddsFieldsNamesPairsPiecesProposals = new RecognitionOddsFieldsNamesPairsPiecesProposals(value) }
final class RecognitionOddsFieldsPiecesProposals(rawValue: Any) extends View(rawValue) with RecognitionOdds {
def `pieces`: List[PieceOdds] = { val value = required("pieces"); require(value != null, "Null native member: " + "pieces"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => PieceOdds.read(item0)) }
def `proposals`: List[BoundaryProposal] = { val value = required("proposals"); require(value != null, "Null native member: " + "proposals"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => BoundaryProposal.read(item0)) }
}
object RecognitionOddsFieldsPiecesProposals { def read(value: Any): RecognitionOddsFieldsPiecesProposals = new RecognitionOddsFieldsPiecesProposals(value) }
final class RecognitionProposal(rawValue: Any) extends View(rawValue) {
def `end`: BigInt = { val value = required("end"); require(value != null, "Null native member: " + "end"); BigInt(Values.integer(value)) }
def `kept`: Boolean = { val value = required("kept"); require(value != null, "Null native member: " + "kept"); value.asInstanceOf[Boolean] }
def `kind`: Option[String] = optional("kind").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `selected`: Option[Place] = optional("selected").flatMap(value => Option(value).map(value => Place.read(value)))
def `spanProbability`: BigDecimal = { val value = required("span_probability"); require(value != null, "Null native member: " + "span_probability"); BigDecimal(value.asInstanceOf[java.math.BigDecimal]) }
def `start`: BigInt = { val value = required("start"); require(value != null, "Null native member: " + "start"); BigInt(Values.integer(value)) }
def `strength`: Option[BigDecimal] = optional("strength").flatMap(value => Option(value).map(value => BigDecimal(value.asInstanceOf[java.math.BigDecimal])))
}
object RecognitionProposal { def read(value: Any): RecognitionProposal = new RecognitionProposal(value) }
final class RecognitionStageContext(rawValue: Any) extends View(rawValue) {
def `boundary`: Option[String] = optional("boundary").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `kindEdge`: Option[String] = optional("kind_edge").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `relation`: Option[String] = optional("relation").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
}
object RecognitionStageContext { def read(value: Any): RecognitionStageContext = new RecognitionStageContext(value) }
final class Relation(rawValue: Any) extends View(rawValue) {
def `answer`: Answers = { val value = required("answer"); require(value != null, "Null native member: " + "answer"); Answers.read(value) }
def `answerId`: AnswerId = { val value = required("answer_id"); require(value != null, "Null native member: " + "answer_id"); AnswerId.read(value) }
def `file`: Option[String] = optional("file").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `firstLine`: Option[BigInt] = optional("first_line").flatMap(value => Option(value).map(value => BigInt(Values.integer(value))))
def `index`: Option[BigInt] = optional("index").flatMap(value => Option(value).map(value => BigInt(Values.integer(value))))
def `input`: Option[Option[Any]] = optional("input").map(value => Option(value).map(value => Values.freeze(value)))
def `inputSources`: Option[List[SessionInputSource]] = optional("input_sources").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => SessionInputSource.read(item0))))
def `lastLine`: Option[BigInt] = optional("last_line").flatMap(value => Option(value).map(value => BigInt(Values.integer(value))))
def `meta`: Meta = { val value = required("meta"); require(value != null, "Null native member: " + "meta"); Meta.read(value) }
def `position`: Option[Position] = optional("position").flatMap(value => Option(value).map(value => Position.read(value)))
def `question`: ReadableQuestion4 = { val value = required("question"); require(value != null, "Null native member: " + "question"); ReadableQuestion4.read(value) }
def `schema`: Version = { val value = required("schema"); require(value != null, "Null native member: " + "schema"); Version.read(value) }
def `value`: List[RelatedEntityEdge] = { val value = required("value"); require(value != null, "Null native member: " + "value"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => RelatedEntityEdge.read(item0)) }
}
object Relation { def read(value: Any): Relation = new Relation(value) }
final class RelationDirection(rawValue: Any) extends View(rawValue) {
def value: String = json.asInstanceOf[String]
}
object RelationDirection { def read(value: Any): RelationDirection = new RelationDirection(value) }
sealed trait RelationMember { def json: Any }
object RelationMember {
def read(value: Any): RelationMember = {
val objectValue = Values.`object`(value)
if (objectValue.containsKey("answer_id")) return RelationMemberAnswerId.read(value)
if (objectValue.containsKey("failure_id")) return RelationMemberFailureId.read(value)
throw new IllegalStateException("Unknown native result alternative")
}
}
final class RelationMemberAnswerId(rawValue: Any) extends View(rawValue) with RelationMember {
def `direction`: RelationDirection = { val value = required("direction"); require(value != null, "Null native member: " + "direction"); RelationDirection.read(value) }
def `method`: RelationMethod = { val value = required("method"); require(value != null, "Null native member: " + "method"); RelationMethod.read(value) }
def `observations`: List[Observation] = { val value = required("observations"); require(value != null, "Null native member: " + "observations"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => Observation.read(item0)) }
def `question`: ReadableQuestion = { val value = required("question"); require(value != null, "Null native member: " + "question"); ReadableQuestion.read(value) }
def `questionSources`: List[QuestionSource] = { val value = required("question_sources"); require(value != null, "Null native member: " + "question_sources"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => QuestionSource.read(item0)) }
def `reads`: String = { val value = required("reads"); require(value != null, "Null native member: " + "reads"); value.asInstanceOf[String] }
def `relation`: String = { val value = required("relation"); require(value != null, "Null native member: " + "relation"); value.asInstanceOf[String] }
def `request`: String = { val value = required("request"); require(value != null, "Null native member: " + "request"); value.asInstanceOf[String] }
def `source`: RelatedEntity = { val value = required("source"); require(value != null, "Null native member: " + "source"); RelatedEntity.read(value) }
def `target`: Option[RelatedEntity] = Option(required("target")).map(value => RelatedEntity.read(value))
def `threshold`: Threshold = { val value = required("threshold"); require(value != null, "Null native member: " + "threshold"); Threshold.read(value) }
def `usage`: Option[Usage] = optional("usage").flatMap(value => Option(value).map(value => Usage.read(value)))
def `accepted`: Boolean = { val value = required("accepted"); require(value != null, "Null native member: " + "accepted"); value.asInstanceOf[Boolean] }
def `answer`: Answer = { val value = required("answer"); require(value != null, "Null native member: " + "answer"); Answer.read(value) }
def `answerId`: AnswerId = { val value = required("answer_id"); require(value != null, "Null native member: " + "answer_id"); AnswerId.read(value) }
def `probability`: BigDecimal = { val value = required("probability"); require(value != null, "Null native member: " + "probability"); BigDecimal(value.asInstanceOf[java.math.BigDecimal]) }
}
object RelationMemberAnswerId { def read(value: Any): RelationMemberAnswerId = new RelationMemberAnswerId(value) }
final class RelationMemberFailureId(rawValue: Any) extends View(rawValue) with RelationMember {
def `direction`: RelationDirection = { val value = required("direction"); require(value != null, "Null native member: " + "direction"); RelationDirection.read(value) }
def `method`: RelationMethod = { val value = required("method"); require(value != null, "Null native member: " + "method"); RelationMethod.read(value) }
def `observations`: List[Observation] = { val value = required("observations"); require(value != null, "Null native member: " + "observations"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => Observation.read(item0)) }
def `question`: ReadableQuestion = { val value = required("question"); require(value != null, "Null native member: " + "question"); ReadableQuestion.read(value) }
def `questionSources`: List[QuestionSource] = { val value = required("question_sources"); require(value != null, "Null native member: " + "question_sources"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => QuestionSource.read(item0)) }
def `reads`: String = { val value = required("reads"); require(value != null, "Null native member: " + "reads"); value.asInstanceOf[String] }
def `relation`: String = { val value = required("relation"); require(value != null, "Null native member: " + "relation"); value.asInstanceOf[String] }
def `request`: String = { val value = required("request"); require(value != null, "Null native member: " + "request"); value.asInstanceOf[String] }
def `source`: RelatedEntity = { val value = required("source"); require(value != null, "Null native member: " + "source"); RelatedEntity.read(value) }
def `target`: Option[RelatedEntity] = Option(required("target")).map(value => RelatedEntity.read(value))
def `threshold`: Threshold = { val value = required("threshold"); require(value != null, "Null native member: " + "threshold"); Threshold.read(value) }
def `usage`: Option[Usage] = optional("usage").flatMap(value => Option(value).map(value => Usage.read(value)))
def `failure`: Failure = { val value = required("failure"); require(value != null, "Null native member: " + "failure"); Failure.read(value) }
def `failureId`: FailureId = { val value = required("failure_id"); require(value != null, "Null native member: " + "failure_id"); FailureId.read(value) }
}
object RelationMemberFailureId { def read(value: Any): RelationMemberFailureId = new RelationMemberFailureId(value) }
final class RelationMethod(rawValue: Any) extends View(rawValue) {
def value: String = json.asInstanceOf[String]
}
object RelationMethod { def read(value: Any): RelationMethod = new RelationMethod(value) }
final class RequestFunction(rawValue: Any) extends View(rawValue) {
def value: String = json.asInstanceOf[String]
}
object RequestFunction { def read(value: Any): RequestFunction = new RequestFunction(value) }
final class SdkRequestId(rawValue: Any) extends View(rawValue) {
def value: String = json.asInstanceOf[String]
}
object SdkRequestId { def read(value: Any): SdkRequestId = new SdkRequestId(value) }
sealed trait SendBudgetDenial { def json: Any }
object SendBudgetDenial {
def read(value: Any): SendBudgetDenial = {
val objectValue = Values.`object`(value)
if (objectValue.get("kind") == "before_first_send") return SendBudgetDenialBeforeFirstSend.read(value)
if (objectValue.get("kind") == "before_additional_send") return SendBudgetDenialBeforeAdditionalSend.read(value)
if (objectValue.get("kind") == "before_retry") return SendBudgetDenialBeforeRetry.read(value)
throw new IllegalStateException("Unknown native result alternative")
}
}
final class SendBudgetDenialBeforeAdditionalSend(rawValue: Any) extends View(rawValue) with SendBudgetDenial {
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
}
object SendBudgetDenialBeforeAdditionalSend { def read(value: Any): SendBudgetDenialBeforeAdditionalSend = new SendBudgetDenialBeforeAdditionalSend(value) }
final class SendBudgetDenialBeforeFirstSend(rawValue: Any) extends View(rawValue) with SendBudgetDenial {
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
}
object SendBudgetDenialBeforeFirstSend { def read(value: Any): SendBudgetDenialBeforeFirstSend = new SendBudgetDenialBeforeFirstSend(value) }
final class SendBudgetDenialBeforeRetry(rawValue: Any) extends View(rawValue) with SendBudgetDenial {
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `lastStatus`: BigInt = { val value = required("last_status"); require(value != null, "Null native member: " + "last_status"); BigInt(Values.integer(value)) }
}
object SendBudgetDenialBeforeRetry { def read(value: Any): SendBudgetDenialBeforeRetry = new SendBudgetDenialBeforeRetry(value) }
final class StopCause(rawValue: Any) extends View(rawValue) {
def value: String = json.asInstanceOf[String]
}
object StopCause { def read(value: Any): StopCause = new StopCause(value) }
final class Stopped(rawValue: Any) extends View(rawValue) {
def `at`: Option[BigInt] = optional("at").flatMap(value => Option(value).map(value => BigInt(Values.integer(value))))
def `cause`: StopCause = { val value = required("cause"); require(value != null, "Null native member: " + "cause"); StopCause.read(value) }
def `retryable`: Boolean = { val value = required("retryable"); require(value != null, "Null native member: " + "retryable"); value.asInstanceOf[Boolean] }
def `status`: Option[BigInt] = optional("status").flatMap(value => Option(value).map(value => BigInt(Values.integer(value))))
}
object Stopped { def read(value: Any): Stopped = new Stopped(value) }
final class StringRoot(rawValue: Any) extends View(rawValue) {
def `type`: StringType = { val value = required("type"); require(value != null, "Null native member: " + "type"); StringType.read(value) }
}
object StringRoot { def read(value: Any): StringRoot = new StringRoot(value) }
final class StringType(rawValue: Any) extends View(rawValue) {
def value: String = json.asInstanceOf[String]
}
object StringType { def read(value: Any): StringType = new StringType(value) }
final class Usage(rawValue: Any) extends View(rawValue) {
def `inputTokens`: Option[BigInt] = optional("input_tokens").flatMap(value => Option(value).map(value => BigInt(Values.integer(value))))
def `outputTokens`: Option[BigInt] = optional("output_tokens").flatMap(value => Option(value).map(value => BigInt(Values.integer(value))))
}
object Usage { def read(value: Any): Usage = new Usage(value) }
final class UsagePersistence(rawValue: Any) extends View(rawValue) {
def value: String = json.asInstanceOf[String]
}
object UsagePersistence { def read(value: Any): UsagePersistence = new UsagePersistence(value) }
final class Verb(rawValue: Any) extends View(rawValue) {
def value: String = json.asInstanceOf[String]
}
object Verb { def read(value: Any): Verb = new Verb(value) }
final class Version(rawValue: Any) extends View(rawValue) {
def value: String = json.asInstanceOf[String]
}
object Version { def read(value: Any): Version = new Version(value) }
final class WordingVersion(rawValue: Any) extends View(rawValue) {
def value: BigInt = BigInt(Values.integer(json))
}
object WordingVersion { def read(value: Any): WordingVersion = new WordingVersion(value) }
sealed trait AnnotatedField { def json: Any }
object AnnotatedField {
def read(value: Any): AnnotatedField = {
if (value.isInstanceOf[Boolean]) return AnnotatedFieldBoolean.read(value)
if (value == null) return AnnotatedFieldNull.read(value)
if (value.isInstanceOf[String]) return AnnotatedFieldString.read(value)
if (value.isInstanceOf[java.util.List[?]]) return AnnotatedFieldArray.read(value)
if (value.isInstanceOf[java.math.BigDecimal]) return AnnotatedFieldNumber.read(value)
if (value.isInstanceOf[java.util.Map[?, ?]]) return AnnotatedFieldObject.read(value)
throw new IllegalStateException("Unknown native result alternative")
}
}
final class AnnotatedFieldBoolean(val json: Any) extends AnnotatedField { def value: Boolean = json.asInstanceOf[Boolean] }
object AnnotatedFieldBoolean { def read(value: Any): AnnotatedFieldBoolean = new AnnotatedFieldBoolean(Values.freeze(value)) }
final class AnnotatedFieldNull(val json: Any) extends AnnotatedField { def value: Any = Values.freeze(json) }
object AnnotatedFieldNull { def read(value: Any): AnnotatedFieldNull = new AnnotatedFieldNull(Values.freeze(value)) }
final class AnnotatedFieldString(val json: Any) extends AnnotatedField { def value: String = json.asInstanceOf[String] }
object AnnotatedFieldString { def read(value: Any): AnnotatedFieldString = new AnnotatedFieldString(Values.freeze(value)) }
final class AnnotatedFieldArray(val json: Any) extends AnnotatedField { def value: List[String] = json.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => item0.asInstanceOf[String]) }
object AnnotatedFieldArray { def read(value: Any): AnnotatedFieldArray = new AnnotatedFieldArray(Values.freeze(value)) }
final class AnnotatedFieldNumber(val json: Any) extends AnnotatedField { def value: BigDecimal = BigDecimal(json.asInstanceOf[java.math.BigDecimal]) }
object AnnotatedFieldNumber { def read(value: Any): AnnotatedFieldNumber = new AnnotatedFieldNumber(Values.freeze(value)) }
final class AnnotatedFieldObject(val json: Any) extends AnnotatedField { def value: Failed = Failed.read(json) }
object AnnotatedFieldObject { def read(value: Any): AnnotatedFieldObject = new AnnotatedFieldObject(Values.freeze(value)) }
final class AnnotatedRow(rawValue: Any) extends View(rawValue) {
def value: Map[String, AnnotatedField] = Values.`object`(json).asScala.toMap.map { case (key, item0) => key -> (AnnotatedField.read(item0)) }
}
object AnnotatedRow { def read(value: Any): AnnotatedRow = new AnnotatedRow(value) }
sealed trait Answer { def json: Any }
object Answer {
def read(value: Any): Answer = {
val objectValue = Values.`object`(value)
if (objectValue.get("kind") == "yes_no") return AnswerYesNo.read(value)
if (objectValue.get("kind") == "choice") return AnswerChoice.read(value)
if (objectValue.get("kind") == "tag") return AnswerTag.read(value)
if (objectValue.get("kind") == "score") return AnswerScore.read(value)
throw new IllegalStateException("Unknown native result alternative")
}
}
final class AnswerChoice(rawValue: Any) extends View(rawValue) with Answer {
def `confidence`: Option[BigDecimal] = optional("confidence").flatMap(value => Option(value).map(value => BigDecimal(value.asInstanceOf[java.math.BigDecimal])))
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `pick`: String = { val value = required("pick"); require(value != null, "Null native member: " + "pick"); value.asInstanceOf[String] }
def `probabilities`: Map[String, BigDecimal] = { val value = required("probabilities"); require(value != null, "Null native member: " + "probabilities"); Values.`object`(value).asScala.toMap.map { case (key, item0) => key -> (BigDecimal(item0.asInstanceOf[java.math.BigDecimal])) } }
}
object AnswerChoice { def read(value: Any): AnswerChoice = new AnswerChoice(value) }
final class AnswerScore(rawValue: Any) extends View(rawValue) with Answer {
def `confidence`: Option[BigDecimal] = optional("confidence").flatMap(value => Option(value).map(value => BigDecimal(value.asInstanceOf[java.math.BigDecimal])))
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `level`: String = { val value = required("level"); require(value != null, "Null native member: " + "level"); value.asInstanceOf[String] }
def `probabilities`: Map[String, BigDecimal] = { val value = required("probabilities"); require(value != null, "Null native member: " + "probabilities"); Values.`object`(value).asScala.toMap.map { case (key, item0) => key -> (BigDecimal(item0.asInstanceOf[java.math.BigDecimal])) } }
}
object AnswerScore { def read(value: Any): AnswerScore = new AnswerScore(value) }
final class AnswerTag(rawValue: Any) extends View(rawValue) with Answer {
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `probabilities`: Map[String, BigDecimal] = { val value = required("probabilities"); require(value != null, "Null native member: " + "probabilities"); Values.`object`(value).asScala.toMap.map { case (key, item0) => key -> (BigDecimal(item0.asInstanceOf[java.math.BigDecimal])) } }
}
object AnswerTag { def read(value: Any): AnswerTag = new AnswerTag(value) }
final class AnswerYesNo(rawValue: Any) extends View(rawValue) with Answer {
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `probability`: BigDecimal = { val value = required("probability"); require(value != null, "Null native member: " + "probability"); BigDecimal(value.asInstanceOf[java.math.BigDecimal]) }
}
object AnswerYesNo { def read(value: Any): AnswerYesNo = new AnswerYesNo(value) }
final class AttemptOutcome(rawValue: Any) extends View(rawValue) {
def value: String = json.asInstanceOf[String]
}
object AttemptOutcome { def read(value: Any): AttemptOutcome = new AttemptOutcome(value) }
sealed trait BatchSetting { def json: Any }
object BatchSetting {
def read(value: Any): BatchSetting = {
if (value.isInstanceOf[java.math.BigDecimal]) return BatchSettingInteger.read(value)
if (value.isInstanceOf[String]) return BatchSettingString.read(value)
throw new IllegalStateException("Unknown native result alternative")
}
}
final class BatchSettingInteger(val json: Any) extends BatchSetting { def value: BigInt = BigInt(Values.integer(json)) }
object BatchSettingInteger { def read(value: Any): BatchSettingInteger = new BatchSettingInteger(Values.freeze(value)) }
final class BatchSettingString(val json: Any) extends BatchSetting { def value: String = json.asInstanceOf[String] }
object BatchSettingString { def read(value: Any): BatchSettingString = new BatchSettingString(Values.freeze(value)) }
final class BatchWarning(rawValue: Any) extends View(rawValue) {
def `running`: BatchSetting = { val value = required("running"); require(value != null, "Null native member: " + "running"); BatchSetting.read(value) }
def `tunedFor`: BatchSetting = { val value = required("tuned_for"); require(value != null, "Null native member: " + "tuned_for"); BatchSetting.read(value) }
}
object BatchWarning { def read(value: Any): BatchWarning = new BatchWarning(value) }
final class Entity(rawValue: Any) extends View(rawValue) {
def `end`: BigInt = { val value = required("end"); require(value != null, "Null native member: " + "end"); BigInt(Values.integer(value)) }
def `file`: Option[String] = optional("file").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `firstLine`: Option[BigInt] = optional("first_line").flatMap(value => Option(value).map(value => BigInt(Values.integer(value))))
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `lastLine`: Option[BigInt] = optional("last_line").flatMap(value => Option(value).map(value => BigInt(Values.integer(value))))
def `length`: BigInt = { val value = required("length"); require(value != null, "Null native member: " + "length"); BigInt(Values.integer(value)) }
def `start`: BigInt = { val value = required("start"); require(value != null, "Null native member: " + "start"); BigInt(Values.integer(value)) }
def `strength`: BigDecimal = { val value = required("strength"); require(value != null, "Null native member: " + "strength"); BigDecimal(value.asInstanceOf[java.math.BigDecimal]) }
def `text`: String = { val value = required("text"); require(value != null, "Null native member: " + "text"); value.asInstanceOf[String] }
}
object Entity { def read(value: Any): Entity = new Entity(value) }
final class EntityEdge(rawValue: Any) extends View(rawValue) {
def `either`: Option[Boolean] = optional("either").flatMap(value => Option(value).map(value => value.asInstanceOf[Boolean]))
def `probability`: BigDecimal = { val value = required("probability"); require(value != null, "Null native member: " + "probability"); BigDecimal(value.asInstanceOf[java.math.BigDecimal]) }
def `relation`: String = { val value = required("relation"); require(value != null, "Null native member: " + "relation"); value.asInstanceOf[String] }
def `source`: Entity = { val value = required("source"); require(value != null, "Null native member: " + "source"); Entity.read(value) }
def `target`: Entity = { val value = required("target"); require(value != null, "Null native member: " + "target"); Entity.read(value) }
}
object EntityEdge { def read(value: Any): EntityEdge = new EntityEdge(value) }
final class Failed(rawValue: Any) extends View(rawValue) {
def `failed`: Failure = { val value = required("failed"); require(value != null, "Null native member: " + "failed"); Failure.read(value) }
}
object Failed { def read(value: Any): Failed = new Failed(value) }
final class Failure(rawValue: Any) extends View(rawValue) {
def `cause`: FailureCause = { val value = required("cause"); require(value != null, "Null native member: " + "cause"); FailureCause.read(value) }
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
}
object Failure { def read(value: Any): Failure = new Failure(value) }
final class FailureCause(rawValue: Any) extends View(rawValue) {
def value: String = json.asInstanceOf[String]
}
object FailureCause { def read(value: Any): FailureCause = new FailureCause(value) }
final class FailureKind(rawValue: Any) extends View(rawValue) {
def value: String = json.asInstanceOf[String]
}
object FailureKind { def read(value: Any): FailureKind = new FailureKind(value) }
final class FindAnswer(rawValue: Any) extends View(rawValue) {
def `confidence`: Option[BigDecimal] = optional("confidence").flatMap(value => Option(value).map(value => BigDecimal(value.asInstanceOf[java.math.BigDecimal])))
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `pick`: String = { val value = required("pick"); require(value != null, "Null native member: " + "pick"); value.asInstanceOf[String] }
def `probabilities`: Map[String, BigDecimal] = { val value = required("probabilities"); require(value != null, "Null native member: " + "probabilities"); Values.`object`(value).asScala.toMap.map { case (key, item0) => key -> (BigDecimal(item0.asInstanceOf[java.math.BigDecimal])) } }
}
object FindAnswer { def read(value: Any): FindAnswer = new FindAnswer(value) }
final class NameOdds(rawValue: Any) extends View(rawValue) {
def `edges`: Option[Map[String, BigDecimal]] = Option(required("edges")).map(value => Values.`object`(value).asScala.toMap.map { case (key, item0) => key -> (BigDecimal(item0.asInstanceOf[java.math.BigDecimal])) })
def `end`: BigInt = { val value = required("end"); require(value != null, "Null native member: " + "end"); BigInt(Values.integer(value)) }
def `kinds`: Option[Map[String, BigDecimal]] = Option(required("kinds")).map(value => Values.`object`(value).asScala.toMap.map { case (key, item0) => key -> (BigDecimal(item0.asInstanceOf[java.math.BigDecimal])) })
def `start`: BigInt = { val value = required("start"); require(value != null, "Null native member: " + "start"); BigInt(Values.integer(value)) }
}
object NameOdds { def read(value: Any): NameOdds = new NameOdds(value) }
final class PairOdds(rawValue: Any) extends View(rawValue) {
def `probability`: BigDecimal = { val value = required("probability"); require(value != null, "Null native member: " + "probability"); BigDecimal(value.asInstanceOf[java.math.BigDecimal]) }
def `relation`: String = { val value = required("relation"); require(value != null, "Null native member: " + "relation"); value.asInstanceOf[String] }
def `source`: Place = { val value = required("source"); require(value != null, "Null native member: " + "source"); Place.read(value) }
def `target`: Place = { val value = required("target"); require(value != null, "Null native member: " + "target"); Place.read(value) }
}
object PairOdds { def read(value: Any): PairOdds = new PairOdds(value) }
final class PieceOdds(rawValue: Any) extends View(rawValue) {
def `end`: BigInt = { val value = required("end"); require(value != null, "Null native member: " + "end"); BigInt(Values.integer(value)) }
def `start`: BigInt = { val value = required("start"); require(value != null, "Null native member: " + "start"); BigInt(Values.integer(value)) }
def `tags`: Map[String, BigDecimal] = { val value = required("tags"); require(value != null, "Null native member: " + "tags"); Values.`object`(value).asScala.toMap.map { case (key, item0) => key -> (BigDecimal(item0.asInstanceOf[java.math.BigDecimal])) } }
}
object PieceOdds { def read(value: Any): PieceOdds = new PieceOdds(value) }
final class Place(rawValue: Any) extends View(rawValue) {
def `end`: BigInt = { val value = required("end"); require(value != null, "Null native member: " + "end"); BigInt(Values.integer(value)) }
def `start`: BigInt = { val value = required("start"); require(value != null, "Null native member: " + "start"); BigInt(Values.integer(value)) }
}
object Place { def read(value: Any): Place = new Place(value) }
final class Plan(rawValue: Any) extends View(rawValue) {
def `estimatedBytes`: BigInt = { val value = required("estimated_bytes"); require(value != null, "Null native member: " + "estimated_bytes"); BigInt(Values.integer(value)) }
def `estimatedInputTokens`: TokenBand = { val value = required("estimated_input_tokens"); require(value != null, "Null native member: " + "estimated_input_tokens"); TokenBand.read(value) }
def `firstBodyUtf8`: Option[String] = Option(required("first_body_utf8")).map(value => value.asInstanceOf[String])
def `largestRequestBytes`: BigInt = { val value = required("largest_request_bytes"); require(value != null, "Null native member: " + "largest_request_bytes"); BigInt(Values.integer(value)) }
def `largestRequestEstimatedInputTokens`: BigInt = { val value = required("largest_request_estimated_input_tokens"); require(value != null, "Null native member: " + "largest_request_estimated_input_tokens"); BigInt(Values.integer(value)) }
def `records`: BigInt = { val value = required("records"); require(value != null, "Null native member: " + "records"); BigInt(Values.integer(value)) }
def `requests`: BigInt = { val value = required("requests"); require(value != null, "Null native member: " + "requests"); BigInt(Values.integer(value)) }
def `tokenEstimateMethod`: String = { val value = required("token_estimate_method"); require(value != null, "Null native member: " + "token_estimate_method"); value.asInstanceOf[String] }
def `upperBound`: Boolean = { val value = required("upper_bound"); require(value != null, "Null native member: " + "upper_bound"); value.asInstanceOf[Boolean] }
}
object Plan { def read(value: Any): Plan = new Plan(value) }
final class ProfileWarning(rawValue: Any) extends View(rawValue) {
def `running`: String = { val value = required("running"); require(value != null, "Null native member: " + "running"); value.asInstanceOf[String] }
def `tunedFor`: String = { val value = required("tuned_for"); require(value != null, "Null native member: " + "tuned_for"); value.asInstanceOf[String] }
}
object ProfileWarning { def read(value: Any): ProfileWarning = new ProfileWarning(value) }
sealed trait Recognize { def json: Any }
object Recognize {
def read(value: Any): Recognize = {
val objectValue = Values.`object`(value)
if (objectValue.containsKey("entities") && !objectValue.containsKey("mode") && !objectValue.containsKey("proposals")) return RecognizeFieldsEntities.read(value)
if (objectValue.containsKey("mode") && objectValue.containsKey("proposals") && !objectValue.containsKey("entities")) return RecognizeFieldsModeProposals.read(value)
throw new IllegalStateException("Unknown native result alternative")
}
}
final class RecognizeAnswer(rawValue: Any) extends View(rawValue) {
def `names`: List[NameOdds] = { val value = required("names"); require(value != null, "Null native member: " + "names"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => NameOdds.read(item0)) }
def `pairs`: List[PairOdds] = { val value = required("pairs"); require(value != null, "Null native member: " + "pairs"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => PairOdds.read(item0)) }
def `pieces`: List[PieceOdds] = { val value = required("pieces"); require(value != null, "Null native member: " + "pieces"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => PieceOdds.read(item0)) }
def `proposals`: List[RecognitionProposal] = { val value = required("proposals"); require(value != null, "Null native member: " + "proposals"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => RecognitionProposal.read(item0)) }
}
object RecognizeAnswer { def read(value: Any): RecognizeAnswer = new RecognizeAnswer(value) }
final class RecognizeFieldsEntities(rawValue: Any) extends View(rawValue) with Recognize {
def `entities`: List[Entity] = { val value = required("entities"); require(value != null, "Null native member: " + "entities"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => Entity.read(item0)) }
def `relations`: Option[List[EntityEdge]] = optional("relations").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => EntityEdge.read(item0))))
}
object RecognizeFieldsEntities { def read(value: Any): RecognizeFieldsEntities = new RecognizeFieldsEntities(value) }
final class RecognizeFieldsModeProposals(rawValue: Any) extends View(rawValue) with Recognize {
def `mode`: BoundaryMode = { val value = required("mode"); require(value != null, "Null native member: " + "mode"); BoundaryMode.read(value) }
def `proposals`: List[BoundaryProposal] = { val value = required("proposals"); require(value != null, "Null native member: " + "proposals"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => BoundaryProposal.read(item0)) }
}
object RecognizeFieldsModeProposals { def read(value: Any): RecognizeFieldsModeProposals = new RecognizeFieldsModeProposals(value) }
final class RelateFields(rawValue: Any) extends View(rawValue) {
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `name`: String = { val value = required("name"); require(value != null, "Null native member: " + "name"); value.asInstanceOf[String] }
}
object RelateFields { def read(value: Any): RelateFields = new RelateFields(value) }
final class RelatedEntity(rawValue: Any) extends View(rawValue) {
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `name`: String = { val value = required("name"); require(value != null, "Null native member: " + "name"); value.asInstanceOf[String] }
}
object RelatedEntity { def read(value: Any): RelatedEntity = new RelatedEntity(value) }
final class RelatedEntityEdge(rawValue: Any) extends View(rawValue) {
def `either`: Option[Boolean] = optional("either").flatMap(value => Option(value).map(value => value.asInstanceOf[Boolean]))
def `probability`: BigDecimal = { val value = required("probability"); require(value != null, "Null native member: " + "probability"); BigDecimal(value.asInstanceOf[java.math.BigDecimal]) }
def `relation`: String = { val value = required("relation"); require(value != null, "Null native member: " + "relation"); value.asInstanceOf[String] }
def `source`: RelatedEntityEdgePropertiesSource = { val value = required("source"); require(value != null, "Null native member: " + "source"); RelatedEntityEdgePropertiesSource.read(value) }
def `target`: RelatedEntityEdgePropertiesSource = { val value = required("target"); require(value != null, "Null native member: " + "target"); RelatedEntityEdgePropertiesSource.read(value) }
}
object RelatedEntityEdge { def read(value: Any): RelatedEntityEdge = new RelatedEntityEdge(value) }
sealed trait RelatedEntityEdgePropertiesSource { def json: Any }
object RelatedEntityEdgePropertiesSource {
def read(value: Any): RelatedEntityEdgePropertiesSource = {
val objectValue = Values.`object`(value)
if (objectValue.containsKey("kind") && objectValue.containsKey("name") && !objectValue.containsKey("file") && !objectValue.containsKey("ordinal") && !objectValue.containsKey("record")) return RelatedEntityEdgePropertiesSourceFieldsKindName.read(value)
if (objectValue.containsKey("file") && objectValue.containsKey("kind") && objectValue.containsKey("name") && objectValue.containsKey("ordinal") && objectValue.containsKey("record")) return RelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord.read(value)
throw new IllegalStateException("Unknown native result alternative")
}
}
final class RelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord(rawValue: Any) extends View(rawValue) with RelatedEntityEdgePropertiesSource {
def `file`: Option[String] = Option(required("file")).map(value => value.asInstanceOf[String])
def `firstLine`: Option[BigInt] = optional("first_line").flatMap(value => Option(value).map(value => BigInt(Values.integer(value))))
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `lastLine`: Option[BigInt] = optional("last_line").flatMap(value => Option(value).map(value => BigInt(Values.integer(value))))
def `name`: String = { val value = required("name"); require(value != null, "Null native member: " + "name"); value.asInstanceOf[String] }
def `ordinal`: BigInt = { val value = required("ordinal"); require(value != null, "Null native member: " + "ordinal"); BigInt(Values.integer(value)) }
def `record`: Any = { val value = required("record"); require(value != null, "Null native member: " + "record"); Values.freeze(value) }
}
object RelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord { def read(value: Any): RelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord = new RelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord(value) }
final class RelatedEntityEdgePropertiesSourceFieldsKindName(rawValue: Any) extends View(rawValue) with RelatedEntityEdgePropertiesSource {
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `name`: String = { val value = required("name"); require(value != null, "Null native member: " + "name"); value.asInstanceOf[String] }
}
object RelatedEntityEdgePropertiesSourceFieldsKindName { def read(value: Any): RelatedEntityEdgePropertiesSourceFieldsKindName = new RelatedEntityEdgePropertiesSourceFieldsKindName(value) }
final class RelationRule(rawValue: Any) extends View(rawValue) {
def `either`: Boolean = { val value = required("either"); require(value != null, "Null native member: " + "either"); value.asInstanceOf[Boolean] }
def `name`: String = { val value = required("name"); require(value != null, "Null native member: " + "name"); value.asInstanceOf[String] }
def `reads`: String = { val value = required("reads"); require(value != null, "Null native member: " + "reads"); value.asInstanceOf[String] }
def `single`: Option[Boolean] = optional("single").flatMap(value => Option(value).map(value => value.asInstanceOf[Boolean]))
def `source`: String = { val value = required("source"); require(value != null, "Null native member: " + "source"); value.asInstanceOf[String] }
def `target`: String = { val value = required("target"); require(value != null, "Null native member: " + "target"); value.asInstanceOf[String] }
}
object RelationRule { def read(value: Any): RelationRule = new RelationRule(value) }
final class SessionAnnotation(rawValue: Any) extends View(rawValue) {
def `name`: String = { val value = required("name"); require(value != null, "Null native member: " + "name"); value.asInstanceOf[String] }
def `value`: AnnotationValue = { val value = required("value"); require(value != null, "Null native member: " + "value"); AnnotationValue.read(value) }
}
object SessionAnnotation { def read(value: Any): SessionAnnotation = new SessionAnnotation(value) }
final class SessionInputSource(rawValue: Any) extends View(rawValue) {
def `index`: BigInt = { val value = required("index"); require(value != null, "Null native member: " + "index"); BigInt(Values.integer(value)) }
def `source`: PhysicalSource = { val value = required("source"); require(value != null, "Null native member: " + "source"); PhysicalSource.read(value) }
}
object SessionInputSource { def read(value: Any): SessionInputSource = new SessionInputSource(value) }
sealed trait SessionJudgment { def json: Any }
object SessionJudgment {
def read(value: Any): SessionJudgment = {
val objectValue = Values.`object`(value)
if (objectValue.get("kind") == "decision") return SessionJudgmentDecision.read(value)
if (objectValue.get("kind") == "choice") return SessionJudgmentChoice.read(value)
if (objectValue.get("kind") == "score") return SessionJudgmentScore.read(value)
if (objectValue.get("kind") == "tags") return SessionJudgmentTags.read(value)
throw new IllegalStateException("Unknown native result alternative")
}
}
final class SessionJudgmentChoice(rawValue: Any) extends View(rawValue) with SessionJudgment {
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `value`: Option[String] = Option(required("value")).map(value => value.asInstanceOf[String])
}
object SessionJudgmentChoice { def read(value: Any): SessionJudgmentChoice = new SessionJudgmentChoice(value) }
final class SessionJudgmentDecision(rawValue: Any) extends View(rawValue) with SessionJudgment {
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `value`: Option[Boolean] = Option(required("value")).map(value => value.asInstanceOf[Boolean])
}
object SessionJudgmentDecision { def read(value: Any): SessionJudgmentDecision = new SessionJudgmentDecision(value) }
final class SessionJudgmentScore(rawValue: Any) extends View(rawValue) with SessionJudgment {
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `value`: BigDecimal = { val value = required("value"); require(value != null, "Null native member: " + "value"); BigDecimal(value.asInstanceOf[java.math.BigDecimal]) }
}
object SessionJudgmentScore { def read(value: Any): SessionJudgmentScore = new SessionJudgmentScore(value) }
final class SessionJudgmentTags(rawValue: Any) extends View(rawValue) with SessionJudgment {
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `value`: List[String] = { val value = required("value"); require(value != null, "Null native member: " + "value"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => item0.asInstanceOf[String]) }
}
object SessionJudgmentTags { def read(value: Any): SessionJudgmentTags = new SessionJudgmentTags(value) }
final class SessionNamedProbability(rawValue: Any) extends View(rawValue) {
def `name`: String = { val value = required("name"); require(value != null, "Null native member: " + "name"); value.asInstanceOf[String] }
def `probability`: BigDecimal = { val value = required("probability"); require(value != null, "Null native member: " + "probability"); BigDecimal(value.asInstanceOf[java.math.BigDecimal]) }
}
object SessionNamedProbability { def read(value: Any): SessionNamedProbability = new SessionNamedProbability(value) }
sealed trait SessionObservation { def json: Any }
object SessionObservation {
def read(value: Any): SessionObservation = {
val objectValue = Values.`object`(value)
if (objectValue.get("kind") == "question") return SessionObservationQuestion.read(value)
if (objectValue.get("kind") == "row") return SessionObservationRow.read(value)
throw new IllegalStateException("Unknown native result alternative")
}
}
final class SessionObservationQuestion(rawValue: Any) extends View(rawValue) with SessionObservation {
def `detail`: SessionQuestionDetail = { val value = required("detail"); require(value != null, "Null native member: " + "detail"); SessionQuestionDetail.read(value) }
def `index`: BigInt = { val value = required("index"); require(value != null, "Null native member: " + "index"); BigInt(Values.integer(value)) }
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `member`: Option[String] = optional("member").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `position`: BigInt = { val value = required("position"); require(value != null, "Null native member: " + "position"); BigInt(Values.integer(value)) }
def `stage`: Option[String] = optional("stage").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
}
object SessionObservationQuestion { def read(value: Any): SessionObservationQuestion = new SessionObservationQuestion(value) }
final class SessionObservationRow(rawValue: Any) extends View(rawValue) with SessionObservation {
def `index`: BigInt = { val value = required("index"); require(value != null, "Null native member: " + "index"); BigInt(Values.integer(value)) }
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `value`: SessionObservedRow = { val value = required("value"); require(value != null, "Null native member: " + "value"); SessionObservedRow.read(value) }
}
object SessionObservationRow { def read(value: Any): SessionObservationRow = new SessionObservationRow(value) }
sealed trait SessionObservedRow { def json: Any }
object SessionObservedRow {
def read(value: Any): SessionObservedRow = {
val objectValue = Values.`object`(value)
if (objectValue.get("kind") == "judgment") return SessionObservedRowJudgment.read(value)
if (objectValue.get("kind") == "annotated") return SessionObservedRowAnnotated.read(value)
if (objectValue.get("kind") == "recognized") return SessionObservedRowRecognized.read(value)
if (objectValue.get("kind") == "find") return SessionObservedRowFind.read(value)
if (objectValue.get("kind") == "relations") return SessionObservedRowRelations.read(value)
throw new IllegalStateException("Unknown native result alternative")
}
}
final class SessionObservedRowAnnotated(rawValue: Any) extends View(rawValue) with SessionObservedRow {
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `value`: List[SessionAnnotation] = { val value = required("value"); require(value != null, "Null native member: " + "value"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => SessionAnnotation.read(item0)) }
}
object SessionObservedRowAnnotated { def read(value: Any): SessionObservedRowAnnotated = new SessionObservedRowAnnotated(value) }
final class SessionObservedRowFind(rawValue: Any) extends View(rawValue) with SessionObservedRow {
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `value`: Option[BigInt] = Option(required("value")).map(value => BigInt(Values.integer(value)))
}
object SessionObservedRowFind { def read(value: Any): SessionObservedRowFind = new SessionObservedRowFind(value) }
final class SessionObservedRowJudgment(rawValue: Any) extends View(rawValue) with SessionObservedRow {
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `value`: SessionJudgment = { val value = required("value"); require(value != null, "Null native member: " + "value"); SessionJudgment.read(value) }
}
object SessionObservedRowJudgment { def read(value: Any): SessionObservedRowJudgment = new SessionObservedRowJudgment(value) }
final class SessionObservedRowRecognized(rawValue: Any) extends View(rawValue) with SessionObservedRow {
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `value`: SessionRecognition = { val value = required("value"); require(value != null, "Null native member: " + "value"); SessionRecognition.read(value) }
}
object SessionObservedRowRecognized { def read(value: Any): SessionObservedRowRecognized = new SessionObservedRowRecognized(value) }
final class SessionObservedRowRelations(rawValue: Any) extends View(rawValue) with SessionObservedRow {
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `value`: List[SessionRelationEdge] = { val value = required("value"); require(value != null, "Null native member: " + "value"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => SessionRelationEdge.read(item0)) }
}
object SessionObservedRowRelations { def read(value: Any): SessionObservedRowRelations = new SessionObservedRowRelations(value) }
sealed trait SessionPacket { def json: Any }
object SessionPacket {
def read(value: Any): SessionPacket = {
val objectValue = Values.`object`(value)
if (objectValue.get("function") == "decide" && objectValue.get("kind") == "row") return SessionPacketDecideRow.read(value)
if (objectValue.get("function") == "choose" && objectValue.get("kind") == "row") return SessionPacketChooseRow.read(value)
if (objectValue.get("function") == "tag" && objectValue.get("kind") == "row") return SessionPacketTagRow.read(value)
if (objectValue.get("function") == "score" && objectValue.get("kind") == "row") return SessionPacketScoreRow.read(value)
if (objectValue.get("function") == "filter" && objectValue.get("kind") == "row") return SessionPacketFilterRow.read(value)
if (objectValue.get("function") == "annotate" && objectValue.get("kind") == "row") return SessionPacketAnnotateRow.read(value)
if (objectValue.get("function") == "decide" && objectValue.get("kind") == "aggregate") return SessionPacketDecideAggregate.read(value)
if (objectValue.get("function") == "choose" && objectValue.get("kind") == "aggregate") return SessionPacketChooseAggregate.read(value)
if (objectValue.get("function") == "tag" && objectValue.get("kind") == "aggregate") return SessionPacketTagAggregate.read(value)
if (objectValue.get("function") == "score" && objectValue.get("kind") == "aggregate") return SessionPacketScoreAggregate.read(value)
if (objectValue.get("function") == "filter" && objectValue.get("kind") == "aggregate") return SessionPacketFilterAggregate.read(value)
if (objectValue.get("function") == "rank" && objectValue.get("kind") == "aggregate") return SessionPacketRankAggregate.read(value)
if (objectValue.get("function") == "find" && objectValue.get("kind") == "aggregate") return SessionPacketFindAggregate.read(value)
if (objectValue.get("function") == "annotate" && objectValue.get("kind") == "aggregate") return SessionPacketAnnotateAggregate.read(value)
if (objectValue.get("function") == "recognize" && objectValue.get("kind") == "aggregate") return SessionPacketRecognizeAggregate.read(value)
if (objectValue.get("function") == "relate" && objectValue.get("kind") == "aggregate") return SessionPacketRelateAggregate.read(value)
if (objectValue.get("kind") == "observation") return SessionPacketObservation.read(value)
if (objectValue.get("kind") == "terminal") return SessionPacketTerminal.read(value)
throw new IllegalStateException("Unknown native result alternative")
}
}
final class SessionPacketAnnotateAggregate(rawValue: Any) extends View(rawValue) with SessionPacket {
def `function`: String = { val value = required("function"); require(value != null, "Null native member: " + "function"); value.asInstanceOf[String] }
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `value`: List[Annotation] = { val value = required("value"); require(value != null, "Null native member: " + "value"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => Annotation.read(item0)) }
}
object SessionPacketAnnotateAggregate { def read(value: Any): SessionPacketAnnotateAggregate = new SessionPacketAnnotateAggregate(value) }
final class SessionPacketAnnotateRow(rawValue: Any) extends View(rawValue) with SessionPacket {
def `function`: String = { val value = required("function"); require(value != null, "Null native member: " + "function"); value.asInstanceOf[String] }
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `value`: Annotation = { val value = required("value"); require(value != null, "Null native member: " + "value"); Annotation.read(value) }
}
object SessionPacketAnnotateRow { def read(value: Any): SessionPacketAnnotateRow = new SessionPacketAnnotateRow(value) }
final class SessionPacketChooseAggregate(rawValue: Any) extends View(rawValue) with SessionPacket {
def `function`: String = { val value = required("function"); require(value != null, "Null native member: " + "function"); value.asInstanceOf[String] }
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `value`: List[AtomicNullableString] = { val value = required("value"); require(value != null, "Null native member: " + "value"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => AtomicNullableString.read(item0)) }
}
object SessionPacketChooseAggregate { def read(value: Any): SessionPacketChooseAggregate = new SessionPacketChooseAggregate(value) }
final class SessionPacketChooseRow(rawValue: Any) extends View(rawValue) with SessionPacket {
def `function`: String = { val value = required("function"); require(value != null, "Null native member: " + "function"); value.asInstanceOf[String] }
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `value`: AtomicNullableString = { val value = required("value"); require(value != null, "Null native member: " + "value"); AtomicNullableString.read(value) }
}
object SessionPacketChooseRow { def read(value: Any): SessionPacketChooseRow = new SessionPacketChooseRow(value) }
final class SessionPacketDecideAggregate(rawValue: Any) extends View(rawValue) with SessionPacket {
def `function`: String = { val value = required("function"); require(value != null, "Null native member: " + "function"); value.asInstanceOf[String] }
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `value`: List[AtomicDecideValue] = { val value = required("value"); require(value != null, "Null native member: " + "value"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => AtomicDecideValue.read(item0)) }
}
object SessionPacketDecideAggregate { def read(value: Any): SessionPacketDecideAggregate = new SessionPacketDecideAggregate(value) }
final class SessionPacketDecideRow(rawValue: Any) extends View(rawValue) with SessionPacket {
def `function`: String = { val value = required("function"); require(value != null, "Null native member: " + "function"); value.asInstanceOf[String] }
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `value`: AtomicDecideValue = { val value = required("value"); require(value != null, "Null native member: " + "value"); AtomicDecideValue.read(value) }
}
object SessionPacketDecideRow { def read(value: Any): SessionPacketDecideRow = new SessionPacketDecideRow(value) }
final class SessionPacketFilterAggregate(rawValue: Any) extends View(rawValue) with SessionPacket {
def `function`: String = { val value = required("function"); require(value != null, "Null native member: " + "function"); value.asInstanceOf[String] }
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `value`: List[AtomicBoolean] = { val value = required("value"); require(value != null, "Null native member: " + "value"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => AtomicBoolean.read(item0)) }
}
object SessionPacketFilterAggregate { def read(value: Any): SessionPacketFilterAggregate = new SessionPacketFilterAggregate(value) }
final class SessionPacketFilterRow(rawValue: Any) extends View(rawValue) with SessionPacket {
def `function`: String = { val value = required("function"); require(value != null, "Null native member: " + "function"); value.asInstanceOf[String] }
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `value`: AtomicBoolean = { val value = required("value"); require(value != null, "Null native member: " + "value"); AtomicBoolean.read(value) }
}
object SessionPacketFilterRow { def read(value: Any): SessionPacketFilterRow = new SessionPacketFilterRow(value) }
final class SessionPacketFindAggregate(rawValue: Any) extends View(rawValue) with SessionPacket {
def `function`: String = { val value = required("function"); require(value != null, "Null native member: " + "function"); value.asInstanceOf[String] }
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `value`: Find = { val value = required("value"); require(value != null, "Null native member: " + "value"); Find.read(value) }
}
object SessionPacketFindAggregate { def read(value: Any): SessionPacketFindAggregate = new SessionPacketFindAggregate(value) }
final class SessionPacketObservation(rawValue: Any) extends View(rawValue) with SessionPacket {
def `function`: RequestFunction = { val value = required("function"); require(value != null, "Null native member: " + "function"); RequestFunction.read(value) }
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `value`: SessionObservation = { val value = required("value"); require(value != null, "Null native member: " + "value"); SessionObservation.read(value) }
}
object SessionPacketObservation { def read(value: Any): SessionPacketObservation = new SessionPacketObservation(value) }
final class SessionPacketRankAggregate(rawValue: Any) extends View(rawValue) with SessionPacket {
def `function`: String = { val value = required("function"); require(value != null, "Null native member: " + "function"); value.asInstanceOf[String] }
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `value`: List[AtomicNonZeroUsize] = { val value = required("value"); require(value != null, "Null native member: " + "value"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => AtomicNonZeroUsize.read(item0)) }
}
object SessionPacketRankAggregate { def read(value: Any): SessionPacketRankAggregate = new SessionPacketRankAggregate(value) }
final class SessionPacketRecognizeAggregate(rawValue: Any) extends View(rawValue) with SessionPacket {
def `function`: String = { val value = required("function"); require(value != null, "Null native member: " + "function"); value.asInstanceOf[String] }
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `value`: List[Recognition] = { val value = required("value"); require(value != null, "Null native member: " + "value"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => Recognition.read(item0)) }
}
object SessionPacketRecognizeAggregate { def read(value: Any): SessionPacketRecognizeAggregate = new SessionPacketRecognizeAggregate(value) }
final class SessionPacketRelateAggregate(rawValue: Any) extends View(rawValue) with SessionPacket {
def `function`: String = { val value = required("function"); require(value != null, "Null native member: " + "function"); value.asInstanceOf[String] }
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `value`: Relation = { val value = required("value"); require(value != null, "Null native member: " + "value"); Relation.read(value) }
}
object SessionPacketRelateAggregate { def read(value: Any): SessionPacketRelateAggregate = new SessionPacketRelateAggregate(value) }
final class SessionPacketScoreAggregate(rawValue: Any) extends View(rawValue) with SessionPacket {
def `function`: String = { val value = required("function"); require(value != null, "Null native member: " + "function"); value.asInstanceOf[String] }
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `value`: List[AtomicDouble] = { val value = required("value"); require(value != null, "Null native member: " + "value"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => AtomicDouble.read(item0)) }
}
object SessionPacketScoreAggregate { def read(value: Any): SessionPacketScoreAggregate = new SessionPacketScoreAggregate(value) }
final class SessionPacketScoreRow(rawValue: Any) extends View(rawValue) with SessionPacket {
def `function`: String = { val value = required("function"); require(value != null, "Null native member: " + "function"); value.asInstanceOf[String] }
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `value`: AtomicDouble = { val value = required("value"); require(value != null, "Null native member: " + "value"); AtomicDouble.read(value) }
}
object SessionPacketScoreRow { def read(value: Any): SessionPacketScoreRow = new SessionPacketScoreRow(value) }
final class SessionPacketTagAggregate(rawValue: Any) extends View(rawValue) with SessionPacket {
def `function`: String = { val value = required("function"); require(value != null, "Null native member: " + "function"); value.asInstanceOf[String] }
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `value`: List[AtomicArrayOfString] = { val value = required("value"); require(value != null, "Null native member: " + "value"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => AtomicArrayOfString.read(item0)) }
}
object SessionPacketTagAggregate { def read(value: Any): SessionPacketTagAggregate = new SessionPacketTagAggregate(value) }
final class SessionPacketTagRow(rawValue: Any) extends View(rawValue) with SessionPacket {
def `function`: String = { val value = required("function"); require(value != null, "Null native member: " + "function"); value.asInstanceOf[String] }
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `value`: AtomicArrayOfString = { val value = required("value"); require(value != null, "Null native member: " + "value"); AtomicArrayOfString.read(value) }
}
object SessionPacketTagRow { def read(value: Any): SessionPacketTagRow = new SessionPacketTagRow(value) }
final class SessionPacketTerminal(rawValue: Any) extends View(rawValue) with SessionPacket {
def `facts`: Option[Facts] = optional("facts").flatMap(value => Option(value).map(value => Facts.read(value)))
def `failure`: Option[CallError] = optional("failure").flatMap(value => Option(value).map(value => CallError.read(value)))
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
}
object SessionPacketTerminal { def read(value: Any): SessionPacketTerminal = new SessionPacketTerminal(value) }
sealed trait SessionProbabilities { def json: Any }
object SessionProbabilities {
def read(value: Any): SessionProbabilities = {
val objectValue = Values.`object`(value)
if (objectValue.get("kind") == "yes_no") return SessionProbabilitiesYesNo.read(value)
if (objectValue.get("kind") == "named") return SessionProbabilitiesNamed.read(value)
throw new IllegalStateException("Unknown native result alternative")
}
}
final class SessionProbabilitiesNamed(rawValue: Any) extends View(rawValue) with SessionProbabilities {
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `value`: List[SessionNamedProbability] = { val value = required("value"); require(value != null, "Null native member: " + "value"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => SessionNamedProbability.read(item0)) }
}
object SessionProbabilitiesNamed { def read(value: Any): SessionProbabilitiesNamed = new SessionProbabilitiesNamed(value) }
final class SessionProbabilitiesYesNo(rawValue: Any) extends View(rawValue) with SessionProbabilities {
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `value`: BigDecimal = { val value = required("value"); require(value != null, "Null native member: " + "value"); BigDecimal(value.asInstanceOf[java.math.BigDecimal]) }
}
object SessionProbabilitiesYesNo { def read(value: Any): SessionProbabilitiesYesNo = new SessionProbabilitiesYesNo(value) }
final class SessionQuestionDetail(rawValue: Any) extends View(rawValue) {
def `answerId`: Option[AnswerId] = optional("answer_id").flatMap(value => Option(value).map(value => AnswerId.read(value)))
def `cached`: Boolean = { val value = required("cached"); require(value != null, "Null native member: " + "cached"); value.asInstanceOf[Boolean] }
def `confidence`: Option[BigDecimal] = optional("confidence").flatMap(value => Option(value).map(value => BigDecimal(value.asInstanceOf[java.math.BigDecimal])))
def `failedQuestions`: BigInt = { val value = required("failed_questions"); require(value != null, "Null native member: " + "failed_questions"); BigInt(Values.integer(value)) }
def `failure`: Option[Failure] = optional("failure").flatMap(value => Option(value).map(value => Failure.read(value)))
def `failureId`: Option[FailureId] = optional("failure_id").flatMap(value => Option(value).map(value => FailureId.read(value)))
def `input`: Option[Option[Any]] = optional("input").map(value => Option(value).map(value => Values.freeze(value)))
def `inputSource`: Option[PhysicalSource] = optional("input_source").flatMap(value => Option(value).map(value => PhysicalSource.read(value)))
def `inputSources`: List[SessionInputSource] = { val value = required("input_sources"); require(value != null, "Null native member: " + "input_sources"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => SessionInputSource.read(item0)) }
def `inputs`: List[Any] = { val value = required("inputs"); require(value != null, "Null native member: " + "inputs"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => Values.freeze(item0)) }
def `model`: String = { val value = required("model"); require(value != null, "Null native member: " + "model"); value.asInstanceOf[String] }
def `observations`: List[Observation] = { val value = required("observations"); require(value != null, "Null native member: " + "observations"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => Observation.read(item0)) }
def `probabilities`: Option[SessionProbabilities] = optional("probabilities").flatMap(value => Option(value).map(value => SessionProbabilities.read(value)))
def `question`: ReadableQuestion = { val value = required("question"); require(value != null, "Null native member: " + "question"); ReadableQuestion.read(value) }
def `questionSha256`: String = { val value = required("question_sha256"); require(value != null, "Null native member: " + "question_sha256"); value.asInstanceOf[String] }
def `questionSources`: List[QuestionSource] = { val value = required("question_sources"); require(value != null, "Null native member: " + "question_sources"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => QuestionSource.read(item0)) }
def `rawPick`: Option[String] = optional("raw_pick").flatMap(value => Option(value).map(value => value.asInstanceOf[String]))
def `reportedUsage`: Option[Usage] = optional("reported_usage").flatMap(value => Option(value).map(value => Usage.read(value)))
def `requests`: List[String] = { val value = required("requests"); require(value != null, "Null native member: " + "requests"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => item0.asInstanceOf[String]) }
def `requestsSent`: BigInt = { val value = required("requests_sent"); require(value != null, "Null native member: " + "requests_sent"); BigInt(Values.integer(value)) }
def `threshold`: Option[Threshold] = optional("threshold").flatMap(value => Option(value).map(value => Threshold.read(value)))
def `url`: String = { val value = required("url"); require(value != null, "Null native member: " + "url"); value.asInstanceOf[String] }
def `usage`: Option[TokenUsage] = optional("usage").flatMap(value => Option(value).map(value => TokenUsage.read(value)))
def `value`: Option[Value] = optional("value").flatMap(value => Option(value).map(value => Value.read(value)))
}
object SessionQuestionDetail { def read(value: Any): SessionQuestionDetail = new SessionQuestionDetail(value) }
final class SessionRecognition(rawValue: Any) extends View(rawValue) {
def `entities`: List[Entity] = { val value = required("entities"); require(value != null, "Null native member: " + "entities"); value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => Entity.read(item0)) }
def `mode`: RecognitionMode = { val value = required("mode"); require(value != null, "Null native member: " + "mode"); RecognitionMode.read(value) }
def `proposals`: Option[List[BoundaryProposal]] = optional("proposals").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => BoundaryProposal.read(item0))))
def `relations`: Option[List[RecognitionEdgeDocument]] = optional("relations").flatMap(value => Option(value).map(value => value.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => RecognitionEdgeDocument.read(item0))))
}
object SessionRecognition { def read(value: Any): SessionRecognition = new SessionRecognition(value) }
final class SessionRelationEdge(rawValue: Any) extends View(rawValue) {
def `either`: Boolean = { val value = required("either"); require(value != null, "Null native member: " + "either"); value.asInstanceOf[Boolean] }
def `probability`: BigDecimal = { val value = required("probability"); require(value != null, "Null native member: " + "probability"); BigDecimal(value.asInstanceOf[java.math.BigDecimal]) }
def `relation`: String = { val value = required("relation"); require(value != null, "Null native member: " + "relation"); value.asInstanceOf[String] }
def `source`: EntityDocument = { val value = required("source"); require(value != null, "Null native member: " + "source"); EntityDocument.read(value) }
def `target`: EntityDocument = { val value = required("target"); require(value != null, "Null native member: " + "target"); EntityDocument.read(value) }
}
object SessionRelationEdge { def read(value: Any): SessionRelationEdge = new SessionRelationEdge(value) }
final class SourceRelationEndpoint(rawValue: Any) extends View(rawValue) {
def `file`: Option[String] = Option(required("file")).map(value => value.asInstanceOf[String])
def `firstLine`: Option[BigInt] = optional("first_line").flatMap(value => Option(value).map(value => BigInt(Values.integer(value))))
def `kind`: String = { val value = required("kind"); require(value != null, "Null native member: " + "kind"); value.asInstanceOf[String] }
def `lastLine`: Option[BigInt] = optional("last_line").flatMap(value => Option(value).map(value => BigInt(Values.integer(value))))
def `name`: String = { val value = required("name"); require(value != null, "Null native member: " + "name"); value.asInstanceOf[String] }
def `ordinal`: BigInt = { val value = required("ordinal"); require(value != null, "Null native member: " + "ordinal"); BigInt(Values.integer(value)) }
def `record`: Any = { val value = required("record"); require(value != null, "Null native member: " + "record"); Values.freeze(value) }
}
object SourceRelationEndpoint { def read(value: Any): SourceRelationEndpoint = new SourceRelationEndpoint(value) }
sealed trait Threshold { def json: Any }
object Threshold {
def read(value: Any): Threshold = {
if (value.isInstanceOf[java.math.BigDecimal]) return ThresholdNumber.read(value)
if (value.isInstanceOf[String]) return ThresholdString.read(value)
throw new IllegalStateException("Unknown native result alternative")
}
}
final class ThresholdNumber(val json: Any) extends Threshold { def value: BigDecimal = BigDecimal(json.asInstanceOf[java.math.BigDecimal]) }
object ThresholdNumber { def read(value: Any): ThresholdNumber = new ThresholdNumber(Values.freeze(value)) }
final class ThresholdString(val json: Any) extends Threshold { def value: String = json.asInstanceOf[String] }
object ThresholdString { def read(value: Any): ThresholdString = new ThresholdString(Values.freeze(value)) }
final class TokenBand(rawValue: Any) extends View(rawValue) {
def `lower`: BigInt = { val value = required("lower"); require(value != null, "Null native member: " + "lower"); BigInt(Values.integer(value)) }
def `upper`: BigInt = { val value = required("upper"); require(value != null, "Null native member: " + "upper"); BigInt(Values.integer(value)) }
}
object TokenBand { def read(value: Any): TokenBand = new TokenBand(value) }
final class TokenUsage(rawValue: Any) extends View(rawValue) {
def `inputTokens`: BigInt = { val value = required("input_tokens"); require(value != null, "Null native member: " + "input_tokens"); BigInt(Values.integer(value)) }
def `outputTokens`: BigInt = { val value = required("output_tokens"); require(value != null, "Null native member: " + "output_tokens"); BigInt(Values.integer(value)) }
}
object TokenUsage { def read(value: Any): TokenUsage = new TokenUsage(value) }
sealed trait Value { def json: Any }
object Value {
def read(value: Any): Value = {
if (value.isInstanceOf[Boolean]) return ValueBoolean.read(value)
if (value == null) return ValueNull.read(value)
if (value.isInstanceOf[String]) return ValueString.read(value)
if (value.isInstanceOf[java.util.List[?]]) return ValueArray.read(value)
if (value.isInstanceOf[java.math.BigDecimal]) return ValueNumber.read(value)
throw new IllegalStateException("Unknown native result alternative")
}
}
final class ValueBoolean(val json: Any) extends Value { def value: Boolean = json.asInstanceOf[Boolean] }
object ValueBoolean { def read(value: Any): ValueBoolean = new ValueBoolean(Values.freeze(value)) }
final class ValueNull(val json: Any) extends Value { def value: Any = Values.freeze(json) }
object ValueNull { def read(value: Any): ValueNull = new ValueNull(Values.freeze(value)) }
final class ValueString(val json: Any) extends Value { def value: String = json.asInstanceOf[String] }
object ValueString { def read(value: Any): ValueString = new ValueString(Values.freeze(value)) }
final class ValueArray(val json: Any) extends Value { def value: List[String] = json.asInstanceOf[java.util.List[Any]].asScala.toList.map(item0 => item0.asInstanceOf[String]) }
object ValueArray { def read(value: Any): ValueArray = new ValueArray(Values.freeze(value)) }
final class ValueNumber(val json: Any) extends Value { def value: BigDecimal = BigDecimal(json.asInstanceOf[java.math.BigDecimal]) }
object ValueNumber { def read(value: Any): ValueNumber = new ValueNumber(Values.freeze(value)) }
}
