#include <thinkthen/results_generated.hpp>
#include <cassert>
int main() {
    using namespace tt;
    using namespace tt::results;
    SessionPacket rank(Json::parse(R"({"kind":"aggregate","function":"rank","value":[{"input":"one","value":1}]})"));
    auto ranked=rank.as_SessionPacketRankAggregate();
    for(const auto& row:*ranked->value().value) assert(*row.value().value==1);
    SessionPacket recognize(Json::parse(R"({"kind":"aggregate","function":"recognize","value":[{"name":"Paris","kind":"place"}]})"));
    auto found=recognize.as_SessionPacketRecognizeAggregate();
    for(const auto& entity:*found->value().value) assert(entity.document().at("name").get<std::string>()=="Paris");
    const auto& retained=ranked->value();
    assert(&retained==&ranked->value());
}
