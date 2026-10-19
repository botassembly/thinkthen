#include <assert.h>
#include <stdio.h>
#include <string.h>
#include <thinkthen.h>

const char *relations = "makes.json";
const char *names[][2] = {
    {"Apple", "company"},
    {"Amazon", "company"},
    {"iPhone", "product"},
    {"Kindle", "product"},
};
char question_json[512];
snprintf(question_json, sizeof question_json,
    "{\"kind\":\"file\",\"path\":\"%s\"}",
    relations);
char input_json[1024];
int at = snprintf(input_json, sizeof input_json,
    "{\"kind\":\"entities\",\"items\":[");
for (int i = 0; i < 4; i++) {
    at += snprintf(input_json + at, sizeof input_json - at,
        "%s{\"original\":{\"kind\":\"json\","
        "\"value\":{\"name\":\"%s\",\"kind\":\"%s\"}}}",
        i ? "," : "", names[i][0], names[i][1]);
}
snprintf(input_json + at, sizeof input_json - at, "]}");

thinkthen_engine *tt = thinkthen_engine_new();
assert(tt != NULL);
thinkthen_named_request_v1 request = {
    .function = {"relate", 6},
    .question_json = {question_json, strlen(question_json)},
    .input_json = {input_json, strlen(input_json)},
    .options_json = {"{}", 2},
};
thinkthen_call_result *makers_call = NULL;
thinkthen_call_error *error = NULL;
int rc = thinkthen_run_call(tt, &request,
    NULL, NULL, NULL, &makers_call, &error);
thinkthen_engine_free(tt);
if (rc != THINKTHEN_OK) {
    thinkthen_complete_utf8_v1 message =
        thinkthen_call_error_message(error);
    fwrite(message.data, 1, message.len, stderr);
    thinkthen_call_error_free(error);
}
assert(rc == THINKTHEN_OK);
thinkthen_complete_utf8_v1 json =
    thinkthen_call_result_json(makers_call);
assert(json.data != NULL && json.len > 0);
thinkthen_call_result_free(makers_call);
