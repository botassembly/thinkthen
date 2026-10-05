#include <cassert>
#include <thinkthen/door.hpp>

int main() {
    const char *settings[] = {
        "{\"backend\":\"typesafe\"}",
        "{\"backend\":\"liquid\"}",
        "{\"backend\":\"ollama\",\"base_url\":"
        "\"http://localhost:11535/v1\"}",
    };
    for (const char *setting : settings) {
        auto engine = tt::create(setting);

        const char *question =
            "Does the customer ask for a refund?";
        auto broken_is_refund = tt::decide(
            engine,
            question,
            "Please refund my order. It arrived broken.");
        auto thanks_is_refund = tt::decide(
            engine,
            question,
            "Thanks for the quick help yesterday!");
        assert(broken_is_refund.value.outcome
            == tt::Outcome::yes);
        assert(thanks_is_refund.value.outcome
            == tt::Outcome::no);
    }
}
