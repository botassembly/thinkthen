#include <iostream>
#include <thinkthen/thinkthen.hpp>

int main() try {
    thinkthen::client client;
    const thinkthen::score how_urgent{"How urgent is the request?",
                                      {"routine", "urgent"}};
    // 0 is routine, 1 is urgent.
    const double urgency =
        client.score(how_urgent, "Please refund my order.").value();
    std::cout << urgency << '\n';
} catch (const thinkthen::error& failure) {
    std::cerr << failure.code().message() << ": " << failure.what() << '\n';
    return 1;
}
