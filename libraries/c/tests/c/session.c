/* Owned session boundaries through the installed C header. */
#include "platform.h"
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
static int same_text(thinkthen_complete_utf8_v1 text, const char *expected) {
    size_t len = strlen(expected);
    return text.len == len && (len == 0 || (text.data && memcmp(text.data, expected, len) == 0));
}
static void packet_view(const thinkthen_complete_session_packet_v1 *view) {
    check(view != NULL, "complete view exists");
    if (!view) return;
    if (view->kind == THINKTHEN_COMPLETE_SESSION_PACKET_TERMINAL_V1) {
        const thinkthen_complete_session_packet_terminal_v1 *terminal = view->data.terminal;
        check(terminal && same_text(terminal->kind, "terminal"), "typed terminal kind");
        if (!terminal) return;
        if (terminal->failure.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) {
            const thinkthen_complete_error_v1 *error = terminal->failure.value->error;
            check(error->kind->kind == THINKTHEN_COMPLETE_FAILURE_KIND_LOCAL_V1, "retained typed reader failure");
            check(error->message.data && error->message.len, "retained typed safe message");
        } else {
            check(terminal->failure.presence == THINKTHEN_COMPLETE_PRESENCE_MISSING_V1, "success has absent failure");
            check(terminal->facts.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1, "settlement has facts");
            check(terminal->facts.value->requests_sent == 1, "typed actual request count");
        }
    } else if (view->kind == THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_ROW_V1) {
        const thinkthen_complete_atomic_decide_value_v1 *row = view->data.decide_row->value;
        check(row->source.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1, "typed physical source present");
        check(same_text(row->source.value->file, "owned.txt"), "retained nested filename");
        check(row->source.value->first_line.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 && row->source.value->first_line.value == 1, "retained first line");
        check(row->source.value->last_line.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 && row->source.value->last_line.value == 1, "retained last line");
        check(row->meta->observations.len && row->meta->question_sources.len, "typed nested observations and origins");
        if (getenv("SESSION_PARTIAL")) {
            check(row->meta->usage.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1, "partial usage present");
            const thinkthen_complete_usage_v1 *usage = row->meta->usage.value;
            check(usage->input_tokens.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 && usage->input_tokens.value == 0, "reported zero input retained");
            check(usage->output_tokens.presence == THINKTHEN_COMPLETE_PRESENCE_MISSING_V1, "unknown output remains missing");
        }
    }
}
struct json_reader {
    thinkthen_session_result *result;
    const char *bytes;
    size_t len;
    int code;
    const thinkthen_complete_session_packet_v1 *view;
};
static THREAD_RESULT read_json(void *argument) {
    struct json_reader *reader = argument;
    reader->code = thinkthen_session_result_json(reader->result, &reader->bytes, &reader->len);
    if (reader->code == THINKTHEN_OK) reader->code = thinkthen_session_result_view(reader->result, &reader->view);
    return THREAD_DONE;
}
static void packet_json(thinkthen_session_result *result) {
    const char *sentinel = (const char *)(uintptr_t)7;
    size_t extent = 99;
    check(thinkthen_session_result_json(result, NULL, &extent) == THINKTHEN_EUSAGE && extent == 99, "live JSON validates byte output");
    check(thinkthen_session_result_json(result, &sentinel, NULL) == THINKTHEN_EUSAGE && sentinel == (const char *)(uintptr_t)7, "live JSON validates length output");
    union { const char *bytes; size_t len; } alias;
    alias.bytes = (const char *)(uintptr_t)7;
    check(thinkthen_session_result_json(result, &alias.bytes, &alias.len) == THINKTHEN_EUSAGE && alias.bytes == (const char *)(uintptr_t)7, "live JSON refuses output alias");
    check(thinkthen_session_result_view(result, NULL) == THINKTHEN_EUSAGE, "view requires output");
    const char *diagnostic = thinkthen_session_error_message();
    struct json_reader readers[2] = {{result, NULL, 0, 99, NULL}, {result, NULL, 0, 99, NULL}};
    THREAD_TYPE threads[2];
    for (size_t i = 0; i < 2; ++i)
        if (fixture_start(&threads[i], read_json, &readers[i]) != 0) exit(1);
    for (size_t i = 0; i < 2; ++i)
        if (fixture_join(threads[i]) != 0) exit(1);
    check(readers[0].code == THINKTHEN_OK && readers[1].code == THINKTHEN_OK, "concurrent JSON succeeds");
    check(readers[0].bytes == readers[1].bytes && readers[0].len == readers[1].len, "concurrent JSON has stable storage");
    check(readers[0].view == readers[1].view, "concurrent typed views have stable storage");
    packet_view(readers[0].view);
    if (!readers[0].bytes) exit(1);
    check(readers[0].bytes[readers[0].len] == 0 && strlen(readers[0].bytes) == readers[0].len, "JSON owns NUL beyond counted bytes");
    check(diagnostic == thinkthen_session_error_message(), "successful JSON preserves immediate diagnostic");
    puts(readers[0].bytes);
}
static void retained_json(thinkthen_session *session, thinkthen_session_result *result) {
    const char *before = NULL, *after = NULL;
    size_t len = 0, after_len = 0;
    check(thinkthen_session_result_json(result, &before, &len) == THINKTHEN_OK, "JSON before session free");
    const thinkthen_complete_session_packet_v1 *first = NULL, *second = NULL;
    check(thinkthen_session_result_view(result, &first) == THINKTHEN_OK, "view before session free");
    thinkthen_session_free(session);
    check(thinkthen_session_result_view(result, &second) == THINKTHEN_OK && first == second, "view survives session free");
    packet_view(second);
    check(thinkthen_session_result_json(result, &after, &after_len) == THINKTHEN_OK, "JSON after session free");
    check(before == after && len == after_len && before[len] == 0, "borrow survives engine and session free");
    thinkthen_session_result_free(result);
}
static thinkthen_session_result *drain(thinkthen_session *session) {
    thinkthen_session_result *kept = NULL;
    for (;;) {
        uint32_t status = 99;
        thinkthen_session_result *out = (thinkthen_session_result *)(uintptr_t)7;
        check(thinkthen_session_try_read(session, &status, &out) == THINKTHEN_OK, "read succeeds");
        if (status == THINKTHEN_SESSION_RESULT_V1) {
            check(out != NULL, "result transfers an owner");
            packet_json(out);
            if (kept) thinkthen_session_result_free(out);
            else kept = out;
        } else {
            check(out == NULL, "pending and end clear result output");
            if (status == THINKTHEN_SESSION_END_V1) return kept;
            check(status == THINKTHEN_SESSION_PENDING_V1, "pending status");
            yield();
        }
    }
}
static THREAD_RESULT separate_error(void *unused) {
    (void)unused;
    check(strcmp(thinkthen_session_error_message(), "no session failure yet") == 0, "new thread has independent diagnostic");
    check(thinkthen_session_new(NULL, NULL, 0, NULL) == THINKTHEN_EUSAGE, "other thread refusal");
    check(strcmp(thinkthen_session_error_message(), "invalid session arguments or input") == 0, "other thread fixed diagnostic");
    return THREAD_DONE;
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
    if (getenv("SESSION_ADMISSION")) {
        size_t len = strlen(getenv("SESSION_ADMISSION"));
        char *json = malloc(len + 1);
        if (!json) return 1;
        memcpy(json, getenv("SESSION_ADMISSION"), len + 1);
        thinkthen_session *out = (thinkthen_session *)(uintptr_t)7;
        check(thinkthen_session_new(engine, json, len, &out) == THINKTHEN_EUSAGE, "native admission kind");
        check(out == (thinkthen_session *)(uintptr_t)7, "native refusal preserves output");
        memset(json, 'x', len); free(json);
        const char *message = thinkthen_session_error_message();
        THREAD_TYPE thread;
        if (fixture_start(&thread, separate_error, NULL) != 0) return 1;
        check(fixture_join(thread) == 0, "join independent diagnostic thread");
        thinkthen_engine_free(engine);
        thinkthen_session_free(NULL);
        check(message == thinkthen_session_error_message(), "native diagnostic survives engine free and success");
        puts(message);
        check(thinkthen_session_new(NULL, NULL, 0, NULL) == THINKTHEN_EUSAGE, "next slot mutation");
        check(strcmp(thinkthen_session_error_message(), "invalid session arguments or input") == 0, "fixed failure replaces owned message");
        return failed;
    }
    check(strcmp(thinkthen_session_error_message(), "no session failure yet") == 0, "fresh session error slot");
    check(thinkthen_engine_new_with("{\"bad\":1}") == NULL, "saved failed constructor");
    const char *old_error = thinkthen_error_message(NULL);
    const char *bad_requests[] = {"", "{}", "{\"schema\":\"thinkthen.request/1\",\"schema\":\"thinkthen.request/1\"}", "{\"unknown\":0}"};
    for (size_t i = 0; i < sizeof bad_requests / sizeof bad_requests[0]; ++i) {
        thinkthen_session *out = (thinkthen_session *)(uintptr_t)7;
        check(thinkthen_session_new(engine, bad_requests[i], strlen(bad_requests[i]), &out) == THINKTHEN_EUSAGE, "bad request refuses");
        check(strcmp(thinkthen_session_error_message(), "invalid session arguments or input") == 0, "malformed controls keep fixed diagnostic");
        check(out == (thinkthen_session *)(uintptr_t)7, "failed constructor preserves output");
    }
    const thinkthen_complete_session_packet_v1 *view_sentinel = (const thinkthen_complete_session_packet_v1 *)(uintptr_t)7;
    check(thinkthen_session_result_view(NULL, &view_sentinel) == THINKTHEN_EUSAGE && view_sentinel == (const thinkthen_complete_session_packet_v1 *)(uintptr_t)7, "view refusal preserves sentinel");
    const char *bytes = (const char *)(uintptr_t)7;
    size_t byte_len = 99;
    check(thinkthen_session_result_json(NULL, &bytes, &byte_len) == THINKTHEN_EUSAGE, "NULL result refuses");
    check(bytes == (const char *)(uintptr_t)7 && byte_len == 99, "JSON refusal preserves both outputs");
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
    const char *untouched = (const char *)(uintptr_t)7;
    size_t untouched_len = 99;
    check(thinkthen_session_result_json(NULL, NULL, &untouched_len) == THINKTHEN_EUSAGE && untouched_len == 99, "JSON requires byte output");
    check(thinkthen_session_result_json(NULL, &untouched, NULL) == THINKTHEN_EUSAGE && untouched == (const char *)(uintptr_t)7, "JSON requires length output");
    union { const char *bytes; size_t len; } json_alias;
    json_alias.bytes = (const char *)(uintptr_t)7;
    check(thinkthen_session_result_json(NULL, &json_alias.bytes, &json_alias.len) == THINKTHEN_EUSAGE && json_alias.bytes == (const char *)(uintptr_t)7, "JSON rejects aliased outputs");
    result = drain(session);
    check(result != NULL, "terminal transfers independent result");
    retained_json(session, result);
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
    retained_json(session, result);
    return failed;
}
