/* Canonical counted Request preview through the installed header. */
#include <thinkthen.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static void check(int holds, const char *what) {
    if (!holds) { fprintf(stderr, "FAIL %s\n", what); exit(1); }
}
static const char p1[] = "{\"schema\":\"thinkthen.request/1\",\"call\":{\"function\":\"decide\",\"question\":{\"kind\":\"text\",\"text\":\"asks for a refund\"},\"input\":{\"kind\":\"text\",\"text\":\"Refund me please.\"},\"options\":{\"max_requests_total\":0}}}";

static void refused(thinkthen_engine *engine, const char *input, int expected) {
    char *sentinel = (char *)(uintptr_t)7;
    size_t extent = 99;
    check(thinkthen_request_plan_json(engine, input, strlen(input), &sentinel, &extent) == expected,
          "canonical refusal kind");
    check(sentinel == (char *)(uintptr_t)7 && extent == 99, "refusal preserves outputs");
    const char *message = thinkthen_session_error_message();
    check(message && !strstr(message, "PRIVATE"), "refusal keeps safe diagnostic");
    check(strcmp(message, expected == THINKTHEN_ELOCAL ? "session local operation failed" :
                 "invalid session arguments or input") == 0, "refusal pins safe diagnostic");
}
static thinkthen_complete_utf8_v1 text(const char *value) {
    return (thinkthen_complete_utf8_v1){value, strlen(value)};
}
static char *typed_preview(thinkthen_engine *engine) {
    thinkthen_request_text_v1 question_text={.text=text("asks for a refund")};
    thinkthen_request_question_selector_text_v1 question_value={.m_text=&question_text};
    thinkthen_request_question_selector_v1 question={.kind=THINKTHEN_REQUEST_QUESTION_SELECTOR_ARM_1_V1,
        .data.arm_1=&question_value};
    thinkthen_request_text_v1 evidence={.text=text("Refund me please.")};
    thinkthen_request_input_text_v1 input_value={.m_text=&evidence};
    thinkthen_request_input_v1 input={.kind=THINKTHEN_REQUEST_INPUT_ARM_1_V1,.data.arm_1=&input_value};
    thinkthen_request_unsigned_v1 limit={.value=0};
    thinkthen_request_call_options_v1 options={.m_max_requests_total=&limit};
    thinkthen_request_arguments_v1 arguments={.m_question=&question,.m_input=&input,.m_options=&options};
    char *out=(char *)(uintptr_t)7;
    size_t length=99;
    check(thinkthen_request_plan_typed_json(engine,text("decide"),&arguments,text("ada"),&out,&length)==THINKTHEN_OK,
          "typed preview uses shared native admission");
    char *json=NULL;
    size_t json_length=0;
    check(thinkthen_request_plan_json(engine,p1,sizeof(p1)-1,&json,&json_length)==THINKTHEN_OK,
          "canonical preview comparison");
    check(length==json_length && !memcmp(out,json,length),"typed and JSON previews share the plan");
    thinkthen_free_string(json);
    question_text.text=text("");
    char *refused=(char *)(uintptr_t)7;
    size_t extent=99;
    check(thinkthen_request_plan_typed_json(engine,text("decide"),&arguments,text("ada"),&refused,&extent)==THINKTHEN_EUSAGE,
          "invalid typed question refuses before planning");
    check(refused==(char *)(uintptr_t)7 && extent==99,"typed refusal preserves outputs");
    check(!strstr(thinkthen_session_error_message(),"PRIVATE"),"typed refusal diagnostic stays safe");
    question_text.text=text("asks for a refund");
    check(thinkthen_request_plan_typed_json(NULL,text("decide"),&arguments,text("ada"),&refused,&extent)==THINKTHEN_EUSAGE,
          "typed preview requires an engine");
    check(thinkthen_request_plan_typed_json(engine,text("decide"),NULL,text("ada"),&refused,&extent)==THINKTHEN_EUSAGE,
          "typed preview requires arguments");
    check(thinkthen_request_plan_typed_json(engine,text("decide"),&arguments,text("ada"),NULL,&extent)==THINKTHEN_EUSAGE,
          "typed preview requires output");
    check(thinkthen_request_plan_typed_json(engine,text("decide"),&arguments,text("ada"),&refused,NULL)==THINKTHEN_EUSAGE,
          "typed preview requires output length");
    union {char *out; size_t len;} alias;
    alias.out=(char *)(uintptr_t)7;
    check(thinkthen_request_plan_typed_json(engine,text("decide"),&arguments,text("ada"),&alias.out,&alias.len)==THINKTHEN_EUSAGE,
          "typed preview refuses aliased outputs");
    char oversized[8193]; memset(oversized,'x',sizeof(oversized));
    evidence.text=(thinkthen_complete_utf8_v1){oversized,sizeof(oversized)};
    check(thinkthen_request_plan_typed_json(engine,text("decide"),&arguments,text("cobol"),&refused,&extent)==THINKTHEN_COBOL_OVERFLOW,
          "typed COBOL preview preserves scalar capacity");
    check(refused==(char *)(uintptr_t)7 && extent==99,"typed NULL and overflow refusals preserve outputs");
    memset(oversized,'?',sizeof(oversized));
    check(out && strlen(out)==length && out[length]==0,"typed preview owns copied bytes");
    return out;
}
static thinkthen_engine *typed_settings(void) {
    thinkthen_engine *engine=thinkthen_engine_new_typed(NULL);
    check(engine!=NULL,"NULL typed settings capture environment");
    thinkthen_engine_free(engine);
    thinkthen_request_engine_settings_v1 settings={0};
    engine=thinkthen_engine_new_typed(&settings);
    check(engine!=NULL,"empty typed settings capture environment");
    thinkthen_engine_free(engine);
    uint32_t ready=99;
    check(thinkthen_session_wait(NULL,THINKTHEN_SESSION_WAIT_RESULT_V1,0,&ready)==THINKTHEN_EUSAGE,
          "immediate session failure before construction");
    const char *session_error=thinkthen_session_error_message();
    thinkthen_request_unsigned_v1 timeout={.value=0};
    settings.m_timeout=&timeout;
    check(thinkthen_engine_new_typed(&settings)==NULL,"typed invalid timeout refuses");
    check(thinkthen_error_code(NULL)==THINKTHEN_EUSAGE,"typed settings use constructor slot");
    check(!strcmp(thinkthen_error_message(NULL),"a timeout is a time above zero"),"typed settings preserve admission diagnostic");
    check(thinkthen_session_error_message()==session_error,"constructor preserves independent session slot");
    check(thinkthen_error_retryable(NULL)==0,"invalid typed settings are not retryable");
    char model[]="model-never-retained-from-host";
    thinkthen_request_text_v1 bad={.text={NULL,1}};
    settings.m_timeout=NULL; settings.m_model=&bad;
    check(thinkthen_engine_new_typed(&settings)==NULL,"typed NULL counted text refuses");
    check(thinkthen_error_code(NULL)==THINKTHEN_EUSAGE,"invalid carrier has usage kind");
    bad.text=text(model); settings.m_model=&bad;
    engine=thinkthen_engine_new_typed(&settings);
    check(engine!=NULL,"typed settings accept model");
    memset(model,'?',sizeof(model)-1);
    check(thinkthen_error_code(NULL)==THINKTHEN_EUSAGE,"successful constructor clears failed build");
    check(thinkthen_session_error_message()==session_error,"successful constructor preserves session slot");
    check(!strstr(thinkthen_error_message(NULL),"model"),"typed settings diagnostic is safe");
    thinkthen_engine_free(engine);
    settings.m_model=NULL;
    return thinkthen_engine_new_typed(&settings);
}
int main(void) {
    thinkthen_engine *engine = typed_settings();
    check(engine != NULL, "engine construction");
    char *typed = typed_preview(engine);
    char *out = (char *)(uintptr_t)7;
    size_t len = 99;
    check(thinkthen_request_plan_json(NULL, p1, sizeof(p1)-1, &out, &len) == THINKTHEN_EUSAGE,
          "null engine");
    check(out == (char *)(uintptr_t)7 && len == 99, "null engine keeps outputs");
    check(thinkthen_request_plan_json(engine, p1, sizeof(p1)-1, NULL, &len) == THINKTHEN_EUSAGE && len == 99,
          "null output");
    check(thinkthen_request_plan_json(engine, p1, sizeof(p1)-1, &out, NULL) == THINKTHEN_EUSAGE && out == (char *)(uintptr_t)7,
          "null length");
    union { char *out; size_t len; } alias;
    alias.out = (char *)(uintptr_t)7;
    check(thinkthen_request_plan_json(engine, p1, sizeof(p1)-1, &alias.out, &alias.len) == THINKTHEN_EUSAGE
          && alias.out == (char *)(uintptr_t)7, "aliased outputs");
    check(thinkthen_request_plan_json(engine, p1, SIZE_MAX, &out, &len) == THINKTHEN_EUSAGE,
          "unrepresentable input extent");
    check(thinkthen_request_plan_json(engine, NULL, 0, &out, &len) == THINKTHEN_EUSAGE,
          "empty request");
    refused(engine, "{\"schema\":\"thinkthen.request/1\",\"schema\":\"thinkthen.request/1\",\"PRIVATE\":true}", THINKTHEN_EUSAGE);
    refused(engine, "{\"schema\":\"thinkthen.request/1\",\"call\":{\"function\":\"decide\",\"question\":{\"kind\":\"text\",\"text\":\"PRIVATE\"},\"input\":{\"kind\":\"text\",\"text\":\"PRIVATE\"},\"options\":{\"batch\":null}}}", THINKTHEN_EUSAGE);
    refused(engine, "{\"schema\":\"thinkthen.request/1\",\"call\":{\"function\":\"filter\",\"question\":{\"kind\":\"text\",\"text\":\"PRIVATE\"},\"input\":{\"kind\":\"source\",\"source\":{\"paths\":[\"PRIVATE-MISSING\"]}}}}", THINKTHEN_ELOCAL);
    /* Counted input deliberately omits the trailing NUL. */
    char *request = malloc(sizeof(p1)-1);
    check(request != NULL, "request storage");
    memcpy(request, p1, sizeof(p1)-1);
    check(thinkthen_request_plan_json(engine, request, sizeof(p1)-1, &out, &len) == THINKTHEN_OK,
          "canonical preview");
    memset(request, 0xff, sizeof(p1)-1);
    free(request);
    thinkthen_engine_free(engine);
    check(out && strlen(out) == len && out[len] == 0, "independent owned preview");
    check(!strcmp(typed, out), "typed preview survives engine and carrier destruction");
    thinkthen_free_string(typed);
    puts(out);
    thinkthen_free_string(out);
    return 0;
}
