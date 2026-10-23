#include <thinkthen/thinkthen.hpp>
#include <cassert>
#include <fstream>
#include <iostream>
#include <limits>
#include <thread>
#include <type_traits>

namespace tt = thinkthen;
using tt::json;

static json read(const char* path) {
    std::ifstream input(path);
    return json::parse(input);
}
static json encode(const tt::row& row) {
    const char* state = row.status() == tt::status::answered ? "answered"
        : row.status() == tt::status::unsure ? "unsure" : "failed";
    json result = {{"status", state}};
    if (row.id()) result["id"] = *row.id();
    if (row.answer()) result["answer"] = *row.answer();
    if (auto p = row.probability()) result["p"] = *p;
    if (auto detail = row.detail()) result["detail"] = *detail;
    if (auto error = row.error()) result["error"] = {{"code", error->code()}, {"message", error->what()}};
    return result;
}
template<class Answer> static json collect(tt::typed_stream<Answer> rows, const char* function) {
    json result = json::array();
    std::vector<tt::typed_row<Answer>> retained;
    for (const auto& row : rows) {
        assert(row.position() >= 1);
        if (std::string(function) == "filter") assert(row.kept() == (*row.raw_answer() == true));
        if (const auto answer = row.answer()) assert(json(*answer) == *row.raw_answer());
        else assert(!row.raw_answer() || row.raw_answer()->is_null());
        retained.push_back(row);
    }
    rows.close();
    for (const auto& row : retained) result.push_back(encode(row));
    return result;
}
static json named(const tt::client& client, const json& request) {
    const auto& function = request.at("function");
    tt::tagged_question question{request.at("question")};
    auto input = request.at("records").get<tt::records>();
    std::size_t reads = 0;
    tt::record_source source = [&]() -> std::optional<json> {
        if (reads == input.size()) return std::nullopt;
        return input[reads++];
    };
    auto options = request.value("options", json::object());
#define CALL(name, type) if (function == #name) { \
    if (question.value.at("type") == "inline") return collect(client.name(question.value.at("value").get<type>(), source, options), #name); \
    return collect(client.name(question, source, options), #name); }
    CALL(decide, std::string); CALL(choose, std::string); CALL(tag, std::string);
    CALL(score, std::string); CALL(filter, std::string); CALL(rank, std::string);
    CALL(find, std::string); CALL(annotate, json);
    CALL(recognize, std::vector<std::string>); CALL(relate, json);
#undef CALL
    throw std::logic_error("unknown fixture function");
}
static void construction(const char* path) {
    for (const auto& test : read(path)) {
        tt::client owner(test.at("client"));
        assert(named(owner, test.at("request")) == test.at("expected"));
    }
}
static void typed_answers() {
    tt::client client(json{{"adapter", "stub"}});
    auto surviving = [] {
        tt::client owner(json{{"adapter", "stub"}});
        return owner.decide("Ready?", {"alpha"});
    }();
    const auto verdict = surviving.next();
    surviving.close();
    static_assert(std::is_same_v<decltype(verdict->answer()), std::optional<bool>>);
    assert(verdict && verdict->answer().has_value());
    const auto found = client.find("Which line?", {"alpha\nbeta\ngamma"}).next();
    assert(found && found->answer()->text == found->raw_answer()->at("text"));
    std::size_t reads = 0;
    auto call = client.choose(tt::tagged_question{json{{"type", "toolbox"},
        {"name", "saved"}, {"value", {{"questions", {{"saved", {{"choose", "Pick a color"},
        {"options", {"red", "green", "blue"}}}}}}}}}},
        [&]() -> std::optional<json> { if (reads++ == 0) return "alpha"; return std::nullopt; });
    assert(reads == 0);
    auto row = call.next();
    assert(row && row->answer() == client.choose("Pick a color", {"alpha"},
        {{"options", {"red", "green", "blue"}}}).next()->answer());
    assert(!call.next() && reads == 2);
    auto annotated = client.annotate(json{{"questions", {{"ready", {{"decide", "Ready?"}}}}}}, {"alpha"}).next();
    assert(annotated->answer()->at("ready").answer<bool>() == verdict->answer());
    auto scaled = client.annotate(json{{"questions", {{"urgency", {{"score", "How urgent?"},
        {"levels", {"low", "high"}}}}}}}, {"alpha"}).next();
    assert(scaled && json(*scaled->answer()) == *scaled->raw_answer());
}
static void options(const char* path) {
    for (const auto& test : read(path)) {
        tt::client client(test.at("client"));
        try {
            const auto actual = named(client, test.at("request"));
            assert(!test.contains("expected_error"));
            const auto& equivalent = test.at("equivalent_request");
            assert(actual.size() == equivalent.at("records").size());
            assert(actual == named(client, equivalent));
        } catch (const tt::error& error) {
            assert(error.code() == test.at("expected_error"));
        }
    }
}
static void boundaries() {
    static_assert(!std::is_copy_constructible_v<tt::client>);
    static_assert(!std::is_copy_constructible_v<tt::stream>);
    static_assert(std::is_same_v<decltype(std::declval<const tt::row>().answer()), const json*>);
    tt::client initial(json{{"adapter", "stub"}});
    tt::client client(std::move(initial));
    auto key = std::string("a\0b", 3);
    json id = {{key, std::numeric_limits<std::uint64_t>::max()}};
    auto rows = client.decide("Ready?", {{{"id", id}, {"body", "evidence"}}});
    auto stop = rows.cancel_handle();
    auto moved = std::move(rows);
    auto first = moved.next();
    assert(first && first->id() && *first->id() == id);
    assert(first->answer() && first->detail());
    auto copy = *first;
    moved.close();
    stop.request_cancel(); // Retained weak handle after closure is safe.
    assert(*copy.id() == id);
    assert(copy.detail() == first->detail());
    auto absent = client.decide("Ready?", {"evidence"}).next();
    assert(absent && absent->id() && *absent->id() == 1);
    assert(!absent->error());
    for (const auto& row : client.decide("Ready?", {"alpha", "beta"})) {
        assert(row.answer());
        break; // Temporary stream cancels and releases on loop exit.
    }
    std::size_t calls = 0;
    tt::record_source source = [&]() -> std::optional<json> {
        if (calls++ == 0) return "evidence";
        return std::nullopt;
    };
    std::size_t count = 0;
    for (const auto& row : client.decide("Ready?", source)) { assert(row.answer()); ++count; }
    assert(count == 1 && calls == 2);
    tt::record_source throwing = []() -> std::optional<json> { throw std::domain_error("source"); };
    try { client.decide("Ready?", throwing).next(); assert(false); }
    catch (const std::domain_error& error) { assert(std::string(error.what()) == "source"); }
    try {
        client.decide("Ready?", {"evidence"}, {{"threshold", "broken"}});
        assert(false);
    } catch (const tt::error& error) { assert(error.code() == "usage.threshold"); }
    auto empty = client.decide("Ready?", tt::records{});
    assert(!empty.next());
    assert(empty.begin() == empty.end());
}
static void backend(const char* url, bool cancel) {
    tt::client client(json{{"url", url}, {"batch", 1}});
    auto rows = client.decide("Ready?", {"evidence"}, json::object());
    if (!cancel) {
        auto row = rows.next();
        assert(row && row->status() == tt::status::failed);
        assert(!row->answer() && !row->probability() && !row->detail());
        assert(row->error() && row->error()->code() == "backend.transport");
        try { rows.next(); assert(false); }
        catch (const tt::error& error) { assert(error.code() == "backend.transport"); }
        return;
    }
    auto stop = rows.cancel_handle();
    std::exception_ptr failure;
    std::thread worker([&] {
        try {
            std::size_t count = 0;
            while (auto row = rows.next()) {
                assert(row->error() && row->error()->code() == "local.cancelled");
                assert(++count <= 1);
            }
        }
        catch (...) { failure = std::current_exception(); }
    });
    // The parent sends this byte after the held fixture observes the request.
    assert(std::cin.get() == 'c');
    stop.request_cancel();
    worker.join();
    assert(failure);
    try { std::rethrow_exception(failure); }
    catch (const tt::error& error) { assert(error.code() == "local.cancelled"); }
}
static void strip_error(json& value) {
    if (value.contains("error")) value["error"].erase("message");
}
static json conformance(const char* path, const char* url) {
    auto test = read(path);
    json result = json::array();
    try {
        auto settings = test.value("client", json::object());
        settings["url"] = url;
        tt::client client(settings);
        auto rows = client.open(test.at("request"));
        bool cancel = test.value("cancelled", false);
        if (cancel) rows.cancel_handle().request_cancel();
        try {
            for (const auto& row : rows) {
                auto value = encode(row);
                strip_error(value);
                if (test["request"]["function"] == "annotate" && value.contains("answer")
                    && value["answer"].is_object())
                    for (auto& entry : value["answer"]) strip_error(entry);
                result.push_back(std::move(value));
            }
        } catch (const tt::error& error) {
            if (cancel) assert(error.code() == "local.cancelled" && result.empty());
            throw;
        }
        return result;
    } catch (const tt::error& error) {
        if (test.contains("expected_rows")) assert(result == test.at("expected_rows"));
        return {{"error", error.code()}};
    }
}
int main(int argc, char** argv) {
    assert(argc >= 3);
    std::string mode = argv[1];
    if (mode == "stub") {
        construction(argv[2]); boundaries(); typed_answers();
        std::cout << "candidate C++: ten shared stub cases and boundaries pass\n";
    }
    else if (mode == "options") options(argv[2]);
    else if (mode == "backend" || mode == "cancel") backend(argv[2], mode == "cancel");
    else { assert(mode == "case" && argc == 4); std::cout << conformance(argv[2], argv[3]) << '\n'; }
}
