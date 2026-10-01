#include <cassert>
#include <string>
#include <vector>
#include <thinkthen/door.hpp>

int main() {
    auto engine = tt::create();

    const char *refund = R"({
        "decide": "Does the customer ask for a refund?",
        "threshold": "0.2:0.8"
    })";
    std::vector<std::string> texts = {
        "Please refund my order. It arrived broken.",
        "Thanks for the quick help yesterday!",
        "I want to send this back.",
    };
    std::vector<tt::Outcome> expected = {
        tt::Outcome::yes,
        tt::Outcome::no,
        tt::Outcome::notSure,
    };
    for (size_t i = 0; i < texts.size(); i++) {
        auto is_refund =
            tt::decide(engine, refund, texts[i]);
        assert(is_refund.value.outcome == expected[i]);
    }
}
