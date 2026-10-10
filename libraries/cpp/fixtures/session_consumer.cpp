#include <thinkthen/client.hpp>
#include <fstream>
#include <iostream>
using namespace tt;
using namespace tt::inputs;
static std::string text(const Json& value) { return value.get<std::string>(); }
static RequestQuestion question(const Json& v) {
    auto kind=text(v.at("kind"));
    if(kind=="definition") return RequestQuestionDefinition().set_value(RequestDefinition::from_document(v.at("value")));
    if(kind=="name") return RequestQuestionName().set_name(text(v.at("name")));
    if(kind=="reference") return RequestQuestionReference().set_reference(text(v.at("reference")));
    return RequestQuestionFile().set_path(text(v.at("path")));
}
static RequestInput input(const Json& v) {
    auto kind=text(v.at("kind"));
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
        std::ifstream file(argv[1]); auto v=Json::parse(file);
        Client client(Json::parse(argv[2]).object());
        auto q=question(v.at("question"));auto in=input(v.at("input"));
        auto opts=v.at("options");RequestOptions options;options.set_attempts(true);
        if(opts.contains("deadline_ms")) options.set_deadline_ms(opts.at("deadline_ms").get<int64_t>());
        if(opts.contains("field")) { std::vector<std::string> fields; for(const auto& field:opts.at("field")) fields.push_back(text(field)); options.set_field(fields); }
        if(opts.contains("context_field")) options.set_context_field(text(opts.at("context_field")));
        if(opts.contains("none")) options.set_none(opts.at("none").get<bool>());
        if(opts.contains("context")) options.set_context(text(opts.at("context")));
        auto verb=text(v.at("verb"));
        auto start=[&]() -> Call {
#define NAMED(name) if(verb==#name) return client.name(q,in,options);
            NAMED(decide) NAMED(choose) NAMED(tag) NAMED(score) NAMED(filter)
            NAMED(rank) NAMED(find) NAMED(annotate) NAMED(recognize) NAMED(relate)
#undef NAMED
            throw std::invalid_argument("unknown fixture function");
        };
        auto call=start();
        if(v.at("cancel").get<bool>()) call.cancel();
        call.finish();
        if(v.at("held_cancel").get<bool>()) { if(std::cin.get()!='!') return 3; call.cancel(); std::cout<<"cancel-fired\n"<<std::flush; }
        Json::Array packets;
        try { for(const auto& packet:call.collect()) packets.push_back(packet.document()); }
        catch(const SessionFailure& error) {
            for(const auto& packet:error.packets) packets.push_back(packet.document());
            packets.push_back(error.terminal.document());
        }
        client.close();call.close();
        std::cout<<Json{{"packets",packets}}.dump()<<'\n';
    } catch(const NativeFailure& error) {
        std::cout<<Json{{"admission",Json{{"code",static_cast<int>(error.kind)},{"message",std::string(error.what())}}}}.dump()<<'\n';
    } catch(const std::exception& error) { std::cerr<<error.what()<<'\n';return 1; }
}
