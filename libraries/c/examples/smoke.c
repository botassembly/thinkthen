/*
 * The replay smoke (ticket 0335): one decide through thinkthen_engine_new,
 * which reads the environment, with the question and text that
 * sdlc/scripts/smoke names.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <thinkthen.h>

int main(void) {
    const char *question = getenv("THINKTHEN_SMOKE_QUESTION");
    const char *text = getenv("THINKTHEN_SMOKE_TEXT");
    thinkthen_engine *tt = thinkthen_engine_new();
    thinkthen_answer a;
    if (question == NULL || text == NULL || tt == NULL ||
        thinkthen_decide(tt, question, text, strlen(text), &a) != THINKTHEN_OK) {
        fprintf(stderr, "smoke: the decide call failed\n");
        return 1;
    }
    thinkthen_engine_free(tt);
    printf("smoke: %s\n", a.outcome == THINKTHEN_YES ? "true"
                          : a.outcome == THINKTHEN_NO ? "false" : "null");
    return 0;
}
