#include <thinkthen/thinkthen.hpp>
#include <cassert>
#include <sstream>
#include <type_traits>
namespace tt = thinkthen;
static_assert(std::is_base_of_v<std::system_error, tt::error>);
static_assert(std::is_base_of_v<std::exception, tt::unsure>);
static_assert(!std::is_base_of_v<tt::error, tt::unsure>);
static_assert(std::is_same_v<decltype(std::declval<tt::row<tt::choose<>>>().answer()), const std::string&>);
static_assert(std::is_same_v<decltype(std::declval<tt::client>().filter_each(tt::filter{"Ready?"}, std::vector<std::string>{})), tt::kept_stream<std::string>>);
int main() {
    tt::settings config; config.backend = tt::backend::stub;
    tt::client client{config};
    const auto snapshot = [&config] { tt::client temporary{config}; return temporary.usage(); }();
    assert(snapshot.attempts == 0 && snapshot.active_calls == 0);
    assert(snapshot.input_tokens.reported == 0 && snapshot.output_tokens.unknown_attempts == 0);
    assert(client.usage().responses == 0);
    const auto question = tt::decide{"Ready?"}.threshold(tt::threshold::band(0.2, 0.8)).yes_means("ready").no_means("not ready");
    bool saw_no = false, saw_unsure = false;
    for (int i = 0; i < 300 && !(saw_no && saw_unsure); ++i) {
        const auto decision = client.decide(question, "Message " + std::to_string(i));
        if (decision.unsure()) {
            try { (void)decision.answer(); assert(false); }
            catch (const tt::unsure& unsure) { assert(unsure.probability() == decision.probability()); }
            saw_unsure = true;
        } else if (!decision.answer()) { assert(!decision); saw_no = true; }
    }
    assert(saw_no && saw_unsure);
    static_assert(std::is_base_of_v<tt::error, tt::failed>);
    static_assert(std::is_base_of_v<tt::no_answer, tt::failed>);
    const auto blank = [&](auto operation) {
        const auto before = client.usage().attempts;
        try { operation(); assert(false); }
        catch (const tt::failed& error) {
            assert(error.code() == tt::errc::record_empty);
            assert(error.family() == "record" && !error.http_status());
            assert(std::string(error.what()).starts_with("record.empty: "));
        }
        assert(client.usage().attempts == before);
        assert(client.usage().active_calls == 0);
    };
    blank([&]{(void)client.decide(question," \t");});
    blank([&]{(void)client.choose(tt::choose<>{"Q",{"a","b"}}," ");});
    blank([&]{(void)client.tag(tt::tag<>{"Q",{"a","b"}}," ");});
    blank([&]{(void)client.score(tt::score<>{"Q",{"a","b"}}," ");});
    blank([&]{(void)client.find(tt::find{"Q"}," ");});
    blank([&]{(void)client.recognize(tt::recognize{{"person"}}," ");});
    blank([&]{(void)client.relate(tt::relate{{tt::rule{.source="person",.target="person",.relation="knows"}}}," ");});
    blank([&]{(void)client.annotate(tt::annotate{{"ready",question}}," ");});
    auto failed_rows = client.decide_each(question,std::vector<std::string>{" \t"});
    const auto failed_row = failed_rows.next();
    assert(failed_row && failed_row->failed() && failed_row->item() == " \t");
    assert(!failed_rows.next());
    try {(void)failed_row->answer();assert(false);}
    catch(const tt::failed& error){assert(error.code()==tt::errc::record_empty);}
    auto team = client.choose(tt::choose<>{"Which team?", {"billing", "shipping"}}.descriptions({{"billing", "Bills"}}), "refund");
    assert(!team.answer().empty());
    std::ostringstream printed; printed << team << team.status() << tt::backend::stub;
    assert(!printed.str().empty());
    const auto identified = client.decide(tt::decide{"Ready?"}, tt::json_text{R"({"id":18446744073709551615,"body":"evidence"})"});
    assert(std::get<std::uint64_t>(identified.id()) == UINT64_MAX);
    std::size_t kept = 0;
    try {
        for (const auto& record : client.filter_each(tt::filter{"Ready?"}.threshold(0.01),
             std::vector<tt::owned_record>{tt::missing_record{}, tt::owned_text{"evidence"}, tt::missing_record{}})) {
            assert(std::get<tt::owned_text>(record).value == "evidence"); ++kept;
        }
        assert(false);
    } catch (const tt::filter_error<tt::record>& error) {
        assert(kept == 1 && error.code() == tt::errc::record_missing);
        assert(error.rows().size() == 2);
        assert(error.rows()[0].position() == 1 && error.rows()[1].position() == 3);
    }
    const std::vector<std::string> priorities{"A routine message", "Urgent: call now"};
    std::size_t ranked_count = 0;
    for (const auto& ranked : client.rank_each(tt::rank<>{"How urgent?", {"low", "high"}}, priorities)) {
        assert(ranked.position() >= 1 && ranked.position() <= priorities.size());
        assert(ranked.item() == priorities[ranked.position() - 1]);
        assert(ranked.answer() >= 1 && ranked.answer() <= priorities.size());
        ++ranked_count;
    }
    assert(ranked_count == priorities.size());
    std::vector<std::string_view> inputs{"alpha", "beta"};
    std::size_t count = 0;
    for (const auto& decision : client.decide_each(tt::decide{"Ready?"}, inputs.begin(), inputs.end())) { assert(decision.answered()); ++count; }
    assert(count == 2);
}
