/* Public C ownership and refusal promises supplement the literal guide branches. */
#include <thinkthen.h>
#include <assert.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static thinkthen_error error=THINKTHEN_ERROR_INIT;
static thinkthen_client *client;
static void ok(int rc) { if(rc!=THINKTHEN_OK)fprintf(stderr,"%s: %s\n",thinkthen_code_name(error.code),thinkthen_error_string(&error));assert(rc==THINKTHEN_OK); }
static thinkthen_question *question(const char *json) {
    thinkthen_question *result=NULL;
    ok(thinkthen_question_from_json(json,THINKTHEN_NUL_TERMINATED,&result,&error));return result;
}
static thinkthen_row run(thinkthen_question *q,const char *text) {
    thinkthen_row row=THINKTHEN_ROW_INIT;
    ok(thinkthen_run_one(client,q,text,THINKTHEN_NUL_TERMINATED,THINKTHEN_RECORD_TEXT,&row,&error));return row;
}
static thinkthen_row bulk(thinkthen_question *q,const char *text) {
    const thinkthen_record_input input={sizeof input,THINKTHEN_RECORD_TEXT,text,strlen(text)};
    thinkthen_call *call=NULL;thinkthen_row row=THINKTHEN_ROW_INIT,end=THINKTHEN_ROW_INIT;
    ok(thinkthen_call_open(client,q,&input,1,&call,&error));
    ok(thinkthen_call_next(call,&row,&error));
    assert(thinkthen_call_next(call,&end,&error)==THINKTHEN_END);
    thinkthen_row_free(&end);thinkthen_call_free(call);return row;
}
static void states(const char *scenario) {
    thinkthen_question *q=question("{\"function\":\"decide\",\"question\":\"Does the customer request a refund?\",\"threshold\":{\"low\":0.2,\"high\":0.8}}");
    const char failure_message[]="the backend answered status 503: {\"error\":\"unavailable\"}";
    thinkthen_row row=THINKTHEN_ROW_INIT;
    int call_status=thinkthen_run_one(client,q,"Please refund my order.",THINKTHEN_NUL_TERMINATED,THINKTHEN_RECORD_TEXT,&row,&error);
    if(!strcmp(scenario,"failed")) {
        assert(call_status==-THINKTHEN_CODE_BACKEND_STATUS && error.http_status==503);
        assert(row.private_data==NULL && error.code==THINKTHEN_CODE_BACKEND_STATUS);
        assert(!strcmp(thinkthen_code_family(error.code),"backend"));
        assert(error.message.len==sizeof failure_message-1 && !memcmp(error.message.data,failure_message,error.message.len));
        row=bulk(q,"Please refund my order.");
    } else {ok(call_status);assert(row.status!=THINKTHEN_STATUS_FAILED);}
    bool yes=true;
    int rc=thinkthen_row_answer_bool(&row,&yes,&error);
    if(!strcmp(scenario,"unsure")){assert(rc==THINKTHEN_UNSURE && yes);assert(!strcmp(thinkthen_row_string(&row),"unsure"));ok(thinkthen_row_err_if_failed(&row,&error));}
    else if(!strcmp(scenario,"failed")){assert(rc==-THINKTHEN_CODE_BACKEND_STATUS && yes);assert(strstr(thinkthen_row_string(&row),"failed: backend.status:"));assert(thinkthen_row_err_if_failed(&row,&error)==rc);assert(error.http_status==503);assert(strstr(thinkthen_error_string(&error),"backend.status: ")==thinkthen_error_string(&error));assert(error.message.len==sizeof failure_message-1 && !memcmp(error.message.data,failure_message,error.message.len));}
    else {ok(rc);assert(yes==(!strcmp(scenario,"yes")));assert(!strcmp(thinkthen_row_string(&row),yes?"true":"false"));}
    thinkthen_buffer saved={0};ok(thinkthen_row_to_json(&row,&saved,&error));assert(saved.len>0);
    thinkthen_row moved=row;row=(thinkthen_row)THINKTHEN_ROW_INIT;
    thinkthen_row_free(&row);assert(thinkthen_row_string(&row)!=NULL);
    assert(thinkthen_row_string(&moved)!=NULL);thinkthen_row_free(&moved);
    assert(saved.data[0]=='{');thinkthen_buffer_free(&saved);thinkthen_question_free(q);
}
static void mismatches(void) {
    thinkthen_question *q=question("{\"function\":\"choose\",\"question\":\"Topic?\",\"choices\":[\"billing\",\"shipping\",\"other\"]}");
    thinkthen_row row=bulk(q,"body"),entry=THINKTHEN_ROW_INIT;
    bool boolean=true,tags[3]={true,true,true};double number=77;uint64_t rank=77;
    thinkthen_span span={0};span.start=77;
    const thinkthen_name *names=NULL;const thinkthen_relation *relations=NULL;size_t count=77;
    const int mismatch=-THINKTHEN_CODE_USAGE_FUNCTION;
    assert(thinkthen_row_answer_bool(&row,&boolean,&error)==mismatch && boolean);
    assert(thinkthen_row_answer_score(&row,&number,&error)==mismatch && number==77);
    assert(thinkthen_row_answer_rank(&row,&rank,&error)==mismatch && rank==77);
    assert(thinkthen_row_answer_span(&row,&span,&error)==mismatch && span.start==77);
    assert(thinkthen_row_answer_tags(&row,tags,3,&error)==mismatch && tags[0] && tags[1] && tags[2]);
    assert(thinkthen_row_answer_names(&row,&names,&count,&error)==mismatch && !names && count==77);
    assert(thinkthen_row_answer_relations(&row,&relations,&count,&error)==mismatch && !relations && count==77);
    assert(thinkthen_row_answer_entry(&row,"missing",THINKTHEN_NUL_TERMINATED,&entry,&error)==mismatch && !entry.private_data);
    thinkthen_value *value=NULL;ok(thinkthen_row_values(&row,&value,&error));uint8_t bit=77;
    assert(thinkthen_value_bool(value,&bit,&error)==mismatch && bit==77);
    assert(thinkthen_value_number(value,&number,&error)==mismatch && number==77);
    assert(thinkthen_value_rank(value,&rank,&error)==mismatch && rank==77);
    assert(thinkthen_value_span(value,&span,&error)==mismatch && span.start==77);
    assert(thinkthen_value_tag_count(value,&count,&error)==mismatch && count==77);
    assert(thinkthen_value_name_count(value,&count,&error)==mismatch && count==77);
    assert(thinkthen_value_relation_count(value,&count,&error)==mismatch && count==77);
    assert(thinkthen_value_annotation_count(value,&count,&error)==mismatch && count==77);
    thinkthen_value_free(value);thinkthen_row_free(&entry);thinkthen_row_free(&row);thinkthen_question_free(q);
}
static void ownership(void) {
    const char *text="😀 Morgan works for Example Group.\nPlease refund my order.";
    const char *definition="{\"function\":\"annotate\",\"questions\":{\"action\":{\"function\":\"find\",\"question\":\"Which line states the requested action?\"},\"topic\":{\"function\":\"choose\",\"question\":\"What is the message about?\",\"choices\":[\"billing\",\"shipping\",\"other\"]}}}";
    for(unsigned unit=THINKTHEN_OFFSET_UTF8;unit<=THINKTHEN_OFFSET_UTF16;unit++) {
        thinkthen_question *q=question(definition);
        thinkthen_call *call=NULL;
        const thinkthen_record_input input=THINKTHEN_TEXT_RECORD(text);
        ok(thinkthen_call_open_records_question(client,q,NULL,&input,1,sizeof input,unit,&call,&error));
        thinkthen_question_free(q);
        thinkthen_row parent=THINKTHEN_ROW_INIT,child=THINKTHEN_ROW_INIT,topic=THINKTHEN_ROW_INIT;
        ok(thinkthen_call_next(call,&parent,&error));
        thinkthen_text selected={0};ok(thinkthen_row_selected_text(&parent,"action",THINKTHEN_NUL_TERMINATED,&selected,&error));
        assert(selected.len==strlen(text) && !memcmp(selected.data,text,selected.len));
        thinkthen_value *snapshot=NULL;ok(thinkthen_row_values(&parent,&snapshot,&error));
        ok(thinkthen_row_answer_entry(&parent,"action",THINKTHEN_NUL_TERMINATED,&child,&error));
        ok(thinkthen_row_answer_entry(&parent,"topic",THINKTHEN_NUL_TERMINATED,&topic,&error));
        thinkthen_call_free(call);
        q=question("{\"function\":\"decide\",\"question\":\"Ready?\"}");
        assert(thinkthen_run_one(client,q,"",0,THINKTHEN_RECORD_TEXT,&parent,&error)==-THINKTHEN_CODE_RECORD_EMPTY);
        assert(parent.private_data==NULL);
        thinkthen_row_free(&parent);thinkthen_question_free(q);
        ok(thinkthen_row_selected_text(&child,NULL,0,&selected,&error));
        assert(selected.len==strlen(text) && !memcmp(selected.data,text,selected.len));
        thinkthen_span span={0};ok(thinkthen_row_answer_span(&child,&span,&error));
        size_t bytes=(size_t)(strstr(text,"Please")-text);
        assert(span.present && span.start==bytes-(unit==THINKTHEN_OFFSET_UTF16?2:0));
        thinkthen_buffer saved={0};ok(thinkthen_row_to_json(&child,&saved,&error));
        char expected[64];snprintf(expected,sizeof expected,"\"start\":%zu",bytes-3);assert(strstr((const char*)saved.data,expected));thinkthen_buffer_free(&saved);
        thinkthen_buffer host={0};ok(thinkthen_row_host_json(&child,&host,&error));
        snprintf(expected,sizeof expected,"\"start\":%zu",bytes-(unit==THINKTHEN_OFFSET_UTF16?2:0));assert(strstr((const char*)host.data,expected));thinkthen_buffer_free(&host);
        int choice=-1;ok(thinkthen_row_answer_choice(&topic,&choice,&error));assert(choice==0);
        thinkthen_value *entry=NULL;ok(thinkthen_value_annotation_get(snapshot,"action",THINKTHEN_NUL_TERMINATED,&entry,&error));
        ok(thinkthen_value_span(entry,&span,&error));assert(span.start==bytes-(unit==THINKTHEN_OFFSET_UTF16?2:0));
        thinkthen_record_view original={0};ok(thinkthen_value_original(snapshot,&original,&error));assert(original.len==strlen(text) && !memcmp(original.data,text,original.len));
        thinkthen_value_free(entry);thinkthen_value_free(snapshot);thinkthen_row_free(&child);thinkthen_row_free(&topic);
    }
}
static void selected(void) {
    const char *json="{\"id\":\"kept\",\"body\":\"😀 Before.\\nPlease refund my order.\",\"other\":\"É Other.\\nPlease refund my order.\"}";
    const char *body="😀 Before.\nPlease refund my order.",*other="É Other.\nPlease refund my order.";
    thinkthen_question *q=question("{\"function\":\"annotate\",\"field\":\"/body\",\"questions\":{\"action\":{\"function\":\"find\",\"question\":\"Which line states the requested action?\"},\"override\":{\"function\":\"find\",\"question\":\"Which line states the requested action?\",\"field\":\"/other\"}}}");
    thinkthen_row row=THINKTHEN_ROW_INIT,child=THINKTHEN_ROW_INIT;
    ok(thinkthen_run_one(client,q,json,THINKTHEN_NUL_TERMINATED,THINKTHEN_RECORD_JSON,&row,&error));thinkthen_question_free(q);
    thinkthen_text text={0};ok(thinkthen_row_selected_text(&row,"action",THINKTHEN_NUL_TERMINATED,&text,&error));assert(text.len==strlen(body) && !memcmp(text.data,body,text.len));
    ok(thinkthen_row_selected_text(&row,"override",THINKTHEN_NUL_TERMINATED,&text,&error));assert(text.len==strlen(other) && !memcmp(text.data,other,text.len));
    assert(thinkthen_row_selected_text(&row,"missing",THINKTHEN_NUL_TERMINATED,&text,&error)==-THINKTHEN_CODE_USAGE_NAME);
    ok(thinkthen_row_answer_entry(&row,"action",THINKTHEN_NUL_TERMINATED,&child,&error));thinkthen_row_free(&row);
    ok(thinkthen_row_selected_text(&child,NULL,0,&text,&error));thinkthen_span span={0};ok(thinkthen_row_answer_span(&child,&span,&error));
    assert(span.present && span.end<=text.len && !memcmp(text.data+span.start,span.text.data,span.text.len));thinkthen_row_free(&child);
    q=question("{\"function\":\"find\",\"question\":\"Which line states the requested action?\"}");
    const char *object="{\"id\":\"drop\",\"b\":\"Second\",\"a\":\"First\"}";
    ok(thinkthen_run_one(client,q,object,THINKTHEN_NUL_TERMINATED,THINKTHEN_RECORD_JSON,&row,&error));thinkthen_question_free(q);
    ok(thinkthen_row_selected_text(&row,NULL,0,&text,&error));const char *normalized="{\"a\":\"First\",\"b\":\"Second\"}";assert(text.len==strlen(normalized) && !memcmp(text.data,normalized,text.len));thinkthen_row_free(&row);
}
static void forms(const char *scenario) {
    thinkthen_question *q=question("{\"function\":\"annotate\",\"questions\":{\"refund\":{\"function\":\"decide\",\"question\":\"Does the customer request a refund?\",\"threshold\":{\"low\":0.2,\"high\":0.8}},\"topic\":{\"function\":\"choose\",\"question\":\"What is the message about?\",\"choices\":[\"billing\",\"shipping\",\"other\"]},\"urgency\":{\"function\":\"score\",\"question\":\"How urgent is the request?\",\"levels\":[\"routine\",\"urgent\"]}}}");
    thinkthen_row row=run(q,"Please refund my order.");
    int rc=thinkthen_row_err_if_failed(&row,&error);
    assert(row.status==THINKTHEN_STATUS_ANSWERED && row.error_code==0);
    assert(rc==(!strcmp(scenario,"failed")?-THINKTHEN_CODE_BACKEND_STATUS:strstr(scenario,"-failed")?-THINKTHEN_CODE_BACKEND_REPLY:THINKTHEN_OK));
    if(!strcmp(scenario,"failed"))assert(error.http_status==503);
    thinkthen_row entry=THINKTHEN_ROW_INIT;
    assert(thinkthen_row_answer_entry(&row,"missing",THINKTHEN_NUL_TERMINATED,&entry,&error)==-THINKTHEN_CODE_USAGE_NAME);
    ok(thinkthen_row_answer_entry(&row,"topic",THINKTHEN_NUL_TERMINATED,&entry,&error));
    thinkthen_row blank=THINKTHEN_ROW_INIT;
    assert(thinkthen_run_one(client,q,"",0,THINKTHEN_RECORD_TEXT,&blank,&error)==-THINKTHEN_CODE_RECORD_EMPTY);
    assert(blank.private_data==NULL);
    blank=bulk(q,"");assert(blank.status==THINKTHEN_STATUS_FAILED);
    assert(thinkthen_row_err_if_failed(&blank,&error)==-THINKTHEN_CODE_RECORD_EMPTY);thinkthen_row_free(&blank);
    thinkthen_row_free(&row);thinkthen_question_free(q);
    int choice=99;rc=thinkthen_row_answer_choice(&entry,&choice,&error);
    if(!strcmp(scenario,"failed"))assert(rc==-THINKTHEN_CODE_BACKEND_STATUS && choice==99);
    else if(!strcmp(scenario,"form-topic-failed"))assert(rc==-THINKTHEN_CODE_BACKEND_REPLY && choice==99);
    else if(!strcmp(scenario,"form-topic-unsure"))assert(rc==THINKTHEN_UNSURE && choice==99);
    else {ok(rc);assert(choice==0);}
    thinkthen_row_free(&entry);
}
static void admission(void) {
    thinkthen_question *q=question("{\"function\":\"choose\",\"question\":\"Topic?\",\"choices\":[\"shipping\",\"billing\",\"other\"]}");
    const char *const words[]={"shipping","billing","other"},*const wrong[]={"billing","shipping","other"};
    ok(thinkthen_question_expect_choices(q,words,3,&error));
    assert(thinkthen_question_expect_choices(q,wrong,3,&error)==-THINKTHEN_CODE_USAGE_QUESTION);
    const char *const gap[]={"billing",NULL,"other"};
    assert(thinkthen_question_set_choices(q,gap,3,&error)==-THINKTHEN_CODE_USAGE_QUESTION);
    thinkthen_row row=run(q,"body");int choice=-1;ok(thinkthen_row_answer_choice(&row,&choice,&error));assert(!strcmp(thinkthen_row_string(&row),"billing") && choice==1);
    bool yes=true;assert(thinkthen_row_answer_bool(&row,&yes,&error)==-THINKTHEN_CODE_USAGE_FUNCTION && yes);
    thinkthen_row_free(&row);thinkthen_question_free(q);
    q=question("{\"function\":\"decide\",\"question\":\"Ready?\"}");
    assert(thinkthen_question_set_choices(q,NULL,0,&error)==-THINKTHEN_CODE_USAGE_QUESTION);
    assert(thinkthen_question_set_labels(q,NULL,0,&error)==-THINKTHEN_CODE_USAGE_QUESTION);
    assert(thinkthen_question_set_levels(q,NULL,0,&error)==-THINKTHEN_CODE_USAGE_QUESTION);
    thinkthen_usage before={.struct_size=sizeof before},after={.struct_size=sizeof after};ok(thinkthen_client_usage_values(client,&before,&error));
    assert(thinkthen_run_one(client,q," \t\n",THINKTHEN_NUL_TERMINATED,THINKTHEN_RECORD_TEXT,&row,&error)==-THINKTHEN_CODE_RECORD_EMPTY);
    assert(row.private_data==NULL && error.code==THINKTHEN_CODE_RECORD_EMPTY);thinkthen_row_free(&row);
    ok(thinkthen_client_usage_values(client,&after,&error));assert(before.attempts==after.attempts);thinkthen_question_free(q);
    q=question("{\"function\":\"filter\",\"question\":\"Ready?\"}");
    assert(thinkthen_run_one(client,q,"body",4,THINKTHEN_RECORD_TEXT,&row,&error)==-THINKTHEN_CODE_USAGE_FUNCTION);
    assert(thinkthen_question_set_band(q,0.2,0.8,&error)==-THINKTHEN_CODE_USAGE_THRESHOLD);
    thinkthen_question *form=NULL;ok(thinkthen_question_new(THINKTHEN_FUNCTION_ANNOTATE,NULL,0,&form,&error));
    assert(thinkthen_question_add_entry(form,"bad",THINKTHEN_NUL_TERMINATED,q,&error)==-THINKTHEN_CODE_USAGE_QUESTION);
    thinkthen_question_free(form);thinkthen_question_free(q);
    q=question("{\"function\":\"rank\",\"question\":\"Urgent?\",\"levels\":[\"routine\",\"urgent\"]}");
    assert(thinkthen_run_one(client,q,"body",4,THINKTHEN_RECORD_TEXT,&row,&error)==-THINKTHEN_CODE_USAGE_FUNCTION);thinkthen_question_free(q);
}
static void filter(void) {
    thinkthen_question *q=question("{\"function\":\"filter\",\"question\":\"Ready?\"}");
    const thinkthen_record_input inputs[]={THINKTHEN_TEXT_RECORD(""),THINKTHEN_TEXT_RECORD("body"),THINKTHEN_TEXT_RECORD(" ")};
    thinkthen_call *call=NULL;ok(thinkthen_call_open(client,q,inputs,3,&call,&error));thinkthen_question_free(q);
    thinkthen_row row=THINKTHEN_ROW_INIT;
    assert(thinkthen_call_next(call,&row,&error)==-THINKTHEN_CODE_USAGE_FUNCTION);
    ok(thinkthen_call_next_kept(call,&row,&error));assert(row.position==2);
    thinkthen_record_view original={0};ok(thinkthen_row_original(&row,&original,&error));assert(original.len==4 && !memcmp(original.data,"body",4));
    assert(thinkthen_call_next_kept(call,&row,&error)==-THINKTHEN_CODE_RECORD_EMPTY);
    assert(strstr(thinkthen_error_string(&error),"2 of 3 records failed"));
    ok(thinkthen_call_next_failed(call,&row,&error));assert(row.position==1);ok(thinkthen_row_original(&row,&original,&error));assert(original.len==0);
    ok(thinkthen_call_next_failed(call,&row,&error));assert(row.position==3);
    assert(thinkthen_call_next_failed(call,&row,&error)==THINKTHEN_END);
    thinkthen_row_free(&row);thinkthen_call_free(call);
}
static void details(const char *scenario) {
    int failed=!strcmp(scenario,"failed");
    thinkthen_question *q=question("{\"function\":\"choose\",\"question\":\"What is the message about?\",\"choices\":[\"billing\",\"shipping\",\"other\"]}");
    thinkthen_row row=bulk(q,"body");thinkthen_question_free(q);
    const thinkthen_probability *labels=NULL;size_t count=99;
    ok(thinkthen_row_detail_labels(&row,&labels,&count,&error));assert(count==(failed?0:3));if(count)assert(labels[0].label.data!=NULL);thinkthen_row_free(&row);
    q=question("{\"function\":\"score\",\"question\":\"How urgent is the request?\",\"levels\":[\"routine\",\"urgent\"]}");
    row=bulk(q,"body");thinkthen_question_free(q);const double *levels=NULL;count=99;
    ok(thinkthen_row_detail_levels(&row,&levels,&count,&error));assert(count==(failed?0:2));if(count)assert(levels[1]==1);thinkthen_row_free(&row);
    q=question("{\"function\":\"recognize\",\"kinds\":[\"person\",\"organization\"]}");
    row=bulk(q,"Morgan works for Example Group.");thinkthen_question_free(q);
    thinkthen_recognition_detail detail={0};ok(thinkthen_row_detail_pieces(&row,&detail,&error));
    if(failed)assert(detail.piece_count==0 && detail.name_count==0);
    else {assert(detail.piece_count>0 && detail.name_count==2);assert(detail.pieces[0].span.text.len>0 && detail.pieces[0].tag_count==5);assert(detail.names[0].kind_count==3);}
    thinkthen_row_free(&row);
    q=question("{\"function\":\"relate\",\"rules\":[{\"source\":\"person\",\"target\":\"organization\",\"relation\":\"works_for\"}]}");
    row=bulk(q,"Morgan works for Example Group.");thinkthen_question_free(q);
    const thinkthen_relation *pairs=NULL;const thinkthen_name *names=NULL;
    ok(thinkthen_row_detail_pairs(&row,&pairs,&count,&error));assert(count==(failed?0:1));if(count)assert(pairs[0].relation.len==9);
    ok(thinkthen_row_detail_names(&row,&names,&count,&error));assert(count==(failed?0:2));
    thinkthen_row_free(&row);
}
static void cancel(void) {
    thinkthen_cancel *stop=NULL;thinkthen_settings *settings=NULL;thinkthen_client *stopped=NULL;
    ok(thinkthen_cancel_new(&stop,&error));ok(thinkthen_settings_new(&settings,&error));ok(thinkthen_settings_set_cancel(settings,stop,&error));
    ok(thinkthen_client_open(settings,&stopped,&error));thinkthen_settings_free(settings);
    thinkthen_cancel_request(stop);thinkthen_cancel_free(stop);
    thinkthen_question *q=question("{\"function\":\"decide\",\"question\":\"Ready?\"}");thinkthen_row row=THINKTHEN_ROW_INIT;
    assert(thinkthen_run_one(stopped,q,"body",4,THINKTHEN_RECORD_TEXT,&row,&error)==-THINKTHEN_CODE_LOCAL_CANCELLED);
    thinkthen_client_free(stopped);thinkthen_question_free(q);thinkthen_row_free(&row);
}
int main(int argc,char **argv) {
    assert(argc==3);ok(thinkthen_client_open(NULL,&client,&error));
    if(!strcmp(argv[1],"states"))states(argv[2]);
    else if(!strcmp(argv[1],"mismatches"))mismatches();
    else if(!strcmp(argv[1],"ownership"))ownership();
    else if(!strcmp(argv[1],"selected"))selected();
    else if(!strcmp(argv[1],"forms"))forms(argv[2]);
    else if(!strcmp(argv[1],"admission"))admission();
    else if(!strcmp(argv[1],"filter"))filter();
    else if(!strcmp(argv[1],"details"))details(argv[2]);
    else if(!strcmp(argv[1],"cancel"))cancel();else abort();
    thinkthen_client_free(client);thinkthen_error_free(&error);puts(argv[1]);return 0;
}
