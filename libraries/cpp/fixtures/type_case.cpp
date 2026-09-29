#include <thinkthen/door.hpp>
#include <iostream>
#include <iterator>
#include <string>

int main() {
    std::string request(std::istreambuf_iterator<char>{std::cin}, {});
    auto engine = tt::create();
    try {
        std::cout << tt::call(engine, tt::Json::parse(request)).dump() << '\n';
    } catch (const tt::Failure& failure) {
        static const char* names[] = {"", "usage", "backend", "deadline", "local", "cancelled", "defect"};
        const auto code = static_cast<int>(failure.kind);
        if (code < 1 || code > 6) return 2;
        std::cout << tt::Json{{"failed", {{"kind", names[code]}, {"code", code}}}}.dump() << '\n';
    }
}
