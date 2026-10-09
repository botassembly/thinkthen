/* Owned session boundaries through the installed C header. */
#include <thinkthen.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#ifdef _WIN32
#include <windows.h>
#define yield() Sleep(0)
#else
#include <sched.h>
#define yield() sched_yield()
#endif
static int failed;
static void check(int holds, const char *what) {
    if (!holds) { fprintf(stderr, "FAIL %s\n", what); failed = 1; }
}
static const char feed[] = "{\"schema\":\"thinkthen.request/1\",\"call\":{\"function\":\"decide\",\"question\":{\"kind\":\"text\",\"text\":\"Does it pass?\"},\"input\":{\"kind\":\"feed\",\"name\":\"records\"}}}";
static const char direct[] = "{\"schema\":\"thinkthen.request/1\",\"call\":{\"function\":\"decide\",\"question\":{\"kind\":\"text\",\"text\":\"Does it pass?\"},\"input\":{\"kind\":\"text\",\"text\":\"Evidence.\"}}}";
static thinkthen_session *open_session(thinkthen_engine *engine, const char *json) {
    size_t len = strlen(json);
    char *buffer = malloc(len);
    if (!buffer) exit(1);
    memcpy(buffer, json, len);
    thinkthen_session *session = NULL;
    check(thinkthen_session_new(engine, buffer, len, &session) == THINKTHEN_OK, "owned constructor");
    memset(buffer, 0xff, len);
    free(buffer);
    return session;
}
static thinkthen_session_result *drain(thinkthen_session *session) {
    thinkthen_session_result *kept = NULL;
    for (;;) {
        uint32_t status = 99;
        thinkthen_session_result *out = (thinkthen_session_result *)(uintptr_t)7;
        check(thinkthen_session_try_read(session, &status, &out) == THINKTHEN_OK, "read succeeds");
        if (status == THINKTHEN_SESSION_RESULT_V1) {
            check(out != NULL, "result transfers an owner");
            if (kept) thinkthen_session_result_free(kept);
            kept = out;
        } else {
            check(out == NULL, "pending and end clear result output");
            if (status == THINKTHEN_SESSION_END_V1) return kept;
            check(status == THINKTHEN_SESSION_PENDING_V1, "pending status");
            yield();
        }
    }
}
int main(void) {
    thinkthen_engine *engine = thinkthen_engine_new_with("{\"cache\":false,\"max_retries\":0}");
    check(engine != NULL, "engine builds");
    if (!engine) return 1;
    if (getenv("SESSION_HELD")) {
        thinkthen_session *session = open_session(engine, direct);
        thinkthen_engine_free(engine);
        if (!session) return 1;
        (void)getchar();
        thinkthen_session_cancel(session);
        puts("cancelled"); fflush(stdout);
        thinkthen_session_free(session);
        puts("freed"); fflush(stdout);
        (void)getchar();
        return failed;
    }
    check(strcmp(thinkthen_session_error_message(), "no session failure yet") == 0, "fresh session error slot");
    check(thinkthen_engine_new_with("{\"bad\":1}") == NULL, "saved failed constructor");
    const char *old_error = thinkthen_error_message(NULL);
    const char *bad_requests[] = {"", "{}", "{\"schema\":\"thinkthen.request/1\",\"schema\":\"thinkthen.request/1\"}", "{\"unknown\":0}"};
    for (size_t i = 0; i < sizeof bad_requests / sizeof bad_requests[0]; ++i) {
        thinkthen_session *out = (thinkthen_session *)(uintptr_t)7;
        check(thinkthen_session_new(engine, bad_requests[i], strlen(bad_requests[i]), &out) == THINKTHEN_EUSAGE, "bad request refuses");
        check(out == (thinkthen_session *)(uintptr_t)7, "failed constructor preserves output");
    }
    const char invalid[] = {(char)0xff};
    thinkthen_session *sentinel = (thinkthen_session *)(uintptr_t)7;
    check(thinkthen_session_new(engine, invalid, sizeof invalid, &sentinel) == THINKTHEN_EUSAGE, "UTF-8 refusal");
    check(thinkthen_session_new(engine, feed, SIZE_MAX, &sentinel) == THINKTHEN_EUSAGE, "extent refusal");
    check(thinkthen_session_new(NULL, feed, strlen(feed), &sentinel) == THINKTHEN_EUSAGE, "NULL engine refusal");
    check(thinkthen_session_new(engine, feed, strlen(feed), NULL) == THINKTHEN_EUSAGE, "required constructor output");
    check(sentinel == (thinkthen_session *)(uintptr_t)7, "invalid bytes preserve output");
    check(old_error == thinkthen_error_message(NULL), "session errors preserve failed constructor slot");
    const char *session_error = thinkthen_session_error_message();
    thinkthen_session *session = open_session(engine, feed);
    check(session_error == thinkthen_session_error_message(), "success preserves immediate session error");
    if (!session) return 1;
    uint32_t status = 99;
    thinkthen_session_result *result = (thinkthen_session_result *)(uintptr_t)7;
    check(thinkthen_session_try_read(session, NULL, &result) == THINKTHEN_EUSAGE, "required read status");
    check(result == (thinkthen_session_result *)(uintptr_t)7, "refusal preserves read result");
    check(thinkthen_session_try_read(session, &status, NULL) == THINKTHEN_EUSAGE, "required read owner");
    check(status == 99, "refusal preserves read status");
    union { uint32_t status; thinkthen_session_result *result; } alias;
    alias.result = (thinkthen_session_result *)(uintptr_t)7;
    check(thinkthen_session_try_read(session, &alias.status, &alias.result) == THINKTHEN_EUSAGE, "aliased outputs refuse");
    check(alias.result == (thinkthen_session_result *)(uintptr_t)7, "alias output unchanged");
    const char *bad_descriptors[] = {"", "{}", "{\"item\":{},\"item\":{}}", "{\"unknown\":0}", "{\"item\":{},\"location\":null}"};
    for (size_t i = 0; i < sizeof bad_descriptors / sizeof bad_descriptors[0]; ++i) {
        status = 99;
        check(thinkthen_session_try_push(session, bad_descriptors[i], strlen(bad_descriptors[i]), &status) == THINKTHEN_EUSAGE, "bad descriptor refuses");
        check(status == 99, "bad descriptor preserves status");
    }
    check(thinkthen_session_try_push(session, invalid, sizeof invalid, &status) == THINKTHEN_EUSAGE, "descriptor UTF-8 refuses");
    check(thinkthen_session_try_push(session, "{}", 2, NULL) == THINKTHEN_EUSAGE, "push requires status");
    const char *bad_failures[] = {"", "{}", "{\"kind\":\"io\",\"kind\":\"utf8\"}", "{\"kind\":\"io\",\"unknown\":0}", "{\"kind\":\"io\",\"location\":null}", "{\"kind\":\"io\",\"location\":{\"file\":\"x\",\"file\":\"y\"}}"};
    for (size_t i = 0; i < sizeof bad_failures / sizeof bad_failures[0]; ++i)
        check(thinkthen_session_finish(session, bad_failures[i], strlen(bad_failures[i])) == THINKTHEN_EUSAGE, "bad reader failure refuses");
    check(thinkthen_session_finish(session, invalid, sizeof invalid) == THINKTHEN_EUSAGE, "reader failure UTF-8 refuses");
    check(thinkthen_session_finish(session, "{\"kind\":\"io\"}", 13) == THINKTHEN_OK, "valid owned reader failure");
    check(thinkthen_session_finish(session, NULL, 0) == THINKTHEN_EUSAGE, "changed finish refuses");
    check(thinkthen_session_try_push(session, "malformed", 9, &status) == THINKTHEN_OK && status == THINKTHEN_SESSION_CLOSED_V1, "closed intake avoids descriptor decoding");
    thinkthen_engine_free(engine);
    result = drain(session);
    check(result != NULL, "terminal transfers independent result");
    thinkthen_session_free(session);
    thinkthen_session_result_free(result);
    thinkthen_session_cancel(NULL); thinkthen_session_free(NULL); thinkthen_session_result_free(NULL);
    if (getenv("SESSION_CONTROLS")) return failed;
    engine = thinkthen_engine_new_with("{\"cache\":false,\"max_retries\":0}");
    session = open_session(engine, feed);
    if (!session) return 1;
    char descriptor[] = "{\"item\":{\"original\":{\"kind\":\"text\",\"text\":\"Evidence.\"}},\"location\":{\"file\":\"owned.txt\",\"first_line\":1,\"last_line\":1}}";
    check(thinkthen_session_try_push(session, descriptor, strlen(descriptor), &status) == THINKTHEN_OK && status == THINKTHEN_SESSION_ACCEPTED_V1, "accepted descriptor owns bytes");
    memset(descriptor, 0xff, sizeof descriptor);
    check(thinkthen_session_finish(session, NULL, 0) == THINKTHEN_OK, "EOF finishes input");
    thinkthen_engine_free(engine);
    result = drain(session);
    check(result != NULL, "owned execution settles");
    thinkthen_session_free(session);
    thinkthen_session_result_free(result);
    return failed;
}
