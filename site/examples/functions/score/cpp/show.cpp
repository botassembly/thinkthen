#include <thinkthen/thinkthen.hpp>
#include <array>
#include <iostream>
#include <string_view>
#include <utility>

namespace tt = thinkthen;

enum class Urgency { routine, urgent };
template<>
inline constexpr std::array<std::pair<Urgency, std::string_view>, 2> thinkthen::words<Urgency>{{
    {Urgency::routine, "routine"},
    {Urgency::urgent, "urgent"},
}};

int main() try {
    tt::client client;
    const tt::score<Urgency> asks_urgency{"How urgent is the request?"};
    const auto urgency = client.score(asks_urgency, "Please refund my order.");
    std::cout << urgency.answer() << '\n';
} catch (const std::exception& error) {
    std::cerr << error.what() << '\n';
    return 1;
}
