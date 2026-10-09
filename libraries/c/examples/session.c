/* Read typed packets from the generated header; link a prebuilt C package. */
#include <thinkthen.h>
#include <stdio.h>
#include <string.h>
#include <threads.h>

int main(void) {
    const char request[] = "{\"schema\":\"thinkthen.request/1\",\"call\":{\"function\":\"decide\",\"question\":{\"kind\":\"text\",\"text\":\"Does it pass?\"},\"input\":{\"kind\":\"text\",\"text\":\"Evidence.\"}}}";
    thinkthen_engine *engine = thinkthen_engine_new_with("{\"cache\":false,\"max_retries\":0}");
    if (!engine) return 1;
    thinkthen_session *session = NULL;
    int code = thinkthen_session_new(engine, request, strlen(request), &session);
    thinkthen_engine_free(engine);
    if (code != THINKTHEN_OK) {
        fprintf(stderr, "%s\n", thinkthen_session_error_message());
        return 1;
    }
    int failed = 0;
    for (;;) {
        uint32_t status;
        thinkthen_session_result *owner = NULL;
        if (thinkthen_session_try_read(session, &status, &owner) != THINKTHEN_OK) {
            failed = 1;
            break;
        }
        if (status == THINKTHEN_SESSION_END_V1) break;
        if (status == THINKTHEN_SESSION_PENDING_V1) {
            thrd_yield();
            continue;
        }
        const thinkthen_complete_session_packet_v1 *packet = NULL;
        if (thinkthen_session_result_view(owner, &packet) != THINKTHEN_OK) {
            thinkthen_session_result_free(owner);
            failed = 1;
            break;
        }
        if (packet->kind == THINKTHEN_COMPLETE_SESSION_PACKET_TERMINAL_V1) {
            const thinkthen_complete_session_packet_terminal_v1 *terminal = packet->data.terminal;
            if (terminal->failure.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) {
                const thinkthen_complete_error_v1 *error = terminal->failure.value->error;
                fprintf(stderr, "failure %u: %.*s\n", error->kind->kind, (int)error->message.len, error->message.data);
                failed = 1;
            }
            if (terminal->facts.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) {
                const thinkthen_complete_facts_v1 *facts = terminal->facts.value;
                printf("records=%llu requests=%llu\n", (unsigned long long)facts->records, (unsigned long long)facts->requests_sent);
            }
        }
        /* All nested views borrow owner. Release after the last field read. */
        thinkthen_session_result_free(owner);
    }
    thinkthen_session_free(session);
    return failed;
}
