#include <thinkthen/thinkthen.hpp>
#include <thinkthen.h>
#include <cassert>
#include <cstdint>
#include <limits>
#include <iostream>
#include <optional>
#include <stdexcept>
#include <string>
#include <type_traits>
#include <utility>
#include <vector>

#if __cplusplus >= 202002L
#include <ranges>
static_assert(std::ranges::input_range<thinkthen::stream<thinkthen::decide, thinkthen::record>>);
#endif
namespace tt = thinkthen;

static tt::settings stub_settings() {
    tt::settings settings;
    settings.backend = tt::backend::stub;
    return settings;
}

static std::string code_name(std::error_code code) {
    return thinkthen_code_name(code.value());
}

/* Feature: required domains and separate record encodings are C++ types. */
static void typed_signatures() {
    static_assert(std::is_same_v<decltype(thinkthen_row{}.function), thinkthen_function>);
    static_assert(std::is_same_v<decltype(thinkthen_row{}.status), thinkthen_status>);
    static_assert(sizeof(thinkthen_function) == sizeof(std::uint32_t));
    static_assert(sizeof(thinkthen_status) == sizeof(std::uint32_t));
    static_assert(offsetof(thinkthen_row, function) == 4);
    static_assert(offsetof(thinkthen_row, status) == 8);
    static_assert(sizeof(thinkthen_backend) == sizeof(std::uint32_t));
    static_assert(sizeof(thinkthen_record_kind) == sizeof(std::uint32_t));
    static_assert(sizeof(thinkthen_offset) == sizeof(std::uint32_t));
    thinkthen_error initialized_error = THINKTHEN_ERROR_INIT;
    thinkthen_row initialized_row = THINKTHEN_ROW_INIT;
    assert(initialized_error.struct_size == sizeof initialized_error && initialized_error.code == 0);
    assert(initialized_row.struct_size == sizeof initialized_row && initialized_row.error_code == 0);
    static_assert(!std::is_copy_constructible_v<tt::client>);
    static_assert(!std::is_constructible_v<tt::choose<>, std::string>);
    static_assert(!std::is_constructible_v<tt::tag<>, std::string>);
    static_assert(!std::is_constructible_v<tt::score<>, std::string>);
    static_assert(!std::is_constructible_v<tt::rank<>, std::string>);
    static_assert(!std::is_default_constructible_v<tt::recognize>);
    static_assert(!std::is_default_constructible_v<tt::relate>);
    static_assert(!std::is_same_v<tt::text, tt::json_text>);
    static_assert(std::is_same_v<tt::record, tt::owned_record>);
    static_assert(std::is_same_v<decltype(std::declval<const tt::row<tt::choose<>>&>().answer()),
                                 const std::string&>);
}

/* Feature: C++ questions admit their required lists through the engine. */
static void empty_domain() {
    try {
        tt::choose<> question{"Which team?", std::vector<std::string>{}};
        (void)question;
        assert(false);
    } catch (const tt::error& error) {
        assert(code_name(error.code()) == "usage.question");
    }
}

/* Feature: native argument failures survive the shared invocation helper. */
static void native_argument_errors() {
    auto config = stub_settings();
    config.backend = static_cast<tt::backend>(999);
    try { tt::client rejected{config}; assert(false); }
    catch (const tt::error& error) { assert(error.code() == tt::errc::usage_backend); }
    try { (void)tt::threshold::band(0.8, 0.2); assert(false); }
    catch (const tt::error& error) { assert(error.code() == tt::errc::usage_threshold); }
}

/* Feature: a copied typed row retains answer and detail after native cleanup. */
static void owned_answer_and_detail() {
    const auto retained = [] {
        tt::client owner(stub_settings());
        auto row = owner.choose(tt::choose<>{"Which team?", {"billing", "shipping"}},
                                tt::text{"refund please"});
        assert(row.status() == tt::status::answered);
        assert(row.answered());
        assert(row.error() == nullptr);
        return row;
    }();
    assert(retained.position() == 1);
    const auto& value = retained.answer();
    assert(value == "billing" || value == "shipping");
    const auto detail = retained.detail();
    assert(detail.has_value());
    assert(detail->labels.size() == 2);
    assert(detail->labels.at(value) == 1.0);
    static_assert(!std::is_copy_constructible_v<decltype(retained)>);
}

/* Feature: deferred callbacks own strings that were local to the producer. */
static void callback_local_record() {
    tt::client owner(stub_settings());
    std::size_t reads = 0;
    auto rows = owner.decide_each(tt::decide{"Ready?"},
        [&reads]() -> std::optional<tt::owned_record> {
            if (reads++ != 0) return std::nullopt;
            std::string local = "{\"id\":18446744073709551615,\"body\":\"quoted \\\"text\\\"\"}";
            return tt::owned_json_text{std::move(local)};
        });
    std::size_t count = 0;
    for (const auto& row : rows) {
        assert(row.status() == tt::status::answered);
        assert(row.id_json() == "18446744073709551615");
        assert(row.answered());
        ++count;
    }
    assert(count == 1);
    assert(reads == 2);
}

