#pragma once
#include <type_traits>
#include <thinkthen/thinkthen.h>
#include <string>
#include <vector>
#include <optional>
#include <memory>
#include <limits>
#include <stdexcept>
namespace tt::native {
// Complete native views copied into host values; no returned pointer borrows C ownership.
struct RelationAnswer;
struct QuestionAuthor;
struct RelateView;
struct Batch;
struct SourceRelations;
struct ImageView;
struct Usage;
struct EntityEdge;
struct ObservationIdentity;
struct Relation;
struct Member;
struct Facts;
struct InputView;
struct SourceEndpoint;
struct SourceDetail;
struct Location;
struct Error;
struct InputProperty;
struct FilterView;
struct QuestionObservation;
struct Name;
struct Edge;
struct Attempt;
struct Summary;
struct SourceRecognition;
struct Content;
struct SourceEntityEdge;
struct InputDeclaration;
struct MemberSuccess;
struct MemberFailure;
struct FindView;
struct DecideValue;
struct ScoreAnswer;
struct QuestionMember;
struct TagView;
struct RecognizeValue;
struct BatchWarning;
struct AnnotateView;
struct Entity;
struct Endpoint;
struct Observation;
struct ReportedUsage;
struct QuestionView;
struct Place;
struct RecognizeView;
struct ObservationSuccess;
struct Answer;
struct QuestionSource;
struct ObservedProbabilities;
struct Piece;
struct SourceEntity;
struct Choice;
struct RowObservation;
struct Probability;
struct RankView;
struct ProfileWarning;
struct Stopped;
struct Rule;
struct NamedAnswer;
struct RecognizeAnswer;
struct Meta;
struct ChooseView;
struct SourceEdge;
struct Details;
struct DecideView;
struct ScoreView;
struct Row;
struct RelationSuccess;
struct Pair;
struct MemberValue;
struct Attempt { uint64_t ordinal; std::string request_sha256; uint64_t wall_ms; uint32_t outcome; std::string sdk_request_id; std::optional<uint16_t> status; std::optional<uint64_t> server_ms; std::optional<std::string> request_id; };
struct Batch { uint32_t kind; size_t records; };
struct BatchWarning { Batch tuned_for; Batch running; };
struct Content { uint32_t kind; std::string data; };
struct Choice { std::string name; std::optional<Content> description; std::optional<double> weight; };
struct DecideValue { uint32_t kind; struct { std::optional<int> boolean; std::optional<Content> authored; } data; };
struct Endpoint { std::string name; std::string kind; };
struct Edge { std::string relation; Endpoint source; Endpoint target; double probability; int either; };
struct Entity { std::string text; size_t start; size_t end; size_t length; std::string kind; double strength; };
struct EntityEdge { std::string relation; Entity source; Entity target; double probability; int either; };
struct Facts { std::string call_id; uint64_t cache_answers; std::optional<std::string> estimated_cost_usd; std::optional<uint64_t> input_tokens; std::optional<std::string> model; std::optional<uint64_t> output_tokens; uint64_t records; uint64_t requests_sent; double seconds; std::optional<uint64_t> command_ms; };
struct ImageView { uint32_t media; std::vector<uint8_t> bytes; uint32_t width; uint32_t height; std::optional<std::string> filename; };
struct InputDeclaration { uint32_t kind; std::vector<InputProperty> properties; std::vector<std::string> required; };
struct InputProperty { std::string name; uint32_t kind; };
struct Location { std::optional<std::string> file; std::optional<size_t> first_line; std::optional<size_t> last_line; };
struct InputView { std::optional<Content> original; std::optional<Location> position; std::optional<std::vector<ImageView>> images; };
struct MemberFailure { std::string failure_id; uint32_t cause; };
struct MemberValue { uint32_t kind; struct { std::optional<DecideValue> decide; std::optional<std::optional<std::string>> choose; std::optional<std::vector<std::string>> tag; std::optional<double> score; } data; };
struct Name { size_t start; size_t end; std::optional<std::vector<Probability>> kinds; std::optional<std::vector<Probability>> edges; };
struct NamedAnswer { std::string pick; std::vector<Probability> probabilities; std::optional<double> confidence; };
struct ObservationIdentity { uint32_t kind; struct { std::optional<std::string> observation_id; std::optional<std::string> failure_id; } data; };
struct ObservedProbabilities { uint32_t kind; struct { std::optional<double> yes; std::optional<std::vector<Probability>> named; } data; };
struct ObservationSuccess { std::string answer_id; std::string observation_id; MemberValue value; ObservedProbabilities probabilities; std::optional<double> confidence; };
struct Piece { size_t start; size_t end; std::vector<Probability> tags; };
struct Place { size_t start; size_t end; };
struct Pair { std::string relation; Place source; Place target; double probability; };
struct Probability { std::string name; double probability; };
struct ProfileWarning { std::string tuned_for; std::string running; };
struct QuestionAuthor { std::optional<std::string> name; std::optional<uint64_t> wording_version; InputDeclaration item_schema; InputDeclaration context_schema; };
struct QuestionMember { std::string name; std::shared_ptr<QuestionView> question; };
struct QuestionSource { uint32_t origin; std::string answered_by; };
struct RecognizeAnswer { std::vector<Piece> pieces; std::vector<Name> names; std::vector<Pair> pairs; };
struct RecognizeValue { std::vector<Entity> entities; std::optional<std::vector<EntityEdge>> relations; };
struct RelationSuccess { std::string answer_id; double probability; int accepted; };
struct RelationAnswer { std::string relation; std::string reads; uint32_t method; uint32_t direction; Endpoint source; std::optional<Endpoint> target; std::string request; uint32_t state; struct { std::optional<RelationSuccess> success; std::optional<MemberFailure> failure; } data; };
struct Relation { std::string name; std::string source; std::string target; std::optional<std::string> reads; int either; int single; };
struct ReportedUsage { int present; std::optional<uint64_t> input_tokens; std::optional<uint64_t> output_tokens; };
struct Rule { uint32_t kind; double low; double high; };
struct QuestionView { uint32_t kind; Content text; std::optional<Content> yes; std::optional<Content> no; std::vector<Choice> choices; Rule threshold; Rule relation_threshold; std::optional<std::string> model; std::optional<std::string> profile; std::optional<size_t> batch; int batch_max; int none; std::vector<std::string> on; std::vector<QuestionMember> members; std::vector<Choice> kinds; std::vector<Relation> relations; std::optional<std::string> name_pointer; std::optional<std::string> kind_pointer; };
struct Details { std::optional<QuestionView> question; std::optional<Rule> threshold; std::optional<std::string> raw_pick; ReportedUsage usage; std::vector<SourceDetail> question_sources; std::vector<ObservationIdentity> observations; std::vector<InputView> inputs; };
struct ScoreAnswer { std::string level; std::vector<Probability> probabilities; std::optional<double> confidence; };
struct Answer { uint32_t kind; struct { std::optional<double> probability; std::optional<NamedAnswer> choice; std::optional<std::vector<Probability>> tag; std::optional<ScoreAnswer> score; std::optional<NamedAnswer> find; } data; };
struct MemberSuccess { std::string answer_id; MemberValue value; Answer answer; Rule threshold; };
struct Member { std::string name; std::string request; QuestionView question; uint32_t state; struct { std::optional<MemberSuccess> success; std::optional<MemberFailure> failure; } data; };
struct SourceDetail { uint32_t origin; std::string answered_by; std::optional<size_t> batch_size; };
struct SourceEndpoint { size_t ordinal; Endpoint endpoint; Content record; std::optional<Location> position; };
struct SourceEdge { std::string relation; SourceEndpoint source; SourceEndpoint target; double probability; int either; };
struct SourceEntity { Entity entity; std::optional<Location> position; };
struct SourceEntityEdge { std::string relation; SourceEntity source; SourceEntity target; double probability; int either; };
struct SourceRecognition { int present; std::vector<SourceEntity> entities; std::optional<std::vector<SourceEntityEdge>> relations; };
struct SourceRelations { int present; std::vector<SourceEdge> edges; };
struct Stopped { std::optional<size_t> at; uint32_t cause; std::optional<uint16_t> status; int retryable; };
struct Error { int code; std::string message; int retryable; std::optional<Stopped> stopped; };
struct Usage { uint64_t input_tokens; uint64_t output_tokens; };
struct Meta { std::string tool; std::optional<std::string> question_sha256; std::optional<std::string> questions_sha256; std::string url; std::string model; std::optional<Usage> usage; uint64_t requests_sent; int cached; std::vector<std::string> requests; size_t failed_questions; std::optional<ProfileWarning> profile_warning; std::optional<Batch> batch_setting; std::optional<BatchWarning> batch_warning; std::optional<std::string> context_sha256; std::optional<std::vector<Attempt>> attempts; std::optional<uint32_t> origin; std::vector<QuestionSource> question_sources; std::vector<ObservationIdentity> observations; std::optional<std::string> answered_by; };
struct QuestionObservation { size_t index; std::optional<std::string> member; std::optional<uint32_t> stage; size_t position; std::string question_sha256; std::string model; std::string url; std::vector<std::string> requests; uint64_t requests_sent; int cached; size_t failed_questions; std::optional<Usage> usage; std::vector<QuestionSource> question_sources; uint32_t state; struct { std::optional<ObservationSuccess> success; std::optional<MemberFailure> failure; } data; };
struct Row { std::string answer_id; std::optional<Content> input; std::optional<QuestionView> question; std::optional<Answer> answer; std::optional<Rule> threshold; std::optional<Location> position; std::optional<std::string> input_file; Meta meta; std::optional<std::vector<ImageView>> images; };
struct AnnotateView { Row common; std::vector<Member> answers; };
struct ChooseView { Row common; std::optional<std::string> value; };
struct DecideView { Row common; DecideValue value; };
struct FilterView { Row common; int value; };
struct FindView { Row common; std::optional<Content> value; std::optional<size_t> index; };
struct RankView { Row common; std::optional<size_t> value; std::optional<std::string> question_name; };
struct RecognizeView { Row common; RecognizeValue value; RecognizeAnswer answer; };
struct RelateView { Row common; std::vector<Edge> value; std::vector<RelationAnswer> questions; };
struct ScoreView { Row common; double value; };
struct Summary { uint32_t state; std::string schema; std::optional<std::string> answer_id; std::optional<uint32_t> function; size_t count; size_t observation_count; std::optional<Meta> meta; std::optional<Facts> facts; std::optional<std::vector<Attempt>> attempts; std::optional<Error> error; };
struct TagView { Row common; std::vector<std::string> value; };
struct RowObservation { size_t index; uint32_t function; struct { std::optional<DecideView> decide; std::optional<ChooseView> choose; std::optional<TagView> tag; std::optional<ScoreView> score; std::optional<FilterView> filter; std::optional<RankView> rank; std::optional<FindView> find; std::optional<AnnotateView> annotate; std::optional<RecognizeView> recognize; std::optional<RelateView> relate; } data; };
struct Observation { uint32_t kind; struct { std::optional<QuestionObservation> question; std::optional<RowObservation> row; } data; };
template<class T, std::enable_if_t<std::is_arithmetic_v<T>,int> = 0> T copy(T v) { return v; }
inline bool present(int n) { if(n!=0 && n!=1) throw std::runtime_error("invalid native presence"); return n!=0; }
inline size_t extent(size_t n, size_t width, const void* p) { if(n>std::numeric_limits<size_t>::max()/width || (n && !p)) throw std::runtime_error("invalid native extent"); return n; }
inline std::string copy(thinkthen_string_v1 v) { extent(v.len,1,v.data); return v.len?std::string(v.data,v.len):std::string(); }
inline AnnotateView copy(thinkthen_annotate_view_v1 v);
inline Answer copy(thinkthen_answer_v1 v);
inline Attempt copy(thinkthen_attempt_v1 v);
inline std::vector<Attempt> copy(thinkthen_attempts_v1 v);
inline Batch copy(thinkthen_batch_v1 v);
inline BatchWarning copy(thinkthen_batch_warning_v1 v);
inline Choice copy(thinkthen_choice_v1 v);
inline std::vector<Choice> copy(thinkthen_choices_v1 v);
inline ChooseView copy(thinkthen_choose_view_v1 v);
inline Content copy(thinkthen_content_v1 v);
inline DecideValue copy(thinkthen_decide_value_v1 v);
inline DecideView copy(thinkthen_decide_view_v1 v);
inline Details copy(thinkthen_details_v1 v);
inline Edge copy(thinkthen_edge_v1 v);
inline std::vector<Edge> copy(thinkthen_edges_v1 v);
inline Endpoint copy(thinkthen_endpoint_v1 v);
inline std::vector<Entity> copy(thinkthen_entities_v1 v);
inline EntityEdge copy(thinkthen_entity_edge_v1 v);
inline std::vector<EntityEdge> copy(thinkthen_entity_edges_v1 v);
inline Entity copy(thinkthen_entity_v1 v);
inline Error copy(thinkthen_error_v1 v);
inline Facts copy(thinkthen_facts_v1 v);
inline FilterView copy(thinkthen_filter_view_v1 v);
inline FindView copy(thinkthen_find_view_v1 v);
inline ImageView copy(thinkthen_image_view_v1 v);
inline std::vector<ImageView> copy(thinkthen_image_views_v1 v);
inline InputDeclaration copy(thinkthen_input_declaration_v1 v);
inline std::vector<InputProperty> copy(thinkthen_input_properties_v1 v);
inline InputProperty copy(thinkthen_input_property_v1 v);
inline InputView copy(thinkthen_input_view_v1 v);
inline std::vector<InputView> copy(thinkthen_input_views_v1 v);
inline Location copy(thinkthen_location_v1 v);
inline MemberFailure copy(thinkthen_member_failure_v1 v);
inline MemberSuccess copy(thinkthen_member_success_v1 v);
inline Member copy(thinkthen_member_v1 v);
inline MemberValue copy(thinkthen_member_value_v1 v);
inline std::vector<Member> copy(thinkthen_members_v1 v);
inline Meta copy(thinkthen_meta_v1 v);
inline Name copy(thinkthen_name_v1 v);
inline NamedAnswer copy(thinkthen_named_answer_v1 v);
inline std::vector<Name> copy(thinkthen_names_v1 v);
inline std::vector<ObservationIdentity> copy(thinkthen_observation_identities_v1 v);
inline ObservationIdentity copy(thinkthen_observation_identity_v1 v);
inline ObservationSuccess copy(thinkthen_observation_success_v1 v);
inline Observation copy(thinkthen_observation_v1 v);
inline ObservedProbabilities copy(thinkthen_observed_probabilities_v1 v);
inline std::optional<Answer> copy(thinkthen_optional_answer_v1 v);
inline std::optional<std::vector<Attempt>> copy(thinkthen_optional_attempts_v1 v);
inline std::optional<Batch> copy(thinkthen_optional_batch_v1 v);
inline std::optional<BatchWarning> copy(thinkthen_optional_batch_warning_v1 v);
inline std::optional<Content> copy(thinkthen_optional_content_v1 v);
inline std::optional<uint32_t> copy(thinkthen_optional_discriminator_v1 v);
inline std::optional<double> copy(thinkthen_optional_double_v1 v);
inline std::optional<Endpoint> copy(thinkthen_optional_endpoint_v1 v);
inline std::optional<std::vector<EntityEdge>> copy(thinkthen_optional_entity_edges_v1 v);
inline std::optional<Error> copy(thinkthen_optional_error_v1 v);
inline std::optional<Facts> copy(thinkthen_optional_facts_v1 v);
inline std::optional<std::vector<ImageView>> copy(thinkthen_optional_image_views_v1 v);
inline std::optional<Location> copy(thinkthen_optional_location_v1 v);
inline std::optional<Meta> copy(thinkthen_optional_meta_v1 v);
inline std::optional<std::vector<Probability>> copy(thinkthen_optional_probabilities_v1 v);
inline std::optional<ProfileWarning> copy(thinkthen_optional_profile_warning_v1 v);
inline std::optional<QuestionView> copy(thinkthen_optional_question_v1 v);
inline std::optional<Rule> copy(thinkthen_optional_rule_v1 v);
inline std::optional<size_t> copy(thinkthen_optional_size_v1 v);
inline std::optional<std::vector<SourceEntityEdge>> copy(thinkthen_optional_source_entity_edges_v1 v);
inline std::optional<Stopped> copy(thinkthen_optional_stopped_v1 v);
inline std::optional<std::string> copy(thinkthen_optional_string_v1 v);
inline std::optional<uint16_t> copy(thinkthen_optional_u16_v1 v);
inline std::optional<uint64_t> copy(thinkthen_optional_u64_v1 v);
inline std::optional<Usage> copy(thinkthen_optional_usage_v1 v);
inline Pair copy(thinkthen_pair_v1 v);
inline std::vector<Pair> copy(thinkthen_pairs_v1 v);
inline Piece copy(thinkthen_piece_v1 v);
inline std::vector<Piece> copy(thinkthen_pieces_v1 v);
inline Place copy(thinkthen_place_v1 v);
inline std::vector<Probability> copy(thinkthen_probabilities_v1 v);
inline Probability copy(thinkthen_probability_v1 v);
inline ProfileWarning copy(thinkthen_profile_warning_v1 v);
inline QuestionAuthor copy(thinkthen_question_author_v1 v);
inline QuestionMember copy(thinkthen_question_member_v1 v);
inline std::vector<QuestionMember> copy(thinkthen_question_members_v1 v);
inline QuestionObservation copy(thinkthen_question_observation_v1 v);
inline QuestionSource copy(thinkthen_question_source_v1 v);
inline std::vector<QuestionSource> copy(thinkthen_question_sources_v1 v);
inline QuestionView copy(thinkthen_question_view_v1 v);
inline RankView copy(thinkthen_rank_view_v1 v);
inline RecognizeAnswer copy(thinkthen_recognize_answer_v1 v);
inline RecognizeValue copy(thinkthen_recognize_value_v1 v);
inline RecognizeView copy(thinkthen_recognize_view_v1 v);
inline RelateView copy(thinkthen_relate_view_v1 v);
inline RelationAnswer copy(thinkthen_relation_answer_v1 v);
inline std::vector<RelationAnswer> copy(thinkthen_relation_answers_v1 v);
inline RelationSuccess copy(thinkthen_relation_success_v1 v);
inline Relation copy(thinkthen_relation_v1 v);
inline std::vector<Relation> copy(thinkthen_relations_v1 v);
inline ReportedUsage copy(thinkthen_reported_usage_v1 v);
inline RowObservation copy(thinkthen_row_observation_v1 v);
inline Row copy(thinkthen_row_v1 v);
inline Rule copy(thinkthen_rule_v1 v);
inline ScoreAnswer copy(thinkthen_score_answer_v1 v);
inline ScoreView copy(thinkthen_score_view_v1 v);
inline SourceDetail copy(thinkthen_source_detail_v1 v);
inline std::vector<SourceDetail> copy(thinkthen_source_details_v1 v);
inline SourceEdge copy(thinkthen_source_edge_v1 v);
inline std::vector<SourceEdge> copy(thinkthen_source_edges_v1 v);
inline SourceEndpoint copy(thinkthen_source_endpoint_v1 v);
inline std::vector<SourceEntity> copy(thinkthen_source_entities_v1 v);
inline SourceEntityEdge copy(thinkthen_source_entity_edge_v1 v);
inline std::vector<SourceEntityEdge> copy(thinkthen_source_entity_edges_v1 v);
inline SourceEntity copy(thinkthen_source_entity_v1 v);
inline SourceRecognition copy(thinkthen_source_recognition_v1 v);
inline SourceRelations copy(thinkthen_source_relations_v1 v);
inline Stopped copy(thinkthen_stopped_v1 v);
inline std::vector<std::string> copy(thinkthen_strings_v1 v);
inline Summary copy(thinkthen_summary_v1 v);
inline TagView copy(thinkthen_tag_view_v1 v);
inline Usage copy(thinkthen_usage_v1 v);
inline AnnotateView copy(thinkthen_annotate_view_v1 v) { AnnotateView out{}; out.common=copy(v.common); out.answers=copy(v.answers); return out; }
inline Answer copy(thinkthen_answer_v1 v) { Answer out{}; out.kind=copy(v.kind); switch(v.kind) { case 1: out.data.probability=copy(v.data.probability); break; case 2: out.data.choice=copy(v.data.choice); break; case 3: out.data.tag=copy(v.data.tag); break; case 4: out.data.score=copy(v.data.score); break; case 5: out.data.find=copy(v.data.find); break; default: throw std::runtime_error("invalid native thinkthen_answer_v1 discriminator " + std::to_string(v.kind)); } return out; }
inline Attempt copy(thinkthen_attempt_v1 v) { Attempt out{}; out.ordinal=copy(v.ordinal); out.request_sha256=copy(v.request_sha256); out.wall_ms=copy(v.wall_ms); out.outcome=copy(v.outcome); out.sdk_request_id=copy(v.sdk_request_id); out.status=copy(v.status); out.server_ms=copy(v.server_ms); out.request_id=copy(v.request_id); return out; }
inline std::vector<Attempt> copy(thinkthen_attempts_v1 v) { std::vector<Attempt> out; extent(v.len,sizeof(*v.data),v.data); out.reserve(v.len); for(size_t i=0;i<v.len;++i) out.push_back(copy(v.data[i])); return out; }
inline Batch copy(thinkthen_batch_v1 v) { Batch out{}; out.kind=copy(v.kind); out.records=copy(v.records); return out; }
inline BatchWarning copy(thinkthen_batch_warning_v1 v) { BatchWarning out{}; out.tuned_for=copy(v.tuned_for); out.running=copy(v.running); return out; }
inline Choice copy(thinkthen_choice_v1 v) { Choice out{}; out.name=copy(v.name); out.description=copy(v.description); out.weight=copy(v.weight); return out; }
inline std::vector<Choice> copy(thinkthen_choices_v1 v) { std::vector<Choice> out; extent(v.len,sizeof(*v.data),v.data); out.reserve(v.len); for(size_t i=0;i<v.len;++i) out.push_back(copy(v.data[i])); return out; }
inline ChooseView copy(thinkthen_choose_view_v1 v) { ChooseView out{}; out.common=copy(v.common); out.value=copy(v.value); return out; }
inline Content copy(thinkthen_content_v1 v) { Content out{}; out.kind=copy(v.kind); out.data=copy(v.data); return out; }
inline DecideValue copy(thinkthen_decide_value_v1 v) { DecideValue out{}; out.kind=copy(v.kind); switch(v.kind) { case 0: break; case 1: out.data.boolean=copy(v.data.boolean); break; case 2: out.data.authored=copy(v.data.authored); break; default: throw std::runtime_error("invalid native thinkthen_decide_value_v1 discriminator " + std::to_string(v.kind)); } return out; }
inline DecideView copy(thinkthen_decide_view_v1 v) { DecideView out{}; out.common=copy(v.common); out.value=copy(v.value); return out; }
inline Details copy(thinkthen_details_v1 v) { Details out{}; out.question=copy(v.question); out.threshold=copy(v.threshold); out.raw_pick=copy(v.raw_pick); out.usage=copy(v.usage); out.question_sources=copy(v.question_sources); out.observations=copy(v.observations); out.inputs=copy(v.inputs); return out; }
inline Edge copy(thinkthen_edge_v1 v) { Edge out{}; out.relation=copy(v.relation); out.source=copy(v.source); out.target=copy(v.target); out.probability=copy(v.probability); out.either=copy(v.either); return out; }
inline std::vector<Edge> copy(thinkthen_edges_v1 v) { std::vector<Edge> out; extent(v.len,sizeof(*v.data),v.data); out.reserve(v.len); for(size_t i=0;i<v.len;++i) out.push_back(copy(v.data[i])); return out; }
inline Endpoint copy(thinkthen_endpoint_v1 v) { Endpoint out{}; out.name=copy(v.name); out.kind=copy(v.kind); return out; }
inline std::vector<Entity> copy(thinkthen_entities_v1 v) { std::vector<Entity> out; extent(v.len,sizeof(*v.data),v.data); out.reserve(v.len); for(size_t i=0;i<v.len;++i) out.push_back(copy(v.data[i])); return out; }
inline EntityEdge copy(thinkthen_entity_edge_v1 v) { EntityEdge out{}; out.relation=copy(v.relation); out.source=copy(v.source); out.target=copy(v.target); out.probability=copy(v.probability); out.either=copy(v.either); return out; }
inline std::vector<EntityEdge> copy(thinkthen_entity_edges_v1 v) { std::vector<EntityEdge> out; extent(v.len,sizeof(*v.data),v.data); out.reserve(v.len); for(size_t i=0;i<v.len;++i) out.push_back(copy(v.data[i])); return out; }
inline Entity copy(thinkthen_entity_v1 v) { Entity out{}; out.text=copy(v.text); out.start=copy(v.start); out.end=copy(v.end); out.length=copy(v.length); out.kind=copy(v.kind); out.strength=copy(v.strength); return out; }
inline Error copy(thinkthen_error_v1 v) { Error out{}; out.code=copy(v.code); out.message=copy(v.message); out.retryable=copy(v.retryable); out.stopped=copy(v.stopped); return out; }
inline Facts copy(thinkthen_facts_v1 v) { Facts out{}; out.call_id=copy(v.call_id); out.cache_answers=copy(v.cache_answers); out.estimated_cost_usd=copy(v.estimated_cost_usd); out.input_tokens=copy(v.input_tokens); out.model=copy(v.model); out.output_tokens=copy(v.output_tokens); out.records=copy(v.records); out.requests_sent=copy(v.requests_sent); out.seconds=copy(v.seconds); out.command_ms=copy(v.command_ms); return out; }
inline FilterView copy(thinkthen_filter_view_v1 v) { FilterView out{}; out.common=copy(v.common); out.value=copy(v.value); return out; }
inline FindView copy(thinkthen_find_view_v1 v) { FindView out{}; out.common=copy(v.common); out.value=copy(v.value); out.index=copy(v.index); return out; }
inline ImageView copy(thinkthen_image_view_v1 v) { ImageView out{}; out.media=copy(v.media); extent(v.bytes_len,1,v.bytes); if(v.bytes_len) out.bytes.assign(v.bytes,v.bytes+v.bytes_len); out.width=copy(v.width); out.height=copy(v.height); out.filename=copy(v.filename); return out; }
inline std::vector<ImageView> copy(thinkthen_image_views_v1 v) { std::vector<ImageView> out; extent(v.len,sizeof(*v.data),v.data); out.reserve(v.len); for(size_t i=0;i<v.len;++i) out.push_back(copy(v.data[i])); return out; }
inline InputDeclaration copy(thinkthen_input_declaration_v1 v) { InputDeclaration out{}; out.kind=copy(v.kind); out.properties=copy(v.properties); out.required=copy(v.required); return out; }
inline std::vector<InputProperty> copy(thinkthen_input_properties_v1 v) { std::vector<InputProperty> out; extent(v.len,sizeof(*v.data),v.data); out.reserve(v.len); for(size_t i=0;i<v.len;++i) out.push_back(copy(v.data[i])); return out; }
inline InputProperty copy(thinkthen_input_property_v1 v) { InputProperty out{}; out.name=copy(v.name); out.kind=copy(v.kind); return out; }
inline InputView copy(thinkthen_input_view_v1 v) { InputView out{}; out.original=copy(v.original); out.position=copy(v.position); out.images=copy(v.images); return out; }
inline std::vector<InputView> copy(thinkthen_input_views_v1 v) { std::vector<InputView> out; extent(v.len,sizeof(*v.data),v.data); out.reserve(v.len); for(size_t i=0;i<v.len;++i) out.push_back(copy(v.data[i])); return out; }
inline Location copy(thinkthen_location_v1 v) { Location out{}; out.file=copy(v.file); out.first_line=copy(v.first_line); out.last_line=copy(v.last_line); return out; }
inline MemberFailure copy(thinkthen_member_failure_v1 v) { MemberFailure out{}; out.failure_id=copy(v.failure_id); out.cause=copy(v.cause); return out; }
inline MemberSuccess copy(thinkthen_member_success_v1 v) { MemberSuccess out{}; out.answer_id=copy(v.answer_id); out.value=copy(v.value); out.answer=copy(v.answer); out.threshold=copy(v.threshold); return out; }
inline Member copy(thinkthen_member_v1 v) { Member out{}; out.name=copy(v.name); out.request=copy(v.request); out.question=copy(v.question); out.state=copy(v.state); switch(v.state) { case 1: out.data.success=copy(v.data.success); break; case 2: out.data.failure=copy(v.data.failure); break; default: throw std::runtime_error("invalid native thinkthen_member_v1 discriminator " + std::to_string(v.state)); } return out; }
inline MemberValue copy(thinkthen_member_value_v1 v) { MemberValue out{}; out.kind=copy(v.kind); switch(v.kind) { case 1: out.data.decide=copy(v.data.decide); break; case 2: out.data.choose=copy(v.data.choose); break; case 3: out.data.tag=copy(v.data.tag); break; case 4: out.data.score=copy(v.data.score); break; default: throw std::runtime_error("invalid native thinkthen_member_value_v1 discriminator " + std::to_string(v.kind)); } return out; }
inline std::vector<Member> copy(thinkthen_members_v1 v) { std::vector<Member> out; extent(v.len,sizeof(*v.data),v.data); out.reserve(v.len); for(size_t i=0;i<v.len;++i) out.push_back(copy(v.data[i])); return out; }
inline Meta copy(thinkthen_meta_v1 v) { Meta out{}; out.tool=copy(v.tool); out.question_sha256=copy(v.question_sha256); out.questions_sha256=copy(v.questions_sha256); out.url=copy(v.url); out.model=copy(v.model); out.usage=copy(v.usage); out.requests_sent=copy(v.requests_sent); out.cached=copy(v.cached); out.requests=copy(v.requests); out.failed_questions=copy(v.failed_questions); out.profile_warning=copy(v.profile_warning); out.batch_setting=copy(v.batch_setting); out.batch_warning=copy(v.batch_warning); out.context_sha256=copy(v.context_sha256); out.attempts=copy(v.attempts); out.origin=copy(v.origin); out.question_sources=copy(v.question_sources); out.observations=copy(v.observations); out.answered_by=copy(v.answered_by); return out; }
inline Name copy(thinkthen_name_v1 v) { Name out{}; out.start=copy(v.start); out.end=copy(v.end); out.kinds=copy(v.kinds); out.edges=copy(v.edges); return out; }
inline NamedAnswer copy(thinkthen_named_answer_v1 v) { NamedAnswer out{}; out.pick=copy(v.pick); out.probabilities=copy(v.probabilities); out.confidence=copy(v.confidence); return out; }
inline std::vector<Name> copy(thinkthen_names_v1 v) { std::vector<Name> out; extent(v.len,sizeof(*v.data),v.data); out.reserve(v.len); for(size_t i=0;i<v.len;++i) out.push_back(copy(v.data[i])); return out; }
inline std::vector<ObservationIdentity> copy(thinkthen_observation_identities_v1 v) { std::vector<ObservationIdentity> out; extent(v.len,sizeof(*v.data),v.data); out.reserve(v.len); for(size_t i=0;i<v.len;++i) out.push_back(copy(v.data[i])); return out; }
inline ObservationIdentity copy(thinkthen_observation_identity_v1 v) { ObservationIdentity out{}; out.kind=copy(v.kind); switch(v.kind) { case 1: out.data.observation_id=copy(v.data.observation_id); break; case 2: out.data.failure_id=copy(v.data.failure_id); break; default: throw std::runtime_error("invalid native thinkthen_observation_identity_v1 discriminator " + std::to_string(v.kind)); } return out; }
inline ObservationSuccess copy(thinkthen_observation_success_v1 v) { ObservationSuccess out{}; out.answer_id=copy(v.answer_id); out.observation_id=copy(v.observation_id); out.value=copy(v.value); out.probabilities=copy(v.probabilities); out.confidence=copy(v.confidence); return out; }
inline Observation copy(thinkthen_observation_v1 v) { Observation out{}; out.kind=copy(v.kind); switch(v.kind) { case 1: out.data.question=copy(v.data.question); break; case 2: out.data.row=copy(v.data.row); break; default: throw std::runtime_error("invalid native thinkthen_observation_v1 discriminator " + std::to_string(v.kind)); } return out; }
inline ObservedProbabilities copy(thinkthen_observed_probabilities_v1 v) { ObservedProbabilities out{}; out.kind=copy(v.kind); switch(v.kind) { case 1: out.data.yes=copy(v.data.yes); break; case 2: out.data.named=copy(v.data.named); break; default: throw std::runtime_error("invalid native thinkthen_observed_probabilities_v1 discriminator " + std::to_string(v.kind)); } return out; }
inline std::optional<Answer> copy(thinkthen_optional_answer_v1 v) { if(!present(v.present)) return {}; return copy(v.value); }
inline std::optional<std::vector<Attempt>> copy(thinkthen_optional_attempts_v1 v) { if(!present(v.present)) return {}; return copy(v.value); }
inline std::optional<Batch> copy(thinkthen_optional_batch_v1 v) { if(!present(v.present)) return {}; return copy(v.value); }
inline std::optional<BatchWarning> copy(thinkthen_optional_batch_warning_v1 v) { if(!present(v.present)) return {}; return copy(v.value); }
inline std::optional<Content> copy(thinkthen_optional_content_v1 v) { if(!present(v.present)) return {}; return copy(v.value); }
inline std::optional<uint32_t> copy(thinkthen_optional_discriminator_v1 v) { if(!present(v.present)) return {}; return copy(v.value); }
inline std::optional<double> copy(thinkthen_optional_double_v1 v) { if(!present(v.present)) return {}; return copy(v.value); }
inline std::optional<Endpoint> copy(thinkthen_optional_endpoint_v1 v) { if(!present(v.present)) return {}; return copy(v.value); }
inline std::optional<std::vector<EntityEdge>> copy(thinkthen_optional_entity_edges_v1 v) { if(!present(v.present)) return {}; return copy(v.value); }
inline std::optional<Error> copy(thinkthen_optional_error_v1 v) { if(!present(v.present)) return {}; return copy(v.value); }
inline std::optional<Facts> copy(thinkthen_optional_facts_v1 v) { if(!present(v.present)) return {}; return copy(v.value); }
inline std::optional<std::vector<ImageView>> copy(thinkthen_optional_image_views_v1 v) { if(!present(v.present)) return {}; return copy(v.value); }
inline std::optional<Location> copy(thinkthen_optional_location_v1 v) { if(!present(v.present)) return {}; return copy(v.value); }
inline std::optional<Meta> copy(thinkthen_optional_meta_v1 v) { if(!present(v.present)) return {}; return copy(v.value); }
inline std::optional<std::vector<Probability>> copy(thinkthen_optional_probabilities_v1 v) { if(!present(v.present)) return {}; return copy(v.value); }
inline std::optional<ProfileWarning> copy(thinkthen_optional_profile_warning_v1 v) { if(!present(v.present)) return {}; return copy(v.value); }
inline std::optional<QuestionView> copy(thinkthen_optional_question_v1 v) { if(!present(v.present)) return {}; return copy(v.value); }
inline std::optional<Rule> copy(thinkthen_optional_rule_v1 v) { if(!present(v.present)) return {}; return copy(v.value); }
inline std::optional<size_t> copy(thinkthen_optional_size_v1 v) { if(!present(v.present)) return {}; return copy(v.value); }
inline std::optional<std::vector<SourceEntityEdge>> copy(thinkthen_optional_source_entity_edges_v1 v) { if(!present(v.present)) return {}; return copy(v.value); }
inline std::optional<Stopped> copy(thinkthen_optional_stopped_v1 v) { if(!present(v.present)) return {}; return copy(v.value); }
inline std::optional<std::string> copy(thinkthen_optional_string_v1 v) { if(!present(v.present)) return {}; return copy(v.value); }
inline std::optional<uint16_t> copy(thinkthen_optional_u16_v1 v) { if(!present(v.present)) return {}; return copy(v.value); }
inline std::optional<uint64_t> copy(thinkthen_optional_u64_v1 v) { if(!present(v.present)) return {}; return copy(v.value); }
inline std::optional<Usage> copy(thinkthen_optional_usage_v1 v) { if(!present(v.present)) return {}; return copy(v.value); }
inline Pair copy(thinkthen_pair_v1 v) { Pair out{}; out.relation=copy(v.relation); out.source=copy(v.source); out.target=copy(v.target); out.probability=copy(v.probability); return out; }
inline std::vector<Pair> copy(thinkthen_pairs_v1 v) { std::vector<Pair> out; extent(v.len,sizeof(*v.data),v.data); out.reserve(v.len); for(size_t i=0;i<v.len;++i) out.push_back(copy(v.data[i])); return out; }
inline Piece copy(thinkthen_piece_v1 v) { Piece out{}; out.start=copy(v.start); out.end=copy(v.end); out.tags=copy(v.tags); return out; }
inline std::vector<Piece> copy(thinkthen_pieces_v1 v) { std::vector<Piece> out; extent(v.len,sizeof(*v.data),v.data); out.reserve(v.len); for(size_t i=0;i<v.len;++i) out.push_back(copy(v.data[i])); return out; }
inline Place copy(thinkthen_place_v1 v) { Place out{}; out.start=copy(v.start); out.end=copy(v.end); return out; }
inline std::vector<Probability> copy(thinkthen_probabilities_v1 v) { std::vector<Probability> out; extent(v.len,sizeof(*v.data),v.data); out.reserve(v.len); for(size_t i=0;i<v.len;++i) out.push_back(copy(v.data[i])); return out; }
inline Probability copy(thinkthen_probability_v1 v) { Probability out{}; out.name=copy(v.name); out.probability=copy(v.probability); return out; }
inline ProfileWarning copy(thinkthen_profile_warning_v1 v) { ProfileWarning out{}; out.tuned_for=copy(v.tuned_for); out.running=copy(v.running); return out; }
inline QuestionAuthor copy(thinkthen_question_author_v1 v) { QuestionAuthor out{}; out.name=copy(v.name); out.wording_version=copy(v.wording_version); out.item_schema=copy(v.item_schema); out.context_schema=copy(v.context_schema); return out; }
inline QuestionMember copy(thinkthen_question_member_v1 v) { QuestionMember out{}; out.name=copy(v.name); if(!v.question) throw std::runtime_error("missing native question"); out.question=std::make_shared<QuestionView>(copy(*v.question)); return out; }
inline std::vector<QuestionMember> copy(thinkthen_question_members_v1 v) { std::vector<QuestionMember> out; extent(v.len,sizeof(*v.data),v.data); out.reserve(v.len); for(size_t i=0;i<v.len;++i) out.push_back(copy(v.data[i])); return out; }
inline QuestionObservation copy(thinkthen_question_observation_v1 v) { QuestionObservation out{}; out.index=copy(v.index); out.member=copy(v.member); out.stage=copy(v.stage); out.position=copy(v.position); out.question_sha256=copy(v.question_sha256); out.model=copy(v.model); out.url=copy(v.url); out.requests=copy(v.requests); out.requests_sent=copy(v.requests_sent); out.cached=copy(v.cached); out.failed_questions=copy(v.failed_questions); out.usage=copy(v.usage); out.question_sources=copy(v.question_sources); out.state=copy(v.state); switch(v.state) { case 1: out.data.success=copy(v.data.success); break; case 2: out.data.failure=copy(v.data.failure); break; default: throw std::runtime_error("invalid native thinkthen_question_observation_v1 discriminator " + std::to_string(v.state)); } return out; }
inline QuestionSource copy(thinkthen_question_source_v1 v) { QuestionSource out{}; out.origin=copy(v.origin); out.answered_by=copy(v.answered_by); return out; }
inline std::vector<QuestionSource> copy(thinkthen_question_sources_v1 v) { std::vector<QuestionSource> out; extent(v.len,sizeof(*v.data),v.data); out.reserve(v.len); for(size_t i=0;i<v.len;++i) out.push_back(copy(v.data[i])); return out; }
inline QuestionView copy(thinkthen_question_view_v1 v) { QuestionView out{}; out.kind=copy(v.kind); out.text=copy(v.text); out.yes=copy(v.yes); out.no=copy(v.no); out.choices=copy(v.choices); out.threshold=copy(v.threshold); out.relation_threshold=copy(v.relation_threshold); out.model=copy(v.model); out.profile=copy(v.profile); out.batch=copy(v.batch); out.batch_max=copy(v.batch_max); out.none=copy(v.none); out.on=copy(v.on); out.members=copy(v.members); out.kinds=copy(v.kinds); out.relations=copy(v.relations); out.name_pointer=copy(v.name_pointer); out.kind_pointer=copy(v.kind_pointer); return out; }
inline RankView copy(thinkthen_rank_view_v1 v) { RankView out{}; out.common=copy(v.common); out.value=copy(v.value); out.question_name=copy(v.question_name); return out; }
inline RecognizeAnswer copy(thinkthen_recognize_answer_v1 v) { RecognizeAnswer out{}; out.pieces=copy(v.pieces); out.names=copy(v.names); out.pairs=copy(v.pairs); return out; }
inline RecognizeValue copy(thinkthen_recognize_value_v1 v) { RecognizeValue out{}; out.entities=copy(v.entities); out.relations=copy(v.relations); return out; }
inline RecognizeView copy(thinkthen_recognize_view_v1 v) { RecognizeView out{}; out.common=copy(v.common); out.value=copy(v.value); out.answer=copy(v.answer); return out; }
inline RelateView copy(thinkthen_relate_view_v1 v) { RelateView out{}; out.common=copy(v.common); out.value=copy(v.value); out.questions=copy(v.questions); return out; }
inline RelationAnswer copy(thinkthen_relation_answer_v1 v) { RelationAnswer out{}; out.relation=copy(v.relation); out.reads=copy(v.reads); out.method=copy(v.method); out.direction=copy(v.direction); out.source=copy(v.source); out.target=copy(v.target); out.request=copy(v.request); out.state=copy(v.state); switch(v.state) { case 1: out.data.success=copy(v.data.success); break; case 2: out.data.failure=copy(v.data.failure); break; default: throw std::runtime_error("invalid native thinkthen_relation_answer_v1 discriminator " + std::to_string(v.state)); } return out; }
inline std::vector<RelationAnswer> copy(thinkthen_relation_answers_v1 v) { std::vector<RelationAnswer> out; extent(v.len,sizeof(*v.data),v.data); out.reserve(v.len); for(size_t i=0;i<v.len;++i) out.push_back(copy(v.data[i])); return out; }
inline RelationSuccess copy(thinkthen_relation_success_v1 v) { RelationSuccess out{}; out.answer_id=copy(v.answer_id); out.probability=copy(v.probability); out.accepted=copy(v.accepted); return out; }
inline Relation copy(thinkthen_relation_v1 v) { Relation out{}; out.name=copy(v.name); out.source=copy(v.source); out.target=copy(v.target); out.reads=copy(v.reads); out.either=copy(v.either); out.single=copy(v.single); return out; }
inline std::vector<Relation> copy(thinkthen_relations_v1 v) { std::vector<Relation> out; extent(v.len,sizeof(*v.data),v.data); out.reserve(v.len); for(size_t i=0;i<v.len;++i) out.push_back(copy(v.data[i])); return out; }
inline ReportedUsage copy(thinkthen_reported_usage_v1 v) { ReportedUsage out{}; out.present=copy(v.present); out.input_tokens=copy(v.input_tokens); out.output_tokens=copy(v.output_tokens); return out; }
inline RowObservation copy(thinkthen_row_observation_v1 v) { RowObservation out{}; out.index=copy(v.index); out.function=copy(v.function); switch(v.function) { case 1: out.data.decide=copy(v.data.decide); break; case 2: out.data.choose=copy(v.data.choose); break; case 3: out.data.tag=copy(v.data.tag); break; case 4: out.data.score=copy(v.data.score); break; case 5: out.data.filter=copy(v.data.filter); break; case 6: out.data.rank=copy(v.data.rank); break; case 7: out.data.find=copy(v.data.find); break; case 8: out.data.annotate=copy(v.data.annotate); break; case 9: out.data.recognize=copy(v.data.recognize); break; case 10: out.data.relate=copy(v.data.relate); break; default: throw std::runtime_error("invalid native thinkthen_row_observation_v1 discriminator " + std::to_string(v.function)); } return out; }
inline Row copy(thinkthen_row_v1 v) { Row out{}; out.answer_id=copy(v.answer_id); out.input=copy(v.input); out.question=copy(v.question); out.answer=copy(v.answer); out.threshold=copy(v.threshold); out.position=copy(v.position); out.input_file=copy(v.input_file); out.meta=copy(v.meta); out.images=copy(v.images); return out; }
inline Rule copy(thinkthen_rule_v1 v) { Rule out{}; out.kind=copy(v.kind); out.low=copy(v.low); out.high=copy(v.high); return out; }
inline ScoreAnswer copy(thinkthen_score_answer_v1 v) { ScoreAnswer out{}; out.level=copy(v.level); out.probabilities=copy(v.probabilities); out.confidence=copy(v.confidence); return out; }
inline ScoreView copy(thinkthen_score_view_v1 v) { ScoreView out{}; out.common=copy(v.common); out.value=copy(v.value); return out; }
inline SourceDetail copy(thinkthen_source_detail_v1 v) { SourceDetail out{}; out.origin=copy(v.origin); out.answered_by=copy(v.answered_by); out.batch_size=copy(v.batch_size); return out; }
inline std::vector<SourceDetail> copy(thinkthen_source_details_v1 v) { std::vector<SourceDetail> out; extent(v.len,sizeof(*v.data),v.data); out.reserve(v.len); for(size_t i=0;i<v.len;++i) out.push_back(copy(v.data[i])); return out; }
inline SourceEdge copy(thinkthen_source_edge_v1 v) { SourceEdge out{}; out.relation=copy(v.relation); out.source=copy(v.source); out.target=copy(v.target); out.probability=copy(v.probability); out.either=copy(v.either); return out; }
inline std::vector<SourceEdge> copy(thinkthen_source_edges_v1 v) { std::vector<SourceEdge> out; extent(v.len,sizeof(*v.data),v.data); out.reserve(v.len); for(size_t i=0;i<v.len;++i) out.push_back(copy(v.data[i])); return out; }
inline SourceEndpoint copy(thinkthen_source_endpoint_v1 v) { SourceEndpoint out{}; out.ordinal=copy(v.ordinal); out.endpoint=copy(v.endpoint); out.record=copy(v.record); out.position=copy(v.position); return out; }
inline std::vector<SourceEntity> copy(thinkthen_source_entities_v1 v) { std::vector<SourceEntity> out; extent(v.len,sizeof(*v.data),v.data); out.reserve(v.len); for(size_t i=0;i<v.len;++i) out.push_back(copy(v.data[i])); return out; }
inline SourceEntityEdge copy(thinkthen_source_entity_edge_v1 v) { SourceEntityEdge out{}; out.relation=copy(v.relation); out.source=copy(v.source); out.target=copy(v.target); out.probability=copy(v.probability); out.either=copy(v.either); return out; }
inline std::vector<SourceEntityEdge> copy(thinkthen_source_entity_edges_v1 v) { std::vector<SourceEntityEdge> out; extent(v.len,sizeof(*v.data),v.data); out.reserve(v.len); for(size_t i=0;i<v.len;++i) out.push_back(copy(v.data[i])); return out; }
inline SourceEntity copy(thinkthen_source_entity_v1 v) { SourceEntity out{}; out.entity=copy(v.entity); out.position=copy(v.position); return out; }
inline SourceRecognition copy(thinkthen_source_recognition_v1 v) { SourceRecognition out{}; out.present=copy(v.present); out.entities=copy(v.entities); out.relations=copy(v.relations); return out; }
inline SourceRelations copy(thinkthen_source_relations_v1 v) { SourceRelations out{}; out.present=copy(v.present); out.edges=copy(v.edges); return out; }
inline Stopped copy(thinkthen_stopped_v1 v) { Stopped out{}; out.at=copy(v.at); out.cause=copy(v.cause); out.status=copy(v.status); out.retryable=copy(v.retryable); return out; }
inline std::vector<std::string> copy(thinkthen_strings_v1 v) { std::vector<std::string> out; extent(v.len,sizeof(*v.data),v.data); out.reserve(v.len); for(size_t i=0;i<v.len;++i) out.push_back(copy(v.data[i])); return out; }
inline Summary copy(thinkthen_summary_v1 v) { Summary out{}; out.state=copy(v.state); out.schema=copy(v.schema); out.answer_id=copy(v.answer_id); out.function=copy(v.function); out.count=copy(v.count); out.observation_count=copy(v.observation_count); out.meta=copy(v.meta); out.facts=copy(v.facts); out.attempts=copy(v.attempts); out.error=copy(v.error); return out; }
inline TagView copy(thinkthen_tag_view_v1 v) { TagView out{}; out.common=copy(v.common); out.value=copy(v.value); return out; }
inline Usage copy(thinkthen_usage_v1 v) { Usage out{}; out.input_tokens=copy(v.input_tokens); out.output_tokens=copy(v.output_tokens); return out; }
} // namespace tt::native
