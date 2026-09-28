/*
 * R2-7: the door after thread-local teardown. glibc runs the main thread's
 * thread-local destructors before the atexit handlers, so this handler
 * calls into the door after that storage is gone. The main thread fails
 * once on the first engine, so its storage exists and is torn down. The
 * handler then fails on the second engine, the thread's first failure
 * there, and frees both. A door whose error path touched a destroyed
 * thread-local panicked there, the panic could not unwind out of an
 * `extern "C"` frame, and the host aborted with exit 134.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <thinkthen.h>

static thinkthen_engine *first;
static thinkthen_engine *second;

static int refused(thinkthen_engine *engine) {
    return thinkthen_decide(engine, "Q?", "x", 1, NULL) == THINKTHEN_EUSAGE &&
           strcmp(thinkthen_error_message(engine), "a null out pointer") == 0;
}

static void at_exit(void) {
    if (!refused(second)) {
        fprintf(stderr, "FAIL the failing call after teardown\n");
        _Exit(1);
    }
    thinkthen_engine_free(first);
    thinkthen_engine_free(second);
}

int main(void) {
    first = thinkthen_engine_new();
    second = thinkthen_engine_new();
    if (first == NULL || second == NULL || atexit(at_exit) != 0) {
        fprintf(stderr, "FAIL no engine or no handler\n");
        return 1;
    }
    if (!refused(first)) {
        fprintf(stderr, "FAIL the failing call before teardown\n");
        return 1;
    }
    return 0;
}
