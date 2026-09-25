/*
 * R2-16: a failure belongs to its engine. A failure on the second engine
 * never hides the first engine's, and a new engine, which may take the
 * freed engine's address, starts with no failure.
 */
#include <stdio.h>
#include <string.h>

#include <thinkthen.h>

static int failed;

static void check(int holds, const char *what) {
    if (!holds) {
        fprintf(stderr, "FAIL %s\n", what);
        failed = 1;
    }
}

int main(void) {
    thinkthen_engine *first = thinkthen_engine_new();
    thinkthen_engine *second = thinkthen_engine_new();
    if (first == NULL || second == NULL) {
        fprintf(stderr, "FAIL no engine came\n");
        return 1;
    }
    thinkthen_answer answer;
    check(thinkthen_decide(first, NULL, "x", 1, &answer) == THINKTHEN_EUSAGE, "the first engine fails");
    check(thinkthen_decide_opts(second, "Q?", "x", 1, 0, NULL, &answer) == THINKTHEN_EDEADLINE,
          "the second engine fails");
    check(thinkthen_error_code(first) == THINKTHEN_EUSAGE &&
              strcmp(thinkthen_error_message(first), "a null question") == 0,
          "the first engine keeps its own failure");
    check(thinkthen_error_code(second) == THINKTHEN_EDEADLINE, "the second engine reads its own failure");

    thinkthen_engine_free(first);
    thinkthen_engine *third = thinkthen_engine_new();
    check(third != NULL, "a third engine came");
    check(thinkthen_error_code(third) == THINKTHEN_OK &&
              strcmp(thinkthen_error_message(third), "no failure yet") == 0,
          "a new engine starts with no failure");
    thinkthen_engine_free(second);
    thinkthen_engine_free(third);
    return failed;
}
