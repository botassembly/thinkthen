#include <thinkthen/client.hpp>
#include <fstream>
#include <iostream>
#include <thread>
#include <mutex>
#include <chrono>
#include <cstdlib>
static std::mutex measurement_mutex;
static void phase(const std::string& label) {
 std::lock_guard<std::mutex> guard(measurement_mutex);
 std::ifstream status("/proc/self/status"); std::string line;
 std::cerr << "phase=" << label << " monotonic_ms=" << std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::steady_clock::now().time_since_epoch()).count();
 while(std::getline(status,line)) if(line.rfind("VmRSS:",0)==0 || line.rfind("VmHWM:",0)==0) std::cerr << " " << line;
 std::cerr << "\n";
}
using namespace tt;
using namespace tt::inputs;
static std::string text(const Data& value) { return value.get<std::string>(); }
static RequestQuestion question(const Data& v) {
    auto kind=text(v.at("kind"));
    if(kind=="definition") return RequestQuestionDefinition().set_value(::tt::results::detail::Loader::input<RequestDefinition>(v.at("value")));
    if(kind=="name") return RequestQuestionName().set_name(text(v.at("name")));
    if(kind=="reference") return RequestQuestionReference().set_reference(text(v.at("reference")));
    return RequestQuestionFile().set_path(text(v.at("path")));
}
static RequestInput input(const Data& v) {
    auto kind=text(v.at("kind"));
    if(kind=="feed") return RequestInputFeed().set_name(text(v.at("name")));
    if(kind=="source") {
        auto source=v.at("source"); std::vector<std::string> paths;
        for(const auto& path:source.at("paths")) paths.push_back(text(path));
        RequestSource result; result.set_paths(paths);
        auto reading=source.at("reading"); RequestReader reader;
        reader.set_unit(SourceUnit(text(reading.at("unit"))));
        if(reading.contains("window")) reader.set_window(reading.at("window").get<uint64_t>());
        result.set_reading(reader);
        if(source.contains("framing")) result.set_framing(RequestFraming(text(source.at("framing"))));
        if(source.contains("media")) result.set_media(ReaderMedia(text(source.at("media"))));
        return RequestInputSource().set_source(result);
    }
    std::vector<RequestItem> items;
    for(const auto& value:v.at("items")) {
        RequestItem item;
        if(value.contains("original")) {
            auto original=value.at("original");
            if(text(original.at("kind"))=="text") item.set_original(RequestOriginalText().set_text(text(original.at("text"))));
            else item.set_original(RequestOriginalJson().set_value(original.at("value")));
        }
        if(value.contains("context")) {
            auto context=value.at("context");
            item.set_context(context.is_string() ? ContextSchema(text(context)) : ContextSchema(context));
        }
        if(value.contains("images")) {
            std::vector<RequestImage> images;
            for(const auto& image:value.at("images")) {
                if(text(image.at("kind"))=="bytes") images.emplace_back(RequestImageBytes().set_bytes(text(image.at("bytes"))).set_media(ImageMedia(text(image.at("media")))));
                else images.emplace_back(RequestImageFile().set_path(text(image.at("path"))).set_media(ImageMedia(text(image.at("media")))));
            }
            item.set_images(images);
        }
        if(value.contains("options")) {
            std::vector<OptionSchema> options;
            for(const auto& option:value.at("options")) options.push_back(OptionSchema().set_name(text(option.at("name"))));
            item.set_options(options);
        }
        items.push_back(item);
    }
    if(kind=="units") return RequestInputUnits().set_items(items);
    if(kind=="entities") return RequestInputEntities().set_items(items);
    return RequestInputRecords().set_items(items);
}
int main(int argc,char** argv) {
    if(argc!=3) return 2;
    try {
        phase("start"); std::ifstream file(argv[1]); auto v=Data::read(file); phase("input_parsed");
        Client client(::tt::results::detail::Loader::input<EngineSettings>(::tt::detail::parse_original(argv[2])));
        auto q=question(v.at("question"));auto in=input(v.at("input"));
        auto opts=v.at("options");RequestOptions options;options.set_attempts(true);
        if(opts.contains("deadline_ms")) options.set_deadline_ms(opts.at("deadline_ms").get<int64_t>());
        if(opts.contains("field")) { std::vector<std::string> fields; for(const auto& field:opts.at("field")) fields.push_back(text(field)); options.set_field(fields); }
        if(opts.contains("context_field")) options.set_context_field(text(opts.at("context_field")));
        if(opts.contains("none")) options.set_none(opts.at("none").get<bool>());
        if(opts.contains("context")) options.set_context(text(opts.at("context")));
        auto verb=text(v.at("verb"));
        const auto& data=::tt::results::detail::document(in);
        auto start=[&]() -> Call {
#define SHAPED(name,type) return client.name(q,::tt::results::detail::Loader::input<type>(data),options);
#define NAMED(name) if(verb==#name) { \
    auto kind=data.at("kind").get<std::string>(); \
    if(kind=="records") { SHAPED(name,RequestInputRecords) } \
    if(kind=="source") { SHAPED(name,RequestInputSource) } \
    if(kind=="units") { SHAPED(name,RequestInputUnits) } \
    if(kind=="entities") { SHAPED(name,RequestInputEntities) } \
    if(kind=="feed") { SHAPED(name,RequestInputFeed) } \
    throw std::invalid_argument("unknown fixture input"); }
            NAMED(decide) NAMED(choose) NAMED(tag) NAMED(score)
            NAMED(annotate) NAMED(recognize)
            if(verb=="filter") return client.filter(q,in,options);
            if(verb=="rank") return client.rank(q,in,options);
            if(verb=="find") return client.find(q,in,options);
            if(verb=="relate") return client.relate(q,in,options);
#undef SHAPED
#undef NAMED
            throw std::invalid_argument("unknown fixture function");
        };
        if(v.contains("call_result_contract") && v.at("call_result_contract").get<bool>()) {
            const auto& original=data.at("items").at(0).at("original");
            auto emit=[&](auto call) {
                Data::Array packets;
                auto result=[&] {
                    auto owned=std::move(call); owned.finish();
                    if(verb=="decide") owned.collect();
                    auto result=owned.result(); auto repeated=owned.result();
                    auto copied=owned.collect(); auto collected=owned.collect();
                    if(::tt::results::detail::encode(result.value)!=::tt::results::detail::encode(repeated.value) ||
                       ::tt::results::detail::encode(result.probability)!=::tt::results::detail::encode(repeated.probability) ||
                       ::tt::results::detail::encode(copied)!=::tt::results::detail::encode(collected) ||
                       *result.facts.requests_sent().value!=1) throw std::logic_error("result cache changed the completed call");
                    for(const auto& packet:copied) packets.push_back(::tt::results::detail::document(packet));
                    client.close(); return result;
                }(); // The operation and native session are destroyed before these fields are read.
                auto fields=Data{{"value",::tt::results::detail::encode(result.value)},{"probability",::tt::results::detail::encode(result.probability)},{"results",::tt::results::detail::encode(result.results)},{"facts",::tt::results::detail::encode(result.facts)}};
                std::cout<<::tt::detail::serialize(Data{{"packets",packets},{"call_result",fields}})<<'\n';
            };
#define CONTRACT(name) if(verb==#name) { \
    if(text(original.at("kind"))=="text") emit(client.name(q,RequestInputText().set_text(text(original.at("text"))),options)); \
    else emit(client.name(q,RequestInputJson().set_value(original.at("value")),options)); \
    return 0; }
            CONTRACT(decide) CONTRACT(choose)
#undef CONTRACT
            throw std::invalid_argument("unsupported result contract fixture");
        }
        auto call=start(); phase("call_started");
        if(v.at("cancel").get<bool>()) {
            call.cancel();
            auto item=RequestSessionDescriptor().set_item(RequestItem().set_original(RequestOriginalText().set_text("closed-push")));
            if(call.try_push(item) || !call.input_closed() || call.try_push(item)) throw std::logic_error("cancelled push did not stop");
            auto moved=std::move(call);
            if(!moved.input_closed() || !call.input_closed() || moved.try_push(item)) throw std::logic_error("moved closed intake reopened");
            call=std::move(moved);
        }
        Data::Array packets;
        try {
            auto read=[&] {
                auto collected=call.collect(); phase("collected_cache_and_copy");
                for(const auto& packet:collected) packets.push_back(::tt::results::detail::document(packet));
                phase("documents_and_collected_copy");
            };
            bool interleaved=std::getenv("MEASUREMENT_READ_WHILE_FEEDING")!=nullptr;
            std::thread reader; std::exception_ptr reader_error;
            if(interleaved) reader=std::thread([&] { try { read(); } catch(...) { reader_error=std::current_exception(); } });
            size_t fed=0;
            if(v.contains("feed_items")) for(const auto& item:v.at("feed_items")) {
                if(call.input_closed()) break;
                auto next=RequestSessionDescriptor().set_item(::tt::results::detail::Loader::input<RequestItem>(item));
                if(!call.push(next)) break;
                if(++fed%25000==0) phase("fed_"+std::to_string(fed));
            }
            phase("feed_complete"); call.finish(); phase("finished_before_join_or_read");
            if(interleaved) { reader.join(); if(reader_error) std::rethrow_exception(reader_error); }
            else read();
            phase("documents_collected_copy_freed");
        }
        catch(const ::tt::Failure& error) {
            if(!error.terminal) throw;
            for(const auto& packet:error.packets) packets.push_back(::tt::results::detail::document(packet));
            packets.push_back(::tt::results::detail::document(*error.terminal));
        }
        phase("before_close"); client.close();call.close(); phase("after_close");
        { auto output=Data{{"packets",packets}}; phase("output_document_copy"); auto serialized=::tt::detail::serialize(output); phase("serialized_string"); std::cout<<serialized<<'\n'; phase("output_written"); }
        phase("output_temporary_freed");
    } catch(const ::tt::Failure& error) {
        std::cout<<::tt::detail::serialize(Data{{"admission",Data{{"code",static_cast<int>(error.kind)},{"message",std::string(error.what())}}}})<<'\n';
    } catch(const std::exception& error) { std::cerr<<error.what()<<'\n';return 1; }
}
