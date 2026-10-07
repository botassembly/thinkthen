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
    thinkthen_question_author_v1 declaration={0}; declaration.name=(thinkthen_optional_string_v1){1,STR("authored_a")};
    declaration.wording_version=(thinkthen_optional_u64_v1){1,4};
    assert(thinkthen_question_new_authored(e,&spec,&declaration,&a)==0); spec.text=TEXT("Criterion B?");
    declaration.name=(thinkthen_optional_string_v1){1,STR("authored_b")};
    assert(thinkthen_question_new_authored(e,&spec,&declaration,&b)==0);
    thinkthen_member_spec_v1 members[]={{STR("first"),a},{STR("second"),b}};
    spec=(thinkthen_question_spec_v1){0}; spec.kind=6; spec.members=(thinkthen_member_specs_v1){members,2};
    assert(thinkthen_question_new(e,&spec,&set)==0);
    thinkthen_record_v1 records[3]={0}; records[0].original=(thinkthen_optional_content_v1){1,TEXT("One")}; records[1].original=(thinkthen_optional_content_v1){1,TEXT("Two")}; records[2].original=(thinkthen_optional_content_v1){1,TEXT("Three")};
    thinkthen_source *source=NULL; assert(thinkthen_source_records(e,records,3,&source)==0);
    thinkthen_controls_v1 controls={0}; controls.deadline_ms=-1; controls.attempts=1;
    thinkthen_result *result=NULL; assert(thinkthen_rank_complete(e,set,source,&controls,&result)==0);
    thinkthen_question_free(a); thinkthen_question_free(b); thinkthen_question_free(set); thinkthen_source_free(source); thinkthen_engine_free(e);
    thinkthen_summary_v1 summary={0}; assert(thinkthen_result_summary(result,&summary)==0);
    assert(summary.count==3 && summary.facts.value.requests_sent==1 && summary.facts.value.records==3);
    assert(summary.facts.value.input_tokens.present && summary.facts.value.input_tokens.value==12);
    assert(!summary.facts.value.output_tokens.present);
    const size_t positions[3][2]={{1,1},{2,3},{3,2}};
    const double probabilities[3][2]={{0.7,1.0},{0.6,0.98},{0.5,0.99}};
    const char *originals[]={"One","Two","Three"};
    for(size_t row=0;row<3;++row) {
        thinkthen_rank_view_v1 ranked={0}; assert(thinkthen_result_rank(result,row,&ranked)==0);
        assert(ranked.value.value==row+1 && ranked.question_name.present);
        assert(same(ranked.question_name.value,row==2?"second":"first"));
        assert(same(ranked.common.input.value.data,originals[row]));
        assert(ranked.common.answer_id.len==64 && ranked.common.meta.observations.len==2);
        size_t count=0; assert(thinkthen_result_rank_member_count(result,row,&count)==0 && count==2);
        for(size_t member=0;member<count;++member) {
            thinkthen_rank_view_v1 judgment={0}; assert(thinkthen_result_rank_member(result,row,member,&judgment)==0);
            assert(same(judgment.question_name.value,member==0?"first":"second"));
            assert(judgment.value.value==positions[row][member] && judgment.common.answer.value.kind==1 && judgment.common.answer.value.data.probability==probabilities[row][member]);
            assert(judgment.common.answer_id.len==64 && judgment.common.meta.observations.len==1);
            thinkthen_question_author_v1 author={0}; assert(thinkthen_result_member_author(result,row,member,&author)==0);
            assert(author.name.present && same(author.name.value,member==0?"authored_a":"authored_b"));
            assert(author.wording_version.present && author.wording_version.value==4);
            thinkthen_details_v1 details={0}; assert(thinkthen_result_rank_member_details(result,row,member,&details)==0);
            assert(details.usage.present && details.usage.input_tokens.present && details.usage.input_tokens.value==2);
            assert(!details.usage.output_tokens.present && details.threshold.present && details.threshold.value.kind==1);
            assert(details.question_sources.len==1 && details.question_sources.data[0].batch_size.present && details.question_sources.data[0].batch_size.value==6);
            assert(same(details.question_sources.data[0].answered_by,"fixed"));
            assert(details.inputs.len==1 && same(details.inputs.data[0].original.value.data,originals[row]));
            assert(details.observations.len==1 && details.question.present);
            thinkthen_details_v1 aggregate={0}; assert(thinkthen_result_details(result,row,&aggregate)==0);
            assert(aggregate.usage.input_tokens.value==4 && !aggregate.usage.output_tokens.present);
            assert(ranked.common.meta.attempts.value.len==1);
        }
    }
    thinkthen_rank_view_v1 unchanged={0}; unchanged.value.value=99;
    assert(thinkthen_result_rank_member(result,0,2,&unchanged)==THINKTHEN_EUSAGE && unchanged.value.value==99);
    thinkthen_details_v1 detail_unchanged={0}; detail_unchanged.usage.input_tokens.value=99;
    thinkthen_details_v1 retained=detail_unchanged;
    assert(thinkthen_result_rank_member_details(result,0,2,&detail_unchanged)==THINKTHEN_EUSAGE);
    assert(thinkthen_result_rank_member_details(result,3,0,&detail_unchanged)==THINKTHEN_EUSAGE);
    assert(thinkthen_result_rank_member_details(NULL,0,0,&detail_unchanged)==THINKTHEN_EUSAGE);
    assert(thinkthen_result_rank_member_details(result,0,0,NULL)==THINKTHEN_EUSAGE);
    assert(!memcmp(&detail_unchanged,&retained,sizeof(retained)));
    thinkthen_result_free(result); return 0;
}
