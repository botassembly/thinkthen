#include <thinkthen/results_generated.hpp>
#include <cassert>
using namespace tt;
using namespace tt::results;
static SessionPacket packet(const char* value) { return SessionPacket(Json::parse(value)); }
static SessionPacket judgment(const char* kind, Json value) {
    return SessionPacket(Json{{"kind","observation"},{"value",Json{{"kind","row"},{"value",Json{{"kind","judgment"},{"value",Json{{"kind",kind},{"value",std::move(value)}}}}}}}});
}
int main() {
    const auto terminal=packet(R"({"kind":"terminal"})");
    auto decision=judgment("decision",false);
    assert(read_decide({decision,terminal})==false);
    assert(!read_decide({judgment("decision",nullptr),terminal}));
    assert(!read_choose({judgment("choice",nullptr),terminal}));
    assert(read_choose({judgment("choice","billing"),terminal})=="billing");
    assert(read_score({judgment("score",0.125),terminal})==0.125);
    assert(read_tag({judgment("tags",Json::Array{"urgent","billing"}),terminal})==std::vector<std::string>({"urgent","billing"}));
    for(const auto& invalid:std::vector<std::vector<SessionPacket>>{
            {terminal}, {decision,decision,terminal}, {decision},
            {decision,terminal,terminal}, {decision,packet(R"({"kind":"terminal","failure":null})")},
            {decision,packet(R"({"kind":"terminal","failure":{}})")}}) {
        bool refused=false;
        try { read_decide(invalid); } catch(const AnswerCardinalityFailure& failure) {
            refused=true; assert(failure.packets.size()==invalid.size());
        }
        assert(refused);
    }
    auto row=packet(R"({"kind":"row","function":"filter","value":{"input":"kept","index":2,"value":true,"extension":17}})");
    auto filtered=read_filter({row,terminal});
    assert(filtered.size()==1 && filtered[0].document().at("extension").get<int>()==17);
    assert(read_filter({terminal}).empty());
    auto annotated=read_annotate({packet(R"({"kind":"row","function":"annotate","value":{"input":"note","value":{"a":null},"extension":false}})"),terminal});
    assert(annotated.size()==1 && !annotated[0].document().at("extension").get<bool>());
    SessionRelationEdge edge(Json{{"source",Json{{"name","one"}}},{"target",Json{{"name","two"}}}});
    assert(edge.source_name()=="one" && edge.target_name()=="two");
}
