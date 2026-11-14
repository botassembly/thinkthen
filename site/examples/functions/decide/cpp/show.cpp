#include <thinkthen/thinkthen.hpp>
#include <array>
#include <iostream>
#include <string_view>
#include <utility>

namespace tt = thinkthen;

int main() try {
    tt::client client;
    const tt::decide asks_refund{"Does the customer request a refund?"};
    const auto is_refund = client.decide(asks_refund, "Please refund my order.");
    std::cout << std::boolalpha << is_refund.answer() << '\n';
} catch (const std::exception& error) {
    std::cerr << error.what() << '\n';
    return 1;
}
