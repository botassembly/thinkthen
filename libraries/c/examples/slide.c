/*
 * The C surface's slide sample, run exactly as drawn.
 *
 * The sample is the C slide in
 * repos/mktg/decks/2026-09-21-thinkthen-semantic-commands/surfaces.md.
 * The harness below supplies the slide's context (`text`, `texts`, `lens`,
 * COUNT) and checks the answers; the drawn lines are the lines between
 * the two markers, unchanged. The stub's rule answers "refund" evidence
 * with 0.97, so THINKTHEN_YES comes with probability 0.97 here and 0.99
 * on the real backend; the answer class is what the sample promises.
 *
 * Build and run through ./check.sh.
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

    /* The answers the stub's rule gives, pinned so a change is a finding.
     * "refund" evidence answers 0.97, which the default cut keeps; "just
     * saying hi" answers 0.03 and is dropped by the same cut. */
    assert(rc == THINKTHEN_OK);
    assert(a.outcome == THINKTHEN_YES);
    assert(a.probability > 0.96 && a.probability < 0.98);

    assert(out[0].outcome == THINKTHEN_YES);
    assert(out[1].outcome == THINKTHEN_NO);
    assert(out[2].outcome == THINKTHEN_YES);

    printf("slide ok: decide YES at %.2f, decide_many YES NO YES\n",
        a.probability);
    return 0;
}
