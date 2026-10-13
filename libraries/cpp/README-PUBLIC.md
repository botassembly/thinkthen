# ThinkThen for cpp

ThinkThen answers typed questions about text. Use it to decide yes or no, choose a label, score an answer, or filter and rank records. Failed calls remain separate from answers.

## Install

```sh
tar -xzf thinkthen-cpp-0.2.0-TARGET.tar.gz
```

Replace `TARGET` with the platform name on your downloaded release archive.

## Example

Configure a model backend before running this example.

```cpp
#include <thinkthen/client.hpp>
#include <iostream>
using namespace tt::inputs;

int main() {
    tt::Client client{EngineSettings()};
    auto question = RequestQuestion(RequestQuestionText().set_text("Does this ask for a refund?"));
    auto input = RequestInput(RequestInputText().set_text("Please refund my order."));
    auto answer = client.decide(question, input).answer();
    std::cout << tt::results::encode(answer).dump() << "\n";
}
```

## Cost

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
