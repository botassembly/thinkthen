#ifndef THINKTHEN_CPP_SETTINGS_HPP
#define THINKTHEN_CPP_SETTINGS_HPP
#include <thinkthen/errors.hpp>
#include <chrono>
namespace thinkthen { inline namespace v0_2 {
enum class backend : std::uint32_t {
    systemone = THINKTHEN_BACKEND_SYSTEMONE, openai = THINKTHEN_BACKEND_OPENAI, stub = THINKTHEN_BACKEND_STUB
};
class api_key {
    std::shared_ptr<const thinkthen_settings> value_;
    friend class client;
public:
    api_key() = default;
    explicit api_key(std::string_view value) {
        detail::owner<thinkthen_settings, thinkthen_settings_free> owner(
            detail::get<thinkthen_settings*>(thinkthen_settings_new), thinkthen_settings_free);
        detail::invoke(thinkthen_settings_set_api_key, owner.get(), detail::bytes(value), value.size());
        value_ = {owner.release(), thinkthen_settings_free};
    }
    std::string_view redacted() const noexcept { return "[REDACTED]"; }
};
struct retry_policy {
    std::uint32_t max_retries;
    std::chrono::milliseconds initial_backoff;
    std::chrono::milliseconds attempt_timeout;
};
struct settings {
    thinkthen::backend backend = thinkthen::backend::systemone;
    std::optional<std::string> model, url;
    thinkthen::api_key api_key;
    std::optional<std::uint64_t> batch;
    std::optional<retry_policy> retry;
};
}}
#endif
