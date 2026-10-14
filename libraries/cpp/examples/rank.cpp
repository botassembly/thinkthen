#include <thinkthen/client.hpp>
#include <fstream>
#include <iostream>
using namespace tt::inputs;
int main(int argc,char** argv) {
    if(argc!=4) return 2;
    std::ifstream file(argv[2]);
    auto original=tt::Data::read(file);
    auto question=RequestQuestion(RequestQuestionFile().set_path(argv[1]));
    auto input=RequestInput(RequestInputRecords().set_items({RequestItem().set_original(RequestOriginalJson().set_value(original))}));
    tt::Client client(EngineSettings().set_replay(argv[3]));
    auto answer=client.rank(question,input).answer();
    std::cout<<::tt::detail::serialize(tt::results::detail::encode(answer))<<'\n';
}
