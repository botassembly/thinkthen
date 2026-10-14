# ThinkThen for C++

ThinkThen is ten functions on top of Jev, TypeSafe's decision model, or any model like it. It puts these decision models in your code, whatever model, language or database you use. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

```sh
tar -xzf thinkthen-cpp-0.2.0-TARGET.tar.gz
```

Replace `TARGET` with the platform name on your downloaded release archive.

Set your hosted backend key: `export THINKTHEN_API_KEY=...`. Backends include Jev, SystemOne, OpenAI and local models.

```cpp
#include <thinkthen/client.hpp>
#include <iostream>
using namespace tt::inputs;
int main() {
    tt::Client client{EngineSettings()};
    auto refund_question = RequestQuestion(RequestQuestionText().set_text("Does the customer ask for a refund?"));
    auto teams = AuthoredOptions(std::vector<std::pair<std::string, AuthoredDescription>>{
        {"billing", AuthoredDescription(std::string("Invoices, fees, and refunds."))},
        {"shipping", AuthoredDescription(std::string("Parcels and delivery."))},
        {"account", AuthoredDescription(std::string("Logins and passwords."))}});
    auto team_question = RequestQuestion(RequestQuestionDefinition().set_value(
        RequestDefinitionAuthoredChoose().set_choose(AuthoredQuestionText(std::string("Which team owns this?"))).set_options(teams)));
    for (const auto* text : {"Please refund my order. It arrived broken.", "Thanks for the quick help yesterday!", "I want to send this back."}) {
        auto refund = client.decide(refund_question, RequestInput(RequestInputText().set_text(text))).answer();
        std::cout << (refund ? (*refund ? "true" : "false") : "not sure") << "\n";
    }
    for (const auto* text : {"Please refund the extra fee on my invoice.", "My parcel went to the wrong address.", "I cannot reset my password."}) {
        auto team = client.choose(team_question, RequestInput(RequestInputText().set_text(text))).answer();
        std::cout << team.value_or("not sure") << "\n";
    }
}
```

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
