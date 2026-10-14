#include <cassert>
#include <string>
#include <vector>
#include <thinkthen/client.hpp>

using namespace tt::inputs;

int main() {
    tt::Client client;
    auto relations =
        RequestQuestionFile().set_path("makes.json");
    std::vector<tt::Data> names{
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
    auto found = call.result().value;
    std::vector<std::string> who_makes;
    for (const auto& edge : found) {
        who_makes.push_back(
            edge.source_name() + " makes " + edge.target_name());
    }
    std::vector<std::string> expected{
        "Apple makes iPhone",
        "Amazon makes Kindle",
    };
    assert(who_makes == expected);
}
