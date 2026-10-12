#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <string.h>
#include <thinkthen.h>

const char *question = "Is this a complaint?";
const char *reviews[] = {
    "Arrived a day early. Thank you!",
    "The zipper broke the first time I used it.",
    "Does this come in blue?",
    "The strap snapped on day two.",
};
char request[1024];
int at = snprintf(request, sizeof request,
    "{\"schema\":\"thinkthen.request/1\","
    "\"call\":{\"function\":\"filter\","
    "\"question\":{\"kind\":\"text\",\"text\":\"%s\"},"
    "\"input\":{\"kind\":\"records\",\"items\":[",
    question);
for (int i = 0; i < 4; i++) {
    at += snprintf(request + at, sizeof request - at,
        "%s{\"original\":"
        "{\"kind\":\"text\",\"text\":\"%s\"}}",
        i ? "," : "", reviews[i]);
}
snprintf(request + at, sizeof request - at, "]}}}");

thinkthen_engine *tt = thinkthen_engine_new();
thinkthen_session *call = NULL;
int rc = thinkthen_session_new(
    tt, request, strlen(request), &call);
assert(rc == THINKTHEN_OK);
thinkthen_engine_free(tt);

bool is_complaint[4] = {false};
const uint32_t filter_row =
    THINKTHEN_COMPLETE_SESSION_PACKET_FILTER_ROW_V1;
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
    if (packet->kind == filter_row) {
        const thinkthen_complete_atomic_boolean_v1 *row =
            packet->data.filter_row->value;
        is_complaint[row->index.value] = row->value == 1;
    }
    thinkthen_session_result_free(owner);
}
thinkthen_session_free(call);
assert(!is_complaint[0] && is_complaint[1]);
assert(!is_complaint[2] && is_complaint[3]);
