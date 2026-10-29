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
static_assert(std::ranges::input_range<thinkthen::stream<thinkthen::decide>>);
#endif
namespace tt = thinkthen;

static tt::settings stub_settings() {
    tt::settings settings;
    settings.backend = tt::backend::stub;
    return settings;
}

static std::string code_name(tt::errc code) {
    return thinkthen_code_name(static_cast<std::int32_t>(code));
}

/* Feature: required domains and separate record encodings are C++ types. */
static void typed_signatures() {
    static_assert(!std::is_copy_constructible_v<tt::client>);
    static_assert(!std::is_constructible_v<tt::choose, std::string>);
    static_assert(!std::is_constructible_v<tt::tag, std::string>);
    static_assert(!std::is_constructible_v<tt::score, std::string>);
    static_assert(!std::is_constructible_v<tt::rank, std::string>);
    static_assert(!std::is_default_constructible_v<tt::recognize>);
    static_assert(!std::is_default_constructible_v<tt::relate>);
    static_assert(!std::is_same_v<tt::text, tt::json_text>);
    static_assert(!std::is_same_v<tt::record, tt::owned_record>);
    static_assert(std::is_same_v<decltype(std::declval<const tt::row<tt::choose>&>().value()),
                                 const std::string*>);
}

/* Feature: C++ questions admit their required lists through the engine. */
static void empty_domain() {
    try {
        tt::choose question{"Which team?", std::vector<std::string>{}};
        (void)question;
        assert(false);
    } catch (const tt::error& error) {
        assert(code_name(error.code()) == "usage.question");
    }
}

/* Feature: a copied typed row retains answer and detail after native cleanup. */
static void owned_answer_and_detail() {
    const auto retained = [] {
        tt::client owner(stub_settings());
        auto row = owner.choose(tt::choose{"Which team?", {"billing", "shipping"}},
                                tt::text{"refund please"});
        assert(row.status() == tt::status::answered);
        assert(row.value() != nullptr);
        assert(row.failure() == nullptr);
        return row;
    }();
    assert(retained.position() == 1);
    const auto* value = retained.value();
    assert(*value == "billing" || *value == "shipping");
    const auto detail = retained.detail();
    assert(detail.has_value());
    assert(detail->labels.size() == 2);
    assert(detail->labels.at(*value) == 1.0);
    const auto copy = retained;
    assert(*copy.value() == *value);
    assert(copy.detail()->labels == detail->labels);
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
        assert(row.value() != nullptr);
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
    std::vector<tt::row<tt::decide>> retained;
    for (const auto& row : rows) retained.push_back(row);
    assert(retained.size() == 2);
    assert(retained[0].position() == 1 && retained[1].position() == 2);
    assert(retained[0].id_json() == "-9223372036854775808");
    assert(retained[1].id_json() == "\"second\"");
    assert(retained[0].value() && retained[1].value());
}

/* Feature: missing records and invalid ids stay typed failed rows. */
static void record_failures() {
    tt::client owner(stub_settings());
    const auto missing = owner.decide(tt::decide{"Ready?"}, tt::missing_record{});
    assert(missing.status() == tt::status::failed);
    assert(missing.value() == nullptr && !missing.probability());
    assert(missing.failure());
    assert(code_name(missing.failure()->code()) == "record.missing");
    const auto invalid = owner.decide(tt::decide{"Ready?"},
        tt::json_text{"{\"id\":{\"nested\":1},\"body\":\"evidence\"}"});
    assert(invalid.status() == tt::status::failed);
    assert(invalid.value() == nullptr && invalid.failure());
    assert(code_name(invalid.failure()->code()) == "record.invalid");
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
    for (const auto& row : moved) { assert(row.value()); ++count; }
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
    assert(row.value());
}

/* Feature: immutable questions preserve options, execution names and toolbox domains. */
static void snapshots_and_names() {
    tt::client owner(stub_settings());
    auto original = tt::choose{"Which?", {"red", "blue"}};
    tt::toolbox tools{{{"color", original}, {"rank", tt::rank{"How much?", {"low", "high"}}}}};
    const auto selected = tools.select<tt::choose>("color");
    const auto unnamed = owner.choose(selected, tt::text{"evidence"});
    const auto named = owner.choose(selected, tt::text{"evidence"}, "execution");
    assert(!unnamed.source().name && named.source().name == "execution");
    assert(unnamed.source().question_identity != named.source().question_identity);
    assert(*unnamed.value() == *named.value());
    assert(std::holds_alternative<tt::rank>(tools.select("rank")));
    try { (void)tools.select<tt::decide>("color"); assert(false); }
    catch (const tt::error& error) { assert(error.code() == tt::errc::usage_name); }
    const auto band = original.cutoff(tt::threshold::band(0.2, 0.8));
    assert(owner.choose(band, tt::text{"evidence"}).source().question_identity != unnamed.source().question_identity);
    assert(owner.choose(original, tt::text{"evidence"}).source().question_identity == unnamed.source().question_identity);
    tt::annotation_questions entries{{"ready", tt::decide{"Ready?"}}};
    const auto annotation = owner.annotate(tt::annotate{entries}, tt::text{"evidence"});
    assert(annotation.value() && annotation.value()->size() == 1);
    assert(std::get<tt::outcome<tt::decide>>(annotation.value()->at("ready")).value());
    std::error_code code = tt::errc::record_missing;
    assert(code.category() == tt::category() && code.message() == "record.missing");
}
/* Feature: filter retains each original record even when the record fails. */
static void filter_originals() {
    tt::client owner(stub_settings());
    std::vector<tt::row<tt::filter>> retained;
    for (const auto& row : owner.filter_each(tt::filter{"Ready?"},
            std::vector<tt::owned_record>{tt::owned_text{"alpha"}, tt::missing_record{}, tt::owned_json_text{"{\"body\":\"beta\"}"}})) retained.push_back(row);
    assert(retained.size() == 3);
    assert(std::get<tt::owned_text>(retained[0].item()).value == "alpha");
    assert(std::holds_alternative<tt::missing_record>(retained[1].item()));
    assert(retained[1].failure() && !retained[1].kept());
    assert(std::get<tt::owned_json_text>(retained[2].item()).value == "{\"body\":\"beta\"}");
}

/* Feature: C++ saved toolboxes hydrate native typed snapshots through engine admission. */
static void saved_toolbox(const char* json) {
    auto tools = tt::toolbox::from_json(json);
    assert(std::holds_alternative<tt::filter>(tools.select("ready")));
    assert(std::holds_alternative<tt::relate>(tools.select("names")));
    const auto topic = tools.select<tt::choose>("topic");
    tt::client client(stub_settings());
    const auto loaded = client.choose(topic, tt::json_text{R"({"body":"Bill"})"});
    const auto direct = client.choose(tt::choose{"Which topic?", {"billing", "shipping"}}.evidence("/body").cutoff(tt::threshold::cutoff(0.7)), tt::json_text{R"({"body":"Bill"})"});
    assert(loaded.source().question_identity == direct.source().question_identity);
    assert(loaded.value() && direct.value() && *loaded.value() == *direct.value());
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
        catch (const tt::error& error) { std::cout << code_name(error.code()) << '\n' << error.what() << '\n'; }
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
    owned_answer_and_detail();
    callback_local_record();
    owned_collection();
    record_failures();
    moved_owners();
    source_exception();
}
