#include <thinkthen/thinkthen.hpp>
#include <thinkthen.h>
#include <cassert>
#include <atomic>
#include <cstdint>
#include <exception>
#include <iostream>
#include <string>
#include <thread>
#include <vector>

namespace tt = thinkthen;

static std::string code_name(std::error_code code) {
    return thinkthen_code_name(code.value());
}

/* The runner supplies isolated configuration, a loopback URL and the selected
 * wire format. The public settings use an explicit synthetic key. No test
 * requires real credentials or a remote host.
 */
static tt::settings settings(const char* wire, const char* url) {
    const std::string backend = wire;
    assert(backend == "systemone" || backend == "openai");
    tt::settings value;
    value.backend = backend == "systemone" ? tt::backend::systemone : tt::backend::openai;
    value.url = url;
    value.api_key = decltype(value.api_key){"explicit-synthetic-consumer-key"};
    value.batch = 1;
    value.retry = tt::retry_policy{0, std::chrono::milliseconds{1}, std::chrono::milliseconds{120000}};
    return value;
}

/* Feature: both wires map to the same typed answer/detail with full precision.
 * The reviewed loopback reply selects billing with this exact binary double.
 * It sends the provider's ordinary reply shape, never a prebuilt public row.
 */
static void typed_wire_projection(const tt::settings& config) {
    auto capped_config = config;
    capped_config.max_requests = 0;
    tt::client capped(capped_config);
    try { (void)capped.decide(tt::decide{"Ready?"}, "evidence"); assert(false); }
    catch (const tt::error& failure) { assert(std::string(thinkthen_code_name(failure.code().value())) == "local.request-limit"); }
    tt::client owner(config);
    const auto row = owner.choose(tt::choose{"Which team?", {"billing", "shipping"}},
                                  tt::text{"refund please"});
    constexpr double expected_probability = 0.7000000000000001;
    assert(row.status() == tt::status::answered && row.position() == 1);
    assert(row.answer() && *row.answer() == "billing");
    assert(row.probability() && *row.probability() == expected_probability);
    assert(row.failure() == nullptr);
    const auto detail = row.detail();
    assert(detail && detail->labels.size() == 2);
    assert(detail->labels.at("billing") == expected_probability);
    const auto used = owner.usage();
    assert(used.attempts == 1 && used.responses == 1 && used.request_body_bytes > 0);
}

/* Feature: exhausted record attempts remain rows and terminate normally. */
static void failed_stream(const tt::settings& config) {
    tt::client owner(config);
    std::size_t count = 0;
    for (const auto& row : owner.decide_each(tt::decide{"Ready?"},
            std::vector<tt::owned_record>{tt::owned_text{"first"}, tt::owned_text{"second"}})) {
        assert(row.status() == tt::status::failed);
        assert(row.position() == ++count);
        assert(!row.answer() && !row.probability() && row.failure());
        assert(code_name(row.failure()->code()) == "backend.transport");
    }
    assert(count == 2);
}

/* Feature: cancellation from another thread produces one library error.
 * The parent sends c only after the actual loopback request has arrived.
 * It then observes connection closure before completing the fixture.
 */
static void cancelled_stream(const tt::settings& config, std::size_t read_bound = 0) {
    tt::client owner(config);
    std::atomic<std::size_t> consumed{0};
    auto rows = read_bound ? owner.decide_each(tt::decide{"Ready?"}, [&]() -> std::optional<tt::owned_record> {
        assert(++consumed <= read_bound);
        return tt::owned_text{"evidence"};
    }) : owner.decide_each(tt::decide{"Ready?"}, std::vector<tt::owned_record>{tt::owned_text{"evidence"}});
    auto stop = rows.cancel_handle();
    std::exception_ptr failure;
    std::size_t published = 0;
    std::thread driver([&] {
        try {
            for (const auto& row : rows) { (void)row; ++published; }
        } catch (...) { failure = std::current_exception(); }
    });
    assert(std::cin.get() == 'c');
    stop.request_cancel();
    stop.request_cancel();
    driver.join();
    assert(published == 0 && failure);
    try { std::rethrow_exception(failure); }
    catch (const tt::error& error) { assert(code_name(error.code()) == "local.cancelled"); }
    for (int attempt = 0; attempt != 2; ++attempt) {
        bool threw = false;
        try { (void)rows.next(); }
        catch (const tt::error& error) {
            threw = true;
            assert(code_name(error.code()) == "local.cancelled");
        }
        assert(threw);
    }
    rows.close();
    assert(!rows.next());
}

int main(int argc, char** argv) {
    assert(argc >= 2);
    const std::string mode = argv[1];
    assert(argc == 4 || (argc == 5 && mode == "read-ahead"));
    const auto config = settings(argv[2], argv[3]);
    if (mode == "projection") typed_wire_projection(config);
    else if (mode == "failed-stream") failed_stream(config);
    else { assert(mode == "cancel" || mode == "read-ahead"); cancelled_stream(config, mode == "read-ahead" ? std::stoull(argv[4]) : 0); }
}
