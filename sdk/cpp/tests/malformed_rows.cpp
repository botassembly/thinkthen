#include <thinkthen/thinkthen.hpp>
#include <cassert>
#include <string_view>

// Feature: a malformed delivered row is refused before a typed row escapes.
int main(int argc, char** argv) {
    assert(argc == 2);
    thinkthen::settings config;
    config.backend = thinkthen::backend::stub;
    thinkthen::client client{config};
    bool refused = false;
    try {
        if (std::string_view{argv[1]} == "unanswered-answer")
            (void)client.decide(thinkthen::decide{"Ready?"}, thinkthen::missing_record{});
        else
            (void)client.decide(thinkthen::decide{"Ready?"}, thinkthen::text{"evidence"});
    } catch (const thinkthen::error& error) {
        assert(error.code() == thinkthen::errc::usage_abi && error.family() == "usage");
        refused = true;
    }
    assert(refused);
}
