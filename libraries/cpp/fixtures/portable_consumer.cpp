#include <thinkthen/door.hpp>

#include <cstdlib>
#include <fstream>
#include <iostream>
#include <stdexcept>
#include <string>
#include <vector>

int main() {
    std::ifstream input(std::getenv("TT_PORTABLE_CORPUS"));
    if (!input) throw std::runtime_error("shared portable corpus is missing");
    auto corpus = tt::Json::parse(input);
    std::vector<std::string> texts;
    for (const auto& text : corpus.at("texts")) texts.push_back(text.get<std::string>());
    if (texts.size() != 5) throw std::runtime_error("shared corpus has another text count");
    auto engine = tt::create(std::getenv("TT_PORTABLE_SETTINGS"));
    auto rows = tt::many(engine, corpus.at("question").get<std::string>(), texts);
    if (rows.value.size() != texts.size() || rows.facts.at("records") != 5 || rows.facts.at("requests_sent") != 1)
        throw std::runtime_error("bulk row count or facts changed");
    for (size_t at = 0; at < rows.value.size(); ++at) {
        if (rows.value[at].outcome != tt::Outcome::yes || rows.value[at].probability != 0.9)
            throw std::runtime_error("bulk answer changed at " + std::to_string(at));
    }
    std::cout << "CPP_PORTABLE_BATCH_PASS five ordered rows\n";
}
