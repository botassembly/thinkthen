#include <thinkthen/thinkthen.hpp>
#include <cassert>
#include <string>
// Feature: call failures from a newer engine preserve unknown raw codes and messages.
int main(int argc, char** argv) {
    assert(argc == 2);
    thinkthen::settings settings;
    settings.backend = thinkthen::backend::stub;
    try {
        thinkthen::client client{settings};
        assert(false);
    } catch (const thinkthen::error& error) {
        assert(error.unknown_code() && error.raw_code() == 2147483647);
        assert(std::string(error.what()) == argv[1]);
    }
}
