#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <string.h>
#include <thinkthen.h>

const char *question =
    "Does the customer ask for a refund?";
const char *broken =
    "Please refund my order. It arrived broken.";
char request[512];
snprintf(request, sizeof request,
    "{\"schema\":\"thinkthen.request/1\","
    "\"call\":{\"function\":\"decide\","
    "\"question\":{\"kind\":\"text\",\"text\":\"%s\"},"
    "\"input\":{\"kind\":\"text\",\"text\":\"%s\"}}}",
    question, broken);

thinkthen_engine *tt = thinkthen_engine_new();
thinkthen_session *call = NULL;
int rc = thinkthen_session_new(
    tt, request, strlen(request), &call);
assert(rc == THINKTHEN_OK);
thinkthen_engine_free(tt);

bool is_refund = false;
const uint32_t decide_row =
    THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_ROW_V1;
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
    if (packet->kind == decide_row) {
        const thinkthen_complete_atomic_decide_value_v1
            *row = packet->data.decide_row->value;
        is_refund = row->value.presence ==
                THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 &&
            row->value.value->value->data.boolean == 1;
    }
    thinkthen_session_result_free(owner);
}
thinkthen_session_free(call);
assert(is_refund);
