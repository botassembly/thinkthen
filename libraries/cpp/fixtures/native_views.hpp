#pragma once
#include <thinkthen/native.hpp>
#include <thinkthen/json.hpp>
namespace fixture {
using tt::Json; using namespace tt::native;
inline Json encode(const std::string& v) { return Json(v); }
template<class T, std::enable_if_t<std::is_arithmetic_v<T>,int> = 0> Json encode(T v) { return Json(static_cast<double>(v)); }
template<class T> Json encode(const std::vector<T>& v);
template<class T> Json encode(const std::optional<T>& v);
template<class T> Json encode(const std::shared_ptr<T>& v);
inline Json encode(const Attempt& v);
inline Json encode(const Batch& v);
inline Json encode(const BatchWarning& v);
inline Json encode(const Content& v);
inline Json encode(const Choice& v);
inline Json encode(const DecideValue& v);
inline Json encode(const Endpoint& v);
inline Json encode(const Edge& v);
inline Json encode(const Entity& v);
inline Json encode(const EntityEdge& v);
inline Json encode(const Facts& v);
inline Json encode(const ImageView& v);
inline Json encode(const InputDeclaration& v);
inline Json encode(const InputProperty& v);
inline Json encode(const Location& v);
inline Json encode(const InputView& v);
inline Json encode(const MemberFailure& v);
inline Json encode(const MemberValue& v);
inline Json encode(const Name& v);
inline Json encode(const NamedAnswer& v);
inline Json encode(const ObservationIdentity& v);
inline Json encode(const ObservedProbabilities& v);
inline Json encode(const ObservationSuccess& v);
inline Json encode(const Piece& v);
inline Json encode(const Place& v);
inline Json encode(const Pair& v);
inline Json encode(const Probability& v);
inline Json encode(const ProfileWarning& v);
inline Json encode(const QuestionAuthor& v);
inline Json encode(const QuestionMember& v);
inline Json encode(const QuestionSource& v);
inline Json encode(const RecognizeAnswer& v);
inline Json encode(const RecognizeValue& v);
inline Json encode(const RelationSuccess& v);
inline Json encode(const RelationAnswer& v);
inline Json encode(const Relation& v);
inline Json encode(const ReportedUsage& v);
inline Json encode(const Rule& v);
inline Json encode(const QuestionView& v);
inline Json encode(const Details& v);
inline Json encode(const ScoreAnswer& v);
inline Json encode(const Answer& v);
inline Json encode(const MemberSuccess& v);
inline Json encode(const Member& v);
inline Json encode(const SourceDetail& v);
inline Json encode(const SourceEndpoint& v);
inline Json encode(const SourceEdge& v);
inline Json encode(const SourceEntity& v);
inline Json encode(const SourceEntityEdge& v);
inline Json encode(const SourceRecognition& v);
inline Json encode(const SourceRelations& v);
inline Json encode(const Stopped& v);
inline Json encode(const Error& v);
inline Json encode(const Usage& v);
inline Json encode(const Meta& v);
inline Json encode(const QuestionObservation& v);
inline Json encode(const Row& v);
inline Json encode(const AnnotateView& v);
inline Json encode(const ChooseView& v);
inline Json encode(const DecideView& v);
inline Json encode(const FilterView& v);
inline Json encode(const FindView& v);
inline Json encode(const RankView& v);
inline Json encode(const RecognizeView& v);
inline Json encode(const RelateView& v);
inline Json encode(const ScoreView& v);
inline Json encode(const Summary& v);
inline Json encode(const TagView& v);
inline Json encode(const RowObservation& v);
inline Json encode(const Observation& v);
template<class T> Json encode(const std::vector<T>& v) { Json::Array a; for(const auto& x:v) a.push_back(encode(x)); return Json(a); }
template<class T> Json encode(const std::optional<T>& v) { return v?encode(*v):Json(nullptr); }
template<class T> Json encode(const std::shared_ptr<T>& v) { return v?encode(*v):Json(nullptr); }
inline Json encode(const Attempt& v) { return Json{{"ordinal",encode(v.ordinal)},{"request_sha256",encode(v.request_sha256)},{"wall_ms",encode(v.wall_ms)},{"outcome",encode(v.outcome)},{"sdk_request_id",encode(v.sdk_request_id)},{"status",encode(v.status)},{"server_ms",encode(v.server_ms)},{"request_id",encode(v.request_id)}}; }
inline Json encode(const Batch& v) { return Json{{"kind",encode(v.kind)},{"records",encode(v.records)}}; }
inline Json encode(const BatchWarning& v) { return Json{{"tuned_for",encode(v.tuned_for)},{"running",encode(v.running)}}; }
inline Json encode(const Content& v) { return Json{{"kind",encode(v.kind)},{"data",encode(v.data)}}; }
inline Json encode(const Choice& v) { return Json{{"name",encode(v.name)},{"description",encode(v.description)},{"weight",encode(v.weight)}}; }
inline Json encode(const DecideValue& v) { return Json{{"kind",encode(v.kind)},{"data",Json{{"boolean",encode(v.data.boolean)},{"authored",encode(v.data.authored)}}}}; }
inline Json encode(const Endpoint& v) { return Json{{"name",encode(v.name)},{"kind",encode(v.kind)}}; }
inline Json encode(const Edge& v) { return Json{{"relation",encode(v.relation)},{"source",encode(v.source)},{"target",encode(v.target)},{"probability",encode(v.probability)},{"either",encode(v.either)}}; }
inline Json encode(const Entity& v) { return Json{{"text",encode(v.text)},{"start",encode(v.start)},{"end",encode(v.end)},{"length",encode(v.length)},{"kind",encode(v.kind)},{"strength",encode(v.strength)}}; }
inline Json encode(const EntityEdge& v) { return Json{{"relation",encode(v.relation)},{"source",encode(v.source)},{"target",encode(v.target)},{"probability",encode(v.probability)},{"either",encode(v.either)}}; }
inline Json encode(const Facts& v) { return Json{{"call_id",encode(v.call_id)},{"cache_answers",encode(v.cache_answers)},{"estimated_cost_usd",encode(v.estimated_cost_usd)},{"input_tokens",encode(v.input_tokens)},{"model",encode(v.model)},{"output_tokens",encode(v.output_tokens)},{"records",encode(v.records)},{"requests_sent",encode(v.requests_sent)},{"seconds",encode(v.seconds)},{"command_ms",encode(v.command_ms)}}; }
inline Json encode(const ImageView& v) { return Json{{"media",encode(v.media)},{"bytes",encode(v.bytes)},{"width",encode(v.width)},{"height",encode(v.height)},{"filename",encode(v.filename)}}; }
inline Json encode(const InputDeclaration& v) { return Json{{"kind",encode(v.kind)},{"properties",encode(v.properties)},{"required",encode(v.required)}}; }
inline Json encode(const InputProperty& v) { return Json{{"name",encode(v.name)},{"kind",encode(v.kind)}}; }
inline Json encode(const Location& v) { return Json{{"file",encode(v.file)},{"first_line",encode(v.first_line)},{"last_line",encode(v.last_line)}}; }
inline Json encode(const InputView& v) { return Json{{"original",encode(v.original)},{"position",encode(v.position)},{"images",encode(v.images)}}; }
inline Json encode(const MemberFailure& v) { return Json{{"failure_id",encode(v.failure_id)},{"cause",encode(v.cause)}}; }
inline Json encode(const MemberValue& v) { return Json{{"kind",encode(v.kind)},{"data",Json{{"decide",encode(v.data.decide)},{"choose",encode(v.data.choose)},{"tag",encode(v.data.tag)},{"score",encode(v.data.score)}}}}; }
inline Json encode(const Name& v) { return Json{{"start",encode(v.start)},{"end",encode(v.end)},{"kinds",encode(v.kinds)},{"edges",encode(v.edges)}}; }
inline Json encode(const NamedAnswer& v) { return Json{{"pick",encode(v.pick)},{"probabilities",encode(v.probabilities)},{"confidence",encode(v.confidence)}}; }
inline Json encode(const ObservationIdentity& v) { return Json{{"kind",encode(v.kind)},{"data",Json{{"observation_id",encode(v.data.observation_id)},{"failure_id",encode(v.data.failure_id)}}}}; }
inline Json encode(const ObservedProbabilities& v) { return Json{{"kind",encode(v.kind)},{"data",Json{{"yes",encode(v.data.yes)},{"named",encode(v.data.named)}}}}; }
inline Json encode(const ObservationSuccess& v) { return Json{{"answer_id",encode(v.answer_id)},{"observation_id",encode(v.observation_id)},{"value",encode(v.value)},{"probabilities",encode(v.probabilities)},{"confidence",encode(v.confidence)}}; }
inline Json encode(const Piece& v) { return Json{{"start",encode(v.start)},{"end",encode(v.end)},{"tags",encode(v.tags)}}; }
inline Json encode(const Place& v) { return Json{{"start",encode(v.start)},{"end",encode(v.end)}}; }
inline Json encode(const Pair& v) { return Json{{"relation",encode(v.relation)},{"source",encode(v.source)},{"target",encode(v.target)},{"probability",encode(v.probability)}}; }
inline Json encode(const Probability& v) { return Json{{"name",encode(v.name)},{"probability",encode(v.probability)}}; }
inline Json encode(const ProfileWarning& v) { return Json{{"tuned_for",encode(v.tuned_for)},{"running",encode(v.running)}}; }
inline Json encode(const QuestionAuthor& v) { return Json{{"name",encode(v.name)},{"wording_version",encode(v.wording_version)},{"item_schema",encode(v.item_schema)},{"context_schema",encode(v.context_schema)}}; }
inline Json encode(const QuestionMember& v) { return Json{{"name",encode(v.name)},{"question",encode(v.question)}}; }
inline Json encode(const QuestionSource& v) { return Json{{"origin",encode(v.origin)},{"answered_by",encode(v.answered_by)}}; }
inline Json encode(const RecognizeAnswer& v) { return Json{{"pieces",encode(v.pieces)},{"names",encode(v.names)},{"pairs",encode(v.pairs)}}; }
inline Json encode(const RecognizeValue& v) { return Json{{"entities",encode(v.entities)},{"relations",encode(v.relations)}}; }
inline Json encode(const RelationSuccess& v) { return Json{{"answer_id",encode(v.answer_id)},{"probability",encode(v.probability)},{"accepted",encode(v.accepted)}}; }
inline Json encode(const RelationAnswer& v) { return Json{{"relation",encode(v.relation)},{"reads",encode(v.reads)},{"method",encode(v.method)},{"direction",encode(v.direction)},{"source",encode(v.source)},{"target",encode(v.target)},{"request",encode(v.request)},{"state",encode(v.state)},{"data",Json{{"success",encode(v.data.success)},{"failure",encode(v.data.failure)}}}}; }
inline Json encode(const Relation& v) { return Json{{"name",encode(v.name)},{"source",encode(v.source)},{"target",encode(v.target)},{"reads",encode(v.reads)},{"either",encode(v.either)},{"single",encode(v.single)}}; }
inline Json encode(const ReportedUsage& v) { return Json{{"present",encode(v.present)},{"input_tokens",encode(v.input_tokens)},{"output_tokens",encode(v.output_tokens)}}; }
inline Json encode(const Rule& v) { return Json{{"kind",encode(v.kind)},{"low",encode(v.low)},{"high",encode(v.high)}}; }
inline Json encode(const QuestionView& v) { return Json{{"kind",encode(v.kind)},{"text",encode(v.text)},{"yes",encode(v.yes)},{"no",encode(v.no)},{"choices",encode(v.choices)},{"threshold",encode(v.threshold)},{"relation_threshold",encode(v.relation_threshold)},{"model",encode(v.model)},{"profile",encode(v.profile)},{"batch",encode(v.batch)},{"batch_max",encode(v.batch_max)},{"none",encode(v.none)},{"on",encode(v.on)},{"members",encode(v.members)},{"kinds",encode(v.kinds)},{"relations",encode(v.relations)},{"name_pointer",encode(v.name_pointer)},{"kind_pointer",encode(v.kind_pointer)}}; }
inline Json encode(const Details& v) { return Json{{"question",encode(v.question)},{"threshold",encode(v.threshold)},{"raw_pick",encode(v.raw_pick)},{"usage",encode(v.usage)},{"question_sources",encode(v.question_sources)},{"observations",encode(v.observations)},{"inputs",encode(v.inputs)}}; }
inline Json encode(const ScoreAnswer& v) { return Json{{"level",encode(v.level)},{"probabilities",encode(v.probabilities)},{"confidence",encode(v.confidence)}}; }
inline Json encode(const Answer& v) { return Json{{"kind",encode(v.kind)},{"data",Json{{"probability",encode(v.data.probability)},{"choice",encode(v.data.choice)},{"tag",encode(v.data.tag)},{"score",encode(v.data.score)},{"find",encode(v.data.find)}}}}; }
inline Json encode(const MemberSuccess& v) { return Json{{"answer_id",encode(v.answer_id)},{"value",encode(v.value)},{"answer",encode(v.answer)},{"threshold",encode(v.threshold)}}; }
inline Json encode(const Member& v) { return Json{{"name",encode(v.name)},{"request",encode(v.request)},{"question",encode(v.question)},{"state",encode(v.state)},{"data",Json{{"success",encode(v.data.success)},{"failure",encode(v.data.failure)}}}}; }
inline Json encode(const SourceDetail& v) { return Json{{"origin",encode(v.origin)},{"answered_by",encode(v.answered_by)},{"batch_size",encode(v.batch_size)}}; }
inline Json encode(const SourceEndpoint& v) { return Json{{"ordinal",encode(v.ordinal)},{"endpoint",encode(v.endpoint)},{"record",encode(v.record)},{"position",encode(v.position)}}; }
inline Json encode(const SourceEdge& v) { return Json{{"relation",encode(v.relation)},{"source",encode(v.source)},{"target",encode(v.target)},{"probability",encode(v.probability)},{"either",encode(v.either)}}; }
inline Json encode(const SourceEntity& v) { return Json{{"entity",encode(v.entity)},{"position",encode(v.position)}}; }
inline Json encode(const SourceEntityEdge& v) { return Json{{"relation",encode(v.relation)},{"source",encode(v.source)},{"target",encode(v.target)},{"probability",encode(v.probability)},{"either",encode(v.either)}}; }
inline Json encode(const SourceRecognition& v) { return Json{{"present",encode(v.present)},{"entities",encode(v.entities)},{"relations",encode(v.relations)}}; }
inline Json encode(const SourceRelations& v) { return Json{{"present",encode(v.present)},{"edges",encode(v.edges)}}; }
inline Json encode(const Stopped& v) { return Json{{"at",encode(v.at)},{"cause",encode(v.cause)},{"status",encode(v.status)},{"retryable",encode(v.retryable)}}; }
inline Json encode(const Error& v) { return Json{{"code",encode(v.code)},{"message",encode(v.message)},{"retryable",encode(v.retryable)},{"stopped",encode(v.stopped)}}; }
inline Json encode(const Usage& v) { return Json{{"input_tokens",encode(v.input_tokens)},{"output_tokens",encode(v.output_tokens)}}; }
inline Json encode(const Meta& v) { return Json{{"tool",encode(v.tool)},{"question_sha256",encode(v.question_sha256)},{"questions_sha256",encode(v.questions_sha256)},{"url",encode(v.url)},{"model",encode(v.model)},{"usage",encode(v.usage)},{"requests_sent",encode(v.requests_sent)},{"cached",encode(v.cached)},{"requests",encode(v.requests)},{"failed_questions",encode(v.failed_questions)},{"profile_warning",encode(v.profile_warning)},{"batch_setting",encode(v.batch_setting)},{"batch_warning",encode(v.batch_warning)},{"context_sha256",encode(v.context_sha256)},{"attempts",encode(v.attempts)},{"origin",encode(v.origin)},{"question_sources",encode(v.question_sources)},{"observations",encode(v.observations)},{"answered_by",encode(v.answered_by)}}; }
inline Json encode(const QuestionObservation& v) { return Json{{"index",encode(v.index)},{"member",encode(v.member)},{"stage",encode(v.stage)},{"position",encode(v.position)},{"question_sha256",encode(v.question_sha256)},{"model",encode(v.model)},{"url",encode(v.url)},{"requests",encode(v.requests)},{"requests_sent",encode(v.requests_sent)},{"cached",encode(v.cached)},{"failed_questions",encode(v.failed_questions)},{"usage",encode(v.usage)},{"question_sources",encode(v.question_sources)},{"state",encode(v.state)},{"data",Json{{"success",encode(v.data.success)},{"failure",encode(v.data.failure)}}}}; }
inline Json encode(const Row& v) { return Json{{"answer_id",encode(v.answer_id)},{"input",encode(v.input)},{"question",encode(v.question)},{"answer",encode(v.answer)},{"threshold",encode(v.threshold)},{"position",encode(v.position)},{"input_file",encode(v.input_file)},{"meta",encode(v.meta)},{"images",encode(v.images)}}; }
inline Json encode(const AnnotateView& v) { return Json{{"common",encode(v.common)},{"answers",encode(v.answers)}}; }
inline Json encode(const ChooseView& v) { return Json{{"common",encode(v.common)},{"value",encode(v.value)}}; }
inline Json encode(const DecideView& v) { return Json{{"common",encode(v.common)},{"value",encode(v.value)}}; }
inline Json encode(const FilterView& v) { return Json{{"common",encode(v.common)},{"value",encode(v.value)}}; }
inline Json encode(const FindView& v) { return Json{{"common",encode(v.common)},{"value",encode(v.value)},{"index",encode(v.index)}}; }
inline Json encode(const RankView& v) { return Json{{"common",encode(v.common)},{"value",encode(v.value)},{"question_name",encode(v.question_name)}}; }
inline Json encode(const RecognizeView& v) { return Json{{"common",encode(v.common)},{"value",encode(v.value)},{"answer",encode(v.answer)}}; }
inline Json encode(const RelateView& v) { return Json{{"common",encode(v.common)},{"value",encode(v.value)},{"questions",encode(v.questions)}}; }
inline Json encode(const ScoreView& v) { return Json{{"common",encode(v.common)},{"value",encode(v.value)}}; }
inline Json encode(const Summary& v) { return Json{{"state",encode(v.state)},{"schema",encode(v.schema)},{"answer_id",encode(v.answer_id)},{"function",encode(v.function)},{"count",encode(v.count)},{"observation_count",encode(v.observation_count)},{"meta",encode(v.meta)},{"facts",encode(v.facts)},{"attempts",encode(v.attempts)},{"error",encode(v.error)}}; }
inline Json encode(const TagView& v) { return Json{{"common",encode(v.common)},{"value",encode(v.value)}}; }
inline Json encode(const RowObservation& v) { return Json{{"index",encode(v.index)},{"function",encode(v.function)},{"data",Json{{"decide",encode(v.data.decide)},{"choose",encode(v.data.choose)},{"tag",encode(v.data.tag)},{"score",encode(v.data.score)},{"filter",encode(v.data.filter)},{"rank",encode(v.data.rank)},{"find",encode(v.data.find)},{"annotate",encode(v.data.annotate)},{"recognize",encode(v.data.recognize)},{"relate",encode(v.data.relate)}}}}; }
inline Json encode(const Observation& v) { return Json{{"kind",encode(v.kind)},{"data",Json{{"question",encode(v.data.question)},{"row",encode(v.data.row)}}}}; }
inline Json encode(const Result& v) { return Json{{"summary",encode(v.summary)},{"rows",encode(v.rows)},{"observations",encode(v.observations)},{"details",encode(v.details)},{"observation_details",encode(v.observation_details)},{"authors",encode(v.authors)},{"observation_authors",encode(v.observation_authors)},{"member_authors",encode(v.member_authors)},{"rank_members",encode(v.rank_members)},{"rank_member_details",encode(v.rank_member_details)},{"located_recognition",encode(v.located_recognition)},{"located_relations",encode(v.located_relations)}}; }
}
