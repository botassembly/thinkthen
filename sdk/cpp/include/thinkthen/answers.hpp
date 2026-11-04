#ifndef THINKTHEN_CPP_ANSWERS_HPP
#define THINKTHEN_CPP_ANSWERS_HPP
#include <thinkthen/questions.hpp>
#include <thinkthen/settings.hpp>
#include <map>
#include <functional>
#include <cmath>
#include <variant>
#include <ostream>
#include <algorithm>
namespace thinkthen { inline namespace v0_2 {
/// A row is answered, not sure, or failed.
enum class status : std::uint32_t {
    /// answered: The row has an answer.
    answered = THINKTHEN_STATUS_ANSWERED,
    /// unsure: The model was not sure, so the row has no answer.
    unsure = THINKTHEN_STATUS_UNSURE,
    /// failed: The record failed; the row's error says why.
    failed = THINKTHEN_STATUS_FAILED,
};
/// A matched span with UTF-8 byte offsets and the original text.
struct span {
    /// Matched text, copied from the original record.
    std::string text;
    /// Inclusive UTF-8 byte offset.
    std::uint64_t start;
    /// Exclusive UTF-8 byte offset.
    std::uint64_t end;
};
/// A recognized name, its kind and probability, with UTF-8 byte offsets.
struct name {
    /// Matched name text.
    std::string text;
    /// Inclusive UTF-8 byte offset.
    std::uint64_t start;
    /// Exclusive UTF-8 byte offset.
    std::uint64_t end;
    /// The selected kind from the question.
    std::string kind;
    /// Probability of that kind.
    double probability;
};
/// One side of a relation, with UTF-8 byte offsets.
struct endpoint {
    /// Matched endpoint text.
    std::string text;
    /// Inclusive UTF-8 byte offset.
    std::uint64_t start;
    /// Exclusive UTF-8 byte offset.
    std::uint64_t end;
    /// The recognized endpoint kind.
    std::string kind;
};
/// A named relation between two endpoints and its probability.
struct relation {
    /// The relation's source name.
    endpoint source;
    /// The relation's target name.
    endpoint target;
    /// The rule label selected by the engine.
    std::string label;
    /// Probability of this relation.
    double probability;
    /// Whether the rule allows either direction; absent when not supplied.
    std::optional<bool> either;
};
/// Probability by label name for choose and tag.
struct label_detail { /** Probabilities keyed by label. */ std::map<std::string, double> labels; };
/// Probabilities in the question’s original level order.
struct level_detail { /** Probabilities in the input level order. */ std::vector<double> levels; };
/// Probabilities for the five span-boundary tags.
struct boundary_probabilities {
    /** First piece of a multi-piece span. */ double begin;
    /** Interior piece of a span. */ double inside;
    /** Last piece of a multi-piece span. */ double end;
    /** One-piece span. */ double single;
    /** Piece outside any span. */ double out;
};
/// Boundary probabilities for one UTF-8 text piece.
struct piece_detail {
    /** Original text piece. */ std::string text;
    /** Inclusive UTF-8 byte offset. */ std::uint64_t start;
    /** Exclusive UTF-8 byte offset. */ std::uint64_t end;
    /** Boundary probabilities. */ boundary_probabilities tags;
};
/// Kind probabilities for one recognized span.
struct kind_detail {
    /** Original matched text. */ std::string text;
    /** Inclusive UTF-8 byte offset. */ std::uint64_t start;
    /** Exclusive UTF-8 byte offset. */ std::uint64_t end;
    /** Probability by kind name. */ std::map<std::string, double> kinds;
};
/// Recognition field for text pieces and names.
struct recognition_detail {
    /** Text pieces with boundary probabilities. */ std::vector<piece_detail> pieces;
    /** Candidate names with kind probabilities. */ std::vector<kind_detail> names;
};
/// Recognized names and candidate relation pairs.
struct relation_detail {
    /** Recognized names considered by the relation question. */ std::vector<name> names;
    /** Candidate name pairs and their relation probabilities. */ std::vector<relation> pairs;
};
/// The question identity and backend that supplied this answer.
struct source {
    /** Engine identity for the admitted question. */ std::string question_identity;
    /** Backend that served the answer. */ thinkthen::backend backend;
    /** Model that served the answer. */ std::string model;
    /** Optional execution name supplied by the caller. */ std::optional<std::string> name;
};
/// A borrowed plain-text record, copied before a call returns.
struct text { /** Borrowed UTF-8 text. */ std::string_view value; };
/// A borrowed JSON-encoded object record, copied before a call returns.
struct json_text { /** Borrowed UTF-8 JSON bytes. */ std::string_view value; };
/// An explicit missing record; the engine returns record.missing.
struct missing_record {};
/// A borrowed text, JSON object or explicit missing input.
using record = std::variant<text, json_text, missing_record>;
/// An owned plain-text input retained by a stream.
struct owned_text { /** Owned UTF-8 text. */ std::string value; };
namespace detail { struct record_encoding; }
/// An owned JSON-encoded object input retained by a stream.
struct owned_json_text {
    /// Owned UTF-8 JSON bytes.
    std::string value;
    /// Own a JSON record; the engine admits it when a call reads it.
    explicit owned_json_text(std::string bytes = {}) : value(std::move(bytes)) {}
private:
    bool invalid_ = false;
    friend struct detail::record_encoding;
};
namespace detail {
struct record_encoding {
    [[nodiscard]] static owned_json_text invalid() { owned_json_text value; value.invalid_ = true; return value; }
    [[nodiscard]] static bool invalid(const owned_json_text& value) { return value.invalid_; }
};
}
/// An owned input suitable for a deferred stream.
using owned_record = std::variant<owned_text, owned_json_text, missing_record>;
/// A pull source; return nullopt to end input.
using record_source = std::function<std::optional<owned_record>()>;
/// Associates each question with its native C++ answer and detail types.
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
template<class Q> class [[nodiscard]] outcome;
/// One typed child outcome from an annotation.
using annotation = std::variant<outcome<decide>, outcome<choose>, outcome<tag>, outcome<score>, outcome<filter>, outcome<find>, outcome<recognize>, outcome<relate>>;
/// Child outcomes keyed by annotation entry name.
using annotations = std::map<std::string, annotation>;
THINKTHEN_RESULT(annotate, annotations, std::monostate)
#undef THINKTHEN_RESULT
namespace detail {
using value_owner = owner<thinkthen_value, thinkthen_value_free>;
[[nodiscard]] inline thinkthen::status status(std::uint32_t value) {
    switch (value) {
        case THINKTHEN_STATUS_ANSWERED: return thinkthen::status::answered;
        case THINKTHEN_STATUS_UNSURE: return thinkthen::status::unsure;
        case THINKTHEN_STATUS_FAILED: return thinkthen::status::failed;
        default: incompatible();
    }
}
[[nodiscard]] inline thinkthen::backend backend(std::uint32_t value) {
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
        kind_detail k{copy(s.text), s.start, s.end, {}};
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
}
/// An immutable typed answer and its status, probability, detail and failure.
template<class Q> class [[nodiscard]] outcome {
    thinkthen::status status_;
    std::optional<typename result_traits<Q>::answer> answer_;
    std::optional<typename result_traits<Q>::detail> detail_;
    std::optional<double> probability_;
    std::optional<error> failure_;
public:
    /// Copy a native value into owned immutable C++ data.
    explicit outcome(const thinkthen_value* value) {
        if (detail::get<thinkthen_function>(thinkthen_value_function, value) != static_cast<std::uint32_t>(Q::kind)) detail::incompatible();
        status_ = detail::status(detail::get<thinkthen_status>(thinkthen_value_status, value));
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
    /// Read the row status without interpreting its answer.
    [[nodiscard]] thinkthen::status status() const noexcept { return status_; }
    /// Read the answer, throwing unsure or the engine failure when it has no answer.
    [[nodiscard]] const typename result_traits<Q>::answer& value() const {
        if (failure_) throw *failure_;
        if (!answer_) throw thinkthen::unsure(probability_);
        return *answer_;
    }
    /// Inspect the answer without throwing; null means not sure or failed.
    [[nodiscard]] const typename result_traits<Q>::answer* answer() const noexcept { return answer_ ? &*answer_ : nullptr; }
    /// Test for a not-sure outcome without throwing.
    [[nodiscard]] bool unsure() const noexcept { return status_ == thinkthen::status::unsure; }
    /// Test for a failed outcome without throwing.
    [[nodiscard]] bool failed() const noexcept { return status_ == thinkthen::status::failed; }
    template<class T = Q, std::enable_if_t<std::is_same_v<T, decide> || std::is_same_v<T, filter>, int> = 0>
    /// Read a decide or filter answer, throwing when not sure or failed.
    explicit operator bool() const { return value(); }
    template<class T = Q, std::enable_if_t<std::is_same_v<T, choose> || std::is_same_v<T, score> || std::is_same_v<T, rank>, int> = 0>
    /// Compare an answered choose, score or rank value; other states compare false.
    [[nodiscard]] bool operator==(const typename result_traits<T>::answer& other) const noexcept { return answer_ && *answer_ == other; }
    template<class T = Q, std::enable_if_t<std::is_same_v<T, tag>, int> = 0>
    /// Test whether an answered tag includes a label; otherwise throw.
    [[nodiscard]] bool contains(std::string_view label) const { const auto& labels = value(); return std::find(labels.begin(), labels.end(), label) != labels.end(); }
    /// Inspect the function-specific detail supplied by the engine.
    [[nodiscard]] const std::optional<typename result_traits<Q>::detail>& detail() const noexcept { return detail_; }
    /// Read the answer probability, when this function has one.
    [[nodiscard]] std::optional<double> probability() const noexcept { return probability_; }
    /// Inspect the engine failure; null means this row did not fail.
    [[nodiscard]] const error* failure() const noexcept { return failure_ ? &*failure_ : nullptr; }
    /// Read the function that produced this outcome.
    [[nodiscard]] static constexpr thinkthen::function function() noexcept { return Q::kind; }
};
namespace detail {
inline annotations annotation_values(const thinkthen_value* v) {
    annotations result;
    auto n = get<std::size_t>(thinkthen_value_annotation_count, v);
    for (std::size_t i = 0; i < n; ++i) {
        thinkthen_text name{};
        value_owner child(get<thinkthen_value*>(thinkthen_value_annotation_at, v, i, &name), thinkthen_value_free);
        switch (get<thinkthen_function>(thinkthen_value_function, child.get())) {
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
/// An owned outcome with its original input position, id and source.
template<class Q> class [[nodiscard]] row : public outcome<Q> {
    std::uint64_t position_;
    std::string id_;
    std::variant<std::monostate, std::string, std::int64_t, std::uint64_t> typed_id_;
    thinkthen::source source_;
    std::optional<owned_record> item_;
    template<class T> friend class stream;
    row(const thinkthen_row& value, const thinkthen_value* owner, std::optional<owned_record> item)
        : outcome<Q>(owner), position_(value.position), id_(detail::copy({reinterpret_cast<const char *>(value.id.data), value.id.len})), item_(std::move(item)) {
        const auto id = detail::get<thinkthen_id>(thinkthen_value_id, owner);
        switch (id.kind) {
            case THINKTHEN_ID_ABSENT: break;
            case THINKTHEN_ID_STRING: typed_id_ = detail::copy(id.text); break;
            case THINKTHEN_ID_SIGNED: typed_id_ = id.signed_value; break;
            case THINKTHEN_ID_UNSIGNED: typed_id_ = id.unsigned_value; break;
            default: detail::incompatible();
        }
        auto s = detail::get<thinkthen_source>(thinkthen_value_source, owner);
        auto n = detail::get<thinkthen_text>(thinkthen_value_source_name, owner);
        source_ = {detail::copy(s.question_identity), detail::backend(s.backend), detail::copy(s.model),
            n.data ? std::optional<std::string>{detail::copy(n)} : std::nullopt};
        if constexpr (std::is_same_v<Q, filter> || std::is_same_v<Q, rank>) {
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
    /// Read the one-based position in the original input.
    [[nodiscard]] std::uint64_t position() const noexcept { return position_; }
    /// Read the original id as JSON, preserving string and integer identity.
    [[nodiscard]] std::string_view id_json() const noexcept { return id_; }
    /// Read the original id without parsing JSON or narrowing an integer.
    [[nodiscard]] const std::variant<std::monostate, std::string, std::int64_t, std::uint64_t>& id() const noexcept { return typed_id_; }
    /// Read the question and backend metadata.
    [[nodiscard]] const thinkthen::source& source() const noexcept { return source_; }
    template<class T = Q, std::enable_if_t<std::is_same_v<T, filter> || std::is_same_v<T, rank>, int> = 0>
    /// Read the original filter or rank input, preserving its encoding.
    [[nodiscard]] const owned_record& item() const { return item_.value(); }
    template<class T = Q, std::enable_if_t<std::is_same_v<T, filter>, int> = 0>
    /// Test whether a filter row is an answered yes; inspect failures separately.
    [[nodiscard]] bool kept() const noexcept { return this->answer() && *this->answer(); }
private:
    row(const thinkthen_row& value, detail::value_owner owner, std::optional<owned_record> item)
        : row(value, owner.get(), std::move(item)) {}
};
/// Print a row status using its contract spelling.
inline std::ostream& operator<<(std::ostream& out, status value) {
    switch (value) { case status::answered: return out << "answered"; case status::unsure: return out << "unsure"; case status::failed: return out << "failed"; default: return out << "unknown"; }
}
namespace detail {
inline void print_answer(std::ostream& out, bool value) { out << (value ? "true" : "false"); }
inline void print_answer(std::ostream& out, const span& value) { out << value.text; }
inline void print_answer(std::ostream& out, const name& value) { out << value.text; }
inline void print_answer(std::ostream& out, const relation& value) { out << value.source.text << ' ' << value.label << ' ' << value.target.text; }
template<class T> void print_answer(std::ostream& out, const T& value) { out << value; }
template<class T> void print_answer(std::ostream& out, const std::optional<T>& value) { if (value) print_answer(out, *value); else out << "none"; }
template<class T> void print_answer(std::ostream& out, const std::vector<T>& values) { out << '['; bool first = true; for (const auto& value : values) { if (!first) out << ", "; first = false; print_answer(out, value); } out << ']'; }
void print_answer(std::ostream& out, const annotations& values);
}
/// Print the answer, or an explicit not-sure or failure status.
template<class Q> std::ostream& operator<<(std::ostream& out, const outcome<Q>& value) {
    if (value.failed()) return out << "failed: " << value.failure()->code().message();
    if (value.unsure()) return out << "unsure";
    detail::print_answer(out, value.value()); return out;
}
namespace detail {
inline void print_answer(std::ostream& out, const annotations& values) { out << '{'; bool first = true; for (const auto& entry : values) { if (!first) out << ", "; first = false; out << entry.first << ": "; std::visit([&](const auto& value) { out << value; }, entry.second); } out << '}'; }
}
/// Read an annotation child as the question type it was built with.
template<class Q> [[nodiscard]] const outcome<Q>& get(const annotations& values, const std::string& name) { return std::get<outcome<Q>>(values.at(name)); }
}}
#endif
