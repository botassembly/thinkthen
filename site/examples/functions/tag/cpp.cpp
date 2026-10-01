#include <iostream>
#include <thinkthen/door.hpp>

int main() {
    auto engine = tt::create();

    const char *question = "Which labels fit this message?";
    tt::Json labels = tt::Json::Array{
        "praise", "bug", "billing",
    };
    const char *message =
        "Love the new dashboard, "
        "but export crashes the app,\n"
        "and I was charged twice.\n";
    auto fitting_labels = tt::call(engine, {
        {"tag", question},
        {"labels", labels},
        {"evidence", message},
    }).at("value");
    std::cout << fitting_labels.dump() << "\n";
}
