#include <cassert>
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
    tt::Json::Array order;
    for (const auto &one : by_urgency) {
        order.push_back(one.at("index"));
    }
    assert(tt::Json(order)
        == (tt::Json::Array{1, 3, 2, 0}));
}
