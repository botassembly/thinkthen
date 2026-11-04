#include <iostream>
#include <thinkthen/thinkthen.hpp>

int main() try {
    thinkthen::client client;
    const thinkthen::recognize people_and_groups{
        std::vector<std::string>{"person", "organization"}};
    const std::vector<thinkthen::name> names =
        client.recognize(people_and_groups, "Morgan works for Example Group.")
            .value();
    for (const auto& name : names)
        std::cout << name.kind << ": " << name.text << '\n';
} catch (const thinkthen::error& failure) {
    std::cerr << failure.code().message() << ": " << failure.what() << '\n';
    return 1;
}
