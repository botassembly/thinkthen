#include <thinkthen/value.hpp>
#include <iostream>
#include <stdexcept>
#include <string>
using tt::Data;
int main() {
    for (const auto& text : {"\"\\uD800\"", "\"\\uDC00\"", "\"\\uD800\\u0061\"",
                             "\"\\q\"", "\"\\uXYZ1\"", "[1,]", "{\"a\":1,\"a\":2}", "01", "1e9999", "-1e9999",
                             "18446744073709551616", "-9223372036854775809"}) {
        try { (void)::tt::detail::parse_original(text); std::cerr << "JSON_NEGATIVE_ACCEPTED " << text << '\n'; return 1; }
        catch (const std::invalid_argument&) { std::cout << "JSON_PARSER_NEGATIVE_PASS " << text << '\n'; }
    }
    for (const auto& text : {"0.0", "-0.0", "1.25e2", "5e-324", "1e-9999"}) {
        const auto number=::tt::detail::parse_original(text);
        if (!number.is_number() || ::tt::detail::parse_original(::tt::detail::serialize(number))!=number) return 7;
    }
    if(::tt::detail::parse_original("1e-9999").get<double>()!=0.0) return 8;
    auto emoji=::tt::detail::parse_original("\"\\ud83e\\uddec\"");
    if (emoji.get<std::string>()!="🧬") return 2;
    if (::tt::detail::parse_original(::tt::detail::serialize(emoji))!=emoji) return 3;
    auto nested=::tt::detail::parse_original(R"({"x":[false,null,1.25,{"q":"line\nend"}]})");
    if (::tt::detail::parse_original(::tt::detail::serialize(nested))!=nested) return 4;
    auto labels=::tt::detail::parse_original(R"({"options":{"zebra":"first","alpha":"second"}})");
    if (::tt::detail::serialize(labels).find("zebra")>::tt::detail::serialize(labels).find("alpha")) return 5;
    if (labels!=::tt::detail::parse_original(R"({"options":{"alpha":"second","zebra":"first"}})")) return 6;
    std::cout << "JSON_PARSER_PASS surrogate_pairs round_trip" << '\n';
}
