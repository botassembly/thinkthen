#include <cassert>
#include <string>
#include <vector>
#include <thinkthen/client.hpp>

using namespace tt::inputs;

int main() {
    tt::Client client;
    auto kinds =
        RequestQuestionFile().set_path("kinds.json");
    auto text = RequestInputText().set_text(
        "Sam Patel says his Kindle arrived broken "
        "in Denver.");
    auto call = client.recognize(kinds, text);
    call.finish();
    std::vector<std::string> names;
    for (const auto& packet : call.collect()) {
        auto found =
            packet.as_SessionPacketRecognizeAggregate();
        if (!found) continue;
        auto rows = found->value().value;
        for (const auto& row : *rows) {
            auto value = *row.value().value;
            auto facts = value.as_RecognizeEntities();
            auto entities = facts->entities().value;
            for (const auto& one : *entities) {
                names.push_back(*one.text().value + " (" +
                    *one.kind().value + ")");
            }
        }
    }
    std::vector<std::string> expected{
        "Sam Patel (person)",
        "Kindle (product)",
        "Denver (place)",
    };
    assert(names == expected);
}
