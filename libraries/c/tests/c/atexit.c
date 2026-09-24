/*
 * R2-7: the door after thread-local teardown. glibc runs the main thread's
 * thread-local destructors before the atexit handlers, so this handler
 * calls into the door after that storage is gone: one failing call, the
 * thread's first on the engine, then the free. A door whose error path
 * touched a destroyed thread-local panicked there, the panic could not
 * unwind out of an `extern "C"` frame, and the host aborted with exit 134.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <thinkthen.h>

static thinkthen_engine *engine;

static void at_exit(void) {
    if (thinkthen_decide(engine, "Q?", "x", 1, NULL) != THINKTHEN_EUSAGE ||
        strcmp(thinkthen_error_message(engine), "a null out pointer") != 0) {
        fprintf(stderr, "FAIL the failing call after teardown\n");
        _Exit(1);
    }
    thinkthen_engine_free(engine);
}

int main(void) {
    engine = thinkthen_engine_new();
    if (engine == NULL || atexit(at_exit) != 0) {
        fprintf(stderr, "FAIL no engine or no handler\n");
        return 1;
    }
    return 0;
}
