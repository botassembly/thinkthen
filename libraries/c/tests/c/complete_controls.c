/* Typed per-record controls, strict replay, cache hits and changed readings. */
#include <thinkthen.h>
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define STR(s) (thinkthen_string_v1){s,sizeof(s)-1}
#define TEXT(s) (thinkthen_content_v1){1,STR(s)}
static int same(thinkthen_string_v1 s,const char *value) { return s.len==strlen(value) && memcmp(s.data,value,s.len)==0; }
static thinkthen_question *question(thinkthen_engine *e,unsigned kind,double cut) {
    thinkthen_question_spec_v1 spec={0}; spec.kind=kind; spec.text=TEXT("Need attention?");
    spec.threshold=(thinkthen_rule_v1){2,cut,0};
    thinkthen_question *q=NULL; assert(thinkthen_question_new(e,&spec,&q)==0); return q;
}
static thinkthen_source *source(thinkthen_engine *e,unsigned kind) {
    thinkthen_record_v1 record={0}; record.original=(thinkthen_optional_content_v1){1,TEXT("evidence")};
    record.context=(thinkthen_optional_content_v1){1,TEXT("record-context")};
    thinkthen_choice_v1 options[2]={0}; options[0].name=STR("runtime-first"); options[1].name=STR("runtime-last");
    options[0].description=(thinkthen_optional_content_v1){1,{2,STR("{\"z\":false,\"a\":null}")}};
    if(kind==2) record.options=(thinkthen_choices_v1){options,2};
    thinkthen_source *s=NULL; assert(thinkthen_source_records(e,&record,1,&s)==0); return s;
}
static thinkthen_engine *engine(const char *mode,const char *folder) {
    /* The Rust caller serializes native paths as JSON string values. */
    char settings[8192];
    int size=mode ? snprintf(settings,sizeof settings,"{\"%s\":%s,\"cache\":false,\"model\":\"fixed\"}",mode,folder)
        : snprintf(settings,sizeof settings,"{\"cache\":%s,\"model\":\"fixed\"}",folder);
    assert(size>0 && (size_t)size<sizeof settings);
    thinkthen_engine *e=thinkthen_engine_new_with(settings); assert(e); return e;
}
int main(void) {
    const char *folder=getenv("TYPED_RECORDING_JSON"); const char *cache=getenv("TYPED_CACHE_JSON"); assert(folder && cache);
    thinkthen_controls_v1 controls={0}; controls.deadline_ms=-1; controls.attempts=1;
    controls.context=(thinkthen_optional_content_v1){1,TEXT("fallback context")};
    thinkthen_engine *recorder=engine("record",folder);
    char answer[2][65],observation[2][65],callid[65];
    for(unsigned kind=1;kind<=2;++kind) {
        thinkthen_question *q=question(recorder,kind,0.5); thinkthen_source *s=source(recorder,kind); thinkthen_result *r=NULL;
        assert((kind==1?thinkthen_decide_complete:thinkthen_choose_complete)(recorder,q,s,&controls,&r)==0);
        thinkthen_summary_v1 sum={0}; assert(thinkthen_result_summary(r,&sum)==0);
        assert(sum.facts.value.requests_sent==1 && sum.meta.present && sum.answer_id.present);
        memcpy(answer[kind-1],sum.answer_id.value.data,64); answer[kind-1][64]=0;
        memcpy(observation[kind-1],sum.meta.value.observations.data[0].data.observation_id.data,64); observation[kind-1][64]=0;
        memcpy(callid,sum.facts.value.call_id.data,64); callid[64]=0;
        if(kind==2) {
            thinkthen_choose_view_v1 v={0}; assert(thinkthen_result_choose(r,0,&v)==0);
            assert(same(v.value.value,"runtime-first") && same(v.common.question.value.choices.data[0].name,"runtime-first"));
            assert(v.common.question.value.choices.data[0].description.value.kind==2);
        }
        thinkthen_result_free(r); thinkthen_question_free(q); thinkthen_source_free(s);
    }
    thinkthen_engine_free(recorder);
    thinkthen_engine *replay=engine("replay",folder);
    for(unsigned kind=1;kind<=2;++kind) for(unsigned changed=0;changed<2;++changed) {
        thinkthen_question *q=question(replay,kind,changed ? 0.95:0.5); thinkthen_source *s=source(replay,kind); thinkthen_result *r=NULL;
        assert((kind==1?thinkthen_decide_complete:thinkthen_choose_complete)(replay,q,s,&controls,&r)==0);
        thinkthen_summary_v1 sum={0}; assert(thinkthen_result_summary(r,&sum)==0);
        assert(sum.facts.value.requests_sent==0 && sum.attempts.present && sum.attempts.value.len==0);
        assert(sum.meta.value.cached && sum.meta.value.origin.value==THINKTHEN_ORIGIN_REPLAY_V1);
        assert(same(sum.meta.value.observations.data[0].data.observation_id,observation[kind-1]));
        assert(same(sum.answer_id.value,answer[kind-1])==!changed);
        assert(!same(sum.facts.value.call_id,callid));
        if(kind==1) { thinkthen_decide_view_v1 v={0}; assert(thinkthen_result_decide(r,0,&v)==0); assert(v.value.kind==1 && v.value.data.boolean==!changed && v.common.answer.value.data.probability==0.9); }
        else { thinkthen_choose_view_v1 v={0}; assert(thinkthen_result_choose(r,0,&v)==0); assert(v.value.present==!changed && same(v.common.answer.value.data.choice.pick,"runtime-first")); }
        thinkthen_result_free(r); thinkthen_question_free(q); thinkthen_source_free(s);
    }
    /* A changed effective context cannot fall back to a live request. */
    thinkthen_question *q=question(replay,1,0.5);
    thinkthen_record_v1 miss={0}; miss.original=(thinkthen_optional_content_v1){1,TEXT("evidence")};
    miss.context=(thinkthen_optional_content_v1){1,TEXT("different")};
    thinkthen_source *s=NULL; assert(thinkthen_source_records(replay,&miss,1,&s)==0);
    thinkthen_result *r=NULL; assert(thinkthen_decide_complete(replay,q,s,&controls,&r)==THINKTHEN_ELOCAL && r==NULL);
    assert(thinkthen_error_complete(replay,&r)==0 && r);
    thinkthen_summary_v1 sum={0}; assert(thinkthen_result_summary(r,&sum)==0);
    assert(sum.error.value.stopped.present && sum.error.value.stopped.value.cause==THINKTHEN_STOP_LOCAL_V1 && sum.facts.value.requests_sent==0);
    thinkthen_result_free(r); thinkthen_source_free(s); thinkthen_question_free(q); thinkthen_engine_free(replay);
    thinkthen_engine *cached=engine(NULL,cache); q=question(cached,1,0.5); s=source(cached,1);
    for(unsigned hit=0;hit<2;++hit) {
        r=NULL; assert(thinkthen_decide_complete(cached,q,s,&controls,&r)==0); assert(thinkthen_result_summary(r,&sum)==0);
        assert(sum.facts.value.requests_sent==!hit && sum.meta.value.cached==(int)hit);
        if(hit) assert(sum.meta.value.origin.value==THINKTHEN_ORIGIN_CACHE_V1 && sum.attempts.value.len==0);
        thinkthen_result_free(r);
    }
    thinkthen_source_free(s); thinkthen_question_free(q); thinkthen_engine_free(cached);
    return 0;
}
