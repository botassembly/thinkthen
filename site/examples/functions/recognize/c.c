#include <assert.h>
#include <stdio.h>
#include <string.h>
#include <thinkthen.h>

const char *kinds = "names.json";
const char *text =
    "Maria Chen joined Northwind Freight, "
    "a company in Chicago.";
char request[512];
snprintf(request, sizeof request,
    "{\"schema\":\"thinkthen.request/1\","
    "\"call\":{\"function\":\"recognize\","
    "\"question\":{\"kind\":\"file\",\"path\":\"%s\"},"
    "\"input\":{\"kind\":\"text\",\"text\":\"%s\"}}}",
    kinds, text);

thinkthen_engine *tt = thinkthen_engine_new();
thinkthen_session *call = NULL;
int rc = thinkthen_session_new(
    tt, request, strlen(request), &call);
assert(rc == THINKTHEN_OK);
thinkthen_engine_free(tt);

char names[256] = "";
const uint32_t recognize_aggregate =
  THINKTHEN_COMPLETE_SESSION_PACKET_RECOGNIZE_AGGREGATE_V1;
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
    if (packet->kind == recognize_aggregate &&
        packet->data.recognize_aggregate->value.len > 0) {
        const thinkthen_complete_recognition_v1 *row =
            packet->data.recognize_aggregate->value.data[0];
        const thinkthen_complete_entity_v1 *const *found =
            row->value->data.fields_entities->entities.data;
        size_t count =
            row->value->data.fields_entities->entities.len;
        for (size_t i = 0; i < count; i++) {
            const thinkthen_complete_entity_v1 *one =
                found[i];
            size_t end = strlen(names);
            snprintf(names + end, sizeof names - end,
                "%.*s (%.*s). ",
                (int)one->text.len, one->text.data,
                (int)one->kind.len, one->kind.data);
        }
    }
    thinkthen_session_result_free(owner);
}
thinkthen_session_free(call);
assert(strcmp(names,
    "Maria Chen (person). "
    "Northwind Freight (organization). "
    "Chicago (place). ") == 0);
