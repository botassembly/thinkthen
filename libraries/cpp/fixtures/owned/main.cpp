#include <thinkthen/client.hpp>
#include <fstream>
#include <clocale>
#include <locale>
#include <iostream>
using namespace tt;
using namespace tt::inputs;
class CommaDecimal : public std::numpunct<char> {
    char do_decimal_point() const override { return ','; }
};
class CallerLocale {
    std::locale previous = std::locale();
    std::string numeric = std::setlocale(LC_NUMERIC, nullptr);
public:
    CallerLocale() {
        std::locale::global(std::locale(previous, new CommaDecimal));
        // Use an installed C locale when available; the facet always exercises writing.
        for (const char* name : {"de_DE.UTF-8", "fr_FR.UTF-8", "de_DE.utf8", "fr_FR.utf8"})
            if (std::setlocale(LC_NUMERIC, name)) break;
    }
    ~CallerLocale() { std::locale::global(previous); std::setlocale(LC_NUMERIC, numeric.c_str()); }
};
static Json read(const char* path) { std::ifstream file(path); return Json::parse(file); }
static RequestItem item(const Json& value) { return RequestItem().set_original(RequestOriginalJson().set_value(value)); }
static RequestInput evidence(const std::string& verb,const Json& original) {
    if(verb=="filter" || verb=="rank") return RequestInputRecords().set_items({item(original)});
    if(verb=="find") { std::vector<RequestItem> items; for(const auto& value:original) items.push_back(item(value)); return RequestInputUnits().set_items(items); }
    if(verb=="relate") { std::vector<RequestItem> items; for(const auto& value:original) items.push_back(item(value)); return RequestInputEntities().set_items(items); }
    return RequestInputText().set_text(original.get<std::string>());
}
static Call named(Client& client,const std::string& verb,const RequestQuestion& question,const RequestInput& input) {
#define NAMED(name) if(verb==#name) return client.name(question,input);
    NAMED(decide) NAMED(choose) NAMED(tag) NAMED(score) NAMED(filter)
    NAMED(rank) NAMED(find) NAMED(annotate) NAMED(recognize) NAMED(relate)
#undef NAMED
    throw std::invalid_argument("unknown fixture function");
}
int main(int argc,char** argv) {
    try {
        if(argc!=4) return 2;
        if(std::string(argv[1])=="usage-written" || std::string(argv[1])=="usage-failed") {
            Client client(Json::Object{{"cache",false}}); auto call=client.decide(RequestQuestionText().set_text("Is it?"),RequestInputText().set_text("yes"));
            call.finish(); auto packets=call.collect(); auto earlier=packets.back().document().dump();
            bool answered=false;
            for(const auto& packet:packets) if(auto row=packet.as_SessionPacketDecideRow()) {
                if(!row->value().value->value().value->value().get<bool>()) return 30;
                answered=true;
            }
            if(!answered) return 30;
            auto terminal=packets.back().as_SessionPacketTerminal();
            if(!terminal || *terminal->facts().value->requests_sent().value!=1) return 31;
            const bool failed=std::string(argv[1])=="usage-failed";
            auto observed=client.usage_persistence();
            if(failed && (observed.state!=UsagePersistenceState::pending || observed.advice)) return 32;
            auto finished=client.finish_usage_status();
            const auto expected=failed ? UsagePersistenceState::failed : UsagePersistenceState::written;
            if(finished.state!=expected || finished.advice.has_value()!=failed) return 33;
            if(failed && *finished.advice!="check the usage folder permissions and free space") return 34;
            if(client.usage_persistence().state!=expected || client.finish_usage_status().state!=expected) return 35;
            if(packets.back().document().dump()!=earlier) return 36;
            client.close();
            try { client.usage_persistence(); return 37; } catch(const std::logic_error&) {}
            try { client.finish_usage_status(); return 38; } catch(const std::logic_error&) {}
            if(finished.state!=expected) return 39;
            std::cout<<"usage-status-pass requests=1\n"; return 0;
        }
        if(std::string(argv[1])=="held") {
            Client client;
            {
            auto call=client.decide(RequestQuestionText().set_text("Is it?"),RequestInputText().set_text("hold-cpp-owned"));
            call.finish(); std::cout<<"started\n"<<std::flush;
            if(std::cin.get()!='!') return 8;
            // Host computation progresses while the real provider is still held.
            unsigned sum=0; for(unsigned i=0;i<100;++i) sum+=i;
            if(sum!=4950) return 9;
            std::cout<<"host-progress\n"<<std::flush;
            client.close(); call.cancel();
            } // Destructor releases the session before the provider is released.
            std::cout<<"cleaned\n"<<std::flush; return 0;
        }
        if(std::string(argv[1])=="values") {
            using namespace tt::results;
            Facts absent(Json::Object{});
            Facts null(Json{{"model",nullptr},{"held_model_mismatch",false},{"input_tokens",uint64_t(-1)},{"future",Json{{"n",uint64_t(-1)}}}});
            if(absent.model().state!=PresenceState::absent || null.model().state!=PresenceState::null) return 10;
            if(null.held_model_mismatch().state!=PresenceState::value || *null.held_model_mismatch().value) return 11;
            if(*null.input_tokens().value!=UINT64_MAX || null.document().at("future").at("n").dump()!=std::to_string(UINT64_MAX)) return 12;
            if(Json::parse(null.document().dump()).at("input_tokens").dump()!=std::to_string(UINT64_MAX)) return 13;
            if(Json(5)!=Json::parse("5") || Json(UINT64_MAX)==Json(static_cast<double>(UINT64_MAX))) return 21;
            if(Json::parse("-9223372036854775808").dump()!="-9223372036854775808") return 22;
            SessionPacket escaped(Json::parse(R"({"kind":"termin\u0061l","facts":{},"future":false})"));
            if(!escaped.as_SessionPacketTerminal() || escaped.document().at("future").get<bool>()) return 14;
            std::cout<<"values-pass\n"; return 0;
        }
        std::string verb=argv[1];
        const std::locale originalLocale;
        const std::string originalNumeric=std::setlocale(LC_NUMERIC,nullptr);
        std::optional<CallerLocale> locale;
        if(verb=="locale") locale.emplace();
        Client client;
        if(verb=="locale") {
            Json original{{"nested",Json::Array{Json{{"number",0.5}},1.25}}};
            auto input=RequestInputRecords().set_items({item(original)});
            auto call=client.decide(RequestQuestionText().set_text("Is it?"),input); call.finish();
            auto packets=call.collect(); client.close(); call.close();
            bool answered=false;
            for(const auto& packet:packets) {
                if(auto row=packet.as_SessionPacketDecideRow()) {
                    if(row->value().value->document().at("input")!=original) return 40;
                    if(!row->value().value->value().value->value().get<bool>()) return 41;
                    auto answer=row->value().value->answer().value->as_AnswerYesNo();
                    if(!answer || *answer->probability().value!=0.9) return 45;
                    answered=true;
                }
                if(Json::parse(packet.document().dump())!=packet.document()) return 42;
                std::cout<<packet.document().dump()<<'\n';
            }
            locale.reset();
            if(std::locale()!=originalLocale || originalNumeric!=std::setlocale(LC_NUMERIC,nullptr)) return 44;
            return answered && packets.back().as_SessionPacketTerminal() ? 0:43;
        }
        if(verb=="failure") {
            auto call=client.decide(RequestQuestionText().set_text("Is it?"),RequestInputText().set_text("backend-failure")); call.finish();
            try { call.collect(); return 15; }
            catch(const SessionFailure& error) {
                if(error.failure.error().value->kind().value->value()!="backend" || error.failure.error().value->retryable().value.value()) return 16;
                if(error.failure.facts().state!=results::PresenceState::value || *error.failure.facts().value->requests_sent().value!=1) return 17;
                if(error.terminal.facts().state!=results::PresenceState::value) return 20;
                std::cout<<"failure-pass\n"; return 0;
            }
        }
        if(verb=="invalid" || verb=="image") {
            try {
                if(verb=="invalid") client.decide(RequestQuestionText(),RequestInputText().set_text("unreachable"));
                else client.tag(RequestQuestionFile().set_path(argv[2]),RequestInputText().set_text("unreachable").set_images({RequestImageBytes().set_media(ImageMedia("image/png")).set_bytes("aGVsbG8=")}));
                return 18;
            } catch(const NativeFailure& error) { if(error.kind!=NativeFailureKind::usage) return 19; std::cout<<"admission-pass\n"; return 0; }
        }
        auto question=RequestQuestionFile().set_path(argv[2]);
        auto input=verb=="files" ? RequestInput(RequestInputSource().set_source(RequestSource().set_paths({argv[3]}).set_reading(RequestReader().set_unit(SourceUnit("line"))))) : evidence(verb,read(argv[3]));
        auto call=named(client,verb=="files" ? "decide":verb=="annotate-null" ? "annotate":verb,question,input);
        client.close(); call.finish();
        auto packets=call.collect(); call.close();
        bool terminal=false, answer=false;
        for(const auto& packet:packets) {
            if(auto value=packet.as_SessionPacketTerminal()) {
                terminal=true;
                if(value->failure().state==results::PresenceState::value) return 3;
                if(value->facts().state!=results::PresenceState::value) return 4;
            }
#define ANSWER(name) if(packet.as_SessionPacket##name()) answer=true;
            ANSWER(DecideRow) ANSWER(ChooseRow) ANSWER(TagRow) ANSWER(ScoreRow)
            ANSWER(FilterRow) ANSWER(RankAggregate) ANSWER(FindAggregate)
            ANSWER(AnnotateRow) ANSWER(RecognizeAggregate) ANSWER(RelateAggregate)
#undef ANSWER
            if(verb=="annotate-null") {
                if(auto row=packet.as_SessionPacketAnnotateRow()) {
                    auto annotation=row->value();
                    if(annotation.state!=results::PresenceState::value || annotation.value->value().state!=results::PresenceState::value) return 23;
                    auto fields=annotation.value->value().value->value();
                    if(fields.size()!=1 || fields[0].first!="refund" || !fields[0].second.document().is_null()) return 24;
                    if(!std::visit([](const auto& value){ return std::is_same_v<std::decay_t<decltype(value)>,std::nullptr_t>; },fields[0].second.value())) return 25;
                }
                if(Json::parse(packet.document().dump())!=packet.document()) return 26;
            }
            std::cout<<packet.document().dump()<<'\n';
        }
        return terminal && answer ? 0 : 5;
    } catch(const SessionFailure& failure) { std::cerr<<failure.failure.document().dump()<<'\n'; return 6; }
    catch(const std::exception& failure) { std::cerr<<failure.what()<<'\n'; return 7; }
}
