#include <iostream>
#include <thinkthen/door.hpp>

int main() {
    auto engine = tt::create();

    const char *question = "Is this urgent?";
    tt::Json inbox = tt::Json::Array{
        "Newsletter: our autumn catalog is here. "
        "No reply needed.",
        "Our checkout page is down and customers "
        "cannot pay",
        "Reminder: your invoice is due in 30 days",
        "Please send the signed quote by 5 pm today",
    };
    auto by_urgency = tt::call(engine, {
        {"rank", question},
        {"records", inbox},
    }).at("value");
    for (const auto &one : by_urgency) {
        std::cout << one.at("record").get<std::string>()
                  << "\n";
    }
}
