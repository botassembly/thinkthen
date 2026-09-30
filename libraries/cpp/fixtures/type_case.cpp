#include <thinkthen/door.hpp>
#include <iostream>
#include <iterator>
#include <string>

// With no argument, stdin is one JSON-door request and stdout its reply.
// "plan" reads one plan input and prints tt::plan's object. "fields" reads
// each annotate row member through tt::annotatedField and prints its state.
int main(int argc, char** argv) {
    const std::string mode = argc > 1 ? argv[1] : "";
    const auto request = tt::Json::parse(std::string(std::istreambuf_iterator<char>{std::cin}, {}));
    auto engine = tt::create();
    static const char* names[] = {"", "usage", "backend", "deadline", "local", "cancelled", "defect"};
    try {
        if (mode == "plan") {
            std::vector<std::string> input;
            for (const auto& text : request.at("input")) input.push_back(text.get<std::string>());
            const auto settings = request.contains("settings") ? request.at("settings") : tt::Json();
            std::cout << tt::plan(engine, request.at("verb").get<std::string>(), request.at("question"), input, settings).dump() << '\n';
            return 0;
        }
        const auto reply = tt::call(engine, request);
        if (mode != "fields") { std::cout << reply.dump() << '\n'; return 0; }
        tt::Json::Array states;
        for (const auto& row : reply.at("value")) {
            tt::Json::Object state;
            for (const auto& [name, member] : row.object()) {
                const auto field = tt::annotatedField(member);
                std::string said = std::holds_alternative<tt::Unresolved>(field) ? "unresolved" : "answered";
                if (auto failed = std::get_if<tt::FailedField>(&field))
                    said = std::string("failed ") + names[static_cast<int>(failed->kind)] + " " + failed->cause;
                state.emplace_back(name, said);
            }
            states.emplace_back(std::move(state));
        }
        std::cout << tt::Json(std::move(states)).dump() << '\n';
    } catch (const tt::Failure& failure) {
        const auto code = static_cast<int>(failure.kind);
        if (code < 1 || code > 6) return 2;
        std::cout << tt::Json{{"failed", {{"kind", names[code]}, {"code", code}}}}.dump() << '\n';
    }
}
