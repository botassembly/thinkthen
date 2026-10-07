/* Saved rank sets execute native turns, retaining every member and original. */
#include <thinkthen.h>
#include <assert.h>
#include <string.h>
#define STR(s) (thinkthen_string_v1){s,sizeof(s)-1}
#define TEXT(s) (thinkthen_content_v1){1,STR(s)}
static int same(thinkthen_string_v1 value,const char *text) { return value.len==strlen(text) && !memcmp(value.data,text,value.len); }
int main(void) {
    thinkthen_engine *e=thinkthen_engine_new_with("{\"cache\":false,\"model\":\"fixed\"}"); assert(e);
    thinkthen_question *a=NULL,*b=NULL,*set=NULL;
    thinkthen_question_spec_v1 spec={0}; spec.kind=1; spec.text=TEXT("Criterion A?");
    assert(thinkthen_question_new(e,&spec,&a)==0); spec.text=TEXT("Criterion B?");
    assert(thinkthen_question_new(e,&spec,&b)==0);
    thinkthen_member_spec_v1 members[]={{STR("first"),a},{STR("second"),b}};
    spec=(thinkthen_question_spec_v1){0}; spec.kind=6; spec.members=(thinkthen_member_specs_v1){members,2};
    assert(thinkthen_question_new(e,&spec,&set)==0);
    thinkthen_record_v1 records[2]={0}; records[0].original=(thinkthen_optional_content_v1){1,TEXT("One")}; records[1].original=(thinkthen_optional_content_v1){1,TEXT("Two")};
    thinkthen_source *source=NULL; assert(thinkthen_source_records(e,records,2,&source)==0);
    thinkthen_controls_v1 controls={0}; controls.deadline_ms=-1; controls.attempts=1;
    thinkthen_result *result=NULL; assert(thinkthen_rank_complete(e,set,source,&controls,&result)==0);
    thinkthen_question_free(a); thinkthen_question_free(b); thinkthen_question_free(set); thinkthen_source_free(source); thinkthen_engine_free(e);
    thinkthen_summary_v1 summary={0}; assert(thinkthen_result_summary(result,&summary)==0);
    assert(summary.count==2 && summary.facts.value.requests_sent==1 && summary.facts.value.records==2);
    for(size_t row=0;row<2;++row) {
        thinkthen_rank_view_v1 ranked={0}; assert(thinkthen_result_rank(result,row,&ranked)==0);
        assert(ranked.value.value==row+1 && ranked.question_name.present);
        assert(same(ranked.question_name.value,"first"));
        assert(same(ranked.common.input.value.data,row==0?"One":"Two"));
        assert(ranked.common.answer_id.len==64 && ranked.common.meta.observations.len==2);
        size_t count=0; assert(thinkthen_result_rank_member_count(result,row,&count)==0 && count==2);
        for(size_t member=0;member<count;++member) {
            thinkthen_rank_view_v1 judgment={0}; assert(thinkthen_result_rank_member(result,row,member,&judgment)==0);
            assert(same(judgment.question_name.value,member==0?"first":"second"));
            assert(judgment.value.value==row+1 && judgment.common.answer.value.kind==1 && judgment.common.answer.value.data.probability==0.9);
            assert(judgment.common.answer_id.len==64 && judgment.common.meta.observations.len==1);
            thinkthen_question_author_v1 author={0}; assert(thinkthen_result_member_author(result,row,member,&author)==0);
        }
    }
    thinkthen_rank_view_v1 unchanged={0}; unchanged.value.value=99;
    assert(thinkthen_result_rank_member(result,0,2,&unchanged)==THINKTHEN_EUSAGE && unchanged.value.value==99);
    thinkthen_result_free(result); return 0;
}
