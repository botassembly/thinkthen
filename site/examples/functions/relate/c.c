#include <assert.h>
#include <stdio.h>
#include <string.h>
#include <thinkthen.h>

const char *relations = "makes.json";
const char *names[][2] = {
    {"Apple", "company"},
    {"Amazon", "company"},
    {"iPhone", "product"},
    {"Kindle", "product"},
};
char request[1024];
int at = snprintf(request, sizeof request,
    "{\"schema\":\"thinkthen.request/1\","
    "\"call\":{\"function\":\"relate\","
    "\"question\":{\"kind\":\"file\",\"path\":\"%s\"},"
    "\"input\":{\"kind\":\"entities\",\"items\":[",
    relations);
for (int i = 0; i < 4; i++) {
    at += snprintf(request + at, sizeof request - at,
        "%s{\"original\":{\"kind\":\"json\",\"value\":"
        "{\"name\":\"%s\",\"kind\":\"%s\"}}}",
        i ? "," : "", names[i][0], names[i][1]);
}
snprintf(request + at, sizeof request - at, "]}}}");

thinkthen_engine *tt = thinkthen_engine_new();
thinkthen_session *call = NULL;
int rc = thinkthen_session_new(
    tt, request, strlen(request), &call);
assert(rc == THINKTHEN_OK);
thinkthen_engine_free(tt);

char who_makes[256] = "";
const uint32_t relate_aggregate =
    THINKTHEN_COMPLETE_SESSION_PACKET_RELATE_AGGREGATE_V1;
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
    if (packet->kind == relate_aggregate) {
        const thinkthen_complete_relation_v1 *edges =
            packet->data.relate_aggregate->value;
        for (size_t i = 0; i < edges->value.len; i++) {
            const thinkthen_complete_related_entity_edge_v1
                *edge = edges->value.data[i];
            thinkthen_complete_utf8_v1 company =
                edge->source->data.related_entity->name;
            thinkthen_complete_utf8_v1 product =
                edge->target->data.related_entity->name;
            size_t end = strlen(who_makes);
            snprintf(who_makes + end,
                sizeof who_makes - end,
                "%.*s makes %.*s. ",
                (int)company.len, company.data,
                (int)product.len, product.data);
        }
    }
    thinkthen_session_result_free(owner);
}
thinkthen_session_free(call);
assert(strcmp(who_makes,
    "Apple makes iPhone. "
    "Amazon makes Kindle. ") == 0);
