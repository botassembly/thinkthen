#include <thinkthen/thinkthen.hpp>
#include <cassert>

// Feature: a newer engine error keeps its raw code and message.
extern "C" const char* thinkthen_code_name(std::int32_t code) {
    return code == 999 ? "future.code" : "usage.abi";
}
extern "C" std::int32_t thinkthen_value_error(const thinkthen_value*, thinkthen_error* out, thinkthen_error*) {
    (void)thinkthen_settings_conversion_error(out);
    out->code = 999;
    return THINKTHEN_OK;
}
int main() {
    thinkthen::settings settings;
    settings.backend = thinkthen::backend::stub;
    const auto row = thinkthen::client{settings}.decide(thinkthen::decide{"Ready?"}, thinkthen::missing_record{});
    const auto* error = row.failure();
    assert(error && error->unknown_code() && error->raw_code() == 999);
    assert(std::string(error->what()) == "settings: expected a JSON-representable settings object; received an incompatible host value (contents redacted)");
}
