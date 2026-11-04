#include <iostream>
#include <thinkthen/thinkthen.hpp>

int main() try {
    thinkthen::client client;
    const thinkthen::find requested{"Which line states the requested action?"};
    const std::optional<thinkthen::span> action =
        client
            .find(requested,
                  "Morgan works for Example Group.\nPlease refund my order.")
            .value();
    // Empty when no line answers the question.
    if (action) std::cout << action->text << '\n';
    else std::cout << "no line answers\n";
} catch (const thinkthen::unsure&) {
    std::cerr << "not sure: a person should read it\n";
    return 1;
} catch (const thinkthen::error& failure) {
    std::cerr << failure.code().message() << ": " << failure.what() << '\n';
    return 1;
}
