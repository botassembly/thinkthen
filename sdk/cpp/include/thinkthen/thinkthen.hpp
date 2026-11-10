#ifndef THINKTHEN_CPP_HPP
#define THINKTHEN_CPP_HPP
#include <thinkthen/answers.hpp>
#include <iterator>
namespace thinkthen { inline namespace v0_2 {
namespace detail {
[[nodiscard]] inline record borrowed(const owned_record& r) {
    return std::visit([](const auto& value) -> record {
        using T = std::decay_t<decltype(value)>;
        if constexpr (std::is_same_v<T, owned_text>) return text{value.value};
        else if constexpr (std::is_same_v<T, owned_json_text>) return json_text{value.value};
        else return missing_record{};
    }, r);
}
[[nodiscard]] inline owned_record owned(record r) {
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

template<class Iterator> std::vector<owned_record> own_records(Iterator first, Iterator last) {
    std::vector<owned_record> records;
    for (; first != last; ++first) {
        if constexpr (std::is_convertible_v<decltype(*first), std::string_view>) records.emplace_back(owned_text{std::string(std::string_view(*first))});
        else records.emplace_back(*first);
    }
    return records;
}

struct call_state {
    thinkthen_call* call;
    explicit call_state(thinkthen_call* value) : call(value) {}
    ~call_state() { thinkthen_call_free(call); }
    call_state(const call_state&) = delete;
    call_state& operator=(const call_state&) = delete;
};
}
/// A weak cancellation handle that can be used safely from another thread.
class [[nodiscard]] cancel_handle {
    std::weak_ptr<detail::call_state> state_;
    explicit cancel_handle(const std::shared_ptr<detail::call_state>& state) : state_(state) {}
    template<class Q> friend class stream;
public:
    /// Construct an inactive handle; cancellation has no effect.
    cancel_handle() = default;
    /// Ask the native call to stop; repeated requests are harmless.
    void request_cancel() const noexcept {
        if (auto state = state_.lock()) thinkthen_call_cancel(state->call);
    }
};
/// A move-only, single-pass stream in engine order; rank puts successful ranks first.
template<class Q> class [[nodiscard]] stream {
    std::shared_ptr<detail::call_state> state_;
    record_source source_;
    bool started_ = false;
    stream(thinkthen_call* call, record_source source) : source_(std::move(source)) {
        detail::owner<thinkthen_call, thinkthen_call_free> owner(call, thinkthen_call_free);
        state_ = std::make_shared<detail::call_state>(call);
        owner.release();
    }
    friend class client;
    [[nodiscard]] std::optional<row<Q>> pull() {
        if (!state_) return std::nullopt;
        auto state = state_;
        for (;;) {
            detail::native_row result;
            detail::diagnostic diagnostic;
            auto event = thinkthen_call_next(state->call, &result.value, &diagnostic.value);
            // Retain the native call's terminal error for later pulls.
            detail::check(event, diagnostic);
            if (event == THINKTHEN_OK) {
                try { return row<Q>(result.value); }
                catch (...) { close(); throw; }
            }
            if (event == THINKTHEN_END) { close(); return std::nullopt; }
            if (event != THINKTHEN_NEEDS_INPUT) detail::incompatible();
            std::optional<owned_record> r;
            try { r = source_ ? source_() : std::nullopt; }
            catch (...) { close(); throw; }
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
    }
public:
    stream(const stream&) = delete;
    stream& operator=(const stream&) = delete;
    /// Transfer the stream and its native call.
    stream(stream&& other) noexcept : state_(std::move(other.state_)), source_(std::move(other.source_)),
        started_(other.started_) { other.started_ = false; }
    /// Close the current call and take ownership of another stream.
    stream& operator=(stream&& other) noexcept {
        if (this != &other) {
            close(); state_ = std::move(other.state_); source_ = std::move(other.source_);
            started_ = other.started_; other.started_ = false;
        }
        return *this;
    }
    /// Cancel and release any unread native call.
    ~stream() { close(); }
    /// Cancel and release this call.
    void close() noexcept {
        if (state_) thinkthen_call_cancel(state_->call);
        state_.reset(); source_ = {};
    }
    /// Obtain a handle that cancels without owning the stream.
    [[nodiscard]] thinkthen::cancel_handle cancel_handle() const noexcept { return thinkthen::cancel_handle(state_); }
    /// Pull one row, or nullopt after input is drained; call errors keep throwing until close().
    [[nodiscard]] std::optional<row<Q>> next() { return pull(); }
    /// A single-pass iterator over typed rows.
    class iterator {
        stream* stream_ = nullptr;
        std::optional<row<Q>> current_;
        explicit iterator(stream* value) : stream_(value), current_(value->pull()) { if (!current_) stream_ = nullptr; }
        friend class stream;
    public:
        /// Standard single-pass iterator category.
        using iterator_category = std::input_iterator_tag;
        /// Owned value produced by this iterator.
        using value_type = row<Q>;
        /// Iterator difference type.
        using difference_type = std::ptrdiff_t;
        /// Borrowed value valid until increment.
        using reference = const row<Q>&;
        /// Borrowed pointer valid until increment.
        using pointer = const row<Q>*;
        /// Construct the end sentinel.
        iterator() = default;
        /// Read the current value.
        reference operator*() const { return *current_; }
        /// Read the current value through a pointer.
        pointer operator->() const { return &*current_; }
        /// Pull the next value.
        iterator& operator++() { current_ = stream_->pull(); if (!current_) stream_ = nullptr; return *this; }
        /// Pull the next value and return the previous iterator.
        iterator operator++(int) { auto old = *this; ++*this; return old; }
        /// Compare iterator positions with the end sentinel.
        friend bool operator==(const iterator& a, const iterator& b) { return a.stream_ == b.stream_; }
        /// Test whether iterator positions differ.
        friend bool operator!=(const iterator& a, const iterator& b) { return !(a == b); }
    };
    /// Begin iteration; a stream can be iterated only once.
    [[nodiscard]] iterator begin() {
        if (!state_) return {};
        if (started_) throw std::logic_error("a stream can be iterated only once");
        started_ = true;
        return iterator(this);
    }
    /// Return the end sentinel.
    [[nodiscard]] iterator end() const { return {}; }
};
/// A stream of kept records. Failures are reported together after input is drained.
class [[nodiscard]] kept_stream {
    stream<filter> rows_;
    std::vector<failed_row> failures_;
    bool started_ = false;
public:
    /// Take ownership of a full filter row stream.
    explicit kept_stream(stream<filter> rows) : rows_(std::move(rows)) {}
    kept_stream(kept_stream&&) noexcept = default;
    kept_stream& operator=(kept_stream&&) noexcept = default;
    /// Stop this stream and release its native call.
    void close() noexcept { rows_.close(); }
    /// Obtain a handle that can cancel from another thread.
    [[nodiscard]] thinkthen::cancel_handle cancel_handle() const noexcept { return rows_.cancel_handle(); }
    /// Pull the next kept record; throw one aggregate failure at the end.
    [[nodiscard]] std::optional<owned_record> next() {
        while (auto current = rows_.next()) {
            if (current->failed()) {
                const auto& failure = *current->failure();
                failed_row failed{current->position(), std::string(current->id_json()), failure.code(), failure.what(), failure.http_status(), THINKTHEN_RECORD_MISSING, {}};
                std::visit([&](const auto& item) {
                    using T = std::decay_t<decltype(item)>;
                    if constexpr (std::is_same_v<T, owned_text>) { failed.record_kind = THINKTHEN_RECORD_TEXT; failed.record = item.value; }
                    else if constexpr (std::is_same_v<T, owned_json_text>) { failed.record_kind = detail::record_encoding::invalid(item) ? THINKTHEN_RECORD_INVALID : THINKTHEN_RECORD_JSON; failed.record = item.value; }
                }, current->item());
                failures_.push_back(std::move(failed));
            } else if (current->kept()) return current->item();
        }
        if (!failures_.empty()) {
            auto failures = std::move(failures_);
            const auto first = failures.front();
            throw error(static_cast<errc>(first.code.value()), first.message, first.http_status, std::move(failures));
        }
        return std::nullopt;
    }
    /// A single-pass iterator over kept records.
    class iterator {
        kept_stream* stream_ = nullptr;
        std::optional<owned_record> current_;
    public:
        /// Standard single-pass iterator category.
        using iterator_category = std::input_iterator_tag;
        /// Owned value produced by this iterator.
        using value_type = owned_record;
        /// Iterator difference type.
        using difference_type = std::ptrdiff_t;
        /// Borrowed value valid until increment.
        using reference = const owned_record&;
        /// Borrowed pointer valid until increment.
        using pointer = const owned_record*;
        /// Construct the end sentinel.
        iterator() = default;
        /// Start reading kept records from this stream.
        explicit iterator(kept_stream* stream) : stream_(stream), current_(stream->next()) { if (!current_) stream_ = nullptr; }
        /// Read the current value.
        reference operator*() const { return *current_; }
        /// Read the current value through a pointer.
        pointer operator->() const { return &*current_; }
        /// Pull the next value.
        iterator& operator++() { current_ = stream_->next(); if (!current_) stream_ = nullptr; return *this; }
        /// Pull the next value and return the previous iterator.
        iterator operator++(int) { auto old = *this; ++*this; return old; }
        /// Compare iterator positions with the end sentinel.
        friend bool operator==(const iterator& a, const iterator& b) { return a.stream_ == b.stream_; }
        /// Test whether iterator positions differ.
        friend bool operator!=(const iterator& a, const iterator& b) { return !(a == b); }
    };
    /// Begin iteration; a stream can be iterated only once.
    [[nodiscard]] iterator begin() { if (started_) throw std::logic_error("a stream can be iterated only once"); started_ = true; return iterator(this); }
    /// Return the end sentinel.
    [[nodiscard]] iterator end() const { return {}; }
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
    const std::uint64_t cache_hits, replay_hits;
};
/// An owned native client; calls opened from it keep their own native state.
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
        if (value->failure() && value->failure()->code() == make_error_code(errc::record_empty)) throw *value->failure();
        return std::move(*value);
    }
public:
    /// Open a client with engine-validated settings and defaults.
    explicit client(const settings& settings = {}) {
        detail::check_abi();
        detail::owner<thinkthen_settings, thinkthen_settings_free> s(
            detail::get<thinkthen_settings*>(thinkthen_settings_new), thinkthen_settings_free);
        if (settings.backend) detail::invoke(thinkthen_settings_set_backend, s.get(), static_cast<thinkthen_backend>(*settings.backend));
        if (settings.model) detail::invoke(thinkthen_settings_set_model, s.get(), detail::bytes(*settings.model), settings.model->size());
        if (settings.url) detail::invoke(thinkthen_settings_set_url, s.get(), detail::bytes(*settings.url), settings.url->size());
        if (settings.api_key.value_) detail::invoke(thinkthen_settings_copy_api_key, s.get(), settings.api_key.value_.get());
        if (settings.batch) detail::invoke(thinkthen_settings_set_batch, s.get(), *settings.batch);
        if (settings.max_requests) detail::invoke(thinkthen_settings_set_max_requests, s.get(), *settings.max_requests);
        if (settings.retry) {
            const auto& r = *settings.retry;
            if (r.initial_backoff.count() < 0 || r.attempt_timeout.count() < 0) detail::invoke(thinkthen_settings_conversion_error);
            detail::invoke(thinkthen_settings_set_retry, s.get(), r.max_retries,
                static_cast<std::uint64_t>(r.initial_backoff.count()), static_cast<std::uint64_t>(r.attempt_timeout.count()));
        }
        if (settings.response_store) {
            const auto& store = *settings.response_store;
            thinkthen_response_store_config config{};
            config.struct_size = sizeof config;
            config.path = detail::view(store.path);
            config.mode = static_cast<std::uint32_t>(store.mode);
            config.namespace_name = detail::view(store.namespace_name);
            config.model_generation = detail::view(store.model_generation);
            config.max_bytes = store.max_bytes ? &*store.max_bytes : nullptr;
            config.max_entries = store.max_entries ? &*store.max_entries : nullptr;
            config.max_age = store.max_age ? &*store.max_age : nullptr;
            config.busy_timeout_ms = store.busy_timeout_ms ? &*store.busy_timeout_ms : nullptr;
            detail::invoke(thinkthen_settings_set_response_store, s.get(), &config);
        }
        value_.reset(detail::get<thinkthen_client*>(thinkthen_client_open, s.get()));
    }
    /// Copy cumulative usage. Existing calls may still send after this snapshot.
    [[nodiscard]] usage_snapshot usage() const {
        live();
        thinkthen_usage_v2 snapshot{};
        snapshot.struct_size = sizeof snapshot;
        detail::invoke(thinkthen_client_usage_v2_values, value_.get(), &snapshot);
        const auto& value = snapshot.usage;
        const auto tokens = [](const thinkthen_token_usage& token) -> token_usage {
            return {token.reported_known ? std::optional<std::uint64_t>{token.reported} : std::nullopt, token.unknown_attempts};
        };
        return {value.attempts, value.responses, value.status_retries, value.connection_retries,
            value.request_body_bytes, value.active_attempts, value.active_calls,
            tokens(value.input_tokens), tokens(value.output_tokens), snapshot.cache_hits, snapshot.replay_hits};
    }
    client(const client&) = delete;
    client& operator=(const client&) = delete;
    client(client&&) noexcept = default;
    client& operator=(client&&) noexcept = default;
    /// Judge one record with a decide question. Blank field throws record.empty.
    [[nodiscard]] row<thinkthen::decide> decide(const thinkthen::decide& question, record input, std::optional<std::string_view> name = {}) const { return one(question, input, name); }
    /// Judge one text record; strings are copied before the call returns.
    [[nodiscard]] row<thinkthen::decide> decide(const thinkthen::decide& question, std::string_view input, std::optional<std::string_view> name = {}) const { return one(question, text{input}, name); }
    /// Stream every decide row in input order, including not-sure and failed rows.
    [[nodiscard]] stream<thinkthen::decide> decide_each(const thinkthen::decide& question, record_source source, std::optional<std::string_view> name = {}) const { return start(question, std::move(source), name); }
    /// Judge owned records; the returned stream owns its inputs.
    [[nodiscard]] stream<thinkthen::decide> decide_each(const thinkthen::decide& question, std::vector<owned_record> records, std::optional<std::string_view> name = {}) const { return start(question, std::move(records), name); }
    /// Judge a collection of strings, copying each text into the stream.
    template<class T, std::enable_if_t<std::is_convertible_v<T, std::string_view>, int> = 0>
    [[nodiscard]] stream<thinkthen::decide> decide_each(const thinkthen::decide& question, const std::vector<T>& records, std::optional<std::string_view> name = {}) const { return decide_each(question, records.begin(), records.end(), name); }
    /// Judge an iterator range of strings or owned records in input order.
    template<class Iterator>
    [[nodiscard]] stream<thinkthen::decide> decide_each(const thinkthen::decide& question, Iterator first, Iterator last, std::optional<std::string_view> name = {}) const {
        return start(question, detail::own_records(first, last), name);
    }
    /// Judge one record with a choose question. Blank field throws record.empty.
    [[nodiscard]] row<thinkthen::choose> choose(const thinkthen::choose& question, record input, std::optional<std::string_view> name = {}) const { return one(question, input, name); }
    /// Judge one text record; strings are copied before the call returns.
    [[nodiscard]] row<thinkthen::choose> choose(const thinkthen::choose& question, std::string_view input, std::optional<std::string_view> name = {}) const { return one(question, text{input}, name); }
    /// Stream every choose row in input order, including not-sure and failed rows.
    [[nodiscard]] stream<thinkthen::choose> choose_each(const thinkthen::choose& question, record_source source, std::optional<std::string_view> name = {}) const { return start(question, std::move(source), name); }
    /// Judge owned records; the returned stream owns its inputs.
    [[nodiscard]] stream<thinkthen::choose> choose_each(const thinkthen::choose& question, std::vector<owned_record> records, std::optional<std::string_view> name = {}) const { return start(question, std::move(records), name); }
    /// Judge a collection of strings, copying each text into the stream.
    template<class T, std::enable_if_t<std::is_convertible_v<T, std::string_view>, int> = 0>
    [[nodiscard]] stream<thinkthen::choose> choose_each(const thinkthen::choose& question, const std::vector<T>& records, std::optional<std::string_view> name = {}) const { return choose_each(question, records.begin(), records.end(), name); }
    /// Judge an iterator range of strings or owned records in input order.
    template<class Iterator>
    [[nodiscard]] stream<thinkthen::choose> choose_each(const thinkthen::choose& question, Iterator first, Iterator last, std::optional<std::string_view> name = {}) const {
        return start(question, detail::own_records(first, last), name);
    }
    /// Judge one record with a tag question. Blank field throws record.empty.
    [[nodiscard]] row<thinkthen::tag> tag(const thinkthen::tag& question, record input, std::optional<std::string_view> name = {}) const { return one(question, input, name); }
    /// Judge one text record; strings are copied before the call returns.
    [[nodiscard]] row<thinkthen::tag> tag(const thinkthen::tag& question, std::string_view input, std::optional<std::string_view> name = {}) const { return one(question, text{input}, name); }
    /// Stream every tag row in input order, including not-sure and failed rows.
    [[nodiscard]] stream<thinkthen::tag> tag_each(const thinkthen::tag& question, record_source source, std::optional<std::string_view> name = {}) const { return start(question, std::move(source), name); }
    /// Judge owned records; the returned stream owns its inputs.
    [[nodiscard]] stream<thinkthen::tag> tag_each(const thinkthen::tag& question, std::vector<owned_record> records, std::optional<std::string_view> name = {}) const { return start(question, std::move(records), name); }
    /// Judge a collection of strings, copying each text into the stream.
    template<class T, std::enable_if_t<std::is_convertible_v<T, std::string_view>, int> = 0>
    [[nodiscard]] stream<thinkthen::tag> tag_each(const thinkthen::tag& question, const std::vector<T>& records, std::optional<std::string_view> name = {}) const { return tag_each(question, records.begin(), records.end(), name); }
    /// Judge an iterator range of strings or owned records in input order.
    template<class Iterator>
    [[nodiscard]] stream<thinkthen::tag> tag_each(const thinkthen::tag& question, Iterator first, Iterator last, std::optional<std::string_view> name = {}) const {
        return start(question, detail::own_records(first, last), name);
    }
    /// Judge one record with a score question. Blank field throws record.empty.
    [[nodiscard]] row<thinkthen::score> score(const thinkthen::score& question, record input, std::optional<std::string_view> name = {}) const { return one(question, input, name); }
    /// Judge one text record; strings are copied before the call returns.
    [[nodiscard]] row<thinkthen::score> score(const thinkthen::score& question, std::string_view input, std::optional<std::string_view> name = {}) const { return one(question, text{input}, name); }
    /// Stream every score row in input order, including not-sure and failed rows.
    [[nodiscard]] stream<thinkthen::score> score_each(const thinkthen::score& question, record_source source, std::optional<std::string_view> name = {}) const { return start(question, std::move(source), name); }
    /// Judge owned records; the returned stream owns its inputs.
    [[nodiscard]] stream<thinkthen::score> score_each(const thinkthen::score& question, std::vector<owned_record> records, std::optional<std::string_view> name = {}) const { return start(question, std::move(records), name); }
    /// Judge a collection of strings, copying each text into the stream.
    template<class T, std::enable_if_t<std::is_convertible_v<T, std::string_view>, int> = 0>
    [[nodiscard]] stream<thinkthen::score> score_each(const thinkthen::score& question, const std::vector<T>& records, std::optional<std::string_view> name = {}) const { return score_each(question, records.begin(), records.end(), name); }
    /// Judge an iterator range of strings or owned records in input order.
    template<class Iterator>
    [[nodiscard]] stream<thinkthen::score> score_each(const thinkthen::score& question, Iterator first, Iterator last, std::optional<std::string_view> name = {}) const {
        return start(question, detail::own_records(first, last), name);
    }
    /// Judge one record with a filter question. Blank field throws record.empty.
    [[nodiscard]] row<thinkthen::filter> filter(const thinkthen::filter& question, record input, std::optional<std::string_view> name = {}) const { return one(question, input, name); }
    /// Judge one text record; strings are copied before the call returns.
    [[nodiscard]] row<thinkthen::filter> filter(const thinkthen::filter& question, std::string_view input, std::optional<std::string_view> name = {}) const { return one(question, text{input}, name); }
    /// Stream every filter row in input order, including not-sure and failed rows.
    [[nodiscard]] stream<thinkthen::filter> filter_rows_each(const thinkthen::filter& question, record_source source, std::optional<std::string_view> name = {}) const { return start(question, std::move(source), name); }
    /// Judge owned records; the returned stream owns its inputs.
    [[nodiscard]] stream<thinkthen::filter> filter_rows_each(const thinkthen::filter& question, std::vector<owned_record> records, std::optional<std::string_view> name = {}) const { return start(question, std::move(records), name); }
    /// Judge a collection of strings, copying each text into the stream.
    template<class T, std::enable_if_t<std::is_convertible_v<T, std::string_view>, int> = 0>
    [[nodiscard]] stream<thinkthen::filter> filter_rows_each(const thinkthen::filter& question, const std::vector<T>& records, std::optional<std::string_view> name = {}) const { return filter_rows_each(question, records.begin(), records.end(), name); }
    /// Judge an iterator range of strings or owned records in input order.
    template<class Iterator>
    [[nodiscard]] stream<thinkthen::filter> filter_rows_each(const thinkthen::filter& question, Iterator first, Iterator last, std::optional<std::string_view> name = {}) const {
        return start(question, detail::own_records(first, last), name);
    }
    /// Judge one record with a rank question. Blank field throws record.empty.
    [[nodiscard]] row<thinkthen::rank> rank(const thinkthen::rank& question, record input, std::optional<std::string_view> name = {}) const { return one(question, input, name); }
    /// Judge one text record; strings are copied before the call returns.
    [[nodiscard]] row<thinkthen::rank> rank(const thinkthen::rank& question, std::string_view input, std::optional<std::string_view> name = {}) const { return one(question, text{input}, name); }
    /// Stream successful rows in rank order, followed by failed rows in input order.
    [[nodiscard]] stream<thinkthen::rank> rank_each(const thinkthen::rank& question, record_source source, std::optional<std::string_view> name = {}) const { return start(question, std::move(source), name); }
    /// Judge owned records; the returned stream owns its inputs.
    [[nodiscard]] stream<thinkthen::rank> rank_each(const thinkthen::rank& question, std::vector<owned_record> records, std::optional<std::string_view> name = {}) const { return start(question, std::move(records), name); }
    /// Judge a collection of strings, copying each text into the stream.
    template<class T, std::enable_if_t<std::is_convertible_v<T, std::string_view>, int> = 0>
    [[nodiscard]] stream<thinkthen::rank> rank_each(const thinkthen::rank& question, const std::vector<T>& records, std::optional<std::string_view> name = {}) const { return rank_each(question, records.begin(), records.end(), name); }
    /// Judge an iterator range, yielding rank order while preserving input positions.
    template<class Iterator>
    [[nodiscard]] stream<thinkthen::rank> rank_each(const thinkthen::rank& question, Iterator first, Iterator last, std::optional<std::string_view> name = {}) const {
        return start(question, detail::own_records(first, last), name);
    }
    /// Judge one record with a find question. Blank field throws record.empty.
    [[nodiscard]] row<thinkthen::find> find(const thinkthen::find& question, record input, std::optional<std::string_view> name = {}) const { return one(question, input, name); }
    /// Judge one text record; strings are copied before the call returns.
    [[nodiscard]] row<thinkthen::find> find(const thinkthen::find& question, std::string_view input, std::optional<std::string_view> name = {}) const { return one(question, text{input}, name); }
    /// Stream every find row in input order, including not-sure and failed rows.
    [[nodiscard]] stream<thinkthen::find> find_each(const thinkthen::find& question, record_source source, std::optional<std::string_view> name = {}) const { return start(question, std::move(source), name); }
    /// Judge owned records; the returned stream owns its inputs.
    [[nodiscard]] stream<thinkthen::find> find_each(const thinkthen::find& question, std::vector<owned_record> records, std::optional<std::string_view> name = {}) const { return start(question, std::move(records), name); }
    /// Judge a collection of strings, copying each text into the stream.
    template<class T, std::enable_if_t<std::is_convertible_v<T, std::string_view>, int> = 0>
    [[nodiscard]] stream<thinkthen::find> find_each(const thinkthen::find& question, const std::vector<T>& records, std::optional<std::string_view> name = {}) const { return find_each(question, records.begin(), records.end(), name); }
    /// Judge an iterator range of strings or owned records in input order.
    template<class Iterator>
    [[nodiscard]] stream<thinkthen::find> find_each(const thinkthen::find& question, Iterator first, Iterator last, std::optional<std::string_view> name = {}) const {
        return start(question, detail::own_records(first, last), name);
    }
    /// Judge one record with a recognize question. Blank field throws record.empty.
    [[nodiscard]] row<thinkthen::recognize> recognize(const thinkthen::recognize& question, record input, std::optional<std::string_view> name = {}) const { return one(question, input, name); }
    /// Judge one text record; strings are copied before the call returns.
    [[nodiscard]] row<thinkthen::recognize> recognize(const thinkthen::recognize& question, std::string_view input, std::optional<std::string_view> name = {}) const { return one(question, text{input}, name); }
    /// Stream every recognize row in input order, including not-sure and failed rows.
    [[nodiscard]] stream<thinkthen::recognize> recognize_each(const thinkthen::recognize& question, record_source source, std::optional<std::string_view> name = {}) const { return start(question, std::move(source), name); }
    /// Judge owned records; the returned stream owns its inputs.
    [[nodiscard]] stream<thinkthen::recognize> recognize_each(const thinkthen::recognize& question, std::vector<owned_record> records, std::optional<std::string_view> name = {}) const { return start(question, std::move(records), name); }
    /// Judge a collection of strings, copying each text into the stream.
    template<class T, std::enable_if_t<std::is_convertible_v<T, std::string_view>, int> = 0>
    [[nodiscard]] stream<thinkthen::recognize> recognize_each(const thinkthen::recognize& question, const std::vector<T>& records, std::optional<std::string_view> name = {}) const { return recognize_each(question, records.begin(), records.end(), name); }
    /// Judge an iterator range of strings or owned records in input order.
    template<class Iterator>
    [[nodiscard]] stream<thinkthen::recognize> recognize_each(const thinkthen::recognize& question, Iterator first, Iterator last, std::optional<std::string_view> name = {}) const {
        return start(question, detail::own_records(first, last), name);
    }
    /// Judge one record with a relate question. Blank field throws record.empty.
    [[nodiscard]] row<thinkthen::relate> relate(const thinkthen::relate& question, record input, std::optional<std::string_view> name = {}) const { return one(question, input, name); }
    /// Judge one text record; strings are copied before the call returns.
    [[nodiscard]] row<thinkthen::relate> relate(const thinkthen::relate& question, std::string_view input, std::optional<std::string_view> name = {}) const { return one(question, text{input}, name); }
    /// Stream every relate row in input order, including not-sure and failed rows.
    [[nodiscard]] stream<thinkthen::relate> relate_each(const thinkthen::relate& question, record_source source, std::optional<std::string_view> name = {}) const { return start(question, std::move(source), name); }
    /// Judge owned records; the returned stream owns its inputs.
    [[nodiscard]] stream<thinkthen::relate> relate_each(const thinkthen::relate& question, std::vector<owned_record> records, std::optional<std::string_view> name = {}) const { return start(question, std::move(records), name); }
    /// Judge a collection of strings, copying each text into the stream.
    template<class T, std::enable_if_t<std::is_convertible_v<T, std::string_view>, int> = 0>
    [[nodiscard]] stream<thinkthen::relate> relate_each(const thinkthen::relate& question, const std::vector<T>& records, std::optional<std::string_view> name = {}) const { return relate_each(question, records.begin(), records.end(), name); }
    /// Judge an iterator range of strings or owned records in input order.
    template<class Iterator>
    [[nodiscard]] stream<thinkthen::relate> relate_each(const thinkthen::relate& question, Iterator first, Iterator last, std::optional<std::string_view> name = {}) const {
        return start(question, detail::own_records(first, last), name);
    }
    /// Judge one record with a annotate question. Blank field throws record.empty.
    [[nodiscard]] row<thinkthen::annotate> annotate(const thinkthen::annotate& question, record input, std::optional<std::string_view> name = {}) const { return one(question, input, name); }
    /// Judge one text record; strings are copied before the call returns.
    [[nodiscard]] row<thinkthen::annotate> annotate(const thinkthen::annotate& question, std::string_view input, std::optional<std::string_view> name = {}) const { return one(question, text{input}, name); }
    /// Stream every annotate row in input order, including not-sure and failed rows.
    [[nodiscard]] stream<thinkthen::annotate> annotate_each(const thinkthen::annotate& question, record_source source, std::optional<std::string_view> name = {}) const { return start(question, std::move(source), name); }
    /// Judge owned records; the returned stream owns its inputs.
    [[nodiscard]] stream<thinkthen::annotate> annotate_each(const thinkthen::annotate& question, std::vector<owned_record> records, std::optional<std::string_view> name = {}) const { return start(question, std::move(records), name); }
    /// Judge a collection of strings, copying each text into the stream.
    template<class T, std::enable_if_t<std::is_convertible_v<T, std::string_view>, int> = 0>
    [[nodiscard]] stream<thinkthen::annotate> annotate_each(const thinkthen::annotate& question, const std::vector<T>& records, std::optional<std::string_view> name = {}) const { return annotate_each(question, records.begin(), records.end(), name); }
    /// Judge an iterator range of strings or owned records in input order.
    template<class Iterator>
    [[nodiscard]] stream<thinkthen::annotate> annotate_each(const thinkthen::annotate& question, Iterator first, Iterator last, std::optional<std::string_view> name = {}) const {
        return start(question, detail::own_records(first, last), name);
    }
    /// Stream kept filter records, then report all failed inputs in one error.
    [[nodiscard]] kept_stream filter_each(const thinkthen::filter& question, record_source source, std::optional<std::string_view> name = {}) const { return kept_stream(filter_rows_each(question, std::move(source), name)); }
    /// Filter an owned collection, retaining the original encoding of each kept input.
    [[nodiscard]] kept_stream filter_each(const thinkthen::filter& question, std::vector<owned_record> records, std::optional<std::string_view> name = {}) const { return kept_stream(filter_rows_each(question, std::move(records), name)); }
    /// Filter a collection of strings, copying the inputs.
    template<class T, std::enable_if_t<std::is_convertible_v<T, std::string_view>, int> = 0>
    [[nodiscard]] kept_stream filter_each(const thinkthen::filter& question, const std::vector<T>& records, std::optional<std::string_view> name = {}) const { return kept_stream(filter_rows_each(question, records, name)); }
    /// Filter an iterator range of strings or owned records.
    template<class Iterator>
    [[nodiscard]] kept_stream filter_each(const thinkthen::filter& question, Iterator first, Iterator last, std::optional<std::string_view> name = {}) const { return kept_stream(filter_rows_each(question, first, last, name)); }
};
}}
#endif
