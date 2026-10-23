#ifndef THINKTHEN_ANSWERS_HPP
#define THINKTHEN_ANSWERS_HPP

#include <nlohmann/json.hpp>
#include <cstddef>
#include <map>
#include <optional>
#include <string>
#include <vector>

namespace thinkthen {
using json = nlohmann::json;
enum class status { answered, unsure, failed };
struct span { std::string text; std::size_t start; std::size_t end; };
struct name { std::string text; std::size_t start; std::size_t end; std::string kind; double p; };
struct endpoint { std::string text; std::size_t start; std::size_t end; std::string kind; };
struct relation { endpoint source; endpoint target; std::string relation; double p; std::optional<bool> either; };
struct failure { std::string code; std::string message; };
struct annotation {
    thinkthen::status status;
    json raw_answer;
    std::optional<json> detail;
    std::optional<double> p;
    std::optional<failure> error;
    template<class Answer> std::optional<Answer> answer() const {
        return raw_answer.is_null() ? std::nullopt : std::optional<Answer>(raw_answer.get<Answer>());
    }
};
using annotations = std::map<std::string, annotation>;
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(span, text, start, end)
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(name, text, start, end, kind, p)
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(endpoint, text, start, end, kind)
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(failure, code, message)
inline void from_json(const json& value, relation& result) {
    value.at("source").get_to(result.source);
    value.at("target").get_to(result.target);
    value.at("relation").get_to(result.relation);
    value.at("p").get_to(result.p);
    result.either = value.contains("either") ? std::optional<bool>(value.at("either").get<bool>()) : std::nullopt;
}
inline void to_json(json& value, const relation& result) {
    value = {{"source", result.source}, {"target", result.target}, {"relation", result.relation}, {"p", result.p}};
    if (result.either) value["either"] = *result.either;
}
inline void from_json(const json& value, annotation& result) {
    const auto state = value.at("status").get<std::string>();
    result.status = state == "answered" ? status::answered : state == "unsure" ? status::unsure : status::failed;
    result.raw_answer = value.value("answer", json());
    result.detail = value.contains("detail") ? std::optional<json>(value.at("detail")) : std::nullopt;
    result.p = value.contains("p") ? std::optional<double>(value.at("p").get<double>()) : std::nullopt;
    result.error = value.contains("error") ? std::optional<failure>(value.at("error").get<failure>()) : std::nullopt;
}
inline void to_json(json& value, const annotation& result) {
    value = {{"status", result.status == status::answered ? "answered" : result.status == status::unsure ? "unsure" : "failed"}};
    if (result.status != status::failed) value["answer"] = result.raw_answer;
    if (result.detail) value["detail"] = *result.detail;
    if (result.p) value["p"] = *result.p;
    if (result.error) value["error"] = *result.error;
}
}
#endif
