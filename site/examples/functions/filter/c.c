#include <assert.h>
#include <stdio.h>
#include <string.h>
#include <thinkthen.h>

const char *question = "Is this a complaint?";
const char *reviews[] = {
    "Arrived a day early. Thank you!",
    "The zipper broke the first time I used it.",
    "Does this come in blue?",
    "The strap snapped on day two.",
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
        "%s{\"original\":{\"kind\":\"text\",\"text\":\"%s\"}}",
        i ? "," : "", reviews[i]);
}
snprintf(input_json + at, sizeof input_json - at, "]}");

thinkthen_engine *tt = thinkthen_engine_new();
assert(tt != NULL);
thinkthen_named_request_v1 request = {
    .function = {"filter", 6},
    .question_json = {question_json, strlen(question_json)},
    .input_json = {input_json, strlen(input_json)},
    .options_json = {"{}", 2},
};
thinkthen_call_result *result = NULL;
thinkthen_call_error *error = NULL;
int rc = thinkthen_run_call(tt, &request,
    NULL, NULL, NULL, &result, &error);
thinkthen_engine_free(tt);
if (rc != THINKTHEN_OK) {
    thinkthen_complete_utf8_v1 message =
        thinkthen_call_error_message(error);
    fwrite(message.data, 1, message.len, stderr);
    thinkthen_call_error_free(error);
}
assert(rc == THINKTHEN_OK);
thinkthen_complete_utf8_v1 json =
    thinkthen_call_result_json(result);
fwrite(json.data, 1, json.len, stdout);
putchar('\n');
thinkthen_call_result_free(result);
