#include <cassert>
#include <thinkthen/door.hpp>

int main() {
    auto engine = tt::create();

    tt::Json kinds = {
        {"person", nullptr},
        {"organization", nullptr},
        {"place", nullptr},
    };
    tt::Json spec = {
        {"version", 1},
        {"recognize", {{"kinds", kinds}}},
    };
    const char *text =
        "Maria Chen joined Northwind Freight, "
        "a company in Chicago.";
    auto facts =
        tt::recognize(engine, spec.dump(), text).value;
    tt::Json::Array names;
    for (const auto &one : facts.at("entities")) {
        names.push_back(tt::Json::Array{
            one.at("text"), one.at("kind"),
        });
    }
    assert(tt::Json(names) == (tt::Json::Array{
        tt::Json::Array{"Maria Chen", "person"},
        tt::Json::Array{
            "Northwind Freight", "organization",
        },
        tt::Json::Array{"Chicago", "place"},
    }));
}
