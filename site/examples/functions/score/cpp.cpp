#include <cassert>
#include <thinkthen/door.hpp>

int main() {
    auto engine = tt::create();

    const char *question = "How urgent is this?";
    tt::Json levels = tt::Json::Array{
        "Routine.", "Soon.", "Immediate.",
    };
    const char *text =
        "Our checkout page is down and customers "
        "cannot pay.\n";
    auto urgency = tt::call(engine, {
        {"score", question},
        {"levels", levels},
        {"evidence", text},
    }).at("value");
    assert(urgency.get<double>() == 2.0);
}
