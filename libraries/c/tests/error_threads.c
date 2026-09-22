/*
 * The per-thread error slot through the C door, under AddressSanitizer:
 * two threads over one engine, each recording its own failure and reading
 * its own message back, with a saved pointer read after the other thread
 * has failed.
 *
 * This is the review's exact repro. On the old last-writer-wins slot the
 * second thread's failure freed the first thread's message, so the saved
 * pointer read below was a use-after-free (ASan reports it) and the
 * message read back was the other thread's (the strstr checks fail).
 * After the fix each thread's message lives in its own slot.
 *
 * Compiled and run by check.sh inside its sanitizer block.
 */
#define _GNU_SOURCE
#include <pthread.h>
#include <stdatomic.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "thinkthen.h"

static atomic_int arrivals;

struct worker {
    thinkthen_engine *engine;
    const char *request;
    const char *own;
    const char *other;
    const char *saved;
};

static void fail(const char *what) {
    fprintf(stderr, "FAIL %s\n", what);
    exit(1);
}

static void *run(void *argument) {
    struct worker *work = argument;

    /* This thread's failure: its message must be its own. */
    char *reply = thinkthen_call_opts(work->engine, work->request, THINKTHEN_NO_DEADLINE, NULL);
    if (reply != NULL) {
        fail("a broken request answered");
    }
    if (thinkthen_error_code(work->engine) != THINKTHEN_EUSAGE) {
        fail("the failure is not the usage kind");
    }
    work->saved = thinkthen_error_message(work->engine);
    if (strstr(work->saved, work->own) == NULL) {
        fail("the first read is not this thread's own message");
    }

    /* Let the other thread record its failure, then read the saved
     * pointer again: on the shared slot it was freed and replaced. */
    atomic_fetch_add(&arrivals, 1);
    while (atomic_load(&arrivals) < 2) {
    }

    if (strstr(work->saved, work->own) == NULL) {
        fail("the saved message no longer carries this thread's text");
    }
    if (strstr(work->saved, work->other) != NULL) {
        fail("the saved message became the other thread's");
    }

    const char *now = thinkthen_error_message(work->engine);
    if (strstr(now, work->own) == NULL) {
        fail("the message read back is not this thread's own");
    }
    if (strstr(now, work->other) != NULL) {
        fail("the message read back is the other thread's");
    }
    return NULL;
}

int main(void) {
    thinkthen_engine *engine = thinkthen_engine_new();
    if (engine == NULL) {
        fail("no engine came");
    }

    struct worker first = {
        .engine = engine,
        .request = "{\"nope\": true}",
        .own = "no verb the door knows",
        .other = "no evidence string",
        .saved = NULL,
    };
    struct worker second = {
        .engine = engine,
        .request = "{\"choose\": \"Pick.\", \"options\": [\"a\", \"b\"]}",
        .own = "no evidence string",
        .other = "no verb the door knows",
        .saved = NULL,
    };

    pthread_t one;
    pthread_t two;
    if (pthread_create(&one, NULL, run, &first) != 0) {
        fail("the first thread");
    }
    if (pthread_create(&two, NULL, run, &second) != 0) {
        fail("the second thread");
    }
    if (pthread_join(one, NULL) != 0 || pthread_join(two, NULL) != 0) {
        fail("a thread did not join");
    }

    /* Only the recording thread's slot sees its failure: this thread
     * recorded none, so it reads none. */
    if (strcmp(thinkthen_error_message(engine), "no failure yet") != 0) {
        fail("the main thread sees a failure it never recorded");
    }

    thinkthen_engine_free(engine);
    printf("ok       two threads read their own messages, none crossed\n");
    return 0;
}
