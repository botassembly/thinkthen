# ThinkThen for C++

ThinkThen is ten functions on top of Jev, TypeSafe's decision model, or any model like it. It puts these decision models in your code, whatever model, language or database you use. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.
```sh
tar -xzf thinkthen-cpp-0.2.0-TARGET.tar.gz
```
Unpack the matching C archive. Follow the included CMake installation instructions, then link `thinkthen::thinkthen_cpp_shared`.
Set your hosted backend key: `export THINKTHEN_API_KEY=...`. Backends include Jev, SystemOne, OpenAI and local models.
```cpp
#include <cassert>
#include <thinkthen/client.hpp>
using namespace tt::inputs;
int main() {
    tt::Client client;
    auto question = RequestQuestionText().set_text("Does the customer ask for a refund?");
    auto teams = AuthoredOptions(std::vector<std::pair<std::string, AuthoredDescription>>{
        {"billing", AuthoredDescription(std::string("Invoices, fees, and refunds."))},
        {"shipping", AuthoredDescription(std::string("Parcels and delivery."))},
        {"account", AuthoredDescription(std::string("Logins and passwords."))}});
    auto routing = RequestQuestionDefinition().set_value(RequestDefinitionAuthoredChoose()
        .set_choose(AuthoredQuestionText(std::string("Which team owns this?"))).set_options(teams));
    std::vector<std::string> messages{"Please refund the extra fee on my invoice.",
        "My parcel went to the wrong address.", "I cannot reset my password."};
    std::vector<std::string> refunds{"Please refund my order. It arrived broken.",
        "Thanks for the quick help yesterday!", "I want to send this back."};
    std::vector<std::string> expected{"billing", "shipping", "account"};
    for (size_t i = 0; i < messages.size(); ++i) {
        auto input = RequestInputText().set_text(messages[i]);
        auto is_refund = client.decide(question, RequestInputText().set_text(refunds[i])).result().value;
        auto owner = client.choose(routing, input).result();
        assert(is_refund.get<bool>() == (i != 1));
        assert(owner.value == expected[i] && owner.probability.has_value() && owner.results.size() == 1);
    }
}
```
Read `value`, `probability`, `results` and `facts` on the same result. A failed call raises `tt::Failure`.
The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
