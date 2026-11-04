# C++ SDK

The C++ SDK asks questions about text from your C++ code. Its first function, decide, answers a yes-or-no question with yes (`true`), no (`false`) or not sure.

Download the C++ and C archives for your platform from the [0.2.0 release](https://github.com/botassembly/thinkthen/releases/tag/v0.2.0), and extract both into the same `thinkthen` folder. You need a C++17 compiler. Save the program below as `example.cpp`, then compile and run it:

```sh
c++ -std=c++17 -Wall -Wextra -Werror \
    -Ithinkthen/include example.cpp \
    -Lthinkthen/lib -lthinkthen \
    -Wl,-rpath,"$PWD/thinkthen/lib" -o example
./example
```

CMake projects use `find_package(ThinkThenCpp CONFIG REQUIRED)`, link `ThinkThen::Cpp`, and set `CMAKE_PREFIX_PATH` to the `thinkthen` folder. The client reads your key from `THINKTHEN_API_KEY`. [Configuration](https://thinkthen.dev/configuration/) shows how to get one.

## Ask a question

Ask whether a support message requests a refund:

```cpp
#include <iostream>
#include <thinkthen/thinkthen.hpp>

int main() try {
    thinkthen::client client;
    const thinkthen::decide refund{"Does the customer request a refund?"};
    // value() throws when not sure or failed, so neither reads as false.
    const bool is_refund =
        client.decide(refund, "Please refund my order.").value();
    std::cout << std::boolalpha << is_refund << '\n';
} catch (const thinkthen::unsure&) {
    std::cerr << "not sure: a person should read it\n";
    return 1;
} catch (const thinkthen::error& failure) {
    std::cerr << failure.code().message() << ": " << failure.what() << '\n';
    return 1;
}
```

Output:

```text
true
```

`true` means yes: the message asks for a refund.

## Not sure and failures

Set a low and a high bar, and decide is not sure between them. `value()` throws `thinkthen::unsure` when decide is not sure, and `thinkthen::error` when ThinkThen could not judge the message, so neither one passes for `false`. Send both to a person.

```cpp
#include <iostream>
#include <thinkthen/thinkthen.hpp>

int main() try {
    thinkthen::client client;
    // No below 0.2, yes at 0.8 or above, not sure between.
    const auto refund =
        thinkthen::decide{"Does the customer request a refund?"}
            .threshold(thinkthen::threshold::band(0.2, 0.8));
    const auto answer =
        client.decide(refund, "I was charged twice for one invoice.");
    // value() throws thinkthen::unsure here, so it never reads as false.
    std::cout << std::boolalpha << answer.value() << '\n';
} catch (const thinkthen::unsure& not_sure) {
    std::cout << "not sure: ask a person";
    if (const auto probability = not_sure.probability())
        std::cout << " (probability " << *probability << ')';
    std::cout << '\n';
} catch (const thinkthen::error& failure) {
    std::cerr << failure.code().message() << ": " << failure.what()
              << '\n';
    return 1;
}
```

Output:

```text
not sure: ask a person (probability 0.7)
```

## Next

The [C++ SDK page](https://thinkthen.dev/sdks/cpp/) shows all ten functions. The [C and C++ guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/c-cpp.md) and the [API reference](https://github.com/botassembly/thinkthen/blob/v0.2.0/sdk/cpp/include/thinkthen/thinkthen.hpp) cover every option.
