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
#include <format>
#include <typeindex>
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
    /// The selected kind from the question.
    std::string kind;
    /// Matched name text.
    std::string text;
    /// Inclusive UTF-8 byte offset.
    std::uint64_t start;
    /// Exclusive UTF-8 byte offset.
    std::uint64_t end;
    /// Probability of that kind.
    double probability;
};
/// One side of a relation, with UTF-8 byte offsets.
struct endpoint {
    /// The recognized endpoint kind.
    std::string kind;
    /// Matched endpoint text.
    std::string text;
    /// Inclusive UTF-8 byte offset.
    std::uint64_t start;
    /// Exclusive UTF-8 byte offset.
    std::uint64_t end;
};
/// A named relation between two endpoints and its probability.
struct relation {
    /// The relation's source name.
    endpoint source;
    /// The relation's target name.
    endpoint target;
    /// The rule label selected by the engine.
    std::string relation;
    /// Probability of this relation.
    double probability;
    /// Whether the rule allows either direction; absent when not supplied.
    bool either;
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
/// Borrowed text; one-record calls copy it before return, streams when the record is pulled.
struct text { /** Borrowed UTF-8 text. */ std::string_view value; };
/// Borrowed JSON bytes; keep them valid until a one-record call returns or a stream pulls them.
struct json_text { /** Borrowed UTF-8 JSON bytes. */ std::string_view value; };
/// An explicit missing record; the engine returns record.missing.
struct missing_record {};
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
using record = std::variant<text, json_text, missing_record, owned_text, owned_json_text>;
/// An existing owned-record spelling for explicit encodings.
using owned_record = record;
/// Convert ordinary UTF-8 text without keeping another copy at the host boundary.
[[nodiscard]] inline record to_record(std::string_view value) { return text{value}; }
/// Pass an explicit text, JSON or missing encoding through unchanged.
[[nodiscard]] inline record to_record(const record& value) { return value; }
[[nodiscard]] inline record to_record(text value) { return value; }
[[nodiscard]] inline record to_record(json_text value) { return value; }
[[nodiscard]] inline record to_record(missing_record value) { return value; }
[[nodiscard]] inline record to_record(const owned_text& value) { return value; }
[[nodiscard]] inline record to_record(const owned_json_text& value) { return value; }
/// A pull source; return nullopt to end input.
using record_source = std::function<std::optional<owned_record>()>;
/// A move-only result, optionally retaining the caller's record type.
template<class Q, class T = void> class row;
namespace detail { struct row_metadata; }
/// Owned form entry views, reached through the form's loud answer read.
class annotations {
    struct implementation;
    std::shared_ptr<implementation> values_;
public:
    /// Retain the native form and the parent's immutable metadata.
    annotations(std::shared_ptr<detail::native_row> native, detail::row_metadata metadata);
    /// Pick a named entry as its function; missing names and wrong functions are native errors.
    template<class Q> requires (Q::kind != function::filter && Q::kind != function::rank && Q::kind != function::annotate)
    [[nodiscard]] const row<Q>& get(std::string_view name) const;
};
/// Associates each question with its plain answer and detail types.
template<class Q> struct result_traits;
#define THINKTHEN_RESULT(Q, A, D) template<> struct result_traits<Q> { using answer = A; using detail = D; };
THINKTHEN_RESULT(decide, bool, std::monostate)
THINKTHEN_RESULT(filter, bool, std::monostate)
THINKTHEN_RESULT(find, std::optional<span>, std::monostate)
THINKTHEN_RESULT(recognize, std::vector<name>, recognition_detail)
THINKTHEN_RESULT(relate, std::vector<relation>, relation_detail)
THINKTHEN_RESULT(annotate, annotations, std::monostate)
#undef THINKTHEN_RESULT
template<class E> struct result_traits<choose<E>> { using answer = E; using detail = label_detail; };
template<class E> struct result_traits<tag<E>> { using answer = std::set<E>; using detail = label_detail; };
template<class E> struct result_traits<score<E>> { using answer = double; using detail = level_detail; };
template<class E> struct result_traits<rank<E>> { using answer = std::uint64_t; using detail = level_detail; };
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
inline name name_value(const thinkthen_name& n) { return {copy(n.kind), copy(n.text), n.start, n.end, n.probability}; }
inline endpoint endpoint_value(const thinkthen_endpoint& v) { return {copy(v.kind), copy(v.text), v.start, v.end}; }
inline relation relation_value(const thinkthen_relation& v) {
    if (v.either > 1) incompatible();
    return {endpoint_value(v.source), endpoint_value(v.target), copy(v.relation), v.probability,
        v.either != 0};
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
template<class Q> typename result_traits<Q>::answer answer(const thinkthen_value* v) {
    if constexpr (Q::kind == function::decide || Q::kind == function::filter) {
        return get<std::uint8_t>(thinkthen_value_bool, v) != 0;
    } else if constexpr (Q::kind == function::choose)
        return member<typename Q::member_type>(copy(get<thinkthen_text>(thinkthen_value_choice, v)));
    else if constexpr (Q::kind == function::score) return get<double>(thinkthen_value_number, v);
    else if constexpr (Q::kind == function::rank) return get<std::uint64_t>(thinkthen_value_rank, v);
    else if constexpr (Q::kind == function::tag) {
        typename result_traits<Q>::answer values;
        const auto count = get<std::size_t>(thinkthen_value_tag_count, v);
        for (std::size_t i = 0; i < count; ++i)
            values.insert(member<typename Q::member_type>(copy(get<thinkthen_text>(thinkthen_value_tag_at, v, i))));
        return values;
    } else if constexpr (Q::kind == function::find) {
        auto value = get<thinkthen_span>(thinkthen_value_span, v);
        return value.present ? std::optional<span>{span_value(value)} : std::nullopt;
    } else if constexpr (Q::kind == function::recognize)
        return collection<thinkthen_name>(v, thinkthen_value_name_count, thinkthen_value_name_at, name_value);
    else return collection<thinkthen_relation>(v, thinkthen_value_relation_count, thinkthen_value_relation_at, relation_value);
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
    if constexpr (Q::kind == function::choose || Q::kind == function::tag) return labels(v);
    else if constexpr (Q::kind == function::score || Q::kind == function::rank) return {
        collection<double>(v, thinkthen_value_level_count, thinkthen_value_level_at, [](double p) { return p; })};
    else if constexpr (Q::kind == function::recognize) return recognition(v);
    else if constexpr (Q::kind == function::relate) return {
        collection<thinkthen_name>(v, thinkthen_value_name_count, thinkthen_value_name_at,
            name_value),
        collection<thinkthen_relation>(v, thinkthen_value_pair_count, thinkthen_value_pair_at, relation_value)};
    else return {};
}
}
namespace detail {
using record_id = std::variant<std::monostate, std::string, std::int64_t, std::uint64_t>;
struct row_metadata {
    std::uint64_t position;
    record_id id;
    thinkthen::source origin;
    std::string id_json;
};
inline row_metadata metadata(const thinkthen_row& native, const thinkthen_value* value) {
    record_id id;
    const auto encoded = get<thinkthen_id>(thinkthen_value_id, value);
    switch (encoded.kind) {
    case THINKTHEN_ID_ABSENT: break;
    case THINKTHEN_ID_STRING: id = copy(encoded.text); break;
    case THINKTHEN_ID_SIGNED: id = encoded.signed_value; break;
    case THINKTHEN_ID_UNSIGNED: id = encoded.unsigned_value; break;
    default: incompatible();
    }
    const auto origin = get<thinkthen_source>(thinkthen_value_source, value);
    const auto name = get<thinkthen_text>(thinkthen_value_source_name, value);
    std::string encoded_id;
    std::visit([&](const auto& value) {
        using T = std::decay_t<decltype(value)>;
        if constexpr(std::same_as<T,std::string>) {
            encoded_id = "\"";
            for(unsigned char c:value) {
                if(c=='"' || c=='\\'){encoded_id+='\\';encoded_id+=static_cast<char>(c);}
                else if(c<32){encoded_id+="\\u00";encoded_id+="0123456789abcdef"[c>>4];encoded_id+="0123456789abcdef"[c&15];}
                else encoded_id+=static_cast<char>(c);
            }
            encoded_id+='"';
        } else if constexpr(!std::same_as<T,std::monostate>) encoded_id=std::to_string(value);
    },id);
    return {native.position, std::move(id), {copy(origin.question_identity), backend(origin.backend),
        copy(origin.model), name.data ? std::optional<std::string>{copy(name)} : std::nullopt}, std::move(encoded_id)};
}
template<class Q> void expect_function(const thinkthen_row* value) {
    if (value->function == static_cast<thinkthen_function>(Q::kind)) return;
    // Let the native reader supply its existing usage.function diagnostic.
    if constexpr (Q::kind == function::decide) (void)get<bool>(thinkthen_row_answer_bool, value);
    else if constexpr (Q::kind == function::choose) (void)get<std::int32_t>(thinkthen_row_answer_choice, value);
    else if constexpr (Q::kind == function::score) (void)get<double>(thinkthen_row_answer_score, value);
    else if constexpr (Q::kind == function::rank) (void)get<std::uint64_t>(thinkthen_row_answer_rank, value);
    else if constexpr (Q::kind == function::find) (void)get<thinkthen_span>(thinkthen_row_answer_span, value);
    else if constexpr (Q::kind == function::tag) invoke(thinkthen_row_answer_tags, value, static_cast<bool*>(nullptr), std::size_t{});
    else if constexpr (Q::kind == function::recognize) {
        const thinkthen_name* names{}; std::size_t count{};
        invoke(thinkthen_row_answer_names, value, &names, &count);
    } else if constexpr (Q::kind == function::relate) {
        const thinkthen_relation* values{}; std::size_t count{};
        invoke(thinkthen_row_answer_relations, value, &values, &count);
    } else {
        native_row child;
        invoke(thinkthen_row_answer_entry, value, "", std::size_t{}, &child.value);
    }
    incompatible();
}
template<class Q> struct row_storage {
    std::shared_ptr<native_row> native;
    row_metadata meta;
    std::optional<typename result_traits<Q>::answer> answer;
    std::optional<typename result_traits<Q>::detail> detail;
    std::optional<failed> failure;
    row_storage(native_row value, std::optional<row_metadata> parent)
        : native(std::make_shared<native_row>(std::move(value))) {
        expect_function<Q>(&native->value);
        value_owner snapshot(get<thinkthen_value*>(thinkthen_row_values, &native->value), thinkthen_value_free);
        meta = parent ? std::move(*parent) : metadata(native->value, snapshot.get());
        if (native->value.status == THINKTHEN_STATUS_ANSWERED) {
            if constexpr (Q::kind == function::annotate)
                answer.emplace(native, meta);
            else answer.emplace(thinkthen::detail::answer<Q>(snapshot.get()));
        }
        if (get<std::uint8_t>(thinkthen_value_has_detail, snapshot.get()))
            detail.emplace(detail_value<Q>(snapshot.get()));
        if (native->value.status == THINKTHEN_STATUS_FAILED) {
            diagnostic result;
            const auto rc = thinkthen_row_err_if_failed(&native->value, &result.value);
            if (rc >= 0) incompatible();
            failure.emplace(thinkthen::detail::failure(result.value));
        }
    }
};
}
/// An immutable, move-only result owning one native row.
template<class Q, class T> class [[nodiscard]] row {
    std::unique_ptr<detail::row_storage<Q>> value_;
    const T* item_ = nullptr;
    std::shared_ptr<const void> item_owner_;
    template<class, class> friend class stream;
    friend class client;
    friend class annotations;
    row(detail::native_row value, std::optional<detail::row_metadata> parent = {})
        : value_(std::make_unique<detail::row_storage<Q>>(std::move(value), std::move(parent))) {}
    row(detail::native_row value, const T* item, std::shared_ptr<const void> owner)
        : row(std::move(value)) { item_ = item; item_owner_ = std::move(owner); }
public:
    row(const row&) = delete;
    row& operator=(const row&) = delete;
    row(row&&) noexcept = default;
    row& operator=(row&&) noexcept = default;
    /// The row's three-state status, never an answer read.
    [[nodiscard]] thinkthen::status status() const noexcept { return static_cast<thinkthen::status>(value_->native->value.status); }
    [[nodiscard]] bool answered() const noexcept { return status() == thinkthen::status::answered; }
    [[nodiscard]] bool unsure() const noexcept { return status() == thinkthen::status::unsure; }
    [[nodiscard]] bool failed() const noexcept { return status() == thinkthen::status::failed; }
    /// Read the plain answer, loudly refusing both unavailable states.
    [[nodiscard]] decltype(auto) answer() const {
        if (const auto* failure = error()) throw *failure;
        if (unsure()) throw thinkthen::unsure(probability());
        using A = typename result_traits<Q>::answer;
        if constexpr (std::is_arithmetic_v<A> || std::is_enum_v<A>) return A(*value_->answer);
        else return static_cast<const A&>(*value_->answer);
    }
    /// Only decide results may read their answer through a condition.
    explicit operator bool() const requires (Q::kind == function::decide) { return answer(); }
    [[nodiscard]] std::optional<double> probability() const noexcept {
        const auto probability = value_->native->value.probability;
        return std::isnan(probability) ? std::nullopt : std::optional<double>{probability};
    }
    [[nodiscard]] const std::optional<typename result_traits<Q>::detail>& detail() const noexcept { return value_->detail; }
    [[nodiscard]] const thinkthen::failed* error() const noexcept { return value_->failure ? &*value_->failure : nullptr; }
    /// Refuse parent or child failure without treating uncertainty as failure.
    row& throw_if_failed() & {
        detail::diagnostic failure;
        const auto result = thinkthen_row_err_if_failed(native_handle(), &failure.value);
        if (result < 0) throw thinkthen::failed(detail::failure(failure.value));
        return *this;
    }
    row&& throw_if_failed() && { throw_if_failed(); return std::move(*this); }
    [[nodiscard]] std::uint64_t position() const noexcept { return value_->meta.position; }
    [[nodiscard]] const detail::record_id& id() const noexcept { return value_->meta.id; }
    /// Existing encoded-ID accessor, preserving string and integer identity.
    [[nodiscard]] std::string_view id_json() const noexcept { return value_->meta.id_json; }
    [[nodiscard]] const thinkthen::source& source() const noexcept { return value_->meta.origin; }
    template<class U = T> requires (!std::is_void_v<U>)
    [[nodiscard]] const U& item() const noexcept { return *item_; }
    [[nodiscard]] static constexpr thinkthen::function function() noexcept { return Q::kind; }
    /// Save the native canonical row; string indexes are always character positions here.
    [[nodiscard]] std::string to_json() const {
        auto value = detail::get<thinkthen_buffer>(thinkthen_row_to_json, native_handle());
        std::unique_ptr<thinkthen_buffer, decltype(&thinkthen_buffer_free)> owner(&value, thinkthen_buffer_free);
        return detail::copy({reinterpret_cast<const char*>(value.data), value.len});
    }
    [[nodiscard]] const thinkthen_row* native_handle() const noexcept { return &value_->native->value; }
};
struct annotations::implementation {
    struct erased { virtual ~erased() = default; };
    template<class Q> struct entry final : erased { row<Q> value; explicit entry(row<Q> item) : value(std::move(item)) {} };
    std::shared_ptr<detail::native_row> parent;
    detail::row_metadata metadata;
    mutable std::map<std::pair<std::string, std::type_index>, std::unique_ptr<erased>> cache;
};
inline annotations::annotations(std::shared_ptr<detail::native_row> native, detail::row_metadata metadata)
    : values_(std::make_shared<implementation>()) {
    values_->parent = std::move(native); values_->metadata = std::move(metadata);
}
template<class Q> requires (Q::kind != function::filter && Q::kind != function::rank && Q::kind != function::annotate)
const row<Q>& annotations::get(std::string_view name) const {
    auto key = std::make_pair(std::string(name), std::type_index(typeid(Q)));
    auto found = values_->cache.find(key);
    if (found == values_->cache.end()) {
        detail::native_row child;
        detail::invoke(thinkthen_row_answer_entry, &values_->parent->value, name.data(), name.size(), &child.value);
        auto entry = std::make_unique<implementation::entry<Q>>(row<Q>(std::move(child), values_->metadata));
        found = values_->cache.emplace(std::move(key), std::move(entry)).first;
    }
    return static_cast<const implementation::entry<Q>&>(*found->second).value;
}
/// Read one named form entry with the question's answer type.
template<class Q> requires (Q::kind != function::filter && Q::kind != function::rank && Q::kind != function::annotate)
[[nodiscard]] const row<Q>& get(const annotations& values, std::string_view name) { return values.template get<Q>(name); }
/// Failed filter inputs retain their native rows and caller record lifetimes.
template<class T> class filter_error : public failed {
    std::shared_ptr<const std::vector<row<filter, T>>> rows_;
public:
    filter_error(const failed& failure, std::vector<row<filter, T>> rows)
        : failed(failure), rows_(std::make_shared<const std::vector<row<filter, T>>>(std::move(rows))) {}
    [[nodiscard]] const std::vector<row<filter, T>>& rows() const noexcept { return *rows_; }
};
/// Print the row status using the shared spelling.
inline std::ostream& operator<<(std::ostream& out, status value) {
    switch (value) { case status::answered: return out << "answered"; case status::unsure: return out << "unsure"; case status::failed: return out << "failed"; }
    return out;
}
/// Printing reads only the native safe text projection, including in failure states.
template<class Q, class T> std::ostream& operator<<(std::ostream& out, const row<Q, T>& value) {
    return out << thinkthen_row_string(value.native_handle());
}
}}
namespace std {
/// Standard formatting shares the native safe text projection with stream insertion.
template<class Q, class T> struct formatter<thinkthen::row<Q, T>> : formatter<string_view> {
    template<class Context> auto format(const thinkthen::row<Q, T>& value, Context& context) const {
        return formatter<string_view>::format(thinkthen_row_string(value.native_handle()), context);
    }
};
}
#endif
