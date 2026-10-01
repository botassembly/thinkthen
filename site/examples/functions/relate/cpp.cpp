#include <cassert>
#include <string>
#include <vector>
#include <thinkthen/door.hpp>

int main() {
    auto engine = tt::create();

    const char *spec = R"({"version": 1, "relate": {
        "relations": [{
            "name": "sings",
            "source": "singer",
            "target": "song"
        }]
    }})";
    std::vector<std::string> names = {
        R"({"name": "Paul McCartney", "kind": "singer"})",
        R"({"name": "Ringo Starr", "kind": "singer"})",
        R"({"name": "Yesterday", "kind": "song"})",
        R"({"name": "Octopus's Garden", "kind": "song"})",
    };
    auto who_sings = tt::relate(engine, spec, names).value;
    tt::Json::Array sings;
    for (const auto &edge : who_sings.at("edges")) {
        sings.push_back(tt::Json::Array{
            edge.at("source").at("name"),
            edge.at("target").at("name"),
        });
    }
    assert(tt::Json(sings) == (tt::Json::Array{
        tt::Json::Array{"Paul McCartney", "Yesterday"},
        tt::Json::Array{"Ringo Starr", "Octopus's Garden"},
    }));
}
