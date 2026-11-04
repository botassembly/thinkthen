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
