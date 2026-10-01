#include <cassert>
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
    auto facts =
        tt::recognize(engine, spec.dump(), text).value;
    tt::Json::Array names;
    for (const auto &one : facts.at("entities")) {
        names.push_back(tt::Json::Array{
            one.at("text"), one.at("kind"),
        });
    }
    assert(tt::Json(names) == (tt::Json::Array{
        tt::Json::Array{"Maria Chen", "PER"},
        tt::Json::Array{"Northwind Freight", "ORG"},
        tt::Json::Array{"Chicago", "LOC"},
    }));
}
