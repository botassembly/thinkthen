// The replay smoke (ticket 0335): one decide through tt::create, which reads
// the environment, with the question and text sdlc/scripts/smoke names.
#include <thinkthen/door.hpp>

#include <cstdlib>
#include <iostream>

int main() {
    auto engine = tt::create();
    auto answer = tt::decide(engine, std::getenv("THINKTHEN_TEST_SMOKE_QUESTION"), std::getenv("THINKTHEN_TEST_SMOKE_TEXT"));
    auto outcome = answer.value.outcome;
    std::cout << "smoke: " << (outcome == tt::Outcome::yes ? "true" : outcome == tt::Outcome::no ? "false" : "null") << "\n";
}
