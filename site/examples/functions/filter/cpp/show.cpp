#include <iostream>
#include <thinkthen/thinkthen.hpp>

int main() try {
    thinkthen::client client;
    const thinkthen::filter refund{"Does the customer request a refund?"};
    const std::vector<std::string> messages{
        "Please refund my order.",
        "Where is my parcel?",
        "I was charged twice for one invoice.",
    };
    // Only the messages that answer yes. Failed messages are reported
    // together after the last kept one.
    for (const auto& refund_request : client.filter_each(refund, messages))
        std::cout << std::get<thinkthen::owned_text>(refund_request).value
                  << '\n';
} catch (const thinkthen::error& failure) {
    std::cerr << failure.code().message() << ": " << failure.what() << '\n';
    for (const auto& failed : failure.failed_rows())
        std::cerr << "message " << failed.position << ": " << failed.message
                  << '\n';
    return 1;
}
