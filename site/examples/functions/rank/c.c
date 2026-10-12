#include <assert.h>
#include <stdio.h>
#include <string.h>
#include <thinkthen.h>

const char *question = "Is this urgent?";
const char *inbox[] = {
    "Newsletter: our autumn catalog is here. "
    "No reply needed.",
    "Our checkout page is down and customers cannot pay",
    "Reminder: your invoice is due in 30 days",
    "Please send the signed quote by 5 pm today",
};
char request[1024];
int at = snprintf(request, sizeof request,
    "{\"schema\":\"thinkthen.request/1\","
    "\"call\":{\"function\":\"rank\","
    "\"question\":{\"kind\":\"text\",\"text\":\"%s\"},"
    "\"input\":{\"kind\":\"records\",\"items\":[",
    question);
for (int i = 0; i < 4; i++) {
    at += snprintf(request + at, sizeof request - at,
        "%s{\"original\":"
        "{\"kind\":\"text\",\"text\":\"%s\"}}",
        i ? "," : "", inbox[i]);
}
snprintf(request + at, sizeof request - at, "]}}}");

thinkthen_engine *tt = thinkthen_engine_new();
thinkthen_session *call = NULL;
int rc = thinkthen_session_new(
    tt, request, strlen(request), &call);
assert(rc == THINKTHEN_OK);
thinkthen_engine_free(tt);

int by_urgency[4] = {-1, -1, -1, -1};
const uint32_t rank_aggregate =
    THINKTHEN_COMPLETE_SESSION_PACKET_RANK_AGGREGATE_V1;
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
    if (packet->kind == rank_aggregate) {
        const thinkthen_complete_atomic_non_zero_usize_v1
            *const *places =
            packet->data.rank_aggregate->value.data;
        size_t count =
            packet->data.rank_aggregate->value.len;
        for (size_t i = 0; i < count; i++) {
            by_urgency[places[i]->value - 1] =
                (int)places[i]->index.value;
        }
    }
    thinkthen_session_result_free(owner);
}
thinkthen_session_free(call);
assert(by_urgency[0] == 1 && by_urgency[1] == 3);
assert(by_urgency[2] == 2 && by_urgency[3] == 0);
