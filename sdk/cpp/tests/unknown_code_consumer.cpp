#include <thinkthen/thinkthen.hpp>
#include <cassert>

// Feature: a newer engine's code must belong to this header's typed registry.
extern "C" const char* thinkthen_code_name(std::int32_t code) {
    return code == 999 ? "future.code" : "usage.abi";
}
extern "C" std::int32_t thinkthen_value_error(const thinkthen_value*, thinkthen_error* out, thinkthen_error*) {
    out->code = 999;
    return THINKTHEN_OK;
}
int main() {
    thinkthen::settings settings;
    settings.backend = thinkthen::backend::stub;
    try {
        (void)thinkthen::client{settings}.decide(thinkthen::decide{"Ready?"}, thinkthen::missing_record{});
        assert(false);
    } catch (const thinkthen::error& error) {
        assert(error.code() == thinkthen::errc::usage_abi);
    }
}
