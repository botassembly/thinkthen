/* Native field readings, authored nulls and member failures. */
#include <thinkthen.h>
#include <assert.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#define STR(s) (thinkthen_string_v1){s,sizeof(s)-1}
#define TEXT(s) (thinkthen_content_v1){1,STR(s)}
static int same(thinkthen_string_v1 s,const char *v) { return s.len==strlen(v) && (s.len==0 || memcmp(s.data,v,s.len)==0); }
static int equal(thinkthen_string_v1 a,thinkthen_string_v1 b) { return a.len==b.len && (a.len==0 || memcmp(a.data,b.data,a.len)==0); }
static thinkthen_question *question(thinkthen_engine *e,thinkthen_question_spec_v1 spec) {
    thinkthen_question *q=NULL; assert(thinkthen_question_new(e,&spec,&q)==0 && q); return q;
}
static thinkthen_source *records(thinkthen_engine *e,thinkthen_content_v1 content) {
    thinkthen_record_v1 r={0}; r.original=(thinkthen_optional_content_v1){1,content};
    thinkthen_source *s=NULL; assert(thinkthen_source_records(e,&r,1,&s)==0); return s;
}
int main(void) {
    thinkthen_engine *e=thinkthen_engine_new_with("{\"cache\":false,\"model\":\"fixed\"}"); assert(e);
    thinkthen_controls_v1 c={0}; c.deadline_ms=-1; c.attempts=1;
    thinkthen_question_spec_v1 spec={0}; spec.kind=1; spec.text=TEXT("Need attention?");
    spec.yes=(thinkthen_optional_content_v1){1,{2,STR("null")}};
    spec.no=(thinkthen_optional_content_v1){1,TEXT("negative")};
    thinkthen_question *yes=question(e,spec); thinkthen_source *s=records(e,TEXT("evidence"));
    thinkthen_result *r=NULL;
    if(getenv("TYPED_PARTIAL")) {
        thinkthen_choice_v1 options[2]={0}; options[0].name=STR("first"); options[1].name=STR("last");
        spec=(thinkthen_question_spec_v1){0}; spec.kind=2; spec.text=TEXT("Pick?"); spec.choices=(thinkthen_choices_v1){options,2};
        thinkthen_question *choose=question(e,spec);
        thinkthen_member_spec_v1 members[2]={{STR("ready"),yes},{STR("selection"),choose}};
        spec=(thinkthen_question_spec_v1){0}; spec.kind=8; spec.members=(thinkthen_member_specs_v1){members,2};
        thinkthen_question *set=question(e,spec);
        assert(thinkthen_annotate_complete(e,set,s,&c,&r)==0);
        thinkthen_annotate_view_v1 v={0}; assert(thinkthen_result_annotate(r,0,&v)==0);
        assert(v.answers.len==2 && v.answers.data[0].state==1 && v.answers.data[1].state==2);
        assert(v.answers.data[0].data.success.value.data.decide.kind==2 && same(v.answers.data[0].data.success.value.data.decide.data.authored.data,"null"));
        assert(v.answers.data[1].data.failure.cause==THINKTHEN_MEMBER_MISSING_ANSWER_V1 && v.answers.data[1].data.failure.failure_id.len==64);
        assert(v.common.meta.failed_questions==1 && v.common.meta.observations.len==2 && v.common.meta.observations.data[1].kind==2);
        size_t failed=0; thinkthen_summary_v1 sum={0}; assert(thinkthen_result_summary(r,&sum)==0);
        for(size_t i=0;i<sum.observation_count;++i) {
            thinkthen_observation_v1 o={0}; assert(thinkthen_result_observation(r,i,&o)==0);
            if(o.kind==1 && o.data.question.state==2) { ++failed; assert(o.data.question.data.failure.failure_id.len==64);
                thinkthen_details_v1 d={0}; assert(thinkthen_result_observation_details(r,i,&d)==0);
                assert(d.question.present && d.observations.len==1 && d.observations.data[0].kind==2);
                assert(equal(d.observations.data[0].data.failure_id,v.answers.data[1].data.failure.failure_id));
            }
        }
        assert(failed==1); thinkthen_question_free(set); thinkthen_question_free(choose);
    } else {
        assert(thinkthen_decide_complete(e,yes,s,&c,&r)==0);
        thinkthen_decide_view_v1 v={0}; assert(thinkthen_result_decide(r,0,&v)==0);
        assert(v.value.kind==2 && v.value.data.authored.kind==2 && same(v.value.data.authored.data,"null"));
        thinkthen_result_free(r); r=NULL; thinkthen_source_free(s);
        thinkthen_string_v1 on=STR("/body");
        spec=(thinkthen_question_spec_v1){0}; spec.kind=7; spec.text=TEXT("Which?"); spec.on=(thinkthen_strings_v1){&on,1};
        thinkthen_question *find=question(e,spec);
        const char *path=getenv("TYPED_SELECTED_FILE"); assert(path);
        thinkthen_string_v1 p={path,strlen(path)};
        thinkthen_string_v1 paths[2]={p,p};
        thinkthen_source_spec_v1 file={{paths,2},THINKTHEN_SOURCE_FILE_V1,0}; s=NULL; assert(thinkthen_source_files(e,&file,&s)==0);
        int rc=thinkthen_find_complete(e,find,s,&c,&r);
        if(rc) { fprintf(stderr,"find: %d %s\n",rc,thinkthen_error_message(e)); abort(); }
        thinkthen_find_view_v1 found={0}; assert(thinkthen_result_find(r,0,&found)==0 && found.index.value==0 && found.value.value.kind==1);
        thinkthen_details_v1 d={0}; assert(thinkthen_result_details(r,0,&d)==0);
        assert(d.inputs.len==2 && d.inputs.data[0].position.present && d.inputs.data[0].original.value.kind==2);
        assert(same(found.value.value.data,"{\"body\":\"selected evidence\",\"other\":\"withheld\"}"));
        thinkthen_question_free(find); thinkthen_result_free(r); r=NULL; thinkthen_source_free(s);
        const char *saved=getenv("TYPED_ANNOTATE_FILE"); assert(saved);
        thinkthen_question *set=NULL; assert(thinkthen_question_load(e,(thinkthen_string_v1){saved,strlen(saved)},&set)==0);
        s=records(e,TEXT("{\"body\":\"selected evidence\",\"other\":\"withheld\"}"));
        assert(thinkthen_annotate_complete(e,set,s,&c,&r)==0);
        thinkthen_annotate_view_v1 a={0}; assert(thinkthen_result_annotate(r,0,&a)==0 && a.answers.len==1 && a.answers.data[0].state==1);
        assert(a.common.input.value.kind==1); thinkthen_question_free(set);
        thinkthen_result_free(r); r=NULL;
        thinkthen_choice_v1 levels[2]={0}; levels[0].name=STR("low"); levels[1].name=STR("high");
        levels[0].description=(thinkthen_optional_content_v1){1,{2,STR("null")}};
        levels[1].description=(thinkthen_optional_content_v1){1,TEXT("High quality.")};
        spec=(thinkthen_question_spec_v1){0}; spec.kind=4; spec.text=TEXT("How good?");
        spec.choices=(thinkthen_choices_v1){levels,2};
        thinkthen_question *scoring=question(e,spec);
        assert(thinkthen_score_complete(e,scoring,s,&c,&r)==0);
        thinkthen_score_view_v1 score={0}; assert(thinkthen_result_score(r,0,&score)==0);
        assert(score.common.question.value.choices.data[0].description.present);
        assert(score.common.question.value.choices.data[0].description.value.kind==2);
        assert(same(score.common.question.value.choices.data[0].description.value.data,"null"));
        thinkthen_question_free(scoring); levels[1].description.present=0;
        scoring=NULL;
        assert(thinkthen_question_new(e,&spec,&scoring)==THINKTHEN_EUSAGE && !scoring);
        assert(strcmp(thinkthen_error_message(e),"give every level a description, or none")==0);

    }
    thinkthen_question_free(yes); thinkthen_source_free(s); thinkthen_engine_free(e);
    thinkthen_result_free(r); return 0;
}
