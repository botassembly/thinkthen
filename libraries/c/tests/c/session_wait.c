/* Bounded intake and cancellation through the installed C session contract. */
#include "platform.h"
#include <thinkthen.h>
#include <stdint.h>
#include <string.h>
static const char request[] = "{\"schema\":\"thinkthen.request/1\",\"call\":{\"function\":\"decide\",\"question\":{\"kind\":\"text\",\"text\":\"Does it pass?\"},\"input\":{\"kind\":\"feed\",\"name\":\"records\"}}}";
static const char item[] = "{\"item\":{\"original\":{\"kind\":\"text\",\"text\":\"hold-session\"}}}";
static void check_at(int value, int line) { if (!value) { fprintf(stderr, "session contract failed at line %d\n", line); exit(1); } }
#define check(value) check_at((value), __LINE__)
struct waiter { thinkthen_session *session; uint32_t ready; };
static THREAD_RESULT wait_result(void *argument) {
    struct waiter *waiter = argument;
    check(thinkthen_session_wait(waiter->session, THINKTHEN_SESSION_WAIT_RESULT, 5000, &waiter->ready) == THINKTHEN_OK);
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
            char generated[128];
            const char *descriptor = item;
            if (!held) {
                snprintf(generated, sizeof generated, "{\"item\":{\"original\":{\"kind\":\"text\",\"text\":\"row %d\"}}}", i);
                descriptor = generated;
            }
            do {
                int code = blocking ? thinkthen_session_push(session, descriptor, strlen(descriptor), &status) : thinkthen_session_try_push(session, descriptor, strlen(descriptor), &status);
                check(code == THINKTHEN_OK);
                if (status == THINKTHEN_SESSION_FULL) {
                    uint32_t ready;
                    check(thinkthen_session_wait(session, THINKTHEN_SESSION_WAIT_INPUT, 5000, &ready) == THINKTHEN_OK);
                }
            } while (status == THINKTHEN_SESSION_FULL);
            check(status == THINKTHEN_SESSION_ACCEPTED);
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
        check(waiter.ready == THINKTHEN_SESSION_READY_CANCELLED);
        uint32_t ready;
        check(thinkthen_session_wait(session, THINKTHEN_SESSION_WAIT_SETTLEMENT, 0, &ready) == THINKTHEN_OK);
        check(ready == THINKTHEN_SESSION_READY_TIMEOUT);
        puts("cancelled"); fflush(stdout);
        (void)getchar();
    } else if (cancel) {
        thinkthen_session_cancel(session);
        uint32_t ready;
        check(thinkthen_session_wait(session, THINKTHEN_SESSION_WAIT_RESULT, 5000, &ready) == THINKTHEN_OK);
        check(ready == THINKTHEN_SESSION_READY_CANCELLED || ready == THINKTHEN_SESSION_READY_RESULT);
        check(thinkthen_session_try_push(session, item, strlen(item), &status) == THINKTHEN_OK && status == THINKTHEN_SESSION_CLOSED);
    }
    int rows = 0, terminals = 0;
    for (;;) {
        thinkthen_session_result *packet = NULL;
        check(thinkthen_session_next_packet(session, THINKTHEN_SESSION_WAIT_SETTLEMENT, &status, &packet) == THINKTHEN_OK);
        if (status == THINKTHEN_SESSION_END) break;
        check(status == THINKTHEN_SESSION_RESULT && packet != NULL);
        const thinkthen_complete_session_packet *view = NULL;
        check(thinkthen_session_result_view(packet, &view) == THINKTHEN_OK && view != NULL);
        if (view->kind == THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_ROW) {
            const thinkthen_complete_atomic_decide_value *row = view->data.decide_row->value;
            check(row->index.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE);
            check(row->index.value == (uint64_t)rows);
            rows++;
        }
        if (view->kind == THINKTHEN_COMPLETE_SESSION_PACKET_TERMINAL) {
            const thinkthen_complete_session_packet_terminal *terminal = view->data.terminal;
            terminals++;
            if (held || cancel) {
                check(terminal->failure.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE);
                check(terminal->failure.value->error->kind->kind == THINKTHEN_COMPLETE_FAILURE_KIND_CANCELLED);
            } else check(terminal->failure.presence == THINKTHEN_COMPLETE_PRESENCE_MISSING);
            if (!cancel) {
                check(terminal->facts.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE);
                check(terminal->facts.value->requests_sent == (held ? 1 : 24));
            }
        }
        thinkthen_session_result_free(packet);
    }
    check(terminals == 1);
    if (!cancel && !held) check(rows == 24);
    thinkthen_session_free(session);
    return 0;
}
