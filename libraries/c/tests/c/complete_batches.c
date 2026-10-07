/* Public lazy reader keeps typed originals, completed prefix and joined facts. */
#include <thinkthen.h>
#include <assert.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#define STR(s) (thinkthen_string_v1){s,sizeof(s)-1}
static int same(thinkthen_string_v1 s,const char *v) { return s.len==strlen(v) && !memcmp(s.data,v,s.len); }
int main(void) {
    const char *path=getenv("TYPED_JSONL"),*mode=getenv("TYPED_BATCH_MODE"); assert(path && mode);
    thinkthen_engine *e=thinkthen_engine_new_with("{\"cache\":false,\"model\":\"fixed\",\"batch\":2,\"throttle\":1,\"max_retries\":0}"); assert(e);
    thinkthen_string_v1 missing[]={STR("missing-owned-image")};
    thinkthen_source_spec_v1 bad={{missing,1},THINKTHEN_SOURCE_LINE_V1,0};
    thinkthen_source *unchanged=(thinkthen_source *)(uintptr_t)13;
    assert(thinkthen_source_image_files(e,&bad,&unchanged)==THINKTHEN_EUSAGE && unchanged==(thinkthen_source *)(uintptr_t)13);
    bad.unit=THINKTHEN_SOURCE_WINDOW_V1; bad.window=2;
    assert(thinkthen_source_image_files(e,&bad,&unchanged)==THINKTHEN_EUSAGE && unchanged==(thinkthen_source *)(uintptr_t)13);
    thinkthen_question *q=NULL;
    const char *body="{\"decide\":\"Refund?\",\"item_schema\":{\"type\":\"object\",\"properties\":{\"body\":{\"type\":\"string\"}},\"required\":[\"body\"]}}";
    assert(thinkthen_question_parse(e,1,(thinkthen_string_v1){body,strlen(body)},&q)==0);
    thinkthen_string_v1 paths[]={{path,strlen(path)}};
    thinkthen_source_spec_v1 spec={{paths,1},!strcmp(mode,"literal")?THINKTHEN_SOURCE_LINE_V1:THINKTHEN_SOURCE_JSONL_V1,0};
    thinkthen_source *source=NULL; assert(thinkthen_source_files(e,&spec,&source)==0);
    thinkthen_controls_v1 controls={0}; controls.deadline_ms=-1; controls.attempts=1;
    thinkthen_cancel_token *cancel=thinkthen_cancel_token_new(); assert(cancel); controls.cancel=cancel;
    thinkthen_batch *batch=NULL; assert(thinkthen_decide_batch_start(e,q,source,&controls,&batch)==0 && batch);
    thinkthen_source_free(source); thinkthen_question_free(q);
    thinkthen_result *out=(thinkthen_result *)(uintptr_t)17;
    assert(thinkthen_batch_facts(batch,&out)==THINKTHEN_EUSAGE && out==(thinkthen_result *)(uintptr_t)17);
    if(!strcmp(mode,"drop")) {
        thinkthen_cancel_token_free(cancel); thinkthen_batch_free(batch); thinkthen_engine_free(e); return 0;
    }
    if(!strcmp(mode,"cancel")) thinkthen_cancel(cancel);
    thinkthen_cancel_token_free(cancel); /* Batch cloned the flag. */
    thinkthen_result *rows[2]={NULL,NULL};
    int prefix=!strcmp(mode,"later")?2:!strcmp(mode,"drop-prefix")?1:0;
    for(int at=0;at<prefix;++at) {
        assert(thinkthen_batch_next(batch,&rows[at])==0 && rows[at]);
        thinkthen_decide_view_v1 row={0}; assert(thinkthen_result_decide(rows[at],0,&row)==0);
        assert(row.value.kind==1 && row.value.data.boolean);
        assert(row.common.answer.value.data.probability==0.9);
        assert(row.common.input.present && row.common.input.value.kind==THINKTHEN_CONTENT_JSON_V1);
        assert(same(row.common.input.value.data,at==0?"{\"body\":\"first\"}":"{\"body\":\"second\"}"));
        assert(row.common.position.present && row.common.position.value.first_line.value==(size_t)at+1);
        assert(row.common.position.value.last_line.value==(size_t)at+1);
        thinkthen_details_v1 details={0}; assert(thinkthen_result_details(rows[at],0,&details)==0);
        assert(details.question_sources.len==1 && details.question_sources.data[0].batch_size.present && details.question_sources.data[0].batch_size.value==2);
        thinkthen_summary_v1 summary={0}; assert(thinkthen_result_summary(rows[at],&summary)==0);
        assert(summary.count==1 && !summary.facts.present && summary.observation_count==2);
    }
    if(!strcmp(mode,"drop-prefix")) {
        thinkthen_batch_free(batch); thinkthen_engine_free(e);
        thinkthen_decide_view_v1 row={0}; assert(thinkthen_result_decide(rows[0],0,&row)==0);
        assert(same(row.common.input.value.data,"{\"body\":\"first\"}") && row.common.answer_id.len==64);
        thinkthen_result_free(rows[0]); return 0;
    }
    int wanted=!strcmp(mode,"cancel")?THINKTHEN_ECANCELLED:THINKTHEN_EUSAGE;
    assert(thinkthen_batch_next(batch,&out)==wanted && out==(thinkthen_result *)(uintptr_t)17);
    thinkthen_result *facts=NULL; assert(thinkthen_batch_facts(batch,&facts)==0 && facts);
    thinkthen_summary_v1 summary={0}; assert(thinkthen_result_summary(facts,&summary)==0);
    assert(summary.state==THINKTHEN_RESULT_FAILURE_V1 && summary.error.present && summary.error.value.code==wanted);
    assert(summary.facts.present && summary.facts.value.records==(uint64_t)prefix && summary.facts.value.requests_sent==(uint64_t)(prefix!=0));
    if(wanted==THINKTHEN_EUSAGE) {
        assert(same(summary.error.value.message,"the item does not match item_schema"));
        assert(summary.error.value.stopped.present && summary.error.value.stopped.value.at.present);
        assert(summary.error.value.stopped.value.at.value==(!strcmp(mode,"later")?4:!strcmp(mode,"literal")?1:2));
    }
    assert(thinkthen_batch_next(batch,&out)==0 && out==NULL);
    thinkthen_batch_free(batch); thinkthen_engine_free(e);
    for(int at=0;at<prefix;++at) {
        thinkthen_decide_view_v1 row={0}; assert(thinkthen_result_decide(rows[at],0,&row)==0);
        assert(row.common.answer_id.len==64 && row.common.meta.question_sources.len==1);
        assert(same(row.common.input.value.data,at==0?"{\"body\":\"first\"}":"{\"body\":\"second\"}"));
        thinkthen_result_free(rows[at]);
    }
    assert(thinkthen_result_summary(facts,&summary)==0 && summary.facts.value.call_id.len==64);
    thinkthen_result_free(facts); return 0;
}
