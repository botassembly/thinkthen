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
    auto is_refund = call.answer().value_or(false);
    assert(is_refund);
}
