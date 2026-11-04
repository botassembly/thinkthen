#include <iostream>
#include <thinkthen/thinkthen.hpp>

int main() try {
    thinkthen::client client;
    const thinkthen::tag appear{"Which topics appear?",
                                {"billing", "shipping"}};
    // Every label that fits, possibly none.
    const std::vector<std::string> topics =
        client.tag(appear, "I was charged twice and my parcel is late again.")
            .value();
    for (const auto& topic : topics) std::cout << topic << '\n';
} catch (const thinkthen::error& failure) {
    std::cerr << failure.code().message() << ": " << failure.what() << '\n';
    return 1;
}
