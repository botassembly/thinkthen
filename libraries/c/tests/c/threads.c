/*
 * R1-9: two threads over one engine, under AddressSanitizer. Each records
 * its own failure, saves the message pointer, waits for the other thread's
 * failure, and reads the saved pointer again. On a shared last-writer-wins
 * slot the second failure freed the first thread's message: ASan reports
 * the read, and the text becomes the other thread's.
 */
#include <pthread.h>
#include <stdatomic.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <thinkthen.h>

static atomic_int arrivals;

struct worker {
    thinkthen_engine *engine;
    const char *request;
    const char *own;
};

static void fail(const char *what) {
    fprintf(stderr, "FAIL %s\n", what);
    exit(1);
}

static void *run(void *argument) {
    struct worker *work = argument;
    if (thinkthen_call(work->engine, work->request) != NULL) {
        fail("a broken request answered");
    }
    const char *saved = thinkthen_error_message(work->engine);
    if (thinkthen_error_code(work->engine) != THINKTHEN_EUSAGE || strcmp(saved, work->own) != 0) {
        fail("the first read is not this thread's own failure");
    }
    atomic_fetch_add(&arrivals, 1);
    while (atomic_load(&arrivals) < 2) {
    }
    if (strcmp(saved, work->own) != 0) {
        fail("the saved message changed after the other thread failed");
    }
    if (strcmp(thinkthen_error_message(work->engine), work->own) != 0) {
        fail("the message read back is not this thread's own");
    }
    return NULL;
}

int main(void) {
    thinkthen_engine *engine = thinkthen_engine_new();
    if (engine == NULL) {
        fail("no engine came");
    }
    struct worker first = {
        engine, "{\"nope\": true}",
        "the request names no verb: decide, choose, score, tag, filter, rank, find, annotate, recognize, relate",
    };
    struct worker second = {
        engine, "{\"choose\": \"Pick.\", \"options\": [\"a\", \"b\"]}",
        "choose takes evidence as one string",
    };
    pthread_t one;
    pthread_t two;
    if (pthread_create(&one, NULL, run, &first) != 0 || pthread_create(&two, NULL, run, &second) != 0) {
        fail("a thread did not start");
    }
    if (pthread_join(one, NULL) != 0 || pthread_join(two, NULL) != 0) {
        fail("a thread did not join");
    }
    if (strcmp(thinkthen_error_message(engine), "no failure yet") != 0) {
        fail("the main thread sees a failure it never recorded");
    }
    thinkthen_engine_free(engine);
    return 0;
}
