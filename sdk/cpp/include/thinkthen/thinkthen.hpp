#ifndef THINKTHEN_CPP_HPP
#define THINKTHEN_CPP_HPP
#include <thinkthen/answers.hpp>
#include <iterator>
#include <ranges>
#include <deque>
namespace thinkthen { inline namespace v0_2 {
namespace detail {
template<class T> concept recordable = requires(const T& value) { { to_record(value) } -> std::same_as<record>; };
inline thinkthen_record_input input(const record& value) {
    thinkthen_record_input result{}; result.struct_size = sizeof result;
    std::visit([&](const auto& item) {
        using T = std::decay_t<decltype(item)>;
        if constexpr (std::same_as<T, missing_record>) result.kind = THINKTHEN_RECORD_MISSING;
        else {
            if constexpr (std::same_as<T, owned_json_text>) {
                if (record_encoding::invalid(item)) { result.kind = THINKTHEN_RECORD_INVALID; return; }
            }
            result.kind = (std::same_as<T, text> || std::same_as<T, owned_text>) ? THINKTHEN_RECORD_TEXT : THINKTHEN_RECORD_JSON;
            result.data = item.value.data(); result.len = item.value.size();
        }
    }, value);
    return result;
}
struct cancellation {
    owner<thinkthen_cancel, thinkthen_cancel_free> native{nullptr, thinkthen_cancel_free};
    std::unique_ptr<std::stop_callback<std::function<void()>>> callback;
    explicit cancellation(std::stop_token token)
        : native(get<thinkthen_cancel*>(thinkthen_cancel_new), thinkthen_cancel_free) {
        callback = std::make_unique<std::stop_callback<std::function<void()>>>(token,
            [stop = native.get()] { thinkthen_cancel_request(stop); });
    }
};
struct call_state {
    thinkthen_call* call;
    std::shared_ptr<cancellation> stop;
    call_state(thinkthen_call* value, std::shared_ptr<cancellation> token) : call(value), stop(std::move(token)) {}
    ~call_state() { thinkthen_call_free(call); }
    call_state(const call_state&) = delete;
    call_state& operator=(const call_state&) = delete;
};
template<class T> struct record_slot {
    const T* item;
    std::shared_ptr<const void> owner;
};
template<class T> struct record_input {
    virtual ~record_input() = default;
    virtual std::optional<record_slot<T>> next() = 0;
};
template<class V> class range_input final : public record_input<std::remove_cvref_t<std::ranges::range_reference_t<V>>> {
    using T = std::remove_cvref_t<std::ranges::range_reference_t<V>>;
    std::shared_ptr<V> range_;
    std::ranges::iterator_t<V> current_;
    std::ranges::sentinel_t<V> end_;
    bool advance_ = false;
    struct generated { T value; std::shared_ptr<V> range; };
public:
    explicit range_input(V range) : range_(std::make_shared<V>(std::move(range))),
        current_(std::ranges::begin(*range_)), end_(std::ranges::end(*range_)) {}
    std::optional<record_slot<T>> next() override {
        if (advance_) { ++current_; advance_ = false; }
        if (current_ == end_) return {};
        advance_ = true;
        if constexpr (std::ranges::forward_range<V> && std::is_lvalue_reference_v<std::ranges::range_reference_t<V>>) {
            const T* value = std::addressof(*current_);
            return record_slot<T>{value, range_};
        } else {
            auto value = [&] {
                if constexpr (std::constructible_from<T, std::ranges::range_reference_t<V>>)
                    return std::make_shared<generated>(generated{*current_, range_});
                else return std::make_shared<generated>(generated{std::ranges::iter_move(current_), range_});
            }();
            return record_slot<T>{&value->value, value};
        }
    }
};
class function_input final : public record_input<record> {
    record_source next_;
public:
    explicit function_input(record_source source) : next_(std::move(source)) {}
    std::optional<record_slot<record>> next() override {
        auto value = next_(); if (!value) return {};
        auto retained = std::make_shared<record>(std::move(*value));
        return record_slot<record>{retained.get(), retained};
    }
};
template<class R> concept record_range = std::ranges::viewable_range<R> && recordable<std::ranges::range_reference_t<R>>;
}
/// A weak cancellation handle, safe to use from another thread.
class [[nodiscard]] cancel_handle {
    std::weak_ptr<detail::call_state> state_;
    explicit cancel_handle(const std::shared_ptr<detail::call_state>& state) : state_(state) {}
    template<class, class> friend class stream;
public:
    cancel_handle() = default;
    void request_cancel() const noexcept { if (auto state = state_.lock()) thinkthen_call_cancel(state->call); }
};
/// A move-only single-pass range retaining each caller record by its input position.
template<class Q, class T = record> class [[nodiscard]] stream {
    std::shared_ptr<detail::call_state> state_;
    std::unique_ptr<detail::record_input<T>> source_;
    std::map<std::uint64_t, detail::record_slot<T>> pending_;
    std::uint64_t submitted_ = 0;
    bool started_ = false;
    friend class client;
    stream(thinkthen_call* call, std::shared_ptr<detail::cancellation> stop,
        std::unique_ptr<detail::record_input<T>> source)
        : source_(std::move(source)) {
        detail::owner<thinkthen_call, thinkthen_call_free> owner(call, thinkthen_call_free);
        state_ = std::make_shared<detail::call_state>(call, std::move(stop)); owner.release();
    }
    std::optional<row<Q, T>> pull() {
        if (!state_) return {};
        auto state = state_;
        for (;;) {
            detail::native_row value;
            detail::diagnostic diagnostic;
            const auto event = thinkthen_call_next(state->call, &value.value, &diagnostic.value);
            detail::check(event, diagnostic);
            if (event == THINKTHEN_OK) {
                auto found = pending_.find(value.value.position);
                if (found == pending_.end()) detail::incompatible();
                auto record = std::move(found->second); pending_.erase(found);
                return row<Q, T>(std::move(value), record.item, std::move(record.owner));
            }
            if (event == THINKTHEN_END) { close(); return {}; }
            if (event != THINKTHEN_NEEDS_INPUT) detail::incompatible();
            auto item = source_ ? source_->next() : std::nullopt;
            if (!item) { thinkthen_call_end_input(state->call); source_.reset(); continue; }
            pending_.emplace(++submitted_, *item);
            std::optional<record> encoding;
            try { encoding.emplace(to_record(*item->item)); }
            catch (const std::bad_alloc&) { throw; }
            catch (...) { detail::invoke(thinkthen_call_send_invalid, state->call); continue; }
            const auto input = detail::input(*encoding);
            if (input.kind == THINKTHEN_RECORD_TEXT) detail::invoke(thinkthen_call_send_text, state->call, input.data, input.len);
            else if (input.kind == THINKTHEN_RECORD_JSON) detail::invoke(thinkthen_call_send_json, state->call, input.data, input.len);
            else if (input.kind == THINKTHEN_RECORD_MISSING) detail::invoke(thinkthen_call_send_missing, state->call);
            else detail::invoke(thinkthen_call_send_invalid, state->call);
        }
    }
public:
    stream(const stream&) = delete;
    stream& operator=(const stream&) = delete;
    stream(stream&&) noexcept = default;
    stream& operator=(stream&& other) noexcept {
        if (this != &other) { close(); state_ = std::move(other.state_); source_ = std::move(other.source_);
            pending_ = std::move(other.pending_); submitted_ = other.submitted_; started_ = other.started_; }
        return *this;
    }
    ~stream() { close(); }
    void close() noexcept { if (state_) thinkthen_call_cancel(state_->call); state_.reset(); source_.reset(); pending_.clear(); }
    [[nodiscard]] thinkthen::cancel_handle cancel_handle() const noexcept { return thinkthen::cancel_handle(state_); }
    [[nodiscard]] std::optional<row<Q, T>> next() { return pull(); }
    class iterator {
        stream* stream_ = nullptr;
        std::optional<row<Q, T>> current_;
    public:
        using iterator_category = std::input_iterator_tag;
        using value_type = row<Q, T>;
        using difference_type = std::ptrdiff_t;
        iterator() = default;
        explicit iterator(stream* value) : stream_(value), current_(value->pull()) {}
        row<Q, T>& operator*() const { return const_cast<row<Q, T>&>(*current_); }
        row<Q, T>* operator->() const { return std::addressof(operator*()); }
        iterator& operator++() { current_ = stream_->pull(); return *this; }
        void operator++(int) { ++*this; }
        friend bool operator==(const iterator& value, std::default_sentinel_t) { return !value.current_; }
    };
    [[nodiscard]] iterator begin() {
        if (started_) throw std::logic_error("a stream can be read once");
        started_ = true; return iterator(this);
    }
    [[nodiscard]] std::default_sentinel_t end() const noexcept { return {}; }
};
/// A range of the original records that answered yes, followed by one aggregate failure.
template<class T> class [[nodiscard]] kept_stream {
    stream<filter, T> rows_;
    std::vector<row<filter, T>> failures_;
    std::optional<row<filter, T>> current_;
    bool started_ = false;
public:
    explicit kept_stream(stream<filter, T> rows) : rows_(std::move(rows)) {}
    kept_stream(kept_stream&&) noexcept = default;
    kept_stream& operator=(kept_stream&&) noexcept = default;
    void close() noexcept { rows_.close(); }
    [[nodiscard]] thinkthen::cancel_handle cancel_handle() const noexcept { return rows_.cancel_handle(); }
    [[nodiscard]] std::optional<std::reference_wrapper<const T>> next() {
        current_.reset();
        while (auto value = rows_.next()) {
            if (value->failed()) failures_.push_back(std::move(*value));
            else if (value->answer()) { current_.emplace(std::move(*value)); return std::cref(current_->item()); }
        }
        if (!failures_.empty()) {
            const auto failure = *failures_.front().error();
            throw filter_error<T>(failure, std::move(failures_));
        }
        return {};
    }
    class iterator {
        kept_stream* stream_{};
        std::optional<std::reference_wrapper<const T>> current_;
    public:
        using iterator_category = std::input_iterator_tag;
        using value_type = T;
        using difference_type = std::ptrdiff_t;
        iterator() = default;
        explicit iterator(kept_stream* value) : stream_(value), current_(value->next()) {}
        const T& operator*() const { return current_->get(); }
        const T* operator->() const { return std::addressof(current_->get()); }
        iterator& operator++() { current_ = stream_->next(); return *this; }
        void operator++(int) { ++*this; }
        friend bool operator==(const iterator& value, std::default_sentinel_t) { return !value.current_; }
    };
    [[nodiscard]] iterator begin() { if (started_) throw std::logic_error("a stream can be read once"); started_ = true; return iterator(this); }
    [[nodiscard]] std::default_sentinel_t end() const noexcept { return {}; }
};
/// Provider-reported subtotal and count of finished attempts with unknown tokens.
struct token_usage {
    const std::optional<std::uint64_t> reported;
    const std::uint64_t unknown_attempts;
};
/// Immutable copied cumulative usage, independent of client lifetime.
struct usage_snapshot {
    const std::uint64_t attempts, responses, status_retries, connection_retries;
    const std::uint64_t request_body_bytes, active_attempts, active_calls;
    const token_usage input_tokens, output_tokens;
};
/// An owned native client. Open calls keep cancellation and native resources alive.
class client {
    detail::owner<thinkthen_client, thinkthen_client_free> value_{nullptr, thinkthen_client_free};
    std::shared_ptr<detail::cancellation> stop_;
    void live() const { if (!value_) throw std::logic_error("the client was moved from"); }
    template<class Q, class T> stream<Q, T> start(const Q& question,
        std::unique_ptr<detail::record_input<T>> source, std::optional<std::string_view> name) const {
        live(); const auto named = name ? detail::view(*name) : thinkthen_text{};
        auto* call = detail::get<thinkthen_call*>(thinkthen_call_open_question, value_.get(), question.native_handle(),
            name ? &named : nullptr, THINKTHEN_OFFSET_UTF8);
        return stream<Q, T>(call, stop_, std::move(source));
    }
    template<class Q, detail::record_range R> auto start(const Q& question, R&& records,
        std::optional<std::string_view> name) const {
        using V = std::views::all_t<R>;
        using T = std::remove_cvref_t<std::ranges::range_reference_t<V>>;
        std::unique_ptr<detail::record_input<T>> source = std::make_unique<detail::range_input<V>>(std::views::all(std::forward<R>(records)));
        return start(question, std::move(source), name);
    }
    template<class Q> auto start(const Q& question, record_source source, std::optional<std::string_view> name) const {
        std::unique_ptr<detail::record_input<record>> input = std::make_unique<detail::function_input>(std::move(source));
        return start(question, std::move(input), name);
    }
    template<class Q, detail::recordable R> row<Q> one(const Q& question, const R& value,
        std::optional<std::string_view> name) const {
        live(); std::optional<record> encoded;
        try { encoded.emplace(to_record(value)); }
        catch (const std::bad_alloc&) { throw; }
        catch (...) { encoded.emplace(detail::record_encoding::invalid()); }
        const auto input = detail::input(*encoded);
        const auto named = name ? detail::view(*name) : thinkthen_text{};
        detail::owner<thinkthen_call, thinkthen_call_free> call(detail::get<thinkthen_call*>(thinkthen_call_open_records_question,
            value_.get(), question.native_handle(), name ? &named : nullptr, &input, std::size_t{1}, sizeof input, THINKTHEN_OFFSET_UTF8), thinkthen_call_free);
        detail::native_row result;
        detail::diagnostic diagnostic;
        const auto event = thinkthen_call_next(call.get(), &result.value, &diagnostic.value);
        detail::check(event, diagnostic);
        if (event != THINKTHEN_OK) detail::incompatible();
        auto row = thinkthen::row<Q>(std::move(result));
        if (const auto* failure = row.error()) throw *failure;
        return row;
    }
public:
    explicit client(const settings& settings = {}) {
        detail::check_abi();
        // Only numeric supplied fields enter this fragment; native admission owns defaults.
        std::string encoded = "{\"retry\":{";
        if (settings.retry) {
            const auto& retry = *settings.retry;
            if (retry.max_retries) encoded += "\"max_retries\":" + std::to_string(*retry.max_retries) + ",";
            const auto duration = [&](std::string_view name, const auto& value) {
                if (value) encoded += "\"" + std::string(name) + "\":" + std::to_string(value->count()) + ",";
            };
            duration("initial_backoff_ms", retry.initial_backoff);
            duration("attempt_timeout_ms", retry.attempt_timeout);
        }
        if (encoded.back() == ',') encoded.pop_back();
        encoded += "}}";
        detail::owner<thinkthen_settings, thinkthen_settings_free> native(
            detail::get<thinkthen_settings*>(thinkthen_settings_from_json, encoded.data(), encoded.size()), thinkthen_settings_free);
        if (settings.backend) detail::invoke(thinkthen_settings_set_backend, native.get(), static_cast<thinkthen_backend>(*settings.backend));
        if (settings.url) detail::invoke(thinkthen_settings_set_url, native.get(), settings.url->data(), settings.url->size());
        if (settings.model) detail::invoke(thinkthen_settings_set_model, native.get(), settings.model->data(), settings.model->size());
        if (settings.api_key.value_) detail::invoke(thinkthen_settings_copy_api_key, native.get(), settings.api_key.value_.get());
        if (settings.batch) detail::invoke(thinkthen_settings_set_batch, native.get(), *settings.batch);
        if (settings.max_requests) detail::invoke(thinkthen_settings_set_max_requests, native.get(), *settings.max_requests);
        if (settings.stop.stop_possible()) {
            stop_ = std::make_shared<detail::cancellation>(settings.stop);
            detail::invoke(thinkthen_settings_set_cancel, native.get(), stop_->native.get());
        }
        value_.reset(detail::get<thinkthen_client*>(thinkthen_client_open, native.get()));
    }
    client(const client&) = delete;
    client& operator=(const client&) = delete;
    client(client&&) noexcept = default;
    client& operator=(client&&) noexcept = default;
    [[nodiscard]] usage_snapshot usage() const {
        live(); thinkthen_usage value{}; value.struct_size = sizeof value;
        detail::invoke(thinkthen_client_usage_values, value_.get(), &value);
        const auto tokens = [](const thinkthen_token_usage& token) -> token_usage {
            return {token.reported_known ? std::optional<std::uint64_t>{token.reported} : std::nullopt, token.unknown_attempts};
        };
        return {value.attempts, value.responses, value.status_retries, value.connection_retries,
            value.request_body_bytes, value.active_attempts, value.active_calls,
            tokens(value.input_tokens), tokens(value.output_tokens)};
    }
#define THINKTHEN_EACH(F) \
    template<class Q, detail::record_range R> requires (Q::kind == function::F) \
    [[nodiscard]] auto F##_each(const Q& question, R&& records, std::optional<std::string_view> name = {}) const { \
        return start(question, std::forward<R>(records), name); } \
    template<class Q> requires (Q::kind == function::F) \
    [[nodiscard]] auto F##_each(const Q& question, record_source source, std::optional<std::string_view> name = {}) const { \
        return start(question, std::move(source), name); } \
    template<class Q, std::input_iterator I, std::sentinel_for<I> S> requires (Q::kind == function::F) \
    [[nodiscard]] auto F##_each(const Q& question, I first, S last, std::optional<std::string_view> name = {}) const { \
        return start(question, std::ranges::subrange(first, last), name); }
