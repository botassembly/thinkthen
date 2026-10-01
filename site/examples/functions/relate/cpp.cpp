#include <iostream>
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
    for (const auto &edge : who_sings.at("edges")) {
        auto singer = edge.at("source").at("name");
        auto song = edge.at("target").at("name");
        std::cout << singer.get<std::string>() << " sings "
                  << song.get<std::string>() << " "
                  << edge.at("probability").get<double>()
                  << "\n";
    }
}
