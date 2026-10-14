#include <cassert>
#include <string>
#include <thinkthen/client.hpp>

using namespace tt::inputs;

int main() {
    tt::Client client;
    auto which_team =
        RequestQuestionFile().set_path("teams.json");
    auto parcel = RequestInputText().set_text(
        "My parcel went to the wrong address.");
    auto call = client.choose(
        which_team,
        parcel);
    auto team = call.answer().value_or("");
    assert(team == "shipping");
}
