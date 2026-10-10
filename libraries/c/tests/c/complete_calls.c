/* Actual ten-call consumer: every read goes through the canonical public ABI. */
#include <thinkthen.h>
#include <assert.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#define STR(s) (thinkthen_string_v1){s, sizeof(s)-1}
#define TEXT(s) (thinkthen_content_v1){1, STR(s)}
static int same(thinkthen_string_v1 s,const char *value) {
    return s.len==strlen(value) && (s.len==0 || memcmp(s.data,value,s.len)==0);
}
static thinkthen_question *make(thinkthen_engine *e,unsigned kind) {
    thinkthen_question_spec_v1 spec={0}; spec.kind=kind;
    if(kind<8) spec.text=TEXT("Does this need attention?");
    if(kind==7) spec.none=getenv("TYPED_FIND_NONE")!=NULL;
    if(kind==5) spec.threshold=(thinkthen_rule_v1){2,0.95,0};
    if(kind==1) spec.yes=(thinkthen_optional_content_v1){1,TEXT("accepted")};
    thinkthen_choice_v1 choices[2]={0}; choices[0].name=STR("z-first"); choices[1].name=STR("a-second");
    if(kind>=2 && kind<=4) spec.choices=(thinkthen_choices_v1){choices,2};
    thinkthen_member_spec_v1 members[2]={0};
    if(kind==8) {
        members[0].name=STR("z_first"); members[0].question=make(e,1);
        members[1].name=STR("a_second"); members[1].question=make(e,2);
        spec.members=(thinkthen_member_specs_v1){members,2};
    }
    thinkthen_relation_v1 rule={0};
    if(kind==10) { rule.name=STR("supports"); rule.source=STR("*"); rule.target=STR("*"); spec.relations=(thinkthen_relations_v1){&rule,1}; }
    thinkthen_question_author_v1 author={0};
    author.name=(thinkthen_optional_string_v1){1,STR("call-check")};
    author.wording_version=(thinkthen_optional_u64_v1){1,17};
    if(kind<8) author.item_schema.kind=1;
    thinkthen_question *q=NULL;
    assert(thinkthen_question_new_authored(e,&spec,kind==8?NULL:&author,&q)==0 && q);
    thinkthen_question_author_v1 borrowed={0}; assert(thinkthen_question_author(q,&borrowed)==0);
    assert(borrowed.name.present==(kind!=8));
    if(kind!=8) assert(same(borrowed.name.value,"call-check") && borrowed.wording_version.value==17);
    if(kind==8) { thinkthen_question_free((thinkthen_question *)members[0].question); thinkthen_question_free((thinkthen_question *)members[1].question); }
    return q;
}
typedef int (*complete_fn)(const thinkthen_engine *,const thinkthen_question *,const thinkthen_source *,const thinkthen_controls_v1 *,thinkthen_result **);
static const complete_fn calls[]={thinkthen_decide_complete,thinkthen_choose_complete,thinkthen_tag_complete,thinkthen_score_complete,thinkthen_filter_complete,thinkthen_rank_complete,thinkthen_find_complete,thinkthen_annotate_complete,thinkthen_recognize_complete,thinkthen_relate_complete};
static void common(thinkthen_row_v1 row,unsigned kind) {
    assert(row.answer_id.len==64);
    assert(row.question.present && row.question.value.kind==kind);
    assert(row.meta.question_sources.len>0);
    assert(row.meta.question_sources.len==row.meta.observations.len);
    assert(row.meta.origin.present && row.meta.origin.value==THINKTHEN_ORIGIN_LIVE_V1);
    assert(row.meta.answered_by.present && same(row.meta.answered_by.value,"jev-latest"));
    assert(row.meta.requests_sent>0 && !row.meta.cached);
    for(size_t i=0;i<row.meta.observations.len;++i) assert(row.meta.observations.data[i].kind==1 && row.meta.observations.data[i].data.observation_id.len==64);
}
static void check(thinkthen_result *r,unsigned kind,int files) {
    thinkthen_summary_v1 summary={0}; assert(thinkthen_result_summary(r,&summary)==0);
    assert(summary.state==THINKTHEN_RESULT_SUCCESS_V1 && same(summary.schema,"thinkthen.result/2"));
    assert(summary.function.present && summary.function.value==kind);
    assert(summary.count==((kind==7 || kind==10)?1:2));
    assert(summary.facts.present && summary.facts.value.call_id.len==64);
    assert(summary.facts.value.requests_sent==((kind==9)?4:1));
    assert(summary.attempts.present && summary.attempts.value.len==summary.facts.value.requests_sent);
    for(size_t i=0;i<summary.attempts.value.len;++i) {
        assert(summary.attempts.value.data[i].ordinal==i+1);
        assert(summary.attempts.value.data[i].sdk_request_id.len==64);
    }
    thinkthen_row_v1 row={0};
    if(kind==1) { thinkthen_decide_view_v1 v={0}; assert(thinkthen_result_decide(r,0,&v)==0); row=v.common; assert(v.value.kind==2 && same(v.value.data.authored.data,"accepted")); assert(v.common.answer.value.kind==1 && v.common.answer.value.data.probability==0.9); }
    if(kind==2) { thinkthen_choose_view_v1 v={0}; assert(thinkthen_result_choose(r,0,&v)==0); row=v.common; assert(v.value.present && same(v.value.value,"z-first")); assert(same(row.answer.value.data.choice.pick,"z-first") && row.answer.value.data.choice.probabilities.len==2); }
    if(kind==3) { thinkthen_tag_view_v1 v={0}; assert(thinkthen_result_tag(r,0,&v)==0); row=v.common; assert(v.value.len==2 && same(v.value.data[0],"z-first")); assert(row.answer.value.data.tag.len==2); }
    if(kind==4) { thinkthen_score_view_v1 v={0}; assert(thinkthen_result_score(r,0,&v)==0); row=v.common; assert(v.value==0.1); assert(same(row.answer.value.data.score.level,"z-first")); }
    if(kind==5) { thinkthen_filter_view_v1 v={0}; assert(thinkthen_result_filter(r,0,&v)==0); row=v.common; assert(v.value==0 && row.answer.value.data.probability==0.9); }
    if(kind==6) { thinkthen_rank_view_v1 v={0}; assert(thinkthen_result_rank(r,0,&v)==0); row=v.common; assert(v.value.present && v.value.value==1); }
    if(kind==7) {
        int none=getenv("TYPED_FIND_NONE")!=NULL;
        thinkthen_find_view_v1 v={0}; assert(thinkthen_result_find(r,0,&v)==0); row=v.common;
        assert(v.value.present==!none && v.index.present==!none && row.question.value.none==none);
        if(!none) { assert(v.index.value==0 && v.value.value.kind==THINKTHEN_CONTENT_TEXT_V1 && v.value.value.data.len>0); if(!files) assert(same(v.value.value.data,"Maria Chen joined Northwind Freight.")); }
        assert(row.answer.present && row.answer.value.kind==THINKTHEN_ANSWER_FIND_V1);
        thinkthen_named_answer_v1 answer=row.answer.value.data.find;
        assert(same(answer.pick,none?"none":"u001"));
        assert(answer.probabilities.len==(none?3:2));
        assert(same(answer.probabilities.data[0].name,"u001") && answer.probabilities.data[0].probability==(none?0.1:0.9));
        assert(same(answer.probabilities.data[1].name,"u002") && answer.probabilities.data[1].probability==0.1);
        if(none) assert(same(answer.probabilities.data[2].name,"none") && answer.probabilities.data[2].probability==0.8);
    }
    if(kind==8) { thinkthen_annotate_view_v1 v={0}; assert(thinkthen_result_annotate(r,0,&v)==0); row=v.common; assert(v.answers.len==2 && same(v.answers.data[0].name,"z_first")); assert(v.answers.data[0].data.success.value.data.decide.kind==2); assert(same(v.answers.data[1].data.success.answer.data.choice.pick,"z-first")); }
    if(kind==9) { thinkthen_recognize_view_v1 v={0}; assert(thinkthen_result_recognize(r,0,&v)==0); row=v.common; assert(v.value.entities.len>0 && v.answer.pieces.len>0 && v.answer.names.len>0); }
    if(kind==10) { thinkthen_relate_view_v1 v={0}; assert(thinkthen_result_relate(r,0,&v)==0); row=v.common; assert(v.value.len==2 && v.questions.len==2); assert(v.questions.data[0].data.success.answer_id.len==64); }
    thinkthen_question_author_v1 author={0}; assert(thinkthen_result_question_author(r,0,&author)==0);
    assert(author.name.present==(kind!=8));
    if(kind!=8) assert(same(author.name.value,"call-check") && author.wording_version.present && author.wording_version.value==17);
    if(kind==8) { assert(thinkthen_result_member_author(r,0,0,&author)==0); assert(same(author.name.value,"call-check") && author.wording_version.value==17); }
    else assert(thinkthen_result_member_author(r,0,0,&author)==THINKTHEN_EUSAGE);
    if(kind==9) {
        thinkthen_source_recognition_v1 located={0}; assert(thinkthen_result_source_recognition(r,0,&located)==0);
        assert(located.present==files);
        if(files) {
            assert(located.entities.len>0);
            for(size_t at=0;at<located.entities.len;++at) {
                assert(located.entities.data[at].position.present && located.entities.data[at].position.value.file.present);
                assert(located.entities.data[at].entity.start<=row.input.value.data.len);
                size_t line=row.position.value.first_line.value;
                for(size_t scalar=0;scalar<located.entities.data[at].entity.start;++scalar) if(row.input.value.data.data[scalar]=='\n') ++line;
                assert(located.entities.data[at].position.value.first_line.present && located.entities.data[at].position.value.first_line.value==line);
                assert(located.entities.data[at].entity.text.len>0);
            }
        }
    }
    if(kind==10) {
        thinkthen_source_relations_v1 located={0}; assert(thinkthen_result_source_relations(r,0,&located)==0);
        assert(located.present==files);
        if(files) { assert(located.edges.len==2); for(size_t at=0;at<located.edges.len;++at) {
            assert(located.edges.data[at].source.position.present && located.edges.data[at].target.position.present);
            assert(located.edges.data[at].source.record.kind==1 && located.edges.data[at].source.record.data.len>0);
            assert(located.edges.data[at].source.ordinal!=located.edges.data[at].target.ordinal);
        } }
    }
    common(row,kind==5 || kind==6 ? 1:kind);
    thinkthen_details_v1 detail={0}; assert(thinkthen_result_details(r,0,&detail)==0);
    thinkthen_details_v1 member_unchanged={0}; member_unchanged.usage.input_tokens.value=99;
    assert(thinkthen_result_rank_member_details(r,0,0,&member_unchanged)==THINKTHEN_EUSAGE);
    assert(member_unchanged.usage.input_tokens.value==99);
    assert(detail.inputs.len==((kind==7 || kind==10)?2:1));
    assert(detail.question_sources.len==row.meta.question_sources.len);
    for(size_t i=0;i<detail.question_sources.len;++i) assert(detail.question_sources.data[i].batch_size.present);
    if(files) for(size_t i=0;i<detail.inputs.len;++i) assert(detail.inputs.data[i].position.present);

    if(kind==4) {
        assert(row.question.value.choices.len==2);
        assert(!row.question.value.choices.data[0].description.present);
        assert(!row.question.value.choices.data[1].description.present);
    }
    if(kind!=7 && kind!=10) { assert(row.input.present); if(files) assert(row.position.present && row.position.value.first_line.present && row.position.value.last_line.present); }
    assert(summary.observation_count>summary.count);
    size_t question_count=0,row_count=0;
    for(size_t i=0;i<summary.observation_count;++i) {
        thinkthen_observation_v1 o={0}; assert(thinkthen_result_observation(r,i,&o)==0);
        thinkthen_question_author_v1 observed={0}; assert(thinkthen_result_observation_author(r,i,&observed)==0);
        if(o.kind==THINKTHEN_EVENT_ROW_V1) assert(observed.name.present==(kind!=8));
        if(o.kind==THINKTHEN_EVENT_QUESTION_V1) { ++question_count; assert(o.data.question.state==1); assert(o.data.question.data.success.answer_id.len==64);
            thinkthen_details_v1 d={0}; assert(thinkthen_result_observation_details(r,i,&d)==0);
            assert(d.question.present && d.inputs.len>0 && d.observations.len==d.question_sources.len);
            if(d.observations.len==1) assert(o.data.question.data.success.observation_id.len==64);
            else assert(o.data.question.data.success.observation_id.len==0 && o.data.question.data.success.observation_id.data==NULL);
            for(size_t j=0;j<d.observations.len;++j) assert(d.observations.data[j].data.observation_id.len==64); }
        else { assert(o.kind==THINKTHEN_EVENT_ROW_V1 && o.data.row.function==kind); ++row_count; }
    }
    assert(question_count>0 && row_count==summary.count);
    thinkthen_observation_v1 unchanged={0}; unchanged.kind=99;
    assert(thinkthen_result_observation(r,summary.observation_count,&unchanged)==THINKTHEN_EUSAGE && unchanged.kind==99);
    thinkthen_decide_view_v1 v={0}; assert(thinkthen_result_decide(r,summary.count,&v)==THINKTHEN_EUSAGE);
    assert(thinkthen_result_summary(NULL,&summary)==THINKTHEN_EUSAGE);
}
static void indexes(thinkthen_engine *e) {
    thinkthen_record_v1 records[3]={0};
    const char *inputs[]={"first","second","third"};
    for(size_t i=0;i<3;++i) records[i].original=(thinkthen_optional_content_v1){1,{1,{inputs[i],strlen(inputs[i])}}};
    const unsigned kinds[]={1,5,6}; thinkthen_result *results[3]={0};
    for(size_t at=0;at<3;++at) {
        thinkthen_question_spec_v1 spec={0}; spec.kind=kinds[at]; spec.text=TEXT("Does this need attention?");
        if(spec.kind==5) spec.threshold=(thinkthen_rule_v1){2,0.75,0};
        thinkthen_question *q=NULL; assert(thinkthen_question_new(e,&spec,&q)==0);
        thinkthen_source *source=NULL; assert(thinkthen_source_records(e,records,at?3:2,&source)==0);
        assert(calls[spec.kind-1](e,q,source,NULL,&results[at])==0);
        thinkthen_question_free(q); thinkthen_source_free(source);
    }
    thinkthen_engine_free(e);
    for(size_t at=0;at<3;++at) {
        thinkthen_result *r=results[at]; thinkthen_summary_v1 summary={0}; assert(thinkthen_result_summary(r,&summary)==0);
        assert(summary.count==(at?3:2) && summary.facts.value.requests_sent==1);
        thinkthen_row_observation_v1 row,unchanged; memset(&row,0xa5,sizeof(row)); memcpy(&unchanged,&row,sizeof(row));
        assert(thinkthen_result_row(NULL,0,&row)==THINKTHEN_EUSAGE && !memcmp(&row,&unchanged,sizeof(row)));
        assert(thinkthen_result_row(r,summary.count,&row)==THINKTHEN_EUSAGE && !memcmp(&row,&unchanged,sizeof(row)));
        assert(thinkthen_result_row(r,SIZE_MAX,&row)==THINKTHEN_EUSAGE && !memcmp(&row,&unchanged,sizeof(row)));
        assert(thinkthen_result_row(r,0,NULL)==THINKTHEN_EUSAGE);
        const size_t ranked[]={1,2,0};
        for(size_t i=0;i<summary.count;++i) {
            assert(thinkthen_result_row(r,i,&row)==0 && row.function==kinds[at]);
            assert(row.index==(at==2?ranked[i]:i));
            thinkthen_row_v1 common=at==0?row.data.decide.common:at==1?row.data.filter.common:row.data.rank.common;
            assert(same(common.input.value.data,inputs[row.index]));
            if(at==1) assert(row.data.filter.value==(i==1));
            if(at==2) assert(row.data.rank.value.value==i+1);
        }
        thinkthen_result_free(r);
    }
}
int main(void) {
    const char *base=getenv("THINKTHEN_BASE_URL"); assert(base && !strncmp(base,"http://127.0.0.1:",17));
    thinkthen_engine *e=thinkthen_engine_new_with("{\"cache\":false,\"model\":\"jev-latest\"}"); assert(e);
    if(getenv("TYPED_ROW_INDEX")) { indexes(e); return 0; }
    thinkthen_source *source=NULL;
    const char *path=getenv("TYPED_SOURCE");
    if(path) { thinkthen_string_v1 p={path,strlen(path)}; thinkthen_source_spec_v1 spec={{&p,1},getenv("TYPED_RELATE_BOUND")?THINKTHEN_SOURCE_LINE_V1:THINKTHEN_SOURCE_FILE_V1,0}; assert(thinkthen_source_files(e,&spec,&source)==0); }
    else { thinkthen_record_v1 records[2]={0}; records[0].original=(thinkthen_optional_content_v1){1,TEXT("Maria Chen joined Northwind Freight.")}; records[1].original=(thinkthen_optional_content_v1){1,TEXT("A different document.")}; assert(thinkthen_source_records(e,records,2,&source)==0); }
    thinkthen_controls_v1 controls={0}; controls.deadline_ms=-1; controls.attempts=1;
    if(getenv("TYPED_RELATE_BOUND")) {
        size_t count=(size_t)strtoul(getenv("TYPED_RELATE_BOUND"),NULL,10);
        thinkthen_question *q=make(e,10); thinkthen_result *result=NULL;
        int code=thinkthen_relate_complete(e,q,source,&controls,&result);
        if(count==255) {
            assert(code==0 && result);
            thinkthen_relate_view_v1 view={0}; assert(thinkthen_result_relate(result,0,&view)==0);
            assert(view.inputs.len==255 && view.value.len==0);
            thinkthen_result_free(result);
        } else {
            assert(code==THINKTHEN_EUSAGE && result==NULL);
            assert(strcmp(thinkthen_error_message(e),"relate takes at most 255 records")==0);
        }
        thinkthen_question_free(q); thinkthen_source_free(source); thinkthen_engine_free(e);
        return 0;
    }
    thinkthen_result *results[10]={0};
    for(unsigned kind=1;kind<=10;++kind) {
        if(getenv("TYPED_FIND_NONE") && kind!=7) continue;
        thinkthen_question *q=make(e,kind);
        int rc=calls[kind-1](e,q,source,&controls,&results[kind-1]);
        if(rc) { fprintf(stderr,"function %u: %d %s\n",kind,rc,thinkthen_error_message(e)); abort(); }
        thinkthen_question_free(q);
    }
    thinkthen_source_free(source); thinkthen_engine_free(e);
    for(unsigned kind=1;kind<=10;++kind) if(results[kind-1]) { check(results[kind-1],kind,path!=NULL); thinkthen_result_free(results[kind-1]); }
    thinkthen_result_free(NULL);
    return 0;
}
