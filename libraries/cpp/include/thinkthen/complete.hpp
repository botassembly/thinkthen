#pragma once
#include "json.hpp"
#include <cstdint>
#include <optional>
#include <limits>
// Host value descriptors only. These are not a native ABI or result/2 decoder.
namespace tt::complete {
enum class Function { decide, choose, tag, score, filter, rank, find, annotate, recognize, relate };
enum class FailureKind { usage=1, backend, deadline, local, cancelled, defect };
enum class ContentKind { text, json };
enum class RuleKind { default_, null_, cut, band };
enum class Media { jpeg, png };
enum class SourceUnit { line, window, file, imageFile };
enum class ValueKind { null_, boolean, authored };
enum class AtomicKind { yesNo, choice, tag, score, find };
enum class MemberState { success, failure };
enum class MemberCause { missingAnswer, wrongKind, missingProbability, invalidProbability, invalidDistribution, unexpectedProbability };
enum class Origin { live, cache, replay, proxy, memory };
enum class AttemptOutcome { ok, status, transport };
enum class RelationMethod { yesNo, choice };
enum class Direction { sourceToTarget, either };
enum class Stage { boundary, kind, edge, relation };
enum class IdentityKind { observation, failure };
enum class StopCause { usage, local, noKey, transport, status, tooLarge, reply, backend, cancelled, defect, deadline };
enum class BatchKind { records, max };
enum class EventKind { question, row };
inline void validate_identity(const std::string& value) {
    if (value.size() != 64) throw std::invalid_argument("identity must be 64 lowercase hex bytes");
    for (char c : value) if (!((c >= '0' && c <= '9') || (c >= 'a' && c <= 'f'))) throw std::invalid_argument("invalid identity");
}
template<class Tag> class Identity {
    std::string value_;
public:
    explicit Identity(std::string value): value_(std::move(value)) { validate_identity(value_); }
    const std::string& value() const { return value_; }
};
struct CallIdTag {}; using CallId = Identity<CallIdTag>;
struct SdkRequestIdTag {}; using SdkRequestId = Identity<SdkRequestIdTag>;
struct ObservationIdTag {}; using ObservationId = Identity<ObservationIdTag>;
struct FailureIdTag {}; using FailureId = Identity<FailureIdTag>;
struct AnswerIdTag {}; using AnswerId = Identity<AnswerIdTag>;
struct DigestTag {}; using Digest = Identity<DigestTag>;
struct Question;
struct QuestionMember;
struct Content { ContentKind kind; std::string data; };
struct Rule { RuleKind kind; double low; double high; };
struct Choice { std::string name; std::optional<Content> description; std::optional<double> weight; };
struct Relation { std::string name; std::string source; std::string target; std::optional<std::string> reads; bool either; bool single; };
struct Question { Function kind; Content text; std::optional<Content> yes; std::optional<Content> no; std::vector<Choice> choices; Rule threshold; Rule relationThreshold; std::optional<std::string> model; std::optional<std::string> profile; std::optional<uint64_t> batch; bool batchMax; bool none; std::vector<std::string> on; std::vector<QuestionMember> members; std::vector<Choice> kinds; std::vector<Relation> relations; std::optional<std::string> namePointer; std::optional<std::string> kindPointer; };
struct QuestionMember { std::string name; Question question; };
struct ImageInput { Media media; std::vector<uint8_t> bytes; std::optional<std::string> filename; };
struct ImageView { Media media; std::vector<uint8_t> bytes; uint32_t width; uint32_t height; std::optional<std::string> filename; };
struct RecordInput { std::optional<Content> original; std::optional<Content> context; std::vector<Choice> options; std::vector<ImageInput> images; };
struct FileSource { std::vector<std::string> paths; SourceUnit unit; uint64_t window; };
struct CallControls { std::optional<Content> context; std::optional<uint64_t> batch; bool batchMax; bool attempts; };
struct Probability { std::string name; double value; };
struct DecideValue { ValueKind kind; bool boolean; std::optional<Content> authored; };
struct AtomicAnswer { AtomicKind kind; std::optional<double> probability; std::optional<std::string> pick; std::optional<std::string> level; std::vector<Probability> probabilities; std::optional<double> confidence; };
struct Location { std::optional<std::string> file; std::optional<uint64_t> firstLine; std::optional<uint64_t> lastLine; };
struct MemberValue { Function function; std::optional<DecideValue> decide; std::optional<std::string> choose; std::optional<std::vector<std::string>> tag; std::optional<double> score; };
struct MemberFailure { FailureId failureId; MemberCause cause; };
struct MemberSuccess { AnswerId answerId; MemberValue value; AtomicAnswer answer; Rule threshold; };
struct AnnotationMember { std::string name; Digest request; Question question; MemberState state; std::optional<MemberSuccess> success; std::optional<MemberFailure> failure; };
struct Entity { std::string text; uint64_t start; uint64_t end; uint64_t length; std::string kind; double strength; };
struct EntityEdge { std::string relation; Entity source; Entity target; double probability; bool either; };
struct Place { uint64_t start; uint64_t end; };
struct Piece { uint64_t start; uint64_t end; std::vector<Probability> tags; };
struct NameSpan { uint64_t start; uint64_t end; std::optional<std::vector<Probability>> kinds; std::optional<std::vector<Probability>> edges; };
struct PairSpan { std::string relation; Place source; Place target; double probability; };
struct RecognizeValue { std::vector<Entity> entities; std::optional<std::vector<EntityEdge>> relations; };
struct RecognizeAnswer { std::vector<Piece> pieces; std::vector<NameSpan> names; std::vector<PairSpan> pairs; };
struct Endpoint { std::string name; std::string kind; };
struct Edge { std::string relation; Endpoint source; Endpoint target; double probability; bool either; };
struct RelationSuccess { AnswerId answerId; double probability; bool accepted; };
struct RelationAnswer { std::string relation; std::string reads; RelationMethod method; Direction direction; Endpoint source; std::optional<Endpoint> target; Digest request; MemberState state; std::optional<RelationSuccess> success; std::optional<MemberFailure> failure; };
struct TokenUsage { uint64_t inputTokens; uint64_t outputTokens; };
struct QuestionSource { Origin origin; std::string answeredBy; };
struct ObservationIdentity { IdentityKind kind; std::optional<ObservationId> observationId; std::optional<FailureId> failureId; };
struct ProfileWarning { std::string tunedFor; std::string running; };
struct BatchSetting { BatchKind kind; uint64_t records; };
struct BatchWarning { BatchSetting tunedFor; BatchSetting running; };
struct Attempt { uint64_t ordinal; Digest requestSha256; uint64_t wallMs; AttemptOutcome outcome; SdkRequestId sdkRequestId; std::optional<uint16_t> status; std::optional<uint64_t> serverMs; std::optional<std::string> requestId; };
struct Meta { std::string tool; std::optional<Digest> questionSha256; std::optional<Digest> questionsSha256; std::string url; std::string model; std::optional<TokenUsage> usage; uint64_t requestsSent; bool cached; std::vector<Digest> requests; uint64_t failedQuestions; std::optional<ProfileWarning> profileWarning; std::optional<BatchSetting> batchSetting; std::optional<BatchWarning> batchWarning; std::optional<Digest> contextSha256; std::optional<std::vector<Attempt>> attempts; std::optional<Origin> origin; std::vector<QuestionSource> questionSources; std::vector<ObservationIdentity> observations; std::optional<std::string> answeredBy; };
struct CallFacts { CallId callId; uint64_t cacheAnswers; std::optional<std::string> estimatedCostUsd; std::optional<uint64_t> inputTokens; std::optional<std::string> model; std::optional<uint64_t> outputTokens; uint64_t records; uint64_t requestsSent; double seconds; std::optional<uint64_t> commandMs; };
struct Stopped { std::optional<uint64_t> at; StopCause cause; std::optional<uint16_t> status; bool retryable; };
struct CompleteError { FailureKind code; std::string message; bool retryable; std::optional<Stopped> stopped; std::optional<CallFacts> facts; std::optional<std::vector<Attempt>> attempts; };
struct CommonRow { AnswerId answerId; std::optional<Content> input; std::optional<Question> question; std::optional<AtomicAnswer> answer; std::optional<Rule> threshold; std::optional<Location> position; std::optional<std::string> inputFile; Meta meta; std::optional<std::vector<ImageView>> images; };
struct DecideRow { CommonRow common; DecideValue value; };
struct ChooseRow { CommonRow common; std::optional<std::string> value; };
struct TagRow { CommonRow common; std::vector<std::string> value; };
struct ScoreRow { CommonRow common; double value; };
struct FilterRow { CommonRow common; bool value; };
struct RankRow { CommonRow common; std::optional<uint64_t> value; std::optional<std::string> questionName; };
struct FindRow { CommonRow common; std::optional<Content> value; std::optional<uint64_t> index; };
struct AnnotateRow { CommonRow common; std::vector<AnnotationMember> answers; };
struct RecognizeRow { CommonRow common; RecognizeValue value; RecognizeAnswer answer; };
struct RelateRow { CommonRow common; std::vector<Edge> value; std::vector<RelationAnswer> questions; };
struct RowValue { Function function; std::optional<DecideRow> decide; std::optional<ChooseRow> choose; std::optional<TagRow> tag; std::optional<ScoreRow> score; std::optional<FilterRow> filter; std::optional<RankRow> rank; std::optional<FindRow> find; std::optional<AnnotateRow> annotate; std::optional<RecognizeRow> recognize; std::optional<RelateRow> relate; };
struct ObservedProbabilities { std::optional<double> yes; std::optional<std::vector<Probability>> named; };
struct ObservationSuccess { AnswerId answerId; ObservationId observationId; MemberValue value; ObservedProbabilities probabilities; std::optional<double> confidence; };
struct QuestionObservation { uint64_t index; std::optional<std::string> member; std::optional<Stage> stage; uint64_t position; Digest questionSha256; std::string model; std::string url; std::vector<Digest> requests; uint64_t requestsSent; bool cached; uint64_t failedQuestions; std::optional<TokenUsage> usage; std::vector<QuestionSource> questionSources; MemberState state; std::optional<ObservationSuccess> success; std::optional<MemberFailure> failure; };
struct RowObservation { uint64_t index; RowValue value; };
struct ObservationEvent { EventKind kind; std::optional<QuestionObservation> question; std::optional<RowObservation> row; };
struct QuestionInput { std::optional<Question> question; std::optional<std::string> file;
    static QuestionInput asked(Question value) { return {std::move(value), {}}; }
    static QuestionInput question_file(std::string path) { return {{}, std::move(path)}; }
};
struct InputSource { std::optional<std::vector<RecordInput>> records; std::optional<FileSource> files;
    static InputSource from_records(std::vector<RecordInput> values) { return {std::move(values), {}}; }
    static InputSource from_files(FileSource value) { return {{}, std::move(value)}; }
};
struct CompleteRequest { Function function; QuestionInput question; InputSource source; CallControls controls; };
struct CompleteSummary { uint64_t count; uint64_t observationCount; std::string schema; AnswerId answerId; Function function; Meta meta; CallFacts facts; std::optional<std::vector<Attempt>> attempts; };
struct CompleteResult { CompleteSummary summary; std::vector<RowValue> rows; std::vector<ObservationEvent> observations; };
inline Content text(std::string value) { return {ContentKind::text, std::move(value)}; }
inline Content json(std::string value) { (void)Json::parse(value); return {ContentKind::json, std::move(value)}; }
inline Question asked(Function kind, Content content) {
    return {kind, std::move(content), {}, {}, {}, {RuleKind::default_,0,0}, {RuleKind::default_,0,0}, {}, {}, {}, false, false, {}, {}, {}, {}, {}, {}};
}
struct Requests {
    static CompleteRequest decide(QuestionInput question, InputSource source, CallControls controls = {{},{},false,false}) { return {Function::decide, std::move(question), std::move(source), std::move(controls)}; }
    static CompleteRequest choose(QuestionInput question, InputSource source, CallControls controls = {{},{},false,false}) { return {Function::choose, std::move(question), std::move(source), std::move(controls)}; }
    static CompleteRequest tag(QuestionInput question, InputSource source, CallControls controls = {{},{},false,false}) { return {Function::tag, std::move(question), std::move(source), std::move(controls)}; }
    static CompleteRequest score(QuestionInput question, InputSource source, CallControls controls = {{},{},false,false}) { return {Function::score, std::move(question), std::move(source), std::move(controls)}; }
    static CompleteRequest filter(QuestionInput question, InputSource source, CallControls controls = {{},{},false,false}) { return {Function::filter, std::move(question), std::move(source), std::move(controls)}; }
    static CompleteRequest rank(QuestionInput question, InputSource source, CallControls controls = {{},{},false,false}) { return {Function::rank, std::move(question), std::move(source), std::move(controls)}; }
    static CompleteRequest find(QuestionInput question, InputSource source, CallControls controls = {{},{},false,false}) { return {Function::find, std::move(question), std::move(source), std::move(controls)}; }
    static CompleteRequest annotate(QuestionInput question, InputSource source, CallControls controls = {{},{},false,false}) { return {Function::annotate, std::move(question), std::move(source), std::move(controls)}; }
    static CompleteRequest recognize(QuestionInput question, InputSource source, CallControls controls = {{},{},false,false}) { return {Function::recognize, std::move(question), std::move(source), std::move(controls)}; }
    static CompleteRequest relate(QuestionInput question, InputSource source, CallControls controls = {{},{},false,false}) { return {Function::relate, std::move(question), std::move(source), std::move(controls)}; }
};
// Lossless host descriptor serialization, not the native request wire format.
inline Json descriptor(const std::string& value) { return Json(value); }
inline Json descriptor(bool value) { return Json(value); }
inline Json descriptor(double value) { return Json(value); }
inline Json descriptor(uint64_t value) {
    if (value > UINT64_C(9007199254740991)) throw std::invalid_argument("integer exceeds JSON host exact range");
    return Json(static_cast<double>(value));
}
inline Json descriptor(uint16_t value) { return Json(static_cast<int>(value)); }
inline Json descriptor(uint32_t value) { return Json(static_cast<double>(value)); }
inline Json descriptor(uint8_t value) { return Json(static_cast<int>(value)); }
template<class T> Json descriptor(const Identity<T>& value) { return Json(value.value()); }
inline Json descriptor(Function value) { switch(value) { case Function::decide: return Json("Decide"); case Function::choose: return Json("Choose"); case Function::tag: return Json("Tag"); case Function::score: return Json("Score"); case Function::filter: return Json("Filter"); case Function::rank: return Json("Rank"); case Function::find: return Json("Find"); case Function::annotate: return Json("Annotate"); case Function::recognize: return Json("Recognize"); case Function::relate: return Json("Relate"); } throw std::invalid_argument("unknown descriptor discriminator"); }
inline Json descriptor(FailureKind value) { return Json(static_cast<int>(value)); }
inline Json descriptor(ContentKind value) { switch(value) { case ContentKind::text: return Json("Text"); case ContentKind::json: return Json("Json"); } throw std::invalid_argument("unknown descriptor discriminator"); }
inline Json descriptor(RuleKind value) { switch(value) { case RuleKind::default_: return Json("Default"); case RuleKind::null_: return Json("Null"); case RuleKind::cut: return Json("Cut"); case RuleKind::band: return Json("Band"); } throw std::invalid_argument("unknown descriptor discriminator"); }
inline Json descriptor(Media value) { switch(value) { case Media::jpeg: return Json("Jpeg"); case Media::png: return Json("Png"); } throw std::invalid_argument("unknown descriptor discriminator"); }
inline Json descriptor(SourceUnit value) { switch(value) { case SourceUnit::line: return Json("Line"); case SourceUnit::window: return Json("Window"); case SourceUnit::file: return Json("File"); case SourceUnit::imageFile: return Json("ImageFile"); } throw std::invalid_argument("unknown descriptor discriminator"); }
inline Json descriptor(ValueKind value) { switch(value) { case ValueKind::null_: return Json("Null"); case ValueKind::boolean: return Json("Boolean"); case ValueKind::authored: return Json("Authored"); } throw std::invalid_argument("unknown descriptor discriminator"); }
inline Json descriptor(AtomicKind value) { switch(value) { case AtomicKind::yesNo: return Json("YesNo"); case AtomicKind::choice: return Json("Choice"); case AtomicKind::tag: return Json("Tag"); case AtomicKind::score: return Json("Score"); case AtomicKind::find: return Json("Find"); } throw std::invalid_argument("unknown descriptor discriminator"); }
inline Json descriptor(MemberState value) { switch(value) { case MemberState::success: return Json("Success"); case MemberState::failure: return Json("Failure"); } throw std::invalid_argument("unknown descriptor discriminator"); }
inline Json descriptor(MemberCause value) { switch(value) { case MemberCause::missingAnswer: return Json("MissingAnswer"); case MemberCause::wrongKind: return Json("WrongKind"); case MemberCause::missingProbability: return Json("MissingProbability"); case MemberCause::invalidProbability: return Json("InvalidProbability"); case MemberCause::invalidDistribution: return Json("InvalidDistribution"); case MemberCause::unexpectedProbability: return Json("UnexpectedProbability"); } throw std::invalid_argument("unknown descriptor discriminator"); }
inline Json descriptor(Origin value) { switch(value) { case Origin::live: return Json("Live"); case Origin::cache: return Json("Cache"); case Origin::replay: return Json("Replay"); case Origin::proxy: return Json("Proxy"); case Origin::memory: return Json("Memory"); } throw std::invalid_argument("unknown descriptor discriminator"); }
inline Json descriptor(AttemptOutcome value) { switch(value) { case AttemptOutcome::ok: return Json("Ok"); case AttemptOutcome::status: return Json("Status"); case AttemptOutcome::transport: return Json("Transport"); } throw std::invalid_argument("unknown descriptor discriminator"); }
inline Json descriptor(RelationMethod value) { switch(value) { case RelationMethod::yesNo: return Json("YesNo"); case RelationMethod::choice: return Json("Choice"); } throw std::invalid_argument("unknown descriptor discriminator"); }
inline Json descriptor(Direction value) { switch(value) { case Direction::sourceToTarget: return Json("SourceToTarget"); case Direction::either: return Json("Either"); } throw std::invalid_argument("unknown descriptor discriminator"); }
inline Json descriptor(Stage value) { switch(value) { case Stage::boundary: return Json("Boundary"); case Stage::kind: return Json("Kind"); case Stage::edge: return Json("Edge"); case Stage::relation: return Json("Relation"); } throw std::invalid_argument("unknown descriptor discriminator"); }
inline Json descriptor(IdentityKind value) { switch(value) { case IdentityKind::observation: return Json("Observation"); case IdentityKind::failure: return Json("Failure"); } throw std::invalid_argument("unknown descriptor discriminator"); }
inline Json descriptor(StopCause value) { switch(value) { case StopCause::usage: return Json("Usage"); case StopCause::local: return Json("Local"); case StopCause::noKey: return Json("NoKey"); case StopCause::transport: return Json("Transport"); case StopCause::status: return Json("Status"); case StopCause::tooLarge: return Json("TooLarge"); case StopCause::reply: return Json("Reply"); case StopCause::backend: return Json("Backend"); case StopCause::cancelled: return Json("Cancelled"); case StopCause::defect: return Json("Defect"); case StopCause::deadline: return Json("Deadline"); } throw std::invalid_argument("unknown descriptor discriminator"); }
inline Json descriptor(BatchKind value) { switch(value) { case BatchKind::records: return Json("Records"); case BatchKind::max: return Json("Max"); } throw std::invalid_argument("unknown descriptor discriminator"); }
inline Json descriptor(EventKind value) { switch(value) { case EventKind::question: return Json("Question"); case EventKind::row: return Json("Row"); } throw std::invalid_argument("unknown descriptor discriminator"); }
inline Json descriptor(const Content& value);
inline Json descriptor(const Rule& value);
inline Json descriptor(const Choice& value);
inline Json descriptor(const Relation& value);
inline Json descriptor(const QuestionMember& value);
inline Json descriptor(const Question& value);
inline Json descriptor(const ImageInput& value);
inline Json descriptor(const ImageView& value);
inline Json descriptor(const RecordInput& value);
inline Json descriptor(const FileSource& value);
inline Json descriptor(const CallControls& value);
inline Json descriptor(const Probability& value);
inline Json descriptor(const DecideValue& value);
inline Json descriptor(const AtomicAnswer& value);
inline Json descriptor(const Location& value);
inline Json descriptor(const MemberValue& value);
inline Json descriptor(const MemberFailure& value);
inline Json descriptor(const MemberSuccess& value);
inline Json descriptor(const AnnotationMember& value);
inline Json descriptor(const Entity& value);
inline Json descriptor(const EntityEdge& value);
inline Json descriptor(const Place& value);
inline Json descriptor(const Piece& value);
inline Json descriptor(const NameSpan& value);
inline Json descriptor(const PairSpan& value);
inline Json descriptor(const RecognizeValue& value);
inline Json descriptor(const RecognizeAnswer& value);
inline Json descriptor(const Endpoint& value);
inline Json descriptor(const Edge& value);
inline Json descriptor(const RelationSuccess& value);
inline Json descriptor(const RelationAnswer& value);
inline Json descriptor(const TokenUsage& value);
inline Json descriptor(const QuestionSource& value);
inline Json descriptor(const ObservationIdentity& value);
inline Json descriptor(const ProfileWarning& value);
inline Json descriptor(const BatchSetting& value);
inline Json descriptor(const BatchWarning& value);
inline Json descriptor(const Attempt& value);
inline Json descriptor(const Meta& value);
inline Json descriptor(const CallFacts& value);
inline Json descriptor(const Stopped& value);
inline Json descriptor(const CompleteError& value);
inline Json descriptor(const CommonRow& value);
inline Json descriptor(const DecideRow& value);
inline Json descriptor(const ChooseRow& value);
inline Json descriptor(const TagRow& value);
inline Json descriptor(const ScoreRow& value);
inline Json descriptor(const FilterRow& value);
inline Json descriptor(const RankRow& value);
inline Json descriptor(const FindRow& value);
inline Json descriptor(const AnnotateRow& value);
inline Json descriptor(const RecognizeRow& value);
inline Json descriptor(const RelateRow& value);
inline Json descriptor(const RowValue& value);
inline Json descriptor(const ObservedProbabilities& value);
inline Json descriptor(const ObservationSuccess& value);
inline Json descriptor(const QuestionObservation& value);
inline Json descriptor(const RowObservation& value);
inline Json descriptor(const ObservationEvent& value);
inline Json descriptor(const QuestionInput& value);
inline Json descriptor(const InputSource& value);
inline Json descriptor(const CompleteRequest& value);
inline Json descriptor(const CompleteSummary& value);
template<class T> Json descriptor(const std::vector<T>& values);
template<class T> Json descriptor(const std::optional<T>& value) { return value ? descriptor(*value) : Json(nullptr); }
template<class T> Json descriptor(const std::vector<T>& values) { Json::Array rows; for (const auto& value : values) rows.push_back(descriptor(value)); return Json(std::move(rows)); }
inline Json descriptor(const Content& value) { return Json{{"kind", descriptor(value.kind)}, {"data", descriptor(value.data)}}; }
inline Json descriptor(const Rule& value) { return Json{{"kind", descriptor(value.kind)}, {"low", descriptor(value.low)}, {"high", descriptor(value.high)}}; }
inline Json descriptor(const Choice& value) { return Json{{"name", descriptor(value.name)}, {"description", descriptor(value.description)}, {"weight", descriptor(value.weight)}}; }
inline Json descriptor(const Relation& value) { return Json{{"name", descriptor(value.name)}, {"source", descriptor(value.source)}, {"target", descriptor(value.target)}, {"reads", descriptor(value.reads)}, {"either", descriptor(value.either)}, {"single", descriptor(value.single)}}; }
inline Json descriptor(const QuestionMember& value) { return Json{{"name", descriptor(value.name)}, {"question", descriptor(value.question)}}; }
inline Json descriptor(const Question& value) { return Json{{"kind", descriptor(value.kind)}, {"text", descriptor(value.text)}, {"yes", descriptor(value.yes)}, {"no", descriptor(value.no)}, {"choices", descriptor(value.choices)}, {"threshold", descriptor(value.threshold)}, {"relationThreshold", descriptor(value.relationThreshold)}, {"model", descriptor(value.model)}, {"profile", descriptor(value.profile)}, {"batch", descriptor(value.batch)}, {"batchMax", descriptor(value.batchMax)}, {"none", descriptor(value.none)}, {"on", descriptor(value.on)}, {"members", descriptor(value.members)}, {"kinds", descriptor(value.kinds)}, {"relations", descriptor(value.relations)}, {"namePointer", descriptor(value.namePointer)}, {"kindPointer", descriptor(value.kindPointer)}}; }
inline Json descriptor(const ImageInput& value) { return Json{{"media", descriptor(value.media)}, {"bytes", descriptor(value.bytes)}, {"filename", descriptor(value.filename)}}; }
inline Json descriptor(const ImageView& value) { return Json{{"media", descriptor(value.media)}, {"bytes", descriptor(value.bytes)}, {"width", descriptor(value.width)}, {"height", descriptor(value.height)}, {"filename", descriptor(value.filename)}}; }
inline Json descriptor(const RecordInput& value) { return Json{{"original", descriptor(value.original)}, {"context", descriptor(value.context)}, {"options", descriptor(value.options)}, {"images", descriptor(value.images)}}; }
inline Json descriptor(const FileSource& value) { return Json{{"paths", descriptor(value.paths)}, {"unit", descriptor(value.unit)}, {"window", descriptor(value.window)}}; }
inline Json descriptor(const CallControls& value) { return Json{{"context", descriptor(value.context)}, {"batch", descriptor(value.batch)}, {"batchMax", descriptor(value.batchMax)}, {"attempts", descriptor(value.attempts)}}; }
inline Json descriptor(const Probability& value) { return Json{{"name", descriptor(value.name)}, {"value", descriptor(value.value)}}; }
inline Json descriptor(const DecideValue& value) { return Json{{"kind", descriptor(value.kind)}, {"boolean", descriptor(value.boolean)}, {"authored", descriptor(value.authored)}}; }
inline Json descriptor(const AtomicAnswer& value) { return Json{{"kind", descriptor(value.kind)}, {"probability", descriptor(value.probability)}, {"pick", descriptor(value.pick)}, {"level", descriptor(value.level)}, {"probabilities", descriptor(value.probabilities)}, {"confidence", descriptor(value.confidence)}}; }
inline Json descriptor(const Location& value) { return Json{{"file", descriptor(value.file)}, {"firstLine", descriptor(value.firstLine)}, {"lastLine", descriptor(value.lastLine)}}; }
inline Json descriptor(const MemberValue& value) { return Json{{"function", descriptor(value.function)}, {"decide", descriptor(value.decide)}, {"choose", descriptor(value.choose)}, {"tag", descriptor(value.tag)}, {"score", descriptor(value.score)}}; }
inline Json descriptor(const MemberFailure& value) { return Json{{"failureId", descriptor(value.failureId)}, {"cause", descriptor(value.cause)}}; }
inline Json descriptor(const MemberSuccess& value) { return Json{{"answerId", descriptor(value.answerId)}, {"value", descriptor(value.value)}, {"answer", descriptor(value.answer)}, {"threshold", descriptor(value.threshold)}}; }
inline Json descriptor(const AnnotationMember& value) { return Json{{"name", descriptor(value.name)}, {"request", descriptor(value.request)}, {"question", descriptor(value.question)}, {"state", descriptor(value.state)}, {"success", descriptor(value.success)}, {"failure", descriptor(value.failure)}}; }
inline Json descriptor(const Entity& value) { return Json{{"text", descriptor(value.text)}, {"start", descriptor(value.start)}, {"end", descriptor(value.end)}, {"length", descriptor(value.length)}, {"kind", descriptor(value.kind)}, {"strength", descriptor(value.strength)}}; }
inline Json descriptor(const EntityEdge& value) { return Json{{"relation", descriptor(value.relation)}, {"source", descriptor(value.source)}, {"target", descriptor(value.target)}, {"probability", descriptor(value.probability)}, {"either", descriptor(value.either)}}; }
inline Json descriptor(const Place& value) { return Json{{"start", descriptor(value.start)}, {"end", descriptor(value.end)}}; }
inline Json descriptor(const Piece& value) { return Json{{"start", descriptor(value.start)}, {"end", descriptor(value.end)}, {"tags", descriptor(value.tags)}}; }
inline Json descriptor(const NameSpan& value) { return Json{{"start", descriptor(value.start)}, {"end", descriptor(value.end)}, {"kinds", descriptor(value.kinds)}, {"edges", descriptor(value.edges)}}; }
inline Json descriptor(const PairSpan& value) { return Json{{"relation", descriptor(value.relation)}, {"source", descriptor(value.source)}, {"target", descriptor(value.target)}, {"probability", descriptor(value.probability)}}; }
inline Json descriptor(const RecognizeValue& value) { return Json{{"entities", descriptor(value.entities)}, {"relations", descriptor(value.relations)}}; }
inline Json descriptor(const RecognizeAnswer& value) { return Json{{"pieces", descriptor(value.pieces)}, {"names", descriptor(value.names)}, {"pairs", descriptor(value.pairs)}}; }
inline Json descriptor(const Endpoint& value) { return Json{{"name", descriptor(value.name)}, {"kind", descriptor(value.kind)}}; }
inline Json descriptor(const Edge& value) { return Json{{"relation", descriptor(value.relation)}, {"source", descriptor(value.source)}, {"target", descriptor(value.target)}, {"probability", descriptor(value.probability)}, {"either", descriptor(value.either)}}; }
inline Json descriptor(const RelationSuccess& value) { return Json{{"answerId", descriptor(value.answerId)}, {"probability", descriptor(value.probability)}, {"accepted", descriptor(value.accepted)}}; }
inline Json descriptor(const RelationAnswer& value) { return Json{{"relation", descriptor(value.relation)}, {"reads", descriptor(value.reads)}, {"method", descriptor(value.method)}, {"direction", descriptor(value.direction)}, {"source", descriptor(value.source)}, {"target", descriptor(value.target)}, {"request", descriptor(value.request)}, {"state", descriptor(value.state)}, {"success", descriptor(value.success)}, {"failure", descriptor(value.failure)}}; }
inline Json descriptor(const TokenUsage& value) { return Json{{"inputTokens", descriptor(value.inputTokens)}, {"outputTokens", descriptor(value.outputTokens)}}; }
inline Json descriptor(const QuestionSource& value) { return Json{{"origin", descriptor(value.origin)}, {"answeredBy", descriptor(value.answeredBy)}}; }
inline Json descriptor(const ObservationIdentity& value) { return Json{{"kind", descriptor(value.kind)}, {"observationId", descriptor(value.observationId)}, {"failureId", descriptor(value.failureId)}}; }
inline Json descriptor(const ProfileWarning& value) { return Json{{"tunedFor", descriptor(value.tunedFor)}, {"running", descriptor(value.running)}}; }
inline Json descriptor(const BatchSetting& value) { return Json{{"kind", descriptor(value.kind)}, {"records", descriptor(value.records)}}; }
inline Json descriptor(const BatchWarning& value) { return Json{{"tunedFor", descriptor(value.tunedFor)}, {"running", descriptor(value.running)}}; }
inline Json descriptor(const Attempt& value) { return Json{{"ordinal", descriptor(value.ordinal)}, {"requestSha256", descriptor(value.requestSha256)}, {"wallMs", descriptor(value.wallMs)}, {"outcome", descriptor(value.outcome)}, {"sdkRequestId", descriptor(value.sdkRequestId)}, {"status", descriptor(value.status)}, {"serverMs", descriptor(value.serverMs)}, {"requestId", descriptor(value.requestId)}}; }
inline Json descriptor(const Meta& value) { return Json{{"tool", descriptor(value.tool)}, {"questionSha256", descriptor(value.questionSha256)}, {"questionsSha256", descriptor(value.questionsSha256)}, {"url", descriptor(value.url)}, {"model", descriptor(value.model)}, {"usage", descriptor(value.usage)}, {"requestsSent", descriptor(value.requestsSent)}, {"cached", descriptor(value.cached)}, {"requests", descriptor(value.requests)}, {"failedQuestions", descriptor(value.failedQuestions)}, {"profileWarning", descriptor(value.profileWarning)}, {"batchSetting", descriptor(value.batchSetting)}, {"batchWarning", descriptor(value.batchWarning)}, {"contextSha256", descriptor(value.contextSha256)}, {"attempts", descriptor(value.attempts)}, {"origin", descriptor(value.origin)}, {"questionSources", descriptor(value.questionSources)}, {"observations", descriptor(value.observations)}, {"answeredBy", descriptor(value.answeredBy)}}; }
inline Json descriptor(const CallFacts& value) { return Json{{"callId", descriptor(value.callId)}, {"cacheAnswers", descriptor(value.cacheAnswers)}, {"estimatedCostUsd", descriptor(value.estimatedCostUsd)}, {"inputTokens", descriptor(value.inputTokens)}, {"model", descriptor(value.model)}, {"outputTokens", descriptor(value.outputTokens)}, {"records", descriptor(value.records)}, {"requestsSent", descriptor(value.requestsSent)}, {"seconds", descriptor(value.seconds)}, {"commandMs", descriptor(value.commandMs)}}; }
inline Json descriptor(const Stopped& value) { return Json{{"at", descriptor(value.at)}, {"cause", descriptor(value.cause)}, {"status", descriptor(value.status)}, {"retryable", descriptor(value.retryable)}}; }
inline Json descriptor(const CompleteError& value) { return Json{{"code", descriptor(value.code)}, {"message", descriptor(value.message)}, {"retryable", descriptor(value.retryable)}, {"stopped", descriptor(value.stopped)}, {"facts", descriptor(value.facts)}, {"attempts", descriptor(value.attempts)}}; }
inline Json descriptor(const CommonRow& value) { return Json{{"answerId", descriptor(value.answerId)}, {"input", descriptor(value.input)}, {"question", descriptor(value.question)}, {"answer", descriptor(value.answer)}, {"threshold", descriptor(value.threshold)}, {"position", descriptor(value.position)}, {"inputFile", descriptor(value.inputFile)}, {"meta", descriptor(value.meta)}, {"images", descriptor(value.images)}}; }
inline Json descriptor(const DecideRow& value) { return Json{{"common", descriptor(value.common)}, {"value", descriptor(value.value)}}; }
inline Json descriptor(const ChooseRow& value) { return Json{{"common", descriptor(value.common)}, {"value", descriptor(value.value)}}; }
inline Json descriptor(const TagRow& value) { return Json{{"common", descriptor(value.common)}, {"value", descriptor(value.value)}}; }
inline Json descriptor(const ScoreRow& value) { return Json{{"common", descriptor(value.common)}, {"value", descriptor(value.value)}}; }
inline Json descriptor(const FilterRow& value) { return Json{{"common", descriptor(value.common)}, {"value", descriptor(value.value)}}; }
inline Json descriptor(const RankRow& value) { return Json{{"common", descriptor(value.common)}, {"value", descriptor(value.value)}, {"questionName", descriptor(value.questionName)}}; }
inline Json descriptor(const FindRow& value) { return Json{{"common", descriptor(value.common)}, {"value", descriptor(value.value)}, {"index", descriptor(value.index)}}; }
inline Json descriptor(const AnnotateRow& value) { return Json{{"common", descriptor(value.common)}, {"answers", descriptor(value.answers)}}; }
inline Json descriptor(const RecognizeRow& value) { return Json{{"common", descriptor(value.common)}, {"value", descriptor(value.value)}, {"answer", descriptor(value.answer)}}; }
inline Json descriptor(const RelateRow& value) { return Json{{"common", descriptor(value.common)}, {"value", descriptor(value.value)}, {"questions", descriptor(value.questions)}}; }
inline Json descriptor(const RowValue& value) { return Json{{"function", descriptor(value.function)}, {"decide", descriptor(value.decide)}, {"choose", descriptor(value.choose)}, {"tag", descriptor(value.tag)}, {"score", descriptor(value.score)}, {"filter", descriptor(value.filter)}, {"rank", descriptor(value.rank)}, {"find", descriptor(value.find)}, {"annotate", descriptor(value.annotate)}, {"recognize", descriptor(value.recognize)}, {"relate", descriptor(value.relate)}}; }
inline Json descriptor(const ObservedProbabilities& value) { return Json{{"yes", descriptor(value.yes)}, {"named", descriptor(value.named)}}; }
inline Json descriptor(const ObservationSuccess& value) { return Json{{"answerId", descriptor(value.answerId)}, {"observationId", descriptor(value.observationId)}, {"value", descriptor(value.value)}, {"probabilities", descriptor(value.probabilities)}, {"confidence", descriptor(value.confidence)}}; }
inline Json descriptor(const QuestionObservation& value) { return Json{{"index", descriptor(value.index)}, {"member", descriptor(value.member)}, {"stage", descriptor(value.stage)}, {"position", descriptor(value.position)}, {"questionSha256", descriptor(value.questionSha256)}, {"model", descriptor(value.model)}, {"url", descriptor(value.url)}, {"requests", descriptor(value.requests)}, {"requestsSent", descriptor(value.requestsSent)}, {"cached", descriptor(value.cached)}, {"failedQuestions", descriptor(value.failedQuestions)}, {"usage", descriptor(value.usage)}, {"questionSources", descriptor(value.questionSources)}, {"state", descriptor(value.state)}, {"success", descriptor(value.success)}, {"failure", descriptor(value.failure)}}; }
inline Json descriptor(const RowObservation& value) { return Json{{"index", descriptor(value.index)}, {"value", descriptor(value.value)}}; }
inline Json descriptor(const ObservationEvent& value) { return Json{{"kind", descriptor(value.kind)}, {"question", descriptor(value.question)}, {"row", descriptor(value.row)}}; }
inline Json descriptor(const QuestionInput& value) { return Json{{"question", descriptor(value.question)}, {"file", descriptor(value.file)}}; }
inline Json descriptor(const InputSource& value) { return Json{{"records", descriptor(value.records)}, {"files", descriptor(value.files)}}; }
inline Json descriptor(const CompleteRequest& value) { return Json{{"function", descriptor(value.function)}, {"question", descriptor(value.question)}, {"source", descriptor(value.source)}, {"controls", descriptor(value.controls)}}; }
inline Json descriptor(const CompleteSummary& value) { return Json{{"count", descriptor(value.count)}, {"observationCount", descriptor(value.observationCount)}, {"schema", descriptor(value.schema)}, {"answerId", descriptor(value.answerId)}, {"function", descriptor(value.function)}, {"meta", descriptor(value.meta)}, {"facts", descriptor(value.facts)}, {"attempts", descriptor(value.attempts)}}; }
inline Json descriptor(const CompleteResult& value) { return Json{{"summary",descriptor(value.summary)},{"rows",descriptor(value.rows)},{"observations",descriptor(value.observations)}}; }
// Serialized field readers reuse the existing host JSON parser. They do not
// accept/upgrade result/1 envelopes or infer IDs, provenance or answer models.
inline uint64_t read_count(const Json& value) {
    const auto number = value.get<double>();
    if (number < 0 || number > 9007199254740991.0 || std::floor(number) != number)
        throw std::invalid_argument("count exceeds exact host JSON range");
    return static_cast<uint64_t>(number);
}
inline double read_probability(const Json& value) {
    const auto number = value.get<double>();
    if (number < 0 || number > 1) throw std::invalid_argument("invalid probability");
    return number;
}
template<class T, class Reader> std::optional<T> read_optional(const Json& object, const std::string& key, Reader read) {
    if (!object.contains(key)) return {};
    return read(object.at(key));
}
inline CallFacts read_facts(const Json& value) {
    CallFacts facts{CallId(value.at("call_id").get<std::string>()), read_count(value.at("cache_answers")),
        read_optional<std::string>(value,"estimated_cost_usd",[](const Json& v){return v.get<std::string>();}),
        read_optional<uint64_t>(value,"input_tokens",read_count),
        read_optional<std::string>(value,"model",[](const Json& v){return v.get<std::string>();}),
        read_optional<uint64_t>(value,"output_tokens",read_count),read_count(value.at("records")),
        read_count(value.at("requests_sent")),value.at("seconds").get<double>(),
        read_optional<uint64_t>(value,"command_ms",read_count)};
    if (facts.seconds < 0) throw std::invalid_argument("negative elapsed time");
    if (facts.estimatedCostUsd) {
        const auto& cost = *facts.estimatedCostUsd;
        if (cost.size() < 8 || cost.at(cost.size()-7) != '.') throw std::invalid_argument("invalid exact cost");
        for (size_t i=0; i<cost.size(); ++i) if (i != cost.size()-7 && (cost.at(i) < '0' || cost.at(i) > '9'))
            throw std::invalid_argument("invalid exact cost");
    }
    return facts;
}
inline AtomicAnswer read_atomic(const Json& value) {
    const auto kind = value.at("kind").get<std::string>();
    if (kind == "yes_no") return {AtomicKind::yesNo,read_probability(value.at("probability")),{},{},{},{}};
    AtomicKind type;
    if (kind == "choice") type=AtomicKind::choice;
    else if (kind == "tag") type=AtomicKind::tag;
    else if (kind == "score") type=AtomicKind::score;
    else if (kind == "find") type=AtomicKind::find;
    else throw std::invalid_argument("unknown atomic kind");
    AtomicAnswer answer{type,{},{},{},{},{}};
    if (type == AtomicKind::choice || type == AtomicKind::find) answer.pick=value.at("pick").get<std::string>();
    if (type == AtomicKind::score) answer.level=value.at("level").get<std::string>();
    if (type != AtomicKind::tag) answer.confidence=read_optional<double>(value,"confidence",read_probability);
    for (const auto& [name,p] : value.at("probabilities").object()) answer.probabilities.push_back({name,read_probability(p)});
    return answer;
}
} // namespace tt::complete
