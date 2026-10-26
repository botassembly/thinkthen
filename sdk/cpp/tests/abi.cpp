#include <thinkthen/thinkthen.hpp>
#include <iostream>
int main(int argc, char** argv) {
    try {
        const std::string first = argc > 1 ? argv[1] : "client";
        if (first == "key") { thinkthen::api_key key{"synthetic"}; }
        if (first == "question") { thinkthen::decide question{"Ready?"}; }
        if (first == "threshold") { auto cut = thinkthen::threshold::cutoff(0.5); (void)cut; }
        thinkthen::settings s; s.backend = thinkthen::backend::stub;
        thinkthen::client client(s);
        auto row = client.decide(thinkthen::decide{"Ready?"}, thinkthen::missing_record{});
        if (!row.failure() || row.failure()->code() != thinkthen::errc::record_missing) return 1;
        std::cout << "accepted\n";
    } catch (const thinkthen::error& error) {
        if (error.code() != thinkthen::errc::usage_abi) return 1;
        std::cout << "usage.abi\n";
    }
}
