#include <thinkthen/door.hpp>
#include <iostream>
#include <stdexcept>
#include <string>
using tt::Json;
int main() {
    const auto goodFacts=Json::parse(R"({"records":1,"requests_sent":1,"cache_answers":0,"seconds":0.25,"model":"fixture"})");
    const auto parsedFacts=tt::decodeFacts(goodFacts);
    if (parsedFacts.records!=1 || parsedFacts.requestsSent!=1 || parsedFacts.cacheAnswers!=0 ||
        parsedFacts.seconds!=0.25 || parsedFacts.inputTokens || parsedFacts.outputTokens ||
        parsedFacts.model!=std::optional<std::string>("fixture")) return 20;
    for (const auto& bad : {
        R"({"requests_sent":1,"cache_answers":0,"seconds":0})",
        R"({"records":null,"requests_sent":1,"cache_answers":0,"seconds":0})",
        R"({"records":1,"requests_sent":1,"cache_answers":0,"seconds":0,"input_tokens":null})",
        R"({"records":1,"requests_sent":1,"cache_answers":0,"seconds":0,"model":null})",
        R"({"records":-1,"requests_sent":1,"cache_answers":0,"seconds":0})",
        R"({"records":1.5,"requests_sent":1,"cache_answers":0,"seconds":0})",
        R"({"records":1,"requests_sent":1,"cache_answers":0,"seconds":-0.1})"}) {
        try { (void)tt::decodeFacts(Json::parse(bad)); return 21; }
        catch (const std::exception&) {}
    }
    std::cout << "CALL_FACTS_STRICT_DECODER_PASS" << '\n';
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
