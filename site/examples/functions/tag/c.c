#include <assert.h>
#include <stdio.h>
#include <string.h>
#include <thinkthen.h>

const char *question = "Which labels fit this message?";
const char *labels = "[\"praise\",\"bug\",\"billing\"]";
const char *message =
    "Love the new dashboard, but export crashes the app,\\n"
    "and I was charged twice.\\n";
char request[1024];
snprintf(request, sizeof request,
    "{\"schema\":\"thinkthen.request/1\","
    "\"call\":{\"function\":\"tag\","
    "\"question\":{\"kind\":\"definition\",\"value\":"
    "{\"tag\":\"%s\",\"labels\":%s}},"
    "\"input\":{\"kind\":\"text\",\"text\":\"%s\"}}}",
    question, labels, message);

thinkthen_engine *tt = thinkthen_engine_new();
thinkthen_session *call = NULL;
int rc = thinkthen_session_new(
    tt, request, strlen(request), &call);
assert(rc == THINKTHEN_OK);
thinkthen_engine_free(tt);

char fitting_labels[128] = "";
const uint32_t tag_row =
    THINKTHEN_COMPLETE_SESSION_PACKET_TAG_ROW_V1;
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
    if (packet->kind == tag_row) {
        const thinkthen_complete_utf8_v1 *found =
            packet->data.tag_row->value->value.data;
        size_t count =
            packet->data.tag_row->value->value.len;
        for (size_t i = 0; i < count; i++) {
            size_t end = strlen(fitting_labels);
            snprintf(fitting_labels + end,
                sizeof fitting_labels - end, "%.*s ",
                (int)found[i].len, found[i].data);
        }
    }
    thinkthen_session_result_free(owner);
}
thinkthen_session_free(call);
assert(strcmp(fitting_labels, "praise bug billing ") == 0);
