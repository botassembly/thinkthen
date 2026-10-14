#include <cassert>
#include <string>
#include <vector>
#include <thinkthen/client.hpp>

using namespace tt::inputs;

int main() {
    tt::Client client;
    auto question = RequestQuestionText().set_text(
        "Is this urgent?");
    std::vector<std::string> inbox{
        "Newsletter: our autumn catalog is here. "
        "No reply needed.",
        "Our checkout page is down and "
        "customers cannot pay",
        "Reminder: your invoice is due in 30 days",
        "Please send the signed quote by 5 pm today",
    };
    std::vector<RequestItem> records;
    for (const auto& message : inbox) {
        records.push_back(RequestItem().set_original(
            RequestOriginalText().set_text(message)));
    }
    auto call = client.rank(
        question, RequestInputRecords().set_items(records));
    auto order = call.result().value;
    std::vector<uint64_t> by_urgency{1, 3, 2, 0};
    assert(order == by_urgency);
}
