#include <thinkthen/thinkthen.hpp>
#include <array>
#include <iostream>
#include <string_view>
#include <utility>

namespace tt = thinkthen;

enum class Topic { billing, shipping, other };
template<>
inline constexpr std::array<std::pair<Topic, std::string_view>, 3> thinkthen::words<Topic>{{
    {Topic::billing, "billing"},
    {Topic::shipping, "shipping"},
    {Topic::other, "other"},
}};

int main() try {
    tt::client client;
    const tt::choose<Topic> asks_topic{"What is the message about?"};
    const auto topic = client.choose(asks_topic, "Please refund my order.");
    std::cout << tt::word(topic.answer()) << '\n';
} catch (const std::exception& error) {
    std::cerr << error.what() << '\n';
    return 1;
}