/* Feature: deferred vectors and rows outlive their caller's temporary owners. */
static void owned_collection() {
    auto rows = [] {
        tt::client owner(stub_settings());
        std::vector<tt::owned_record> local;
        local.emplace_back(tt::owned_json_text{"{\"id\":-9223372036854775808,\"body\":\"alpha\"}"});
        local.emplace_back(tt::owned_json_text{"{\"id\":\"second\",\"body\":\"beta\"}"});
        return owner.decide_each(tt::decide{"Ready?"}, std::move(local));
    }();
    std::vector<tt::row<tt::decide, tt::record>> retained;
    for (auto& row : rows) retained.push_back(std::move(row));
    assert(retained.size() == 2);
    assert(retained[0].position() == 1 && retained[1].position() == 2);
    assert(retained[0].id_json() == "-9223372036854775808");
    assert(retained[1].id_json() == "\"second\"");
    assert(retained[0].answered() && retained[1].answered());
}

/* Feature: missing records and invalid ids stay typed failed rows. */
static void record_failures() {
    tt::client owner(stub_settings());
    auto missing_rows = owner.decide_each(tt::decide{"Ready?"},std::vector<tt::owned_record>{tt::missing_record{}});
    const auto missing = std::move(*missing_rows.next());
    assert(missing.status() == tt::status::failed);
    assert(!missing.probability());
    try{(void)missing.answer();assert(false);}catch(const tt::no_answer&){}
    assert(missing.error());
    assert(code_name(missing.error()->code()) == "record.missing");
    auto invalid_rows = owner.decide_each(tt::decide{"Ready?"},
        std::vector<tt::owned_record>{tt::owned_json_text{"{\"id\":{\"nested\":1},\"body\":\"evidence\"}"}});
    const auto invalid = std::move(*invalid_rows.next());
    assert(invalid.status() == tt::status::failed);
    assert(invalid.error());
    try{(void)invalid.answer();assert(false);}catch(const tt::no_answer&){}
    assert(code_name(invalid.error()->code()) == "record.invalid");
}

/* Feature: moved-from C++ owners report host misuse without an invented code. */
static void moved_owners() {
    tt::client original(stub_settings());
    tt::client owner(std::move(original));
    bool host_error = false;
    try {
        (void)original.decide(tt::decide{"Ready?"}, tt::text{"evidence"});
    } catch (const tt::error&) {
        assert(false);
    } catch (const std::logic_error&) {
        host_error = true;
    }
    assert(host_error);
    auto first = owner.decide_each(tt::decide{"Ready?"},
        std::vector<tt::owned_record>{tt::owned_text{"evidence"}});
    auto moved = std::move(first);
    first.cancel_handle().request_cancel();
    assert(first.begin() == first.end());
    std::size_t count = 0;
    for (const auto& row : moved) { assert(row.answered()); ++count; }
    assert(count == 1);
}

/* Feature: a throwing source propagates its exception and releases its call. */
static void source_exception() {
    tt::client owner(stub_settings());
    bool propagated = false;
    try {
        auto rows = owner.decide_each(tt::decide{"Ready?"},
            []() -> std::optional<tt::owned_record> { throw std::domain_error("source sentinel"); });
        for (const auto& row : rows) { (void)row; assert(false); }
    } catch (const std::domain_error& error) {
        assert(std::string(error.what()) == "source sentinel");
        propagated = true;
    }
    assert(propagated);
    const auto row = owner.decide(tt::decide{"Still usable?"}, tt::text{"evidence"});
    assert(row.answered());
}

