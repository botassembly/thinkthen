#include <cassert>
#include <thinkthen/door.hpp>

int main() {
    auto engine = tt::create();

    const char *question =
        "Does the customer ask for a refund?";
    auto is_refund = tt::decide(
        engine,
        question,
        "Please refund my order. It arrived broken.");
    assert(is_refund.value.outcome == tt::Outcome::yes);

    const char *refund = R"({
        "decide": "Does the customer ask for a refund?",
        "threshold": "0.2:0.8"
    })";
    is_refund = tt::decide(
        engine,
        refund,
        "I want to send this back.");
    assert(is_refund.value.outcome == tt::Outcome::notSure);
}
