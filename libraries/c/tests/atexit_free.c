/*
 * Free the engine from an atexit handler: the review's teardown repro.
 *
 * glibc runs thread-local destructors before the atexit handlers when the
 * process exits, so this handler calls into the door after thread-local
 * storage is gone. A door whose error path touched a thread-local would
 * panic there — a panic that cannot unwind out of an `extern "C"` frame,
 * so the host aborts with signal 6 (exit 134). The error path holds no
 * thread-local state now, so this program must exit 0.
 *
 * Run by check.sh; anything but 0 fails the gate.
 */
#include <stdlib.h>

#include "thinkthen.h"

static thinkthen_engine *engine;

static void free_at_exit(void) {
    thinkthen_engine_free(engine);
}

int main(void) {
    engine = thinkthen_engine_new();
    if (engine == NULL) {
        return 2;
    }
    if (atexit(free_at_exit) != 0) {
        return 3;
    }
    /* Returning from main exits, which runs the atexit handler after
     * thread-local teardown. */
    return 0;
}
