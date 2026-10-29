/* Database callers use sized rows and the separate generated SQL declarations. */
#include "thinkthen_sql.h"
#include "thinkthen_sql.h"
#include <assert.h>
#include <string.h>
static void ok(int32_t result, thinkthen_error *error) { assert(result == THINKTHEN_OK); assert(error->code == 0); }
int main(void) {
    thinkthen_error error = {0}; error.struct_size = sizeof(error);
    thinkthen_client *client = NULL;
    const char *settings = "{\"backend\":\"stub\"}", *policy = "stub:*";
    ok(thinkthen_sql_client_open_with_policy((const uint8_t *)settings, strlen(settings), (const uint8_t *)policy, strlen(policy), &client, &error), &error);
    thinkthen_client *unrestricted = NULL;
    ok(thinkthen_sql_client_open((const uint8_t *)settings, strlen(settings), &unrestricted, &error), &error);
    thinkthen_client_free(unrestricted);
    thinkthen_question *question = NULL;
    ok(thinkthen_question_new(THINKTHEN_FUNCTION_DECIDE, (const uint8_t *)"{literal?", 9, &question, &error), &error);
    thinkthen_sql_call *call = NULL;
    ok(thinkthen_sql_call_open_question(client, question, NULL, &call, &error), &error);
    thinkthen_row row = {0}; row.struct_size = sizeof(row);
    assert(thinkthen_sql_call_poll(call, 20, NULL, NULL, &row, &error) == THINKTHEN_NEEDS_INPUT);
    const uint8_t invalid[] = {0xff};
    thinkthen_sql_input records[] = {
        {THINKTHEN_RECORD_TEXT, (const uint8_t *)"evidence", 8},
        {THINKTHEN_RECORD_MISSING, NULL, 0},
        {THINKTHEN_RECORD_JSON, (const uint8_t *)"{broken", 7},
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
        assert(event == THINKTHEN_ERROR && error.code == THINKTHEN_CODE_LOCAL_CANCELLED); thinkthen_error_free(&error); break;
    }
    assert(thinkthen_sql_call_poll(call, 20, NULL, NULL, &row, &error) == THINKTHEN_END);
    thinkthen_sql_call_free(call);
    double cut=0.7, low=0.2, high=0.8;
    assert(thinkthen_sql_question_set_threshold(question,&cut,&low,&high,&error)==THINKTHEN_ERROR);
    assert(error.code==THINKTHEN_CODE_USAGE_THRESHOLD);thinkthen_error_free(&error);
    assert(thinkthen_sql_question_set_threshold(question,NULL,&low,NULL,&error)==THINKTHEN_ERROR);
    assert(error.code==THINKTHEN_CODE_USAGE_THRESHOLD);thinkthen_error_free(&error);
    ok(thinkthen_sql_question_set_threshold(question,NULL,&low,&high,&error),&error);
    thinkthen_question_free(question);question=NULL;
    const char *nested="{\"function\":\"annotate\",\"questions\":{\"bad\":{\"function\":\"rank\",\"question\":\"Rank?\",\"levels\":[\"low\",\"high\"]}}}";
    assert(thinkthen_sql_question_from_json((const uint8_t*)nested,strlen(nested),&question,&error)==THINKTHEN_ERROR);
    assert(error.code==THINKTHEN_CODE_USAGE_QUESTION);thinkthen_error_free(&error);
    const char *canonical="{\"function\":\"decide\",\"question\":\"Ready?\",\"evidence\":\"/body\"}";
    ok(thinkthen_sql_question_from_json((const uint8_t*)canonical,strlen(canonical),&question,&error),&error);
    thinkthen_text name={(const uint8_t*)"audit",5};
    ok(thinkthen_sql_call_open_question(client,question,&name,&call,&error),&error);
    assert(thinkthen_sql_call_poll_postgres(call,20,NULL,NULL,&row,&error)==THINKTHEN_NEEDS_INPUT);
    const char *record="{\"id\":\"a\\u0000b\",\"body\":\"evidence\"}";
    thinkthen_sql_input input={THINKTHEN_RECORD_JSON,(const uint8_t*)record,strlen(record)};
    ok(thinkthen_sql_call_send_chunk(call,&input,1,&error),&error);thinkthen_sql_call_end_input(call);
    for(;;) {
        int32_t event=thinkthen_sql_call_poll_postgres(call,20,NULL,NULL,&row,&error);
        if(event==THINKTHEN_PENDING)continue;
        ok(event,&error);break;
    }
    assert(row.position==1 && row.status==THINKTHEN_STATUS_FAILED && row.error_code==THINKTHEN_CODE_RECORD_JSON);
    assert(row.id.data==NULL && row.answer.data==NULL && row.detail.data==NULL);
    assert(row.source.data!=NULL);
    thinkthen_text explicit_id={(const uint8_t*)"true",4};
    ok(thinkthen_sql_row_postgres_id(&row,&explicit_id,&error),&error);
    assert(row.id.len==4 && !memcmp(row.id.data,"true",4));
    const char *zero_id="\"a\\u0000b\"";
    explicit_id.data=(const uint8_t*)zero_id;explicit_id.len=strlen(zero_id);
    ok(thinkthen_sql_row_postgres_id(&row,&explicit_id,&error),&error);
    assert(row.id.data==NULL && row.error_code==THINKTHEN_CODE_RECORD_JSON);
    thinkthen_row_free(&row);
    assert(thinkthen_sql_call_poll_postgres(call,20,NULL,NULL,&row,&error)==THINKTHEN_END);
    thinkthen_sql_call_free(call);
    ok(thinkthen_sql_call_open_question(client,question,NULL,&call,&error),&error);
    assert(thinkthen_sql_call_poll_postgres(call,20,NULL,NULL,&row,&error)==THINKTHEN_NEEDS_INPUT);
    const char *zero_body="{\"id\":42,\"body\":\"a\\u0000b\"}";
    input.data=(const uint8_t*)zero_body;input.len=strlen(zero_body);
    ok(thinkthen_sql_call_send_chunk_postgres(call,&input,1,&error),&error);thinkthen_sql_call_end_input(call);
    for(;;) {
        int32_t event=thinkthen_sql_call_poll_postgres(call,20,NULL,NULL,&row,&error);
        if(event==THINKTHEN_PENDING)continue;
        ok(event,&error);break;
    }
    assert(row.status==THINKTHEN_STATUS_ANSWERED && row.error_code==0);
    assert(row.id.len==2 && !memcmp(row.id.data,"42",2));thinkthen_row_free(&row);
    assert(thinkthen_sql_call_poll_postgres(call,20,NULL,NULL,&row,&error)==THINKTHEN_END);
    thinkthen_question_free(question);question=NULL;
    const char *sql_rules="{\"function\":\"relate\",\"rules\":[{\"name\":\"works_for\",\"source\":\"person\",\"target\":\"organization\"}]}";
    ok(thinkthen_sql_question_from_json((const uint8_t*)sql_rules,strlen(sql_rules),&question,&error),&error);
    thinkthen_sql_call_free(call); thinkthen_question_free(question); thinkthen_client_free(client); return 0;
}
