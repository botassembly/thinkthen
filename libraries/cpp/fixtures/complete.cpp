#include "thinkthen/complete.hpp"
#include <cassert>
#include <type_traits>
using namespace tt::complete;
int main() {
    static_assert(!std::is_same_v<CallId, AnswerId>);
    static_assert(!std::is_default_constructible_v<AnswerId>);
    auto q = asked(Function::choose, text("α\r\nquestion"));
    q.choices = {{"second", json("{\"detail\":false}"), 0}, {"first", text("description"), {}}};
    auto question = QuestionInput::asked(q);
    const ImageInput image{Media::png, {0,255,1}, "one.png"};
    RecordInput r{{}, text("context"), q.choices, {image,image}};
    auto source = InputSource::from_records({r,r});
    using Builder = CompleteRequest (*)(QuestionInput, InputSource, CallControls);
    const Builder builders[] = {Requests::decide, Requests::choose, Requests::tag, Requests::score,
        Requests::filter, Requests::rank, Requests::find, Requests::annotate, Requests::recognize, Requests::relate};
    for (size_t i=0; i<10; ++i) {
        auto request = builders[i](question, source, {{}, {}, false, true});
        assert(static_cast<size_t>(request.function) == i);
        auto encoded = descriptor(request);
        assert(encoded.at("source").at("records").at(0).at("original").is_null());
        assert(encoded.at("source").at("records").at(0).at("images").at(1).at("bytes").at(1).get<double>() == 255);
        assert(request.question.question->choices.at(0).name == "second");
    }
    auto prepared = Requests::choose(question, source);
    q.text.data = "changed"; source.records->at(0).images.at(0).bytes.at(0) = 9;
    assert(prepared.question.question->text.data == "α\r\nquestion");
    assert(prepared.source.records->at(0).images.at(0).bytes.at(0) == 0);
    auto file = Requests::find(QuestionInput::question_file("explicit.json"),
        InputSource::from_files({{"a","a"},SourceUnit::window,2}));
    assert(file.question.file == "explicit.json" && file.source.files->paths.size() == 2);
    auto empty = descriptor(InputSource::from_records({}));
    assert(empty.at("records").is_array() && empty.at("files").is_null());
    const DecideValue no{ValueKind::boolean,false,{}};
    const DecideValue unresolved{ValueKind::null_,false,{}};
    const DecideValue authored{ValueKind::authored,false,json("false")};
    assert(descriptor(no) != descriptor(unresolved) && descriptor(authored) != descriptor(no));
    NameSpan span{1,2,{}, std::vector<Probability>{}};
    assert(descriptor(span).at("kinds").is_null() && descriptor(span).at("edges").is_array());
    CallFacts facts{CallId(std::string(64,'a')),0,"0.000001",0,{}, {},2,0,0,{}};
    assert(descriptor(facts).at("estimatedCostUsd").get<std::string>() == "0.000001");
    assert(descriptor(facts).at("inputTokens").get<double>() == 0);
    assert(descriptor(facts).at("outputTokens").is_null());
    for (const auto& bad : {std::string(64,'A'), std::string(63,'a'), std::string(65,'a')}) {
        bool refused=false; try { (void)AnswerId(bad); } catch (const std::invalid_argument&) { refused=true; }
        assert(refused);
    }
    bool refused=false; try { (void)descriptor(UINT64_MAX); } catch (const std::invalid_argument&) { refused=true; }
    assert(refused);
    AtomicAnswer answer{AtomicKind::choice,{},"second",{},{{"second",0.7},{"first",0.3}},0};
    assert(descriptor(answer).at("probabilities").at(0).at("name").get<std::string>() == "second");
    assert(descriptor(answer).at("confidence").get<double>() == 0);
    auto observed = read_facts(tt::Json::parse(R"FACTS({"call_id":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","cache_answers":0,"estimated_cost_usd":"0.000001","input_tokens":0,"records":2,"requests_sent":0,"seconds":0})FACTS"));
    assert(observed.inputTokens == 0 && !observed.outputTokens && observed.estimatedCostUsd == "0.000001");
    bool legacy=false; try { (void)read_facts(tt::Json::parse("{\"cache_answers\":0}")); } catch (const std::out_of_range&) { legacy=true; }
    assert(legacy);
    auto ordered = read_atomic(tt::Json::parse(R"ANSWER({"kind":"choice","pick":"second","probabilities":{"second":0.7,"first":0.3},"confidence":0})ANSWER"));
    assert(ordered.probabilities.at(0).name == "second" && ordered.confidence == 0);
    bool invalid=false; try { (void)read_atomic(tt::Json::parse("{\"kind\":\"yes_no\",\"probability\":false}")); } catch (const std::bad_variant_access&) { invalid=true; }
    assert(invalid);
}
