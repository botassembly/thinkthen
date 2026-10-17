#include <cassert>
#include <string>
#include <vector>
#include <thinkthen/client.hpp>

using namespace tt::inputs;

int main() {
    tt::Client client;
    auto which_labels =
        RequestQuestionFile().set_path("labels.json");
    auto message = RequestInputText().set_text(
        "Love the new dashboard, "
        "but export crashes the app,\n"
        "and I was charged twice.\n");
    auto call = client.tag(
        which_labels,
        message);
    auto fitting_labels = call.result().value;
    std::vector<std::string> all_three{
        "praise", "bug", "billing",
    };
    assert(fitting_labels == all_three);
}
