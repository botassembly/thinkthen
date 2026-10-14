#include <thinkthen/results_generated.hpp>
#include <cassert>
using namespace tt;
using namespace tt::results;
static SessionPacket packet(const char* value) { return ::tt::results::detail::Loader::input<SessionPacket>(::tt::detail::parse_original(value)); }
int main() {
    const auto terminal=packet(R"({"kind":"terminal","facts":{}})");
    auto observation=packet(R"({"kind":"observation","value":{"kind":"row","index":0,"value":{"kind":"judgment","value":{"kind":"decision","value":false}}}})");
    auto authored=packet(R"({"kind":"aggregate","function":"decide","value":[{"index":0,"value":null,"answer":{"probability":0.1}}]})");
    auto result=::tt::results::detail::project_decide<true>({observation,authored,terminal});
    assert(result.value.is_null());
    assert(result.probability==0.1);
    for(const auto& invalid:std::vector<std::vector<SessionPacket>>{
            {terminal}, {observation,authored,authored,terminal}, {observation,authored},
            {observation,authored,terminal,terminal}, {observation,authored,packet(R"({"kind":"terminal","failure":null})")}}) {
        bool refused=false;
        try { ::tt::results::detail::project_decide<true>(invalid); } catch(const tt::Failure& failure) {
            refused=true; assert(failure.packets.size()==invalid.size());
        }
        assert(refused);
    }
    auto edge=::tt::results::detail::Loader::input<SessionRelationEdge>(Data{{"source",Data{{"name","one"}}},{"target",Data{{"name","two"}}}});
    assert(edge.source_name()=="one" && edge.target_name()=="two");
}
