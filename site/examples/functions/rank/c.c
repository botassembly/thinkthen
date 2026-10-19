#include <assert.h>
#include <stdio.h>
#include <string.h>
#include <thinkthen.h>

const char *question = "Is this urgent?";
const char *inbox[] = {
    "Newsletter: our autumn catalog is here. "
    "No reply needed.",
    "Our checkout page is down and customers cannot pay",
    "Reminder: your invoice is due in 30 days",
    "Please send the signed quote by 5 pm today",
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
        i ? "," : "", inbox[i]);
}
snprintf(input_json + at, sizeof input_json - at, "]}");

thinkthen_engine *tt = thinkthen_engine_new();
assert(tt != NULL);
thinkthen_named_request_v1 request = {
    .function = {"rank", 4},
    .question_json = {question_json, strlen(question_json)},
    .input_json = {input_json, strlen(input_json)},
    .options_json = {"{}", 2},
};
thinkthen_call_result *priority_call = NULL;
thinkthen_call_error *error = NULL;
int rc = thinkthen_run_call(tt, &request,
    NULL, NULL, NULL, &priority_call, &error);
thinkthen_engine_free(tt);
if (rc != THINKTHEN_OK) {
    thinkthen_complete_utf8_v1 message =
        thinkthen_call_error_message(error);
    fwrite(message.data, 1, message.len, stderr);
    thinkthen_call_error_free(error);
}
assert(rc == THINKTHEN_OK);
thinkthen_complete_utf8_v1 json =
    thinkthen_call_result_json(priority_call);
assert(json.data != NULL && json.len > 0);
thinkthen_call_result_free(priority_call);
