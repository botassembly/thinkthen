#include <cassert>
#include <fstream>
#include <thinkthen/door.hpp>

int main() {
    auto engine = tt::create();

    std::ifstream form("form.json");
    auto questions = tt::Json::parse(form);
    const char *report =
        "Steps: click Log in. Nobody gets in.";
    auto triage = tt::call(engine, {
        {"annotate", questions},
        {"records", tt::Json::Array{report}},
    }).at("value");
    assert(triage.at(0) == tt::Json({
        {"steps", true},
        {"area", "login"},
        {"impact", 1.98},
    }));
}
