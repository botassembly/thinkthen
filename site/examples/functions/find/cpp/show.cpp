#include <thinkthen/thinkthen.hpp>
#include <array>
#include <iostream>
#include <string_view>
#include <utility>

namespace tt = thinkthen;

int main() try {
    tt::client client;
    const tt::find asks_action{"Which line states the requested action?"};
    const auto action = client.find(asks_action,
        "Morgan works for Example Group.\nPlease refund my order.");
    if (const auto& span = action.answer())
        std::cout << span->text << '\n';
    else
        std::cout << "no line answers\n";
} catch (const std::exception& error) {
    std::cerr << error.what() << '\n';
    return 1;
}
