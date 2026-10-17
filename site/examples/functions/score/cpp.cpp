#include <cassert>
#include <string>
#include <thinkthen/client.hpp>

using namespace tt::inputs;

int main() {
    tt::Client client;
    auto how_urgent =
        RequestQuestionFile().set_path("levels.json");
    auto outage = RequestInputText().set_text(
        "Our checkout page is down and "
        "customers cannot pay.\n");
    auto call = client.score(
        how_urgent,
        outage);
    auto urgency = call.result().value;
    assert(urgency == 2.0);
}
