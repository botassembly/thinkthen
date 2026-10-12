#include <cassert>
#include <string>
#include <vector>
#include <thinkthen/client.hpp>

using namespace tt::inputs;

int main() {
    tt::Client client;
    auto relations =
        RequestQuestionFile().set_path("sings.json");
    std::vector<tt::Json> names{
        {{"name", "Paul McCartney"}, {"kind", "singer"}},
        {{"name", "Ringo Starr"}, {"kind", "singer"}},
        {{"name", "Yesterday"}, {"kind", "song"}},
        {{"name", "Octopus's Garden"}, {"kind", "song"}},
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
    std::vector<std::string> who_sings;
    for (const auto& packet : call.collect()) {
        auto found =
            packet.as_SessionPacketRelateAggregate();
        if (!found) continue;
        auto edges = found->value().value->value().value;
        for (const auto& edge : *edges) {
            auto singer = edge.source().value->document();
            auto song = edge.target().value->document();
            who_sings.push_back(
                singer.at("name").get<std::string>() +
                " sings " +
                song.at("name").get<std::string>());
        }
    }
    std::vector<std::string> expected{
        "Paul McCartney sings Yesterday",
        "Ringo Starr sings Octopus's Garden",
    };
    assert(who_sings == expected);
}
