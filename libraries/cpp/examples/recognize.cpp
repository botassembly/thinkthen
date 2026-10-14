#include <thinkthen/client.hpp>
#include <fstream>
#include <iostream>
using namespace tt::inputs;
int main(int argc,char** argv) {
    if(argc!=4) return 2;
    std::ifstream file(argv[2]);
    auto original=tt::Data::read(file);
    auto question=RequestQuestion(RequestQuestionDefinition().set_value(
        RequestDefinitionRecognizeDefinition().set_recognize(RecognizeQuestion().set_kinds({{"person",AuthoredDescription(std::string("A person's name."))}}))
        .set_threshold(AuthoredCut(0.5)).set_relation_threshold(AuthoredCut(0.5))));
    auto input=RequestInputText().set_text(original.get<std::string>());
    tt::Client client(EngineSettings().set_replay(argv[3]));
    auto answer=client.recognize(question,input).result();
    std::cout<<::tt::detail::serialize(tt::Data{{"value",tt::results::detail::encode(answer.value)},{"probability",tt::results::detail::encode(answer.probability)},{"results",tt::results::detail::encode(answer.results)},{"facts",tt::results::detail::encode(answer.facts)}})<<'\n';
}
