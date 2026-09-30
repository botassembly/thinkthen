#include <thinkthen/door.hpp>
#include <chrono>
#include <cstdlib>
#include <filesystem>
#include <fstream>
#include <future>
#include <iostream>
#include <set>
#include <thread>

namespace fs = std::filesystem;
using namespace std::chrono_literals;
using tt::Json;
void check(bool condition, const char* label) {
    if (!condition) throw std::runtime_error(std::string("CHECK_FAILED ") + label);
    std::cout << "PASS " << label << std::endl;
}
fs::path barrier() { return fs::path(std::getenv("TT_BARRIER_DIR")); }
void release(const std::string& name) { std::ofstream(barrier() / ("release-" + name)) << "go"; }
void awaitArrival(const std::string& name) {
    for (int i = 0; i < 2000; ++i) {
        if (fs::exists(barrier() / ("arrived-" + name))) return;
        std::this_thread::sleep_for(5ms);
    }
    throw std::runtime_error("missing counted arrival " + name);
}
std::string plant() { auto p = std::getenv("TT_PLANT"); return p ? p : ""; }
struct ThreadJoin { std::thread& thread; ~ThreadJoin() { if (thread.joinable()) thread.join(); } };
void held(const tt::Engine& engine, const std::string& name, int expected, bool bulk = false) {
    auto token = tt::token();
    struct Result { int code = 0; bool retryable = true; std::string message; std::optional<Json> facts; } result;
    std::promise<void> workerCaptured;
    auto captured = workerCaptured.get_future().share();
    std::thread worker([&] {
        try {
            if (bulk) tt::many(engine, "Is it?", {name, "hold-bulk-second"}, THINKTHEN_NO_DEADLINE, token.get());
            else tt::decide(engine, "Is it?", name, expected == 3 ? 35 : THINKTHEN_NO_DEADLINE, token.get());
            result.code = -1;
        } catch (const tt::Failure& e) {
            // The error accessors and borrowed facts are copied ON THIS WORKER THREAD.
            result.code = static_cast<int>(e.kind);
            result.retryable = e.retryable;
            result.message = e.what();
            result.facts = e.facts;
        } catch (const std::exception& e) { result.code = -2; result.message = e.what(); }
        workerCaptured.set_value();
    });
    ThreadJoin workerJoin{worker};
    std::thread canceller;
    ThreadJoin cancelJoin{canceller};
    struct ErrorSnapshot { int code = 0; std::string message; int retryable = -1; } before, after;
    int deadlineReturn = -1;
    std::exception_ptr cancellerException;
    std::promise<void> fired;
    auto firedFuture = fired.get_future();
    bool completed = false;
    try {
        awaitArrival(name);
        if (plant() == "fail-mid-case" && name == "hold-scalar")
            throw std::runtime_error("CHECK_FAILED PLANTED_MID_CASE_FAILURE");
        if (expected == 5) {
            canceller = std::thread([&] {
                bool signalled = false;
                try {
                    auto refused = thinkthen_answer{123, -1};
                    deadlineReturn = thinkthen_decide_opts(engine.get(), "Is it?", "unused", 6, 0, nullptr, &refused);
                    before = {thinkthen_error_code(engine.get()),
                              thinkthen_error_message(engine.get()) ? thinkthen_error_message(engine.get()) : "",
                              thinkthen_error_retryable(engine.get())};
                    if (refused.outcome != 123 || refused.probability != -1)
                        throw std::runtime_error("canceller deadline changed output");
                    thinkthen_cancel(token.get()); thinkthen_cancel(token.get());
                    fired.set_value(); signalled = true;
                    captured.wait(); // Worker copied its own code, message and facts first.
                    after = {thinkthen_error_code(engine.get()),
                             thinkthen_error_message(engine.get()) ? thinkthen_error_message(engine.get()) : "",
                             thinkthen_error_retryable(engine.get())};
                } catch (...) {
                    cancellerException = std::current_exception();
                    if (!signalled) fired.set_value();
                }
            });
            firedFuture.wait(); // Deadline recorded and token fired twice (or canceller failed).
        } else std::this_thread::sleep_for(100ms);
        release(name);
        if (bulk) release("hold-bulk-second");
        worker.join();
        if (canceller.joinable()) canceller.join();
        std::cout << "THREADS_JOINED worker canceller(" << expected << ")" << std::endl;
        if (cancellerException) std::rethrow_exception(cancellerException);
        check(result.code == expected && !result.retryable && !result.message.empty() && result.facts.has_value(),
              bulk ? "bulk wrapper-enforced output sentinels; worker code and copied facts" :
              expected == 3 ? "deadline wrapper-enforced output sentinel; worker code and copied facts" :
              "scalar wrapper-enforced output sentinel; worker code and copied facts");
        if (expected == 5) {
            check(deadlineReturn == 3 && before.code == 3 && !before.message.empty() && before.retryable == 0 &&
                  after.code == before.code && after.message == before.message &&
                  after.retryable == before.retryable && result.code == 5,
                  "thread-local error code message retryable unchanged after worker cancellation capture");
            std::cout << "THREAD_LOCAL_ERROR_ISOLATION_PASS worker_code=" << result.code
                      << " canceller_code=" << after.code
                      << " canceller_message_unchanged=true canceller_retryable_unchanged=true" << std::endl;
        }
        if (expected == 5) {
            try { tt::decide(engine, "Is it?", "no-arrival-spent-" + name, -1, token.get());
                  throw std::runtime_error("CHECK_FAILED fired token reuse accepted"); }
            catch (const tt::Failure& e) { check(e.kind == tt::ErrorKind::cancelled,
                                                   "fired-token reuse refused without send"); }
        }
        completed = true;
    } catch (...) {
        release(name);
        if (bulk) release("hold-bulk-second");
        if (canceller.joinable()) canceller.join();
        if (worker.joinable()) worker.join();
        std::cout << "THREADS_JOINED worker canceller(" << expected << ")" << std::endl;
        token.reset();
        std::cout << "TOKEN_FREED_AFTER_JOIN\nCLEANUP_ORDER_PASS " << name << std::endl;
        throw;
    }
    if (completed) {
        token.reset();
        std::cout << "TOKEN_FREED_AFTER_JOIN" << std::endl;
    }
}
void ownedFactsCase() {
    auto engine = tt::create();
    auto noUsage = tt::decide(engine, "Is it?", "without-usage");
    check(noUsage.value.outcome == tt::Outcome::yes && noUsage.facts.at("records") == 1 &&
          noUsage.facts.at("requests_sent") == 1 && !noUsage.facts.contains("input_tokens") &&
          !noUsage.facts.contains("output_tokens") && noUsage.facts.at("model") == "jev-1.13.0",
          "typed valid reply without usage retains model");
    auto empty = tt::many(engine, "Is it?", {});
    check(empty.value.empty() && empty.facts.at("records") == 0 && empty.facts.at("requests_sent") == 0 &&
          empty.facts.at("cache_answers") == 0 && !empty.facts.contains("model") && !empty.facts.contains("input_tokens") &&
          !empty.facts.contains("output_tokens"), "empty bulk owned zero facts, no model");
    std::optional<tt::CallResult<tt::Judgment>> scalar;
    std::optional<tt::CallResult<std::vector<tt::Judgment>>> bulk;
    std::exception_ptr scalarError, bulkError;
    std::thread first([&] { try { scalar = tt::decide(engine, "Is it?", "hold-owned-scalar"); }
                            catch (...) { scalarError = std::current_exception(); } });
    ThreadJoin firstJoin{first};
    std::thread second;
    ThreadJoin secondJoin{second};
    try {
        second = std::thread([&] { try { bulk = tt::many(engine, "Is it?", {"hold-owned-bulk-first", "hold-owned-bulk-second"}); }
                                   catch (...) { bulkError = std::current_exception(); } });
        awaitArrival("hold-owned-scalar"); awaitArrival("hold-owned-bulk-first");
    } catch (...) { release("hold-owned-scalar"); release("hold-owned-bulk-first"); throw; }
    release("hold-owned-scalar"); release("hold-owned-bulk-first");
    first.join(); second.join();
    if (scalarError) std::rethrow_exception(scalarError);
    if (bulkError) std::rethrow_exception(bulkError);
    check(scalar && bulk && scalar->value.outcome == tt::Outcome::yes && scalar->facts.at("records") == 1 &&
          scalar->facts.at("requests_sent") == 1 && bulk->value.size() == 2 && bulk->facts.at("records") == 2 &&
          bulk->facts.at("requests_sent") == 1, "both held requests arrived before release and returned distinct owned facts");
    auto later = tt::decide(engine, "Is it?", "without-usage");
    engine.reset();
    check(later.facts.at("cache_answers") == 1 && scalar->facts.at("records") == 1 && bulk->facts.at("records") == 2 &&
          noUsage.facts.at("model") == "jev-1.13.0",
          "owned facts survive later call and engine close");
    std::cout << "CPP_OWNED_FACTS_PASS" << std::endl;
}
// Ticket 0291. run.py's exact arrival count proves each of these sends nothing.
void planAndLimits(const tt::Engine& engine, const std::string& spec, const std::string& relSpec) {
    const char* held = std::getenv("THINKTHEN_API_KEY");
    const std::string key = held ? held : "";
    unsetenv("THINKTHEN_API_KEY");
    auto keyless = tt::create();
    setenv("THINKTHEN_API_KEY", key.c_str(), 1);
    auto p1 = tt::plan(keyless, "decide", "asks for a refund", {"Refund me please."}, Json::Object{});
    check(p1.at("records") == 1 && p1.at("requests") == 1 && p1.at("estimated_bytes") == 182 &&
          p1.at("estimated_input_tokens") == Json{{"lower", 93}, {"upper", 166}} && p1.at("upper_bound") == false &&
          p1.at("first_body_utf8").get<std::string>().size() == 182, "P1 plan with no key");
    try { tt::plan(keyless, "decide", "asks for a refund", {"Refund me please."}, Json{{"batch", 0}});
          throw std::runtime_error("CHECK_FAILED batch 0 planned"); }
    catch (const tt::Failure& e) { check(e.kind == tt::ErrorKind::usage && static_cast<int>(e.kind) == 1, "plan refusal is usage 1"); }
    for (int verb = 0; verb < 3; ++verb) {
        try {
            if (verb == 0) tt::call(engine, {{"decide", "Is it?"}, {"evidence", "zero-call"}}, 0);
            if (verb == 1) tt::recognize(engine, spec, "zero-recognize", 0);
            if (verb == 2) tt::relate(engine, relSpec, {R"({"name":"A","kind":"alert"})", R"({"name":"B","kind":"alert"})"}, 0);
            throw std::runtime_error("CHECK_FAILED zero budget accepted");
        } catch (const tt::Failure& e) { check(e.kind == tt::ErrorKind::deadline, "zero budget refused before sending"); }
    }
    auto capped = tt::create(R"({"max_requests_total":0,"cache":false})");
    try { tt::decide(capped, "Is it?", "capped"); throw std::runtime_error("CHECK_FAILED zero cap sent"); }
    catch (const tt::Failure& e) {
        check(e.kind == tt::ErrorKind::usage && std::string(e.what()).find("process send budget") != std::string::npos,
              "zero active cap refused before sending");
    }
}
int main() {
    try {
        if (std::getenv("TT_FACTS_ONLY")) { ownedFactsCase(); return 0; }
        auto engine = tt::create();
        std::optional<Json> retainedFailureFacts;
        try {
            for (const auto& row : std::vector<std::tuple<std::string,tt::Outcome,double>>{
                    {"yes",tt::Outcome::yes,.9},{"no",tt::Outcome::no,.1},{"unsure",tt::Outcome::notSure,.5}}) {
                auto text = std::get<0>(row);
                auto q = text == "unsure" ? R"({"decide":"Is it?","threshold":"0.4:0.8"})" : "Is it?";
                auto value = tt::decide(engine, q, text);
                check(value.value.outcome == std::get<1>(row) && value.value.probability == std::get<2>(row) &&
                      value.facts.at("records") == 1 && value.facts.at("requests_sent") == 1,
                      text == "yes" ? "scalar yes 0/0.9" : text == "no" ? "scalar no 1/0.1" : "scalar unsure 2/0.5");
            }
            auto before = tt::call(engine, {{"usage", true}});
            check(tt::decide(engine, "Is it?", "yes").value.outcome == tt::Outcome::yes, "cached yes");
            auto after = tt::call(engine, {{"usage", true}});
            check(after.at("requests_sent") == before.at("requests_sent") &&
                  after.at("cache_answers").get<double>() > before.at("cache_answers").get<double>(), "cache and counters");
            auto rows = tt::many(engine, "Is it?", {"batch-one", "batch-two"});
            check(rows.value.size() == 2 && rows.value[0].outcome == tt::Outcome::yes && rows.value[0].probability == .9 &&
                  rows.value[1].outcome == tt::Outcome::no && rows.value[1].probability == .1 && rows.facts.at("records") == 2,
                  "bulk ordered answers .9 then .1");
            auto answer = tt::call(engine, {{"decide","Is it?"},{"evidence","json-decide"}});
            check(tt::decisionValue(answer.at("value")) == tt::NullableOutcome{tt::Outcome::yes} &&
                  answer.at("facts").at("records") == 1 &&
                  answer.at("facts").at("requests_sent") == 1, "JSON door exact decision and facts identity");
            auto annotate = tt::call(engine, Json::parse(R"({"annotate":{"version":1,"questions":{"check":{"decide":"Is it?"}}},"records":["annotate-one"]})"));
            check(annotate.at("value") == Json::parse(R"([{"check":true}])") &&
                  std::holds_alternative<tt::NullableOutcome>(tt::annotatedDecision(annotate.at("value").at(0).at("check"))) &&
                  annotate.at("facts").at("records") == 1, "annotate JSON exact field identity");
            const std::string spec = R"({"version":1,"recognize":{"kinds":{"person":"A person name."}}})";
            auto recognized = tt::recognize(engine, spec, "John Smith");
            check(recognized.value == Json::parse(R"({"entities":[{"text":"John Smith","start":0,"end":10,"length":10,"kind":"person","strength":0.81}]})") &&
                  recognized.facts.at("records") == 1,
                  "recognize JSON exact entity identity");
            const std::string relSpec = R"({"version":1,"relate":{"relations":[{"name":"caused_by","source":"alert","target":"alert"}]}})";
            std::vector<std::string> entities = {R"({"name":"First","kind":"alert"})", R"({"name":"Second","kind":"alert"})"};
            auto related = tt::relate(engine, relSpec, entities);
            check(related.value == Json::parse(R"({"edges":[{"relation":"caused_by","source":{"name":"First","kind":"alert"},"target":{"name":"Second","kind":"alert"},"probability":0.9},{"relation":"caused_by","source":{"name":"Second","kind":"alert"},"target":{"name":"First","kind":"alert"},"probability":0.9}]})") && related.facts.at("records") == 1,
                  "relate JSON exact edges and order");
            // Alias entry points use cached keys and reproduce exact results, not merely successful codes.
            thinkthen_answer plain{123,-1};
            check(thinkthen_decide(engine.get(),"Is it?","yes",3,&plain)==0 && plain.outcome==1 && plain.probability==.9,
                  "plain decide alias");
            const char* one[] = {"yes"}; const size_t oneLen[] = {3};
            check(thinkthen_decide_many(engine.get(),"Is it?",one,oneLen,1,&plain)==0 && plain.outcome==1 && plain.probability==.9,
                  "plain many alias");
            tt::OwnedString usage(thinkthen_call(engine.get(),R"({"usage":true})"));
            check(usage && Json::parse(usage.get()).at("requests_sent") == tt::call(engine,{{"usage",true}}).at("requests_sent"),
                  "plain JSON alias");
            char* raw = nullptr; size_t size = 999;
            check(thinkthen_recognize(engine.get(),spec.c_str(),"John Smith",10,&raw,&size)==0 && raw != nullptr,
                  "plain recognize return");
            tt::OwnedString plainRecognize(raw);
            check(Json::parse(std::string(plainRecognize.get(),size))==recognized.value, "plain recognize alias identity");
            const char* two[] = {entities[0].c_str(),entities[1].c_str()};
            const size_t twoLen[] = {entities[0].size(),entities[1].size()}; raw=nullptr; size=999;
            check(thinkthen_relate(engine.get(),relSpec.c_str(),two,twoLen,2,&raw,&size)==0 && raw != nullptr,
                  "plain relate return");
            tt::OwnedString plainRelate(raw);
            check(Json::parse(std::string(plainRelate.get(),size))==related.value,"plain relate alias identity");
            try { tt::decide(engine,"Is it?","backend-failure");
                  throw std::runtime_error("CHECK_FAILED missing backend refusal"); }
            catch (const tt::Failure& e) {
                check(e.kind==tt::ErrorKind::backend && !e.retryable && !std::string(e.what()).empty() &&
                      e.facts && e.facts->at("requests_sent")==1 && e.facts->at("records")==0,
                      "backend failure code message retryable copied facts");
                retainedFailureFacts = e.facts;
            }
            try { tt::decide(engine,"Is it?","never-spent",0);
                  throw std::runtime_error("CHECK_FAILED spent budget accepted"); }
            catch(const tt::Failure& e){ check(e.kind==tt::ErrorKind::deadline,"spent budget refusal"); }
            check(retainedFailureFacts && retainedFailureFacts->at("requests_sent")==1 && retainedFailureFacts->at("records")==0,
                  "borrowed failure facts copied before next failure");
            try { tt::decide(engine, R"({"choose":"Which?","options":["a","b"]})", "never-spent");
                  throw std::runtime_error("CHECK_FAILED non-decide question accepted"); }
            catch (const tt::Failure& e) { check(e.kind==tt::ErrorKind::usage && !e.facts,
                                                  "typed pre-start question refusal has no facts"); }
            for (const char* invalid : {R"({"not_a_setting":1})", R"({"timeout":"wrong"})"}) {
                try { auto bad=tt::create(invalid); throw std::runtime_error("CHECK_FAILED invalid constructor accepted"); }
                catch(const tt::Failure& e){ check(e.kind==tt::ErrorKind::usage && !e.facts,"invalid constructor EUSAGE zero sends"); }
            }
            const auto realURL = std::string(std::getenv("THINKTHEN_BASE_URL"));
            setenv("THINKTHEN_BASE_URL", "http://127.0.0.1:1/generic/v1", 1);
            tt::Engine configured;
            try { configured = tt::create(tt::question({{"base_url",realURL},{"cache",std::getenv("THINKTHEN_CACHE")}}).c_str()); }
            catch (...) { setenv("THINKTHEN_BASE_URL",realURL.c_str(),1); throw; }
            setenv("THINKTHEN_BASE_URL",realURL.c_str(),1);
            check(tt::decide(configured,"Is it?","configured").value.outcome==tt::Outcome::yes,"configured route overrides invalid env URL");
            configured.reset();
            auto empty=tt::create("{}");
            check(tt::decide(empty,"Is it?","yes").value.outcome==tt::Outcome::yes,"empty settings equivalent");
            empty.reset();
            // Shared J1 non-BMP case 41: parse actual corpus sample, not invented offsets.
            std::ifstream corpus(std::string(std::getenv("TT_SOURCE"))+"/specification/fixtures/types/corpus.json");
            check(corpus.good(),"shared J1 corpus readable"); Json cases=Json::parse(corpus);
            std::set<std::string> corpusNames; size_t corpusCount=0;
            for (const auto& item: cases.at("cases")) {
                ++corpusCount;
                check(corpusNames.insert(item.at("name").get<std::string>()).second,
                      "shared J1 corpus name occurs exactly once");
            }
            check(corpusCount > 0 && corpusNames.size() == corpusCount, "J1 corpus names are unique");
            int offsetCases = 0;
            for (const auto& item : cases.at("cases")) if (item.at("name")=="41-offsets-past-an-accent-and-an-emoji") {
                ++offsetCases;
                const auto& e=item.at("response").at("entities").at(0);
                check(item.at("request").at("evidence")=="Le café 😀 Maria Chen arrived." &&
                      e.at("text")=="Maria Chen" && e.at("start")==10 && e.at("end")==20 && e.at("length")==10 &&
                      item.at("offsets").at("utf16").at(0)==11 &&
                      item.at("offsets").at("utf16").at(1)==21,
                      "SHARED_NON_BMP_CASE_41_PASS scalar[10,20) utf16[11,21)");
            }
            check(offsetCases == 1, "J1 corpus case 41 occurs exactly once");
            // Structured descriptions go unmodified through the JSON ABI. A syntactically
            // valid map is tested against the core parser without a new backend arrival.
            auto described=tt::call(engine,Json::parse(R"({"choose":"Which?","options":{"billing":{"what":"Charges and refunds.","not_for":"Shipping.","examples":["refund"]},"other":"Everything else."},"evidence":"described"})"));
            check(described.contains("value") && described.contains("facts"),"structured descriptions map carried through");
            planAndLimits(engine, spec, relSpec);
            held(engine,"hold-scalar",5);
            auto prior=tt::call(engine,{{"usage",true}});
            check(tt::decide(engine,"Is it?","hold-scalar").value.outcome==tt::Outcome::yes,"cancelled reply cached");
            auto now=tt::call(engine,{{"usage",true}});
            check(now.at("requests_sent")==prior.at("requests_sent") && now.at("cache_answers").get<double>()>prior.at("cache_answers").get<double>(),
                  "cancel drain counters");
            auto fresh=tt::token();
            check(tt::decide(engine,"Is it?","recovery-scalar",-1,fresh.get()).value.outcome==tt::Outcome::yes,
                  "fresh token recovery");
            fresh.reset();
            held(engine,"hold-bulk-first",5,true);
            held(engine,"hold-deadline",3);
            std::cout << "CPP_PRODUCT_PASS" << std::endl;
        } catch (...) { engine.reset(); std::cout << "ENGINE_FREED_AFTER_JOIN" << std::endl; throw; }
        engine.reset(); std::cout << "ENGINE_FREED_AFTER_JOIN" << std::endl;
        check(retainedFailureFacts && retainedFailureFacts->at("requests_sent")==1 &&
              retainedFailureFacts->at("records")==0, "typed failure facts survive engine close");
        return 0;
    } catch (const std::exception& e) { std::cerr << e.what() << std::endl; return 255; }
}
