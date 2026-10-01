#include <cassert>
#include <thinkthen/door.hpp>

int main() {
    auto engine = tt::create();

    const char *question =
        "Which line gives the refund deadline?";
    tt::Json policy = tt::Json::Array{
        "Returns need the original receipt.",
        "Refunds are issued within 30 days of purchase.",
        "Shipping is free on orders over $50.",
        "Gift cards cannot be exchanged for cash.",
    };
    auto refund_deadline = tt::call(engine, {
        {"find", question},
        {"units", policy},
    }).at("value");
    assert(refund_deadline.at("unit") == policy.at(1));
}
