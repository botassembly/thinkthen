/* Installed blocking caller: callbacks stay here; errors move to another thread. */
#include <thinkthen.h>
#include "platform.h"
#include <string.h>
static thinkthen_complete_utf8_v1 text(const char *s) {
    return (thinkthen_complete_utf8_v1){s,strlen(s)};
}
static void quoted(thinkthen_complete_utf8_v1 s) {
    putchar('"');
    for(size_t i=0;i<s.len;i++) {
        unsigned char c=(unsigned char)s.data[i];
        if(c=='"'||c=='\\') {putchar('\\');putchar(c);}
        else if(c<32) printf("\\u%04x",c);
        else putchar(c);
    }
    putchar('"');
}
struct producer {int index,count;char **items;THREAD_TYPE caller;int failed;};
static uint32_t produce(void *context, thinkthen_complete_utf8_v1 *out) {
    struct producer *p=context;
#ifndef _WIN32
    if(!pthread_equal(p->caller,pthread_self()))abort();
#endif
    /* Reentry must remain safe and must not change call ownership. */
    thinkthen_call_cancel *token=thinkthen_call_cancel_new();
    if(!token)abort();
    thinkthen_call_cancel_free(token);
    if(p->failed)return THINKTHEN_CALL_PRODUCER_FAILURE_V1;
    if(p->index==p->count)return THINKTHEN_CALL_PRODUCER_END_V1;
    *out=text(p->items[p->index++]);return THINKTHEN_CALL_PRODUCER_ITEM_V1;
}
static THREAD_RESULT cancel_held(void *argument) {
    if(getchar()!='!')abort();
    thinkthen_call_cancel_fire(argument);puts("cancel-fired");fflush(stdout);
    return THREAD_DONE;
}
static THREAD_RESULT print_error(void *argument) {
    thinkthen_call_error *error=argument;
    printf("{\"code\":%d,\"retryable\":%s,\"message\":",thinkthen_call_error_code(error),thinkthen_call_error_retryable(error)?"true":"false");
    quoted(thinkthen_call_error_message(error));
    thinkthen_complete_utf8_v1 facts=thinkthen_call_error_facts_json(error);
    fputs(",\"facts\":",stdout);
    if(facts.len)fwrite(facts.data,1,facts.len,stdout);else fputs("null",stdout);
    puts("}");return THREAD_DONE;
}
int main(int argc,char **argv) {
    if(argc<8)return 2;
    binary_streams();
    thinkthen_engine *engine=thinkthen_engine_new_with(argv[1]);if(!engine)return 3;
    thinkthen_named_request_v1 request={text(argv[2]),text(argv[3]),text(argv[4]),text(argv[5])};
    thinkthen_call_cancel *cancel=thinkthen_call_cancel_new();if(!cancel)return 3;
    if(!strcmp(argv[6],"cancel"))thinkthen_call_cancel_fire(cancel);
    struct producer p={0,argc-8,argv+8,0,!strcmp(argv[7],"fail")};
#ifndef _WIN32
    p.caller=pthread_self();
#endif
    THREAD_TYPE worker;
    int held=!strcmp(argv[6],"held");
    if(held&&fixture_start(&worker,cancel_held,cancel))return 4;
    thinkthen_call_result *result=NULL;
    thinkthen_call_error *error=NULL;
    /* Invalid output storage must refuse before touching a slot or producer. */
    if(thinkthen_run_call(engine,&request,NULL,NULL,cancel,NULL,&error)!=THINKTHEN_EUSAGE||error)return 7;
    if(thinkthen_run_call(engine,&request,NULL,NULL,cancel,&result,(thinkthen_call_error **)&result)!=THINKTHEN_EUSAGE||result)return 7;
    int code=thinkthen_run_call(engine,&request,!strcmp(argv[7],"plain")?NULL:produce,&p,cancel,&result,&error);
    if(held&&fixture_join(worker))return 4;
    thinkthen_engine_free(engine);thinkthen_call_cancel_free(cancel);
    if(code) {
        if(!error||result||thinkthen_call_error_code(error)!=code)return 5;
        /* Failure details survive the producing thread and engine lifetime. */
        if(fixture_start(&worker,print_error,error)||fixture_join(worker))return 6;
        thinkthen_call_error_free(error);
    } else {
        if(!result||error)return 5;
        thinkthen_complete_utf8_v1 json=thinkthen_call_result_json(result);
        fwrite(json.data,1,json.len,stdout);putchar('\n');
        thinkthen_call_result_free(result);
    }
    return 0;
}
