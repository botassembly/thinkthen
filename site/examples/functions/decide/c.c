#include <assert.h>
#include <stdio.h>
#include <string.h>
#include <thinkthen.h>

const char *question =
    "Does the customer ask for a refund?";
const char *broken =
    "Please refund my order. It arrived broken.";
char question_json[512];
snprintf(question_json, sizeof question_json,
    "{\"kind\":\"text\",\"text\":\"%s\"}",
    question);
char input_json[512];
snprintf(input_json, sizeof input_json,
    "{\"kind\":\"text\",\"text\":\"%s\"}",
    broken);

thinkthen_engine *tt = thinkthen_engine_new();
assert(tt != NULL);
thinkthen_named_request_v1 request = {
    .function = {"decide", 6},
    .question_json = {question_json, strlen(question_json)},
    .input_json = {input_json, strlen(input_json)},
    .options_json = {"{}", 2},
};
thinkthen_call_result *refund_call = NULL;
thinkthen_call_error *error = NULL;
int rc = thinkthen_run_call(tt, &request,
    NULL, NULL, NULL, &refund_call, &error);
thinkthen_engine_free(tt);
if (rc != THINKTHEN_OK) {
    thinkthen_complete_utf8_v1 message =
        thinkthen_call_error_message(error);
    fwrite(message.data, 1, message.len, stderr);
    thinkthen_call_error_free(error);
}
assert(rc == THINKTHEN_OK);
thinkthen_complete_utf8_v1 json =
    thinkthen_call_result_json(refund_call);
assert(json.data != NULL && json.len > 0);
/* Parse with your application's JSON library. */
thinkthen_call_result_free(refund_call);
