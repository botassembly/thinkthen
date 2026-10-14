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
    auto rows = call.answer();
    std::vector<std::string> names;
    for (const auto& row : rows) {
        auto facts =
            row.value().value->as_RecognizeEntities();
        for (const auto& one : *facts->entities().value) {
            names.push_back(*one.text().value + " (" +
                *one.kind().value + ")");
        }
    }
    std::vector<std::string> expected{
        "Sam Patel (person)",
        "Kindle (product)",
        "Denver (place)",
    };
    assert(names == expected);
}
