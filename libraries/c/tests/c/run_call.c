/* Installed blocking caller: callbacks stay here; errors move to another thread. */
#ifndef _WIN32
#define _POSIX_C_SOURCE 200809L
#endif
#include <thinkthen.h>
#include "platform.h"
#include <string.h>
static thinkthen_complete_utf8 text(const char *s) {
    return (thinkthen_complete_utf8){s,strlen(s)};
}
static void quoted(thinkthen_complete_utf8 s) {
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
static uint32_t produce(void *context, thinkthen_complete_utf8 *out) {
    struct producer *p=context;
#ifndef _WIN32
    if(!pthread_equal(p->caller,pthread_self()))abort();
#endif
    /* Reentry must remain safe and must not change call ownership. */
    thinkthen_call_cancel *token=thinkthen_call_cancel_new();
    if(!token)abort();
    thinkthen_call_cancel_free(token);
    if(p->failed)return THINKTHEN_CALL_PRODUCER_FAILURE;
    if(p->index==p->count)return THINKTHEN_CALL_PRODUCER_END;
    *out=text(p->items[p->index++]);return THINKTHEN_CALL_PRODUCER_ITEM;
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
    thinkthen_complete_utf8 facts=thinkthen_call_error_facts_json(error);
    fputs(",\"facts\":",stdout);
    if(facts.len)fwrite(facts.data,1,facts.len,stdout);else fputs("null",stdout);
    puts("}");return THREAD_DONE;
}
/* Constructors still expose their separate immediate diagnostic while shipped.
   A refusal here creates no engine, call result or owned call error. */
static int constructor_error(void) {
    const thinkthen_error_details *error=thinkthen_error_view(NULL);
    if(!error||error->facts)return 3;
    printf("{\"constructor\":true,\"code\":%d,\"retryable\":%s,\"message\":",
           error->code,error->retryable?"true":"false");
    quoted(error->message);puts(",\"facts\":null}");
    return 0;
}
static THREAD_RESULT print_result(void *argument) {
    thinkthen_complete_utf8 json=thinkthen_call_result_json(argument);
    fwrite(json.data,1,json.len,stdout);putchar('\n');
    return THREAD_DONE;
}
/* No platform state base disables usage persistence during constructor capture.
   Restore the owned home before calls resolve their named question files. */
static thinkthen_engine *engine_without_usage(const char *settings) {
    const char *names[]={"HOME","XDG_STATE_HOME","LOCALAPPDATA"};
    char *saved[3]={NULL,NULL,NULL};
    for(size_t i=0;i<3;++i) {
        const char *value=getenv(names[i]);
        if(value) {
            saved[i]=malloc(strlen(value)+1);
            if(!saved[i])abort();
            strcpy(saved[i],value);
        }
#ifdef _WIN32
        if(_putenv_s(names[i],""))abort();
#else
        if(unsetenv(names[i]))abort();
#endif
    }
    thinkthen_engine *engine=thinkthen_engine_new_with(settings);
    for(size_t i=0;i<3;++i) {
#ifdef _WIN32
        if(_putenv_s(names[i],saved[i]?saved[i]:""))abort();
#else
        if(saved[i]?setenv(names[i],saved[i],1):unsetenv(names[i]))abort();
#endif
        free(saved[i]);
    }
    return engine;
}
/* Large image cases stay in an owned input file instead of OS argument storage. */
static char *arguments(const char *path, int *count, char ***values) {
    FILE *input=fopen(path,"rb");
    if(!input||fseek(input,0,SEEK_END))return NULL;
    long size=ftell(input);
    if(size<0||fseek(input,0,SEEK_SET))return NULL;
    char *bytes=malloc((size_t)size+1);
    if(!bytes||fread(bytes,1,(size_t)size,input)!=(size_t)size)return NULL;
    fclose(input);bytes[size]=0;
    size_t lines=1;
    for(long i=0;i<size;++i)if(bytes[i]=='\n')++lines;
    char **items=calloc(lines+1,sizeof(*items));
    if(!items)return NULL;
    items[0]=NULL;*count=1;
    char *start=bytes;
    for(long i=0;i<size;++i)if(bytes[i]=='\n') {
        bytes[i]=0;items[(*count)++]=start;start=bytes+i+1;
    }
    *values=items;return bytes;
}
int main(int argc,char **argv) {
    if(argc!=2)return 2;
    binary_streams();
    char *bytes=arguments(argv[1],&argc,&argv);
    if(!bytes||argc<8)return 2;
    thinkthen_engine *engine=engine_without_usage(argv[1]);
    if(!engine) {
        int status=constructor_error();
        free(argv);free(bytes);return status;
    }
    thinkthen_named_request request={text(argv[2]),text(argv[3]),text(argv[4]),text(argv[5])};
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
    int code=thinkthen_run_call(engine,&request,(!strcmp(argv[7],"plain")||!strcmp(argv[7],"missing"))?NULL:produce,&p,cancel,&result,&error);
    if(held&&fixture_join(worker))return 4;
    thinkthen_engine_free(engine);thinkthen_call_cancel_free(cancel);
    if(code) {
        if(!error||result||thinkthen_call_error_code(error)!=code)return 5;
        /* Failure details survive the producing thread and engine lifetime. */
        if(fixture_start(&worker,print_error,error)||fixture_join(worker))return 6;
        thinkthen_call_error_free(error);
    } else {
        if(!result||error)return 5;
        /* Result readers also survive the engine and move to another thread. */
        if(fixture_start(&worker,print_result,result)||fixture_join(worker))return 6;
        thinkthen_call_result_free(result);
    }
    free(argv);free(bytes);
    return 0;
}
