# ThinkThen for C++

Download the matching C++ and C archives from the [0.2.0 release](https://github.com/botassembly/thinkthen/releases/tag/v0.2.0). Extract both into the same `thinkthen` prefix. The C++ package requires the matching C package and a C++17 compiler. Save the example as `example.cpp`. Compile with `c++ -std=c++17 -Ithinkthen/include example.cpp -Lthinkthen/lib -lthinkthen -Wl,-rpath,"$PWD/thinkthen/lib" -o example` and run `./example`. CMake consumers use `find_package(ThinkThenCpp CONFIG REQUIRED)` and link `ThinkThen::Cpp`. Set `CMAKE_PREFIX_PATH` to the shared prefix.

The example uses the stub backend and needs no key or service.

```cpp
#include <thinkthen/thinkthen.hpp>
#include <iostream>

int main() {
    try {
        thinkthen::settings settings;
        settings.backend = thinkthen::backend::stub;
        thinkthen::client client(settings);
        for (const auto& row : client.decide_each(thinkthen::decide{"Ready?"},
                 std::vector<thinkthen::owned_record>{thinkthen::owned_text{"evidence"}})) {
            if (const auto* value = row.value())
                std::cout << *value << '\n';
            else if (row.failure())
                std::cerr << row.failure()->what() << '\n';
        }
    } catch (const thinkthen::error& error) {
        std::cerr << error.what() << '\n';
        return 1;
    }
}
```

`thinkthen::error::family()` returns the engine’s error family, or an empty string for an unknown code.

[API reference](https://github.com/botassembly/thinkthen/blob/v0.2.0/sdk/cpp/include/thinkthen/thinkthen.hpp).
