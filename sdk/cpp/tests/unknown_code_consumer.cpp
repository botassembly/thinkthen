#include <thinkthen/thinkthen.hpp>
#include <thinkthen/nlohmann.hpp>
#include <cassert>

// Feature: a newer engine error keeps its raw code and message.
extern "C" const char* thinkthen_code_name(std::int32_t code) {
    return code == 999 ? "future.code" : "usage.abi";
}
extern "C" std::int32_t thinkthen_row_err_if_failed(const thinkthen_row*, thinkthen_error* out) {
    if (thinkthen_settings_conversion_error(out) >= 0) return THINKTHEN_ERROR;
    out->code = 999;
    return -999;
}
int main() {
    thinkthen::settings settings;
    settings.backend = thinkthen::backend::stub;
    auto rows = thinkthen::client{settings}.decide_each(thinkthen::decide{"Ready?"}, std::vector<thinkthen::owned_record>{thinkthen::missing_record{}});
    const auto row = std::move(*rows.next());
    const auto* error = row.error();
    assert(error && error->unknown_code() && error->raw_code() == 999);
    assert(std::string(error->what()) == "future.code: settings: expected a JSON-representable settings object; received an incompatible host value (contents redacted)");
    const thinkthen::error future{static_cast<thinkthen::errc>(999), "future failure", 503};
    assert(future.unknown_code() && future.raw_code() == 999);
    assert(future.code().message() == "future.code");
    assert(std::string(future.what()) == "future.code: future failure");
    assert(future.family().empty());
    assert(future.http_status() == 503);
    const nlohmann::json expected = {{"code", "future.code"}, {"message", "future failure"}, {"http_status", 503}};
    assert(nlohmann::json(future) == expected);
}
