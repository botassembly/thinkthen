#include <thinkthen/nlohmann.hpp>
#include <cassert>
#include <iostream>
#include <ranges>
#include <sstream>
namespace tt=thinkthen;
using nlohmann::json;
enum class Level { routine=50, urgent=-2 };
template<> inline constexpr std::array<std::pair<Level,std::string_view>,2> thinkthen::words<Level>{{{Level::routine,"routine"},{Level::urgent,"urgent"}}};
enum class Topic { billing,shipping,other };
template<> inline constexpr std::array<std::pair<Topic,std::string_view>,3> thinkthen::words<Topic>{{{Topic::billing,"billing"},{Topic::shipping,"shipping"},{Topic::other,"other"}}};
namespace app {
struct Ticket { std::string text; std::shared_ptr<int> alive; };
tt::record to_record(const Ticket& value) {return tt::text{value.text};}
}
template<class F> void refuses(tt::errc code,F function) {
    try {function();assert(false);}
    catch(const tt::error& error){assert(error.code()==tt::make_error_code(code));}
}
int main(int argc,char** argv) {
    assert(argc==2);const std::string scenario=argv[1];
    tt::client client{tt::settings{.retry=tt::retry_policy{.max_retries=0}}};
    const auto decide=tt::decide{"Does the customer request a refund?"}.threshold(tt::threshold::band(.2,.8));
    if(scenario.starts_with("form-")) {
        auto form=client.annotate(tt::annotate{{"refund",decide},{"topic",tt::choose<Topic>{"What is the message about?"}},{"urgency",tt::score<Level>{"How urgent is the request?"}}},"Please refund my order.");
        assert(form.answered());
        const auto& entries=form.answer();
        const auto& refund=tt::get<tt::decide>(entries,"refund");
        const auto& topic=tt::get<tt::choose<Topic>>(entries,"topic");
        const auto& urgency=tt::get<tt::score<Level>>(entries,"urgency");
        const bool failed=scenario.ends_with("failed");
        assert(refund.failed()==(scenario=="form-refund-failed"));
        assert(topic.failed()==(scenario=="form-topic-failed"));
        assert(urgency.failed()==(scenario=="form-urgency-failed"));
        assert(refund.unsure()==(scenario=="form-refund-unsure"));
        assert(topic.unsure()==(scenario=="form-topic-unsure"));
        assert(!urgency.unsure());
        bool threw=false;try{(void)form.throw_if_failed();}catch(const tt::failed&){threw=true;}assert(threw==failed);
        refuses(tt::errc::usage_function,[&]{(void)tt::get<tt::choose<Topic>>(entries,"refund");});
        std::cout<<scenario<<": aggregate failure "<<std::boolalpha<<threw<<'\n';return 0;
    }
    if(scenario=="failed") {
        const auto failed_call=[&](auto operation) {
            try {operation();assert(false);}
            catch(const tt::failed& failure) {
                assert(failure.code()==tt::errc::backend_status);
                assert(failure.http_status()==503 && failure.family()=="backend");
                assert(std::string(failure.what()).starts_with("backend.status: "));
            }
            assert(client.usage().active_calls==0);
        };
        failed_call([&]{(void)client.decide(decide,"evidence");});
        failed_call([&]{(void)client.choose(tt::choose<>{"Q",{"a","b"}},"evidence");});
        failed_call([&]{(void)client.tag(tt::tag<>{"Q",{"a","b"}},"evidence");});
        failed_call([&]{(void)client.score(tt::score<>{"Q",{"a","b"}},"evidence");});
        failed_call([&]{(void)client.find(tt::find{"Q"},"evidence");});
        failed_call([&]{(void)client.recognize(tt::recognize{{"person"}},"evidence");});
        failed_call([&]{(void)client.relate(tt::relate{{tt::rule{.source="person",.target="person",.relation="knows"}}},"evidence");});
    }
    auto result_rows=client.decide_each(decide,std::vector<std::string>{"Please refund my order."});
    auto result=std::move(*result_rows.next());
    assert(!result_rows.next());
    std::ostringstream printed;printed<<result;
    assert(std::format("{}",result)==printed.str());
    auto wire=json::parse(result.to_json());
    assert(wire.at("status")== (scenario=="failed"?"failed":scenario=="unsure"?"unsure":"answered"));
    assert(json(result)==wire);
    if(scenario=="failed" || scenario=="unsure") {
        bool caught=false;try{(void)result.answer();}catch(const tt::no_answer&){caught=true;}assert(caught);
    }
    const tt::annotate form_question{{"refund",decide},{"urgency",tt::score<Level>{"How urgent is the request?"}}};
    auto form=client.annotate(form_question,"Please refund my order.");
    const auto& entries=form.answer();
    refuses(tt::errc::usage_name,[&]{(void)tt::get<tt::decide>(entries,"missing");});
    refuses(tt::errc::usage_function,[&]{(void)tt::get<tt::choose<>>(entries,"refund");});
    const auto& child=tt::get<tt::decide>(entries,"refund");
    assert(child.status()==result.status());
    const auto saved_child=json::parse(child.to_json());
    const auto saved_parent=json::parse(form.to_json());
    assert(saved_child.at("position")==child.position());
    assert(saved_child.at("source")==json(child.source()));
    assert(saved_child.at("source")==saved_parent.at("source"));
    assert(!saved_parent.at("answer").at("refund").contains("position"));
    if(scenario=="failed") {
        assert(form.answered());bool caught=false;
        try{(void)form.throw_if_failed();}catch(const tt::failed&){caught=true;}assert(caught);
    } else {(void)form.throw_if_failed();}
    if(scenario!="yes") {std::cout<<"safe state and loud answer: "<<scenario<<'\n';return 0;}
    const auto before_blank=client.usage().attempts;
    refuses(tt::errc::record_empty,[&]{(void)client.annotate(form_question,"   ");});
    assert(client.usage().attempts==before_blank && client.usage().active_calls==0);
    {
        tt::client measured;const auto answer=measured.decide(decide,"Please refund my order.");assert(answer.answered());
        const auto usage=measured.usage();assert(usage.attempts==1 && usage.responses==1);
        assert(usage.input_tokens.reported==0 && usage.output_tokens.reported==0);
        assert(usage.input_tokens.unknown_attempts==1 && usage.output_tokens.unknown_attempts==1);
    }
    // The moved-out result is the sole owner after call, source and sentinel die.
    std::optional<tt::row<tt::decide,app::Ticket>> retained;
    std::weak_ptr<int> alive;
    {
        auto token=std::make_shared<int>(42);alive=token;
        std::vector<app::Ticket> input{{"Please refund my order.",token}};token.reset();
        auto call=client.decide_each(decide,std::move(input));
        for(auto& value:call)retained.emplace(std::move(value));
    }
    assert(!alive.expired());assert(retained->item().text=="Please refund my order.");assert(retained->answer());
    retained.reset();assert(alive.expired());
    std::optional<tt::row<tt::decide,app::Ticket>> retained_failure;
    std::weak_ptr<int> failed_alive;
    {
        auto token=std::make_shared<int>(9);failed_alive=token;
        std::vector<app::Ticket> input{{"   ",token}};token.reset();
        auto call=client.decide_each(decide,std::move(input));
        for(auto& value:call)retained_failure.emplace(std::move(value));
    }
    assert(!failed_alive.expired() && retained_failure->failed());
    assert(retained_failure->item().text=="   ");
    assert(retained_failure->error()->code()==tt::errc::record_empty);
    retained_failure.reset();assert(failed_alive.expired());
    std::optional<tt::row<tt::decide,app::Ticket>> generated;
    {
        auto call=client.decide_each(decide,std::views::iota(0,1)|std::views::transform([](int){return app::Ticket{"Please refund my order.",std::make_shared<int>(7)};}));
        for(auto& value:call)generated.emplace(std::move(value));
    }
    assert(generated->item().text=="Please refund my order.");assert(generated->answer());
    std::vector<tt::row<tt::decide,std::string>> input_rows;
    {
        std::istringstream source{"first second third"};
        auto call=client.decide_each(decide,std::ranges::istream_view<std::string>(source));
        for(auto& value:call)input_rows.push_back(std::move(value));
    }
    assert(input_rows.size()==3);assert(input_rows[0].item()=="first");
    assert(input_rows[1].item()=="second");assert(input_rows[2].item()=="third");
    {
        std::istringstream source{R"({"id":"first","text":"alpha"} {"id":"other","text":"bravo"})"};
        auto views=std::ranges::istream_view<std::string>(source) |
            std::views::transform([](const std::string& value){return tt::json_text{value};});
        std::vector<std::string> ids;
        for(const auto& value:client.decide_each(decide.field("/text"),views))
            ids.push_back(std::get<std::string>(value.id()));
        assert((ids==std::vector<std::string>{"first","other"}));
    }
    for(const auto& input: {json{{"id","quoted\"id"},{"text","alpha"}},json{{"id",UINT64_MAX},{"text","alpha"}}}) {
        auto identified=client.annotate(tt::annotate{{"refund",decide.field("/text")}},input,"retained-name");
        const auto& entry=tt::get<tt::decide>(identified.answer(),"refund");
        assert(entry.id()==identified.id());assert(entry.id_json()==identified.id_json());
        assert(entry.id_json()==input["id"].dump());assert(entry.position()==identified.position());
        assert(entry.source().question_identity==identified.source().question_identity);
        assert(entry.source().backend==identified.source().backend);
        assert(entry.source().model==identified.source().model);assert(entry.source().name==identified.source().name);
        const auto saved=json::parse(entry.to_json());
        assert(saved.at("id")==input["id"] && saved.at("position")==entry.position());
        assert(saved.at("source")==json(entry.source()));
    }
    std::vector<std::string> borrowed{"Please refund my order."};
    for(auto& value:client.decide_each(decide,borrowed))assert(&value.item()==&borrowed[0]);
    const auto saved_question=[](const auto& question) {
        thinkthen_buffer bytes{};tt::detail::invoke(thinkthen_question_to_json,question.native_handle(),&bytes);
        const auto value=json::parse(bytes.data,bytes.data+bytes.len);thinkthen_buffer_free(&bytes);return value;
    };
    const json levels={"routine","urgent"};
    assert(saved_question(tt::score<Level>{"Q"})["levels"]==levels);
    assert(saved_question(tt::rank<Level>{"Q"})["levels"]==levels);
    assert(saved_question(tt::annotate{{"urgency",tt::score<Level>{"Q"}}})["questions"]["urgency"]["levels"]==levels);
    auto score=client.score(tt::score<Level>{"How urgent is the request?"},"Please refund my order.");
    assert(score.answer()==1.0); // Pair order, not enum numeric order.
    const std::string text="😀 Before.\nPlease refund my order.";
    const tt::find action_question{"Which line states the requested action?"};
    auto action=client.find(action_question,text);
    const auto start=text.find("Please");
    assert(action.answer()->start==start);assert(text.substr(action.answer()->start,action.answer()->end-action.answer()->start)==action.answer()->text);
    assert(json::parse(action.to_json())["answer"]["start"]==start-3);
    auto action_form=client.annotate(tt::annotate{{"action",action_question}},text);
    const auto& child_action=tt::get<tt::find>(action_form.answer(),"action");
    assert(child_action.answer()->start==start);assert(json::parse(child_action.to_json())["answer"]["start"]==start-3);
    std::optional<tt::filter_error<std::string>> failure;
    try {
        for(const auto& item:client.filter_each(tt::filter{"Q"},std::vector<std::string>{"   "})) (void)item;
        assert(false);
    } catch(const tt::filter_error<std::string>& error) {failure.emplace(error);}
    assert(failure->rows().size()==1);assert(failure->rows()[0].item()=="   ");assert(failure->rows()[0].failed());
    refuses(tt::errc::record_empty,[&]{(void)client.decide(decide,json::object());});
    refuses(tt::errc::usage_threshold,[&]{
        auto saved=tt::toolbox::from_json(R"({"entries":{"bad":{"question":{"function":"filter","question":"Q","threshold":{"low":0.2,"high":0.8}}}}})");
        (void)saved.select<tt::filter>("bad");
    });
    std::stop_source stop;
    auto call=[&]{tt::client temporary{tt::settings{.stop=stop.get_token()}};return temporary.decide_each(decide,std::vector<std::string>{"Please refund my order."});}();
    stop.request_stop();refuses(tt::errc::local_cancelled,[&]{(void)call.next();});
    std::cout<<"retained original records, form refusals, native print and cancellation\n";
}
