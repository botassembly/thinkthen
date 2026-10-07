/* Actual 0456 judgments, typed object context, native loaders and staged refusal. */
#include <thinkthen.h>
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#ifdef _WIN32
#include <direct.h>
#define chdir _chdir
#else
#include <unistd.h>
#endif
#define STR(s) (thinkthen_string_v1){s,sizeof(s)-1}
#define TEXT(s) (thinkthen_content_v1){1,STR(s)}
#define JSON(s) (thinkthen_content_v1){2,STR(s)}
static int same(thinkthen_string_v1 s,const char *v) { return s.len==strlen(v) && (s.len==0 || memcmp(s.data,v,s.len)==0); }
static thinkthen_engine *engine(void) {
    const char *base=getenv("THINKTHEN_BASE_URL"); assert(base && strncmp(base,"http://127.0.0.1:",17)==0);
    char settings[4096]; int n=snprintf(settings,sizeof settings,"{\"base_url\":\"%s\",\"cache\":false,\"model\":\"jev-latest\"}",base);
    assert(n>0 && (size_t)n<sizeof settings);
    thinkthen_engine *e=thinkthen_engine_new_with(settings); assert(e); return e;
}
static thinkthen_question_author_v1 metadata(int object_item,int object_context) {
    static thinkthen_input_property_v1 item_properties[2];
    static thinkthen_input_property_v1 context_properties[1];
    static thinkthen_string_v1 item_required[1],context_required[1];
    item_properties[0]=(thinkthen_input_property_v1){STR("body"),1};
    item_properties[1]=(thinkthen_input_property_v1){STR("ready"),3};
    item_required[0]=STR("body");
    context_properties[0]=(thinkthen_input_property_v1){STR("flag"),3}; context_required[0]=STR("flag");
    thinkthen_question_author_v1 a={0}; a.name=(thinkthen_optional_string_v1){1,STR("typed-check")};
    a.wording_version=(thinkthen_optional_u64_v1){1,23};
    a.item_schema.kind=object_item?2:1; a.context_schema.kind=object_context?2:1;
    if(object_item) { a.item_schema.properties=(thinkthen_input_properties_v1){item_properties,2}; a.item_schema.required=(thinkthen_strings_v1){item_required,1}; }
    if(object_context) { a.context_schema.properties=(thinkthen_input_properties_v1){context_properties,1}; a.context_schema.required=(thinkthen_strings_v1){context_required,1}; }
    return a;
}
static thinkthen_question *question(thinkthen_engine *e,thinkthen_question_author_v1 a) {
    thinkthen_question_spec_v1 spec={0}; spec.kind=1; spec.text=TEXT("Need attention?");
    thinkthen_question *q=NULL; assert(thinkthen_question_new_authored(e,&spec,&a,&q)==0 && q); return q;
}
static thinkthen_source *source(thinkthen_engine *e,const thinkthen_record_v1 *records,size_t len) {
    thinkthen_source *s=NULL; assert(thinkthen_source_records(e,records,len,&s)==0 && s); return s;
}
static thinkthen_controls_v1 controls(void) {
    thinkthen_controls_v1 c={0}; c.deadline_ms=-1; c.attempts=1;
    c.context=(thinkthen_optional_content_v1){1,TEXT("shared fallback")}; return c;
}
static void success(void) {
    thinkthen_engine *e=engine(); thinkthen_result *results[3]={0}; thinkthen_controls_v1 c=controls();
    for(int at=0;at<3;++at) {
        thinkthen_question *q=question(e,metadata(at==0,at!=2));
        thinkthen_record_v1 r={0};
        r.original=(thinkthen_optional_content_v1){1,at==0?JSON("{\"body\":\"typed evidence\",\"ready\":false}"):TEXT("typed evidence")};
        if(at==0) r.context=(thinkthen_optional_content_v1){1,JSON("{\"flag\":false,\"unselected\":null}")};
        if(at==2) r.context=(thinkthen_optional_content_v1){1,TEXT("")};
        thinkthen_source *s=source(e,&r,1);
        assert(thinkthen_decide_complete(e,q,s,&c,&results[at])==0);
        thinkthen_question_free(q); thinkthen_source_free(s);
    }
    thinkthen_engine_free(e);
    for(int at=0;at<3;++at) {
        thinkthen_question_author_v1 a={0}; assert(thinkthen_result_question_author(results[at],0,&a)==0);
        assert(a.name.present && same(a.name.value,"typed-check") && a.wording_version.present && a.wording_version.value==23);
        assert(a.item_schema.kind==(at==0?2u:1u) && a.context_schema.kind==(at==2?1u:2u));
        if(at==0) { assert(a.item_schema.properties.len==2 && same(a.item_schema.properties.data[0].name,"body") && a.item_schema.properties.data[1].kind==3); assert(a.context_schema.required.len==1 && same(a.context_schema.required.data[0],"flag")); }
        thinkthen_summary_v1 summary={0}; assert(thinkthen_result_summary(results[at],&summary)==0);
        assert(summary.facts.value.requests_sent==1);
        for(size_t i=0;i<summary.observation_count;++i) {
            assert(thinkthen_result_observation_author(results[at],i,&a)==0);
            assert(same(a.name.value,"typed-check") && a.wording_version.value==23);
        }
        a.wording_version.value=777;
        assert(thinkthen_result_question_author(results[at],1,&a)==THINKTHEN_EUSAGE && a.wording_version.value==777);
        assert(thinkthen_result_question_author(NULL,0,&a)==THINKTHEN_EUSAGE);
        assert(thinkthen_result_question_author(results[at],0,NULL)==THINKTHEN_EUSAGE);
        thinkthen_result_free(results[at]);
    }
}
static void refused(void) {
    thinkthen_engine *e=engine(); thinkthen_controls_v1 c=controls();
    for(int at=0;at<7;++at) {
        thinkthen_question_author_v1 a=metadata(at==0 || at==6,at>=3);
        if(at==5) a.context_schema=(thinkthen_input_declaration_v1){0};
        thinkthen_question *q=question(e,a);
        thinkthen_record_v1 r[2]={0}; r[0].original=(thinkthen_optional_content_v1){1,at==0 || at==6?JSON("{\"body\":\"good\",\"ready\":false}"):TEXT("good")}; r[1]=r[0];
        if(at==0) r[1].original=(thinkthen_optional_content_v1){1,JSON("{\"body\":false}")};
        if(at==1) r[1].original=(thinkthen_optional_content_v1){1,JSON("false")};
        if(at==2) r[1].context=(thinkthen_optional_content_v1){1,JSON("null")};
        if(at==3) r[1].context=(thinkthen_optional_content_v1){1,TEXT("{\"flag\":false}")};
        if(at==4) r[1].context=(thinkthen_optional_content_v1){1,JSON("{\"flag\":null}")};
        if(at==5) r[1].context=(thinkthen_optional_content_v1){1,JSON("{\"flag\":false}")};
        if(at==6) r[1].original=(thinkthen_optional_content_v1){1,TEXT("{\"body\":\"good\"}")};
        thinkthen_source *s=source(e,r,2); thinkthen_result *result=(thinkthen_result *)(uintptr_t)1;
        assert(thinkthen_decide_complete(e,q,s,&c,&result)==THINKTHEN_EUSAGE && result==(thinkthen_result *)(uintptr_t)1);
        const char *message=thinkthen_error_message(e); assert(!strstr(message,"flag") && !strstr(message,"good"));
        thinkthen_result *error=NULL; assert(thinkthen_error_complete(e,&error)==0 && error);
        thinkthen_summary_v1 summary={0}; assert(thinkthen_result_summary(error,&summary)==0);
        assert(summary.error.value.code==THINKTHEN_EUSAGE && (!summary.facts.present || summary.facts.value.requests_sent==0));
        thinkthen_result_free(error); thinkthen_question_free(q); thinkthen_source_free(s);
    }
    thinkthen_question_spec_v1 spec={0}; spec.kind=1; spec.text=TEXT("Need attention?");
    for(int at=0;at<6;++at) {
        thinkthen_question_author_v1 a=metadata(1,0);
        if(at==0) a.name.value=STR("INVALID");
        if(at==1) a.wording_version.value=0;
        if(at==2) a.wording_version.value=UINT64_MAX;
        if(at==3) a.item_schema.properties.len=SIZE_MAX;
        if(at==4) a.item_schema.properties.data=NULL;
        if(at==5) a.item_schema.kind=99;
        thinkthen_question *q=(thinkthen_question *)(uintptr_t)1;
        assert(thinkthen_question_new_authored(e,&spec,&a,&q)==THINKTHEN_EUSAGE && q==(thinkthen_question *)(uintptr_t)1);
    }
    thinkthen_engine_free(e);
}
static void named(void) {
    thinkthen_engine *e=engine(); thinkthen_controls_v1 c=controls();
    const char *cwd=getenv("TYPED_WORKING_DIRECTORY"); assert(cwd && chdir(cwd)==0);
    thinkthen_record_v1 r={0}; r.original=(thinkthen_optional_content_v1){1,TEXT("Maria Chen joined Northwind Freight.")};
    thinkthen_source *s=source(e,&r,1);
    for(int at=0;at<2;++at) {
        thinkthen_question *q=NULL;
        int code=at==0?thinkthen_question_load_named(e,1,STR("catalog"),&q):thinkthen_question_load_reference(e,1,STR("@catalog"),&q);
        assert(code==0 && q); thinkthen_question_author_v1 a={0}; assert(thinkthen_question_author(q,&a)==0);
        assert(same(a.name.value,"catalog") && a.wording_version.value==(at==0?31u:32u));
        thinkthen_result *result=NULL; assert(thinkthen_decide_complete(e,q,s,&c,&result)==0); thinkthen_question_free(q);
        assert(thinkthen_result_question_author(result,0,&a)==0 && a.wording_version.value==(at==0?31u:32u)); thinkthen_result_free(result);
    }
    const char *names[]={"members","dynamic","recognition","relation"};
    thinkthen_record_v1 inputs[2]={r,r};
    inputs[1].original=(thinkthen_optional_content_v1){1,TEXT("Other entity.")};
    thinkthen_choice_v1 choices[2]={0}; choices[0].name=STR("first"); choices[1].name=STR("last");
    for(unsigned role=2;role<=5;++role) {
        thinkthen_string_v1 name={names[role-2],strlen(names[role-2])}; thinkthen_question *q=NULL;
        assert(thinkthen_question_load_named(e,role,name,&q)==0);
        inputs[0].options=role==3?(thinkthen_choices_v1){choices,2}:(thinkthen_choices_v1){0};
        thinkthen_source *records=source(e,inputs,role==5?2:1); thinkthen_result *result=NULL;
        int code=role==2?thinkthen_annotate_complete(e,q,records,&c,&result):role==3?thinkthen_choose_complete(e,q,records,&c,&result):role==4?thinkthen_recognize_complete(e,q,records,&c,&result):thinkthen_relate_complete(e,q,records,&c,&result);
        if(code) { fprintf(stderr,"native named role %u: %d %s\n",role,code,thinkthen_error_message(e)); abort(); }
        assert(result);
        thinkthen_question_free(q); thinkthen_source_free(records);
        thinkthen_question_author_v1 a={0}; assert(thinkthen_result_question_author(result,0,&a)==0);
        if(role==2) { assert(!a.name.present); assert(thinkthen_result_member_author(result,0,0,&a)==0 && same(a.name.value,"member-check")); }
        else assert(a.wording_version.present && a.wording_version.value==33);
        thinkthen_result_free(result);
    }
    const char *more[]={"ranking","rank-members","finding"};
    for(unsigned role=6;role<=8;++role) {
        thinkthen_question *loaded=NULL;
        assert(thinkthen_question_load_named(e,role,(thinkthen_string_v1){more[role-6],strlen(more[role-6])},&loaded)==0);
        inputs[0].options=(thinkthen_choices_v1){0};
        thinkthen_source *records=source(e,inputs,2); thinkthen_result *result=NULL;
        int code=role==8?thinkthen_find_complete(e,loaded,records,&c,&result):thinkthen_rank_complete(e,loaded,records,&c,&result);
        if(code) { fprintf(stderr,"native named role %u: %d %s\n",role,code,thinkthen_error_message(e)); abort(); }
        thinkthen_question_free(loaded); thinkthen_source_free(records);
        thinkthen_question_author_v1 a={0}; assert(thinkthen_result_question_author(result,0,&a)==0);
        if(role!=7) assert(a.wording_version.present && a.wording_version.value==34);
        else { size_t count=0; assert(thinkthen_result_rank_member_count(result,0,&count)==0 && count==2); }
        thinkthen_result_free(result);
    }
    thinkthen_question *q=(thinkthen_question *)(uintptr_t)1;
    assert(thinkthen_question_load_named(e,1,STR("mismatch"),&q)==THINKTHEN_ELOCAL && q==(thinkthen_question *)(uintptr_t)1);
    assert(thinkthen_question_load_reference(e,1,STR("catalog"),&q)==THINKTHEN_EUSAGE);
    assert(thinkthen_question_load_named(e,1,STR("../catalog"),&q)==THINKTHEN_EUSAGE);
    assert(thinkthen_question_load_reference(e,1,STR("@broken"),&q)==THINKTHEN_ELOCAL);
    thinkthen_source_free(s); thinkthen_engine_free(e);
}
int main(void) {
    const char *mode=getenv("TYPED_AUTHOR_MODE"); assert(mode);
    if(strcmp(mode,"success")==0) success(); else if(strcmp(mode,"refused")==0) refused(); else named();
    return 0;
}
