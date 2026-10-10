#include <thinkthen/json.hpp>
#include <iostream>
#include <stdexcept>
#include <string>
using tt::Json;
int main() {
    for (const auto& text : {"\"\\uD800\"", "\"\\uDC00\"", "\"\\uD800\\u0061\"",
                             "\"\\q\"", "\"\\uXYZ1\"", "[1,]", "{\"a\":1,\"a\":2}", "01", "1e9999", "-1e9999"}) {
        try { (void)Json::parse(text); std::cerr << "JSON_NEGATIVE_ACCEPTED " << text << '\n'; return 1; }
        catch (const std::invalid_argument&) { std::cout << "JSON_PARSER_NEGATIVE_PASS " << text << '\n'; }
    }
    for (const auto& text : {"0.0", "-0.0", "1.25e2", "5e-324", "1e-9999"}) {
        const auto number=Json::parse(text);
        if (!number.is_number() || Json::parse(number.dump())!=number) return 7;
    }
    if(Json::parse("1e-9999").get<double>()!=0.0) return 8;
    auto emoji=Json::parse("\"\\ud83e\\uddec\"");
    if (emoji.get<std::string>()!="🧬") return 2;
    if (Json::parse(emoji.dump())!=emoji) return 3;
    auto nested=Json::parse(R"({"x":[false,null,1.25,{"q":"line\nend"}]})");
    if (Json::parse(nested.dump())!=nested) return 4;
    auto labels=Json::parse(R"({"options":{"zebra":"first","alpha":"second"}})");
    if (labels.dump().find("zebra")>labels.dump().find("alpha")) return 5;
    if (labels!=Json::parse(R"({"options":{"alpha":"second","zebra":"first"}})")) return 6;
    std::cout << "JSON_PARSER_PASS surrogate_pairs round_trip" << '\n';
}