/* Feature: immutable questions preserve options, execution names and toolbox domains. */
static void snapshots_and_names() {
    tt::client owner(stub_settings());
    auto original = tt::choose<>{"Which?", {"red", "blue"}};
    tt::toolbox tools{{{"color", original}, {"rank", tt::rank<>{"How much?", {"low", "high"}}}}};
    const auto selected = tools.select<tt::choose<>>("color");
    const auto unnamed = owner.choose(selected, tt::text{"evidence"});
    const auto named = owner.choose(selected, tt::text{"evidence"}, "execution");
    assert(!unnamed.source().name && named.source().name == "execution");
    assert(unnamed.source().question_identity != named.source().question_identity);
    assert(unnamed.answer() == named.answer());
    assert(std::holds_alternative<tt::rank<>>(tools.select("rank")));
    try { (void)tools.select<tt::decide>("color"); assert(false); }
    catch (const tt::error& error) { assert(error.code() == tt::errc::usage_name); }
    const auto cut = original.threshold(0.8);
    assert(owner.choose(cut, tt::text{"evidence"}).source().question_identity != unnamed.source().question_identity);
    assert(owner.choose(original, tt::text{"evidence"}).source().question_identity == unnamed.source().question_identity);
    tt::annotation_questions entries{{"ready", tt::decide{"Ready?"}}};
    const auto annotation = owner.annotate(tt::annotate{entries}, tt::text{"evidence"});
    assert(tt::get<tt::decide>(annotation.answer(),"ready").answered());
    std::error_code code = tt::errc::record_missing;
    assert(code.category() == tt::category() && code.message() == "record.missing");
}
/* Feature: filter retains each original record even when the record fails. */
static void filter_originals() {
    tt::client owner(stub_settings());
    std::size_t kept=0;
    try {
        for(const auto& item:owner.filter_each(tt::filter{"Ready?"}.threshold(0.01),
            std::vector<tt::record>{tt::owned_text{"alpha"},tt::missing_record{},tt::owned_json_text{"{\"body\":\"beta\"}"}})) {
            assert(!std::holds_alternative<tt::missing_record>(item));++kept;
        }
        assert(false);
    } catch(const tt::filter_error<tt::record>& failure) {
        assert(kept==2);assert(failure.rows().size()==1);
        assert(failure.rows()[0].position()==2);
        assert(std::holds_alternative<tt::missing_record>(failure.rows()[0].item()));
    }
}

/* Feature: C++ saved toolboxes hydrate native typed snapshots through engine admission. */
static void saved_toolbox(const char* json) {
    auto tools = tt::toolbox::from_json(json);
    assert(std::holds_alternative<tt::filter>(tools.select("ready")));
    assert(std::holds_alternative<tt::relate>(tools.select("names")));
    const auto topic = tools.select<tt::choose<>>("topic");
    tt::client client(stub_settings());
    const auto loaded = client.choose(topic, tt::json_text{R"({"body":"Bill"})"});
    const auto direct = client.choose(tt::choose<>{"Which topic?", {"billing", "shipping"}}.field("/body").threshold(0.7), tt::json_text{R"({"body":"Bill"})"});
    assert(loaded.source().question_identity == direct.source().question_identity);
    assert(loaded.answer() == direct.answer());
    std::map<std::string, tt::toolbox_question> entries;
    for (const auto* name : {"ready", "topic", "names", "decision", "tags", "urgency", "priority", "find", "recognized"})
        entries.emplace(name, tools.select(name));
    tt::toolbox constructed(entries);
    auto saved = constructed.to_json();
    tools = tt::toolbox::from_json(saved);
    try { (void)tools.select("missing"); assert(false); }
    catch (const tt::error& error) { assert(code_name(error.code()) == "usage.name"); }
    try { (void)tools.select<tt::filter>("topic"); assert(false); }
    catch (const tt::error& error) { assert(code_name(error.code()) == "usage.name"); }
    std::cout << saved;
}
int main(int argc, char** argv) {
    if (argc == 3 && std::string_view(argv[1]) == "family") {
        tt::error value{static_cast<tt::errc>(std::stoi(argv[2])), "synthetic"};
        std::cout << value.family() << '\n';
        return 0;
    }
    if (argc == 2 && std::string_view(argv[1]) == "admission-surface") {
        std::cout << "cpp\n";
        return 0;
    }
    if (argc == 3 && std::string_view(argv[1]) == "toolbox-invalid") {
        try { (void)tt::toolbox::from_json(argv[2]); assert(false); }
        catch (const tt::error& error) { const auto prefix=code_name(error.code())+": ";
            const std::string_view message=error.what();assert(message.starts_with(prefix));
            // Shared admission driver protocol carries code and native message separately.
            std::cout << code_name(error.code()) << '\n' << message.substr(prefix.size()) << '\n'; }
        return 0;
    }
    if (argc == 3 && std::string_view(argv[1]) == "toolbox") {
        saved_toolbox(argv[2]);
        return 0;
    }
    {
        auto config = stub_settings();
        config.retry = tt::retry_policy{11, std::chrono::milliseconds{1}, std::chrono::milliseconds{120000}};
        try { tt::client rejected{config}; assert(false); }
        catch (const tt::error& error) { assert(error.code() == tt::errc::usage_settings && error.family() == "usage"); }
    }
    typed_signatures();
    snapshots_and_names();
    filter_originals();
    empty_domain();
    native_argument_errors();
    owned_answer_and_detail();
    callback_local_record();
    owned_collection();
    record_failures();
    moved_owners();
    source_exception();
}
