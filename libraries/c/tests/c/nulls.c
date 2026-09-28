/*
 * The header's argument rules, one row per pointer (DESIGN.md section 4).
 *
 * Each row checks the return, the recorded code, the exact message, and
 * that the out parameters kept what they held. `tests/door/` also counts
 * the loopback backend's requests after this program and expects none,
 * because every refusal comes before the engine sends.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <thinkthen.h>

static int failed;

static void row(const thinkthen_engine *tt, const char *name, int rc, int code, const char *message) {
    const char *said = thinkthen_error_message(tt);
    if (rc != code || thinkthen_error_code(tt) != code || strcmp(said, message) != 0) {
        fprintf(stderr, "FAIL %s: rc %d code %d message \"%s\"\n", name, rc,
                thinkthen_error_code(tt), said);
        failed = 1;
    }
}

int main(void) {
    const char *q = "Is this a complaint?";
    const char *bad = "\xff\xfe";
    const char *texts[1] = {"a text"};
    size_t lengths[1] = {6};
    thinkthen_answer kept = {7, 7.0};
    char *out = (char *)q;
    size_t out_len = 7;

    /* No engine: the usage code, and no table to hold a message. */
    if (thinkthen_decide(NULL, q, "x", 1, &kept) != THINKTHEN_EUSAGE ||
        thinkthen_decide_many(NULL, q, texts, lengths, 1, &kept) != THINKTHEN_EUSAGE ||
        thinkthen_recognize(NULL, "{}", "x", 1, &out, &out_len) != THINKTHEN_EUSAGE ||
        thinkthen_relate(NULL, "{}", texts, lengths, 1, &out, &out_len) != THINKTHEN_EUSAGE ||
        thinkthen_call(NULL, "{}") != NULL || thinkthen_error_code(NULL) != THINKTHEN_EUSAGE ||
        thinkthen_error_facts_json(NULL) != NULL ||
        thinkthen_error_retryable(NULL) != 0 ||
        strcmp(thinkthen_error_message(NULL), "no engine came, so no failure is named") != 0) {
        fprintf(stderr, "FAIL a null engine\n");
        failed = 1;
    }
    thinkthen_engine_free(NULL);
    thinkthen_free_string(NULL);
    thinkthen_cancel(NULL);
    thinkthen_cancel_token_free(NULL);

    thinkthen_engine *tt = thinkthen_engine_new();
    if (tt == NULL) {
        fprintf(stderr, "FAIL no engine came\n");
        return 1;
    }
    row(tt, "no failure yet", THINKTHEN_OK, THINKTHEN_OK, "no failure yet");
    if (thinkthen_error_facts_json(tt) != NULL) {
        fprintf(stderr, "FAIL a new engine has failure facts\n");
        failed = 1;
    }

    row(tt, "decide null question", thinkthen_decide(tt, NULL, "x", 1, &kept), 1, "a null question");
    row(tt, "decide question not UTF-8", thinkthen_decide(tt, bad, "x", 1, &kept), 1, "the question is not UTF-8");
    row(tt, "decide null text, length", thinkthen_decide(tt, q, NULL, 3, &kept), 1, "a null text with a nonzero length");
    row(tt, "decide null text, zero", thinkthen_decide(tt, q, NULL, 0, &kept), 1, "evidence is text, not white space");
    row(tt, "decide text not UTF-8", thinkthen_decide(tt, q, bad, 2, &kept), 1, "a text is not UTF-8");
    row(tt, "decide null out", thinkthen_decide(tt, q, "x", 1, NULL), 1, "a null out pointer");

    row(tt, "many null texts", thinkthen_decide_many(tt, q, NULL, lengths, 1, &kept), 1, "a null texts array with a nonzero count");
    row(tt, "many null lengths", thinkthen_decide_many(tt, q, texts, NULL, 1, &kept), 1, "a null lengths array with a nonzero count");
    row(tt, "many null out", thinkthen_decide_many(tt, q, texts, lengths, 1, NULL), 1, "a null out array with a nonzero count");
    row(tt, "many null question", thinkthen_decide_many(tt, NULL, texts, lengths, 1, &kept), 1, "a null question");
    if (thinkthen_decide_many(tt, q, NULL, NULL, 0, NULL) != THINKTHEN_OK) {
        fprintf(stderr, "FAIL a zero count with null arrays\n");
        failed = 1;
    }

    row(tt, "call null request", thinkthen_call(tt, NULL) == NULL, 1, "a null request");
    if (thinkthen_error_facts_json(tt) != NULL) {
        fprintf(stderr, "FAIL a parser refusal gained call facts\n");
        failed = 1;
    }
    row(tt, "call request not UTF-8", thinkthen_call(tt, bad) == NULL, 1, "the request is not UTF-8");
    row(tt, "find none not a boolean",
        thinkthen_call(tt, "{\"find\":\"Which?\",\"none\":\"yes\",\"units\":[\"a\",\"b\"]}") == NULL, 1,
        "find takes `none` as true or false");

    row(tt, "recognize null spec", thinkthen_recognize(tt, NULL, "x", 1, &out, &out_len), 1, "a null spec");
    row(tt, "recognize null out", thinkthen_recognize(tt, "{}", "x", 1, NULL, &out_len), 1, "a null out pointer");
    row(tt, "recognize null out_len", thinkthen_recognize(tt, "{}", "x", 1, &out, NULL), 1, "a null out_len pointer");

    row(tt, "relate null spec", thinkthen_relate(tt, NULL, texts, lengths, 1, &out, &out_len), 1, "a null spec");
    row(tt, "relate null texts", thinkthen_relate(tt, "{\"version\":1,\"relate\":{\"relations\":[{\"name\":\"r\",\"source\":\"*\",\"target\":\"*\"}]}}", NULL, lengths, 1, &out, &out_len), 1, "a null texts array with a nonzero count");
    row(tt, "relate past 255", thinkthen_relate(tt, NULL, NULL, NULL, 256, NULL, NULL), 1, "relate takes at most 255 records");
    row(tt, "relate null out", thinkthen_relate(tt, "{}", texts, lengths, 1, NULL, &out_len), 1, "a null out pointer");

    if (kept.outcome != 7 || kept.probability != 7.0 || out != q || out_len != 7) {
        fprintf(stderr, "FAIL a refusal wrote an out parameter\n");
        failed = 1;
    }
    thinkthen_engine_free(tt);
    return failed;
}
