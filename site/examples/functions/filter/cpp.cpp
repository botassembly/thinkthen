#include <iostream>
#include <thinkthen/door.hpp>

int main() {
    auto engine = tt::create();

    const char *question = "Is this a complaint?";
    tt::Json reviews = tt::Json::Array{
        "Arrived a day early. Thank you!",
        "The zipper broke the first time I used it.",
        "Does this come in blue?",
        "The strap snapped on day two.",
    };
    auto complaints = tt::call(engine, {
        {"filter", question},
        {"records", reviews},
    }).at("value");
    for (const auto &complaint : complaints) {
        std::cout << complaint.get<std::string>() << "\n";
    }
}
