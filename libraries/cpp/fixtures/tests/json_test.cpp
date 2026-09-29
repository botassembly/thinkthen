#include <thinkthen/door.hpp>
#include <iostream>
#include <stdexcept>
#include <string>
using tt::Json;
int main() {
    for (const auto& text : {"\"\\uD800\"", "\"\\uDC00\"", "\"\\uD800\\u0061\"",
                             "\"\\q\"", "\"\\uXYZ1\"", "[1,]", "{\"a\":1,\"a\":2}", "01", "1e9999"}) {
        try { (void)Json::parse(text); std::cerr << "JSON_NEGATIVE_ACCEPTED " << text << '\n'; return 1; }
        catch (const std::invalid_argument&) { std::cout << "JSON_PARSER_NEGATIVE_PASS " << text << '\n'; }
    }
    auto emoji=Json::parse("\"\\ud83e\\uddec\"");
    if (emoji.get<std::string>()!="🧬") return 2;
    if (Json::parse(emoji.dump())!=emoji) return 3;
    auto nested=Json::parse(R"({"x":[false,null,1.25,{"q":"line\nend"}]})");
    if (Json::parse(nested.dump())!=nested) return 4;
    auto labels=Json::parse(R"({"options":{"zebra":"first","alpha":"second"}})");
    if (labels.dump().find("zebra")>labels.dump().find("alpha")) return 5;
    if (labels!=Json::parse(R"({"options":{"alpha":"second","zebra":"first"}})")) return 6;
    if (Json::parse("9007199254740992").get<size_t>() != static_cast<size_t>(9007199254740992ULL)) return 7;
    std::cout << "JSON_EXACT_INTEGER_BOUNDARY_PASS 9007199254740992" << '\n';
    try { (void)Json::parse("9007199254740993"); return 8; }
    catch (const std::invalid_argument& e) {
        if (std::string(e.what()) != "JSON integer outside exact double range") return 9;
        std::cout << "JSON_INTEGER_PRECISION_REJECT_PASS 9007199254740993" << '\n';
    }
    try { (void)Json::parse("18446744073709551616").get<size_t>(); return 10; }
    catch (const std::invalid_argument&) { std::cout << "JSON_SIZE_OVERFLOW_PASS" << '\n'; }
    try {
        (void)tt::entity(Json::parse(R"({"text":"x","start":9007199254740993,"end":9007199254740993,"length":0,"kind":"alert","strength":0.9})"));
        return 11;
    } catch (const std::invalid_argument& e) {
        if (std::string(e.what()) != "JSON integer outside exact double range") return 12;
        std::cout << "JSON_ENTITY_OFFSET_PRECISION_REJECT_PASS 9007199254740993" << '\n';
    }
    std::cout << "JSON_PARSER_PASS surrogate_pairs round_trip" << '\n';
}
