#pragma once
#include <thinkthen/thinkthen.h>
#include "json.hpp"
#include <algorithm>
#include <memory>
#include <optional>
#include <stdexcept>
#include <string>
#include <type_traits>
#include <vector>
#include <map>
#include <variant>

namespace tt {
enum class Outcome { no = THINKTHEN_NO, yes = THINKTHEN_YES, notSure = THINKTHEN_UNSURE };
enum class ErrorKind { usage = 1, backend, deadline, local, cancelled, defect };
struct Failure : std::runtime_error {
    ErrorKind kind;
    bool retryable;
    std::optional<Json> facts;
    Failure(ErrorKind kind, std::string message, bool retryable, std::optional<Json> facts)
        : std::runtime_error(std::move(message)), kind(kind), retryable(retryable), facts(std::move(facts)) {}
};
struct EngineFree { void operator()(thinkthen_engine* p) const noexcept { thinkthen_engine_free(p); } };
struct TokenFree { void operator()(thinkthen_cancel_token* p) const noexcept { thinkthen_cancel_token_free(p); } };
struct StringFree { void operator()(char* p) const noexcept { thinkthen_free_string(p); } };
using Engine = std::unique_ptr<thinkthen_engine, EngineFree>;
using CancelToken = std::unique_ptr<thinkthen_cancel_token, TokenFree>;
using OwnedString = std::unique_ptr<char, StringFree>;
static_assert(!std::is_copy_constructible<Engine>::value, "engines cannot be copied");
static_assert(!std::is_copy_constructible<CancelToken>::value, "tokens cannot be copied");
static_assert(!std::is_copy_constructible<OwnedString>::value, "returned strings cannot be copied");
struct Judgment { Outcome outcome; double probability; };
using NullableOutcome = std::optional<Outcome>; // null means unresolved, never failure.
struct FailedField { ErrorKind kind; std::string cause; };
using AnnotatedDecision = std::variant<NullableOutcome, FailedField>;
inline NullableOutcome decisionValue(const Json& value) {
    if (value.is_null()) return std::nullopt;
    if (!value.is_boolean()) throw std::invalid_argument("decide value is not boolean or null");
    return value.get<bool>() ? Outcome::yes : Outcome::no;
}
inline AnnotatedDecision annotatedDecision(const Json& value) {
    if (!value.is_object() || !value.contains("failed")) return decisionValue(value);
    const auto& failed = value.at("failed");
    static const std::vector<std::string> names{"usage", "backend", "deadline", "local", "cancelled", "defect"};
    const auto kind = failed.at("kind").get<std::string>();
    auto it = std::find(names.begin(), names.end(), kind);
    if (it == names.end()) throw std::invalid_argument("unknown failed kind");
    return FailedField{static_cast<ErrorKind>(std::distance(names.begin(), it) + 1),
                       failed.at("cause").get<std::string>()};
}
// The JSON ABI carries descriptions unmodified, including structured {what,not_for,examples}.
inline std::string question(const Json& request) { return request.dump(); }
inline Failure failure(const thinkthen_engine* engine, int returned) {
    const int code = thinkthen_error_code(engine);
    if (code != returned) throw std::logic_error("thread-local error code differs from call return");
    // Both borrowed pointers must be copied before another failure on this calling thread.
    const char* message = thinkthen_error_message(engine);
    const char* borrowed = thinkthen_error_facts_json(engine);
    std::string copied = borrowed ? std::string(borrowed) : std::string();
    return Failure(static_cast<ErrorKind>(code), message ? std::string(message) : "",
                   thinkthen_error_retryable(engine) != 0,
                   borrowed ? std::optional<Json>(Json::parse(copied)) : std::nullopt);
}
inline Engine create(const char* settings = nullptr) {
    Engine engine(settings ? thinkthen_engine_new_with(settings) : thinkthen_engine_new());
    if (!engine) throw failure(nullptr, thinkthen_error_code(nullptr));
    return engine;
}
inline CancelToken token() {
    CancelToken result(thinkthen_cancel_token_new());
    if (!result) throw std::runtime_error("native cancel token allocation failed");
    return result;
}
inline Judgment decode(thinkthen_answer value) {
    if (value.outcome < THINKTHEN_NO || value.outcome > THINKTHEN_UNSURE) throw std::runtime_error("invalid outcome");
    return {static_cast<Outcome>(value.outcome), value.probability};
}
inline Judgment decide(const Engine& engine, const std::string& q, const std::string& text,
                       int64_t deadline = THINKTHEN_NO_DEADLINE, thinkthen_cancel_token* cancel = nullptr) {
    thinkthen_answer value{123, -1};
    int code = thinkthen_decide_opts(engine.get(), q.c_str(), text.data(), text.size(), deadline, cancel, &value);
    if (code) {
        if (value.outcome != 123 || value.probability != -1) throw std::logic_error("failure changed scalar output");
        throw failure(engine.get(), code);
    }
    return decode(value);
}
inline std::vector<Judgment> many(const Engine& engine, const std::string& q,
                                   const std::vector<std::string>& texts,
                                   int64_t deadline = THINKTHEN_NO_DEADLINE, thinkthen_cancel_token* cancel = nullptr) {
    std::vector<const char*> pointers;
    std::vector<size_t> lengths;
    for (const auto& s : texts) { pointers.push_back(s.data()); lengths.push_back(s.size()); }
    std::vector<thinkthen_answer> result(texts.size(), {123, -1});
    int code = thinkthen_decide_many_opts(engine.get(), q.c_str(), pointers.data(), lengths.data(),
                                          texts.size(), deadline, cancel, result.data());
    if (code) {
        for (auto value : result) if (value.outcome != 123 || value.probability != -1)
            throw std::logic_error("failure changed bulk output");
        throw failure(engine.get(), code);
    }
    std::vector<Judgment> judgments;
    for (auto value : result) judgments.push_back(decode(value));
    return judgments;
}
inline Json call(const Engine& engine, const Json& request) {
    const auto encoded = request.dump();
    OwnedString raw(thinkthen_call_opts(engine.get(), encoded.c_str(), THINKTHEN_NO_DEADLINE, nullptr));
    if (!raw) throw failure(engine.get(), thinkthen_error_code(engine.get()));
    return Json::parse(raw.get());
}
inline Json recognize(const Engine& engine, const std::string& spec, const std::string& text) {
    char* raw = nullptr; size_t length = 999;
    int code = thinkthen_recognize_opts(engine.get(), spec.c_str(), text.data(), text.size(),
                                        THINKTHEN_NO_DEADLINE, nullptr, &raw, &length);
    if (code) {
        if (raw || length != 999) throw std::logic_error("failure changed recognize outputs");
        throw failure(engine.get(), code);
    }
    OwnedString owned(raw);
    if (!owned) throw std::logic_error("successful recognize returned null");
    return Json::parse(std::string(owned.get(), length));
}
inline Json relate(const Engine& engine, const std::string& spec, const std::vector<std::string>& texts) {
    std::vector<const char*> pointers;
    std::vector<size_t> lengths;
    for (const auto& s : texts) { pointers.push_back(s.data()); lengths.push_back(s.size()); }
    char* raw = nullptr; size_t length = 999;
    int code = thinkthen_relate_opts(engine.get(), spec.c_str(), pointers.data(), lengths.data(), texts.size(),
                                     THINKTHEN_NO_DEADLINE, nullptr, &raw, &length);
    if (code) {
        if (raw || length != 999) throw std::logic_error("failure changed relate outputs");
        throw failure(engine.get(), code);
    }
    OwnedString owned(raw);
    if (!owned) throw std::logic_error("successful relate returned null");
    return Json::parse(std::string(owned.get(), length));
}
// Offsets are zero-based, end-exclusive Unicode scalar indices (not UTF-16).
struct Entity { std::string text; size_t startScalar; size_t endScalar; size_t lengthScalar; std::string kind; double strength; };
inline Entity entity(const Json& row) {
    Entity parsed{row.at("text").get<std::string>(), row.at("start").get<size_t>(),
                  row.at("end").get<size_t>(), row.at("length").get<size_t>(),
                  row.at("kind").get<std::string>(), row.at("strength").get<double>()};
    if (parsed.endScalar < parsed.startScalar || parsed.endScalar - parsed.startScalar != parsed.lengthScalar)
        throw std::invalid_argument("invalid zero-based end-exclusive Unicode scalar span");
    return parsed;
}
using AnnotatedField = std::variant<NullableOutcome, std::string, double, std::vector<std::string>, FailedField>;
using AnnotatedRow = std::map<std::string, AnnotatedField>;
inline AnnotatedField annotatedField(const Json& value) {
    if (value.is_null() || value.is_boolean()) return decisionValue(value);
    if (value.is_object()) {
        auto result = annotatedDecision(value);
        if (auto failed = std::get_if<FailedField>(&result)) return *failed;
        throw std::invalid_argument("invalid annotated object");
    }
    if (value.is_string()) return value.get<std::string>();
    if (value.is_number()) return value.get<double>();
    if (value.is_array()) {
        std::vector<std::string> names;
        for (const auto& name : value) names.push_back(name.get<std::string>());
        return names;
    }
    throw std::invalid_argument("invalid annotated field");
}
inline std::vector<AnnotatedRow> annotateTyped(const Json& value) {
    std::vector<AnnotatedRow> rows;
    for (const auto& row : value) {
        AnnotatedRow decoded;
        for (const auto& [key, field] : row.object()) decoded.emplace(key, annotatedField(field));
        rows.push_back(std::move(decoded));
    }
    return rows;
}
struct RecognizedRelation { std::string relation; Entity source; Entity target; double probability; };
struct Recognized { std::vector<Entity> entities; std::vector<RecognizedRelation> relations; };
inline Recognized recognizeTyped(const Json& value) {
    Recognized result;
    for (const auto& row : value.at("entities")) result.entities.push_back(entity(row));
    if (value.contains("relations")) for (const auto& row : value.at("relations")) {
        result.relations.push_back({row.at("relation").get<std::string>(),
            entity(row.at("source")), entity(row.at("target")), row.at("probability").get<double>()});
    }
    return result;
}
struct RelatedEntity { std::string name; std::string kind; };
struct Edge { std::string relation; RelatedEntity source; RelatedEntity target; double probability; };
inline RelatedEntity relatedEntity(const Json& row) {
    return {row.at("name").get<std::string>(),row.at("kind").get<std::string>()};
}
inline std::vector<Edge> relateTyped(const Json& value) {
    std::vector<Edge> edges;
    for (const auto& row : value.at("edges")) edges.push_back({row.at("relation").get<std::string>(),
        relatedEntity(row.at("source")),relatedEntity(row.at("target")),row.at("probability").get<double>()});
    return edges;
}
} // namespace tt
