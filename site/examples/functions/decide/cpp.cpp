#include <cassert>
#include <thinkthen/client.hpp>

using namespace tt::inputs;

int main() {
    tt::Client client;
    auto question = RequestQuestionText().set_text(
        "Does the customer ask for a refund?");
    auto broken = RequestInputText().set_text(
        "Please refund my order. It arrived broken.");
    auto call = client.decide(question, broken);
    call.finish();
    bool is_refund = false;
    for (const auto& packet : call.collect()) {
        auto row = packet.as_SessionPacketDecideRow();
        if (!row) continue;
        auto yes = row->value().value->value().value;
        is_refund = yes && yes->value() == tt::Json(true);
    }
    assert(is_refund);
}
