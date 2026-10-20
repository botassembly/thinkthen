#include <assert.h>
#include <stdio.h>
#include <string.h>
#include <thinkthen.h>

const char *form = "form.json";
const char *report = "Steps: click Log in. Nobody gets in.";
char question_json[512];
snprintf(question_json, sizeof question_json,
    "{\"kind\":\"file\",\"path\":\"%s\"}",
    form);
char input_json[512];
snprintf(input_json, sizeof input_json,
    "{\"kind\":\"text\",\"text\":\"%s\"}",
    report);

thinkthen_engine *tt = thinkthen_engine_new();
assert(tt != NULL);
thinkthen_named_request_v1 request = {
    .function = {"annotate", 8},
    .question_json = {question_json, strlen(question_json)},
    .input_json = {input_json, strlen(input_json)},
    .options_json = {"{}", 2},
};
thinkthen_call_result *report_call = NULL;
thinkthen_call_error *error = NULL;
int rc = thinkthen_run_call(tt, &request,
    NULL, NULL, NULL, &report_call, &error);
thinkthen_engine_free(tt);
if (rc != THINKTHEN_OK) {
    thinkthen_complete_utf8_v1 message =
        thinkthen_call_error_message(error);
    fwrite(message.data, 1, message.len, stderr);
    thinkthen_call_error_free(error);
}
assert(rc == THINKTHEN_OK);
thinkthen_complete_utf8_v1 json =
    thinkthen_call_result_json(report_call);
assert(json.data != NULL && json.len > 0);
fwrite(json.data, 1, json.len, stdout);
putchar('\n');
thinkthen_call_result_free(report_call);
