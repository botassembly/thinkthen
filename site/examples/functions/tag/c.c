#include <assert.h>
#include <stdio.h>
#include <string.h>
#include <thinkthen.h>

const char *question = "Which labels fit this message?";
const char *labels = "[\"praise\",\"bug\",\"billing\"]";
const char *message =
    "Love the new dashboard, but export crashes the app,\\n"
    "and I was charged twice.\\n";
char question_json[512];
snprintf(question_json, sizeof question_json,
    "{\"kind\":\"definition\","
    "\"value\":{\"tag\":\"%s\",\"labels\":%s}}",
    question, labels);
char input_json[512];
snprintf(input_json, sizeof input_json,
    "{\"kind\":\"text\",\"text\":\"%s\"}",
    message);

thinkthen_engine *tt = thinkthen_engine_new();
assert(tt != NULL);
thinkthen_named_request_v1 request = {
    .function = {"tag", 3},
    .question_json = {question_json, strlen(question_json)},
    .input_json = {input_json, strlen(input_json)},
    .options_json = {"{}", 2},
};
thinkthen_call_result *labels_call = NULL;
thinkthen_call_error *error = NULL;
int rc = thinkthen_run_call(tt, &request,
    NULL, NULL, NULL, &labels_call, &error);
thinkthen_engine_free(tt);
if (rc != THINKTHEN_OK) {
    thinkthen_complete_utf8_v1 message =
        thinkthen_call_error_message(error);
    fwrite(message.data, 1, message.len, stderr);
    thinkthen_call_error_free(error);
}
assert(rc == THINKTHEN_OK);
thinkthen_complete_utf8_v1 json =
    thinkthen_call_result_json(labels_call);
assert(json.data != NULL && json.len > 0);
fwrite(json.data, 1, json.len, stdout);
putchar('\n');
thinkthen_call_result_free(labels_call);
