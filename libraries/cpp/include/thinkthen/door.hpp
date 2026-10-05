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
// Facts are the engine's facts object; the result schema describes its members.
template <typename T> struct CallResult { T value; Json facts; };
inline Json ownedJson(const OwnedString& text, size_t length) {
    if (!text) throw std::logic_error("successful call returned null JSON");
    return Json::parse(std::string(text.get(), length));
}
using NullableOutcome = std::optional<Outcome>; // null means unresolved, never failure.
struct FailedField { ErrorKind kind; std::string cause; };
struct Unresolved {};
// One annotate answer member: JSON null, any other value, or {"failed": {...}}.
// No answered value is an object, so an object that is not a failure throws.
using AnnotatedField = std::variant<Unresolved, Json, FailedField>;
inline AnnotatedField annotatedField(const Json& value) {
    if (value.is_null()) return Unresolved{};
    if (!value.is_object()) return value;
    if (!value.contains("failed") || !value.at("failed").is_object()) throw std::invalid_argument("invalid annotated object");
    const auto& failed = value.at("failed");
    static const std::vector<std::string> names{"usage", "backend", "deadline", "local", "cancelled", "defect"};
    const auto kind = failed.at("kind").get<std::string>();
    auto it = std::find(names.begin(), names.end(), kind);
    if (it == names.end()) throw std::invalid_argument("unknown failed kind");
    return FailedField{static_cast<ErrorKind>(std::distance(names.begin(), it) + 1),
                       failed.at("cause").get<std::string>()};
}
using AnnotatedDecision = std::variant<NullableOutcome, FailedField>;
inline NullableOutcome decisionValue(const Json& value) {
    if (value.is_null()) return std::nullopt;
    if (!value.is_boolean()) throw std::invalid_argument("decide value is not boolean or null");
    return value.get<bool>() ? Outcome::yes : Outcome::no;
}
inline AnnotatedDecision annotatedDecision(const Json& value) {
    auto field = annotatedField(value);
    if (auto failed = std::get_if<FailedField>(&field)) return *failed;
    return decisionValue(value);
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
inline CallResult<Judgment> decide(const Engine& engine, const std::string& q, const std::string& text,
                       int64_t deadline = THINKTHEN_NO_DEADLINE, thinkthen_cancel_token* cancel = nullptr) {
    thinkthen_answer value{123, -1};
    char* rawFacts = nullptr; size_t factsLength = 999;
    int code = thinkthen_decide_with_facts_opts(engine.get(), q.c_str(), text.data(), text.size(),
                                                deadline, cancel, &value, &rawFacts, &factsLength);
    OwnedString facts(rawFacts);
    if (code) {
        if (value.outcome != 123 || value.probability != -1 || rawFacts || factsLength != 999)
            throw std::logic_error("failure changed scalar outputs");
        throw failure(engine.get(), code);
    }
    return {decode(value), ownedJson(facts, factsLength)};
}
inline CallResult<std::vector<Judgment>> many(const Engine& engine, const std::string& q,
                                   const std::vector<std::string>& texts,
                                   int64_t deadline = THINKTHEN_NO_DEADLINE, thinkthen_cancel_token* cancel = nullptr) {
    std::vector<const char*> pointers;
    std::vector<size_t> lengths;
    for (const auto& s : texts) { pointers.push_back(s.data()); lengths.push_back(s.size()); }
    std::vector<thinkthen_answer> result(texts.size(), {123, -1});
    char* rawFacts = nullptr; size_t factsLength = 999;
    int code = thinkthen_decide_many_with_facts_opts(engine.get(), q.c_str(), pointers.data(), lengths.data(),
                                                     texts.size(), deadline, cancel, result.data(), &rawFacts, &factsLength);
    OwnedString facts(rawFacts);
    if (code) {
        for (auto value : result) if (value.outcome != 123 || value.probability != -1)
            throw std::logic_error("failure changed bulk output");
        if (rawFacts || factsLength != 999) throw std::logic_error("failure changed bulk facts output");
        throw failure(engine.get(), code);
    }
    std::vector<Judgment> judgments;
    for (auto value : result) judgments.push_back(decode(value));
    return {std::move(judgments), ownedJson(facts, factsLength)};
}
inline Json call(const Engine& engine, const Json& request,
                 int64_t deadline = THINKTHEN_NO_DEADLINE, thinkthen_cancel_token* cancel = nullptr) {
    const auto encoded = request.dump();
    OwnedString raw(thinkthen_call_opts(engine.get(), encoded.c_str(), deadline, cancel));
    if (!raw) throw failure(engine.get(), thinkthen_error_code(engine.get()));
    return Json::parse(raw.get());
}
// Explicit source selection for every verb; typed text/record APIs keep their inputs.
inline Json files(const Engine& engine, const Json& question, const std::vector<std::string>& paths,
                  const std::string& unit = "line", const Json& window = Json(nullptr),
                  int64_t deadline = THINKTHEN_NO_DEADLINE, thinkthen_cancel_token* cancel = nullptr) {
    if (!question.is_object() || question.contains("source")) throw std::invalid_argument("files takes question members alone");
    Json::Array names; for (const auto& path : paths) names.emplace_back(path);
    Json::Object selection{{"paths", Json(std::move(names))}, {"unit", Json(unit)}};
    if (!window.is_null()) selection.emplace_back("window", window);
    auto request = question.object(); request.emplace_back("source", Json(std::move(selection)));
    return call(engine, Json(std::move(request)), deadline, cancel);
}
inline CallResult<Json> recognize(const Engine& engine, const std::string& spec, const std::string& text,
                                  int64_t deadline = THINKTHEN_NO_DEADLINE, thinkthen_cancel_token* cancel = nullptr) {
    char* raw = nullptr; size_t length = 999; char* rawFacts = nullptr; size_t factsLength = 999;
    int code = thinkthen_recognize_with_facts_opts(engine.get(), spec.c_str(), text.data(), text.size(),
                                                   deadline, cancel, &raw, &length, &rawFacts, &factsLength);
    OwnedString owned(raw), facts(rawFacts);
    if (code) {
        if (raw || length != 999 || rawFacts || factsLength != 999)
            throw std::logic_error("failure changed recognize outputs");
        throw failure(engine.get(), code);
    }
    return {ownedJson(owned, length), ownedJson(facts, factsLength)};
}
inline CallResult<Json> relate(const Engine& engine, const std::string& spec, const std::vector<std::string>& texts,
                               int64_t deadline = THINKTHEN_NO_DEADLINE, thinkthen_cancel_token* cancel = nullptr) {
    std::vector<const char*> pointers;
    std::vector<size_t> lengths;
    for (const auto& s : texts) { pointers.push_back(s.data()); lengths.push_back(s.size()); }
    char* raw = nullptr; size_t length = 999; char* rawFacts = nullptr; size_t factsLength = 999;
    int code = thinkthen_relate_with_facts_opts(engine.get(), spec.c_str(), pointers.data(), lengths.data(), texts.size(),
                                                deadline, cancel, &raw, &length, &rawFacts, &factsLength);
    OwnedString owned(raw), facts(rawFacts);
    if (code) {
        if (raw || length != 999 || rawFacts || factsLength != 999)
            throw std::logic_error("failure changed relate outputs");
        throw failure(engine.get(), code);
    }
    return {ownedJson(owned, length), ownedJson(facts, factsLength)};
}
// Preview a decide, choose, score or tag call through thinkthen_plan_json and
// return the result schema's plan object. The question is bare text (a JSON
// string) or one question object; settings is null or a thinkthen.settings/1
// object. The preview needs no key, reads no cache and sends nothing.
inline Json plan(const Engine& engine, const std::string& verb, const Json& question,
                 const std::vector<std::string>& input, const Json& settings = nullptr) {
    Json::Array texts(input.begin(), input.end());
    Json::Object request{{"verb", verb}, {"question", question}, {"input", std::move(texts)}};
    if (!settings.is_null()) request.emplace_back("settings", settings);
    const auto encoded = Json(std::move(request)).dump();
    char* raw = nullptr; size_t length = 999;
    int code = thinkthen_plan_json(engine.get(), encoded.c_str(), &raw, &length);
    OwnedString owned(raw);
    if (code) throw failure(engine.get(), code);
    return ownedJson(owned, length);
}
} // namespace tt
