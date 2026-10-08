/* Sticky failure snapshots are owned, safe, typed and leave outputs unchanged. */
#include <thinkthen.h>
#include <assert.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#define STR(s) (thinkthen_string_v1){s,sizeof(s)-1}
#define TEXT(s) (thinkthen_content_v1){1,STR(s)}
int main(void) {
    thinkthen_engine *e=thinkthen_engine_new_with("{\"cache\":false,\"model\":\"jev-latest\",\"max_retries\":0}"); assert(e);
    thinkthen_result *snapshot=(thinkthen_result *)(uintptr_t)17;
    assert(thinkthen_error_complete(e,&snapshot)==0 && snapshot==NULL);
    thinkthen_question_spec_v1 spec={0}; spec.kind=1; spec.text=TEXT("Need attention?");
    thinkthen_question *q=NULL; assert(thinkthen_question_new(e,&spec,&q)==0);
    thinkthen_record_v1 record={0}; record.original=(thinkthen_optional_content_v1){1,TEXT("evidence")};
    thinkthen_source *source=NULL; assert(thinkthen_source_records(e,&record,1,&source)==0);
    thinkthen_result *result=(thinkthen_result *)(uintptr_t)23;
    thinkthen_controls_v1 controls={0}; controls.deadline_ms=-1; controls.attempts=1;
    const char *mode=getenv("TYPED_FAILURE"); assert(mode);
    int expected=atoi(mode);
    if(expected==THINKTHEN_EUSAGE) controls.attempts=2;
    thinkthen_cancel_token *cancel=NULL;
    if(expected==THINKTHEN_ECANCELLED) { cancel=thinkthen_cancel_token_new(); assert(cancel); thinkthen_cancel(cancel); controls.cancel=cancel; }
    if(expected==THINKTHEN_EDEADLINE) controls.deadline_ms=0;
    int rc=thinkthen_decide_complete(e,q,source,&controls,&result);
    assert(rc==expected && result==(thinkthen_result *)(uintptr_t)23);
    assert(thinkthen_error_code(e)==expected);
    assert(thinkthen_error_complete(e,NULL)==THINKTHEN_EUSAGE && thinkthen_error_code(e)==expected);
    assert(thinkthen_error_complete(e,&snapshot)==0 && snapshot);
    thinkthen_summary_v1 summary={0}; assert(thinkthen_result_summary(snapshot,&summary)==0);
    assert(summary.state==THINKTHEN_RESULT_FAILURE_V1 && summary.count==0 && summary.observation_count==0);
    assert(summary.schema.len==0 && summary.schema.data==NULL && !summary.function.present && !summary.answer_id.present && !summary.meta.present);
    assert(summary.error.present && summary.error.value.code==expected && summary.error.value.message.len>0);
    assert(summary.error.value.retryable==thinkthen_error_retryable(e));
    if(expected==THINKTHEN_EUSAGE || expected==THINKTHEN_ECANCELLED) {
        assert(!summary.facts.present && !summary.attempts.present);
        if(expected==THINKTHEN_EUSAGE) assert(!summary.error.value.stopped.present);
        else assert(summary.error.value.stopped.present && summary.error.value.stopped.value.cause==THINKTHEN_STOP_CANCELLED_V1);
    }
    else {
        assert(summary.facts.present && summary.facts.value.call_id.len==64 && summary.attempts.present);
        assert(summary.error.value.stopped.present);
        assert(summary.error.value.stopped.value.cause==(expected==2 ? 5u : expected==3 ? 11u : 9u));
        assert(summary.facts.value.requests_sent==(expected==2 ? 1:0));
        assert(summary.attempts.value.len==summary.facts.value.requests_sent);
        if(expected==2) assert(summary.error.value.stopped.value.status.present && summary.error.value.stopped.value.status.value==401);
    }
    assert(thinkthen_decide_complete(e,NULL,source,NULL,&result)==THINKTHEN_EUSAGE);
    thinkthen_question_free(q); thinkthen_source_free(source); thinkthen_cancel_token_free(cancel); thinkthen_engine_free(e);
    /* Existing bytes remain alive after sticky replacement and engine destruction. */
    assert(summary.error.value.code==expected && summary.error.value.message.len>0);
    thinkthen_result_free(snapshot);
    return 0;
}
