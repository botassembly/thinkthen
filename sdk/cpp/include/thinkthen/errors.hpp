#ifndef THINKTHEN_CPP_ERRORS_HPP
#define THINKTHEN_CPP_ERRORS_HPP
#include <thinkthen.h>
#include <thinkthen/codes.hpp>
#include <cstdint>
#include <memory>
#include <optional>
#include <stdexcept>
#include <string>
#include <string_view>
#include <system_error>
#include <type_traits>
#include <vector>
namespace thinkthen { inline namespace v0_2 {
namespace detail {
[[nodiscard]] inline const char* code_name(std::int32_t code) noexcept {
    if (!thinkthen_abi_compatible(THINKTHEN_ABI_VERSION, thinkthen_abi_version()))
        return code == THINKTHEN_CODE_USAGE_ABI ? THINKTHEN_ABI_ERROR_CODE : nullptr;
    return thinkthen_code_name(code);
}
}
/// The standard category for every engine error code.
[[nodiscard]] const std::error_category& category() noexcept;
/// A native failure, with the engine's message and standard error code.
class error : public std::system_error {
    std::string message_;
    std::optional<std::uint16_t> http_status_;
public:
    /// Copy an engine failure and its optional HTTP status.
    error(errc code, std::string message, std::optional<std::uint16_t> http = {})
        : std::system_error(std::error_code(static_cast<int>(code), category()), message),
          message_(this->code().message() + ": " + message), http_status_(http) {}
    /// Read the code name followed by the engine message.
    [[nodiscard]] const char* what() const noexcept override { return message_.c_str(); }
    /// Read the backend HTTP status, if supplied.
    [[nodiscard]] std::optional<std::uint16_t> http_status() const noexcept { return http_status_; }
    /// Read the numeric code, including a code unknown to these headers.
    [[nodiscard]] std::int32_t raw_code() const noexcept { return code().value(); }
    /// Read the engine code family, or an empty view when its family is unavailable.
    [[nodiscard]] std::string_view family() const noexcept {
        if (!thinkthen_abi_compatible(THINKTHEN_ABI_VERSION, thinkthen_abi_version())) {
            const auto* name = detail::code_name(raw_code());
            const std::string_view code = name ? name : "";
            return code.substr(0, code.find('.'));
        }
        const auto* family = thinkthen_code_family(raw_code());
        return family ? std::string_view{family} : std::string_view{};
    }
    /// Test whether this header knows the code.
    [[nodiscard]] bool unknown_code() const noexcept { return !detail::known_code(raw_code()); }
};
/// A marker caught for either way a result has no answer.
class no_answer {};
/// A record failure, caught both as an engine error and as no answer.
class failed : public error, public no_answer {
public:
    /// Copy the admitted native error.
    explicit failed(const error& failure) : error(failure) {}
};
/// An aggregate filter failure retaining the affected caller records.
template<class T> class filter_error;
/// Reading a not-sure answer throws this exception, which is not an engine failure.
class unsure : public std::exception, public no_answer {
    std::optional<double> probability_;
public:
    /// Retain the probability supplied by the not-sure row.
    explicit unsure(std::optional<double> probability) noexcept : probability_(probability) {}
    /// Describe the not-sure state without inventing an engine error code.
    [[nodiscard]] const char* what() const noexcept override { return "the answer is not sure"; }
    /// Read the probability that accompanied the not-sure answer.
    [[nodiscard]] std::optional<double> probability() const noexcept { return probability_; }
};
namespace detail {
[[noreturn]] inline void incompatible() { throw error(errc::usage_abi, THINKTHEN_ABI_ERROR_MESSAGE); }
inline void check_abi() {
    if (!thinkthen_abi_compatible(THINKTHEN_ABI_VERSION, thinkthen_abi_version())) incompatible();
}
}
[[nodiscard]] inline const std::error_category& category() noexcept {
    struct codes final : std::error_category {
        [[nodiscard]] const char* name() const noexcept override { return "thinkthen"; }
        [[nodiscard]] std::string message(int code) const override {
            const auto* text = detail::code_name(code);
            return text ? text : THINKTHEN_ABI_ERROR_MESSAGE;
        }
    };
    static codes value;
    return value;
}
/// Convert a registry code to the standard error-code representation.
[[nodiscard]] inline std::error_code make_error_code(errc code) noexcept { return {static_cast<int>(code), category()}; }
namespace detail {
[[nodiscard]] inline const char* bytes(std::string_view text) { return text.data(); }
inline thinkthen_text view(std::string_view text) { return {bytes(text), text.size()}; }
[[nodiscard]] inline std::string copy(thinkthen_text text) {
    return text.len ? std::string(reinterpret_cast<const char*>(text.data), text.len) : std::string{};
}
inline errc code(std::int32_t value) {
    return static_cast<errc>(value);
}
inline error failure(const thinkthen_error& value) {
    return error(code(value.code), copy({reinterpret_cast<const char *>(value.message.data), value.message.len}),
        value.http_status ? std::optional<std::uint16_t>{value.http_status} : std::nullopt);
}
struct diagnostic {
    thinkthen_error value{};
    diagnostic() { value.struct_size = sizeof value; }
    ~diagnostic() { thinkthen_error_free(&value); }
    diagnostic(const diagnostic&) = delete;
    diagnostic& operator=(const diagnostic&) = delete;
};
inline void check(std::int32_t result, const diagnostic& d) {
    if (result < 0) throw failure(d.value);
}
template<class F, class... Args> inline void invoke(F f, Args... args) {
    check_abi();
    diagnostic d;
    check(f(args..., &d.value), d);
}
template<class T, class F, class... Args> inline T get(F f, Args... args) {
    T out{};
    invoke(f, args..., &out);
    return out;
}
template<class T, void (*Free)(T*)> using owner = std::unique_ptr<T, decltype(Free)>;
using question_owner = owner<thinkthen_question, thinkthen_question_free>;
inline question_owner clone(const thinkthen_question* source) {
    return {get<thinkthen_question*>(thinkthen_question_clone, source), thinkthen_question_free};
}
struct native_row {
    thinkthen_row value{};
    native_row() { value.struct_size = sizeof value; }
    native_row(native_row&& other) noexcept : value(other.value) {
        other.value = {}; other.value.struct_size = sizeof other.value;
    }
    native_row& operator=(native_row&& other) noexcept {
        if (this != &other) {
            thinkthen_row_free(&value); value = other.value;
            other.value = {}; other.value.struct_size = sizeof other.value;
        }
        return *this;
    }
    ~native_row() { thinkthen_row_free(&value); }
    native_row(const native_row&) = delete;
    native_row& operator=(const native_row&) = delete;
};
}
}}
namespace std { template<> struct is_error_code_enum<thinkthen::errc> : true_type {}; }
#endif
