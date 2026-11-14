#include <thinkthen/thinkthen.hpp>
#include <array>
#include <format>
#include <ranges>
#include <type_traits>
namespace tt = thinkthen;
enum class Topic { billing, shipping, other };
enum class Urgency { routine = 9, urgent = 1 };
template<> inline constexpr std::array<std::pair<Topic, std::string_view>, 3> thinkthen::words<Topic>{{
    {Topic::billing, "billing"}, {Topic::shipping, "shipping"}, {Topic::other, "other"}}};
template<> inline constexpr std::array<std::pair<Urgency, std::string_view>, 2> thinkthen::words<Urgency>{{
    {Urgency::routine, "routine"}, {Urgency::urgent, "urgent"}}};
struct Ticket { std::string text; };
tt::record to_record(const Ticket& ticket) { return tt::text{ticket.text}; }
static_assert(!std::is_copy_constructible_v<tt::row<tt::decide>>);
static_assert(std::is_move_constructible_v<tt::row<tt::decide>>);
static_assert(std::is_copy_constructible_v<tt::filter_error<Ticket>>);
void public_api(tt::client& client) {
    const auto asks = tt::decide{"Refund?"}.threshold(tt::threshold::band(.2, .8));
    const auto answer = client.decide(asks, "text");
    if (answer) { (void)std::format("{}", answer); }
    auto topic = client.choose(tt::choose<Topic>{"Topic?"}, "text");
    const Topic choice = topic.answer(); (void)choice;
    auto tags = client.tag(tt::tag<Topic>{"Topics?"}, "text");
    (void)tags.answer().contains(Topic::billing);
    auto score = client.score(tt::score<Urgency>{"Urgency?"}, "text"); (void)score.answer();
    std::vector<Ticket> records{{"first"}, {"second"}};
    for (auto& row : client.decide_each(asks, records)) { (void)row.item(); auto kept = std::move(row); }
    for (const auto& item : client.filter_each(tt::filter{"Keep?"}, records)) { (void)item.text; }
    auto ranked = client.rank_each(tt::rank<Urgency>{"Urgency?"}, std::vector<Ticket>{{"generated"}});
    for (auto& row : ranked) { (void)row.item(); (void)row.answer(); }
    auto generated = std::views::iota(0, 2) | std::views::transform([](int i) { return Ticket{std::to_string(i)}; });
    for (auto& row : client.decide_each(asks, generated)) { (void)row.item(); }
    const tt::annotate form{{"refund", asks}, {"topic", tt::choose<Topic>{"Topic?"}}};
    auto result = client.annotate(form, "text");
    const auto& child = tt::get<tt::choose<Topic>>(result.answer(), "topic"); (void)child.answer();
    auto saved = tt::toolbox::from_json("{}"); (void)saved.select<tt::choose<Topic>>("topic");
    const tt::rule relation{.source = "person", .target = "organization", .relation = "works_for"};
    auto pairs = client.relate(tt::relate{{relation}}, "text"); (void)pairs.answer();
    std::stop_source stop; tt::client cancelled{tt::settings{.stop = stop.get_token()}};
    (void)cancelled;
}
