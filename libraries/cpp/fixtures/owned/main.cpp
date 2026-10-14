#include <thinkthen/client.hpp>
#include <chrono>
#include <thread>
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
static Data read(const char* path) { std::ifstream file(path); return Data::read(file); }
static RequestItem item(const Data& value) { return RequestItem().set_original(RequestOriginalJson().set_value(value)); }
static RequestInput evidence(const std::string& verb,const Data& original) {
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
        if(argc==2 && std::string(argv[1])=="smoke") {
            Client client;
            auto call=client.decide(RequestQuestionText().set_text(std::getenv("THINKTHEN_TEST_SMOKE_QUESTION")),
                RequestInputText().set_text(std::getenv("THINKTHEN_TEST_SMOKE_TEXT")));
            call.finish();
            bool answered=false;
            for(const auto& packet:call.collect()) if(auto row=packet.as_SessionPacketDecideRow()) {
                std::cout<<"smoke: "<<std::boolalpha<<row->value().value->value().value->value().get<bool>()<<'\n';
                answered=true;
            }
            return answered ? 0:3;
        }
        if(argc!=4) return 2;
        if(std::string(argv[1]).rfind("plan",0)==0) {
            Client client;
            const auto verb=std::string(argv[1])=="plan" ? "decide" : std::string(argv[1]).substr(5);
            auto question=std::string(argv[1])=="plan" ? RequestQuestion(RequestQuestionText().set_text("asks for a refund"))
                : RequestQuestion(RequestQuestionFile().set_path(argv[2]));
            auto input=std::string(argv[1])=="plan" ? RequestInput(RequestInputText().set_text("Refund me please."))
                : evidence(verb,read(argv[3]));
            auto call=RequestCall().set_function(RequestFunction(verb)).set_question(question).set_input(input);
            const bool supported=verb=="decide" || verb=="choose" || verb=="tag" || verb=="score" || verb=="filter" || verb=="rank";
            try {
                auto plan=client.plan(call); client.close();
                if(!supported) throw std::logic_error("unsupported plan was accepted");
                const auto& band=plan.estimated_input_tokens().value.value();
                if(!plan.records().value || !plan.requests().value || !plan.estimated_bytes().value ||
                   !plan.largest_request_bytes().value || !plan.largest_request_estimated_input_tokens().value ||
                   !band.lower().value || !band.upper().value || !plan.token_estimate_method().value ||
                   !plan.upper_bound().value || plan.first_body_utf8().state==results::PresenceState::absent)
                    throw std::logic_error("plan lost a required typed field");
                if(*plan.records().value!=1 || *plan.requests().value<1 || *band.lower().value>*band.upper().value ||
                   *plan.largest_request_bytes().value>*plan.estimated_bytes().value || plan.token_estimate_method().value->empty())
                    throw std::logic_error("plan estimates are inconsistent");
                std::cout<<::tt::detail::serialize(::tt::results::detail::document(plan))<<'\n'; return 0;
            } catch(const ::tt::Failure& error) {
                if(supported || error.kind!=NativeFailureKind::usage || error.retryable) throw;
                std::cout<<"plan-refusal-pass"<<'\n'; return 0;
            }
        }
        if(std::string(argv[1])=="closed") {
            Client client(EngineSettings().set_cache(CacheDocument(DisabledCache(false))));
            auto question=RequestQuestionDefinition().set_value(::tt::results::detail::Loader::input<RequestDefinition>(Data{{"decide","Is it?"},{"item_schema",Data{{"type","string"}}}}));
            auto call=client.decide(question,RequestInputFeed().set_name("closed"));
            auto invalid=RequestSessionDescriptor().set_item(item(Data(42)));
            if(!call.try_push(invalid)) throw std::logic_error("fresh intake did not accept descriptor");
            const auto deadline=std::chrono::steady_clock::now()+std::chrono::seconds(2);
            for(;;) {
                try { call.poll(); }
                catch(const ::tt::Failure&) { break; }
                if(std::chrono::steady_clock::now()>deadline) throw std::logic_error("invalid descriptor did not stop native intake");
                std::this_thread::yield();
            }
            auto next=RequestSessionDescriptor().set_item(item(Data("unreachable")));
            if(call.try_push(next) || !call.input_closed() || call.try_push(next)) throw std::logic_error("native closure did not stop repeat pushes");
            auto moved=std::move(call);
            if(!moved.input_closed() || !call.input_closed() || moved.try_push(next)) throw std::logic_error("moved closure reopened intake");
            moved.close(); if(!moved.input_closed()) throw std::logic_error("closed call reopened intake");
            std::cout<<"closed-push-pass\n"; return 0;
        }
        if(std::string(argv[1])=="usage-written" || std::string(argv[1])=="usage-failed") {
            Client client(EngineSettings().set_cache(CacheDocument(DisabledCache(false)))); auto call=client.decide(RequestQuestionText().set_text("Is it?"),RequestInputText().set_text("yes"));
            call.finish(); auto packets=call.collect(); auto earlier=::tt::detail::serialize(::tt::results::detail::document(packets.back()));
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
            if(::tt::detail::serialize(::tt::results::detail::document(packets.back()))!=earlier) return 36;
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
            std::exception_ptr unexpected;
            std::thread reader([&] {
                try { call.next(); unexpected=std::make_exception_ptr(std::logic_error("cancelled read returned a packet")); }
                catch(const ::tt::Failure& error) { if(error.kind!=NativeFailureKind::cancelled) unexpected=std::current_exception(); }
                catch(...) { unexpected=std::current_exception(); }
            });
            client.close(); call.cancel(); reader.join();
            if(unexpected) std::rethrow_exception(unexpected);
            } // Destructor releases the session before the provider is released.
            std::cout<<"cleaned\n"<<std::flush; return 0;
        }
        if(std::string(argv[1])=="values") {
            using namespace tt::results;
            Facts absent(Data::Object{});
            Facts null(Data{{"model",nullptr},{"held_model_mismatch",false},{"input_tokens",uint64_t(-1)},{"future",Data{{"n",uint64_t(-1)}}}});
            if(absent.model().state!=PresenceState::absent || null.model().state!=PresenceState::null) return 10;
            if(null.held_model_mismatch().state!=PresenceState::value || *null.held_model_mismatch().value) return 11;
            if(*null.input_tokens().value!=UINT64_MAX || ::tt::detail::serialize(::tt::results::detail::document(null).at("future").at("n"))!=std::to_string(UINT64_MAX)) return 12;
            if(::tt::detail::serialize(::tt::detail::parse_original(::tt::detail::serialize(::tt::results::detail::document(null))).at("input_tokens"))!=std::to_string(UINT64_MAX)) return 13;
            if(Data(5)!=::tt::detail::parse_original("5") || Data(UINT64_MAX)==Data(static_cast<double>(UINT64_MAX))) return 21;
            if(::tt::detail::serialize(::tt::detail::parse_original("-9223372036854775808"))!="-9223372036854775808") return 22;
            SessionPacket escaped(::tt::detail::parse_original(R"({"kind":"termin\u0061l","facts":{},"future":false})"));
            if(!escaped.as_SessionPacketTerminal() || ::tt::results::detail::document(escaped).at("future").get<bool>()) return 14;
            std::cout<<"values-pass\n"; return 0;
        }
        std::string verb=argv[1];
        const std::locale originalLocale;
        const std::string originalNumeric=std::setlocale(LC_NUMERIC,nullptr);
        std::optional<CallerLocale> locale;
        if(verb=="locale") locale.emplace();
        auto settings=EngineSettings();
        if(auto record=std::getenv("THINKTHEN_CPP_RECORD")) settings.set_record(record);
        Client client(settings);
        if(verb=="locale") {
            Data original{{"nested",Data::Array{Data{{"number",0.5}},1.25}}};
            auto input=RequestInputRecords().set_items({item(original)});
            auto call=client.decide(RequestQuestionText().set_text("Is it?"),input); call.finish();
            auto packets=call.collect(); client.close(); call.close();
            bool answered=false;
            for(const auto& packet:packets) {
                if(auto row=packet.as_SessionPacketDecideRow()) {
                    if(::tt::results::detail::document(*row->value().value).at("input")!=original) return 40;
                    if(!row->value().value->value().value->value().get<bool>()) return 41;
                    auto answer=row->value().value->answer().value->as_AnswerYesNo();
                    if(!answer || *answer->probability().value!=0.9) return 45;
                    answered=true;
                }
                if(::tt::detail::parse_original(::tt::detail::serialize(::tt::results::detail::document(packet)))!=::tt::results::detail::document(packet)) return 42;
                std::cout<<::tt::detail::serialize(::tt::results::detail::document(packet))<<'\n';
            }
            locale.reset();
            if(std::locale()!=originalLocale || originalNumeric!=std::setlocale(LC_NUMERIC,nullptr)) return 44;
            return answered && packets.back().as_SessionPacketTerminal() ? 0:43;
        }
        if(verb=="cardinality-zero" || verb=="cardinality-many") {
            auto input=verb=="cardinality-zero" ? RequestInput(RequestInputFeed().set_name("empty")) : RequestInput(RequestInputRecords().set_items({item(Data("first")),item(Data("second"))}));
            auto call=client.decide(RequestQuestionText().set_text("Is it?"),input);
            try { call.answer(); return 48; }
            catch(const ::tt::Failure& error) {
                if(error.packets.empty() || !error.packets.back().as_SessionPacketTerminal()) return 49;
                const bool empty=verb=="cardinality-zero";
                if(std::string(error.what())!=(empty ? "no matching judgment":"multiple matching judgments")) return 50;
                size_t judgments=0;
                for(const auto& packet:error.packets) {
                    const auto& value=::tt::results::detail::document(packet);
                    if(value.at("kind").get<std::string>()=="observation" && value.at("value").at("kind").get<std::string>()=="row" && value.at("value").at("value").at("kind").get<std::string>()=="judgment") ++judgments;
                }
                if(judgments!=(empty ? 0u:2u)) return 51;
                std::cout<<"cardinality-pass judgments="<<judgments<<'\n'; return 0;
            }
        }
        if(verb=="failure") {
            auto call=client.decide(RequestQuestionText().set_text("Is it?"),RequestInputText().set_text("backend-failure")); call.finish();
            try { call.answer(); return 15; }
            catch(const ::tt::Failure& error) {
                if(!error.failure || error.failure->error().value->kind().value->value()!="backend" || error.failure->error().value->retryable().value.value()) return 16;
                if(error.failure->facts().state!=results::PresenceState::value || *error.failure->facts().value->requests_sent().value!=1) return 17;
                if(!error.terminal || error.terminal->facts().state!=results::PresenceState::value) return 20;
                std::cout<<"failure-pass\n"; return 0;
            }
        }
        if(verb=="invalid" || verb=="image") {
            try {
                if(verb=="invalid") client.decide(RequestQuestionText(),RequestInputText().set_text("unreachable"));
                else client.tag(RequestQuestionFile().set_path(argv[2]),RequestInputText().set_text("unreachable").set_images({RequestImageBytes().set_media(ImageMedia("image/png")).set_bytes("aGVsbG8=")}));
                return 18;
            } catch(const ::tt::Failure& error) { if(error.kind!=NativeFailureKind::usage) return 19; std::cout<<"admission-pass\n"; return 0; }
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
                    if(fields.size()!=1 || fields[0].first!="refund" || !::tt::results::detail::document(fields[0].second).is_null()) return 24;
                    if(!std::visit([](const auto& value){ return std::is_same_v<std::decay_t<decltype(value)>,std::nullptr_t>; },fields[0].second.value())) return 25;
                }
                if(::tt::detail::parse_original(::tt::detail::serialize(::tt::results::detail::document(packet)))!=::tt::results::detail::document(packet)) return 26;
            }
            // The shipped nested range form must retain its decoded container.
            if(auto ranked=packet.as_SessionPacketRankAggregate()) {
                for(const auto& index:*ranked->value().value) {
                    if(*index.value().value==0) return 46;
                }
            }
            if(auto found=packet.as_SessionPacketRecognizeAggregate()) {
                for(const auto& entity:*found->value().value) {
                    if(::tt::results::detail::document(entity).is_null()) return 47;
                }
            }
            std::cout<<::tt::detail::serialize(::tt::results::detail::document(packet))<<'\n';
        }
        return terminal && answer ? 0 : 5;
    } catch(const ::tt::Failure& failure) { if(failure.failure) std::cerr<<::tt::detail::serialize(::tt::results::detail::document(*failure.failure))<<'\n'; else std::cerr<<failure.what()<<'\n'; return 6; }
    catch(const std::exception& failure) { std::cerr<<failure.what()<<'\n'; return 7; }
}
