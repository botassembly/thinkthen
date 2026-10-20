/* Run one named call and read its typed answer and probability. */
#include <thinkthen.h>
#include <stdio.h>
#include <string.h>
#ifdef _WIN32
#include <fcntl.h>
#include <io.h>
#endif
static thinkthen_complete_utf8 text(const char *s) {
    return (thinkthen_complete_utf8){s,strlen(s)};
}
static const thinkthen_complete_json *member(
    const thinkthen_complete_json *value, const char *name) {
    if (!value || value->kind != THINKTHEN_COMPLETE_JSON_OBJECT) return NULL;
    thinkthen_complete_json_object object = value->data.object;
    for (size_t i = 0; i < object.len; ++i) {
        thinkthen_complete_json_entry entry = object.data[i];
        if (entry.name.len == strlen(name) && !memcmp(entry.name.data, name, entry.name.len))
            return entry.value;
    }
    return NULL;
}
int main(void) {
#ifdef _WIN32
    if (_setmode(_fileno(stdout), _O_BINARY) == -1 ||
        _setmode(_fileno(stderr), _O_BINARY) == -1) {
        perror("set binary output mode");
        return 1;
    }
#endif
    thinkthen_engine *engine=thinkthen_engine_new_with("{\"cache\":false,\"max_retries\":0}", NULL);
    if(!engine)return 1;
    thinkthen_named_request request={text("decide"),text("{\"kind\":\"text\",\"text\":\"Does it pass?\"}"),
        text("{\"kind\":\"text\",\"text\":\"Evidence.\"}"),text("{}")};
    thinkthen_call_result *result=NULL;
    thinkthen_call_error *error=NULL;
    int status=thinkthen_run_call(engine,&request,NULL,NULL,NULL,&result,&error);
    thinkthen_engine_free(engine);
    if(status) {
        thinkthen_complete_utf8 message=thinkthen_call_error_message(error);
        fwrite(message.data,1,message.len,stderr);fputc('\n',stderr);
        thinkthen_call_error_free(error);return 1;
    }
    thinkthen_complete_utf8 json=thinkthen_call_result_json(result);
    thinkthen_value *owner = NULL;
    const thinkthen_complete_json *call = NULL;
    int failed = thinkthen_value_parse(json.data, json.len, &owner) ||
        thinkthen_value_view(owner, &call);
    thinkthen_call_result_free(result);
    const thinkthen_complete_json *settled = member(call, "answer");
    const thinkthen_complete_json *answer = member(settled, "value");
    const thinkthen_complete_json *probability = member(settled, "probability");
    if (failed || !answer || !probability ||
        (answer->kind != THINKTHEN_COMPLETE_JSON_BOOLEAN &&
         answer->kind != THINKTHEN_COMPLETE_JSON_NULL) ||
        (probability->kind != THINKTHEN_COMPLETE_JSON_NUMBER &&
         probability->kind != THINKTHEN_COMPLETE_JSON_NULL)) {
        fputs("Expected a decide answer and probability\n", stderr);
        thinkthen_value_free(owner);
        return 1;
    }
    printf("{\"answer\":%s,\"probability\":",
        answer->kind == THINKTHEN_COMPLETE_JSON_NULL ? "null" :
        answer->data.boolean ? "true" : "false");
    if (probability->kind == THINKTHEN_COMPLETE_JSON_NULL) fputs("null", stdout);
    else fwrite(probability->data.number.data, 1, probability->data.number.len, stdout);
    puts("}");
    thinkthen_value_free(owner);
    return 0;
}
