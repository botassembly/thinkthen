#include <cassert>
#include <string>
#include <thinkthen/client.hpp>

using namespace tt::inputs;

int main() {
    tt::Client client;
    std::string question = "How urgent is this?";
    tt::Json levels =
        tt::Json::Array{"Routine.", "Soon.", "Immediate."};
    auto how_urgent = RequestDefinition::from_document(
        tt::Json{{"score", question}, {"levels", levels}});
    auto outage = RequestInputText().set_text(
        "Our checkout page is down and "
        "customers cannot pay.\n");
    auto call = client.score(
        RequestQuestionDefinition().set_value(how_urgent),
        outage);
    auto urgency = call.answer();
    assert(urgency == 2.0);
}
