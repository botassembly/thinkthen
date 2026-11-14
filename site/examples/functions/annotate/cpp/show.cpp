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
    const tt::annotate questions{
        {"refund", tt::decide{"Does the customer request a refund?"}},
        {"topic", tt::choose<Topic>{"What is the message about?"}},
        {"urgency", tt::score<Urgency>{"How urgent is the request?"}},
    };
    for (auto& form : client.annotate_each(questions, messages)) {
        (void)form.throw_if_failed(); // Includes failed entries, not only the form.
        const auto& entries = form.answer();
        const auto& refund = tt::get<tt::decide>(entries, "refund");
        const auto& topic = tt::get<tt::choose<Topic>>(entries, "topic");
        const auto& urgency = tt::get<tt::score<Urgency>>(entries, "urgency");
        // Each answer read refuses not-sure entries instead of inventing a value.
        std::cout << std::boolalpha << refund.answer() << ' '
                  << tt::word(topic.answer()) << ' ' << urgency.answer() << '\n';
    }
} catch (const std::exception& error) {
    std::cerr << error.what() << '\n';
    return 1;
}
