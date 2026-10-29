#include <thinkthen/nlohmann.hpp>
#include <cassert>
#include <iostream>
#include <limits>
namespace tt = thinkthen;
using json = nlohmann::json;
using any_question = std::variant<tt::decide, tt::choose, tt::tag, tt::score, tt::filter, tt::rank, tt::find, tt::recognize, tt::relate, tt::annotate>;
// Feature: annotation construction refuses ranking at the public type boundary.
static_assert(!std::is_constructible_v<tt::annotation_question, tt::rank>);

template<class Q> Q options(Q q, const json& value) {
    if (value.contains("evidence")) q = q.evidence(value.at("evidence").get<std::string>());
    if constexpr (std::is_same_v<Q, tt::decide> || std::is_same_v<Q, tt::choose> || std::is_same_v<Q, tt::filter>) {
        if (value.contains("threshold")) {
            const auto& t = value.at("threshold");
            q = q.cutoff(t.is_number() ? tt::threshold::cutoff(t.get<double>()) : tt::threshold::band(t.at("low"), t.at("high")));
        }
    } else if constexpr (std::is_same_v<Q, tt::tag> || std::is_same_v<Q, tt::find> || std::is_same_v<Q, tt::recognize> || std::is_same_v<Q, tt::relate>) {
        if (value.contains("threshold")) q = q.cutoff(value.at("threshold").get<double>());
    } else if constexpr (std::is_same_v<Q, tt::rank>) {
        if (value.contains("top")) q = q.top(value.at("top").get<std::size_t>());
    }
    return q;
}
static any_question question(const json& q) {
    const std::string f = q.at("function");
    const auto text = q.value("question", std::string{});
    if (f == "decide") return options(tt::decide{text}, q);
    if (f == "choose") return options(tt::choose{text, q.at("choices").get<std::vector<std::string>>()}, q);
    if (f == "tag") return options(tt::tag{text, q.at("labels").get<std::vector<std::string>>()}, q);
    if (f == "score") return options(tt::score{text, q.at("levels").get<std::vector<std::string>>()}, q);
    if (f == "filter") return options(tt::filter{text}, q);
    if (f == "rank") return options(tt::rank{text, q.at("levels").get<std::vector<std::string>>()}, q);
    if (f == "find") return options(tt::find{text}, q);
    if (f == "recognize") return options(tt::recognize{q.at("kinds").get<std::vector<std::string>>()}, q);
    if (f == "relate") {
        std::vector<tt::rule> rules;
        for (const auto& r : q.at("rules")) rules.push_back({r.at("relation"), r.at("source"), r.at("target"),
            r.contains("reads") ? std::optional<std::string>{r.at("reads").get<std::string>()} : std::nullopt, r.value("either", false)});
        return options(tt::relate{rules}, q);
    }
    assert(f == "annotate");
    tt::annotation_questions entries;
    for (auto it = q.at("questions").begin(); it != q.at("questions").end(); ++it) {
        std::visit([&](const auto& child) {
            using Q = std::decay_t<decltype(child)>;
            if constexpr (!std::is_same_v<Q, tt::rank> && !std::is_same_v<Q, tt::annotate>) entries.emplace(it.key(), child);
            else assert(false);
        }, question(it.value()));
    }
    return options(tt::annotate{entries}, q);
}
static tt::settings settings(const json& value) {
    tt::settings result;
    const std::string backend = value.value("backend", "systemone");
    result.backend = backend == "stub" ? tt::backend::stub : backend == "openai" ? tt::backend::openai : tt::backend::systemone;
    if (value.contains("model") && !value.at("model").is_null()) result.model = value.at("model");
    if (value.contains("url") && !value.at("url").is_null()) result.url = value.at("url");
    if (value.contains("api_key") && !value.at("api_key").is_null()) result.api_key = tt::api_key{value.at("api_key").get<std::string>()};
    if (value.contains("batch") && !value.at("batch").is_null()) result.batch = value.at("batch");
    if (value.contains("retry")) {
        const auto& r = value.at("retry");
        // Fixture-only scheduling bounds; these do not claim engine defaults.
        result.retry = tt::retry_policy{r.at("max_retries"), std::chrono::milliseconds{r.value("initial_backoff_ms", std::int64_t{1})},
            std::chrono::milliseconds{r.value("attempt_timeout_ms", std::int64_t{10000})}};
    }
    return result;
}
#include "stress.hpp"

