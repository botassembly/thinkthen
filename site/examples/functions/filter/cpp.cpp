#include <cassert>
#include <string>
#include <vector>
#include <thinkthen/client.hpp>

using namespace tt::inputs;

int main() {
    tt::Client client;
    auto question = RequestQuestionText().set_text(
        "Is this a complaint?");
    std::vector<std::string> reviews{
        "Arrived a day early. Thank you!",
        "The zipper broke the first time I used it.",
        "Does this come in blue?",
        "The strap snapped on day two.",
    };
    std::vector<RequestItem> records;
    for (const auto& review : reviews) {
        records.push_back(RequestItem().set_original(
            RequestOriginalText().set_text(review)));
    }
    auto call = client.filter(
        question, RequestInputRecords().set_items(records));
    auto kept = call.answer();
    std::vector<std::string> complaints;
    for (const auto& text : kept) {
        complaints.push_back(text.get<std::string>());
    }
    std::vector<std::string> expected{
        reviews[1],
        reviews[3],
    };
    assert(complaints == expected);
}
