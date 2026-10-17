#include <cassert>
#include <string>
#include <vector>
#include <thinkthen/client.hpp>

using namespace tt::inputs;

int main() {
    tt::Client client;
    auto question = RequestQuestionText().set_text(
        "Which line gives the refund deadline?");
    std::vector<std::string> policy{
        "Returns need the original receipt.",
        "Refunds are issued within 30 days of purchase.",
        "Shipping is free on orders over $50.",
        "Gift cards cannot be exchanged for cash.",
    };
    std::vector<RequestItem> records;
    for (const auto& line : policy) {
        records.push_back(RequestItem().set_original(
            RequestOriginalText().set_text(line)));
    }
    auto call = client.find(
        question, RequestInputRecords().set_items(records));
    auto found = call.result().value;
    auto refund_deadline = found.get<std::string>();
    assert(refund_deadline == policy[1]);
}
