// Generated from the Rust result graph. Do not edit.
package thinkthen;
import java.util.*;
import java.math.*;
public final class Results {
private Results() {}
public static final class Annotation extends Values.View {
public Annotation(Object value) { super(value); }
static Annotation read(Object value) { return new Annotation(value); }
public AnswerId answerId() { Object value = required("answer_id"); return AnswerId.read(value); }
public Map<String,AnnotationMember> answers() { Object value = required("answers"); return Values.map(value, item0 -> AnnotationMember.read(item0)); }
public Presence<String> file() { return presence("file", value -> (String)value); }
public Presence<BigInteger> firstLine() { return presence("first_line", value -> Values.integer(value)); }
public Presence<BigInteger> index() { return presence("index", value -> Values.integer(value)); }
public Presence<Object> input() { required("input"); return presence("input", value -> Values.freeze(value)); }
public Presence<BigInteger> lastLine() { return presence("last_line", value -> Values.integer(value)); }
public Meta meta() { Object value = required("meta"); return Meta.read(value); }
public Presence<Position> position() { return presence("position", value -> Position.read(value)); }
public Version schema() { Object value = required("schema"); return Version.read(value); }
public Presence<PhysicalSource> source() { return presence("source", value -> PhysicalSource.read(value)); }
public AnnotatedRow value() { Object value = required("value"); return AnnotatedRow.read(value); }
}
public sealed interface AnnotationMember extends Values.Value permits AnnotationMemberAnswerId,AnnotationMemberFailureId {
static AnnotationMember read(Object value) { Map<String,Object> object = Values.object(value);
if (object.containsKey("answer_id")) return AnnotationMemberAnswerId.read(value);
if (object.containsKey("failure_id")) return AnnotationMemberFailureId.read(value);
throw new IllegalStateException("Unknown native result alternative");
}
}
public static final class AnnotationMemberAnswerId extends Values.View implements AnnotationMember {
public AnnotationMemberAnswerId(Object value) { super(value); }
static AnnotationMemberAnswerId read(Object value) { return new AnnotationMemberAnswerId(value); }
public Answer answer() { Object value = required("answer"); return Answer.read(value); }
public AnswerId answerId() { Object value = required("answer_id"); return AnswerId.read(value); }
public List<Observation> observations() { Object value = required("observations"); return Values.list(value, item0 -> Observation.read(item0)); }
public ReadableQuestion question() { Object value = required("question"); return ReadableQuestion.read(value); }
public List<QuestionSource> questionSources() { Object value = required("question_sources"); return Values.list(value, item0 -> QuestionSource.read(item0)); }
public String request() { Object value = required("request"); return (String)value; }
public Presence<Threshold> threshold() { required("threshold"); return presence("threshold", value -> Threshold.read(value)); }
public Presence<Usage> usage() { return presence("usage", value -> Usage.read(value)); }
public Presence<Value> value() { required("value"); return presence("value", value -> Value.read(value)); }
}
public static final class AnnotationMemberFailureId extends Values.View implements AnnotationMember {
public AnnotationMemberFailureId(Object value) { super(value); }
static AnnotationMemberFailureId read(Object value) { return new AnnotationMemberFailureId(value); }
public Failure failure() { Object value = required("failure"); return Failure.read(value); }
public FailureId failureId() { Object value = required("failure_id"); return FailureId.read(value); }
public List<Observation> observations() { Object value = required("observations"); return Values.list(value, item0 -> Observation.read(item0)); }
public ReadableQuestion question() { Object value = required("question"); return ReadableQuestion.read(value); }
public List<QuestionSource> questionSources() { Object value = required("question_sources"); return Values.list(value, item0 -> QuestionSource.read(item0)); }
public String request() { Object value = required("request"); return (String)value; }
public Presence<Threshold> threshold() { required("threshold"); return presence("threshold", value -> Threshold.read(value)); }
public Presence<Usage> usage() { return presence("usage", value -> Usage.read(value)); }
}
public sealed interface AnnotationValue extends Values.Value permits AnnotationValueDecision,AnnotationValueChoice,AnnotationValueScore,AnnotationValueTags,AnnotationValueFailed {
static AnnotationValue read(Object value) { Map<String,Object> object = Values.object(value);
if ("decision".equals(object.get("kind"))) return AnnotationValueDecision.read(value);
if ("choice".equals(object.get("kind"))) return AnnotationValueChoice.read(value);
if ("score".equals(object.get("kind"))) return AnnotationValueScore.read(value);
if ("tags".equals(object.get("kind"))) return AnnotationValueTags.read(value);
if ("failed".equals(object.get("kind"))) return AnnotationValueFailed.read(value);
throw new IllegalStateException("Unknown native result alternative");
}
}
public static final class AnnotationValueChoice extends Values.View implements AnnotationValue {
public AnnotationValueChoice(Object value) { super(value); }
static AnnotationValueChoice read(Object value) { return new AnnotationValueChoice(value); }
public String kind() { Object value = required("kind"); return (String)value; }
public Presence<String> value() { required("value"); return presence("value", value -> (String)value); }
}
public static final class AnnotationValueDecision extends Values.View implements AnnotationValue {
public AnnotationValueDecision(Object value) { super(value); }
static AnnotationValueDecision read(Object value) { return new AnnotationValueDecision(value); }
public String kind() { Object value = required("kind"); return (String)value; }
public Presence<Boolean> value() { required("value"); return presence("value", value -> (Boolean)value); }
}
public static final class AnnotationValueFailed extends Values.View implements AnnotationValue {
public AnnotationValueFailed(Object value) { super(value); }
static AnnotationValueFailed read(Object value) { return new AnnotationValueFailed(value); }
public String kind() { Object value = required("kind"); return (String)value; }
public Failure value() { Object value = required("value"); return Failure.read(value); }
}
public static final class AnnotationValueScore extends Values.View implements AnnotationValue {
public AnnotationValueScore(Object value) { super(value); }
static AnnotationValueScore read(Object value) { return new AnnotationValueScore(value); }
public String kind() { Object value = required("kind"); return (String)value; }
public BigDecimal value() { Object value = required("value"); return (BigDecimal)value; }
}
public static final class AnnotationValueTags extends Values.View implements AnnotationValue {
public AnnotationValueTags(Object value) { super(value); }
static AnnotationValueTags read(Object value) { return new AnnotationValueTags(value); }
public String kind() { Object value = required("kind"); return (String)value; }
public List<String> value() { Object value = required("value"); return Values.list(value, item0 -> (String)item0); }
}
public static final class AnswerId extends Values.View {
public AnswerId(Object value) { super(value); }
static AnswerId read(Object value) { return new AnswerId(value); }
public String value() { return (String)json(); }
}
public static final class Answers extends Values.View {
public Answers(Object value) { super(value); }
static Answers read(Object value) { return new Answers(value); }
public List<RelationMember> questions() { Object value = required("questions"); return Values.list(value, item0 -> RelationMember.read(item0)); }
}
public static final class AtomicArrayOfString extends Values.View {
public AtomicArrayOfString(Object value) { super(value); }
static AtomicArrayOfString read(Object value) { return new AtomicArrayOfString(value); }
public Answer answer() { Object value = required("answer"); return Answer.read(value); }
public AnswerId answerId() { Object value = required("answer_id"); return AnswerId.read(value); }
public Presence<List<Image>> images() { return presence("images", value -> Values.list(value, item0 -> Image.read(item0))); }
public Presence<BigInteger> index() { return presence("index", value -> Values.integer(value)); }
public Presence<Object> input() { return presence("input", value -> Values.freeze(value)); }
public Presence<List<RankMember>> members() { return presence("members", value -> Values.list(value, item0 -> RankMember.read(item0))); }
public Meta meta() { Object value = required("meta"); return Meta.read(value); }
public ReadableQuestion question() { Object value = required("question"); return ReadableQuestion.read(value); }
public Presence<String> questionName() { return presence("question_name", value -> (String)value); }
public Version schema() { Object value = required("schema"); return Version.read(value); }
public Presence<PhysicalSource> source() { return presence("source", value -> PhysicalSource.read(value)); }
public Presence<Threshold> threshold() { required("threshold"); return presence("threshold", value -> Threshold.read(value)); }
public List<String> value() { Object value = required("value"); return Values.list(value, item0 -> (String)item0); }
}
public static final class AtomicDecideValue extends Values.View {
public AtomicDecideValue(Object value) { super(value); }
static AtomicDecideValue read(Object value) { return new AtomicDecideValue(value); }
public Answer answer() { Object value = required("answer"); return Answer.read(value); }
public AnswerId answerId() { Object value = required("answer_id"); return AnswerId.read(value); }
public Presence<List<Image>> images() { return presence("images", value -> Values.list(value, item0 -> Image.read(item0))); }
public Presence<BigInteger> index() { return presence("index", value -> Values.integer(value)); }
public Presence<Object> input() { return presence("input", value -> Values.freeze(value)); }
public Presence<List<RankMember>> members() { return presence("members", value -> Values.list(value, item0 -> RankMember.read(item0))); }
public Meta meta() { Object value = required("meta"); return Meta.read(value); }
public ReadableQuestion question() { Object value = required("question"); return ReadableQuestion.read(value); }
public Presence<String> questionName() { return presence("question_name", value -> (String)value); }
public Version schema() { Object value = required("schema"); return Version.read(value); }
public Presence<PhysicalSource> source() { return presence("source", value -> PhysicalSource.read(value)); }
public Presence<Threshold> threshold() { required("threshold"); return presence("threshold", value -> Threshold.read(value)); }
public DecideValue value() { Object value = required("value"); return DecideValue.read(value); }
}
public static final class AtomicNonZeroUsize extends Values.View {
public AtomicNonZeroUsize(Object value) { super(value); }
static AtomicNonZeroUsize read(Object value) { return new AtomicNonZeroUsize(value); }
public Answer answer() { Object value = required("answer"); return Answer.read(value); }
public AnswerId answerId() { Object value = required("answer_id"); return AnswerId.read(value); }
public Presence<List<Image>> images() { return presence("images", value -> Values.list(value, item0 -> Image.read(item0))); }
public Presence<BigInteger> index() { return presence("index", value -> Values.integer(value)); }
public Presence<Object> input() { return presence("input", value -> Values.freeze(value)); }
public Presence<List<RankMember>> members() { return presence("members", value -> Values.list(value, item0 -> RankMember.read(item0))); }
public Meta meta() { Object value = required("meta"); return Meta.read(value); }
public ReadableQuestion question() { Object value = required("question"); return ReadableQuestion.read(value); }
public Presence<String> questionName() { return presence("question_name", value -> (String)value); }
public Version schema() { Object value = required("schema"); return Version.read(value); }
public Presence<PhysicalSource> source() { return presence("source", value -> PhysicalSource.read(value)); }
public Presence<Threshold> threshold() { required("threshold"); return presence("threshold", value -> Threshold.read(value)); }
public BigInteger value() { Object value = required("value"); return Values.integer(value); }
}
public static final class AtomicNullableString extends Values.View {
public AtomicNullableString(Object value) { super(value); }
static AtomicNullableString read(Object value) { return new AtomicNullableString(value); }
public Answer answer() { Object value = required("answer"); return Answer.read(value); }
public AnswerId answerId() { Object value = required("answer_id"); return AnswerId.read(value); }
public Presence<List<Image>> images() { return presence("images", value -> Values.list(value, item0 -> Image.read(item0))); }
public Presence<BigInteger> index() { return presence("index", value -> Values.integer(value)); }
public Presence<Object> input() { return presence("input", value -> Values.freeze(value)); }
public Presence<List<RankMember>> members() { return presence("members", value -> Values.list(value, item0 -> RankMember.read(item0))); }
public Meta meta() { Object value = required("meta"); return Meta.read(value); }
public ReadableQuestion question() { Object value = required("question"); return ReadableQuestion.read(value); }
public Presence<String> questionName() { return presence("question_name", value -> (String)value); }
public Version schema() { Object value = required("schema"); return Version.read(value); }
public Presence<PhysicalSource> source() { return presence("source", value -> PhysicalSource.read(value)); }
public Presence<Threshold> threshold() { required("threshold"); return presence("threshold", value -> Threshold.read(value)); }
public Presence<String> value() { required("value"); return presence("value", value -> (String)value); }
}
public static final class AtomicBoolean extends Values.View {
public AtomicBoolean(Object value) { super(value); }
static AtomicBoolean read(Object value) { return new AtomicBoolean(value); }
public Answer answer() { Object value = required("answer"); return Answer.read(value); }
public AnswerId answerId() { Object value = required("answer_id"); return AnswerId.read(value); }
public Presence<List<Image>> images() { return presence("images", value -> Values.list(value, item0 -> Image.read(item0))); }
public Presence<BigInteger> index() { return presence("index", value -> Values.integer(value)); }
public Presence<Object> input() { return presence("input", value -> Values.freeze(value)); }
public Presence<List<RankMember>> members() { return presence("members", value -> Values.list(value, item0 -> RankMember.read(item0))); }
public Meta meta() { Object value = required("meta"); return Meta.read(value); }
public ReadableQuestion question() { Object value = required("question"); return ReadableQuestion.read(value); }
public Presence<String> questionName() { return presence("question_name", value -> (String)value); }
public Version schema() { Object value = required("schema"); return Version.read(value); }
public Presence<PhysicalSource> source() { return presence("source", value -> PhysicalSource.read(value)); }
public Presence<Threshold> threshold() { required("threshold"); return presence("threshold", value -> Threshold.read(value)); }
public Boolean value() { Object value = required("value"); return (Boolean)value; }
}
public static final class AtomicDouble extends Values.View {
public AtomicDouble(Object value) { super(value); }
static AtomicDouble read(Object value) { return new AtomicDouble(value); }
public Answer answer() { Object value = required("answer"); return Answer.read(value); }
public AnswerId answerId() { Object value = required("answer_id"); return AnswerId.read(value); }
public Presence<List<Image>> images() { return presence("images", value -> Values.list(value, item0 -> Image.read(item0))); }
public Presence<BigInteger> index() { return presence("index", value -> Values.integer(value)); }
public Presence<Object> input() { return presence("input", value -> Values.freeze(value)); }
public Presence<List<RankMember>> members() { return presence("members", value -> Values.list(value, item0 -> RankMember.read(item0))); }
public Meta meta() { Object value = required("meta"); return Meta.read(value); }
public ReadableQuestion question() { Object value = required("question"); return ReadableQuestion.read(value); }
public Presence<String> questionName() { return presence("question_name", value -> (String)value); }
public Version schema() { Object value = required("schema"); return Version.read(value); }
public Presence<PhysicalSource> source() { return presence("source", value -> PhysicalSource.read(value)); }
public Presence<Threshold> threshold() { required("threshold"); return presence("threshold", value -> Threshold.read(value)); }
public BigDecimal value() { Object value = required("value"); return (BigDecimal)value; }
}
public static final class Attempt extends Values.View {
public Attempt(Object value) { super(value); }
static Attempt read(Object value) { return new Attempt(value); }
public BigInteger ordinal() { Object value = required("ordinal"); return Values.integer(value); }
public AttemptOutcome outcome() { Object value = required("outcome"); return AttemptOutcome.read(value); }
public Presence<String> requestId() { return presence("request_id", value -> (String)value); }
public String requestSha256() { Object value = required("request_sha256"); return (String)value; }
public SdkRequestId sdkRequestId() { Object value = required("sdk_request_id"); return SdkRequestId.read(value); }
public Presence<BigInteger> serverMs() { return presence("server_ms", value -> Values.integer(value)); }
public Presence<BigInteger> status() { return presence("status", value -> Values.integer(value)); }
public BigInteger wallMs() { Object value = required("wall_ms"); return Values.integer(value); }
}
public sealed interface Batch extends Values.Value permits BatchInteger,BatchString {
static Batch read(Object value) {
if (value instanceof BigDecimal) return BatchInteger.read(value);
if (value instanceof String) return BatchString.read(value);
throw new IllegalStateException("Unknown native value alternative");
}
}
public record BatchInteger(BigInteger value) implements Batch {
static BatchInteger read(Object value) { return new BatchInteger(Values.integer(value)); }
public Object json() { return Values.json(value); }
}
public record BatchString(String value) implements Batch {
static BatchString read(Object value) { return new BatchString((String)value); }
public Object json() { return Values.json(value); }
}
public static final class BoundaryMode extends Values.View {
public BoundaryMode(Object value) { super(value); }
static BoundaryMode read(Object value) { return new BoundaryMode(value); }
public String value() { return (String)json(); }
}
public static final class BoundaryOdds extends Values.View {
public BoundaryOdds(Object value) { super(value); }
static BoundaryOdds read(Object value) { return new BoundaryOdds(value); }
public List<PieceOdds> pieces() { Object value = required("pieces"); return Values.list(value, item0 -> PieceOdds.read(item0)); }
public List<BoundaryProposal> proposals() { Object value = required("proposals"); return Values.list(value, item0 -> BoundaryProposal.read(item0)); }
}
public static final class BoundaryProposal extends Values.View {
public BoundaryProposal(Object value) { super(value); }
static BoundaryProposal read(Object value) { return new BoundaryProposal(value); }
public BigInteger end() { Object value = required("end"); return Values.integer(value); }
public BigInteger length() { Object value = required("length"); return Values.integer(value); }
public BigDecimal probability() { Object value = required("probability"); return (BigDecimal)value; }
public BigInteger start() { Object value = required("start"); return Values.integer(value); }
public String text() { Object value = required("text"); return (String)value; }
}
public static final class CallError extends Values.View {
public CallError(Object value) { super(value); }
static CallError read(Object value) { return new CallError(value); }
public Error error() { Object value = required("error"); return Error.read(value); }
public Presence<Facts> facts() { return presence("facts", value -> Facts.read(value)); }
}
public static final class CallId extends Values.View {
public CallId(Object value) { super(value); }
static CallId read(Object value) { return new CallId(value); }
public String value() { return (String)json(); }
}
public static final class DecideValue extends Values.View {
public DecideValue(Object value) { super(value); }
static DecideValue read(Object value) { return new DecideValue(value); }
public Object value() { return Values.freeze(json()); }
}
public static final class EntityDocument extends Values.View {
public EntityDocument(Object value) { super(value); }
static EntityDocument read(Object value) { return new EntityDocument(value); }
public String kind() { Object value = required("kind"); return (String)value; }
public String name() { Object value = required("name"); return (String)value; }
}
public static final class Error extends Values.View {
public Error(Object value) { super(value); }
static Error read(Object value) { return new Error(value); }
public Presence<EstimatedInputDenial> estimatedInputDenial() { return presence("estimated_input_denial", value -> EstimatedInputDenial.read(value)); }
public FailureKind kind() { Object value = required("kind"); return FailureKind.read(value); }
public String message() { Object value = required("message"); return (String)value; }
public Boolean retryable() { Object value = required("retryable"); return (Boolean)value; }
public Presence<SendBudgetDenial> sendBudgetDenial() { return presence("send_budget_denial", value -> SendBudgetDenial.read(value)); }
public Stopped stopped() { Object value = required("stopped"); return Stopped.read(value); }
}
public sealed interface EstimatedInputDenial extends Values.Value permits EstimatedInputDenialInitialRequest,EstimatedInputDenialAdditionalRequest,EstimatedInputDenialRetry {
static EstimatedInputDenial read(Object value) { Map<String,Object> object = Values.object(value);
if ("initial_request".equals(object.get("kind"))) return EstimatedInputDenialInitialRequest.read(value);
if ("additional_request".equals(object.get("kind"))) return EstimatedInputDenialAdditionalRequest.read(value);
if ("retry".equals(object.get("kind"))) return EstimatedInputDenialRetry.read(value);
throw new IllegalStateException("Unknown native result alternative");
}
}
public static final class EstimatedInputDenialAdditionalRequest extends Values.View implements EstimatedInputDenial {
public EstimatedInputDenialAdditionalRequest(Object value) { super(value); }
static EstimatedInputDenialAdditionalRequest read(Object value) { return new EstimatedInputDenialAdditionalRequest(value); }
public String kind() { Object value = required("kind"); return (String)value; }
public BigInteger limit() { Object value = required("limit"); return Values.integer(value); }
}
public static final class EstimatedInputDenialInitialRequest extends Values.View implements EstimatedInputDenial {
public EstimatedInputDenialInitialRequest(Object value) { super(value); }
static EstimatedInputDenialInitialRequest read(Object value) { return new EstimatedInputDenialInitialRequest(value); }
public String kind() { Object value = required("kind"); return (String)value; }
public BigInteger limit() { Object value = required("limit"); return Values.integer(value); }
}
public static final class EstimatedInputDenialRetry extends Values.View implements EstimatedInputDenial {
public EstimatedInputDenialRetry(Object value) { super(value); }
static EstimatedInputDenialRetry read(Object value) { return new EstimatedInputDenialRetry(value); }
public String kind() { Object value = required("kind"); return (String)value; }
public BigInteger lastStatus() { Object value = required("last_status"); return Values.integer(value); }
public BigInteger limit() { Object value = required("limit"); return Values.integer(value); }
}
public static final class Facts extends Values.View {
public Facts(Object value) { super(value); }
static Facts read(Object value) { return new Facts(value); }
public Presence<List<Attempt>> attempts() { return presence("attempts", value -> Values.list(value, item0 -> Attempt.read(item0))); }
public BigInteger cacheAnswers() { Object value = required("cache_answers"); return Values.integer(value); }
public CallId callId() { Object value = required("call_id"); return CallId.read(value); }
public Presence<String> estimatedCostUsd() { return presence("estimated_cost_usd", value -> (String)value); }
public Presence<Boolean> heldModelMismatch() { return presence("held_model_mismatch", value -> (Boolean)value); }
public Presence<BigInteger> inputTokens() { return presence("input_tokens", value -> Values.integer(value)); }
public BigInteger largestRequestBytes() { Object value = required("largest_request_bytes"); return Values.integer(value); }
public Presence<BigInteger> largestRequestEstimatedInputTokens() { required("largest_request_estimated_input_tokens"); return presence("largest_request_estimated_input_tokens", value -> Values.integer(value)); }
public Presence<String> model() { return presence("model", value -> (String)value); }
public Presence<BigInteger> outputTokens() { return presence("output_tokens", value -> Values.integer(value)); }
public BigInteger records() { Object value = required("records"); return Values.integer(value); }
public BigInteger requestsSent() { Object value = required("requests_sent"); return Values.integer(value); }
public BigDecimal seconds() { Object value = required("seconds"); return (BigDecimal)value; }
public String tokenEstimateMethod() { Object value = required("token_estimate_method"); return (String)value; }
public Presence<PersistenceObservation> usagePersistence() { return presence("usage_persistence", value -> PersistenceObservation.read(value)); }
}
public static final class FailureId extends Values.View {
public FailureId(Object value) { super(value); }
static FailureId read(Object value) { return new FailureId(value); }
public String value() { return (String)json(); }
}
public static final class Find extends Values.View {
public Find(Object value) { super(value); }
static Find read(Object value) { return new Find(value); }
public FindAnswer answer() { Object value = required("answer"); return FindAnswer.read(value); }
public AnswerId answerId() { Object value = required("answer_id"); return AnswerId.read(value); }
public Presence<List<FindCandidate>> candidates() { return presence("candidates", value -> Values.list(value, item0 -> FindCandidate.read(item0))); }
public Presence<String> file() { return presence("file", value -> (String)value); }
public Presence<BigInteger> firstLine() { return presence("first_line", value -> Values.integer(value)); }
public Presence<BigInteger> index() { required("index"); return presence("index", value -> Values.integer(value)); }
public Presence<BigInteger> lastLine() { return presence("last_line", value -> Values.integer(value)); }
public Meta meta() { Object value = required("meta"); return Meta.read(value); }
public Presence<Position> position() { return presence("position", value -> Position.read(value)); }
public ReadableQuestion2 question() { Object value = required("question"); return ReadableQuestion2.read(value); }
public Version schema() { Object value = required("schema"); return Version.read(value); }
public Presence<Object> threshold() { required("threshold"); return presence("threshold", value -> Values.freeze(value)); }
public Presence<Object> value() { required("value"); return presence("value", value -> Values.freeze(value)); }
}
public static final class FindCandidate extends Values.View {
public FindCandidate(Object value) { super(value); }
static FindCandidate read(Object value) { return new FindCandidate(value); }
public Presence<BigInteger> index() { required("index"); return presence("index", value -> Values.integer(value)); }
public Presence<Object> input() { required("input"); return presence("input", value -> Values.freeze(value)); }
public BigDecimal probability() { Object value = required("probability"); return (BigDecimal)value; }
public Presence<PhysicalSource> source() { return presence("source", value -> PhysicalSource.read(value)); }
}
public static final class Image extends Values.View {
public Image(Object value) { super(value); }
static Image read(Object value) { return new Image(value); }
public String base64() { Object value = required("base64"); return (String)value; }
public BigInteger height() { Object value = required("height"); return Values.integer(value); }
public ImageMedia media() { Object value = required("media"); return ImageMedia.read(value); }
public BigInteger width() { Object value = required("width"); return Values.integer(value); }
}
public static final class ImageMedia extends Values.View {
public ImageMedia(Object value) { super(value); }
static ImageMedia read(Object value) { return new ImageMedia(value); }
public String value() { return (String)json(); }
}
public sealed interface InputDeclaration extends Values.Value permits InputDeclarationString,InputDeclarationObject {
static InputDeclaration read(Object value) { Map<String,Object> object = Values.object(value);
if ("string".equals(object.get("type"))) return InputDeclarationString.read(value);
if ("object".equals(object.get("type"))) return InputDeclarationObject.read(value);
throw new IllegalStateException("Unknown native result alternative");
}
}
public static final class InputDeclarationObject extends Values.View implements InputDeclaration {
public InputDeclarationObject(Object value) { super(value); }
static InputDeclarationObject read(Object value) { return new InputDeclarationObject(value); }
public Map<String,InputPropertyType> properties() { Object value = required("properties"); return Values.map(value, item0 -> InputPropertyType.read(item0)); }
public Presence<List<String>> required() { return presence("required", value -> Values.list(value, item0 -> (String)item0)); }
public ObjectType type() { Object value = required("type"); return ObjectType.read(value); }
}
public static final class InputDeclarationString extends Values.View implements InputDeclaration {
public InputDeclarationString(Object value) { super(value); }
static InputDeclarationString read(Object value) { return new InputDeclarationString(value); }
public StringType type() { Object value = required("type"); return StringType.read(value); }
}
public sealed interface InputPropertyType extends Values.Value permits InputPropertyTypeString,InputPropertyTypeNumber,InputPropertyTypeBoolean,InputPropertyTypeArray {
static InputPropertyType read(Object value) { Map<String,Object> object = Values.object(value);
if ("string".equals(object.get("type"))) return InputPropertyTypeString.read(value);
if ("number".equals(object.get("type"))) return InputPropertyTypeNumber.read(value);
if ("boolean".equals(object.get("type"))) return InputPropertyTypeBoolean.read(value);
if ("array".equals(object.get("type"))) return InputPropertyTypeArray.read(value);
throw new IllegalStateException("Unknown native result alternative");
}
}
public static final class InputPropertyTypeArray extends Values.View implements InputPropertyType {
public InputPropertyTypeArray(Object value) { super(value); }
static InputPropertyTypeArray read(Object value) { return new InputPropertyTypeArray(value); }
public StringRoot items() { Object value = required("items"); return StringRoot.read(value); }
public String type() { Object value = required("type"); return (String)value; }
}
public static final class InputPropertyTypeBoolean extends Values.View implements InputPropertyType {
public InputPropertyTypeBoolean(Object value) { super(value); }
static InputPropertyTypeBoolean read(Object value) { return new InputPropertyTypeBoolean(value); }
public String type() { Object value = required("type"); return (String)value; }
}
public static final class InputPropertyTypeNumber extends Values.View implements InputPropertyType {
public InputPropertyTypeNumber(Object value) { super(value); }
static InputPropertyTypeNumber read(Object value) { return new InputPropertyTypeNumber(value); }
public String type() { Object value = required("type"); return (String)value; }
}
public static final class InputPropertyTypeString extends Values.View implements InputPropertyType {
public InputPropertyTypeString(Object value) { super(value); }
static InputPropertyTypeString read(Object value) { return new InputPropertyTypeString(value); }
public String type() { Object value = required("type"); return (String)value; }
}
public static final class Label extends Values.View {
public Label(Object value) { super(value); }
static Label read(Object value) { return new Label(value); }
public Presence<Object> description() { return presence("description", value -> Values.freeze(value)); }
public String name() { Object value = required("name"); return (String)value; }
}
public static final class Meta extends Values.View {
public Meta(Object value) { super(value); }
static Meta read(Object value) { return new Meta(value); }
public Presence<String> answeredBy() { return presence("answered_by", value -> (String)value); }
public Presence<List<Attempt>> attempts() { return presence("attempts", value -> Values.list(value, item0 -> Attempt.read(item0))); }
public Presence<BatchSetting> batchSetting() { return presence("batch_setting", value -> BatchSetting.read(value)); }
public Presence<BatchWarning> batchWarning() { return presence("batch_warning", value -> BatchWarning.read(value)); }
public Boolean cached() { Object value = required("cached"); return (Boolean)value; }
public Presence<String> contextSha256() { return presence("context_sha256", value -> (String)value); }
public BigInteger failedQuestions() { Object value = required("failed_questions"); return Values.integer(value); }
public String model() { Object value = required("model"); return (String)value; }
public List<Observation> observations() { Object value = required("observations"); return Values.list(value, item0 -> Observation.read(item0)); }
public Presence<Origin> origin() { required("origin"); return presence("origin", value -> Origin.read(value)); }
public Presence<ProfileWarning> profileWarning() { return presence("profile_warning", value -> ProfileWarning.read(value)); }
public Presence<String> questionSha256() { return presence("question_sha256", value -> (String)value); }
public List<QuestionSource> questionSources() { Object value = required("question_sources"); return Values.list(value, item0 -> QuestionSource.read(item0)); }
public Presence<String> questionsSha256() { return presence("questions_sha256", value -> (String)value); }
public List<String> requests() { Object value = required("requests"); return Values.list(value, item0 -> (String)item0); }
public BigInteger requestsSent() { Object value = required("requests_sent"); return Values.integer(value); }
public String tool() { Object value = required("tool"); return (String)value; }
public String url() { Object value = required("url"); return (String)value; }
public Presence<Usage> usage() { return presence("usage", value -> Usage.read(value)); }
}
public static final class ObjectRoot extends Values.View {
public ObjectRoot(Object value) { super(value); }
static ObjectRoot read(Object value) { return new ObjectRoot(value); }
public Map<String,InputPropertyType> properties() { Object value = required("properties"); return Values.map(value, item0 -> InputPropertyType.read(item0)); }
public Presence<List<String>> required() { return presence("required", value -> Values.list(value, item0 -> (String)item0)); }
public ObjectType type() { Object value = required("type"); return ObjectType.read(value); }
}
public static final class ObjectType extends Values.View {
public ObjectType(Object value) { super(value); }
static ObjectType read(Object value) { return new ObjectType(value); }
public String value() { return (String)json(); }
}
public sealed interface Observation extends Values.Value permits ObservationObservationId,ObservationFailureId {
static Observation read(Object value) { Map<String,Object> object = Values.object(value);
if (object.containsKey("observation_id")) return ObservationObservationId.read(value);
if (object.containsKey("failure_id")) return ObservationFailureId.read(value);
throw new IllegalStateException("Unknown native result alternative");
}
}
public static final class ObservationId extends Values.View {
public ObservationId(Object value) { super(value); }
static ObservationId read(Object value) { return new ObservationId(value); }
public String value() { return (String)json(); }
}
public static final class ObservationFailureId extends Values.View implements Observation {
public ObservationFailureId(Object value) { super(value); }
static ObservationFailureId read(Object value) { return new ObservationFailureId(value); }
public FailureId failureId() { Object value = required("failure_id"); return FailureId.read(value); }
}
public static final class ObservationObservationId extends Values.View implements Observation {
public ObservationObservationId(Object value) { super(value); }
static ObservationObservationId read(Object value) { return new ObservationObservationId(value); }
public ObservationId observationId() { Object value = required("observation_id"); return ObservationId.read(value); }
}
public static final class Origin extends Values.View {
public Origin(Object value) { super(value); }
static Origin read(Object value) { return new Origin(value); }
public String value() { return (String)json(); }
}
public static final class PersistenceObservation extends Values.View {
public PersistenceObservation(Object value) { super(value); }
static PersistenceObservation read(Object value) { return new PersistenceObservation(value); }
public Presence<String> advice() { return presence("advice", value -> (String)value); }
public String observedAt() { Object value = required("observed_at"); return (String)value; }
public UsagePersistence state() { Object value = required("state"); return UsagePersistence.read(value); }
}
public static final class PhysicalSource extends Values.View {
public PhysicalSource(Object value) { super(value); }
static PhysicalSource read(Object value) { return new PhysicalSource(value); }
public String file() { Object value = required("file"); return (String)value; }
public Presence<BigInteger> firstLine() { return presence("first_line", value -> Values.integer(value)); }
public Presence<BigInteger> lastLine() { return presence("last_line", value -> Values.integer(value)); }
}
public static final class Position extends Values.View {
public Position(Object value) { super(value); }
static Position read(Object value) { return new Position(value); }
public Presence<String> file() { required("file"); return presence("file", value -> (String)value); }
public Presence<BigInteger> first() { return presence("first", value -> Values.integer(value)); }
public Presence<List<String>> images() { return presence("images", value -> Values.list(value, item0 -> (String)item0)); }
public Presence<BigInteger> last() { return presence("last", value -> Values.integer(value)); }
}
public static final class QuestionName extends Values.View {
public QuestionName(Object value) { super(value); }
static QuestionName read(Object value) { return new QuestionName(value); }
public String value() { return (String)json(); }
}
public static final class QuestionSource extends Values.View {
public QuestionSource(Object value) { super(value); }
static QuestionSource read(Object value) { return new QuestionSource(value); }
public String answeredBy() { Object value = required("answered_by"); return (String)value; }
public Presence<BigInteger> batchSize() { return presence("batch_size", value -> Values.integer(value)); }
public Origin origin() { Object value = required("origin"); return Origin.read(value); }
}
public static final class RankMember extends Values.View {
public RankMember(Object value) { super(value); }
static RankMember read(Object value) { return new RankMember(value); }
public String name() { Object value = required("name"); return (String)value; }
public RankMemberResult result() { Object value = required("result"); return RankMemberResult.read(value); }
}
public static final class RankMemberResult extends Values.View {
public RankMemberResult(Object value) { super(value); }
static RankMemberResult read(Object value) { return new RankMemberResult(value); }
public Answer answer() { Object value = required("answer"); return Answer.read(value); }
public AnswerId answerId() { Object value = required("answer_id"); return AnswerId.read(value); }
public Presence<List<Image>> images() { return presence("images", value -> Values.list(value, item0 -> Image.read(item0))); }
public Meta meta() { Object value = required("meta"); return Meta.read(value); }
public ReadableQuestion question() { Object value = required("question"); return ReadableQuestion.read(value); }
public Version schema() { Object value = required("schema"); return Version.read(value); }
public Presence<PhysicalSource> source() { return presence("source", value -> PhysicalSource.read(value)); }
public Presence<Object> threshold() { required("threshold"); return presence("threshold", value -> Values.freeze(value)); }
public BigInteger value() { Object value = required("value"); return Values.integer(value); }
}
public sealed interface ReadableQuestion extends Values.Value permits ReadableQuestionDecide,ReadableQuestionChoose,ReadableQuestionTag,ReadableQuestionScore {
static ReadableQuestion read(Object value) { Map<String,Object> object = Values.object(value);
if ("decide".equals(object.get("verb"))) return ReadableQuestionDecide.read(value);
if ("choose".equals(object.get("verb"))) return ReadableQuestionChoose.read(value);
if ("tag".equals(object.get("verb"))) return ReadableQuestionTag.read(value);
if ("score".equals(object.get("verb"))) return ReadableQuestionScore.read(value);
throw new IllegalStateException("Unknown native result alternative");
}
}
public static final class ReadableQuestion2 extends Values.View {
public ReadableQuestion2(Object value) { super(value); }
static ReadableQuestion2 read(Object value) { return new ReadableQuestion2(value); }
public Presence<Batch> batch() { return presence("batch", value -> Batch.read(value)); }
public Presence<InputDeclaration> contextSchema() { return presence("context_schema", value -> InputDeclaration.read(value)); }
public Presence<InputDeclaration> itemSchema() { return presence("item_schema", value -> InputDeclaration.read(value)); }
public Presence<List<Label>> labelDetails() { return presence("label_details", value -> Values.list(value, item0 -> Label.read(item0))); }
public Presence<String> model() { return presence("model", value -> (String)value); }
public Presence<QuestionName> name() { return presence("name", value -> QuestionName.read(value)); }
public Boolean none() { Object value = required("none"); return (Boolean)value; }
public Presence<List<String>> on() { return presence("on", value -> Values.list(value, item0 -> (String)item0)); }
public Presence<String> profile() { return presence("profile", value -> (String)value); }
public Object text() { Object value = required("text"); return Values.freeze(value); }
public String verb() { Object value = required("verb"); return (String)value; }
public Presence<WordingVersion> wordingVersion() { return presence("wording_version", value -> WordingVersion.read(value)); }
}
public static final class ReadableQuestion3 extends Values.View {
public ReadableQuestion3(Object value) { super(value); }
static ReadableQuestion3 read(Object value) { return new ReadableQuestion3(value); }
public Presence<Batch> batch() { return presence("batch", value -> Batch.read(value)); }
public Presence<InputDeclaration> contextSchema() { return presence("context_schema", value -> InputDeclaration.read(value)); }
public Presence<Object> entityDefinition() { return presence("entity_definition", value -> Values.freeze(value)); }
public Presence<Object> instructions() { return presence("instructions", value -> Values.freeze(value)); }
public Presence<InputDeclaration> itemSchema() { return presence("item_schema", value -> InputDeclaration.read(value)); }
public Map<String,Object> kinds() { Object value = required("kinds"); return Values.map(value, item0 -> Values.freeze(item0)); }
public Presence<List<Label>> labelDetails() { return presence("label_details", value -> Values.list(value, item0 -> Label.read(item0))); }
public Presence<RecognitionMode> mode() { return presence("mode", value -> RecognitionMode.read(value)); }
public Presence<String> model() { return presence("model", value -> (String)value); }
public Presence<QuestionName> name() { return presence("name", value -> QuestionName.read(value)); }
public Presence<List<String>> on() { return presence("on", value -> Values.list(value, item0 -> (String)item0)); }
public Presence<String> profile() { return presence("profile", value -> (String)value); }
public Presence<Threshold> relationThreshold() { return presence("relation_threshold", value -> Threshold.read(value)); }
public Presence<List<RelationRule>> relations() { return presence("relations", value -> Values.list(value, item0 -> RelationRule.read(item0))); }
public Presence<BigInteger> snippetPieces() { return presence("snippet_pieces", value -> Values.integer(value)); }
public Presence<RecognitionStageContext> stageContext() { return presence("stage_context", value -> RecognitionStageContext.read(value)); }
public Threshold threshold() { Object value = required("threshold"); return Threshold.read(value); }
public Verb verb() { Object value = required("verb"); return Verb.read(value); }
public Presence<WordingVersion> wordingVersion() { return presence("wording_version", value -> WordingVersion.read(value)); }
}
public static final class ReadableQuestion4 extends Values.View {
public ReadableQuestion4(Object value) { super(value); }
static ReadableQuestion4 read(Object value) { return new ReadableQuestion4(value); }
public Presence<Batch> batch() { return presence("batch", value -> Batch.read(value)); }
public Presence<InputDeclaration> contextSchema() { return presence("context_schema", value -> InputDeclaration.read(value)); }
public Presence<RelateFields> fields() { required("fields"); return presence("fields", value -> RelateFields.read(value)); }
public Presence<InputDeclaration> itemSchema() { return presence("item_schema", value -> InputDeclaration.read(value)); }
public Presence<List<Label>> labelDetails() { return presence("label_details", value -> Values.list(value, item0 -> Label.read(item0))); }
public Presence<String> model() { return presence("model", value -> (String)value); }
public Presence<QuestionName> name() { return presence("name", value -> QuestionName.read(value)); }
public Presence<List<String>> on() { return presence("on", value -> Values.list(value, item0 -> (String)item0)); }
public Presence<String> profile() { return presence("profile", value -> (String)value); }
public List<RelationRule> relations() { Object value = required("relations"); return Values.list(value, item0 -> RelationRule.read(item0)); }
public Threshold threshold() { Object value = required("threshold"); return Threshold.read(value); }
public String verb() { Object value = required("verb"); return (String)value; }
public Presence<WordingVersion> wordingVersion() { return presence("wording_version", value -> WordingVersion.read(value)); }
}
public static final class ReadableQuestionChoose extends Values.View implements ReadableQuestion {
public ReadableQuestionChoose(Object value) { super(value); }
static ReadableQuestionChoose read(Object value) { return new ReadableQuestionChoose(value); }
public Presence<Batch> batch() { return presence("batch", value -> Batch.read(value)); }
public Presence<InputDeclaration> contextSchema() { return presence("context_schema", value -> InputDeclaration.read(value)); }
public Presence<InputDeclaration> itemSchema() { return presence("item_schema", value -> InputDeclaration.read(value)); }
public Presence<List<Label>> labelDetails() { return presence("label_details", value -> Values.list(value, item0 -> Label.read(item0))); }
public Presence<String> model() { return presence("model", value -> (String)value); }
public Presence<QuestionName> name() { return presence("name", value -> QuestionName.read(value)); }
public Presence<List<String>> on() { return presence("on", value -> Values.list(value, item0 -> (String)item0)); }
public Presence<String> profile() { return presence("profile", value -> (String)value); }
public Presence<WordingVersion> wordingVersion() { return presence("wording_version", value -> WordingVersion.read(value)); }
public List<String> options() { Object value = required("options"); return Values.list(value, item0 -> (String)item0); }
public Object text() { Object value = required("text"); return Values.freeze(value); }
public String verb() { Object value = required("verb"); return (String)value; }
}
public static final class ReadableQuestionDecide extends Values.View implements ReadableQuestion {
public ReadableQuestionDecide(Object value) { super(value); }
static ReadableQuestionDecide read(Object value) { return new ReadableQuestionDecide(value); }
public Presence<Batch> batch() { return presence("batch", value -> Batch.read(value)); }
public Presence<InputDeclaration> contextSchema() { return presence("context_schema", value -> InputDeclaration.read(value)); }
public Presence<InputDeclaration> itemSchema() { return presence("item_schema", value -> InputDeclaration.read(value)); }
public Presence<List<Label>> labelDetails() { return presence("label_details", value -> Values.list(value, item0 -> Label.read(item0))); }
public Presence<String> model() { return presence("model", value -> (String)value); }
public Presence<QuestionName> name() { return presence("name", value -> QuestionName.read(value)); }
public Presence<List<String>> on() { return presence("on", value -> Values.list(value, item0 -> (String)item0)); }
public Presence<String> profile() { return presence("profile", value -> (String)value); }
public Presence<WordingVersion> wordingVersion() { return presence("wording_version", value -> WordingVersion.read(value)); }
public Presence<Object> falseValue() { return presence("false", value -> Values.freeze(value)); }
public Object text() { Object value = required("text"); return Values.freeze(value); }
public Presence<Object> trueValue() { return presence("true", value -> Values.freeze(value)); }
public String verb() { Object value = required("verb"); return (String)value; }
}
public static final class ReadableQuestionScore extends Values.View implements ReadableQuestion {
public ReadableQuestionScore(Object value) { super(value); }
static ReadableQuestionScore read(Object value) { return new ReadableQuestionScore(value); }
public Presence<Batch> batch() { return presence("batch", value -> Batch.read(value)); }
public Presence<InputDeclaration> contextSchema() { return presence("context_schema", value -> InputDeclaration.read(value)); }
public Presence<InputDeclaration> itemSchema() { return presence("item_schema", value -> InputDeclaration.read(value)); }
public Presence<List<Label>> labelDetails() { return presence("label_details", value -> Values.list(value, item0 -> Label.read(item0))); }
public Presence<String> model() { return presence("model", value -> (String)value); }
public Presence<QuestionName> name() { return presence("name", value -> QuestionName.read(value)); }
public Presence<List<String>> on() { return presence("on", value -> Values.list(value, item0 -> (String)item0)); }
public Presence<String> profile() { return presence("profile", value -> (String)value); }
public Presence<WordingVersion> wordingVersion() { return presence("wording_version", value -> WordingVersion.read(value)); }
public List<String> levels() { Object value = required("levels"); return Values.list(value, item0 -> (String)item0); }
public Object text() { Object value = required("text"); return Values.freeze(value); }
public String verb() { Object value = required("verb"); return (String)value; }
}
public static final class ReadableQuestionTag extends Values.View implements ReadableQuestion {
public ReadableQuestionTag(Object value) { super(value); }
static ReadableQuestionTag read(Object value) { return new ReadableQuestionTag(value); }
public Presence<Batch> batch() { return presence("batch", value -> Batch.read(value)); }
public Presence<InputDeclaration> contextSchema() { return presence("context_schema", value -> InputDeclaration.read(value)); }
public Presence<InputDeclaration> itemSchema() { return presence("item_schema", value -> InputDeclaration.read(value)); }
public Presence<List<Label>> labelDetails() { return presence("label_details", value -> Values.list(value, item0 -> Label.read(item0))); }
public Presence<String> model() { return presence("model", value -> (String)value); }
public Presence<QuestionName> name() { return presence("name", value -> QuestionName.read(value)); }
public Presence<List<String>> on() { return presence("on", value -> Values.list(value, item0 -> (String)item0)); }
public Presence<String> profile() { return presence("profile", value -> (String)value); }
public Presence<WordingVersion> wordingVersion() { return presence("wording_version", value -> WordingVersion.read(value)); }
public List<String> labels() { Object value = required("labels"); return Values.list(value, item0 -> (String)item0); }
public Object text() { Object value = required("text"); return Values.freeze(value); }
public String verb() { Object value = required("verb"); return (String)value; }
}
public static final class Recognition extends Values.View {
public Recognition(Object value) { super(value); }
static Recognition read(Object value) { return new Recognition(value); }
public RecognitionOdds answer() { Object value = required("answer"); return RecognitionOdds.read(value); }
public AnswerId answerId() { Object value = required("answer_id"); return AnswerId.read(value); }
public Presence<String> file() { return presence("file", value -> (String)value); }
public Presence<BigInteger> firstLine() { return presence("first_line", value -> Values.integer(value)); }
public Presence<BigInteger> index() { return presence("index", value -> Values.integer(value)); }
public Presence<Object> input() { return presence("input", value -> Values.freeze(value)); }
public Presence<BigInteger> lastLine() { return presence("last_line", value -> Values.integer(value)); }
public Meta meta() { Object value = required("meta"); return Meta.read(value); }
public Presence<Position> position() { return presence("position", value -> Position.read(value)); }
public ReadableQuestion3 question() { Object value = required("question"); return ReadableQuestion3.read(value); }
public Version schema() { Object value = required("schema"); return Version.read(value); }
public Presence<PhysicalSource> source() { return presence("source", value -> PhysicalSource.read(value)); }
public Recognize value() { Object value = required("value"); return Recognize.read(value); }
}
public static final class RecognitionEdgeDocument extends Values.View {
public RecognitionEdgeDocument(Object value) { super(value); }
static RecognitionEdgeDocument read(Object value) { return new RecognitionEdgeDocument(value); }
public Boolean either() { Object value = required("either"); return (Boolean)value; }
public BigDecimal probability() { Object value = required("probability"); return (BigDecimal)value; }
public String relation() { Object value = required("relation"); return (String)value; }
public Entity source() { Object value = required("source"); return Entity.read(value); }
public Entity target() { Object value = required("target"); return Entity.read(value); }
}
public static final class RecognitionMode extends Values.View {
public RecognitionMode(Object value) { super(value); }
static RecognitionMode read(Object value) { return new RecognitionMode(value); }
public String value() { return (String)json(); }
}
public sealed interface RecognitionOdds extends Values.Value permits RecognitionOddsFieldsNamesPairsPiecesProposals,RecognitionOddsFieldsPiecesProposals {
static RecognitionOdds read(Object value) { Map<String,Object> object = Values.object(value);
if (object.containsKey("names") && object.containsKey("pairs") && object.containsKey("pieces") && object.containsKey("proposals")) return RecognitionOddsFieldsNamesPairsPiecesProposals.read(value);
if (object.containsKey("pieces") && object.containsKey("proposals") && !object.containsKey("names") && !object.containsKey("pairs")) return RecognitionOddsFieldsPiecesProposals.read(value);
throw new IllegalStateException("Unknown native result alternative");
}
}
public static final class RecognitionOddsFieldsNamesPairsPiecesProposals extends Values.View implements RecognitionOdds {
public RecognitionOddsFieldsNamesPairsPiecesProposals(Object value) { super(value); }
static RecognitionOddsFieldsNamesPairsPiecesProposals read(Object value) { return new RecognitionOddsFieldsNamesPairsPiecesProposals(value); }
public List<NameOdds> names() { Object value = required("names"); return Values.list(value, item0 -> NameOdds.read(item0)); }
public List<PairOdds> pairs() { Object value = required("pairs"); return Values.list(value, item0 -> PairOdds.read(item0)); }
public List<PieceOdds> pieces() { Object value = required("pieces"); return Values.list(value, item0 -> PieceOdds.read(item0)); }
public List<RecognitionProposal> proposals() { Object value = required("proposals"); return Values.list(value, item0 -> RecognitionProposal.read(item0)); }
}
public static final class RecognitionOddsFieldsPiecesProposals extends Values.View implements RecognitionOdds {
public RecognitionOddsFieldsPiecesProposals(Object value) { super(value); }
static RecognitionOddsFieldsPiecesProposals read(Object value) { return new RecognitionOddsFieldsPiecesProposals(value); }
public List<PieceOdds> pieces() { Object value = required("pieces"); return Values.list(value, item0 -> PieceOdds.read(item0)); }
public List<BoundaryProposal> proposals() { Object value = required("proposals"); return Values.list(value, item0 -> BoundaryProposal.read(item0)); }
}
public static final class RecognitionProposal extends Values.View {
public RecognitionProposal(Object value) { super(value); }
static RecognitionProposal read(Object value) { return new RecognitionProposal(value); }
public BigInteger end() { Object value = required("end"); return Values.integer(value); }
public Boolean kept() { Object value = required("kept"); return (Boolean)value; }
public Presence<String> kind() { return presence("kind", value -> (String)value); }
public Presence<Place> selected() { return presence("selected", value -> Place.read(value)); }
public BigDecimal spanProbability() { Object value = required("span_probability"); return (BigDecimal)value; }
public BigInteger start() { Object value = required("start"); return Values.integer(value); }
public Presence<BigDecimal> strength() { return presence("strength", value -> (BigDecimal)value); }
}
public static final class RecognitionStageContext extends Values.View {
public RecognitionStageContext(Object value) { super(value); }
static RecognitionStageContext read(Object value) { return new RecognitionStageContext(value); }
public Presence<String> boundary() { return presence("boundary", value -> (String)value); }
public Presence<String> kindEdge() { return presence("kind_edge", value -> (String)value); }
public Presence<String> relation() { return presence("relation", value -> (String)value); }
}
public static final class Relation extends Values.View {
public Relation(Object value) { super(value); }
static Relation read(Object value) { return new Relation(value); }
public Answers answer() { Object value = required("answer"); return Answers.read(value); }
public AnswerId answerId() { Object value = required("answer_id"); return AnswerId.read(value); }
public Presence<String> file() { return presence("file", value -> (String)value); }
public Presence<BigInteger> firstLine() { return presence("first_line", value -> Values.integer(value)); }
public Presence<BigInteger> index() { return presence("index", value -> Values.integer(value)); }
public Presence<Object> input() { return presence("input", value -> Values.freeze(value)); }
public Presence<BigInteger> lastLine() { return presence("last_line", value -> Values.integer(value)); }
public Meta meta() { Object value = required("meta"); return Meta.read(value); }
public Presence<Position> position() { return presence("position", value -> Position.read(value)); }
public ReadableQuestion4 question() { Object value = required("question"); return ReadableQuestion4.read(value); }
public Version schema() { Object value = required("schema"); return Version.read(value); }
public List<RelatedEntityEdge> value() { Object value = required("value"); return Values.list(value, item0 -> RelatedEntityEdge.read(item0)); }
}
public static final class RelationDirection extends Values.View {
public RelationDirection(Object value) { super(value); }
static RelationDirection read(Object value) { return new RelationDirection(value); }
public String value() { return (String)json(); }
}
public sealed interface RelationMember extends Values.Value permits RelationMemberAnswerId,RelationMemberFailureId {
static RelationMember read(Object value) { Map<String,Object> object = Values.object(value);
if (object.containsKey("answer_id")) return RelationMemberAnswerId.read(value);
if (object.containsKey("failure_id")) return RelationMemberFailureId.read(value);
throw new IllegalStateException("Unknown native result alternative");
}
}
public static final class RelationMemberAnswerId extends Values.View implements RelationMember {
public RelationMemberAnswerId(Object value) { super(value); }
static RelationMemberAnswerId read(Object value) { return new RelationMemberAnswerId(value); }
public RelationDirection direction() { Object value = required("direction"); return RelationDirection.read(value); }
public RelationMethod method() { Object value = required("method"); return RelationMethod.read(value); }
public List<Observation> observations() { Object value = required("observations"); return Values.list(value, item0 -> Observation.read(item0)); }
public ReadableQuestion question() { Object value = required("question"); return ReadableQuestion.read(value); }
public List<QuestionSource> questionSources() { Object value = required("question_sources"); return Values.list(value, item0 -> QuestionSource.read(item0)); }
public String reads() { Object value = required("reads"); return (String)value; }
public String relation() { Object value = required("relation"); return (String)value; }
public String request() { Object value = required("request"); return (String)value; }
public RelatedEntity source() { Object value = required("source"); return RelatedEntity.read(value); }
public Presence<RelatedEntity> target() { required("target"); return presence("target", value -> RelatedEntity.read(value)); }
public Threshold threshold() { Object value = required("threshold"); return Threshold.read(value); }
public Presence<Usage> usage() { return presence("usage", value -> Usage.read(value)); }
public Boolean accepted() { Object value = required("accepted"); return (Boolean)value; }
public Answer answer() { Object value = required("answer"); return Answer.read(value); }
public AnswerId answerId() { Object value = required("answer_id"); return AnswerId.read(value); }
public BigDecimal probability() { Object value = required("probability"); return (BigDecimal)value; }
}
public static final class RelationMemberFailureId extends Values.View implements RelationMember {
public RelationMemberFailureId(Object value) { super(value); }
static RelationMemberFailureId read(Object value) { return new RelationMemberFailureId(value); }
public RelationDirection direction() { Object value = required("direction"); return RelationDirection.read(value); }
public RelationMethod method() { Object value = required("method"); return RelationMethod.read(value); }
public List<Observation> observations() { Object value = required("observations"); return Values.list(value, item0 -> Observation.read(item0)); }
public ReadableQuestion question() { Object value = required("question"); return ReadableQuestion.read(value); }
public List<QuestionSource> questionSources() { Object value = required("question_sources"); return Values.list(value, item0 -> QuestionSource.read(item0)); }
public String reads() { Object value = required("reads"); return (String)value; }
public String relation() { Object value = required("relation"); return (String)value; }
public String request() { Object value = required("request"); return (String)value; }
public RelatedEntity source() { Object value = required("source"); return RelatedEntity.read(value); }
public Presence<RelatedEntity> target() { required("target"); return presence("target", value -> RelatedEntity.read(value)); }
public Threshold threshold() { Object value = required("threshold"); return Threshold.read(value); }
public Presence<Usage> usage() { return presence("usage", value -> Usage.read(value)); }
public Failure failure() { Object value = required("failure"); return Failure.read(value); }
public FailureId failureId() { Object value = required("failure_id"); return FailureId.read(value); }
}
public static final class RelationMethod extends Values.View {
public RelationMethod(Object value) { super(value); }
static RelationMethod read(Object value) { return new RelationMethod(value); }
public String value() { return (String)json(); }
}
public static final class RequestFunction extends Values.View {
public RequestFunction(Object value) { super(value); }
static RequestFunction read(Object value) { return new RequestFunction(value); }
public String value() { return (String)json(); }
}
public static final class SdkRequestId extends Values.View {
public SdkRequestId(Object value) { super(value); }
static SdkRequestId read(Object value) { return new SdkRequestId(value); }
public String value() { return (String)json(); }
}
public sealed interface SendBudgetDenial extends Values.Value permits SendBudgetDenialBeforeFirstSend,SendBudgetDenialBeforeAdditionalSend,SendBudgetDenialBeforeRetry {
static SendBudgetDenial read(Object value) { Map<String,Object> object = Values.object(value);
if ("before_first_send".equals(object.get("kind"))) return SendBudgetDenialBeforeFirstSend.read(value);
if ("before_additional_send".equals(object.get("kind"))) return SendBudgetDenialBeforeAdditionalSend.read(value);
if ("before_retry".equals(object.get("kind"))) return SendBudgetDenialBeforeRetry.read(value);
throw new IllegalStateException("Unknown native result alternative");
}
}
public static final class SendBudgetDenialBeforeAdditionalSend extends Values.View implements SendBudgetDenial {
public SendBudgetDenialBeforeAdditionalSend(Object value) { super(value); }
static SendBudgetDenialBeforeAdditionalSend read(Object value) { return new SendBudgetDenialBeforeAdditionalSend(value); }
public String kind() { Object value = required("kind"); return (String)value; }
}
public static final class SendBudgetDenialBeforeFirstSend extends Values.View implements SendBudgetDenial {
public SendBudgetDenialBeforeFirstSend(Object value) { super(value); }
static SendBudgetDenialBeforeFirstSend read(Object value) { return new SendBudgetDenialBeforeFirstSend(value); }
public String kind() { Object value = required("kind"); return (String)value; }
}
public static final class SendBudgetDenialBeforeRetry extends Values.View implements SendBudgetDenial {
public SendBudgetDenialBeforeRetry(Object value) { super(value); }
static SendBudgetDenialBeforeRetry read(Object value) { return new SendBudgetDenialBeforeRetry(value); }
public String kind() { Object value = required("kind"); return (String)value; }
public BigInteger lastStatus() { Object value = required("last_status"); return Values.integer(value); }
}
public static final class StopCause extends Values.View {
public StopCause(Object value) { super(value); }
static StopCause read(Object value) { return new StopCause(value); }
public String value() { return (String)json(); }
}
public static final class Stopped extends Values.View {
public Stopped(Object value) { super(value); }
static Stopped read(Object value) { return new Stopped(value); }
public Presence<BigInteger> at() { return presence("at", value -> Values.integer(value)); }
public StopCause cause() { Object value = required("cause"); return StopCause.read(value); }
public Boolean retryable() { Object value = required("retryable"); return (Boolean)value; }
public Presence<BigInteger> status() { return presence("status", value -> Values.integer(value)); }
}
public static final class StringRoot extends Values.View {
public StringRoot(Object value) { super(value); }
static StringRoot read(Object value) { return new StringRoot(value); }
public StringType type() { Object value = required("type"); return StringType.read(value); }
}
public static final class StringType extends Values.View {
public StringType(Object value) { super(value); }
static StringType read(Object value) { return new StringType(value); }
public String value() { return (String)json(); }
}
public static final class Usage extends Values.View {
public Usage(Object value) { super(value); }
static Usage read(Object value) { return new Usage(value); }
public Presence<BigInteger> inputTokens() { return presence("input_tokens", value -> Values.integer(value)); }
public Presence<BigInteger> outputTokens() { return presence("output_tokens", value -> Values.integer(value)); }
}
public static final class UsagePersistence extends Values.View {
public UsagePersistence(Object value) { super(value); }
static UsagePersistence read(Object value) { return new UsagePersistence(value); }
public String value() { return (String)json(); }
}
public static final class Verb extends Values.View {
public Verb(Object value) { super(value); }
static Verb read(Object value) { return new Verb(value); }
public String value() { return (String)json(); }
}
public static final class Version extends Values.View {
public Version(Object value) { super(value); }
static Version read(Object value) { return new Version(value); }
public String value() { return (String)json(); }
}
public static final class WordingVersion extends Values.View {
public WordingVersion(Object value) { super(value); }
static WordingVersion read(Object value) { return new WordingVersion(value); }
public BigInteger value() { return Values.integer(json()); }
}
public sealed interface AnnotatedField extends Values.Value permits AnnotatedFieldBoolean,AnnotatedFieldNull,AnnotatedFieldString,AnnotatedFieldArray,AnnotatedFieldNumber,AnnotatedFieldObject {
static AnnotatedField read(Object value) {
if (value instanceof Boolean) return AnnotatedFieldBoolean.read(value);
if (value == null) return new AnnotatedFieldNull(null);
if (value instanceof String) return AnnotatedFieldString.read(value);
if (value instanceof List<?>) return AnnotatedFieldArray.read(value);
if (value instanceof BigDecimal) return AnnotatedFieldNumber.read(value);
if (value instanceof Map<?,?>) return AnnotatedFieldObject.read(value);
throw new IllegalStateException("Unknown native value alternative");
}
}
public record AnnotatedFieldBoolean(Boolean value) implements AnnotatedField {
static AnnotatedFieldBoolean read(Object value) { return new AnnotatedFieldBoolean((Boolean)value); }
public Object json() { return Values.json(value); }
}
public record AnnotatedFieldNull(Object value) implements AnnotatedField {
static AnnotatedFieldNull read(Object value) { return new AnnotatedFieldNull(Values.freeze(value)); }
public Object json() { return Values.json(value); }
}
public record AnnotatedFieldString(String value) implements AnnotatedField {
static AnnotatedFieldString read(Object value) { return new AnnotatedFieldString((String)value); }
public Object json() { return Values.json(value); }
}
public record AnnotatedFieldArray(List<String> value) implements AnnotatedField {
static AnnotatedFieldArray read(Object value) { return new AnnotatedFieldArray(Values.list(value, item0 -> (String)item0)); }
public Object json() { return Values.json(value); }
}
public record AnnotatedFieldNumber(BigDecimal value) implements AnnotatedField {
static AnnotatedFieldNumber read(Object value) { return new AnnotatedFieldNumber((BigDecimal)value); }
public Object json() { return Values.json(value); }
}
public record AnnotatedFieldObject(Failed value) implements AnnotatedField {
static AnnotatedFieldObject read(Object value) { return new AnnotatedFieldObject(Failed.read(value)); }
public Object json() { return Values.json(value); }
}
public static final class AnnotatedRow extends Values.View {
public AnnotatedRow(Object value) { super(value); }
static AnnotatedRow read(Object value) { return new AnnotatedRow(value); }
public Map<String,AnnotatedField> value() { return Values.map(json(), item0 -> AnnotatedField.read(item0)); }
}
public sealed interface Answer extends Values.Value permits AnswerYesNo,AnswerChoice,AnswerTag,AnswerScore {
static Answer read(Object value) { Map<String,Object> object = Values.object(value);
if ("yes_no".equals(object.get("kind"))) return AnswerYesNo.read(value);
if ("choice".equals(object.get("kind"))) return AnswerChoice.read(value);
if ("tag".equals(object.get("kind"))) return AnswerTag.read(value);
if ("score".equals(object.get("kind"))) return AnswerScore.read(value);
throw new IllegalStateException("Unknown native result alternative");
}
}
public static final class AnswerChoice extends Values.View implements Answer {
public AnswerChoice(Object value) { super(value); }
static AnswerChoice read(Object value) { return new AnswerChoice(value); }
public Presence<BigDecimal> confidence() { return presence("confidence", value -> (BigDecimal)value); }
public String kind() { Object value = required("kind"); return (String)value; }
public String pick() { Object value = required("pick"); return (String)value; }
public Map<String,BigDecimal> probabilities() { Object value = required("probabilities"); return Values.map(value, item0 -> (BigDecimal)item0); }
}
public static final class AnswerScore extends Values.View implements Answer {
public AnswerScore(Object value) { super(value); }
static AnswerScore read(Object value) { return new AnswerScore(value); }
public Presence<BigDecimal> confidence() { return presence("confidence", value -> (BigDecimal)value); }
public String kind() { Object value = required("kind"); return (String)value; }
public String level() { Object value = required("level"); return (String)value; }
public Map<String,BigDecimal> probabilities() { Object value = required("probabilities"); return Values.map(value, item0 -> (BigDecimal)item0); }
}
public static final class AnswerTag extends Values.View implements Answer {
public AnswerTag(Object value) { super(value); }
static AnswerTag read(Object value) { return new AnswerTag(value); }
public String kind() { Object value = required("kind"); return (String)value; }
public Map<String,BigDecimal> probabilities() { Object value = required("probabilities"); return Values.map(value, item0 -> (BigDecimal)item0); }
}
public static final class AnswerYesNo extends Values.View implements Answer {
public AnswerYesNo(Object value) { super(value); }
static AnswerYesNo read(Object value) { return new AnswerYesNo(value); }
public String kind() { Object value = required("kind"); return (String)value; }
public BigDecimal probability() { Object value = required("probability"); return (BigDecimal)value; }
}
public static final class AttemptOutcome extends Values.View {
public AttemptOutcome(Object value) { super(value); }
static AttemptOutcome read(Object value) { return new AttemptOutcome(value); }
public String value() { return (String)json(); }
}
public sealed interface BatchSetting extends Values.Value permits BatchSettingInteger,BatchSettingString {
static BatchSetting read(Object value) {
if (value instanceof BigDecimal) return BatchSettingInteger.read(value);
if (value instanceof String) return BatchSettingString.read(value);
throw new IllegalStateException("Unknown native value alternative");
}
}
public record BatchSettingInteger(BigInteger value) implements BatchSetting {
static BatchSettingInteger read(Object value) { return new BatchSettingInteger(Values.integer(value)); }
public Object json() { return Values.json(value); }
}
public record BatchSettingString(String value) implements BatchSetting {
static BatchSettingString read(Object value) { return new BatchSettingString((String)value); }
public Object json() { return Values.json(value); }
}
public static final class BatchWarning extends Values.View {
public BatchWarning(Object value) { super(value); }
static BatchWarning read(Object value) { return new BatchWarning(value); }
public BatchSetting running() { Object value = required("running"); return BatchSetting.read(value); }
public BatchSetting tunedFor() { Object value = required("tuned_for"); return BatchSetting.read(value); }
}
public static final class Entity extends Values.View {
public Entity(Object value) { super(value); }
static Entity read(Object value) { return new Entity(value); }
public BigInteger end() { Object value = required("end"); return Values.integer(value); }
public Presence<String> file() { return presence("file", value -> (String)value); }
public Presence<BigInteger> firstLine() { return presence("first_line", value -> Values.integer(value)); }
public String kind() { Object value = required("kind"); return (String)value; }
public Presence<BigInteger> lastLine() { return presence("last_line", value -> Values.integer(value)); }
public BigInteger length() { Object value = required("length"); return Values.integer(value); }
public BigInteger start() { Object value = required("start"); return Values.integer(value); }
public BigDecimal strength() { Object value = required("strength"); return (BigDecimal)value; }
public String text() { Object value = required("text"); return (String)value; }
}
public static final class EntityEdge extends Values.View {
public EntityEdge(Object value) { super(value); }
static EntityEdge read(Object value) { return new EntityEdge(value); }
public Presence<Boolean> either() { return presence("either", value -> (Boolean)value); }
public BigDecimal probability() { Object value = required("probability"); return (BigDecimal)value; }
public String relation() { Object value = required("relation"); return (String)value; }
public Entity source() { Object value = required("source"); return Entity.read(value); }
public Entity target() { Object value = required("target"); return Entity.read(value); }
}
public static final class Failed extends Values.View {
public Failed(Object value) { super(value); }
static Failed read(Object value) { return new Failed(value); }
public Failure failed() { Object value = required("failed"); return Failure.read(value); }
}
public static final class Failure extends Values.View {
public Failure(Object value) { super(value); }
static Failure read(Object value) { return new Failure(value); }
public FailureCause cause() { Object value = required("cause"); return FailureCause.read(value); }
public String kind() { Object value = required("kind"); return (String)value; }
}
public static final class FailureCause extends Values.View {
public FailureCause(Object value) { super(value); }
static FailureCause read(Object value) { return new FailureCause(value); }
public String value() { return (String)json(); }
}
public static final class FailureKind extends Values.View {
public FailureKind(Object value) { super(value); }
static FailureKind read(Object value) { return new FailureKind(value); }
public String value() { return (String)json(); }
}
public static final class FindAnswer extends Values.View {
public FindAnswer(Object value) { super(value); }
static FindAnswer read(Object value) { return new FindAnswer(value); }
public Presence<BigDecimal> confidence() { return presence("confidence", value -> (BigDecimal)value); }
public String kind() { Object value = required("kind"); return (String)value; }
public String pick() { Object value = required("pick"); return (String)value; }
public Map<String,BigDecimal> probabilities() { Object value = required("probabilities"); return Values.map(value, item0 -> (BigDecimal)item0); }
}
public static final class NameOdds extends Values.View {
public NameOdds(Object value) { super(value); }
static NameOdds read(Object value) { return new NameOdds(value); }
public Presence<Map<String,BigDecimal>> edges() { required("edges"); return presence("edges", value -> Values.map(value, item0 -> (BigDecimal)item0)); }
public BigInteger end() { Object value = required("end"); return Values.integer(value); }
public Presence<Map<String,BigDecimal>> kinds() { required("kinds"); return presence("kinds", value -> Values.map(value, item0 -> (BigDecimal)item0)); }
public BigInteger start() { Object value = required("start"); return Values.integer(value); }
}
public static final class PairOdds extends Values.View {
public PairOdds(Object value) { super(value); }
static PairOdds read(Object value) { return new PairOdds(value); }
public BigDecimal probability() { Object value = required("probability"); return (BigDecimal)value; }
public String relation() { Object value = required("relation"); return (String)value; }
public Place source() { Object value = required("source"); return Place.read(value); }
public Place target() { Object value = required("target"); return Place.read(value); }
}
public static final class PieceOdds extends Values.View {
public PieceOdds(Object value) { super(value); }
static PieceOdds read(Object value) { return new PieceOdds(value); }
public BigInteger end() { Object value = required("end"); return Values.integer(value); }
public BigInteger start() { Object value = required("start"); return Values.integer(value); }
public Map<String,BigDecimal> tags() { Object value = required("tags"); return Values.map(value, item0 -> (BigDecimal)item0); }
}
public static final class Place extends Values.View {
public Place(Object value) { super(value); }
static Place read(Object value) { return new Place(value); }
public BigInteger end() { Object value = required("end"); return Values.integer(value); }
public BigInteger start() { Object value = required("start"); return Values.integer(value); }
}
public static final class Plan extends Values.View {
public Plan(Object value) { super(value); }
static Plan read(Object value) { return new Plan(value); }
public BigInteger estimatedBytes() { Object value = required("estimated_bytes"); return Values.integer(value); }
public TokenBand estimatedInputTokens() { Object value = required("estimated_input_tokens"); return TokenBand.read(value); }
public Presence<String> firstBodyUtf8() { required("first_body_utf8"); return presence("first_body_utf8", value -> (String)value); }
public BigInteger largestRequestBytes() { Object value = required("largest_request_bytes"); return Values.integer(value); }
public BigInteger largestRequestEstimatedInputTokens() { Object value = required("largest_request_estimated_input_tokens"); return Values.integer(value); }
public BigInteger records() { Object value = required("records"); return Values.integer(value); }
public BigInteger requests() { Object value = required("requests"); return Values.integer(value); }
public String tokenEstimateMethod() { Object value = required("token_estimate_method"); return (String)value; }
public Boolean upperBound() { Object value = required("upper_bound"); return (Boolean)value; }
}
public static final class ProfileWarning extends Values.View {
public ProfileWarning(Object value) { super(value); }
static ProfileWarning read(Object value) { return new ProfileWarning(value); }
public String running() { Object value = required("running"); return (String)value; }
public String tunedFor() { Object value = required("tuned_for"); return (String)value; }
}
public sealed interface Recognize extends Values.Value permits RecognizeFieldsEntities,RecognizeFieldsModeProposals {
static Recognize read(Object value) { Map<String,Object> object = Values.object(value);
if (object.containsKey("entities") && !object.containsKey("mode") && !object.containsKey("proposals")) return RecognizeFieldsEntities.read(value);
if (object.containsKey("mode") && object.containsKey("proposals") && !object.containsKey("entities")) return RecognizeFieldsModeProposals.read(value);
throw new IllegalStateException("Unknown native result alternative");
}
}
public static final class RecognizeAnswer extends Values.View {
public RecognizeAnswer(Object value) { super(value); }
static RecognizeAnswer read(Object value) { return new RecognizeAnswer(value); }
public List<NameOdds> names() { Object value = required("names"); return Values.list(value, item0 -> NameOdds.read(item0)); }
public List<PairOdds> pairs() { Object value = required("pairs"); return Values.list(value, item0 -> PairOdds.read(item0)); }
public List<PieceOdds> pieces() { Object value = required("pieces"); return Values.list(value, item0 -> PieceOdds.read(item0)); }
public List<RecognitionProposal> proposals() { Object value = required("proposals"); return Values.list(value, item0 -> RecognitionProposal.read(item0)); }
}
public static final class RecognizeFieldsEntities extends Values.View implements Recognize {
public RecognizeFieldsEntities(Object value) { super(value); }
static RecognizeFieldsEntities read(Object value) { return new RecognizeFieldsEntities(value); }
public List<Entity> entities() { Object value = required("entities"); return Values.list(value, item0 -> Entity.read(item0)); }
public Presence<List<EntityEdge>> relations() { return presence("relations", value -> Values.list(value, item0 -> EntityEdge.read(item0))); }
}
public static final class RecognizeFieldsModeProposals extends Values.View implements Recognize {
public RecognizeFieldsModeProposals(Object value) { super(value); }
static RecognizeFieldsModeProposals read(Object value) { return new RecognizeFieldsModeProposals(value); }
public BoundaryMode mode() { Object value = required("mode"); return BoundaryMode.read(value); }
public List<BoundaryProposal> proposals() { Object value = required("proposals"); return Values.list(value, item0 -> BoundaryProposal.read(item0)); }
}
public static final class RelateFields extends Values.View {
public RelateFields(Object value) { super(value); }
static RelateFields read(Object value) { return new RelateFields(value); }
public String kind() { Object value = required("kind"); return (String)value; }
public String name() { Object value = required("name"); return (String)value; }
}
public static final class RelatedEntity extends Values.View {
public RelatedEntity(Object value) { super(value); }
static RelatedEntity read(Object value) { return new RelatedEntity(value); }
public String kind() { Object value = required("kind"); return (String)value; }
public String name() { Object value = required("name"); return (String)value; }
}
public static final class RelatedEntityEdge extends Values.View {
public RelatedEntityEdge(Object value) { super(value); }
static RelatedEntityEdge read(Object value) { return new RelatedEntityEdge(value); }
public Presence<Boolean> either() { return presence("either", value -> (Boolean)value); }
public BigDecimal probability() { Object value = required("probability"); return (BigDecimal)value; }
public String relation() { Object value = required("relation"); return (String)value; }
public RelatedEntityEdgePropertiesSource source() { Object value = required("source"); return RelatedEntityEdgePropertiesSource.read(value); }
public RelatedEntityEdgePropertiesSource target() { Object value = required("target"); return RelatedEntityEdgePropertiesSource.read(value); }
}
public sealed interface RelatedEntityEdgePropertiesSource extends Values.Value permits RelatedEntityEdgePropertiesSourceFieldsKindName,RelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord {
static RelatedEntityEdgePropertiesSource read(Object value) { Map<String,Object> object = Values.object(value);
if (object.containsKey("kind") && object.containsKey("name") && !object.containsKey("file") && !object.containsKey("ordinal") && !object.containsKey("record")) return RelatedEntityEdgePropertiesSourceFieldsKindName.read(value);
if (object.containsKey("file") && object.containsKey("kind") && object.containsKey("name") && object.containsKey("ordinal") && object.containsKey("record")) return RelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord.read(value);
throw new IllegalStateException("Unknown native result alternative");
}
}
public static final class RelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord extends Values.View implements RelatedEntityEdgePropertiesSource {
public RelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord(Object value) { super(value); }
static RelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord read(Object value) { return new RelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord(value); }
public Presence<String> file() { required("file"); return presence("file", value -> (String)value); }
public Presence<BigInteger> firstLine() { return presence("first_line", value -> Values.integer(value)); }
public String kind() { Object value = required("kind"); return (String)value; }
public Presence<BigInteger> lastLine() { return presence("last_line", value -> Values.integer(value)); }
public String name() { Object value = required("name"); return (String)value; }
public BigInteger ordinal() { Object value = required("ordinal"); return Values.integer(value); }
public Object record() { Object value = required("record"); return Values.freeze(value); }
}
public static final class RelatedEntityEdgePropertiesSourceFieldsKindName extends Values.View implements RelatedEntityEdgePropertiesSource {
public RelatedEntityEdgePropertiesSourceFieldsKindName(Object value) { super(value); }
static RelatedEntityEdgePropertiesSourceFieldsKindName read(Object value) { return new RelatedEntityEdgePropertiesSourceFieldsKindName(value); }
public String kind() { Object value = required("kind"); return (String)value; }
public String name() { Object value = required("name"); return (String)value; }
}
public static final class RelationRule extends Values.View {
public RelationRule(Object value) { super(value); }
static RelationRule read(Object value) { return new RelationRule(value); }
public Boolean either() { Object value = required("either"); return (Boolean)value; }
public String name() { Object value = required("name"); return (String)value; }
public String reads() { Object value = required("reads"); return (String)value; }
public Presence<Boolean> single() { return presence("single", value -> (Boolean)value); }
public String source() { Object value = required("source"); return (String)value; }
public String target() { Object value = required("target"); return (String)value; }
}
public static final class SessionAnnotation extends Values.View {
public SessionAnnotation(Object value) { super(value); }
static SessionAnnotation read(Object value) { return new SessionAnnotation(value); }
public String name() { Object value = required("name"); return (String)value; }
public AnnotationValue value() { Object value = required("value"); return AnnotationValue.read(value); }
}
public static final class SessionInputSource extends Values.View {
public SessionInputSource(Object value) { super(value); }
static SessionInputSource read(Object value) { return new SessionInputSource(value); }
public BigInteger index() { Object value = required("index"); return Values.integer(value); }
public PhysicalSource source() { Object value = required("source"); return PhysicalSource.read(value); }
}
public sealed interface SessionJudgment extends Values.Value permits SessionJudgmentDecision,SessionJudgmentChoice,SessionJudgmentScore,SessionJudgmentTags {
static SessionJudgment read(Object value) { Map<String,Object> object = Values.object(value);
if ("decision".equals(object.get("kind"))) return SessionJudgmentDecision.read(value);
if ("choice".equals(object.get("kind"))) return SessionJudgmentChoice.read(value);
if ("score".equals(object.get("kind"))) return SessionJudgmentScore.read(value);
if ("tags".equals(object.get("kind"))) return SessionJudgmentTags.read(value);
throw new IllegalStateException("Unknown native result alternative");
}
}
public static final class SessionJudgmentChoice extends Values.View implements SessionJudgment {
public SessionJudgmentChoice(Object value) { super(value); }
static SessionJudgmentChoice read(Object value) { return new SessionJudgmentChoice(value); }
public String kind() { Object value = required("kind"); return (String)value; }
public Presence<String> value() { required("value"); return presence("value", value -> (String)value); }
}
public static final class SessionJudgmentDecision extends Values.View implements SessionJudgment {
public SessionJudgmentDecision(Object value) { super(value); }
static SessionJudgmentDecision read(Object value) { return new SessionJudgmentDecision(value); }
public String kind() { Object value = required("kind"); return (String)value; }
public Presence<Boolean> value() { required("value"); return presence("value", value -> (Boolean)value); }
}
public static final class SessionJudgmentScore extends Values.View implements SessionJudgment {
public SessionJudgmentScore(Object value) { super(value); }
static SessionJudgmentScore read(Object value) { return new SessionJudgmentScore(value); }
public String kind() { Object value = required("kind"); return (String)value; }
public BigDecimal value() { Object value = required("value"); return (BigDecimal)value; }
}
public static final class SessionJudgmentTags extends Values.View implements SessionJudgment {
public SessionJudgmentTags(Object value) { super(value); }
static SessionJudgmentTags read(Object value) { return new SessionJudgmentTags(value); }
public String kind() { Object value = required("kind"); return (String)value; }
public List<String> value() { Object value = required("value"); return Values.list(value, item0 -> (String)item0); }
}
public static final class SessionNamedProbability extends Values.View {
public SessionNamedProbability(Object value) { super(value); }
static SessionNamedProbability read(Object value) { return new SessionNamedProbability(value); }
public String name() { Object value = required("name"); return (String)value; }
public BigDecimal probability() { Object value = required("probability"); return (BigDecimal)value; }
}
public sealed interface SessionObservation extends Values.Value permits SessionObservationQuestion,SessionObservationRow {
static SessionObservation read(Object value) { Map<String,Object> object = Values.object(value);
if ("question".equals(object.get("kind"))) return SessionObservationQuestion.read(value);
if ("row".equals(object.get("kind"))) return SessionObservationRow.read(value);
throw new IllegalStateException("Unknown native result alternative");
}
}
public static final class SessionObservationQuestion extends Values.View implements SessionObservation {
public SessionObservationQuestion(Object value) { super(value); }
static SessionObservationQuestion read(Object value) { return new SessionObservationQuestion(value); }
public SessionQuestionDetail detail() { Object value = required("detail"); return SessionQuestionDetail.read(value); }
public BigInteger index() { Object value = required("index"); return Values.integer(value); }
public String kind() { Object value = required("kind"); return (String)value; }
public Presence<String> member() { return presence("member", value -> (String)value); }
public BigInteger position() { Object value = required("position"); return Values.integer(value); }
public Presence<String> stage() { return presence("stage", value -> (String)value); }
}
public static final class SessionObservationRow extends Values.View implements SessionObservation {
public SessionObservationRow(Object value) { super(value); }
static SessionObservationRow read(Object value) { return new SessionObservationRow(value); }
public BigInteger index() { Object value = required("index"); return Values.integer(value); }
public String kind() { Object value = required("kind"); return (String)value; }
public SessionObservedRow value() { Object value = required("value"); return SessionObservedRow.read(value); }
}
public sealed interface SessionObservedRow extends Values.Value permits SessionObservedRowJudgment,SessionObservedRowAnnotated,SessionObservedRowRecognized,SessionObservedRowFind,SessionObservedRowRelations {
static SessionObservedRow read(Object value) { Map<String,Object> object = Values.object(value);
if ("judgment".equals(object.get("kind"))) return SessionObservedRowJudgment.read(value);
if ("annotated".equals(object.get("kind"))) return SessionObservedRowAnnotated.read(value);
if ("recognized".equals(object.get("kind"))) return SessionObservedRowRecognized.read(value);
if ("find".equals(object.get("kind"))) return SessionObservedRowFind.read(value);
if ("relations".equals(object.get("kind"))) return SessionObservedRowRelations.read(value);
throw new IllegalStateException("Unknown native result alternative");
}
}
public static final class SessionObservedRowAnnotated extends Values.View implements SessionObservedRow {
public SessionObservedRowAnnotated(Object value) { super(value); }
static SessionObservedRowAnnotated read(Object value) { return new SessionObservedRowAnnotated(value); }
public String kind() { Object value = required("kind"); return (String)value; }
public List<SessionAnnotation> value() { Object value = required("value"); return Values.list(value, item0 -> SessionAnnotation.read(item0)); }
}
public static final class SessionObservedRowFind extends Values.View implements SessionObservedRow {
public SessionObservedRowFind(Object value) { super(value); }
static SessionObservedRowFind read(Object value) { return new SessionObservedRowFind(value); }
public String kind() { Object value = required("kind"); return (String)value; }
public Presence<BigInteger> value() { required("value"); return presence("value", value -> Values.integer(value)); }
}
public static final class SessionObservedRowJudgment extends Values.View implements SessionObservedRow {
public SessionObservedRowJudgment(Object value) { super(value); }
static SessionObservedRowJudgment read(Object value) { return new SessionObservedRowJudgment(value); }
public String kind() { Object value = required("kind"); return (String)value; }
public SessionJudgment value() { Object value = required("value"); return SessionJudgment.read(value); }
}
public static final class SessionObservedRowRecognized extends Values.View implements SessionObservedRow {
public SessionObservedRowRecognized(Object value) { super(value); }
static SessionObservedRowRecognized read(Object value) { return new SessionObservedRowRecognized(value); }
public String kind() { Object value = required("kind"); return (String)value; }
public SessionRecognition value() { Object value = required("value"); return SessionRecognition.read(value); }
}
public static final class SessionObservedRowRelations extends Values.View implements SessionObservedRow {
public SessionObservedRowRelations(Object value) { super(value); }
static SessionObservedRowRelations read(Object value) { return new SessionObservedRowRelations(value); }
public String kind() { Object value = required("kind"); return (String)value; }
public List<SessionRelationEdge> value() { Object value = required("value"); return Values.list(value, item0 -> SessionRelationEdge.read(item0)); }
}
public sealed interface SessionPacket extends Values.Value permits SessionPacketDecideRow,SessionPacketChooseRow,SessionPacketTagRow,SessionPacketScoreRow,SessionPacketFilterRow,SessionPacketAnnotateRow,SessionPacketDecideAggregate,SessionPacketChooseAggregate,SessionPacketTagAggregate,SessionPacketScoreAggregate,SessionPacketFilterAggregate,SessionPacketRankAggregate,SessionPacketFindAggregate,SessionPacketAnnotateAggregate,SessionPacketRecognizeAggregate,SessionPacketRelateAggregate,SessionPacketObservation,SessionPacketTerminal {
static SessionPacket read(Object value) { Map<String,Object> object = Values.object(value);
if ("decide".equals(object.get("function")) && "row".equals(object.get("kind"))) return SessionPacketDecideRow.read(value);
if ("choose".equals(object.get("function")) && "row".equals(object.get("kind"))) return SessionPacketChooseRow.read(value);
if ("tag".equals(object.get("function")) && "row".equals(object.get("kind"))) return SessionPacketTagRow.read(value);
if ("score".equals(object.get("function")) && "row".equals(object.get("kind"))) return SessionPacketScoreRow.read(value);
if ("filter".equals(object.get("function")) && "row".equals(object.get("kind"))) return SessionPacketFilterRow.read(value);
if ("annotate".equals(object.get("function")) && "row".equals(object.get("kind"))) return SessionPacketAnnotateRow.read(value);
if ("decide".equals(object.get("function")) && "aggregate".equals(object.get("kind"))) return SessionPacketDecideAggregate.read(value);
if ("choose".equals(object.get("function")) && "aggregate".equals(object.get("kind"))) return SessionPacketChooseAggregate.read(value);
if ("tag".equals(object.get("function")) && "aggregate".equals(object.get("kind"))) return SessionPacketTagAggregate.read(value);
if ("score".equals(object.get("function")) && "aggregate".equals(object.get("kind"))) return SessionPacketScoreAggregate.read(value);
if ("filter".equals(object.get("function")) && "aggregate".equals(object.get("kind"))) return SessionPacketFilterAggregate.read(value);
if ("rank".equals(object.get("function")) && "aggregate".equals(object.get("kind"))) return SessionPacketRankAggregate.read(value);
if ("find".equals(object.get("function")) && "aggregate".equals(object.get("kind"))) return SessionPacketFindAggregate.read(value);
if ("annotate".equals(object.get("function")) && "aggregate".equals(object.get("kind"))) return SessionPacketAnnotateAggregate.read(value);
if ("recognize".equals(object.get("function")) && "aggregate".equals(object.get("kind"))) return SessionPacketRecognizeAggregate.read(value);
if ("relate".equals(object.get("function")) && "aggregate".equals(object.get("kind"))) return SessionPacketRelateAggregate.read(value);
if ("observation".equals(object.get("kind"))) return SessionPacketObservation.read(value);
if ("terminal".equals(object.get("kind"))) return SessionPacketTerminal.read(value);
throw new IllegalStateException("Unknown native result alternative");
}
}
public static final class SessionPacketAnnotateAggregate extends Values.View implements SessionPacket {
public SessionPacketAnnotateAggregate(Object value) { super(value); }
static SessionPacketAnnotateAggregate read(Object value) { return new SessionPacketAnnotateAggregate(value); }
public String function() { Object value = required("function"); return (String)value; }
public String kind() { Object value = required("kind"); return (String)value; }
public List<Annotation> value() { Object value = required("value"); return Values.list(value, item0 -> Annotation.read(item0)); }
}
public static final class SessionPacketAnnotateRow extends Values.View implements SessionPacket {
public SessionPacketAnnotateRow(Object value) { super(value); }
static SessionPacketAnnotateRow read(Object value) { return new SessionPacketAnnotateRow(value); }
public String function() { Object value = required("function"); return (String)value; }
public String kind() { Object value = required("kind"); return (String)value; }
public Annotation value() { Object value = required("value"); return Annotation.read(value); }
}
public static final class SessionPacketChooseAggregate extends Values.View implements SessionPacket {
public SessionPacketChooseAggregate(Object value) { super(value); }
static SessionPacketChooseAggregate read(Object value) { return new SessionPacketChooseAggregate(value); }
public String function() { Object value = required("function"); return (String)value; }
public String kind() { Object value = required("kind"); return (String)value; }
public List<AtomicNullableString> value() { Object value = required("value"); return Values.list(value, item0 -> AtomicNullableString.read(item0)); }
}
public static final class SessionPacketChooseRow extends Values.View implements SessionPacket {
public SessionPacketChooseRow(Object value) { super(value); }
static SessionPacketChooseRow read(Object value) { return new SessionPacketChooseRow(value); }
public String function() { Object value = required("function"); return (String)value; }
public String kind() { Object value = required("kind"); return (String)value; }
public AtomicNullableString value() { Object value = required("value"); return AtomicNullableString.read(value); }
}
public static final class SessionPacketDecideAggregate extends Values.View implements SessionPacket {
public SessionPacketDecideAggregate(Object value) { super(value); }
static SessionPacketDecideAggregate read(Object value) { return new SessionPacketDecideAggregate(value); }
public String function() { Object value = required("function"); return (String)value; }
public String kind() { Object value = required("kind"); return (String)value; }
public List<AtomicDecideValue> value() { Object value = required("value"); return Values.list(value, item0 -> AtomicDecideValue.read(item0)); }
}
public static final class SessionPacketDecideRow extends Values.View implements SessionPacket {
public SessionPacketDecideRow(Object value) { super(value); }
static SessionPacketDecideRow read(Object value) { return new SessionPacketDecideRow(value); }
public String function() { Object value = required("function"); return (String)value; }
public String kind() { Object value = required("kind"); return (String)value; }
public AtomicDecideValue value() { Object value = required("value"); return AtomicDecideValue.read(value); }
}
public static final class SessionPacketFilterAggregate extends Values.View implements SessionPacket {
public SessionPacketFilterAggregate(Object value) { super(value); }
static SessionPacketFilterAggregate read(Object value) { return new SessionPacketFilterAggregate(value); }
public String function() { Object value = required("function"); return (String)value; }
public String kind() { Object value = required("kind"); return (String)value; }
public List<AtomicBoolean> value() { Object value = required("value"); return Values.list(value, item0 -> AtomicBoolean.read(item0)); }
}
public static final class SessionPacketFilterRow extends Values.View implements SessionPacket {
public SessionPacketFilterRow(Object value) { super(value); }
static SessionPacketFilterRow read(Object value) { return new SessionPacketFilterRow(value); }
public String function() { Object value = required("function"); return (String)value; }
public String kind() { Object value = required("kind"); return (String)value; }
public AtomicBoolean value() { Object value = required("value"); return AtomicBoolean.read(value); }
}
public static final class SessionPacketFindAggregate extends Values.View implements SessionPacket {
public SessionPacketFindAggregate(Object value) { super(value); }
static SessionPacketFindAggregate read(Object value) { return new SessionPacketFindAggregate(value); }
public String function() { Object value = required("function"); return (String)value; }
public String kind() { Object value = required("kind"); return (String)value; }
public Find value() { Object value = required("value"); return Find.read(value); }
}
public static final class SessionPacketObservation extends Values.View implements SessionPacket {
public SessionPacketObservation(Object value) { super(value); }
static SessionPacketObservation read(Object value) { return new SessionPacketObservation(value); }
public RequestFunction function() { Object value = required("function"); return RequestFunction.read(value); }
public String kind() { Object value = required("kind"); return (String)value; }
public SessionObservation value() { Object value = required("value"); return SessionObservation.read(value); }
}
public static final class SessionPacketRankAggregate extends Values.View implements SessionPacket {
public SessionPacketRankAggregate(Object value) { super(value); }
static SessionPacketRankAggregate read(Object value) { return new SessionPacketRankAggregate(value); }
public String function() { Object value = required("function"); return (String)value; }
public String kind() { Object value = required("kind"); return (String)value; }
public List<AtomicNonZeroUsize> value() { Object value = required("value"); return Values.list(value, item0 -> AtomicNonZeroUsize.read(item0)); }
}
public static final class SessionPacketRecognizeAggregate extends Values.View implements SessionPacket {
public SessionPacketRecognizeAggregate(Object value) { super(value); }
static SessionPacketRecognizeAggregate read(Object value) { return new SessionPacketRecognizeAggregate(value); }
public String function() { Object value = required("function"); return (String)value; }
public String kind() { Object value = required("kind"); return (String)value; }
public List<Recognition> value() { Object value = required("value"); return Values.list(value, item0 -> Recognition.read(item0)); }
}
public static final class SessionPacketRelateAggregate extends Values.View implements SessionPacket {
public SessionPacketRelateAggregate(Object value) { super(value); }
static SessionPacketRelateAggregate read(Object value) { return new SessionPacketRelateAggregate(value); }
public String function() { Object value = required("function"); return (String)value; }
public String kind() { Object value = required("kind"); return (String)value; }
public Relation value() { Object value = required("value"); return Relation.read(value); }
}
public static final class SessionPacketScoreAggregate extends Values.View implements SessionPacket {
public SessionPacketScoreAggregate(Object value) { super(value); }
static SessionPacketScoreAggregate read(Object value) { return new SessionPacketScoreAggregate(value); }
public String function() { Object value = required("function"); return (String)value; }
public String kind() { Object value = required("kind"); return (String)value; }
public List<AtomicDouble> value() { Object value = required("value"); return Values.list(value, item0 -> AtomicDouble.read(item0)); }
}
public static final class SessionPacketScoreRow extends Values.View implements SessionPacket {
public SessionPacketScoreRow(Object value) { super(value); }
static SessionPacketScoreRow read(Object value) { return new SessionPacketScoreRow(value); }
public String function() { Object value = required("function"); return (String)value; }
public String kind() { Object value = required("kind"); return (String)value; }
public AtomicDouble value() { Object value = required("value"); return AtomicDouble.read(value); }
}
public static final class SessionPacketTagAggregate extends Values.View implements SessionPacket {
public SessionPacketTagAggregate(Object value) { super(value); }
static SessionPacketTagAggregate read(Object value) { return new SessionPacketTagAggregate(value); }
public String function() { Object value = required("function"); return (String)value; }
public String kind() { Object value = required("kind"); return (String)value; }
public List<AtomicArrayOfString> value() { Object value = required("value"); return Values.list(value, item0 -> AtomicArrayOfString.read(item0)); }
}
public static final class SessionPacketTagRow extends Values.View implements SessionPacket {
public SessionPacketTagRow(Object value) { super(value); }
static SessionPacketTagRow read(Object value) { return new SessionPacketTagRow(value); }
public String function() { Object value = required("function"); return (String)value; }
public String kind() { Object value = required("kind"); return (String)value; }
public AtomicArrayOfString value() { Object value = required("value"); return AtomicArrayOfString.read(value); }
}
public static final class SessionPacketTerminal extends Values.View implements SessionPacket {
public SessionPacketTerminal(Object value) { super(value); }
static SessionPacketTerminal read(Object value) { return new SessionPacketTerminal(value); }
public Presence<Facts> facts() { return presence("facts", value -> Facts.read(value)); }
public Presence<CallError> failure() { return presence("failure", value -> CallError.read(value)); }
public String kind() { Object value = required("kind"); return (String)value; }
}
public sealed interface SessionProbabilities extends Values.Value permits SessionProbabilitiesYesNo,SessionProbabilitiesNamed {
static SessionProbabilities read(Object value) { Map<String,Object> object = Values.object(value);
if ("yes_no".equals(object.get("kind"))) return SessionProbabilitiesYesNo.read(value);
if ("named".equals(object.get("kind"))) return SessionProbabilitiesNamed.read(value);
throw new IllegalStateException("Unknown native result alternative");
}
}
public static final class SessionProbabilitiesNamed extends Values.View implements SessionProbabilities {
public SessionProbabilitiesNamed(Object value) { super(value); }
static SessionProbabilitiesNamed read(Object value) { return new SessionProbabilitiesNamed(value); }
public String kind() { Object value = required("kind"); return (String)value; }
public List<SessionNamedProbability> value() { Object value = required("value"); return Values.list(value, item0 -> SessionNamedProbability.read(item0)); }
}
public static final class SessionProbabilitiesYesNo extends Values.View implements SessionProbabilities {
public SessionProbabilitiesYesNo(Object value) { super(value); }
static SessionProbabilitiesYesNo read(Object value) { return new SessionProbabilitiesYesNo(value); }
public String kind() { Object value = required("kind"); return (String)value; }
public BigDecimal value() { Object value = required("value"); return (BigDecimal)value; }
}
public static final class SessionQuestionDetail extends Values.View {
public SessionQuestionDetail(Object value) { super(value); }
static SessionQuestionDetail read(Object value) { return new SessionQuestionDetail(value); }
public Presence<AnswerId> answerId() { return presence("answer_id", value -> AnswerId.read(value)); }
public Boolean cached() { Object value = required("cached"); return (Boolean)value; }
public Presence<BigDecimal> confidence() { return presence("confidence", value -> (BigDecimal)value); }
public BigInteger failedQuestions() { Object value = required("failed_questions"); return Values.integer(value); }
public Presence<Failure> failure() { return presence("failure", value -> Failure.read(value)); }
public Presence<FailureId> failureId() { return presence("failure_id", value -> FailureId.read(value)); }
public Presence<Object> input() { return presence("input", value -> Values.freeze(value)); }
public Presence<PhysicalSource> inputSource() { return presence("input_source", value -> PhysicalSource.read(value)); }
public List<SessionInputSource> inputSources() { Object value = required("input_sources"); return Values.list(value, item0 -> SessionInputSource.read(item0)); }
public List<Object> inputs() { Object value = required("inputs"); return Values.list(value, item0 -> Values.freeze(item0)); }
public String model() { Object value = required("model"); return (String)value; }
public List<Observation> observations() { Object value = required("observations"); return Values.list(value, item0 -> Observation.read(item0)); }
public Presence<SessionProbabilities> probabilities() { return presence("probabilities", value -> SessionProbabilities.read(value)); }
public ReadableQuestion question() { Object value = required("question"); return ReadableQuestion.read(value); }
public String questionSha256() { Object value = required("question_sha256"); return (String)value; }
public List<QuestionSource> questionSources() { Object value = required("question_sources"); return Values.list(value, item0 -> QuestionSource.read(item0)); }
public Presence<String> rawPick() { return presence("raw_pick", value -> (String)value); }
public Presence<Usage> reportedUsage() { return presence("reported_usage", value -> Usage.read(value)); }
public List<String> requests() { Object value = required("requests"); return Values.list(value, item0 -> (String)item0); }
public BigInteger requestsSent() { Object value = required("requests_sent"); return Values.integer(value); }
public Presence<Threshold> threshold() { return presence("threshold", value -> Threshold.read(value)); }
public String url() { Object value = required("url"); return (String)value; }
public Presence<TokenUsage> usage() { return presence("usage", value -> TokenUsage.read(value)); }
public Presence<Value> value() { return presence("value", value -> Value.read(value)); }
}
public static final class SessionRecognition extends Values.View {
public SessionRecognition(Object value) { super(value); }
static SessionRecognition read(Object value) { return new SessionRecognition(value); }
public List<Entity> entities() { Object value = required("entities"); return Values.list(value, item0 -> Entity.read(item0)); }
public RecognitionMode mode() { Object value = required("mode"); return RecognitionMode.read(value); }
public Presence<List<BoundaryProposal>> proposals() { return presence("proposals", value -> Values.list(value, item0 -> BoundaryProposal.read(item0))); }
public Presence<List<RecognitionEdgeDocument>> relations() { return presence("relations", value -> Values.list(value, item0 -> RecognitionEdgeDocument.read(item0))); }
}
public static final class SessionRelationEdge extends Values.View {
public SessionRelationEdge(Object value) { super(value); }
static SessionRelationEdge read(Object value) { return new SessionRelationEdge(value); }
public Boolean either() { Object value = required("either"); return (Boolean)value; }
public BigDecimal probability() { Object value = required("probability"); return (BigDecimal)value; }
public String relation() { Object value = required("relation"); return (String)value; }
public EntityDocument source() { Object value = required("source"); return EntityDocument.read(value); }
public EntityDocument target() { Object value = required("target"); return EntityDocument.read(value); }
}
public static final class SourceRelationEndpoint extends Values.View {
public SourceRelationEndpoint(Object value) { super(value); }
static SourceRelationEndpoint read(Object value) { return new SourceRelationEndpoint(value); }
public Presence<String> file() { required("file"); return presence("file", value -> (String)value); }
public Presence<BigInteger> firstLine() { return presence("first_line", value -> Values.integer(value)); }
public String kind() { Object value = required("kind"); return (String)value; }
public Presence<BigInteger> lastLine() { return presence("last_line", value -> Values.integer(value)); }
public String name() { Object value = required("name"); return (String)value; }
public BigInteger ordinal() { Object value = required("ordinal"); return Values.integer(value); }
public Object record() { Object value = required("record"); return Values.freeze(value); }
}
public sealed interface Threshold extends Values.Value permits ThresholdNumber,ThresholdString {
static Threshold read(Object value) {
if (value instanceof BigDecimal) return ThresholdNumber.read(value);
if (value instanceof String) return ThresholdString.read(value);
throw new IllegalStateException("Unknown native value alternative");
}
}
public record ThresholdNumber(BigDecimal value) implements Threshold {
static ThresholdNumber read(Object value) { return new ThresholdNumber((BigDecimal)value); }
public Object json() { return Values.json(value); }
}
public record ThresholdString(String value) implements Threshold {
static ThresholdString read(Object value) { return new ThresholdString((String)value); }
public Object json() { return Values.json(value); }
}
public static final class TokenBand extends Values.View {
public TokenBand(Object value) { super(value); }
static TokenBand read(Object value) { return new TokenBand(value); }
public BigInteger lower() { Object value = required("lower"); return Values.integer(value); }
public BigInteger upper() { Object value = required("upper"); return Values.integer(value); }
}
public static final class TokenUsage extends Values.View {
public TokenUsage(Object value) { super(value); }
static TokenUsage read(Object value) { return new TokenUsage(value); }
public BigInteger inputTokens() { Object value = required("input_tokens"); return Values.integer(value); }
public BigInteger outputTokens() { Object value = required("output_tokens"); return Values.integer(value); }
}
public sealed interface Value extends Values.Value permits ValueBoolean,ValueNull,ValueString,ValueArray,ValueNumber {
static Value read(Object value) {
if (value instanceof Boolean) return ValueBoolean.read(value);
if (value == null) return new ValueNull(null);
if (value instanceof String) return ValueString.read(value);
if (value instanceof List<?>) return ValueArray.read(value);
if (value instanceof BigDecimal) return ValueNumber.read(value);
throw new IllegalStateException("Unknown native value alternative");
}
}
public record ValueBoolean(Boolean value) implements Value {
static ValueBoolean read(Object value) { return new ValueBoolean((Boolean)value); }
public Object json() { return Values.json(value); }
}
public record ValueNull(Object value) implements Value {
static ValueNull read(Object value) { return new ValueNull(Values.freeze(value)); }
public Object json() { return Values.json(value); }
}
public record ValueString(String value) implements Value {
static ValueString read(Object value) { return new ValueString((String)value); }
public Object json() { return Values.json(value); }
}
public record ValueArray(List<String> value) implements Value {
static ValueArray read(Object value) { return new ValueArray(Values.list(value, item0 -> (String)item0)); }
public Object json() { return Values.json(value); }
}
public record ValueNumber(BigDecimal value) implements Value {
static ValueNumber read(Object value) { return new ValueNumber((BigDecimal)value); }
public Object json() { return Values.json(value); }
}
}
