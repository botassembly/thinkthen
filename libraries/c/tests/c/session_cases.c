/* Shared cases read complete typed packets after session and engine destruction. */
#include "session_views.c"
#include <string.h>
#include <sched.h>
int main(int argc, char **argv) {
    if(argc!=5) return 2;
    FILE *input=fopen(argv[1],"rb"); if(!input) return 2;
    if(fseek(input,0,SEEK_END)) return 2;
    long size=ftell(input); if(size<0||fseek(input,0,SEEK_SET)) return 2;
    char *bytes=malloc((size_t)size+1); if(!bytes) return 2;
    if(fread(bytes,1,(size_t)size,input)!=(size_t)size) return 2;
    fclose(input);
    thinkthen_engine *engine=thinkthen_engine_new_with(argv[2]);
    thinkthen_session *session=NULL;
    int code=engine?thinkthen_session_new(engine,bytes,(size_t)size,&session):thinkthen_error_code(NULL);
    free(bytes);
    if(code) {
        const char *message=engine?thinkthen_session_error_message():thinkthen_error_message(NULL);
        printf("{\"admission\":{\"code\":%d,\"message\":",code);
        quoted((thinkthen_complete_utf8_v1){message,strlen(message)});fputs("}}\n",stdout);
        thinkthen_engine_free(engine);return 0;
    }
    if(strcmp(argv[3],"cancel")==0) thinkthen_session_cancel(session);
    if(thinkthen_session_finish(session,NULL,0)) return 1;
    if(strcmp(argv[4],"held")==0) {
        if(getchar()!='!')return 3;
        thinkthen_session_cancel(session);puts("cancel-fired");fflush(stdout);
    }
    thinkthen_engine_free(engine);
    thinkthen_session_result **results=NULL;size_t count=0;
    for(;;) {
        uint32_t status;thinkthen_session_result *result=NULL;
        if(thinkthen_session_try_read(session,&status,&result)) return 1;
        if(status==THINKTHEN_SESSION_END_V1)break;
        if(status==THINKTHEN_SESSION_PENDING_V1){sched_yield();continue;}
        thinkthen_session_result **grown=realloc(results,(count+1)*sizeof(*results));if(!grown)return 1;
        results=grown;results[count++]=result;
    }
    thinkthen_session_free(session);
    fputs("{\"packets\":[",stdout);
    for(size_t i=0;i<count;++i) {
        const thinkthen_complete_session_packet_v1 *view=NULL;
        if(thinkthen_session_result_view(results[i],&view))return 1;
        if(i) { putchar(44); }
        emit_thinkthen_complete_session_packet_v1(*view);
    }
    fputs("],\"native\":[",stdout);
    for(size_t i=0;i<count;++i) {
        const char *json=NULL;size_t len=0;
        if(thinkthen_session_result_json(results[i],&json,&len))return 1;
        if(i) { putchar(44); }
        fwrite(json,1,len,stdout);
        thinkthen_session_result_free(results[i]);
    }
    fputs("]}\n",stdout);free(results);return 0;
}
