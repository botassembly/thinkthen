#include <assert.h>
#include <stdio.h>
#include <string.h>
#include <thinkthen.h>

const char *question =
    "Which line gives the refund deadline?";
const char *policy[] = {
    "Returns need the original receipt.",
    "Refunds are issued within 30 days of purchase.",
    "Shipping is free on orders over $50.",
    "Gift cards cannot be exchanged for cash.",
};
char question_json[512];
snprintf(question_json, sizeof question_json,
    "{\"kind\":\"text\",\"text\":\"%s\"}",
    question);
char input_json[1024];
int at = snprintf(input_json, sizeof input_json,
    "{\"kind\":\"records\",\"items\":[");
for (int i = 0; i < 4; i++) {
    at += snprintf(input_json + at, sizeof input_json - at,
        "%s{\"original\":{\"kind\":\"text\","
        "\"text\":\"%s\"}}",
        i ? "," : "", policy[i]);
}
snprintf(input_json + at, sizeof input_json - at, "]}");

thinkthen_engine *tt = thinkthen_engine_new();
assert(tt != NULL);
thinkthen_named_request_v1 request = {
    .function = {"find", 4},
    .question_json = {question_json, strlen(question_json)},
    .input_json = {input_json, strlen(input_json)},
    .options_json = {"{}", 2},
};
thinkthen_call_result *deadline_call = NULL;
thinkthen_call_error *error = NULL;
int rc = thinkthen_run_call(tt, &request,
    NULL, NULL, NULL, &deadline_call, &error);
thinkthen_engine_free(tt);
if (rc != THINKTHEN_OK) {
    thinkthen_complete_utf8_v1 message =
        thinkthen_call_error_message(error);
    fwrite(message.data, 1, message.len, stderr);
    thinkthen_call_error_free(error);
}
assert(rc == THINKTHEN_OK);
thinkthen_complete_utf8_v1 json =
    thinkthen_call_result_json(deadline_call);
assert(json.data != NULL && json.len > 0);
/* Parse with your application's JSON library. */
thinkthen_call_result_free(deadline_call);
