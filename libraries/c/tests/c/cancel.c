/*
 * Ticket 0166: a token fired while the backend holds the reply ends the
 * call with THINKTHEN_ECANCELLED and untouched out values, for the typed
 * scalar, the JSON scalar, and the typed bulk door. No request starts after
 * the fire, and a fresh token answers from the reply the cancelled call
 * paid for.
 *
 * `tests/door/` holds each reply on the backend's held arm. For each of
 * tokens A, B, and C it writes one line once the request arrives; the
 * reader thread fires that token and prints `fired A`, `fired B`, or
 * `fired C`, and only then does the harness let the held replies go.
 */
#include <pthread.h>
#include <stdio.h>
#include <string.h>

#include <thinkthen.h>

static int failed;
static thinkthen_cancel_token *tokens[3];

static void check(int holds, const char *what) {
    if (!holds) {
        fprintf(stderr, "FAIL %s\n", what);
        failed = 1;
    }
}

static int cancelled(const thinkthen_engine *tt) {
    return thinkthen_error_code(tt) == THINKTHEN_ECANCELLED &&
           strcmp(thinkthen_error_message(tt), "the call was cancelled") == 0;
}

static int untouched(const thinkthen_answer *answer) {
    return answer->outcome == 7 && answer->probability == 7.0;
}

/* Fire each token when the harness says its request is held. */
static void *reader(void *unused) {
    (void)unused;
    char line[16];
    for (int i = 0; i < 3 && fgets(line, sizeof line, stdin) != NULL; i++) {
        thinkthen_cancel(tokens[i]);
        printf("fired %c\n", 'A' + i);
        fflush(stdout);
    }
    return NULL;
}

int main(void) {
    const int64_t none = THINKTHEN_NO_DEADLINE;
    const char *q = "Is this a complaint?";
    thinkthen_engine *tt = thinkthen_engine_new();
    if (tt == NULL) {
        fprintf(stderr, "FAIL no engine came\n");
        return 1;
    }
    for (int i = 0; i < 3; i++) {
        tokens[i] = thinkthen_cancel_token_new();
    }
    pthread_t thread;
    if (pthread_create(&thread, NULL, reader, NULL) != 0) {
        fprintf(stderr, "FAIL no reader thread\n");
        return 1;
    }

    thinkthen_answer one = {7, 7.0};
    check(thinkthen_decide_opts(tt, q, "one", 3, none, tokens[0], &one) == THINKTHEN_ECANCELLED &&
              cancelled(tt) && untouched(&one),
          "a fire during a held decide cancels it");

    check(thinkthen_call_opts(tt, "{\"decide\":\"Is this a complaint?\",\"evidence\":\"two\"}", none,
                              tokens[1]) == NULL &&
              cancelled(tt),
          "a fire during a held JSON decide cancels it");

    const char *texts[5] = {"three", "four", "five", "six", "seven"};
    size_t lengths[5] = {5, 4, 4, 3, 5};
    thinkthen_answer many[5];
    for (int i = 0; i < 5; i++) {
        many[i] = (thinkthen_answer){7, 7.0};
    }
    check(thinkthen_decide_many_opts(tt, q, texts, lengths, 5, none, tokens[2], many) ==
                  THINKTHEN_ECANCELLED &&
              cancelled(tt),
          "a fire during a held bulk cancels it");
    for (int i = 0; i < 5; i++) {
        check(untouched(&many[i]), "a cancelled bulk writes no row");
    }
    if (pthread_join(thread, NULL) != 0) {
        fprintf(stderr, "FAIL the reader did not join\n");
        return 1;
    }

    /* A fresh token answers from the reply the cancelled decide paid for. */
    thinkthen_cancel_token *fresh = thinkthen_cancel_token_new();
    check(thinkthen_decide_opts(tt, q, "one", 3, none, fresh, &one) == THINKTHEN_OK &&
              one.outcome == THINKTHEN_YES && one.probability == 0.9,
          "a fresh token answers from the cache");
    thinkthen_answer spent = {7, 7.0};
    check(thinkthen_decide_opts(tt, q, "one", 3, none, tokens[0], &spent) == THINKTHEN_ECANCELLED &&
              untouched(&spent),
          "the fired token still refuses");

    char *usage = thinkthen_call(tt, "{\"usage\":true}");
    check(usage != NULL &&
              strcmp(usage, "{\"requests_sent\":6,\"input_tokens\":0,\"output_tokens\":0,"
                            "\"cache_answers\":1}") == 0,
          "six requests were sent and one answer came from the cache");
    if (failed && usage != NULL) {
        fprintf(stderr, "usage %s\n", usage);
    }
    thinkthen_free_string(usage);

    thinkthen_cancel_token_free(fresh);
    for (int i = 0; i < 3; i++) {
        thinkthen_cancel_token_free(tokens[i]);
    }
    thinkthen_engine_free(tt);
    return failed;
}
