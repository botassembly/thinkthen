/* Bounded intake and cancellation through the installed C session contract. */
#include "platform.h"
#include <thinkthen.h>
#include <stdint.h>
#include <string.h>
static const char request[] = "{\"schema\":\"thinkthen.request/1\",\"call\":{\"function\":\"decide\",\"question\":{\"kind\":\"text\",\"text\":\"Does it pass?\"},\"input\":{\"kind\":\"feed\",\"name\":\"records\"}}}";
static const char item[] = "{\"item\":{\"original\":{\"kind\":\"text\",\"text\":\"hold-session\"}}}";
static void check(int value) { if (!value) { fputs("session contract failed\n", stderr); exit(1); } }
struct waiter { thinkthen_session *session; uint32_t ready; };
static THREAD_RESULT wait_result(void *argument) {
    struct waiter *waiter = argument;
    check(thinkthen_session_wait(waiter->session, THINKTHEN_SESSION_WAIT_RESULT_V1, 5000, &waiter->ready) == THINKTHEN_OK);
    return THREAD_DONE;
}
int main(int argc, char **argv) {
    check(argc == 2);
    thinkthen_engine *engine = thinkthen_engine_new_with("{\"cache\":false,\"max_retries\":0,\"batch\":1,\"throttle\":1}");
    check(engine != NULL);
    thinkthen_session *session = NULL;
    check(thinkthen_session_new(engine, request, strlen(request), &session) == THINKTHEN_OK);
    thinkthen_engine_free(engine);
    uint32_t status = 99;
    int cancel = strcmp(argv[1], "cancel") == 0;
    int held = strcmp(argv[1], "held") == 0;
    int blocking = strcmp(argv[1], "push") == 0;
    if (!cancel) {
        for (int i = 0; i < (held ? 1 : 24); ++i) {
            do {
                int code = blocking ? thinkthen_session_push(session, item, strlen(item), &status) : thinkthen_session_try_push(session, item, strlen(item), &status);
                check(code == THINKTHEN_OK);
                if (status == THINKTHEN_SESSION_FULL_V1) {
                    uint32_t ready;
                    check(thinkthen_session_wait(session, THINKTHEN_SESSION_WAIT_INPUT_V1, 5000, &ready) == THINKTHEN_OK);
                }
            } while (status == THINKTHEN_SESSION_FULL_V1);
            check(status == THINKTHEN_SESSION_ACCEPTED_V1);
        }
        check(thinkthen_session_finish(session, NULL, 0) == THINKTHEN_OK);
    }
    if (held) {
        puts("submitted"); fflush(stdout);
        (void)getchar();
        struct waiter waiter = {session, 99};
        THREAD_TYPE thread;
        check(fixture_start(&thread, wait_result, &waiter) == 0);
        thinkthen_session_cancel(session);
        check(fixture_join(thread) == 0);
        check(waiter.ready == THINKTHEN_SESSION_READY_CANCELLED_V1);
        uint32_t ready;
        check(thinkthen_session_wait(session, THINKTHEN_SESSION_WAIT_SETTLEMENT_V1, 0, &ready) == THINKTHEN_OK);
        check(ready == THINKTHEN_SESSION_READY_TIMEOUT_V1);
        puts("cancelled"); fflush(stdout);
        (void)getchar();
    } else if (cancel) {
        thinkthen_session_cancel(session);
        uint32_t ready;
        check(thinkthen_session_wait(session, THINKTHEN_SESSION_WAIT_RESULT_V1, 5000, &ready) == THINKTHEN_OK);
        check(ready == THINKTHEN_SESSION_READY_CANCELLED_V1 || ready == THINKTHEN_SESSION_READY_RESULT_V1);
        check(thinkthen_session_try_push(session, item, strlen(item), &status) == THINKTHEN_OK && status == THINKTHEN_SESSION_CLOSED_V1);
    }
    int rows = 0, terminals = 0;
    for (;;) {
        thinkthen_session_result *packet = NULL;
        check(thinkthen_session_next_packet(session, THINKTHEN_SESSION_WAIT_SETTLEMENT_V1, &status, &packet) == THINKTHEN_OK);
        if (status == THINKTHEN_SESSION_END_V1) break;
        check(status == THINKTHEN_SESSION_RESULT_V1 && packet != NULL);
        const char *json = NULL; size_t len = 0;
        check(thinkthen_session_result_json(packet, &json, &len) == THINKTHEN_OK);
        if (strstr(json, "\"kind\":\"row\"")) rows++;
        if (strstr(json, "\"kind\":\"terminal\"")) {
            terminals++;
            if (held || cancel) check(strstr(json, "\"kind\":\"cancelled\"") != NULL);
            if (held) check(strstr(json, "\"requests_sent\":1") != NULL);
            if (!held && !cancel) check(strstr(json, "\"requests_sent\":24") != NULL);
        }
        thinkthen_session_result_free(packet);
    }
    check(terminals == 1);
    if (!cancel && !held) check(rows == 24);
    thinkthen_session_free(session);
    return 0;
}
