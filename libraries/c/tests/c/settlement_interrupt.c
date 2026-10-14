/* An exclusive drain can interrupt its wait without freeing a borrowed owner. */
#define main session_fixture_main
#include "session.c"
#undef main
struct drain_waiter { thinkthen_session *session; uint32_t interest, ready; int code; };
static THREAD_RESULT wait_drain(void *argument) {
    struct drain_waiter *waiter = argument;
    waiter->code = thinkthen_session_wait(waiter->session, waiter->interest, UINT32_MAX, &waiter->ready);
    return THREAD_DONE;
}
static void drain_after_interrupt(thinkthen_session *session) {
    uint32_t state;
    do {
        thinkthen_session_result *packet = NULL;
        check(thinkthen_session_try_read(session, &state, &packet) == THINKTHEN_OK, "settlement transfer succeeds");
        if (packet) { packet_json(packet); thinkthen_session_result_free(packet); }
    } while (state == THINKTHEN_SESSION_RESULT_V1);
    check(state == THINKTHEN_SESSION_END_V1, "terminal exhausts the interrupted drain");
}
int main(void) {
    thinkthen_engine *engine = thinkthen_engine_new_with("{\"cache\":false,\"max_retries\":0}");
    check(engine != NULL, "engine builds");
    if (!engine) return 1;
    thinkthen_session *session = open_session(engine, direct);
    thinkthen_engine_free(engine);
    if (!session) return 1;
    (void)getchar();
    for (;;) {
        uint32_t state; thinkthen_session_result *packet = NULL;
        check(thinkthen_session_try_read(session, &state, &packet) == THINKTHEN_OK, "consume queued prefix");
        thinkthen_session_result_free(packet);
        if (state != THINKTHEN_SESSION_RESULT_V1) break;
    }
    thinkthen_session_cancel(session);
    uint32_t ready = 99;
    check(thinkthen_session_wait(session, THINKTHEN_SESSION_WAIT_SETTLEMENT_ARM_V1, 0, &ready) == THINKTHEN_OK && ready == THINKTHEN_SESSION_READY_TIMEOUT_V1, "arm ignores prior cancellation and never waits");
    // Cancel before native entry as well as during a borrowed wait.
    thinkthen_session_cancel(session);
    ready = 99;
    check(thinkthen_session_wait(session, THINKTHEN_SESSION_WAIT_SETTLEMENT_ARM_V1, 1, &ready) == THINKTHEN_EUSAGE && ready == 99, "nonzero arm preserves output and interrupt latch");
    const char *refusal = thinkthen_session_error_message();
    check(thinkthen_session_wait(session, THINKTHEN_SESSION_WAIT_INTERRUPTIBLE_SETTLEMENT_V1, UINT32_MAX, &ready) == THINKTHEN_OK && ready == THINKTHEN_SESSION_READY_CANCELLED_V1, "cancellation before entry persists");
    check(refusal == thinkthen_session_error_message(), "successful interrupt preserves calling-thread failure");
    check(thinkthen_session_wait(session, THINKTHEN_SESSION_WAIT_SETTLEMENT_ARM_V1, 0, &ready) == THINKTHEN_OK && ready == THINKTHEN_SESSION_READY_TIMEOUT_V1, "fresh drain rearms");
    struct drain_waiter interrupted = {session, THINKTHEN_SESSION_WAIT_INTERRUPTIBLE_SETTLEMENT_V1, 99, 99};
    struct drain_waiter legacy = {session, THINKTHEN_SESSION_WAIT_SETTLEMENT_V1, 99, 99};
    THREAD_TYPE interrupt_thread, legacy_thread;
    check(fixture_start(&interrupt_thread, wait_drain, &interrupted) == 0, "start interruptible drain");
    check(fixture_start(&legacy_thread, wait_drain, &legacy) == 0, "start legacy drain");
    puts("waiting"); fflush(stdout);
    (void)getchar();
    thinkthen_session_cancel(session);
    check(fixture_join(interrupt_thread) == 0, "join interrupted reader before owner release");
    check(interrupted.code == THINKTHEN_OK && interrupted.ready == THINKTHEN_SESSION_READY_CANCELLED_V1, "cancel interrupts indefinite settlement wait");
    puts("interrupted"); fflush(stdout);
    check(fixture_join(legacy_thread) == 0, "legacy drain returns only after provider release");
    check(legacy.code == THINKTHEN_OK && legacy.ready == THINKTHEN_SESSION_READY_RESULT_V1, "legacy settlement ignores every cancel");
    drain_after_interrupt(session);
    thinkthen_session_free(session);
    puts("freed"); fflush(stdout);
    return failed;
}
