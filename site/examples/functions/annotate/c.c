#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <string.h>
#include <thinkthen.h>

const char *form = "form.json";
const char *report = "Steps: click Log in. Nobody gets in.";
char request[512];
snprintf(request, sizeof request,
    "{\"schema\":\"thinkthen.request/1\","
    "\"call\":{\"function\":\"annotate\","
    "\"question\":{\"kind\":\"file\",\"path\":\"%s\"},"
    "\"input\":{\"kind\":\"text\",\"text\":\"%s\"}}}",
    form, report);

thinkthen_engine *tt = thinkthen_engine_new();
thinkthen_session *call = NULL;
int rc = thinkthen_session_new(
    tt, request, strlen(request), &call);
assert(rc == THINKTHEN_OK);
thinkthen_engine_free(tt);

bool steps = false;
char area[32] = "";
double impact = 0;
const uint32_t annotate_row =
    THINKTHEN_COMPLETE_SESSION_PACKET_ANNOTATE_ROW_V1;
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
    if (packet->kind == annotate_row) {
        thinkthen_complete_annotated_row_value_v1 triage =
            packet->data.annotate_row->value->value->value;
        for (size_t i = 0; i < triage.len; i++) {
            char name[16];
            snprintf(name, sizeof name, "%.*s",
                (int)triage.data[i].name.len,
                triage.data[i].name.data);
            const thinkthen_complete_annotated_field_v1
                *field = triage.data[i].value;
            if (strcmp(name, "steps") == 0) {
                steps = field->data.boolean == 1;
            } else if (strcmp(name, "area") == 0) {
                snprintf(area, sizeof area, "%.*s",
                    (int)field->data.string.len,
                    field->data.string.data);
            } else if (strcmp(name, "impact") == 0) {
                impact = field->data.number;
            }
        }
    }
    thinkthen_session_result_free(owner);
}
thinkthen_session_free(call);
assert(steps);
assert(strcmp(area, "login") == 0);
assert(impact == 1.98);
