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
namespace thinkthen { inline namespace v0_2 {
class error : public std::runtime_error {
    errc code_;
    std::optional<std::uint16_t> http_status_;
public:
    error(errc code, std::string message, std::optional<std::uint16_t> http = {})
        : std::runtime_error(std::move(message)), code_(code), http_status_(http) {}
    errc code() const noexcept { return code_; }
    std::optional<std::uint16_t> http_status() const noexcept { return http_status_; }
    std::int32_t raw_code() const noexcept { return static_cast<std::int32_t>(code_); }
    std::string_view family() const noexcept {
        const auto* family = thinkthen_code_family(raw_code());
        return family ? std::string_view{family} : std::string_view{};
    }
    bool unknown_code() const noexcept { return !detail::known_code(raw_code()); }
};
namespace detail {
[[noreturn]] inline void incompatible() { throw error(errc::usage_abi, THINKTHEN_ABI_ERROR_MESSAGE); }
inline void check_abi() {
    if (!thinkthen_abi_compatible(THINKTHEN_ABI_VERSION, thinkthen_abi_version())) incompatible();
}
}
inline const std::error_category& category() noexcept {
    struct codes final : std::error_category {
        const char* name() const noexcept override { return "thinkthen"; }
        std::string message(int code) const override {
            detail::check_abi();
            const auto* text = thinkthen_code_name(code);
            return text ? text : THINKTHEN_ABI_ERROR_MESSAGE;
        }
    };
    static codes value;
    return value;
}
inline std::error_code make_error_code(errc code) noexcept { return {static_cast<int>(code), category()}; }
namespace detail {
inline const std::uint8_t* bytes(std::string_view text) { return reinterpret_cast<const std::uint8_t*>(text.data()); }
inline thinkthen_text view(std::string_view text) { return {bytes(text), text.size()}; }
inline std::string copy(thinkthen_text text) {
    return text.len ? std::string(reinterpret_cast<const char*>(text.data), text.len) : std::string{};
}
inline errc code(std::int32_t value) {
    return static_cast<errc>(value);
}
inline error failure(const thinkthen_error& value) {
    return error(code(value.code), copy({value.message.data, value.message.len}),
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
    if (result == THINKTHEN_ERROR) throw failure(d.value);
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
    ~native_row() { thinkthen_row_free(&value); }
    native_row(const native_row&) = delete;
    native_row& operator=(const native_row&) = delete;
};
}
}}
namespace std { template<> struct is_error_code_enum<thinkthen::errc> : true_type {}; }
#endif
