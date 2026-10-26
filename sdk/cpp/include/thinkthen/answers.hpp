#ifndef THINKTHEN_ANSWERS_HPP
#define THINKTHEN_ANSWERS_HPP
#include <thinkthen/questions.hpp>
#include <thinkthen/settings.hpp>
#include <map>
#include <functional>
#include <cmath>
#include <variant>
namespace thinkthen { inline namespace v0_2 {
enum class status : std::uint32_t {
    answered = THINKTHEN_STATUS_ANSWERED, unsure = THINKTHEN_STATUS_UNSURE, failed = THINKTHEN_STATUS_FAILED
};
struct span { std::string text; std::uint64_t start, end; };
struct name { std::string text; std::uint64_t start, end; std::string kind; double probability; };
struct endpoint { std::string text; std::uint64_t start, end; std::string kind; };
struct relation { endpoint source, target; std::string label; double probability; std::optional<bool> either; };
struct label_detail { std::map<std::string, double> labels; };
struct level_detail { std::vector<double> levels; };
struct boundary_probabilities { double begin, inside, end, single, out; };
struct piece_detail { std::string text; std::uint64_t start, end; boundary_probabilities tags; };
struct kind_detail { std::uint64_t start, end; std::map<std::string, double> kinds; };
struct recognition_detail { std::vector<piece_detail> pieces; std::vector<kind_detail> names; };
struct relation_detail { std::vector<name> names; std::vector<relation> pairs; };
struct source { std::string question_identity; thinkthen::backend backend; std::string model; std::optional<std::string> name; };
struct text { std::string_view value; };
struct json_text { std::string_view value; };
struct missing_record {};
using record = std::variant<text, json_text, missing_record>;
struct owned_text { std::string value; };
namespace detail { struct record_encoding; }
struct owned_json_text {
    std::string value;
    explicit owned_json_text(std::string bytes = {}) : value(std::move(bytes)) {}
private:
    bool invalid_ = false;
    friend struct detail::record_encoding;
};
namespace detail {
struct record_encoding {
    static owned_json_text invalid() { owned_json_text value; value.invalid_ = true; return value; }
    static bool invalid(const owned_json_text& value) { return value.invalid_; }
};
}
using owned_record = std::variant<owned_text, owned_json_text, missing_record>;
using record_source = std::function<std::optional<owned_record>()>;
template<class Q> struct result_traits;
#define THINKTHEN_RESULT(Q, A, D) template<> struct result_traits<Q> { using answer = A; using detail = D; };
THINKTHEN_RESULT(decide, bool, std::monostate)
THINKTHEN_RESULT(filter, bool, std::monostate)
THINKTHEN_RESULT(choose, std::string, label_detail)
THINKTHEN_RESULT(tag, std::vector<std::string>, label_detail)
THINKTHEN_RESULT(score, double, level_detail)
THINKTHEN_RESULT(rank, std::uint64_t, level_detail)
THINKTHEN_RESULT(find, std::optional<span>, std::monostate)
THINKTHEN_RESULT(recognize, std::vector<name>, recognition_detail)
THINKTHEN_RESULT(relate, std::vector<relation>, relation_detail)
template<class Q> class outcome;
using annotation = std::variant<outcome<decide>, outcome<choose>, outcome<tag>, outcome<score>, outcome<filter>, outcome<find>, outcome<recognize>, outcome<relate>>;
using annotations = std::map<std::string, annotation>;
THINKTHEN_RESULT(annotate, annotations, std::monostate)
#undef THINKTHEN_RESULT
namespace detail {
using value_owner = owner<thinkthen_value, thinkthen_value_free>;
inline thinkthen::status status(std::uint32_t value) {
    switch (value) {
        case THINKTHEN_STATUS_ANSWERED: return thinkthen::status::answered;
        case THINKTHEN_STATUS_UNSURE: return thinkthen::status::unsure;
        case THINKTHEN_STATUS_FAILED: return thinkthen::status::failed;
        default: incompatible();
    }
}
inline thinkthen::backend backend(std::uint32_t value) {
    switch (value) {
        case THINKTHEN_BACKEND_SYSTEMONE: return thinkthen::backend::systemone;
        case THINKTHEN_BACKEND_OPENAI: return thinkthen::backend::openai;
        case THINKTHEN_BACKEND_STUB: return thinkthen::backend::stub;
        default: incompatible();
    }
}
inline name name_value(const thinkthen_name& n) { return {copy(n.text), n.start, n.end, copy(n.kind), n.probability}; }
inline endpoint endpoint_value(const thinkthen_endpoint& v) { return {copy(v.text), v.start, v.end, copy(v.kind)}; }
inline relation relation_value(const thinkthen_relation& v) {
    if (v.either > 1) incompatible();
    return {endpoint_value(v.source), endpoint_value(v.target), copy(v.relation), v.probability,
        v.either ? std::optional<bool>{true} : std::nullopt};
}
inline span span_value(const thinkthen_span& v) { return {copy(v.text), v.start, v.end}; }
template<class T, class Count, class At, class Convert> inline auto collection(const thinkthen_value* v, Count count, At at, Convert convert) {
    using result = decltype(convert(T{}));
    std::vector<result> values;
    auto n = get<std::size_t>(count, v);
    values.reserve(n);
    for (std::size_t i = 0; i < n; ++i) values.push_back(convert(get<T>(at, v, i)));
    return values;
}
annotations annotation_values(const thinkthen_value* v);
template<class Q> typename result_traits<Q>::answer answer(const thinkthen_value* v) {
    if constexpr (std::is_same_v<Q, decide> || std::is_same_v<Q, filter>) {
        const auto value = get<std::uint8_t>(thinkthen_value_bool, v);
        if (value > 1) incompatible();
        return value != 0;
    } else if constexpr (std::is_same_v<Q, choose>) return copy(get<thinkthen_text>(thinkthen_value_choice, v));
    else if constexpr (std::is_same_v<Q, score>) return get<double>(thinkthen_value_number, v);
    else if constexpr (std::is_same_v<Q, rank>) return get<std::uint64_t>(thinkthen_value_rank, v);
    else if constexpr (std::is_same_v<Q, tag>) return collection<thinkthen_text>(v, thinkthen_value_tag_count, thinkthen_value_tag_at, [](auto value) { return copy(value); });
    else if constexpr (std::is_same_v<Q, find>) {
        auto value = get<thinkthen_span>(thinkthen_value_found, v);
        if (value.present > 1) incompatible();
        return value.present ? std::optional<span>{span_value(value)} : std::nullopt;
    } else if constexpr (std::is_same_v<Q, recognize>) return collection<thinkthen_name>(v, thinkthen_value_name_count, thinkthen_value_name_at,
        name_value);
    else if constexpr (std::is_same_v<Q, relate>) return collection<thinkthen_relation>(v, thinkthen_value_relation_count, thinkthen_value_relation_at, relation_value);
    else return annotation_values(v);
}
inline label_detail labels(const thinkthen_value* v) {
    label_detail result;
    auto n = get<std::size_t>(thinkthen_value_label_count, v);
    for (std::size_t i = 0; i < n; ++i) {
        auto p = get<thinkthen_probability>(thinkthen_value_label_at, v, i);
        result.labels.emplace(copy(p.label), p.probability);
    }
    return result;
}
inline recognition_detail recognition(const thinkthen_value* v) {
    recognition_detail result;
    auto n = get<std::size_t>(thinkthen_value_piece_count, v);
    for (std::size_t i = 0; i < n; ++i) {
        auto s = get<thinkthen_span>(thinkthen_value_piece_at, v, i);
        std::map<std::string, double> tags;
        auto count = get<std::size_t>(thinkthen_value_piece_tag_count, v, i);
        for (std::size_t j = 0; j < count; ++j) {
            auto p = get<thinkthen_probability>(thinkthen_value_piece_tag_at, v, i, j);
            tags.emplace(copy(p.label), p.probability);
        }
        result.pieces.push_back({copy(s.text), s.start, s.end,
            {tags.at("BEGIN"), tags.at("INSIDE"), tags.at("END"), tags.at("SINGLE"), tags.at("OUT")}});
    }
    n = get<std::size_t>(thinkthen_value_kind_count, v);
    for (std::size_t i = 0; i < n; ++i) {
        auto s = get<thinkthen_span>(thinkthen_value_kind_span, v, i);
        kind_detail k{s.start, s.end, {}};
        auto count = get<std::size_t>(thinkthen_value_kind_probability_count, v, i);
        for (std::size_t j = 0; j < count; ++j) {
            auto p = get<thinkthen_probability>(thinkthen_value_kind_probability_at, v, i, j);
            k.kinds.emplace(copy(p.label), p.probability);
        }
        result.names.push_back(std::move(k));
    }
    return result;
}
template<class Q> typename result_traits<Q>::detail detail_value(const thinkthen_value* v) {
    if constexpr (std::is_same_v<Q, choose> || std::is_same_v<Q, tag>) return labels(v);
    else if constexpr (std::is_same_v<Q, score> || std::is_same_v<Q, rank>) return {
        collection<double>(v, thinkthen_value_level_count, thinkthen_value_level_at, [](double p) { return p; })};
    else if constexpr (std::is_same_v<Q, recognize>) return recognition(v);
    else if constexpr (std::is_same_v<Q, relate>) return {
        collection<thinkthen_name>(v, thinkthen_value_name_count, thinkthen_value_name_at,
            name_value),
        collection<thinkthen_relation>(v, thinkthen_value_pair_count, thinkthen_value_pair_at, relation_value)};
    else return {};
}
annotations annotation_values(const thinkthen_value* v);
}
template<class Q> class outcome {
    thinkthen::status status_;
    std::optional<typename result_traits<Q>::answer> answer_;
    std::optional<typename result_traits<Q>::detail> detail_;
    std::optional<double> probability_;
    std::optional<error> failure_;
public:
    explicit outcome(const thinkthen_value* value) {
        if (detail::get<std::uint32_t>(thinkthen_value_function, value) != static_cast<std::uint32_t>(Q::kind)) detail::incompatible();
        status_ = detail::status(detail::get<std::uint32_t>(thinkthen_value_status, value));
        const double p = detail::get<double>(thinkthen_value_probability, value);
        if (!std::isnan(p)) probability_ = p;
        if (status_ == status::answered) answer_.emplace(detail::answer<Q>(value));
        if (detail::get<std::uint8_t>(thinkthen_value_has_detail, value)) detail_.emplace(detail::detail_value<Q>(value));
        if (status_ == status::failed) {
            detail::diagnostic failure;
            detail::invoke(thinkthen_value_error, value, &failure.value);
            failure_.emplace(detail::failure(failure.value));
        }
    }
    thinkthen::status status() const noexcept { return status_; }
    const typename result_traits<Q>::answer* value() const noexcept { return answer_ ? &*answer_ : nullptr; }
    const std::optional<typename result_traits<Q>::detail>& detail() const noexcept { return detail_; }
    std::optional<double> probability() const noexcept { return probability_; }
    const error* failure() const noexcept { return failure_ ? &*failure_ : nullptr; }
    static constexpr thinkthen::function function() noexcept { return Q::kind; }
};
namespace detail {
inline annotations annotation_values(const thinkthen_value* v) {
    annotations result;
    auto n = get<std::size_t>(thinkthen_value_annotation_count, v);
    for (std::size_t i = 0; i < n; ++i) {
        thinkthen_text name{};
        value_owner child(get<thinkthen_value*>(thinkthen_value_annotation_at, v, i, &name), thinkthen_value_free);
        switch (get<std::uint32_t>(thinkthen_value_function, child.get())) {
#define THINKTHEN_ENTRY(F, CODE) case THINKTHEN_FUNCTION_##CODE: result.emplace(copy(name), outcome<F>{child.get()}); break;
            THINKTHEN_ENTRY(decide, DECIDE) THINKTHEN_ENTRY(choose, CHOOSE)
            THINKTHEN_ENTRY(tag, TAG) THINKTHEN_ENTRY(score, SCORE)
            THINKTHEN_ENTRY(filter, FILTER) THINKTHEN_ENTRY(find, FIND)
            THINKTHEN_ENTRY(recognize, RECOGNIZE) THINKTHEN_ENTRY(relate, RELATE)
#undef THINKTHEN_ENTRY
            default: incompatible();
        }
    }
    return result;
}
}
template<class Q> class row : public outcome<Q> {
    std::uint64_t position_;
    std::string id_;
    thinkthen::source source_;
    std::optional<owned_record> item_;
    template<class T> friend class stream;
    row(const thinkthen_row& value, const thinkthen_value* owner, std::optional<owned_record> item)
        : outcome<Q>(owner), position_(value.position), id_(detail::copy({value.id.data, value.id.len})), item_(std::move(item)) {
        auto s = detail::get<thinkthen_source>(thinkthen_value_source, owner);
        auto n = detail::get<thinkthen_text>(thinkthen_value_source_name, owner);
        source_ = {detail::copy(s.question_identity), detail::backend(s.backend), detail::copy(s.model),
            n.data ? std::optional<std::string>{detail::copy(n)} : std::nullopt};
        if constexpr (std::is_same_v<Q, filter>) {
            auto input = detail::get<thinkthen_record_view>(thinkthen_value_original, owner);
            const auto bytes = input.len ? std::string(reinterpret_cast<const char*>(input.data), input.len) : std::string{};
            switch (input.kind) {
                case THINKTHEN_RECORD_TEXT: item_ = owned_text{bytes}; break;
                case THINKTHEN_RECORD_JSON: item_ = owned_json_text{bytes}; break;
                case THINKTHEN_RECORD_MISSING: item_ = missing_record{}; break;
                case THINKTHEN_RECORD_INVALID: item_ = detail::record_encoding::invalid(); break;
                default: detail::incompatible();
            }
        }
    }
    explicit row(const thinkthen_row& value, std::optional<owned_record> item = {})
        : row(value, detail::value_owner(detail::get<thinkthen_value*>(thinkthen_row_values, &value), thinkthen_value_free), std::move(item)) {}
public:
    std::uint64_t position() const noexcept { return position_; }
    std::string_view id_json() const noexcept { return id_; }
    const thinkthen::source& source() const noexcept { return source_; }
    template<class T = Q, std::enable_if_t<std::is_same_v<T, filter>, int> = 0>
    const owned_record& item() const { return item_.value(); }
    template<class T = Q, std::enable_if_t<std::is_same_v<T, filter>, int> = 0>
    bool kept() const noexcept { return this->value() && *this->value(); }
private:
    row(const thinkthen_row& value, detail::value_owner owner, std::optional<owned_record> item)
        : row(value, owner.get(), std::move(item)) {}
};
}}
#endif