#define THINKTHEN_ONE(F) \
    template<class Q, detail::recordable R> requires (Q::kind == function::F) \
    [[nodiscard]] row<Q> F(const Q& question, const R& record, std::optional<std::string_view> name = {}) const { \
        return one(question, record, name); } \
    THINKTHEN_EACH(F)
    THINKTHEN_ONE(decide) THINKTHEN_ONE(choose) THINKTHEN_ONE(tag) THINKTHEN_ONE(score)
    THINKTHEN_ONE(find) THINKTHEN_ONE(recognize) THINKTHEN_ONE(relate) THINKTHEN_ONE(annotate)
    THINKTHEN_EACH(rank)
#undef THINKTHEN_ONE
#undef THINKTHEN_EACH
    template<detail::record_range R>
    [[nodiscard]] auto filter_each(const thinkthen::filter& question, R&& records,
        std::optional<std::string_view> name = {}) const {
        auto rows = start(question, std::forward<R>(records), name);
        using T = std::remove_cvref_t<decltype(std::declval<decltype(*rows.begin())>().item())>;
        return kept_stream<T>(std::move(rows));
    }
    [[nodiscard]] auto filter_each(const thinkthen::filter& question, record_source source,
        std::optional<std::string_view> name = {}) const {
        return kept_stream<record>(start(question, std::move(source), name));
    }
    template<std::input_iterator I, std::sentinel_for<I> S>
    [[nodiscard]] auto filter_each(const thinkthen::filter& question, I first, S last,
        std::optional<std::string_view> name = {}) const {
        return filter_each(question, std::ranges::subrange(first, last), name);
    }
};
}}
#endif
