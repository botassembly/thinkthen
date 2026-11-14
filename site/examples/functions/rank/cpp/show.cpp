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
    const std::vector<std::string> messages{
        "Please refund my order.",
        "Where is my parcel?",
        "I was charged twice for one invoice.",
    };
    const tt::rank<Urgency> asks_urgency{"How urgent is the request?"};
    for (const auto& ranked : client.rank_each(asks_urgency, messages))
        std::cout << ranked.answer() << ' ' << ranked.item() << '\n';
} catch (const std::exception& error) {
    std::cerr << error.what() << '\n';
    return 1;
}
