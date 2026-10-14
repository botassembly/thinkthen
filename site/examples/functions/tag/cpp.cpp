#include <cassert>
#include <string>
#include <vector>
#include <thinkthen/client.hpp>

using namespace tt::inputs;

int main() {
    tt::Client client;
    std::string question = "Which labels fit this message?";
    tt::Json labels =
        tt::Json::Array{"praise", "bug", "billing"};
    auto which_labels = RequestDefinition::from_document(
        tt::Json{{"tag", question}, {"labels", labels}});
    auto message = RequestInputText().set_text(
        "Love the new dashboard, "
        "but export crashes the app,\n"
        "and I was charged twice.\n");
    auto call = client.tag(
        RequestQuestionDefinition().set_value(which_labels),
        message);
    auto fitting_labels = call.answer();
    std::vector<std::string> all_three{
        "praise", "bug", "billing",
    };
    assert(fitting_labels == all_three);
}