template<class Q> static void append_row(json& rows, json& kept, const tt::row<Q>& row) {
    rows.push_back(row);
    if constexpr (std::is_same_v<Q, tt::filter>) kept.push_back(row.kept());
}

int main(int argc, char** argv) {
    if (argc > 1 && std::string{argv[1]}.find("stress") == 0) return stress_main(argc, argv);
    if (argc == 1) {
        tt::settings config; config.backend = tt::backend::stub;
        tt::client conversion_owner{config};
        // Feature: optional JSON conversion keeps unrepresentable records as failed rows.
        for (const auto& invalid : std::vector<json>{std::numeric_limits<double>::quiet_NaN(), std::string(1, static_cast<char>(0xff)),
                json::binary({1, 2}), json::parse("invalid", nullptr, false),
                json{{"body", json::binary({3})}}, json{{"body", json::parse("invalid", nullptr, false)}}}) {
            auto rows = conversion_owner.decide_each(tt::decide{"Ready?"}, std::vector<tt::owned_record>{tt::json_record(invalid)});
            auto failed = rows.next();
            assert(failed && failed->status() == tt::status::failed && failed->failure()->code() == tt::errc::record_invalid);
            assert(!rows.next());
        }
        tt::settings s; s.backend = tt::backend::stub;
        auto row = tt::client{s}.decide(tt::decide{"Ready?"}, tt::missing_record{});
        assert(row.failure() && row.failure()->code() == tt::errc::record_missing);
        return 0;
    }
    assert(argc == 3 || (argc == 4 && (std::string{argv[3]} == "cancelled" || std::string{argv[3]} == "shortcut")));
    const bool shortcut = argc == 4 && std::string{argv[3]} == "shortcut";
    json rows = json::array();
    json kept = json::array();
    try {
        const auto request = json::parse(argv[1]);
        tt::client owner{settings(json::parse(argv[2]))};
        // Saved entries admit each representable question before typed conversion.
        const auto& serialized = request.at("question");
        json entries = json::object();
        if (serialized.at("function") == "annotate") {
            for (auto it = serialized.at("questions").begin(); it != serialized.at("questions").end(); ++it)
                entries[it.key()] = {{"question", it.value()}};
        } else entries["case"] = {{"question", serialized}};
        (void)tt::toolbox::from_json(json{{"entries", entries}}.dump());
        auto q = question(request.at("question"));
        std::vector<tt::owned_record> records;
        for (const auto& r : request.value("records", json::array())) {
            if (r.is_null()) records.push_back(tt::missing_record{});
            else if (r.is_string()) records.push_back(tt::owned_text{r.get<std::string>()});
            else records.push_back(tt::json_record(r));
        }
        std::optional<std::string> name;
        if (request.contains("name")) name = request.at("name");
        std::optional<std::string_view> named = name ? std::optional<std::string_view>{*name} : std::nullopt;
        std::string single_text;
        tt::record single = tt::missing_record{};
        if (shortcut) {
            assert(request.at("records").size() == 1);
            const auto& input = request.at("records").front();
            if (!input.is_null()) {
                single_text = input.is_string() ? input.get<std::string>() : input.dump();
                single = input.is_string() ? tt::record{tt::text{single_text}} : tt::record{tt::json_text{single_text}};
            }
        }
        std::visit([&](const auto& value) {
            using Q = std::decay_t<decltype(value)>;
#define THINKTHEN_COLLECT(QNAME) if constexpr (std::is_same_v<Q, tt::QNAME>) { if (shortcut) append_row(rows, kept, owner.QNAME(value, single, named)); else { auto stream = owner.QNAME##_each(value, records, named); if (argc == 4) stream.cancel_handle().request_cancel(); for (const auto& row : stream) append_row(rows, kept, row); } }
            THINKTHEN_COLLECT(decide) THINKTHEN_COLLECT(choose)
            THINKTHEN_COLLECT(tag) THINKTHEN_COLLECT(score)
            THINKTHEN_COLLECT(filter) THINKTHEN_COLLECT(rank)
            THINKTHEN_COLLECT(find) THINKTHEN_COLLECT(recognize)
            THINKTHEN_COLLECT(relate) THINKTHEN_COLLECT(annotate)
#undef THINKTHEN_COLLECT
        }, q);
    } catch (const tt::error& error) {
        std::cout << json{{"rows", rows}, {"kept", kept}, {"error", thinkthen_code_name(static_cast<int>(error.code()))}} << '\n';
        return 0;
    }
    std::cout << json{{"rows", rows}, {"kept", kept}} << '\n';
}
