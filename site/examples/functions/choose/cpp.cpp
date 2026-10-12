#include <cassert>
#include <string>
#include <thinkthen/client.hpp>

using namespace tt::inputs;

int main() {
    tt::Client client;
    std::string question = "Which team owns this?";
    tt::Json teams{
        {"billing", "Invoices, fees, and refunds."},
        {"shipping", "Parcels and delivery."},
        {"account", "Logins and passwords."},
    };
    auto which_team = RequestDefinition::from_document(
        tt::Json{{"choose", question}, {"options", teams}});
    auto parcel = RequestInputText().set_text(
        "My parcel went to the wrong address.");
    auto call = client.choose(
        RequestQuestionDefinition().set_value(which_team),
        parcel);
    call.finish();
    std::string team;
    for (const auto& packet : call.collect()) {
        auto row = packet.as_SessionPacketChooseRow();
        if (!row) continue;
        auto name = row->value().value->value().value;
        team = name.value_or("");
    }
    assert(team == "shipping");
}
