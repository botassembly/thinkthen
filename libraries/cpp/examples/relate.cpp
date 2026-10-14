#include <thinkthen/client.hpp>
#include <fstream>
#include <iostream>
using namespace tt::inputs;
int main(int argc,char** argv) {
    if(argc!=4) return 2;
    std::ifstream file(argv[2]);
    auto original=tt::Data::read(file);
    auto question=RequestQuestion(RequestQuestionDefinition().set_value(
        RequestDefinitionAuthoredRelate().set_relate(RelateQuestion().set_relations({
            AuthoredRelation().set_name(AuthoredName("caused_by")).set_source(AuthoredName("alert")).set_target(AuthoredName("alert")),
            AuthoredRelation().set_name(AuthoredName("same_as")).set_source(AuthoredName("alert")).set_target(AuthoredName("alert")).set_either(true)}))
        .set_threshold(AuthoredCut(0.5))));
    std::vector<RequestItem> items;
    for(const auto& value:original) items.push_back(RequestItem().set_original(RequestOriginalJson().set_value(value)));
    auto input=RequestInput(RequestInputEntities().set_items(items));
    tt::Client client(EngineSettings().set_replay(argv[3]));
    auto answer=client.relate(question,input).result();
    std::cout<<::tt::detail::serialize(tt::Data{{"value",tt::results::detail::encode(answer.value)},{"probability",tt::results::detail::encode(answer.probability)},{"results",tt::results::detail::encode(answer.results)},{"facts",tt::results::detail::encode(answer.facts)}})<<'\n';
}
