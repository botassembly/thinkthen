# ThinkThen for C++

ThinkThen is ten functions on top of Jev, TypeSafe's System One model, or any model like it. System One models return probabilities. Your code decides what to do with them. ThinkThen makes these models simple to call from any language or database. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

```sh
tar -xzf thinkthen-cpp-0.2.0-TARGET.tar.gz
```
Download the matching archive from the [release downloads](https://github.com/botassembly/thinkthen/releases). Replace `TARGET` with your platform and unpack the matching C archive. Follow the included CMake installation instructions, then link `thinkthen::thinkthen_cpp_shared`.
Set your hosted backend key: `export THINKTHEN_API_KEY=...`. Backends include Jev, SystemOne, OpenAI and local models. Judgment calls can send your question and text to the selected backend; the default is TypeSafe.
```cpp
#include <cassert>
#include <thinkthen/client.hpp>
using namespace tt::inputs;
int main() {
    tt::Client client; auto question = RequestQuestionText().set_text("Does the customer ask for a refund?");
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
    std::vector<double> refund_probabilities{0.99, 0.01, 0.54};
    for (size_t i = 0; i < messages.size(); ++i) {
        auto refund = client.decide(question, RequestInputText().set_text(refunds[i])).result();
        auto owner = client.choose(routing, RequestInputText().set_text(messages[i])).result();
        assert(refund.value.get<bool>() == (i != 1) && refund.probability == refund_probabilities[i]);
        assert(owner.value == expected[i] && owner.probability == 1.0 && owner.results.size() == 1);
    }
}
```
Read `value`, `probability`, `results` and `facts` on the same result. A failed call raises `tt::Failure`.
The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them. These examples were checked with saved answers; live answers and probabilities can differ. See the [package guide](https://thinkthen.dev/languages/cpp/) for results, failures, record inputs and offline replay.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
