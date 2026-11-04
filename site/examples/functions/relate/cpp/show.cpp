#include <iostream>
#include <thinkthen/thinkthen.hpp>

int main() try {
    thinkthen::client client;
    const thinkthen::relate works_for{
        std::vector<thinkthen::rule>{{"works_for", "person", "organization"}}};
    const std::vector<thinkthen::relation> relations =
        client
            .relate(works_for,
                    "Morgan works for Example Group.\nPlease refund my order.")
            .value();
    for (const auto& relation : relations)
        std::cout << relation.source.text << " --" << relation.label << "--> "
                  << relation.target.text << '\n';
} catch (const thinkthen::error& failure) {
    std::cerr << failure.code().message() << ": " << failure.what() << '\n';
    return 1;
}
