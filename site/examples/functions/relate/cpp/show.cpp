#include <thinkthen/thinkthen.hpp>
#include <array>
#include <iostream>
#include <string_view>
#include <utility>

namespace tt = thinkthen;

int main() try {
    tt::client client;
    const tt::rule works_for{.source = "person", .target = "organization", .relation = "works_for"};
    const tt::relate finds_relations{{works_for}};
    const auto relations = client.relate(finds_relations,
        "Morgan works for Example Group.\nPlease refund my order.");
    for (const auto& relation : relations.answer())
        std::cout << relation.source.text << " --" << relation.relation
                  << "--> " << relation.target.text << '\n';
} catch (const std::exception& error) {
    std::cerr << error.what() << '\n';
    return 1;
}
