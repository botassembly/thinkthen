#include <cassert>
#include <string>
#include <vector>
#include <thinkthen/client.hpp>

using namespace tt::inputs;

int main() {
    tt::Client client;
    auto relations =
        RequestQuestionFile().set_path("makes.json");
    std::vector<tt::Json> names{
        {{"name", "Apple"}, {"kind", "company"}},
        {{"name", "Amazon"}, {"kind", "company"}},
        {{"name", "iPhone"}, {"kind", "product"}},
        {{"name", "Kindle"}, {"kind", "product"}},
    };
    std::vector<RequestItem> entities;
    for (const auto& name : names) {
        entities.push_back(RequestItem().set_original(
            RequestOriginalJson().set_value(name)));
    }
    auto call = client.relate(
        relations,
        RequestInputEntities().set_items(entities));
    call.finish();
    std::vector<std::string> who_makes;
    for (const auto& packet : call.collect()) {
        auto found =
            packet.as_SessionPacketRelateAggregate();
        if (!found) continue;
        auto edges = found->value().value->value().value;
        for (const auto& edge : *edges) {
            auto company = edge.source().value->document();
            auto product = edge.target().value->document();
            who_makes.push_back(
                company.at("name").get<std::string>() +
                " makes " +
                product.at("name").get<std::string>());
        }
    }
    std::vector<std::string> expected{
        "Apple makes iPhone",
        "Amazon makes Kindle",
    };
    assert(who_makes == expected);
}
