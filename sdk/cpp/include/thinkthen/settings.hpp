#ifndef THINKTHEN_CPP_SETTINGS_HPP
#define THINKTHEN_CPP_SETTINGS_HPP
#include <thinkthen/errors.hpp>
#include <chrono>
#include <ostream>
namespace thinkthen { inline namespace v0_2 {
/// A named backend supported by the engine.
enum class backend : std::uint32_t {
    /// systemone: The System One service; needs an API key.
    systemone = THINKTHEN_BACKEND_SYSTEMONE,
    /// openai: An OpenAI-compatible service; needs an API key.
    openai = THINKTHEN_BACKEND_OPENAI,
    /// stub: A local stand-in that answers without a network or a key, for tests.
    stub = THINKTHEN_BACKEND_STUB,
};
/// Print the backend name used in configuration.
inline std::ostream& operator<<(std::ostream& out, backend value) {
    switch (value) { case backend::systemone: return out << "systemone"; case backend::openai: return out << "openai"; case backend::stub: return out << "stub"; default: return out << "unknown"; }
}
/// An owned secret whose display is always redacted.
class [[nodiscard]] api_key {
    std::shared_ptr<const thinkthen_settings> value_;
    friend class client;
public:
    /// Leave the engine to read its configured key source.
    api_key() = default;
    /// Copy a key without exposing it through a string accessor.
    explicit api_key(std::string_view value) {
        detail::owner<thinkthen_settings, thinkthen_settings_free> owner(
            detail::get<thinkthen_settings*>(thinkthen_settings_new), thinkthen_settings_free);
        detail::invoke(thinkthen_settings_set_api_key, owner.get(), detail::bytes(value), value.size());
        value_ = {owner.release(), thinkthen_settings_free};
    }
    /// Return a printable replacement for the key.
    [[nodiscard]] std::string_view redacted() const noexcept { return "[REDACTED]"; }
};
/// Retry settings admitted by the engine when a client opens.
struct retry_policy {
    /// Retries after the first attempt.
    std::uint32_t max_retries;
    /// Initial delay before a retry.
    std::chrono::milliseconds initial_backoff;
    /// Timeout for each backend attempt.
    std::chrono::milliseconds attempt_timeout;
};
/// Explicit local response-store mode.
enum class response_mode : std::uint32_t {
    cache = THINKTHEN_RESPONSE_CACHE,
    record = THINKTHEN_RESPONSE_RECORD,
    replay = THINKTHEN_RESPONSE_REPLAY,
};
/// Private local storage of input/output text. Optional limits use shared defaults.
struct response_store {
    std::string path;
    response_mode mode;
    std::string namespace_name;
    std::string model_generation;
    std::optional<std::uint64_t> max_bytes, max_entries, max_age, busy_timeout_ms;
};
/// Client configuration. Absent settings use the engine’s defaults and environment.
struct settings {
    /// Override THINKTHEN_BACKEND when supplied.
    std::optional<thinkthen::backend> backend;
    /// Override the backend model.
    std::optional<std::string> model;
    /// Override the backend address.
    std::optional<std::string> url;
    /// Explicit key, otherwise the backend reads its own key source.
    thinkthen::api_key api_key;
    /// Override the engine batch size.
    std::optional<std::uint64_t> batch;
    /// Client send allowance; absent is unlimited and zero forbids sends.
    std::optional<std::uint64_t> max_requests;
    /// Explicit local storage; absent retains no responses.
    std::optional<thinkthen::response_store> response_store;
    /// Override the engine retry policy.
    std::optional<retry_policy> retry;
};
}}
#endif
