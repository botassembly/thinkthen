#include <iostream>
#include <thinkthen/door.hpp>

int main() {
    auto engine = tt::create();

    tt::Json kinds = {
        {"PER", "Part of a person's name."},
        {"ORG", "Part of the name of an organization: "
            "a company, band, team, agency, government "
            "body, or media outlet."},
        {"LOC", "Part of the name of a place: a country, "
            "region, city, or geographic feature."},
        {"MISC", "Part of another named entity: a "
            "nationality, an event, a product, or the "
            "name of a creative work."},
    };
    tt::Json spec = {
        {"version", 1},
        {"recognize", {{"kinds", kinds}}},
    };
    const char *text =
        "Maria Chen joined Northwind Freight in Chicago"
        " last spring.";
    auto names =
        tt::recognize(engine, spec.dump(), text).value;
    for (const auto &name : names.at("entities")) {
        auto entity = name.at("text").get<std::string>();
        auto kind = name.at("kind").get<std::string>();
        auto strength = name.at("strength").get<double>();
        std::cout << entity << " " << kind << " "
                  << strength << "\n";
    }
}
