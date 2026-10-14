#include <thinkthen/results_generated.hpp>
#include <atomic>
#include <cassert>
#include <thread>
#include <iostream>
int main() {
    using namespace tt;
    using namespace tt::results;
    Data::Array ranks, names;
    for(unsigned i=0;i<64;++i) {
        ranks.emplace_back(Data{{"input","one"},{"value",1}});
        names.emplace_back(Data{{"input","Paris"},{"value",Data{{"entities",Data::Array{Data{{"text","Paris"},{"kind","place"},{"start",0},{"end",5},{"length",5},{"strength",0.9}}}}}}});
    }
    SessionPacket rank(Data{{"kind","aggregate"},{"function","rank"},{"value",Data(std::move(ranks))}});
    SessionPacket recognize(Data{{"kind","aggregate"},{"function","recognize"},{"value",Data(std::move(names))}});
    auto ranked=rank.as_SessionPacketRankAggregate();
    auto found=recognize.as_SessionPacketRecognizeAggregate();
    std::atomic<unsigned> ready{0};
    std::atomic<bool> go{false}, valid{true};
    std::atomic<const void*> rank_owner{nullptr}, recognize_owner{nullptr};
    auto keep_owner=[&](std::atomic<const void*>& owner,const void* address) {
        const void* first=nullptr;
        if(!owner.compare_exchange_strong(first,address) && first!=address) valid=false;
    };
    std::vector<std::thread> readers;
    for(unsigned thread=0;thread<16;++thread) readers.emplace_back([&] {
        ++ready;
        while(!go.load()) std::this_thread::yield();
        for(unsigned repeat=0;repeat<2;++repeat) {
            const auto& ordered=ranked->value();
            keep_owner(rank_owner,&ordered);
            for(const auto& row:*ordered.value) if(*row.value().value!=1) valid=false;
            const auto& recognized=found->value();
            keep_owner(recognize_owner,&recognized);
            for(const auto& row:*recognized.value) {
                auto entities=row.value().value->as_RecognizeEntities();
                for(const auto& entity:*entities->entities().value) if(*entity.text().value!="Paris") valid=false;
            }
            auto copied_rank=*ranked;
            auto copied_recognize=*found;
            if(copied_rank.value().value->size()!=64 || copied_recognize.value().value->size()!=64) valid=false;
        }
    });
    while(ready.load()!=16) std::this_thread::yield();
    go=true;
    for(auto& reader:readers) reader.join();
    assert(valid && "CONCURRENT_CACHE_CHANGED_OWNER");
    std::cout<<"concurrent-result-pass\n";
}
