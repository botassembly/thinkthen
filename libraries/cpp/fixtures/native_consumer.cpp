#include "native_views.hpp"
#include <fstream>
#include <iostream>
#include <thread>
using tt::Json;
namespace n=tt::native;
static std::string read(const std::string& path) { std::ifstream f(path,std::ios::binary); if(!f) throw std::runtime_error("fixture file missing"); return std::string(std::istreambuf_iterator<char>(f),{}); }
static std::string str(const Json& v) { return v.get<std::string>(); }
static bool flag(const Json& v,const std::string& key) { return v.contains(key) && !v.at(key).is_null() && v.at(key).get<bool>(); }
static thinkthen_content_v1 content(const std::string& bytes,bool text) { return {static_cast<uint32_t>(text?1:2),n::counted(bytes)}; }
static void selected(const n::Result& r,const std::string& verb) {
    for(size_t i=0;i<r.rows.size();++i) {
#define SELECT(name) if(verb==#name) { (void)r.name(i); continue; }
        SELECT(decide) SELECT(choose) SELECT(tag) SELECT(score) SELECT(filter)
        SELECT(rank) SELECT(find) SELECT(annotate) SELECT(recognize) SELECT(relate)
#undef SELECT
        throw std::runtime_error("fixture function missing");
    }
}
int main(int argc,char **argv) {
    if(argc!=3) return 2;
    std::thread cancellation;
    auto token=std::unique_ptr<thinkthen_cancel_token,decltype(&thinkthen_cancel_token_free)>(thinkthen_cancel_token_new(),thinkthen_cancel_token_free);
    try {
        const auto v=Json::parse(read(argv[1])); const auto settings=Json::parse(argv[2]);
        if(str(settings.at("base_url")).rfind("http://127.0.0.1:",0)!=0) return 3;
        auto e=n::create(argv[2]); const auto verb=str(v.at("verb"));
        const auto role=static_cast<n::Role>(v.at("role").get<double>());
        n::Question q;
        if(v.contains("find_none") && flag(v,"find_none")) {
            thinkthen_question_spec_v1 spec{}; spec.kind=7; spec.none=1;
            const auto text=str(v.at("find_text")); spec.text=content(text,flag(v,"find_text_literal"));
            thinkthen_question_author_v1 author{};
            std::string name;
            if(v.contains("metadata")) { name=str(v.at("metadata").at("name")); author.name={1,n::counted(name)}; author.wording_version={1,static_cast<uint64_t>(v.at("metadata").at("wording_version").get<double>())}; }
            q=n::question(e,spec,&author);
        } else if(v.contains("loader") && !v.at("loader").is_null()) {
            const auto loader=str(v.at("loader")), reference=str(v.at("reference"));
            if(loader=="load" || loader=="file") q=n::load(e,reference);
            else if(loader=="load_named" || loader=="named") q=n::named(e,role,reference);
            else q=n::reference(e,role,reference);
        } else if(v.contains("question_form") && !v.at("question_form").is_null() && str(v.at("question_form"))=="file") q=n::load(e,"fixture-question.json");
        else q=n::parse(e,role,str(v.at("question_json")));
        auto ctl=n::controls(); ctl.attempts=1;
        if(v.contains("operation") && !v.at("operation").is_null()) {
            const auto& op=v.at("operation"); if(op.contains("injection")) {
                const auto injection=str(op.at("injection"));
                if(injection=="cancel_token") { thinkthen_cancel(token.get()); ctl.cancel=token.get(); }
                if(injection=="expired_deadline") ctl.deadline_ms=0;
            }
        }
        std::vector<n::Image> owners; std::vector<const thinkthen_image*> images;
        for(const auto& hex:v.at("image_data")) {
            const auto s=str(hex); std::vector<uint8_t> bytes;
            for(size_t i=0;i<s.size();i+=2) bytes.push_back(static_cast<uint8_t>(std::stoul(s.substr(i,2),nullptr,16)));
            owners.push_back(n::image(e,bytes,static_cast<uint32_t>(v.at("media_code").get<double>()))); images.push_back(owners.back().get());
            auto copied=n::image_view(owners.back()); if(copied.bytes!=bytes) throw std::runtime_error("image copy changed bytes");
        }
        n::Source source;
        std::string shared;
        if(v.contains("shared_context") && !v.at("shared_context").is_null()) {
            const auto& context=v.at("shared_context"); shared=context.is_string()?str(context):context.dump(); ctl.context={1,content(shared,context.is_string())};
        }
        if(v.contains("paths") && !v.at("paths").is_null()) {
            std::vector<std::string> paths; for(const auto& p:v.at("paths")) paths.push_back(str(p));
            source=n::files(e,paths,static_cast<n::Unit>(v.at("source_unit").get<double>()),v.contains("window")?v.at("window").get<double>():0,flag(v,"image_reader"));
        } else {
            std::vector<thinkthen_record_v1> records; std::vector<std::string> original,contexts,option_names;
            const auto& items=v.at("items"); original.reserve(static_cast<size_t>(std::distance(items.begin(),items.end()))); contexts.reserve(static_cast<size_t>(std::distance(items.begin(),items.end())));
            std::vector<std::vector<thinkthen_choice_v1>> choices(static_cast<size_t>(std::distance(items.begin(),items.end())));
            if(v.contains("candidate_orders")) for(const auto& order:v.at("candidate_orders")) for(const auto& option:order) option_names.push_back(str(option));
            size_t oi=0;
            for(size_t i=0;i<static_cast<size_t>(std::distance(items.begin(),items.end()));++i) {
                thinkthen_record_v1 row{};
                if(!flag(v,"image_only")) {
                    original.push_back(flag(v,"caption_files")?read("caption-"+std::to_string(i)+".txt"):flag(v,"text")&&items.at(i).is_string()?str(items.at(i)):items.at(i).dump());
                    row.original={1,content(original.back(),flag(v,"text")&&items.at(i).is_string())};
                }
                if(flag(v,"context_present")) { const auto& c=v.at("context"); contexts.push_back(c.is_string()?str(c):c.dump()); row.context={1,content(contexts.back(),c.is_string())}; }
                if(v.contains("candidate_orders")) { for(const auto& unused:v.at("candidate_orders").at(i)) { (void)unused; thinkthen_choice_v1 option{}; option.name=n::counted(option_names.at(oi++)); choices[i].push_back(option); } row.options={choices[i].data(),choices[i].size()}; }
                row.images={images.data(),images.size()}; records.push_back(row);
            }
            source=n::records(e,records);
        }
        if(flag(v,"held_cancel")) { ctl.cancel=token.get(); cancellation=std::thread([&]{ if(std::cin.get()!='!') std::terminate(); thinkthen_cancel(token.get()); std::cout<<"cancel-fired\n"<<std::flush; }); }
        if(flag(v,"incremental")) {
#define BATCH(name) if(verb==#name) { auto batch=n::name##_batch(e,q,source,ctl); q.reset(); source.reset(); owners.clear(); while(auto row=batch.next()) { selected(*row,verb); std::cout<<fixture::encode(*row).dump()<<'\n'; } std::cout<<fixture::encode(batch.facts()).dump()<<'\n'; }
            BATCH(decide) else BATCH(choose) else BATCH(tag) else BATCH(score) else BATCH(filter) else BATCH(annotate)
#undef BATCH
            else throw std::runtime_error("fixture has no batch function");
        } else {
            n::Result r;
#define CALL(name) if(verb==#name) r=n::name(e,q,source,ctl);
            CALL(decide) else CALL(choose) else CALL(tag) else CALL(score) else CALL(filter)
            else CALL(rank) else CALL(find) else CALL(annotate) else CALL(recognize) else CALL(relate)
#undef CALL
            else throw std::runtime_error("fixture function missing");
            q.reset(); source.reset(); owners.clear(); e.reset(); selected(r,verb); std::cout<<fixture::encode(r).dump()<<'\n';
        }
    } catch(const n::NativeFailure& f) { if(cancellation.joinable()) cancellation.join(); std::cout<<fixture::encode(f.snapshot).dump()<<'\n'; return 0; }
    catch(const std::exception& f) { if(cancellation.joinable()) cancellation.join(); std::cerr<<f.what()<<'\n'; return 1; }
    if(cancellation.joinable()) cancellation.join();
}
