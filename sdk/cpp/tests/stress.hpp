// Feature: C++ callers share a client while owning independent faulted streams.
#include <cstdlib>
#include <thread>
#include <atomic>
static int stress_main(int argc, char** argv) {
    assert(argc == 10);
    tt::client owner{settings(json::parse(argv[2]))};
    const auto count = std::stoull(argv[3]);
    const bool cancel = std::string{argv[1]} == "stress-cancel";
    auto records_for = [&](std::size_t offset) {
        std::vector<tt::owned_record> records;
        for (std::size_t i = 0; i < count; ++i)
            records.push_back(tt::json_record(json{{"id", "record-" + std::to_string(offset + i)}, {"body", "record " + std::to_string(offset + i)}}));
        return records;
    };
    std::atomic<std::size_t> failures{0};
    std::atomic<std::uint64_t> widths{0};
    const auto workers = cancel ? 1 : std::stoull(argv[7]);
    auto drive = [&](std::size_t offset) {
        const auto records = records_for(offset);
        std::size_t received = 0, failed = 0;
        for (const auto& row : owner.decide_each(tt::decide{"Ready?"}.evidence("/body"), records)) {
            assert(row.position() == ++received);
            widths.fetch_or(UINT64_C(1) << json(row).at("id").get<std::string>().size());
            assert(json(row).at("id") == "record-" + std::to_string(offset + received - 1));
            if (row.status() == tt::status::failed) {
                ++failed;
                assert(std::string{thinkthen_code_name(static_cast<int>(row.failure()->code()))} == argv[5]);
            } else {
                assert(row.status() == tt::status::answered && row.value() && *row.value());
                assert(row.probability() && *row.probability() == std::stod(argv[6]));
            }
        }
        assert(received == count);
        failures += failed;
    };
    const auto rounds = std::stoull(argv[8]);
    for (std::size_t round = 0; round < rounds; ++round) {
        if (cancel) {
            auto rows = owner.decide_each(tt::decide{"Ready?"}.evidence("/body"), records_for(0));
            auto stop = rows.cancel_handle();
            bool cancelled = false;
            std::thread worker([&] {
                try { for (const auto& row : rows) { (void)row; assert(false); } }
                catch (const tt::error& error) { assert(error.code() == tt::errc::local_cancelled); cancelled = true; }
            });
            assert(std::cin.get() == 'c');
            stop.request_cancel();
            worker.join();
            assert(cancelled);
        } else {
            std::vector<std::thread> threads;
            for (std::size_t i = 0; i < workers; ++i) threads.emplace_back(drive, std::stoull(argv[9]) + (round * workers + i) * count);
            for (auto& thread : threads) thread.join();

        }
        if (const auto* hook = std::getenv("THINKTHEN_STRESS_CHECKPOINT_COMMAND")) assert(std::system(hook) == 0);
    }
    assert(cancel || (std::string{argv[4]} == "failed" ? failures > 0 : failures == 0));
    std::cout << json{{"identifier_width_mask", widths.load()}, {"calls", workers * rounds}, {"rows", cancel ? 0 : count * workers * rounds}, {"failed", failures.load()}, {"cancelled", cancel}} << '\n';
    return 0;
}
