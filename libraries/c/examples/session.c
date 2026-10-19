/* Run one named call with owned canonical JSON from the installed C package. */
#include <thinkthen.h>
#include <stdio.h>
#include <string.h>
#ifdef _WIN32
#include <fcntl.h>
#include <io.h>
#endif
static thinkthen_complete_utf8_v1 text(const char *s) {
    return (thinkthen_complete_utf8_v1){s,strlen(s)};
}
int main(void) {
#ifdef _WIN32
    if (_setmode(_fileno(stdout), _O_BINARY) == -1 ||
        _setmode(_fileno(stderr), _O_BINARY) == -1) {
        perror("set binary output mode");
        return 1;
    }
#endif
    thinkthen_engine *engine=thinkthen_engine_new_with("{\"cache\":false,\"max_retries\":0}");
    if(!engine)return 1;
    thinkthen_named_request_v1 request={text("decide"),text("{\"kind\":\"text\",\"text\":\"Does it pass?\"}"),
        text("{\"kind\":\"text\",\"text\":\"Evidence.\"}"),text("{}")};
    thinkthen_call_result *result=NULL;
    thinkthen_call_error *error=NULL;
    int status=thinkthen_run_call(engine,&request,NULL,NULL,NULL,&result,&error);
    thinkthen_engine_free(engine);
    if(status) {
        thinkthen_complete_utf8_v1 message=thinkthen_call_error_message(error);
        fwrite(message.data,1,message.len,stderr);fputc('\n',stderr);
        thinkthen_call_error_free(error);return 1;
    }
    thinkthen_complete_utf8_v1 json=thinkthen_call_result_json(result);
    fwrite(json.data,1,json.len,stdout);putchar('\n');
    thinkthen_call_result_free(result);return 0;
}
