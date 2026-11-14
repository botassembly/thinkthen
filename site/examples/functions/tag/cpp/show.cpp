#include <thinkthen/thinkthen.hpp>
#include <array>
#include <iostream>
#include <string_view>
#include <utility>

namespace tt = thinkthen;

enum class Topic { billing, shipping };
template<>
inline constexpr std::array<std::pair<Topic, std::string_view>, 2> thinkthen::words<Topic>{{
    {Topic::billing, "billing"},
    {Topic::shipping, "shipping"},
}};

int main() try {
    tt::client client;
    const tt::tag<Topic> asks_topics{"Which topics appear?"};
    const auto topics = client.tag(asks_topics, "I was charged twice and my parcel is late again.");
    for (const Topic topic : topics.answer())
        std::cout << tt::word(topic) << '\n';
} catch (const std::exception& error) {
    std::cerr << error.what() << '\n';
    return 1;
}
