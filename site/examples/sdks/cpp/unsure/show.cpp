#include <iostream>
#include <thinkthen/thinkthen.hpp>

int main() try {
    thinkthen::client client;
    // No below 0.2, yes at 0.8 or above, not sure between.
    const auto refund =
        thinkthen::decide{"Does the customer request a refund?"}
            .threshold(thinkthen::threshold::band(0.2, 0.8));
    const auto answer =
        client.decide(refund, "I was charged twice for one invoice.");
    // answer() throws thinkthen::unsure here, so it never reads as false.
    std::cout << std::boolalpha << answer.answer() << '\n';
} catch (const thinkthen::unsure& not_sure) {
    std::cout << "not sure: ask a person";
    if (const auto probability = not_sure.probability())
        std::cout << " (probability " << *probability << ')';
    std::cout << '\n';
} catch (const thinkthen::error& failure) {
    std::cerr << failure.code().message() << ": " << failure.what()
              << '\n';
    return 1;
}
