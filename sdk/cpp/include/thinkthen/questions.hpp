#ifndef THINKTHEN_CPP_QUESTIONS_HPP
#define THINKTHEN_CPP_QUESTIONS_HPP
#include <thinkthen/errors.hpp>
#include <map>
#include <variant>
#include <vector>
namespace thinkthen { inline namespace v0_2 {
/// The ten judgment functions.
enum class function : std::uint32_t {
    /// decide: Answer yes or no.
    decide = THINKTHEN_FUNCTION_DECIDE,
    /// choose: Pick one of the choices.
    choose = THINKTHEN_FUNCTION_CHOOSE,
    /// tag: Pick every label that applies.
    tag = THINKTHEN_FUNCTION_TAG,
    /// score: Place the text on ordered levels and give a number.
    score = THINKTHEN_FUNCTION_SCORE,
    /// filter: Keep the records the answer is yes for.
    filter = THINKTHEN_FUNCTION_FILTER,
    /// rank: Order the records by score.
    rank = THINKTHEN_FUNCTION_RANK,
    /// find: Find the part of the text that answers the question.
    find = THINKTHEN_FUNCTION_FIND,
    /// recognize: Find names of the given kinds.
    recognize = THINKTHEN_FUNCTION_RECOGNIZE,
    /// relate: Find relations between recognized names.
    relate = THINKTHEN_FUNCTION_RELATE,
    /// annotate: Ask several named questions at once.
    annotate = THINKTHEN_FUNCTION_ANNOTATE,
};
/// A single cutoff or not-sure band, validated by the engine.
class [[nodiscard]] threshold {
    double low_, high_;
    bool band_;
    threshold(double low, double high, bool band) : low_(low), high_(high), band_(band) {
        detail::question_owner q(detail::get<thinkthen_question*>(thinkthen_question_new,
            THINKTHEN_FUNCTION_DECIDE, detail::bytes("threshold"), std::size_t{9}), thinkthen_question_free);
        apply(q.get());
    }
public:
    /// Create a cutoff, validated by the engine.
    [[nodiscard]] static threshold cutoff(double value) { return {value, value, false}; }
    /// Create a not-sure band, validated by the engine.
    [[nodiscard]] static threshold band(double low, double high) { return {low, high, true}; }
    /// Apply the threshold to a caller-owned native question.
    void apply(thinkthen_question* q) const {
        if (band_) detail::invoke(thinkthen_question_set_band, q, low_, high_);
        else detail::invoke(thinkthen_question_set_cut, q, low_);
    }
};
/// A relation label, source kind and target kind, with optional description and symmetry.
struct rule {
    /** Relation label. */ std::string label;
    /** Required source name kind. */ std::string source;
    /** Required target name kind. */ std::string target;
    /** Optional wording describing the relation. */ std::optional<std::string> reads;
    /** Permit both endpoint directions. */ bool either = false;
    /// Construct a relation rule, with optional description and symmetry.
    rule(std::string relation, std::string from, std::string to, std::optional<std::string> description = {}, bool symmetric = false)
        : label(std::move(relation)), source(std::move(from)), target(std::move(to)), reads(std::move(description)), either(symmetric) {}
};
namespace detail {
template<class Q> class question {
    std::shared_ptr<const thinkthen_question> value_;
protected:
    explicit question(question_owner value) : value_(value.release(), thinkthen_question_free) {}
    question(std::uint32_t f, std::string_view text) : question(question_owner(
        get<thinkthen_question*>(thinkthen_question_new, static_cast<thinkthen_function>(f), bytes(text), text.size()), thinkthen_question_free)) {}
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
    /// Borrow the immutable native question; it lasts as long as this question.
    [[nodiscard]] const thinkthen_question* native_handle() const noexcept { return value_.get(); }
    /// Select field inside a JSON object with a JSON Pointer.
    [[nodiscard]] Q field(std::string_view pointer) const {
        return changed([&](auto* q) { invoke(thinkthen_question_set_field, q, bytes(pointer), pointer.size()); });
    }
};
}
#define THINKTHEN_QUESTION_COMMON(Q, CODE) \
    friend class detail::question<Q>; \
    friend class toolbox; \
    explicit Q(detail::question_owner value) : detail::question<Q>(std::move(value)) {} \
public: \
    /** The function this question asks. */ \
    static constexpr function kind = function::CODE;
// Required-domain constructors admit their complete snapshots before publication.
/// An immutable decide question admitted and validated by the engine.
class [[nodiscard]] decide : public detail::question<decide> {
    THINKTHEN_QUESTION_COMMON(decide, decide)
    /// Admit this question text through the engine.
    explicit decide(std::string_view text) : question(THINKTHEN_FUNCTION_DECIDE, text) { complete(); }
    /// Return a question with this cutoff or not-sure band.
    [[nodiscard]] decide threshold(thinkthen::threshold value) const { return changed([&](auto* q) { value.apply(q); }); }
    /// Return a question with this cutoff or not-sure band.
    [[nodiscard]] decide threshold(double value) const { return threshold(thinkthen::threshold::cutoff(value)); }
    /// Return a question that describes the meaning of a yes answer.
    decide yes_means(std::string_view text) const { return changed([&](auto* q) { detail::invoke(thinkthen_question_set_yes_means, q, text.data(), text.size()); }); }
    /// Return a question that describes the meaning of a no answer.
    decide no_means(std::string_view text) const { return changed([&](auto* q) { detail::invoke(thinkthen_question_set_no_means, q, text.data(), text.size()); }); }
};
/// An immutable filter question admitted and validated by the engine.
class [[nodiscard]] filter : public detail::question<filter> {
    THINKTHEN_QUESTION_COMMON(filter, filter)
    /// Admit this question text through the engine.
    explicit filter(std::string_view text) : question(THINKTHEN_FUNCTION_FILTER, text) { complete(); }
    /// Return a question with this cutoff or not-sure band.
    [[nodiscard]] filter threshold(thinkthen::threshold value) const { return changed([&](auto* q) { value.apply(q); }); }
    /// Return a question with this cutoff or not-sure band.
    [[nodiscard]] filter threshold(double value) const { return threshold(thinkthen::threshold::cutoff(value)); }
    /// Return a question that describes the meaning of a yes answer.
    filter yes_means(std::string_view text) const { return changed([&](auto* q) { detail::invoke(thinkthen_question_set_yes_means, q, text.data(), text.size()); }); }
    /// Return a question that describes the meaning of a no answer.
    filter no_means(std::string_view text) const { return changed([&](auto* q) { detail::invoke(thinkthen_question_set_no_means, q, text.data(), text.size()); }); }
};
/// An immutable choose question admitted and validated by the engine.
class [[nodiscard]] choose : public detail::question<choose> {
    THINKTHEN_QUESTION_COMMON(choose, choose)
    /// Return a question with descriptions for its named choices, labels or levels.
    choose descriptions(const std::map<std::string, std::string>& meanings) const {
        return changed([&](auto* q) { for (const auto& entry : meanings)
            detail::invoke(thinkthen_question_set_description, q, entry.first.data(), entry.first.size(), entry.second.data(), entry.second.size()); });
    }
    /// Admit the question and its ordered choices.
    choose(std::string_view text, const std::vector<std::string>& choices) : question(THINKTHEN_FUNCTION_CHOOSE, text) {
        add(choices, thinkthen_question_add_choice); complete();
    }
    /// Return a question with this cutoff.
    [[nodiscard]] choose threshold(double value) const { return changed([&](auto* q) { detail::invoke(thinkthen_question_set_cut, q, value); }); }
};
/// An immutable tag question admitted and validated by the engine.
class [[nodiscard]] tag : public detail::question<tag> {
    THINKTHEN_QUESTION_COMMON(tag, tag)
    /// Return a question with descriptions for its named choices, labels or levels.
    tag descriptions(const std::map<std::string, std::string>& meanings) const {
        return changed([&](auto* q) { for (const auto& entry : meanings)
            detail::invoke(thinkthen_question_set_description, q, entry.first.data(), entry.first.size(), entry.second.data(), entry.second.size()); });
    }
    /// Admit the question and its ordered labels.
    tag(std::string_view text, const std::vector<std::string>& labels) : question(THINKTHEN_FUNCTION_TAG, text) {
        add(labels, thinkthen_question_add_label); complete();
    }
    /// Return a question with this cutoff or not-sure band.
    [[nodiscard]] tag threshold(double value) const { return changed([&](auto* q) { detail::invoke(thinkthen_question_set_cut, q, value); }); }
};
/// An immutable score question admitted and validated by the engine.
class [[nodiscard]] score : public detail::question<score> {
    THINKTHEN_QUESTION_COMMON(score, score)
    /// Return a question with descriptions for its named choices, labels or levels.
    score descriptions(const std::map<std::string, std::string>& meanings) const {
        return changed([&](auto* q) { for (const auto& entry : meanings)
            detail::invoke(thinkthen_question_set_description, q, entry.first.data(), entry.first.size(), entry.second.data(), entry.second.size()); });
    }
    /// Admit the question and its ordered levels.
    score(std::string_view text, const std::vector<std::string>& levels) : question(THINKTHEN_FUNCTION_SCORE, text) {
        add(levels, thinkthen_question_add_level); complete();
    }
};
/// An immutable rank question admitted and validated by the engine.
class [[nodiscard]] rank : public detail::question<rank> {
    THINKTHEN_QUESTION_COMMON(rank, rank)
    /// Return a question with descriptions for its named choices, labels or levels.
    rank descriptions(const std::map<std::string, std::string>& meanings) const {
        return changed([&](auto* q) { for (const auto& entry : meanings)
            detail::invoke(thinkthen_question_set_description, q, entry.first.data(), entry.first.size(), entry.second.data(), entry.second.size()); });
    }
    /// Admit the question and its ordered levels.
    rank(std::string_view text, const std::vector<std::string>& levels) : question(THINKTHEN_FUNCTION_RANK, text) {
        add(levels, thinkthen_question_add_level); complete();
    }
    /// Return a question that keeps only this many highest-ranked answered inputs.
    [[nodiscard]] rank top(std::size_t value) const { return changed([&](auto* q) { detail::invoke(thinkthen_question_set_top, q, value); }); }
};
/// An immutable find question admitted and validated by the engine.
class [[nodiscard]] find : public detail::question<find> {
    THINKTHEN_QUESTION_COMMON(find, find)
    /// Admit this question text through the engine.
    explicit find(std::string_view text) : question(THINKTHEN_FUNCTION_FIND, text) { complete(); }
    /// Return a question with this cutoff or not-sure band.
    [[nodiscard]] find threshold(double value) const { return changed([&](auto* q) { detail::invoke(thinkthen_question_set_cut, q, value); }); }
};
/// An immutable recognize question admitted and validated by the engine.
class [[nodiscard]] recognize : public detail::question<recognize> {
    THINKTHEN_QUESTION_COMMON(recognize, recognize)
    /// Admit the name kinds to recognize.
    explicit recognize(const std::vector<std::string>& kinds) : question(THINKTHEN_FUNCTION_RECOGNIZE, {}) {
        add(kinds, thinkthen_question_add_kind); complete();
    }
    /// Return a question with this cutoff or not-sure band.
    [[nodiscard]] recognize threshold(double value) const { return changed([&](auto* q) { detail::invoke(thinkthen_question_set_cut, q, value); }); }
};
/// An immutable relate question admitted and validated by the engine.
class [[nodiscard]] relate : public detail::question<relate> {
    THINKTHEN_QUESTION_COMMON(relate, relate)
    /// Admit the relation rules to apply.
    explicit relate(const std::vector<rule>& rules) : question(THINKTHEN_FUNCTION_RELATE, {}) {
        for (const auto& value : rules) {
            thinkthen_rule input{detail::view(value.label), detail::view(value.source), detail::view(value.target),
                value.reads ? detail::view(*value.reads) : thinkthen_text{}, static_cast<std::uint8_t>(value.either)};
            detail::invoke(thinkthen_question_add_rule, building(), &input);
        }
        complete();
    }
    /// Return a question with this cutoff or not-sure band.
    [[nodiscard]] relate threshold(double value) const { return changed([&](auto* q) { detail::invoke(thinkthen_question_set_cut, q, value); }); }
};
/// The functions that may be annotation children.
using annotation_question = std::variant<decide, choose, tag, score, filter, find, recognize, relate>;
/// Annotation questions keyed by child name.
using annotation_questions = std::map<std::string, annotation_question>;
/// A question that a saved toolbox can hold.
using toolbox_question = std::variant<decide, choose, tag, score, filter, rank, find, recognize, relate>;
/// An immutable annotate question admitted and validated by the engine.
class [[nodiscard]] annotate : public detail::question<annotate> {
    THINKTHEN_QUESTION_COMMON(annotate, annotate)
    /// Admit named child questions; rank and nested annotate are excluded by type.
    explicit annotate(const annotation_questions& entries) : question(THINKTHEN_FUNCTION_ANNOTATE, {}) {
        for (const auto& entry : entries) std::visit([&](const auto& q) {
            detail::invoke(thinkthen_question_add_entry, building(), detail::bytes(entry.first), entry.first.size(), q.native_handle());
        }, entry.second);
        complete();
    }
};
#undef THINKTHEN_QUESTION_COMMON
/// An immutable set of named questions serialized by the engine.
class [[nodiscard]] toolbox {
    std::shared_ptr<const thinkthen_toolbox> native_;
    explicit toolbox(thinkthen_toolbox* value) : native_(value, thinkthen_toolbox_free) {}
    template<std::size_t I = 0> static toolbox_question wrap(std::uint32_t kind, detail::question_owner value) {
        if constexpr (I < std::variant_size_v<toolbox_question>) {
            using Q = std::variant_alternative_t<I, toolbox_question>;
            if (kind == static_cast<std::uint32_t>(Q::kind)) return Q(std::move(value));
            return wrap<I + 1>(kind, std::move(value));
        } else detail::incompatible();
    }
    [[nodiscard]] static std::string buffer(thinkthen_buffer value) {
        std::unique_ptr<thinkthen_buffer, decltype(&thinkthen_buffer_free)> owner(&value, thinkthen_buffer_free);
        return detail::copy({reinterpret_cast<const char *>(value.data), value.len});
    }
public:
    /// Copy named questions into a new toolbox.
    explicit toolbox(const std::map<std::string, toolbox_question>& entries) {
        detail::owner<thinkthen_toolbox, thinkthen_toolbox_free> native(
            detail::get<thinkthen_toolbox*>(thinkthen_toolbox_new), thinkthen_toolbox_free);
        for (const auto& entry : entries) std::visit([&](const auto& q) {
            detail::invoke(thinkthen_toolbox_insert, native.get(), detail::bytes(entry.first), entry.first.size(), q.native_handle());
        }, entry.second);
        native_ = {native.release(), thinkthen_toolbox_free};
    }
    /// Load the saved entries JSON format, admitted by the engine.
    [[nodiscard]] static toolbox from_json(std::string_view json) {
        return toolbox(detail::get<thinkthen_toolbox*>(thinkthen_toolbox_from_json, detail::bytes(json), json.size()));
    }
    /// Serialize the saved entries JSON format.
    [[nodiscard]] std::string to_json() const {
        return buffer(detail::get<thinkthen_buffer>(thinkthen_toolbox_to_json, native_.get()));
    }
    /// Select a named question while retaining its function type.
    [[nodiscard]] toolbox_question select(std::string_view name) const {
        detail::question_owner selected(detail::get<thinkthen_question*>(thinkthen_toolbox_select,
            native_.get(), detail::bytes(name), name.size()), thinkthen_question_free);
        const auto kind = detail::get<thinkthen_function>(thinkthen_question_function, selected.get());
        return wrap(kind, std::move(selected));
    }
    /// Select a named question as Q; the engine rejects a different function.
    template<class Q> [[nodiscard]] Q select(std::string_view name) const {
        detail::question_owner selected(detail::get<thinkthen_question*>(thinkthen_toolbox_select_as,
            native_.get(), detail::bytes(name), name.size(), static_cast<thinkthen_function>(Q::kind)), thinkthen_question_free);
        return Q(std::move(selected));
    }
};
}}
#endif
