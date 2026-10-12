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
char request[1024];
snprintf(request, sizeof request,
    "{\"schema\":\"thinkthen.request/1\","
    "\"call\":{\"function\":\"choose\","
    "\"question\":{\"kind\":\"definition\",\"value\":"
    "{\"choose\":\"%s\",\"options\":%s}},"
    "\"input\":{\"kind\":\"text\",\"text\":\"%s\"}}}",
    question, teams, parcel);

thinkthen_engine *tt = thinkthen_engine_new();
thinkthen_session *call = NULL;
int rc = thinkthen_session_new(
    tt, request, strlen(request), &call);
assert(rc == THINKTHEN_OK);
thinkthen_engine_free(tt);

char team[64] = "";
const uint32_t choose_row =
    THINKTHEN_COMPLETE_SESSION_PACKET_CHOOSE_ROW_V1;
uint32_t status = THINKTHEN_SESSION_PENDING_V1;
while (status != THINKTHEN_SESSION_END_V1) {
    thinkthen_session_result *owner = NULL;
    rc = thinkthen_session_try_read(call, &status, &owner);
    assert(rc == THINKTHEN_OK);
    if (status == THINKTHEN_SESSION_PENDING_V1) {
        uint32_t ready;
        thinkthen_session_wait(call,
            THINKTHEN_SESSION_WAIT_RESULT_V1, 50, &ready);
    }
    if (status != THINKTHEN_SESSION_RESULT_V1) continue;
    const thinkthen_complete_session_packet_v1 *packet;
    thinkthen_session_result_view(owner, &packet);
    if (packet->kind == choose_row) {
        thinkthen_complete_utf8_v1 name =
            packet->data.choose_row->value->value.value;
        snprintf(team, sizeof team, "%.*s",
            (int)name.len, name.data);
    }
    thinkthen_session_result_free(owner);
}
thinkthen_session_free(call);
assert(strcmp(team, "shipping") == 0);
