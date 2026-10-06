/* Native null, authored, empty, failed, source and concurrency states. */
#include <thinkthen.h>
#include "platform.h"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define STR(s) (thinkthen_string_v1){s, sizeof(s)-1}
#define TEXT(s) (thinkthen_content_v1){THINKTHEN_CONTENT_TEXT_V1, STR(s)}
static thinkthen_question *question(thinkthen_engine *e, const char *meaning, unsigned content) {
    thinkthen_question_spec_v1 s={0}; s.kind=1; s.text=TEXT("Is it relevant?");
    if(meaning) { s.yes.present=1; s.yes.value.kind=content; s.yes.value.data=(thinkthen_string_v1){meaning,strlen(meaning)}; }
    else { s.threshold.kind=THINKTHEN_RULE_BAND_V1; s.threshold.low=0.2; s.threshold.high=0.8; }
    thinkthen_question *q=NULL; assert(thinkthen_question_new(e,&s,&q)==0); return q;
}
static thinkthen_source *source(thinkthen_engine *e, int empty) {
    thinkthen_record_v1 r={0}; r.original.present=1; r.original.value=TEXT("evidence");
    thinkthen_source *s=NULL; assert(thinkthen_source_records(e,empty ? NULL : &r,empty ? 0 : 1,&s)==0); return s;
}
static int equal(thinkthen_string_v1 s,const char *value) { return s.len==strlen(value) && (s.len==0 || memcmp(s.data,value,s.len)==0); }
static void states(thinkthen_engine *e,int null) {
    const char *meaning=null ? NULL : "granted";
    thinkthen_question *q=question(e,meaning,THINKTHEN_CONTENT_TEXT_V1);
    thinkthen_source *s=source(e,0);
    thinkthen_result *r=NULL;
    assert(thinkthen_decide_current(e,q,s,NULL,&r)==0);
    thinkthen_current_atomic_v1 v;
    assert(thinkthen_result_current_atomic(r,0,&v)==0);
    assert(v.value.decide_kind==(null ? THINKTHEN_DECIDE_NULL_V1 : THINKTHEN_DECIDE_AUTHORED_V1));
    assert(v.value.boolean==0 && v.value.authored.present==!null);
    if(!null) assert(v.value.authored.value.kind==THINKTHEN_CONTENT_TEXT_V1 && equal(v.value.authored.value.data,"granted"));
    thinkthen_current_atomic_v1 sentinel={0}; sentinel.value.kind=79;
    assert(thinkthen_result_current_atomic(r,1,&sentinel)==THINKTHEN_EUSAGE && sentinel.value.kind==79);
    thinkthen_current_row_v1 wrong={0}; wrong.index.value=17;
    assert(thinkthen_result_current_rank(r,0,&wrong)==THINKTHEN_EUSAGE && wrong.index.value==17);
    assert(thinkthen_result_current_atomic(r,0,NULL)==THINKTHEN_EUSAGE);
    assert(thinkthen_error_code(e)==THINKTHEN_OK);
    thinkthen_result_free(r);
    if(!null) {
        thinkthen_question_free(q); q=question(e,"{\"result\":true}",THINKTHEN_CONTENT_JSON_V1);
        assert(thinkthen_decide_current(e,q,s,NULL,&r)==0);
        assert(thinkthen_result_current_atomic(r,0,&v)==0 && v.value.decide_kind==THINKTHEN_DECIDE_AUTHORED_V1);
        assert(v.value.authored.value.kind==THINKTHEN_CONTENT_JSON_V1 && equal(v.value.authored.value.data,"{\"result\":true}"));
        thinkthen_result_free(r);
    }
    thinkthen_source_free(s); s=source(e,1);
    thinkthen_controls_v1 c={0}; c.deadline_ms=-1; c.attempts=1;
    assert(thinkthen_decide_current(e,q,s,&c,&r)==0);
    thinkthen_current_summary_v1 summary;
    assert(thinkthen_result_current_summary(r,&summary)==0);
    assert(summary.count==0 && summary.question_count==0 && summary.facts.records==0 && summary.facts.requests_sent==0);
    assert(summary.attempts_present==1 && summary.attempt_count==0 && summary.facts.model.present==0);
    assert(thinkthen_result_current_atomic(r,0,&v)==THINKTHEN_EUSAGE);
    thinkthen_result_free(r); thinkthen_source_free(s); thinkthen_question_free(q); thinkthen_engine_free(e);
}
struct worker { thinkthen_engine *e; thinkthen_question *q; thinkthen_source *s; thinkthen_result *r; int code; };
static THREAD_RESULT work(void *opaque) {
    struct worker *w=opaque;
    /* Each caller gets its own owned result over immutable shared handles. */
    w->code=thinkthen_decide_current(w->e,w->q,w->s,NULL,&w->r);
    return THREAD_DONE;
}
static void concurrent(thinkthen_engine *e) {
    thinkthen_question *q=question(e,"granted",1); thinkthen_source *s=source(e,0);
    struct worker a={e,q,s,NULL,9},b={e,q,s,NULL,9}; THREAD_TYPE first,second;
    assert(fixture_start(&first,work,&a)==0); assert(fixture_start(&second,work,&b)==0);
    if(fixture_join(first)!=0 || fixture_join(second)!=0) _Exit(1);
    assert(a.code==0 && b.code==0);
    thinkthen_question_free(q); thinkthen_source_free(s); thinkthen_engine_free(e);
    thinkthen_current_atomic_v1 av,bv;
    assert(thinkthen_result_current_atomic(a.r,0,&av)==0 && thinkthen_result_current_atomic(b.r,0,&bv)==0);
    thinkthen_result_free(a.r);
    assert(bv.value.decide_kind==THINKTHEN_DECIDE_AUTHORED_V1 && equal(bv.value.authored.value.data,"granted"));
    thinkthen_result_free(b.r);
}
static void failure(thinkthen_engine *e) {
    thinkthen_question *q=question(e,"granted",1); thinkthen_source *s=source(e,0);
    thinkthen_result *out=(thinkthen_result *)(uintptr_t)17;
    assert(thinkthen_decide_current(e,q,s,NULL,&out)==THINKTHEN_EBACKEND && out==(thinkthen_result *)(uintptr_t)17);
    assert(thinkthen_error_code(e)==THINKTHEN_EBACKEND && thinkthen_error_retryable(e)==0);
    const char *facts=thinkthen_error_facts_json(e);
    assert(facts && strstr(facts,"\"requests_sent\":1") && strstr(facts,"\"records\":0"));
    thinkthen_current_summary_v1 summary={0}; summary.count=29;
    assert(thinkthen_result_current_summary(NULL,&summary)==THINKTHEN_EUSAGE && summary.count==29);
    assert(thinkthen_error_code(e)==THINKTHEN_EBACKEND && facts==thinkthen_error_facts_json(e));
    thinkthen_source_free(s); thinkthen_question_free(q); thinkthen_engine_free(e);
}
static void loaded(thinkthen_engine *e) {
    const char *path=getenv("TYPED_QUESTION"); assert(path);
    thinkthen_question *q=NULL;
    assert(thinkthen_question_load(e,(thinkthen_string_v1){path,strlen(path)},&q)==0);
    thinkthen_source *s=source(e,0); thinkthen_result *r=NULL;
    assert(thinkthen_annotate_current(e,q,s,NULL,&r)==0);
    thinkthen_current_annotation_v1 a;
    assert(thinkthen_result_current_annotate(r,0,&a)==0 && a.members.len==2);
    assert(equal(a.members.data[0].name,"contract") && equal(a.members.data[1].name,"urgent"));
    thinkthen_result_free(r); thinkthen_source_free(s); thinkthen_question_free(q); thinkthen_engine_free(e);
}
static void replay(void) {
    char settings[8192]; assert(fgets(settings,sizeof settings,stdin));
    thinkthen_engine *first=thinkthen_engine_new_with(settings); assert(first);
    thinkthen_question *q=question(first,"granted",1); thinkthen_source *s=source(first,0);
    thinkthen_controls_v1 c={0}; c.deadline_ms=-1; c.attempts=1;
    thinkthen_result *live=NULL,*saved=NULL; assert(thinkthen_decide_current(first,q,s,&c,&live)==0);
    thinkthen_engine_free(first);
    assert(fgets(settings,sizeof settings,stdin));
    thinkthen_engine *second=thinkthen_engine_new_with(settings); assert(second);
    assert(thinkthen_decide_current(second,q,s,&c,&saved)==0);
    thinkthen_engine_free(second); thinkthen_question_free(q); thinkthen_source_free(s);
    thinkthen_current_summary_v1 a,b;
    assert(thinkthen_result_current_summary(live,&a)==0 && thinkthen_result_current_summary(saved,&b)==0);
    assert(a.facts.requests_sent==1 && a.attempt_count==1 && b.facts.requests_sent==0 && b.attempts_present==1 && b.attempt_count==0);
    thinkthen_current_atomic_v1 x,y;
    assert(thinkthen_result_current_atomic(live,0,&x)==0 && thinkthen_result_current_atomic(saved,0,&y)==0);
    assert(x.meta.cached==0 && y.meta.cached==1 && x.meta.requests.len==1 && y.meta.requests.len==1);
    assert(x.meta.requests.data[0].len==y.meta.requests.data[0].len && memcmp(x.meta.requests.data[0].data,y.meta.requests.data[0].data,x.meta.requests.data[0].len)==0);
    assert(x.value.decide_kind==2 && y.value.decide_kind==2 && equal(y.value.authored.value.data,"granted"));
    thinkthen_result_free(live); thinkthen_result_free(saved);
}
static void member_failure(thinkthen_engine *e) {
    thinkthen_question *first=question(e,"granted",1);
    thinkthen_question_spec_v1 choice={0}; choice.kind=2; choice.text=TEXT("Which?");
    thinkthen_choice_v1 options[2]={0}; options[0].name=STR("yes"); options[1].name=STR("no"); choice.choices=(thinkthen_choices_v1){options,2};
    thinkthen_question *second=NULL; assert(thinkthen_question_new(e,&choice,&second)==0);
    thinkthen_member_spec_v1 members[]={{STR("first"),first},{STR("second"),second}};
    thinkthen_question_spec_v1 spec={0}; spec.kind=8; spec.members=(thinkthen_member_specs_v1){members,2};
    thinkthen_question *set=NULL; assert(thinkthen_question_new(e,&spec,&set)==0);
    thinkthen_question_free(first); thinkthen_question_free(second);
    thinkthen_source *s=source(e,0); thinkthen_result *r=NULL;
    assert(thinkthen_annotate_current(e,set,s,NULL,&r)==0);
    thinkthen_current_annotation_v1 v; assert(thinkthen_result_current_annotate(r,0,&v)==0);
    assert(v.members.len==2 && v.members.data[0].state==THINKTHEN_MEMBER_SUCCESS_V1 && v.members.data[0].value.decide_kind==2);
    assert(equal(v.members.data[0].value.authored.value.data,"granted"));
    assert(v.members.data[1].state==THINKTHEN_MEMBER_FAILURE_V1 && v.members.data[1].failure==THINKTHEN_FAILURE_MISSING_PROBABILITY_V1);
    assert(v.members.data[1].value.kind==0 && v.members.data[1].value.choice.present==0);
    thinkthen_current_question_v1 observed; assert(thinkthen_result_current_question(r,1,&observed)==0);
    assert(observed.state==THINKTHEN_MEMBER_FAILURE_V1 && observed.failure==THINKTHEN_FAILURE_MISSING_PROBABILITY_V1 && observed.failed_questions==1);
    assert(observed.value.kind==0 && observed.probabilities.len==0 && observed.confidence.present==0);
    thinkthen_result_free(r); thinkthen_question_free(set); thinkthen_source_free(s); thinkthen_engine_free(e);
}
int main(void) {
    binary_streams(); int mode=getchar();
    if(mode=='P') { assert(getchar()=='\n'); replay(); return 0; }
    thinkthen_engine *e=thinkthen_engine_new_with("{\"cache\":false,\"max_retries\":0}"); assert(e);
    if(mode=='M') member_failure(e); else if(mode=='C') concurrent(e); else if(mode=='E') failure(e); else if(mode=='L') loaded(e); else states(e,mode=='N');
    return 0;
}
