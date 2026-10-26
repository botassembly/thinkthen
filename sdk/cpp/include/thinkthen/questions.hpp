#ifndef THINKTHEN_CPP_QUESTIONS_HPP
#define THINKTHEN_CPP_QUESTIONS_HPP
#include <thinkthen/errors.hpp>
#include <map>
#include <variant>
#include <vector>
namespace thinkthen { inline namespace v0_2 {
enum class function : std::uint32_t {
    decide = THINKTHEN_FUNCTION_DECIDE, choose = THINKTHEN_FUNCTION_CHOOSE,
    tag = THINKTHEN_FUNCTION_TAG, score = THINKTHEN_FUNCTION_SCORE,
    filter = THINKTHEN_FUNCTION_FILTER, rank = THINKTHEN_FUNCTION_RANK,
    find = THINKTHEN_FUNCTION_FIND, recognize = THINKTHEN_FUNCTION_RECOGNIZE,
    relate = THINKTHEN_FUNCTION_RELATE, annotate = THINKTHEN_FUNCTION_ANNOTATE
};
class threshold {
    double low_, high_;
    bool band_;
    threshold(double low, double high, bool band) : low_(low), high_(high), band_(band) {
        detail::question_owner q(detail::get<thinkthen_question*>(thinkthen_question_new,
            THINKTHEN_FUNCTION_DECIDE, detail::bytes("threshold"), std::size_t{9}), thinkthen_question_free);
        apply(q.get());
    }
public:
    static threshold cutoff(double value) { return {value, value, false}; }
    static threshold band(double low, double high) { return {low, high, true}; }
    void apply(thinkthen_question* q) const {
        if (band_) detail::invoke(thinkthen_question_set_band, q, low_, high_);
        else detail::invoke(thinkthen_question_set_cut, q, low_);
    }
};
struct rule {
    std::string label, source, target;
    std::optional<std::string> reads;
    bool either = false;
};
namespace detail {
template<class Q> class question {
    std::shared_ptr<const thinkthen_question> value_;
protected:
    explicit question(question_owner value) : value_(value.release(), thinkthen_question_free) {}
    question(std::uint32_t f, std::string_view text) : question(question_owner(
        get<thinkthen_question*>(thinkthen_question_new, f, bytes(text), text.size()), thinkthen_question_free)) {}
    template<class F> Q changed(F change) const {
        auto value = clone(value_.get());
        change(value.get());
        return Q(std::move(value));
    }
    void complete() { auto admitted = clone(value_.get()); value_ = {admitted.release(), thinkthen_question_free}; }
    template<class F> void add(const std::vector<std::string>& values, F setter) {
        for (const auto& value : values) invoke(setter, const_cast<thinkthen_question*>(value_.get()), bytes(value), value.size());
    }
    thinkthen_question* building() { return const_cast<thinkthen_question*>(value_.get()); }
public:
    const thinkthen_question* native_handle() const noexcept { return value_.get(); }
    Q evidence(std::string_view pointer) const {
        return changed([&](auto* q) { invoke(thinkthen_question_set_evidence, q, bytes(pointer), pointer.size()); });
    }
};
}
#define THINKTHEN_QUESTION_COMMON(Q, CODE) \
    friend class detail::question<Q>; \
    friend class toolbox; \
    explicit Q(detail::question_owner value) : detail::question<Q>(std::move(value)) {} \
public: \
    static constexpr function kind = function::CODE;
#define THINKTHEN_PREDICATE(Q) \
class Q : public detail::question<Q> { \
    THINKTHEN_QUESTION_COMMON(Q, Q) \
    explicit Q(std::string_view text) : detail::question<Q>(THINKTHEN_FUNCTION_##Q, text) { complete(); } \
    Q cutoff(thinkthen::threshold value) const { return changed([&](auto* q) { value.apply(q); }); } \
};
// Required-domain constructors admit their complete snapshots before publication.
class decide : public detail::question<decide> {
    THINKTHEN_QUESTION_COMMON(decide, decide)
    explicit decide(std::string_view text) : question(THINKTHEN_FUNCTION_DECIDE, text) { complete(); }
    decide cutoff(thinkthen::threshold value) const { return changed([&](auto* q) { value.apply(q); }); }
};
class filter : public detail::question<filter> {
    THINKTHEN_QUESTION_COMMON(filter, filter)
    explicit filter(std::string_view text) : question(THINKTHEN_FUNCTION_FILTER, text) { complete(); }
    filter cutoff(thinkthen::threshold value) const { return changed([&](auto* q) { value.apply(q); }); }
};
class choose : public detail::question<choose> {
    THINKTHEN_QUESTION_COMMON(choose, choose)
    choose(std::string_view text, const std::vector<std::string>& choices) : question(THINKTHEN_FUNCTION_CHOOSE, text) {
        add(choices, thinkthen_question_add_choice); complete();
    }
    choose cutoff(thinkthen::threshold value) const { return changed([&](auto* q) { value.apply(q); }); }
};
class tag : public detail::question<tag> {
    THINKTHEN_QUESTION_COMMON(tag, tag)
    tag(std::string_view text, const std::vector<std::string>& labels) : question(THINKTHEN_FUNCTION_TAG, text) {
        add(labels, thinkthen_question_add_label); complete();
    }
    tag cutoff(double value) const { return changed([&](auto* q) { detail::invoke(thinkthen_question_set_cut, q, value); }); }
};
class score : public detail::question<score> {
    THINKTHEN_QUESTION_COMMON(score, score)
    score(std::string_view text, const std::vector<std::string>& levels) : question(THINKTHEN_FUNCTION_SCORE, text) {
        add(levels, thinkthen_question_add_level); complete();
    }
};
class rank : public detail::question<rank> {
    THINKTHEN_QUESTION_COMMON(rank, rank)
    rank(std::string_view text, const std::vector<std::string>& levels) : question(THINKTHEN_FUNCTION_RANK, text) {
        add(levels, thinkthen_question_add_level); complete();
    }
    rank top(std::size_t value) const { return changed([&](auto* q) { detail::invoke(thinkthen_question_set_top, q, value); }); }
};
class find : public detail::question<find> {
    THINKTHEN_QUESTION_COMMON(find, find)
    explicit find(std::string_view text) : question(THINKTHEN_FUNCTION_FIND, text) { complete(); }
    find cutoff(double value) const { return changed([&](auto* q) { detail::invoke(thinkthen_question_set_cut, q, value); }); }
};
class recognize : public detail::question<recognize> {
    THINKTHEN_QUESTION_COMMON(recognize, recognize)
    explicit recognize(const std::vector<std::string>& kinds) : question(THINKTHEN_FUNCTION_RECOGNIZE, {}) {
        add(kinds, thinkthen_question_add_kind); complete();
    }
    recognize cutoff(double value) const { return changed([&](auto* q) { detail::invoke(thinkthen_question_set_cut, q, value); }); }
};
class relate : public detail::question<relate> {
    THINKTHEN_QUESTION_COMMON(relate, relate)
    explicit relate(const std::vector<rule>& rules) : question(THINKTHEN_FUNCTION_RELATE, {}) {
        for (const auto& value : rules) {
            thinkthen_rule input{detail::view(value.label), detail::view(value.source), detail::view(value.target),
                value.reads ? detail::view(*value.reads) : thinkthen_text{}, static_cast<std::uint8_t>(value.either)};
            detail::invoke(thinkthen_question_add_rule, building(), &input);
        }
        complete();
    }
    relate cutoff(double value) const { return changed([&](auto* q) { detail::invoke(thinkthen_question_set_cut, q, value); }); }
};
using annotation_question = std::variant<decide, choose, tag, score, filter, find, recognize, relate>;
using annotation_questions = std::map<std::string, annotation_question>;
using toolbox_question = std::variant<decide, choose, tag, score, filter, rank, find, recognize, relate>;
class annotate : public detail::question<annotate> {
    THINKTHEN_QUESTION_COMMON(annotate, annotate)
    explicit annotate(const annotation_questions& entries) : question(THINKTHEN_FUNCTION_ANNOTATE, {}) {
        for (const auto& entry : entries) std::visit([&](const auto& q) {
            detail::invoke(thinkthen_question_add_entry, building(), detail::bytes(entry.first), entry.first.size(), q.native_handle());
        }, entry.second);
        complete();
    }
};
#undef THINKTHEN_QUESTION_COMMON
#undef THINKTHEN_PREDICATE
class toolbox {
    std::shared_ptr<const thinkthen_toolbox> native_;
    explicit toolbox(thinkthen_toolbox* value) : native_(value, thinkthen_toolbox_free) {}
    template<std::size_t I = 0> static toolbox_question wrap(std::uint32_t kind, detail::question_owner value) {
        if constexpr (I < std::variant_size_v<toolbox_question>) {
            using Q = std::variant_alternative_t<I, toolbox_question>;
            if (kind == static_cast<std::uint32_t>(Q::kind)) return Q(std::move(value));
            return wrap<I + 1>(kind, std::move(value));
        } else detail::incompatible();
    }
    static std::string buffer(thinkthen_buffer value) {
        std::unique_ptr<thinkthen_buffer, decltype(&thinkthen_buffer_free)> owner(&value, thinkthen_buffer_free);
        return detail::copy({value.data, value.len});
    }
public:
    explicit toolbox(const std::map<std::string, toolbox_question>& entries) {
        detail::owner<thinkthen_toolbox, thinkthen_toolbox_free> native(
            detail::get<thinkthen_toolbox*>(thinkthen_toolbox_new), thinkthen_toolbox_free);
        for (const auto& entry : entries) std::visit([&](const auto& q) {
            detail::invoke(thinkthen_toolbox_insert, native.get(), detail::bytes(entry.first), entry.first.size(), q.native_handle());
        }, entry.second);
        native_ = {native.release(), thinkthen_toolbox_free};
    }
    static toolbox from_json(std::string_view json) {
        return toolbox(detail::get<thinkthen_toolbox*>(thinkthen_toolbox_from_json, detail::bytes(json), json.size()));
    }
    std::string to_json() const {
        return buffer(detail::get<thinkthen_buffer>(thinkthen_toolbox_to_json, native_.get()));
    }
    toolbox_question select(std::string_view name) const {
        detail::question_owner selected(detail::get<thinkthen_question*>(thinkthen_toolbox_select,
            native_.get(), detail::bytes(name), name.size()), thinkthen_question_free);
        const auto kind = detail::get<std::uint32_t>(thinkthen_question_function, selected.get());
        return wrap(kind, std::move(selected));
    }
    template<class Q> Q select(std::string_view name) const {
        detail::question_owner selected(detail::get<thinkthen_question*>(thinkthen_toolbox_select_as,
            native_.get(), detail::bytes(name), name.size(), static_cast<std::uint32_t>(Q::kind)), thinkthen_question_free);
        return Q(std::move(selected));
    }
};
}}
#endif
