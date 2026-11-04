#ifndef THINKTHEN_CPP_NLOHMANN_HPP
#define THINKTHEN_CPP_NLOHMANN_HPP
#include <thinkthen/thinkthen.hpp>
#include <nlohmann/json.hpp>
namespace thinkthen { inline namespace v0_2 {
namespace detail {
[[nodiscard]] inline bool lossless(const nlohmann::json& value) {
    if (value.is_binary() || value.is_discarded()) return false;
    if (value.is_number_float()) return std::isfinite(value.get<double>());
    if (value.is_structured()) for (const auto& item : value) if (!lossless(item)) return false;
    return true;
}
}
/// Copy a JSON object into a record, preserving invalid host values for engine rejection.
[[nodiscard]] inline owned_json_text json_record(const nlohmann::json& value) {
    if (!detail::lossless(value)) return detail::record_encoding::invalid();
    try { return owned_json_text{value.dump()}; }
    catch (const nlohmann::json::exception&) { return detail::record_encoding::invalid(); }
}
/// Serialize the spa fields using the shared contract.
inline void to_json(nlohmann::json& j, const span& v) { j = {{"text", v.text}, {"start", v.start}, {"end", v.end}}; }
/// Serialize the name fields using the shared contract.
inline void to_json(nlohmann::json& j, const name& v) { j = {{"text", v.text}, {"start", v.start}, {"end", v.end}, {"kind", v.kind}, {"probability", v.probability}}; }
/// Serialize the endpoint fields using the shared contract.
inline void to_json(nlohmann::json& j, const endpoint& v) { j = {{"text", v.text}, {"start", v.start}, {"end", v.end}, {"kind", v.kind}}; }
/// Serialize the relation fields using the shared contract.
inline void to_json(nlohmann::json& j, const relation& v) {
    j = {{"source", v.source}, {"target", v.target}, {"relation", v.label}, {"probability", v.probability}};
    if (v.either) j["either"] = *v.either;
}
/// Serialize the label_detail fields using the shared contract.
inline void to_json(nlohmann::json& j, const label_detail& v) { j = {{"labels", v.labels}}; }
/// Serialize the level_detail fields using the shared contract.
inline void to_json(nlohmann::json& j, const level_detail& v) { j = {{"levels", v.levels}}; }
/// Serialize the boundary_probabilities fields using the shared contract.
inline void to_json(nlohmann::json& j, const boundary_probabilities& v) {
    j = {{"BEGIN", v.begin}, {"INSIDE", v.inside}, {"END", v.end}, {"SINGLE", v.single}, {"OUT", v.out}};
}
/// Serialize the piece_detail fields using the shared contract.
inline void to_json(nlohmann::json& j, const piece_detail& v) { j = {{"text", v.text}, {"start", v.start}, {"end", v.end}, {"tags", v.tags}}; }
/// Serialize the kind_detail fields using the shared contract.
inline void to_json(nlohmann::json& j, const kind_detail& v) { j = {{"text", v.text}, {"start", v.start}, {"end", v.end}, {"kinds", v.kinds}}; }
/// Serialize the recognition_detail fields using the shared contract.
inline void to_json(nlohmann::json& j, const recognition_detail& v) { j = {{"pieces", v.pieces}, {"names", v.names}}; }
/// Serialize the relation_detail fields using the shared contract.
inline void to_json(nlohmann::json& j, const relation_detail& v) { j = {{"names", v.names}, {"pairs", v.pairs}}; }
/// Serialize the error fields using the shared contract.
inline void to_json(nlohmann::json& j, const error& v) {
    j = {{"code", thinkthen_code_name(v.code().value())}, {"message", v.what()}};
    if (v.http_status()) j["http_status"] = *v.http_status();
}
/// Serialize the source fields using the shared contract.
inline void to_json(nlohmann::json& j, const source& v) {
    const char* backend = v.backend == backend::systemone ? "systemone" : v.backend == backend::openai ? "openai" : "stub";
    j = {{"question_identity", v.question_identity}, {"backend", backend}, {"model", v.model}};
    if (v.name) j["name"] = *v.name;
}
namespace detail {
[[nodiscard]] inline const char* function_name(thinkthen::function f) {
    switch (f) {
#define THINKTHEN_FUNCTION_NAME(Q) case function::Q: return #Q;
        THINKTHEN_FUNCTION_NAME(decide) THINKTHEN_FUNCTION_NAME(choose)
        THINKTHEN_FUNCTION_NAME(tag) THINKTHEN_FUNCTION_NAME(score)
        THINKTHEN_FUNCTION_NAME(filter) THINKTHEN_FUNCTION_NAME(rank)
        THINKTHEN_FUNCTION_NAME(find) THINKTHEN_FUNCTION_NAME(recognize)
        THINKTHEN_FUNCTION_NAME(relate) THINKTHEN_FUNCTION_NAME(annotate)
#undef THINKTHEN_FUNCTION_NAME
    }
    incompatible();
}
}
/// Serialize a typed outcome without raising its not-sure or failure state.
template<class Q> void to_json(nlohmann::json& j, const outcome<Q>& v) {
    j = {{"function", detail::function_name(Q::kind)}, {"status", v.status() == status::answered ? "answered" : v.status() == status::unsure ? "unsure" : "failed"}};
    if (v.status() == status::answered) {
        j["answer"] = nullptr;
        if (auto* answer = v.answer()) {
            if constexpr (std::is_same_v<Q, find>) { if (*answer) j["answer"] = **answer; }
            else if constexpr (std::is_same_v<Q, annotate>) {
                j["answer"] = nlohmann::json::object();
                for (const auto& e : *answer) std::visit([&](const auto& item) { j["answer"][e.first] = item; }, e.second);
            } else j["answer"] = *answer;
        }
    }
    if constexpr (!std::is_same_v<typename result_traits<Q>::detail, std::monostate>) if (v.detail()) j["detail"] = *v.detail();
    if (v.probability()) j["probability"] = *v.probability();
    if (v.failure()) j["error"] = *v.failure();
}
/// Serialize a typed row, its position, id and source.
template<class Q> void to_json(nlohmann::json& j, const row<Q>& v) {
    to_json(j, static_cast<const outcome<Q>&>(v));
    j["position"] = v.position();
    if (!v.id_json().empty()) j["id"] = nlohmann::json::parse(v.id_json());
    j["source"] = v.source();
}
}}
#endif
