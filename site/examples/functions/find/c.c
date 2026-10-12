#include <assert.h>
#include <stdio.h>
#include <string.h>
#include <thinkthen.h>

const char *question =
    "Which line gives the refund deadline?";
const char *policy[] = {
    "Returns need the original receipt.",
    "Refunds are issued within 30 days of purchase.",
    "Shipping is free on orders over $50.",
    "Gift cards cannot be exchanged for cash.",
};
char request[1024];
int at = snprintf(request, sizeof request,
    "{\"schema\":\"thinkthen.request/1\","
    "\"call\":{\"function\":\"find\","
    "\"question\":{\"kind\":\"text\",\"text\":\"%s\"},"
    "\"input\":{\"kind\":\"records\",\"items\":[",
    question);
for (int i = 0; i < 4; i++) {
    at += snprintf(request + at, sizeof request - at,
        "%s{\"original\":"
        "{\"kind\":\"text\",\"text\":\"%s\"}}",
        i ? "," : "", policy[i]);
}
snprintf(request + at, sizeof request - at, "]}}}");

thinkthen_engine *tt = thinkthen_engine_new();
thinkthen_session *call = NULL;
int rc = thinkthen_session_new(
    tt, request, strlen(request), &call);
assert(rc == THINKTHEN_OK);
thinkthen_engine_free(tt);

char refund_deadline[128] = "";
const uint32_t find_aggregate =
    THINKTHEN_COMPLETE_SESSION_PACKET_FIND_AGGREGATE_V1;
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
    if (packet->kind == find_aggregate) {
        const thinkthen_complete_find_v1 *found =
            packet->data.find_aggregate->value;
        if (found->value.presence ==
            THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) {
            thinkthen_complete_utf8_v1 line =
                found->value.value->data.string;
            snprintf(refund_deadline,
                sizeof refund_deadline, "%.*s",
                (int)line.len, line.data);
        }
    }
    thinkthen_session_result_free(owner);
}
thinkthen_session_free(call);
assert(strcmp(refund_deadline, policy[1]) == 0);
