#pragma once
#include "native_views.hpp"
#include <thread>
namespace tt::native {
using Engine = std::shared_ptr<thinkthen_engine>;
struct QuestionFree { void operator()(thinkthen_question *p) const { thinkthen_question_free(p); } };
struct SourceFree { void operator()(thinkthen_source *p) const { thinkthen_source_free(p); } };
struct ImageFree { void operator()(thinkthen_image *p) const { thinkthen_image_free(p); } };
using Question = std::unique_ptr<thinkthen_question,QuestionFree>;
using Source = std::unique_ptr<thinkthen_source,SourceFree>;
using Image = std::unique_ptr<thinkthen_image,ImageFree>;
inline thinkthen_string_v1 counted(const std::string& s) { return {s.data(),s.size()}; }
enum class FailureKind : int { usage=1,backend,deadline,local,cancelled,defect };
enum class Role : uint32_t { atomic=1,set,dynamicChoose,recognize,relate,rank,rankSet,find };
enum class Unit : uint32_t { line=1,window,file,imageFile,jsonl };
struct Result {
    Summary summary;
    std::vector<RowObservation> rows;
    std::vector<Observation> observations;
    std::vector<Details> details, observation_details;
    std::vector<QuestionAuthor> authors, observation_authors;
    std::vector<RecognitionTask> recognition_tasks;
    std::vector<std::vector<QuestionAuthor>> member_authors;
    std::vector<std::vector<RankView>> rank_members;
    std::vector<SourceRecognition> located_recognition;
    std::vector<SourceRelations> located_relations;
    std::vector<std::vector<Details>> rank_member_details{};
    const DecideView& decide(size_t i) const { return rows.at(i).data.decide.value(); }
    const ChooseView& choose(size_t i) const { return rows.at(i).data.choose.value(); }
    const TagView& tag(size_t i) const { return rows.at(i).data.tag.value(); }
    const ScoreView& score(size_t i) const { return rows.at(i).data.score.value(); }
    const FilterView& filter(size_t i) const { return rows.at(i).data.filter.value(); }
    const RankView& rank(size_t i) const { return rows.at(i).data.rank.value(); }
    const FindView& find(size_t i) const { return rows.at(i).data.find.value(); }
    const AnnotateView& annotate(size_t i) const { return rows.at(i).data.annotate.value(); }
    const RecognizeView& recognize(size_t i) const { return rows.at(i).data.recognize.value(); }
    const RelateView& relate(size_t i) const { return rows.at(i).data.relate.value(); }
};
inline void view_ok(int code) { if(code) throw std::runtime_error("native result accessor refused"); }
// Takes ownership immediately, including on a failed host allocation/conversion.
inline Result snapshot(thinkthen_result *raw) {
    std::unique_ptr<thinkthen_result,decltype(&thinkthen_result_free)> owner(raw,thinkthen_result_free);
    if(!raw) throw std::runtime_error("missing native result");
    thinkthen_summary_v1 summary{}; view_ok(thinkthen_result_summary(raw,&summary));
    Result out{}; out.summary=copy(summary);
    extent(summary.count,sizeof(RowObservation),raw); extent(summary.observation_count,sizeof(Observation),raw);
    for(size_t i=0;i<summary.observation_count;++i) {
        thinkthen_observation_v1 value{}; view_ok(thinkthen_result_observation(raw,i,&value));
        out.observations.push_back(copy(value));
        thinkthen_details_v1 details{}; view_ok(thinkthen_result_observation_details(raw,i,&details));
        out.observation_details.push_back(copy(details));
        thinkthen_question_author_v1 author{}; view_ok(thinkthen_result_observation_author(raw,i,&author));
        out.observation_authors.push_back(copy(author));
    }
    for(size_t i=0;i<summary.count;++i) {
        thinkthen_row_observation_v1 row{}; view_ok(thinkthen_result_row(raw,i,&row));
        out.rows.push_back(copy(row));
        thinkthen_recognition_task_v1 task{};
        if(row.function==9) view_ok(thinkthen_result_recognition_task_v1(raw,i,&task));
        out.recognition_tasks.push_back(copy(task));
        thinkthen_details_v1 details{}; view_ok(thinkthen_result_details(raw,i,&details)); out.details.push_back(copy(details));
        thinkthen_question_author_v1 author{}; view_ok(thinkthen_result_question_author(raw,i,&author)); out.authors.push_back(copy(author));
        out.member_authors.emplace_back(); out.rank_members.emplace_back(); out.rank_member_details.emplace_back();
        if(row.function==8) for(size_t j=0;j<row.data.annotate.answers.len;++j) {
            view_ok(thinkthen_result_member_author(raw,i,j,&author)); out.member_authors.back().push_back(copy(author));
        }
        if(row.function==6) {
            size_t count=0; view_ok(thinkthen_result_rank_member_count(raw,i,&count)); extent(count,sizeof(RankView),raw);
            for(size_t j=0;j<count;++j) { thinkthen_rank_view_v1 v{}; view_ok(thinkthen_result_rank_member(raw,i,j,&v)); out.rank_members.back().push_back(copy(v));
                view_ok(thinkthen_result_member_author(raw,i,j,&author)); out.member_authors.back().push_back(copy(author));
                view_ok(thinkthen_result_rank_member_details(raw,i,j,&details)); out.rank_member_details.back().push_back(copy(details)); }
        }
        thinkthen_source_recognition_v1 recognition{};
        if(row.function==9) view_ok(thinkthen_result_source_recognition(raw,i,&recognition));
        out.located_recognition.push_back(copy(recognition));
        thinkthen_source_relations_v1 relations{};
        if(row.function==10) view_ok(thinkthen_result_source_relations(raw,i,&relations));
        out.located_relations.push_back(copy(relations));
    }
    return out;
}
struct NativeFailure : std::runtime_error {
    Result snapshot;
    explicit NativeFailure(Result s):std::runtime_error(s.summary.error ? s.summary.error->message : "native failure snapshot missing"),snapshot(std::move(s)) {}
    int code() const { return snapshot.summary.error ? snapshot.summary.error->code : THINKTHEN_EDEFECT; }
    FailureKind kind() const { return static_cast<FailureKind>(code()); }
};
inline void checked(const thinkthen_engine *engine,int code) {
    if(!code) return;
    thinkthen_result *raw=nullptr; view_ok(thinkthen_error_complete(engine,&raw));
    throw NativeFailure(snapshot(raw));
}
inline Engine create(const std::string& settings) {
    if(settings.find('\0')!=std::string::npos) throw std::invalid_argument("interior NUL in settings");
    Engine engine(thinkthen_engine_new_with(settings.c_str()),thinkthen_engine_free);
    if(!engine) checked(nullptr,thinkthen_error_code(nullptr));
    return engine;
}
inline Question question(const Engine& engine,const thinkthen_question_spec_v1& spec,const thinkthen_question_author_v1 *author=nullptr,const thinkthen_recognition_task_v1 *task=nullptr) {
    thinkthen_question *raw=nullptr; checked(engine.get(),task?thinkthen_question_new_recognition_v1(engine.get(),&spec,author,task,&raw):thinkthen_question_new_authored(engine.get(),&spec,author,&raw)); return Question(raw);
}
inline Question parse(const Engine& engine,Role role,const std::string& json) {
    thinkthen_question *raw=nullptr; checked(engine.get(),thinkthen_question_parse(engine.get(),static_cast<uint32_t>(role),counted(json),&raw)); return Question(raw);
}
inline Question load(const Engine& engine,const std::string& path) {
    thinkthen_question *raw=nullptr; checked(engine.get(),thinkthen_question_load(engine.get(),counted(path),&raw)); return Question(raw);
}
inline Question named(const Engine& engine,Role role,const std::string& name) {
    thinkthen_question *raw=nullptr; checked(engine.get(),thinkthen_question_load_named(engine.get(),static_cast<uint32_t>(role),counted(name),&raw)); return Question(raw);
}
inline Question reference(const Engine& engine,Role role,const std::string& name) {
    thinkthen_question *raw=nullptr; checked(engine.get(),thinkthen_question_load_reference(engine.get(),static_cast<uint32_t>(role),counted(name),&raw)); return Question(raw);
}
inline QuestionAuthor author(const Question& q) { thinkthen_question_author_v1 v{}; view_ok(thinkthen_question_author(q.get(),&v)); return copy(v); }
inline Image image(const Engine& engine,const std::vector<uint8_t>& bytes,uint32_t media,const std::optional<std::string>& filename={}) {
    thinkthen_image *raw=nullptr; thinkthen_optional_string_v1 name{}; if(filename) name={1,counted(*filename)};
    checked(engine.get(),thinkthen_image_clone(engine.get(),bytes.data(),bytes.size(),media,name,&raw)); return Image(raw);
}
inline ImageView image_view(const Image& image) { thinkthen_image_view_v1 value{}; view_ok(thinkthen_image_view(image.get(),&value)); return copy(value); }
inline Source records(const Engine& engine,const std::vector<thinkthen_record_v1>& values) {
    thinkthen_source *raw=nullptr; checked(engine.get(),thinkthen_source_records(engine.get(),values.data(),values.size(),&raw)); return Source(raw);
}
inline Source files(const Engine& engine,const std::vector<std::string>& paths,Unit unit,size_t window=0,bool image_reader=false) {
    std::vector<thinkthen_string_v1> names; for(const auto& path:paths) names.push_back(counted(path));
    thinkthen_source_spec_v1 spec{{names.data(),names.size()},static_cast<uint32_t>(unit),window}; thinkthen_source *raw=nullptr;
    checked(engine.get(),image_reader?thinkthen_source_image_files(engine.get(),&spec,&raw):thinkthen_source_files(engine.get(),&spec,&raw)); return Source(raw);
}
inline thinkthen_controls_v1 controls() { thinkthen_controls_v1 c{}; c.deadline_ms=-1; c.surface={"cpp",3}; return c; }
#define TT_CALL(name) inline Result name(const Engine& e,const Question& q,const Source& s,thinkthen_controls_v1 c=controls()) { \
    thinkthen_result *r=nullptr; checked(e.get(),thinkthen_##name##_complete(e.get(),q.get(),s.get(),&c,&r)); return snapshot(r); }
TT_CALL(decide) TT_CALL(choose) TT_CALL(tag) TT_CALL(score) TT_CALL(filter)
TT_CALL(rank) TT_CALL(find) TT_CALL(annotate) TT_CALL(recognize) TT_CALL(relate)
#undef TT_CALL
class LazyBatch {
    Engine engine_;
    thinkthen_batch *handle_=nullptr;
    std::thread::id thread_=std::this_thread::get_id();
    void thread() const { if(thread_!=std::this_thread::get_id()) throw std::logic_error("batch belongs to its creating thread"); }
public:
    using Start=int(*)(const thinkthen_engine*,const thinkthen_question*,const thinkthen_source*,const thinkthen_controls_v1*,thinkthen_batch**);
    LazyBatch(const Engine& e,const Question& q,const Source& s,thinkthen_controls_v1 c,Start start):engine_(e) { checked(e.get(),start(e.get(),q.get(),s.get(),&c,&handle_)); }
    LazyBatch(const LazyBatch&)=delete; Batch& operator=(const LazyBatch&)=delete;
    ~LazyBatch() { if(thread_!=std::this_thread::get_id()) std::terminate(); thinkthen_batch_free(handle_); }
    std::optional<Result> next() { thread(); thinkthen_result *r=nullptr; checked(engine_.get(),thinkthen_batch_next(handle_,&r)); return r?std::optional<Result>(snapshot(r)):std::nullopt; }
    Result facts() { thread(); thinkthen_result *r=nullptr; checked(engine_.get(),thinkthen_batch_facts(handle_,&r)); return snapshot(r); }
};
#define TT_START(name) inline LazyBatch name##_batch(const Engine& e,const Question& q,const Source& s,thinkthen_controls_v1 c=controls()) { return LazyBatch(e,q,s,c,thinkthen_##name##_batch_start); }
TT_START(decide) TT_START(choose) TT_START(tag) TT_START(score) TT_START(filter) TT_START(annotate)
#undef TT_START
} // namespace tt::native
