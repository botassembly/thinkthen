#ifndef THINKTHEN_CPP_HPP
#define THINKTHEN_CPP_HPP

#include <thinkthen.h>
#include <thinkthen/answers.hpp>
#include <functional>
#include <iterator>
#include <memory>
#include <optional>
#include <stdexcept>
#include <string>
#include <utility>
#include <vector>

namespace thinkthen {
using records = std::vector<json>;
using record_source = std::function<std::optional<json>()>;

class error : public std::runtime_error {
    std::string code_;
public:
    error(std::string code, std::string message)
        : std::runtime_error(std::move(message)), code_(std::move(code)) {}
    const std::string& code() const noexcept { return code_; }
};

namespace detail {
struct buffer {
    thinkthen_buffer value{};
    ~buffer() { thinkthen_buffer_free(&value); }
    buffer() = default;
    buffer(const buffer&) = delete;
    buffer& operator=(const buffer&) = delete;
};
inline json parse(const thinkthen_buffer& value) {
    return json::parse(value.data, value.data + value.len);
}
inline error failure(const json& value) {
    return error(value.at("code").get<std::string>(),
                 value.at("message").get<std::string>());
}
inline void check(int32_t result, const buffer& diagnostic) {
    if (result == THINKTHEN_ERROR) throw failure(parse(diagnostic.value));
}
struct native_row {
    thinkthen_row value{};
    ~native_row() { thinkthen_row_free(&value); }
    native_row() = default;
    native_row(const native_row&) = delete;
    native_row& operator=(const native_row&) = delete;
};
struct call_state {
    thinkthen_call* call;
    explicit call_state(thinkthen_call* value) : call(value) {}
    ~call_state() { thinkthen_free(call); }
    call_state(const call_state&) = delete;
    call_state& operator=(const call_state&) = delete;
};
}

template<class Row> class basic_stream;
template<class Answer> class typed_row;

class row {
    struct data {
        json fields;
        std::optional<std::string> detail;
        std::optional<thinkthen::error> failure;
        std::size_t position;
        bool kept;
        explicit data(const thinkthen_row& value)
            : fields(detail::parse(value.fields)), position(value.position),
              kept(value.kept != 0) {
            if (value.detail.len != 0)
                detail.emplace(reinterpret_cast<const char*>(value.detail.data), value.detail.len);
            if (fields.contains("error")) failure.emplace(detail::failure(fields.at("error")));
        }
    };
    std::shared_ptr<const data> data_;
    template<class Answer> friend class typed_row;
    explicit row(const thinkthen_row& value) : data_(std::make_shared<data>(value)) {}
    template<class Row> friend class basic_stream;
    const json* member(const char* name) const {
        auto found = data_->fields.find(name);
        return found == data_->fields.end() ? nullptr : &*found;
    }
public:
    thinkthen::status status() const {
        const auto& value = data_->fields.at("status");
        if (value == "answered") return thinkthen::status::answered;
        if (value == "unsure") return thinkthen::status::unsure;
        return thinkthen::status::failed;
    }
    const json* id() const { return member("id"); }
    const json* answer() const { return member("answer"); }
    const json* raw_answer() const { return member("answer"); }
    std::optional<double> probability() const {
        const auto* value = member("p");
        return value ? std::optional<double>(value->get<double>()) : std::nullopt;
    }
    const thinkthen::error* error() const noexcept {
        return data_->failure ? &*data_->failure : nullptr;
    }
    std::optional<json> detail() const {
        return data_->detail ? std::optional<json>(json::parse(*data_->detail)) : std::nullopt;
    }
    std::size_t position() const noexcept { return data_->position; }
    bool kept() const noexcept { return data_->kept; }
};

class cancel_handle {
    std::weak_ptr<detail::call_state> state_;
    explicit cancel_handle(const std::shared_ptr<detail::call_state>& state) : state_(state) {}
    template<class Row> friend class basic_stream;
public:
    void request_cancel() const noexcept {
        if (auto state = state_.lock()) thinkthen_cancel(state->call);
    }
};

template<class Answer> class typed_row : public row {
    template<class Row> friend class basic_stream;
    explicit typed_row(const thinkthen_row& value) : row(value) {}
public:
    std::optional<Answer> answer() const {
        const auto* value = raw_answer();
        return !value || value->is_null() ? std::nullopt : std::optional<Answer>(value->template get<Answer>());
    }
};

template<class Row> class basic_stream {
    std::shared_ptr<detail::call_state> state_;
    record_source source_;
    bool started_ = false;
    basic_stream(thinkthen_call* call, record_source source) : source_(std::move(source)) {
        // Preserve ownership if allocating the shared control block fails.
        std::unique_ptr<thinkthen_call, decltype(&thinkthen_free)> owner(call, thinkthen_free);
        state_ = std::make_shared<detail::call_state>(call);
        owner.release();
    }
    friend class client;
    std::optional<Row> pull() {
        if (!state_) return std::nullopt;
        // Keep native free away from every active driver operation and cancel.
        auto state = state_;
        try {
            for (;;) {
                detail::native_row result;
                detail::buffer diagnostic;
                auto event = thinkthen_next(state->call, &result.value, &diagnostic.value);
                detail::check(event, diagnostic);
                if (event == THINKTHEN_OK) return Row(result.value);
                if (event == THINKTHEN_END) { close(); return std::nullopt; }
                auto record = source_ ? source_() : std::nullopt;
                if (!record) {
                    thinkthen_end_input(state->call);
                    source_ = {};
                } else if (record->is_string()) {
                    const auto& text = record->template get_ref<const std::string&>();
                    detail::check(thinkthen_send_text(state->call,
                        reinterpret_cast<const uint8_t*>(text.data()), text.size(),
                        &diagnostic.value), diagnostic);
                } else {
                    auto bytes = record->dump();
                    detail::check(thinkthen_send_json(state->call,
                        reinterpret_cast<const uint8_t*>(bytes.data()), bytes.size(),
                        &diagnostic.value), diagnostic);
                }
            }
        } catch (...) { close(); throw; }
    }
public:
    basic_stream(const basic_stream&) = delete;
    basic_stream& operator=(const basic_stream&) = delete;
    basic_stream(basic_stream&&) noexcept = default;
    basic_stream& operator=(basic_stream&&) noexcept = default;
    ~basic_stream() { close(); }
    void close() noexcept {
        if (state_) thinkthen_cancel(state_->call);
        state_.reset();
        source_ = {};
    }
    thinkthen::cancel_handle cancel_handle() const { return thinkthen::cancel_handle(state_); }
    std::optional<Row> next() { return pull(); }
    class iterator {
        basic_stream* stream_ = nullptr;
        std::optional<Row> current_;
        explicit iterator(basic_stream* value) : stream_(value), current_(value->pull()) {
            if (!current_) stream_ = nullptr;
        }
        friend class basic_stream;
    public:
        using iterator_category = std::input_iterator_tag;
        using value_type = Row;
        using difference_type = std::ptrdiff_t;
        using reference = const Row&;
        using pointer = const Row*;
        iterator() = default;
        reference operator*() const { return *current_; }
        pointer operator->() const { return &*current_; }
        iterator& operator++() {
            current_ = stream_->pull();
            if (!current_) stream_ = nullptr;
            return *this;
        }
        iterator operator++(int) { auto old = *this; ++*this; return old; }
        friend bool operator==(const iterator& left, const iterator& right) {
            return left.stream_ == right.stream_;
        }
        friend bool operator!=(const iterator& left, const iterator& right) { return !(left == right); }
    };
    iterator begin() {
        if (started_) throw std::logic_error("a stream can be iterated only once");
        started_ = true;
        return state_ ? iterator(this) : iterator();
    }
    iterator end() const { return {}; }
};

using stream = basic_stream<row>;
template<class Answer> using typed_stream = basic_stream<typed_row<Answer>>;
struct tagged_question { json value; };

class client {
    struct deleter { void operator()(thinkthen_client* value) const { thinkthen_client_free(value); } };
    std::unique_ptr<thinkthen_client, deleter> value_;
    template<class Row = row> basic_stream<Row> start(const json& request, record_source source) const {
        auto bytes = request.dump();
        thinkthen_call* call = nullptr;
        detail::buffer diagnostic;
        detail::check(thinkthen_call_open(value_.get(),
            reinterpret_cast<const uint8_t*>(bytes.data()), bytes.size(),
            THINKTHEN_UTF8, &call, &diagnostic.value), diagnostic);
        return basic_stream<Row>(call, std::move(source));
    }
    template<class Answer> typed_stream<Answer> named(const char* function, tagged_question question,
                 record_source input, json options) const {
        return start<typed_row<Answer>>({{"function", function},
                      {"question", std::move(question.value)},
                      {"options", std::move(options)}}, std::move(input));
    }
    static record_source source(records input) {
        return [input = std::move(input), index = std::size_t{}]() mutable -> std::optional<json> {
            if (index == input.size()) return std::nullopt;
            return std::move(input[index++]);
        };
    }
public:
    explicit client(const json& settings = json::object()) {
        auto bytes = settings.dump();
        thinkthen_client* value = nullptr;
        detail::buffer diagnostic;
        detail::check(thinkthen_client_open(reinterpret_cast<const uint8_t*>(bytes.data()),
            bytes.size(), &value, &diagnostic.value), diagnostic);
        value_.reset(value);
    }
    client(const client&) = delete;
    client& operator=(const client&) = delete;
    client(client&&) noexcept = default;
    client& operator=(client&&) noexcept = default;
    stream open(const json& request) const {
        auto result = start(request, {});
        thinkthen_end_input(result.state_->call);
        return result;
    }
#define THINKTHEN_CPP_FUNCTION(name, question_type, answer_type) \
    typed_stream<answer_type> name(question_type question, records input, json options = json::object()) const { \
        return name(tagged_question{json{{"type", "inline"}, {"value", std::move(question)}}}, source(std::move(input)), std::move(options)); \
    } \
    typed_stream<answer_type> name(question_type question, record_source input, json options = json::object()) const { \
        return name(tagged_question{json{{"type", "inline"}, {"value", std::move(question)}}}, std::move(input), std::move(options)); \
    } \
    typed_stream<answer_type> name(tagged_question question, records input, json options = json::object()) const { \
        return name(std::move(question), source(std::move(input)), std::move(options)); \
    } \
    typed_stream<answer_type> name(tagged_question question, record_source input, json options = json::object()) const { \
        return named<answer_type>(#name, std::move(question), std::move(input), std::move(options)); \
    }
    THINKTHEN_CPP_FUNCTION(decide, std::string, bool)
    THINKTHEN_CPP_FUNCTION(choose, std::string, std::string)
    THINKTHEN_CPP_FUNCTION(tag, std::string, std::vector<std::string>)
    THINKTHEN_CPP_FUNCTION(score, std::string, double)
    THINKTHEN_CPP_FUNCTION(filter, std::string, bool)
    THINKTHEN_CPP_FUNCTION(rank, std::string, std::size_t)
    THINKTHEN_CPP_FUNCTION(find, std::string, span)
    THINKTHEN_CPP_FUNCTION(annotate, json, annotations)
    THINKTHEN_CPP_FUNCTION(recognize, std::vector<std::string>, std::vector<thinkthen::name>)
    THINKTHEN_CPP_FUNCTION(relate, json, std::vector<relation>)
#undef THINKTHEN_CPP_FUNCTION
};
}
#endif
