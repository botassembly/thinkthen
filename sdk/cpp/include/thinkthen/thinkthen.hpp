#ifndef THINKTHEN_CPP_HPP
#define THINKTHEN_CPP_HPP
#include <thinkthen/answers.hpp>
#include <iterator>
namespace thinkthen { inline namespace v0_2 {
namespace detail {
inline record borrowed(const owned_record& r) {
    return std::visit([](const auto& value) -> record {
        using T = std::decay_t<decltype(value)>;
        if constexpr (std::is_same_v<T, owned_text>) return text{value.value};
        else if constexpr (std::is_same_v<T, owned_json_text>) return json_text{value.value};
        else return missing_record{};
    }, r);
}
inline owned_record owned(record r) {
    return std::visit([](const auto& value) -> owned_record {
        using T = std::decay_t<decltype(value)>;
        if constexpr (std::is_same_v<T, text>) return owned_text{std::string(value.value)};
        else if constexpr (std::is_same_v<T, json_text>) return owned_json_text{std::string(value.value)};
        else return missing_record{};
    }, r);
}
inline thinkthen_record_input input(record r) {
    thinkthen_record_input result{};
    result.struct_size = sizeof result;
    std::visit([&](const auto& value) {
        using T = std::decay_t<decltype(value)>;
        if constexpr (std::is_same_v<T, missing_record>) result.kind = THINKTHEN_RECORD_MISSING;
        else {
            result.kind = std::is_same_v<T, text> ? THINKTHEN_RECORD_TEXT : THINKTHEN_RECORD_JSON;
            result.data = bytes(value.value); result.len = value.value.size();
        }
    }, r);
    return result;
}
inline thinkthen_record_input input(const owned_record& r) {
    if (const auto* json = std::get_if<owned_json_text>(&r); json && record_encoding::invalid(*json)) {
        thinkthen_record_input value{};
        value.struct_size = sizeof value; value.kind = THINKTHEN_RECORD_INVALID;
        return value;
    }
    return input(borrowed(r));
}

struct call_state {
    thinkthen_call* call;
    explicit call_state(thinkthen_call* value) : call(value) {}
    ~call_state() { thinkthen_call_free(call); }
    call_state(const call_state&) = delete;
    call_state& operator=(const call_state&) = delete;
};
}
class cancel_handle {
    std::weak_ptr<detail::call_state> state_;
    explicit cancel_handle(const std::shared_ptr<detail::call_state>& state) : state_(state) {}
    template<class Q> friend class stream;
public:
    cancel_handle() = default;
    void request_cancel() const noexcept {
        if (auto state = state_.lock()) thinkthen_call_cancel(state->call);
    }
    void cancel() const noexcept { request_cancel(); }
};
template<class Q> class stream {
    std::shared_ptr<detail::call_state> state_;
    record_source source_;
    bool started_ = false;
    stream(thinkthen_call* call, record_source source) : source_(std::move(source)) {
        detail::owner<thinkthen_call, thinkthen_call_free> owner(call, thinkthen_call_free);
        state_ = std::make_shared<detail::call_state>(call);
        owner.release();
    }
    friend class client;
    std::optional<row<Q>> pull() {
        if (!state_) return std::nullopt;
        auto state = state_;
        try {
            for (;;) {
                detail::native_row result;
                detail::diagnostic diagnostic;
                auto event = thinkthen_call_next(state->call, &result.value, &diagnostic.value);
                detail::check(event, diagnostic);
                if (event == THINKTHEN_OK) {
                    return row<Q>(result.value);
                }
                if (event == THINKTHEN_END) { close(); return std::nullopt; }
                if (event != THINKTHEN_NEEDS_INPUT) detail::incompatible();
                auto r = source_ ? source_() : std::nullopt;
                if (!r) {
                    thinkthen_call_end_input(state->call); source_ = {};
                } else {
                    const auto input = detail::input(*r);
                    if (input.kind == THINKTHEN_RECORD_TEXT) detail::invoke(thinkthen_call_send_text, state->call, input.data, input.len);
                    else if (input.kind == THINKTHEN_RECORD_JSON) detail::invoke(thinkthen_call_send_json, state->call, input.data, input.len);
                    else if (input.kind == THINKTHEN_RECORD_MISSING) detail::invoke(thinkthen_call_send_missing, state->call);
                    else detail::invoke(thinkthen_call_send_invalid, state->call);
                }
            }
        } catch (...) { close(); throw; }
    }
public:
    stream(const stream&) = delete;
    stream& operator=(const stream&) = delete;
    stream(stream&& other) noexcept : state_(std::move(other.state_)), source_(std::move(other.source_)),
        started_(other.started_) { other.started_ = false; }
    stream& operator=(stream&& other) noexcept {
        if (this != &other) {
            close(); state_ = std::move(other.state_); source_ = std::move(other.source_);
            started_ = other.started_; other.started_ = false;
        }
        return *this;
    }
    ~stream() { close(); }
    void close() noexcept {
        if (state_) thinkthen_call_cancel(state_->call);
        state_.reset(); source_ = {};
    }
    void cancel() const noexcept { if (state_) thinkthen_call_cancel(state_->call); }
    thinkthen::cancel_handle cancel_handle() const noexcept { return thinkthen::cancel_handle(state_); }
    std::optional<row<Q>> next() { return pull(); }
    class iterator {
        stream* stream_ = nullptr;
        std::optional<row<Q>> current_;
        explicit iterator(stream* value) : stream_(value), current_(value->pull()) { if (!current_) stream_ = nullptr; }
        friend class stream;
    public:
        using iterator_category = std::input_iterator_tag;
        using value_type = row<Q>;
        using difference_type = std::ptrdiff_t;
        using reference = const row<Q>&;
        using pointer = const row<Q>*;
        iterator() = default;
        reference operator*() const { return *current_; }
        pointer operator->() const { return &*current_; }
        iterator& operator++() { current_ = stream_->pull(); if (!current_) stream_ = nullptr; return *this; }
        iterator operator++(int) { auto old = *this; ++*this; return old; }
        friend bool operator==(const iterator& a, const iterator& b) { return a.stream_ == b.stream_; }
        friend bool operator!=(const iterator& a, const iterator& b) { return !(a == b); }
    };
    iterator begin() {
        if (!state_) return {};
        if (started_) throw std::logic_error("a stream can be iterated only once");
        started_ = true;
        return iterator(this);
    }
    iterator end() const { return {}; }
};
class client {
    detail::owner<thinkthen_client, thinkthen_client_free> value_{nullptr, thinkthen_client_free};
    void live() const { if (!value_) throw std::logic_error("the client was moved from"); }
    template<class Q> stream<Q> start(const Q& question, record_source source, std::optional<std::string_view> name) const {
        live();
        thinkthen_text named = name ? detail::view(*name) : thinkthen_text{};
        auto* call = detail::get<thinkthen_call*>(thinkthen_call_open_question, value_.get(), question.native_handle(),
            name ? &named : nullptr, THINKTHEN_OFFSET_UTF8);
        return stream<Q>(call, std::move(source));
    }
    template<class Q> stream<Q> start(const Q& question, std::vector<owned_record> records, std::optional<std::string_view> name) const {
        live();
        std::vector<thinkthen_record_input> inputs;
        inputs.reserve(records.size());
        for (const auto& r : records) inputs.push_back(detail::input(r));
        thinkthen_text named = name ? detail::view(*name) : thinkthen_text{};
        auto* call = detail::get<thinkthen_call*>(thinkthen_call_open_records_question, value_.get(), question.native_handle(),
            name ? &named : nullptr, inputs.data(), inputs.size(), sizeof(thinkthen_record_input), THINKTHEN_OFFSET_UTF8);
        return stream<Q>(call, {});
    }
    template<class Q> row<Q> one(const Q& question, record r, std::optional<std::string_view> name) const {
        auto rows = start(question, std::vector<owned_record>{detail::owned(r)}, name);
        auto value = rows.next();
        if (!value || rows.next()) detail::incompatible();
        return std::move(*value);
    }
public:
    explicit client(const settings& settings = {}) {
        detail::check_abi();
        detail::owner<thinkthen_settings, thinkthen_settings_free> s(
            detail::get<thinkthen_settings*>(thinkthen_settings_new), thinkthen_settings_free);
        detail::invoke(thinkthen_settings_set_backend, s.get(), static_cast<std::uint32_t>(settings.backend));
        if (settings.model) detail::invoke(thinkthen_settings_set_model, s.get(), detail::bytes(*settings.model), settings.model->size());
        if (settings.url) detail::invoke(thinkthen_settings_set_url, s.get(), detail::bytes(*settings.url), settings.url->size());
        if (settings.api_key.value_) detail::invoke(thinkthen_settings_copy_api_key, s.get(), settings.api_key.value_.get());
        if (settings.batch) detail::invoke(thinkthen_settings_set_batch, s.get(), *settings.batch);
        if (settings.retry) {
            const auto& r = *settings.retry;
            if (r.initial_backoff.count() < 0 || r.attempt_timeout.count() < 0) detail::invoke(thinkthen_settings_conversion_error);
            detail::invoke(thinkthen_settings_set_retry, s.get(), r.max_retries,
                static_cast<std::uint64_t>(r.initial_backoff.count()), static_cast<std::uint64_t>(r.attempt_timeout.count()));
        }
        value_.reset(detail::get<thinkthen_client*>(thinkthen_client_open, s.get()));
    }
    client(const client&) = delete;
    client& operator=(const client&) = delete;
    client(client&&) noexcept = default;
    client& operator=(client&&) noexcept = default;
#define THINKTHEN_CPP_FUNCTION(Q) \
    row<thinkthen::Q> Q(const thinkthen::Q& question, record r, std::optional<std::string_view> name = {}) const { return one(question, r, name); } \
    stream<thinkthen::Q> Q##_each(const thinkthen::Q& question, record_source source, std::optional<std::string_view> name = {}) const { return start(question, std::move(source), name); } \
    stream<thinkthen::Q> Q##_each(const thinkthen::Q& question, std::vector<owned_record> records, std::optional<std::string_view> name = {}) const { return start(question, std::move(records), name); }
    THINKTHEN_CPP_FUNCTION(decide) THINKTHEN_CPP_FUNCTION(choose)
    THINKTHEN_CPP_FUNCTION(tag) THINKTHEN_CPP_FUNCTION(score)
    THINKTHEN_CPP_FUNCTION(filter) THINKTHEN_CPP_FUNCTION(rank)
    THINKTHEN_CPP_FUNCTION(find) THINKTHEN_CPP_FUNCTION(recognize)
    THINKTHEN_CPP_FUNCTION(relate) THINKTHEN_CPP_FUNCTION(annotate)
#undef THINKTHEN_CPP_FUNCTION
};
}}
#endif
