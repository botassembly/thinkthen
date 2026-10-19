#include <assert.h>
#include <stdio.h>
#include <string.h>
#include <thinkthen.h>

const char *question = "Which team owns this?";
const char *teams =
    "{\"billing\":\"Invoices, fees, and refunds.\","
    "\"shipping\":\"Parcels and delivery.\","
    "\"account\":\"Logins and passwords.\"}";
const char *parcel = "My parcel went to the wrong address.";
char question_json[512];
snprintf(question_json, sizeof question_json,
    "{\"kind\":\"definition\","
    "\"value\":{\"choose\":\"%s\",\"options\":%s}}",
    question, teams);
char input_json[512];
snprintf(input_json, sizeof input_json,
    "{\"kind\":\"text\",\"text\":\"%s\"}",
    parcel);

thinkthen_engine *tt = thinkthen_engine_new();
assert(tt != NULL);
thinkthen_named_request_v1 request = {
    .function = {"choose", 6},
    .question_json = {question_json, strlen(question_json)},
    .input_json = {input_json, strlen(input_json)},
    .options_json = {"{}", 2},
};
thinkthen_call_result *team_call = NULL;
thinkthen_call_error *error = NULL;
int rc = thinkthen_run_call(tt, &request,
    NULL, NULL, NULL, &team_call, &error);
thinkthen_engine_free(tt);
if (rc != THINKTHEN_OK) {
    thinkthen_complete_utf8_v1 message =
        thinkthen_call_error_message(error);
    fwrite(message.data, 1, message.len, stderr);
    thinkthen_call_error_free(error);
}
assert(rc == THINKTHEN_OK);
thinkthen_complete_utf8_v1 json =
    thinkthen_call_result_json(team_call);
assert(json.data != NULL && json.len > 0);
fwrite(json.data, 1, json.len, stdout);
putchar('\n');
thinkthen_call_result_free(team_call);
