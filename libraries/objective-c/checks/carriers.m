#include "TTComplete.h"
#include "TTJSON.h"
#include <assert.h>
#include <string.h>
int main(void) {
    const TTCText qtext = {"α\r\nquestion", sizeof("α\r\nquestion")-1};
    TTCQuestion q = ttc_question(TTCFunctionChoose, (TTCContent){TTCContentKindText,qtext});
    TTCChoice choices[] = {{{"second",6},{true,{TTCContentKindJson,{"{\"detail\":false}",16}}},{true,0}},{{"first",5},{0},{0}}};
    uint8_t bytes[] = {0,255,1};
    TTCImageInput images[] = {{TTCMediaPng,{bytes,3},{true,{"one.png",7}}},{TTCMediaPng,{bytes,3},{true,{"one.png",7}}}};
    TTCRecordInput records[] = {{{0},{true,{TTCContentKindText,{"context",7}}},{choices,2},{images,2}}};
    TTCInputSource source = ttc_records((TTCListRecordInput){records,1});
    TTCQuestionInput question = ttc_asked(&q);
    TTCCallControls controls = {0}; controls.attempts = true;
    TTCCompleteRequest (*builders[])(TTCQuestionInput,TTCInputSource,TTCCallControls) = {
        ttc_decide,ttc_choose,ttc_tag,ttc_score,ttc_filter,ttc_rank,ttc_find,ttc_annotate,ttc_recognize,ttc_relate};
    for (size_t i=0;i<10;++i) {
        TTCCompleteRequest request = builders[i](question,source,controls);
        assert(request.function == (TTCFunction)i);
        assert(request.source.records.value.data[0].images.count == 2);
        assert(request.source.records.value.data[0].images.data[1].bytes.data[1] == 255);
        assert(!request.source.records.value.data[0].original.present);
        assert(request.question.question.value->text.data.length == sizeof("α\r\nquestion")-1);
    }
    TTCText paths[] = {{"a",1},{"a",1}};
    TTCCompleteRequest file = ttc_find(ttc_question_file((TTCText){"explicit.json",13}),ttc_files((TTCFileSource){{paths,2},TTCSourceUnitWindow,2}),controls);
    assert(file.question.file.present && file.source.files.value.paths.count == 2);
    TTCNameSpan span = {1,2,{0},{true,{0}}};
    assert(!span.kinds.present && span.edges.present && span.edges.value.count == 0);
    TTCAnswerId id; memset(&id,'z',sizeof(id));
    assert(!ttc_identity("Aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",64,id.value));
    assert(id.value[0] == 'z');
    assert(ttc_identity("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",64,id.value));
    TTCAtomicAnswer answer = {0};
    assert(ttc_atomic_read(NULL,&answer,NULL,0) == false);
    const char *fixture = "{\"kind\":\"choice\",\"pick\":\"second\",\"probabilities\":{\"second\":0.7,\"first\":0.3},\"confidence\":0}";
    TTJSON *tree = tt_json_parse(fixture,strlen(fixture));
    TTCProbability probabilities[2];
    answer.kind = TTCAtomicKindScore;
    probabilities[0].value = -1;
    assert(!ttc_atomic_read(tree,&answer,probabilities,1));
    assert(answer.kind == TTCAtomicKindScore && probabilities[0].value == -1);
    assert(!ttc_atomic_read(tree,&answer,NULL,2));
    assert(ttc_atomic_read(tree,&answer,probabilities,2));
    assert(answer.kind == TTCAtomicKindChoice && answer.confidence.present && answer.confidence.value == 0);
    assert(probabilities[0].name.length == 6 && memcmp(probabilities[0].name.data,"second",6) == 0);
    tt_json_free(tree);
    TTCAtomicAnswer saved = answer;
    TTCProbability untouched[2] = {{{"untouched",9},-1},{{"untouched",9},-1}};
    fixture = "{\"kind\":\"choice\",\"pick\":\"x\",\"probabilities\":{\"x\":false}}";
    tree = tt_json_parse(fixture,strlen(fixture));
    assert(tree);
    assert(!ttc_atomic_read(tree,&answer,untouched,2));
    assert(memcmp(&saved,&answer,sizeof(answer)) == 0 && untouched[0].value == -1);
    tt_json_free(tree);
    fixture = "{\"call_id\":\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\",\"cache_answers\":0,\"estimated_cost_usd\":\"0.000001\",\"input_tokens\":0,\"records\":2,\"requests_sent\":0,\"seconds\":0}";
    tree = tt_json_parse(fixture,strlen(fixture));
    TTCCallFacts facts;
    assert(ttc_facts_read(tree,&facts));
    assert(facts.inputTokens.present && facts.inputTokens.value == 0 && !facts.outputTokens.present);
    assert(facts.estimatedCostUsd.value.length == 8 && !memcmp(facts.estimatedCostUsd.value.data,"0.000001",8));
    tt_json_free(tree);
    fixture = "{\"cache_answers\":0}";
    tree = tt_json_parse(fixture,strlen(fixture));
    TTCCallFacts savedFacts = facts;
    assert(!ttc_facts_read(tree,&facts) && !memcmp(&savedFacts,&facts,sizeof(facts)));
    tt_json_free(tree);
    fixture = "{\"call_id\":\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\",\"cache_answers\":0,\"input_tokens\":18446744073709551615,\"records\":0,\"requests_sent\":0,\"seconds\":0}";
    tree = tt_json_parse(fixture,strlen(fixture));
    assert(ttc_facts_read(tree,&facts) && facts.inputTokens.value == UINT64_MAX);
    tt_json_free(tree);
    fixture = "{\"call_id\":\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\",\"cache_answers\":0,\"input_tokens\":18446744073709551616,\"records\":0,\"requests_sent\":0,\"seconds\":0}";
    tree = tt_json_parse(fixture,strlen(fixture));
    savedFacts = facts;
    assert(!ttc_facts_read(tree,&facts) && !memcmp(&savedFacts,&facts,sizeof(facts)));
    tt_json_free(tree);
}
