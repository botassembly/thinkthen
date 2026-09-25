/*
 * The C surface's slide sample, run exactly as drawn.
 *
 * The sample is the C slide in the product deck's surfaces page. The
 * harness below supplies the slide's context (`text`, `texts`, `lens`,
 * COUNT) and checks the answers; the drawn lines are the lines between
 * the two markers, unchanged. The loopback backend's generic arm gives the
 * yes side 0.9 on every question, so every answer here is THINKTHEN_YES at
 * 0.90. A model would answer "just saying hi" with THINKTHEN_NO.
 *
 * `tests/door/` builds and runs it against that arm.
 */

#include <assert.h>
#include <stdio.h>
#include <string.h>

#include <thinkthen.h>

int main(void) {
    /* The slide's context. */
    const char *text = "I want a refund for order 9";
    const char *texts[3] = {
        "I want a refund for order 9",
        "just saying hi",
        "maybe a refund",
    };
    size_t lens[3] = {
        strlen(texts[0]), strlen(texts[1]), strlen(texts[2]),
    };
    enum { COUNT = 3 };

    /* The sample, as drawn. */
    thinkthen_engine *tt = thinkthen_engine_new();
    const char *q = "Does the customer ask for a refund?";

    /* every call returns 0, or one of six error kinds.
       one text: a.outcome is THINKTHEN_YES, at 0.99 */
    thinkthen_answer a;
    int rc = thinkthen_decide(tt, q, text, strlen(text), &a);

    /* many texts cross once and run 32 at a time */
    thinkthen_answer out[COUNT];
    rc |= thinkthen_decide_many(tt, q, texts, lens, COUNT, out);

    thinkthen_engine_free(tt);
    /* End of the sample. */

    assert(rc == THINKTHEN_OK);
    printf("decide: %d %.2f\n", a.outcome, a.probability);
    for (int i = 0; i < COUNT; i++) {
        printf("decide_many %d: %d %.2f\n", i, out[i].outcome, out[i].probability);
    }
    return 0;
}
