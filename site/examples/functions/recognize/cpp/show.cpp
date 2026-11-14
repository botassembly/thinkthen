#include <thinkthen/thinkthen.hpp>
#include <array>
#include <iostream>
#include <string_view>
#include <utility>

namespace tt = thinkthen;

int main() try {
    tt::client client;
    const tt::recognize finds_names{{"person", "organization"}};
    const auto names = client.recognize(finds_names, "Morgan works for Example Group.");
    for (const auto& name : names.answer())
        std::cout << name.kind << ": " << name.text << '\n';
} catch (const std::exception& error) {
    std::cerr << error.what() << '\n';
    return 1;
}
