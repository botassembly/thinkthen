#include <iostream>
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
    const char *words[] = {"no", "yes", "unsure"};
    for (const auto &text : texts) {
        auto is_refund = tt::decide(engine, refund, text);
        int outcome = static_cast<int>(
            is_refund.value.outcome);
        std::cout << words[outcome] << "\n";
    }
}
