#include <thinkthen/door.hpp>
#include <iostream>

int main() {
    const auto engine = tt::create();
    const auto value = tt::decide(engine, "Is it?", "yes");
    if (value.value.outcome != tt::Outcome::yes || value.value.probability != .9 ||
        value.facts.records != 1 || value.facts.requestsSent != 1) return 1;
    std::cout << "outcome=1 probability=0.9\n";
}
