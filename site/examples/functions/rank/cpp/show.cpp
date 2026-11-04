#include <iostream>
#include <thinkthen/thinkthen.hpp>

int main() try {
    thinkthen::client client;
    const thinkthen::rank by_urgency{"How urgent is the request?",
                                     {"routine", "urgent"}};
    const std::vector<std::string> messages{
        "Please refund my order.",
        "Where is my parcel?",
        "I was charged twice for one invoice.",
    };
    // Rank 1 is the most urgent. item() is the message that rank belongs to.
    for (const auto& ranked : client.rank_each(by_urgency, messages))
        std::cout << ranked.value() << ' '
                  << std::get<thinkthen::owned_text>(ranked.item()).value
                  << '\n';
} catch (const thinkthen::error& failure) {
    std::cerr << failure.code().message() << ": " << failure.what() << '\n';
    return 1;
}
