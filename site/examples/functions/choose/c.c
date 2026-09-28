#include <assert.h>
#include <stdio.h>
#include <string.h>
#include <thinkthen.h>

thinkthen_engine *tt = thinkthen_engine_new();

const char *choose =
    "{\"choose\": \"Which team owns this?\", "
    "\"options\": {"
    "\"billing\": \"Invoices, fees, and refunds.\", "
    "\"shipping\": \"Parcels and delivery.\", "
    "\"account\": \"Logins and passwords.\"}, "
    "\"threshold\": 0.9, \"evidence\": \"%s\"}";
const char *texts[] = {
    "Please refund the extra fee on my invoice.",
    "My parcel went to the wrong address.",
    "I cannot reset my password.",
    "My parcel never came, and now "
        "I cannot log in to track it.",
};
const char *teams[] = {
    "\"billing\"",
    "\"shipping\"",
    "\"account\"",
    "null",
};
char request[320];
for (int i = 0; i < 4; i++) {
    snprintf(request, sizeof request, choose, texts[i]);
    char *team_call = thinkthen_call(tt, request);
    assert(team_call);
    assert(strncmp(team_call, "{\"value\":", 9) == 0);
    const char *team = team_call + 9;
    size_t team_len = strlen(teams[i]);
    assert(strncmp(team, teams[i], team_len) == 0);
    assert(strncmp(
        team + team_len, ",\"facts\":{", 10
    ) == 0);
    thinkthen_free_string(team_call);
}
