/*
 * The door driver `tests/door/` feeds the shared cases through.
 *
 * Standard input holds requests. Each is a line `VERB COUNT`, then COUNT
 * fields, each a line holding its byte length and then its bytes and a
 * newline. The first field is the base address; the engine is built from
 * it through THINKTHEN_BASE_URL, as a host's would be, and serves each
 * request to that base until an `env` request or another base comes.
 *
 *   env NAME VALUE                    -> setenv, for the engines after it
 *   settings BASE JSON                -> build with the JSON settings object
 *   call BASE REQUEST                 -> thinkthen_call
 *   decide BASE QUESTION TEXT         -> thinkthen_decide
 *   expired BASE QUESTION TEXT        -> thinkthen_decide_opts, budget 0
 *   cancelled BASE QUESTION TEXT      -> thinkthen_decide_opts, fired token
 *   retryable BASE QUESTION TEXT      -> thinkthen_decide, then the code and
 *                                        thinkthen_error_retryable as "CODE RETRYABLE"
 *   nullcode BASE X                   -> thinkthen_error_code(NULL), after
 *                                        this base's engine builds
 *   many BASE QUESTION TEXT...        -> thinkthen_decide_many
 *   recognize BASE SPEC TEXT          -> thinkthen_recognize
 *   relate BASE SPEC RECORD...        -> thinkthen_relate
 *
 * Each request but `env` prints a line `CODE LENGTH`, then LENGTH bytes and
 * a newline: the answer when CODE is 0, where a judgment is `OUTCOME
 * PROBABILITY`, or else the engine's message. When no engine builds, the
 * reply is the null engine's code and message.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <thinkthen.h>

enum { MOST = 300 };

static char *fields[MOST];
static size_t lengths[MOST];

static void fail(const char *what) {
    fprintf(stderr, "driver: %s\n", what);
    exit(1);
}

static void read_field(size_t place) {
    size_t length = 0;
    if (scanf("%zu", &length) != 1 || getchar() != '\n') {
        fail("a field has no length line");
    }
    char *bytes = malloc(length + 1);
    if (bytes == NULL || fread(bytes, 1, length, stdin) != length || getchar() != '\n') {
        fail("a field is short");
    }
    bytes[length] = '\0';
    fields[place] = bytes;
    lengths[place] = length;
}

static void said(int code, const char *bytes, size_t length) {
    printf("%d %zu\n", code, length);
    fwrite(bytes, 1, length, stdout);
    printf("\n");
}

static void judged(const thinkthen_answer *answers, size_t count) {
    static char line[MOST * 32];
    size_t used = 0;
    for (size_t place = 0; place < count; place++) {
        used += (size_t)snprintf(line + used, sizeof line - used, "%s%d %.17g", place ? " " : "",
                                 answers[place].outcome, answers[place].probability);
    }
    said(THINKTHEN_OK, line, used);
}

static void answer(thinkthen_engine *tt, const char *verb, size_t count) {
    const char *const *rest = (const char *const *)&fields[2];
    char *out = NULL;
    size_t out_len = 0;
    int rc = THINKTHEN_EUSAGE;
    if (strcmp(verb, "call") == 0) {
        out = thinkthen_call(tt, fields[1]);
        rc = out == NULL ? thinkthen_error_code(tt) : THINKTHEN_OK;
    } else if (strcmp(verb, "recognize") == 0) {
        rc = thinkthen_recognize(tt, fields[1], fields[2], lengths[2], &out, &out_len);
    } else if (strcmp(verb, "relate") == 0) {
        rc = thinkthen_relate(tt, fields[1], rest, &lengths[2], count - 2, &out, &out_len);
    } else if (strcmp(verb, "decide") == 0 || strcmp(verb, "expired") == 0 ||
               strcmp(verb, "cancelled") == 0) {
        thinkthen_answer one;
        thinkthen_cancel_token *token = thinkthen_cancel_token_new();
        int64_t budget = strcmp(verb, "expired") == 0 ? 0 : THINKTHEN_NO_DEADLINE;
        if (strcmp(verb, "cancelled") == 0) {
            thinkthen_cancel(token);
        }
        rc = thinkthen_decide_opts(tt, fields[1], fields[2], lengths[2], budget, token, &one);
        thinkthen_cancel_token_free(token);
        if (rc == THINKTHEN_OK) {
            judged(&one, 1);
            return;
        }
    } else if (strcmp(verb, "retryable") == 0) {
        thinkthen_answer one;
        char line[32];
        rc = thinkthen_decide(tt, fields[1], fields[2], lengths[2], &one);
        int used = snprintf(line, sizeof line, "%d %d", rc, thinkthen_error_retryable(tt));
        said(THINKTHEN_OK, line, (size_t)used);
        return;
    } else if (strcmp(verb, "nullcode") == 0) {
        char line[16];
        int used = snprintf(line, sizeof line, "%d", thinkthen_error_code(NULL));
        said(THINKTHEN_OK, line, (size_t)used);
        return;
    } else if (strcmp(verb, "many") == 0) {
        static thinkthen_answer many[MOST];
        rc = thinkthen_decide_many(tt, fields[1], rest, &lengths[2], count - 2, many);
        if (rc == THINKTHEN_OK) {
            judged(many, count - 2);
            return;
        }
    }
    if (rc != THINKTHEN_OK) {
        const char *message = thinkthen_error_message(tt);
        said(rc, message, strlen(message));
        return;
    }
    said(THINKTHEN_OK, out, out_len == 0 ? strlen(out) : out_len);
    thinkthen_free_string(out);
}

int main(void) {
    char verb[16];
    size_t count = 0;
    thinkthen_engine *tt = NULL;
    char *base = NULL;
    while (scanf("%15s %zu", verb, &count) == 2) {
        if (getchar() != '\n' || count < 2 || count > MOST) {
            fail("a request line is malformed");
        }
        for (size_t place = 0; place < count; place++) {
            read_field(place);
        }
        if (strcmp(verb, "env") == 0) {
            setenv(fields[0], fields[1], 1);
            free(fields[0]);
            free(fields[1]);
            thinkthen_engine_free(tt);
            tt = NULL;
            continue;
        }
        /* One engine serves every request to one base, so its counters
         * carry from call to call as a host's engine's do. */
        if (strcmp(verb, "settings") == 0) {
            thinkthen_engine_free(tt);
            free(base);
            base = strdup(fields[0]);
            setenv("THINKTHEN_BASE_URL", base, 1);
            tt = thinkthen_engine_new_with(fields[1]);
            if (tt == NULL) {
                const char *message = thinkthen_error_message(NULL);
                said(thinkthen_error_code(NULL), message, strlen(message));
            } else {
                said(THINKTHEN_OK, "", 0);
            }
            fflush(stdout);
            for (size_t place = 0; place < count; place++) free(fields[place]);
            continue;
        }
        if (tt == NULL || strcmp(base, fields[0]) != 0) {
            thinkthen_engine_free(tt);
            free(base);
            base = strdup(fields[0]);
            setenv("THINKTHEN_BASE_URL", base, 1);
            tt = thinkthen_engine_new();
        }
        if (tt == NULL) {
            const char *message = thinkthen_error_message(NULL);
            said(thinkthen_error_code(NULL), message, strlen(message));
        } else {
            answer(tt, verb, count);
        }
        fflush(stdout);
        for (size_t place = 0; place < count; place++) {
            free(fields[place]);
        }
    }
    thinkthen_engine_free(tt);
    free(base);
    return 0;
}
