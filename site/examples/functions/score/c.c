#include <assert.h>
#include <stdio.h>
#include <string.h>
#include <thinkthen.h>

const char *question = "How urgent is this?";
const char *levels =
    "[\"Routine.\",\"Soon.\",\"Immediate.\"]";
const char *outage =
    "Our checkout page is down and "
    "customers cannot pay.\\n";
char request[1024];
snprintf(request, sizeof request,
    "{\"schema\":\"thinkthen.request/1\","
    "\"call\":{\"function\":\"score\","
    "\"question\":{\"kind\":\"definition\",\"value\":"
    "{\"score\":\"%s\",\"levels\":%s}},"
    "\"input\":{\"kind\":\"text\",\"text\":\"%s\"}}}",
    question, levels, outage);

thinkthen_engine *tt = thinkthen_engine_new();
thinkthen_session *call = NULL;
int rc = thinkthen_session_new(
    tt, request, strlen(request), &call);
assert(rc == THINKTHEN_OK);
thinkthen_engine_free(tt);

double urgency = -1;
const uint32_t score_row =
    THINKTHEN_COMPLETE_SESSION_PACKET_SCORE_ROW_V1;
uint32_t status = THINKTHEN_SESSION_PENDING_V1;
while (status != THINKTHEN_SESSION_END_V1) {
    thinkthen_session_result *owner = NULL;
    rc = thinkthen_session_next_packet(call,
        THINKTHEN_SESSION_WAIT_SETTLEMENT_V1,
        &status, &owner);
    assert(rc == THINKTHEN_OK);

    if (status != THINKTHEN_SESSION_RESULT_V1) continue;
    const thinkthen_complete_session_packet_v1 *packet;
    thinkthen_session_result_view(owner, &packet);
    if (packet->kind == score_row) {
        urgency = packet->data.score_row->value->value;
    }
    thinkthen_session_result_free(owner);
}
thinkthen_session_free(call);
assert(urgency == 2.0);
