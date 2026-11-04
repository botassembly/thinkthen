#include <iostream>
#include <thinkthen/thinkthen.hpp>

int main() try {
    thinkthen::client client;
    const thinkthen::annotate triage_form{thinkthen::annotation_questions{
        {"refund", thinkthen::decide{"Does the customer request a refund?"}},
        {"topic", thinkthen::choose{"What is the message about?",
                                    {"billing", "shipping", "other"}}},
        {"urgency", thinkthen::score{"How urgent is the request?",
                                     {"routine", "urgent"}}},
    }};
    const std::vector<std::string> messages{
        "Please refund my order.",
        "Where is my parcel?",
        "I was charged twice for one invoice.",
    };
    for (const auto& triage : client.annotate_each(triage_form, messages)) {
        // Each answer prints bare, or as its not-sure or failed status.
        const thinkthen::annotations& answers = triage.value();
        std::cout << thinkthen::get<thinkthen::decide>(answers, "refund")
                  << ' ' << thinkthen::get<thinkthen::choose>(answers, "topic")
                  << ' '
                  << thinkthen::get<thinkthen::score>(answers, "urgency")
                  << '\n';
    }
} catch (const thinkthen::error& failure) {
    std::cerr << failure.code().message() << ": " << failure.what() << '\n';
    return 1;
}
