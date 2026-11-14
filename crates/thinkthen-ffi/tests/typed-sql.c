/* Database callers use sized rows and the separate generated SQL declarations. */
#include "thinkthen_sql.h"
#include "thinkthen_sql.h"
#include <assert.h>
#include <string.h>
#include <stdio.h>
static void ok(int32_t result, thinkthen_error *error) { assert(result == THINKTHEN_OK); assert(error->code == 0); }
static int contains(const thinkthen_row *row,const char *part){thinkthen_buffer json={0};assert(thinkthen_row_host_json(row,&json,NULL)==THINKTHEN_OK);int result=strstr((const char*)json.data,part)!=NULL;thinkthen_buffer_free(&json);return result;}
int main(void) {
    thinkthen_error error = {0}; error.struct_size = sizeof(error);
    thinkthen_client *client = NULL;
    const char *settings = "{\"backend\":\"stub\"}", *policy = "stub:*";
    ok(thinkthen_sql_client_open_with_policy((const char *)settings, strlen(settings), (const char *)policy, strlen(policy), &client, &error), &error);
    thinkthen_client *unrestricted = NULL;
    ok(thinkthen_sql_client_open((const char *)settings, strlen(settings), &unrestricted, &error), &error);
    thinkthen_client_free(unrestricted);
    thinkthen_question *question = NULL;
    ok(thinkthen_question_new(THINKTHEN_FUNCTION_DECIDE, (const char *)"{literal?", 9, &question, &error), &error);
    thinkthen_sql_call *call = NULL;
    ok(thinkthen_sql_call_open_question(client, question, NULL, &call, &error), &error);
    thinkthen_row row = {0}; row.struct_size = sizeof(row);
    assert(thinkthen_sql_call_poll(call, 20, NULL, NULL, &row, &error) == THINKTHEN_NEEDS_INPUT);
    const char invalid[] = {(char)0xff};
    thinkthen_sql_input records[] = {
        {THINKTHEN_RECORD_TEXT, (const char *)"evidence", 8},
        {THINKTHEN_RECORD_MISSING, NULL, 0},
        {THINKTHEN_RECORD_JSON, (const char *)"{broken", 7},
        {THINKTHEN_RECORD_TEXT, invalid, sizeof(invalid)}
    };
    ok(thinkthen_sql_call_send_chunk(call, records, sizeof(records)/sizeof(records[0]), &error), &error);
    thinkthen_sql_call_end_input(call);
    uint64_t position = 0;
    for (;;) {
        int32_t event = thinkthen_sql_call_poll(call, 20, NULL, NULL, &row, &error);
        if (event == THINKTHEN_END) break;
        if (event == THINKTHEN_PENDING) continue;
        ok(event, &error); assert(row.position == ++position);
        if (position > 1) {
            assert(row.status == THINKTHEN_STATUS_FAILED);
            assert(row.error_code == (position == 2 ? THINKTHEN_CODE_RECORD_MISSING : position == 3 ? THINKTHEN_CODE_RECORD_JSON : THINKTHEN_CODE_RECORD_INVALID));
        }
        thinkthen_row_free(&row);
    }
    assert(position == 4); thinkthen_sql_call_free(call);
    ok(thinkthen_sql_call_open_question(client, question, NULL, &call, &error), &error);
    thinkthen_sql_call_cancel(call);
    for (;;) {
        int32_t event = thinkthen_sql_call_poll(call, 20, NULL, NULL, &row, &error);
        if (event == THINKTHEN_PENDING || event == THINKTHEN_NEEDS_INPUT) continue;
        assert(event < 0 && error.code == THINKTHEN_CODE_LOCAL_CANCELLED); thinkthen_error_free(&error); break;
    }
    assert(thinkthen_sql_call_poll(call, 20, NULL, NULL, &row, &error) == THINKTHEN_END);
    thinkthen_sql_call_free(call);
    double cut=0.7, low=0.2, high=0.8;
    assert(thinkthen_sql_question_set_threshold(question,&cut,&low,&high,&error)< 0);
    assert(error.code==THINKTHEN_CODE_USAGE_THRESHOLD);thinkthen_error_free(&error);
    assert(thinkthen_sql_question_set_threshold(question,NULL,&low,NULL,&error)< 0);
    assert(error.code==THINKTHEN_CODE_USAGE_THRESHOLD);thinkthen_error_free(&error);
    ok(thinkthen_sql_question_set_threshold(question,NULL,&low,&high,&error),&error);
    thinkthen_question_free(question);question=NULL;
    const char *nested="{\"function\":\"annotate\",\"questions\":{\"bad\":{\"function\":\"rank\",\"question\":\"Rank?\",\"levels\":[\"low\",\"high\"]}}}";
    assert(thinkthen_sql_question_from_json((const char*)nested,strlen(nested),&question,&error)< 0);
    assert(error.code==THINKTHEN_CODE_USAGE_QUESTION);thinkthen_error_free(&error);
    const char *canonical="{\"function\":\"decide\",\"question\":\"Ready?\",\"field\":\"/body\"}";
    ok(thinkthen_sql_question_from_json((const char*)canonical,strlen(canonical),&question,&error),&error);
    thinkthen_text name={(const char*)"audit",5};
    ok(thinkthen_sql_call_open_question(client,question,&name,&call,&error),&error);
    assert(thinkthen_sql_call_poll_postgres(call,20,NULL,NULL,&row,&error)==THINKTHEN_NEEDS_INPUT);
    const char *record="{\"id\":\"a\\u0000b\",\"body\":\"evidence\"}";
    thinkthen_sql_input input={THINKTHEN_RECORD_JSON,(const char*)record,strlen(record)};
    ok(thinkthen_sql_call_send_chunk(call,&input,1,&error),&error);thinkthen_sql_call_end_input(call);
    for(;;) {
        int32_t event=thinkthen_sql_call_poll_postgres(call,20,NULL,NULL,&row,&error);
        if(event==THINKTHEN_PENDING)continue;
        ok(event,&error);break;
    }
    assert(row.position==1 && row.status==THINKTHEN_STATUS_FAILED && row.error_code==THINKTHEN_CODE_RECORD_JSON);
    assert(!contains(&row,"\"id\":") && !contains(&row,"\"answer\":") && !contains(&row,"\"detail\":"));
    assert(contains(&row,"\"source\":"));
    thinkthen_text explicit_id={(const char*)"true",4};
    ok(thinkthen_sql_row_postgres_id(call,&row,&explicit_id,&error),&error);
    assert(contains(&row,"\"id\":true"));
    const char *raw_ids[]={"7","\"selected\"","null","184467440737095516160000001","{\"arbitrary\":[true,null]}"};
    for(size_t n=0;n<sizeof raw_ids/sizeof raw_ids[0];n++){
        explicit_id.data=raw_ids[n];explicit_id.len=strlen(raw_ids[n]);
        ok(thinkthen_sql_row_postgres_id(call,&row,&explicit_id,&error),&error);
        char marker[128];snprintf(marker,sizeof marker,"\"id\":%s",raw_ids[n]);assert(contains(&row,marker));
        thinkthen_id selected={0},copied={0};thinkthen_value *retained=NULL;
        ok(thinkthen_row_id(&row,&selected,&error),&error);
        ok(thinkthen_row_values(&row,&retained,&error),&error);
        ok(thinkthen_value_id(retained,&copied,&error),&error);
        assert(selected.kind==copied.kind);
        if(n==0)assert(selected.kind==THINKTHEN_ID_SIGNED && selected.signed_value==7 && copied.signed_value==7);
        else if(n==1)assert(selected.kind==THINKTHEN_ID_STRING && selected.text.len==8 && !memcmp(selected.text.data,"selected",8) && copied.text.len==8);
        else assert(selected.kind==THINKTHEN_ID_ABSENT);
        thinkthen_value_free(retained);
        thinkthen_buffer canonical={0};ok(thinkthen_row_to_json(&row,&canonical,&error),&error);
        printf("%.*s\n",(int)canonical.len,(const char*)canonical.data);thinkthen_buffer_free(&canonical);
    }
    const char *zero_id="\"a\\u0000b\"";
    explicit_id.data=(const char*)zero_id;explicit_id.len=strlen(zero_id);
    ok(thinkthen_sql_row_postgres_id(call,&row,&explicit_id,&error),&error);
    assert(!contains(&row,"\"id\":") && row.error_code==THINKTHEN_CODE_RECORD_JSON);
    thinkthen_row_free(&row);
    assert(thinkthen_sql_call_poll_postgres(call,20,NULL,NULL,&row,&error)==THINKTHEN_END);
    thinkthen_sql_call_free(call);
    ok(thinkthen_sql_call_open_question(client,question,NULL,&call,&error),&error);
    assert(thinkthen_sql_call_poll_postgres(call,20,NULL,NULL,&row,&error)==THINKTHEN_NEEDS_INPUT);
    const char *zero_body="{\"id\":42,\"body\":\"a\\u0000b\"}";
    input.data=(const char*)zero_body;input.len=strlen(zero_body);
    ok(thinkthen_sql_call_send_chunk_postgres(call,&input,1,&error),&error);thinkthen_sql_call_end_input(call);
    for(;;) {
        int32_t event=thinkthen_sql_call_poll_postgres(call,20,NULL,NULL,&row,&error);
        if(event==THINKTHEN_PENDING)continue;
        ok(event,&error);break;
    }
    assert(row.status==THINKTHEN_STATUS_ANSWERED && row.error_code==0);
    assert(contains(&row,"\"id\":42"));
    assert(thinkthen_sql_call_set_raise_if_failed(call,2,&error)<0);
    assert(error.code==THINKTHEN_CODE_USAGE_ABI);thinkthen_error_free(&error);
    ok(thinkthen_sql_call_set_raise_if_failed(call,1,&error),&error);
    const thinkthen_text nan_id={(const char*)"NaN",3};
    ok(thinkthen_sql_row_postgres_id(call,&row,&nan_id,&error),&error);
    assert(row.status==THINKTHEN_STATUS_FAILED && row.error_code==THINKTHEN_CODE_RECORD_JSON);
    assert(strstr(thinkthen_row_string(&row),"failed: record.json:"));
    bool stale=true;assert(thinkthen_row_answer_bool(&row,&stale,&error)==-THINKTHEN_CODE_RECORD_JSON && stale);
    thinkthen_value *snapshot=NULL;ok(thinkthen_row_values(&row,&snapshot,&error),&error);
    uint8_t stale_value=77;assert(thinkthen_value_bool(snapshot,&stale_value,&error)==-THINKTHEN_CODE_RECORD_JSON && stale_value==77);
    thinkthen_value_free(snapshot);
    thinkthen_row_free(&row);
    assert(thinkthen_sql_call_poll_postgres(call,20,NULL,NULL,&row,&error)<0);
    assert(error.code==THINKTHEN_CODE_RECORD_JSON);
    const char summary[]="1 of 1 records failed; the first, at position 1";
    assert(error.message.len>=sizeof(summary)-1 && !memcmp(error.message.data,summary,sizeof(summary)-1));
    thinkthen_error_free(&error);
    assert(thinkthen_sql_call_poll_postgres(call,20,NULL,NULL,&row,&error)==THINKTHEN_END);
    thinkthen_question_free(question);question=NULL;
    const char *sql_rules="{\"function\":\"relate\",\"rules\":[{\"relation\":\"works_for\",\"source\":\"person\",\"target\":\"organization\"}]}";
    ok(thinkthen_sql_question_from_json((const char*)sql_rules,strlen(sql_rules),&question,&error),&error);
    thinkthen_question_free(question); question=NULL;
    const char *named_rules="{\"function\":\"relate\",\"rules\":[{\"name\":\"works_for\",\"source\":\"person\",\"target\":\"organization\"}]}";
    assert(thinkthen_sql_question_from_json(named_rules,strlen(named_rules),&question,&error)<0);
    assert(error.code==THINKTHEN_CODE_USAGE_QUESTION);
    thinkthen_error_free(&error);
    thinkthen_sql_call_free(call); thinkthen_question_free(question); thinkthen_client_free(client); return 0;
}
