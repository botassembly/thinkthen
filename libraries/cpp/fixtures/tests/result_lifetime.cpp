#include <thinkthen/results_generated.hpp>
#include <cassert>
int main() {
    using namespace tt;
    using namespace tt::results;
    auto rank=::tt::results::detail::Loader::input<SessionPacket>(::tt::detail::parse_original(R"({"kind":"aggregate","function":"rank","value":[{"input":"one","value":1}]})"));
    auto ranked=rank.as_SessionPacketRankAggregate();
    for(const auto& row:*ranked->value().value) assert(*row.value().value==1);
    auto recognize=::tt::results::detail::Loader::input<SessionPacket>(::tt::detail::parse_original(R"({"kind":"aggregate","function":"recognize","value":[{"name":"Paris","kind":"place"}]})"));
    auto found=recognize.as_SessionPacketRecognizeAggregate();
    for(const auto& entity:*found->value().value) assert(::tt::results::detail::document(entity).at("name").get<std::string>()=="Paris");
    const auto& retained=ranked->value();
    assert(&retained==&ranked->value());
    auto replacement=::tt::results::detail::Loader::input<SessionPacketRankAggregate>(::tt::detail::parse_original(R"({"kind":"aggregate","function":"rank","value":[{"input":"two","value":2}]})"));
    auto snapshot=*ranked;
    *ranked=replacement;
    assert(*ranked->value().value->at(0).value().value==2);
    assert(*retained.value->at(0).value().value==1);
    assert(&snapshot.value()==&retained);
    assert(::tt::results::detail::document(*ranked).at("value").at(0).at("value").get<int>()==2);
    auto copied=*ranked;
    *ranked=std::move(replacement);
    assert(*ranked->value().value->at(0).value().value==2);
    assert(*copied.value().value->at(0).value().value==2);
}
