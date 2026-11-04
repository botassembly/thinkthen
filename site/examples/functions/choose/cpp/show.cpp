#include <iostream>
#include <thinkthen/thinkthen.hpp>

int main() try {
    thinkthen::client client;
    const thinkthen::choose about{"What is the message about?",
                                  {"billing", "shipping", "other"}};
    const std::string topic =
        client.choose(about, "Please refund my order.").value();
    std::cout << topic << '\n';
} catch (const thinkthen::unsure&) {
    std::cerr << "not sure: a person should read it\n";
    return 1;
} catch (const thinkthen::error& failure) {
    std::cerr << failure.code().message() << ": " << failure.what() << '\n';
    return 1;
}
