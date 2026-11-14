#include <thinkthen/thinkthen.hpp>
#include <thinkthen/nlohmann.hpp>
#include <cassert>
#include <iostream>

// Feature: rejecting a native ABI leaves errors safe to inspect and serialize.
static void inspect(const thinkthen::error& error, thinkthen::errc code,
                    const char* name, const char* message, std::string_view family,
                    bool unknown, std::optional<std::uint16_t> http = {}) {
    assert(error.code() == code);
    assert(error.raw_code() == static_cast<std::int32_t>(code));
    assert(error.code().category() == thinkthen::category());
    assert(error.code().message() == (name ? name : THINKTHEN_ABI_ERROR_MESSAGE));
    assert(std::string{error.what()} == std::string{name ? name : THINKTHEN_ABI_ERROR_MESSAGE} + ": " + message);
    assert(error.family() == family);
    assert(error.unknown_code() == unknown);
    assert(error.http_status() == http);
    if (name) {
        nlohmann::json expected = {{"code", name}, {"message", message}};
        if (http) expected["http_status"] = *http;
        assert(nlohmann::json(error) == expected);
    }
}

static void inspect_constructed_errors() {
    inspect(thinkthen::error{thinkthen::errc::usage_abi, "refused", 503},
            thinkthen::errc::usage_abi, THINKTHEN_ABI_ERROR_CODE, "refused", "usage", false, 503);
    const auto unknown = static_cast<thinkthen::errc>(2147483647);
    inspect(thinkthen::error{unknown, "future failure", 502}, unknown,
            nullptr, "future failure", "", true, 502);
}

int main(int argc, char** argv) {
    try {
        const std::string first = argc > 1 ? argv[1] : "client";
        if (first == "key") { thinkthen::api_key key{"synthetic"}; }
        if (first == "question") { thinkthen::decide question{"Ready?"}; }
        if (first == "threshold") { auto cut = thinkthen::threshold::cutoff(0.5); (void)cut; }
        thinkthen::settings s; s.backend = thinkthen::backend::stub;
        thinkthen::client client(s);
        auto rows = client.decide_each(thinkthen::decide{"Ready?"}, std::vector<thinkthen::owned_record>{thinkthen::missing_record{}});
        auto row = std::move(*rows.next());
        if (!row.error() || row.error()->code() != thinkthen::errc::record_missing) return 1;
        inspect_constructed_errors();
        inspect(thinkthen::error{thinkthen::errc::backend_status, "backend refused", 429},
                thinkthen::errc::backend_status, "backend.status", "backend refused", "backend", false, 429);
        std::cout << "accepted\n";
    } catch (const thinkthen::error& error) {
        if (error.code() != thinkthen::errc::usage_abi) return 1;
        inspect(error, thinkthen::errc::usage_abi, THINKTHEN_ABI_ERROR_CODE,
                THINKTHEN_ABI_ERROR_MESSAGE, "usage", false);
        inspect_constructed_errors();
        std::cout << "usage.abi\n";
    }
}
