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
    auto found = call.result().value;
    std::vector<std::string> names;
    for (const auto& one : std::get<std::vector<tt::results::Entity>>(found)) {
        names.push_back(*one.text().value + " (" +
            *one.kind().value + ")");
    }
    std::vector<std::string> expected{
        "Sam Patel (person)",
        "Kindle (product)",
        "Denver (place)",
    };
    assert(names == expected);
}
