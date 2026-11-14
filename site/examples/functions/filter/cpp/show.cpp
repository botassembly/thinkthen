#include <thinkthen/thinkthen.hpp>
#include <array>
#include <iostream>
#include <string_view>
#include <utility>

namespace tt = thinkthen;

int main() try {
    tt::client client;
    const std::vector<std::string> messages{
        "Please refund my order.",
        "Where is my parcel?",
        "I was charged twice for one invoice.",
    };
    const tt::filter asks_refund{"Does the customer request a refund?"};
    try {
        for (const auto& refund_request : client.filter_each(asks_refund, messages))
            std::cout << refund_request << '\n';
    } catch (const tt::filter_error<std::string>& error) {
        for (const auto& failed : error.rows())
            std::cerr << failed.item() << ": " << failed << '\n';
        return 1;
    }
} catch (const std::exception& error) {
    std::cerr << error.what() << '\n';
    return 1;
}
