/*
 * Every plain spelling is its `_opts` twin with THINKTHEN_NO_DEADLINE and
 * a null token, on the answer path and the failure path, and the options
 * follow the header's rules: the budget's sentinel, spent, negative, and
 * largest values, and a fired token. The details flag reads its value
 * (R2-26: `"details": false` once turned the audit view on).
 *
 * `tests/door/` runs it against the loopback backend's generic arm.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <thinkthen.h>

static int failed;

static void check(int holds, const char *what) {
    if (!holds) {
        fprintf(stderr, "FAIL %s\n", what);
        failed = 1;
    }
}

static int said(const thinkthen_engine *tt, int code, const char *message) {
    return thinkthen_error_code(tt) == code && strcmp(thinkthen_error_message(tt), message) == 0;
}

/* One JSON-door answer, compared with `expected` and freed. */
static int replies(const thinkthen_engine *tt, char *reply, const char *expected) {
    int same = reply != NULL && strcmp(reply, expected) == 0;
    if (!same) {
        fprintf(stderr, "got %s\n", reply == NULL ? thinkthen_error_message(tt) : reply);
    }
    thinkthen_free_string(reply);
    return same;
}

static int twin(const thinkthen_answer *one, const thinkthen_answer *other) {
    return one->outcome == other->outcome && one->probability == other->probability;
}

int main(void) {
    const int64_t none = THINKTHEN_NO_DEADLINE;
    const char *q = "Is this a complaint?";
    const char *texts[3] = {"good morning", "I demand a refund", "maybe"};
    size_t lengths[3] = {12, 17, 5};
    thinkthen_engine *tt = thinkthen_engine_new();
    if (tt == NULL) {
        fprintf(stderr, "FAIL no engine came\n");
        return 1;
    }

    thinkthen_answer plain = {9, 9.0};
    thinkthen_answer opts = {8, 8.0};
    check(thinkthen_decide(tt, q, "x", 1, &plain) == 0, "decide answers");
    check(thinkthen_decide_opts(tt, q, "x", 1, none, NULL, &opts) == 0, "decide_opts answers");
    check(twin(&plain, &opts) && plain.outcome == THINKTHEN_YES && plain.probability == 0.9,
          "decide equals its twin at yes 0.9");

    thinkthen_answer many[3];
    thinkthen_answer many_opts[3];
    check(thinkthen_decide_many(tt, q, texts, lengths, 3, many) == 0, "decide_many answers");
    check(thinkthen_decide_many_opts(tt, q, texts, lengths, 3, none, NULL, many_opts) == 0,
          "decide_many_opts answers");
    for (int i = 0; i < 3; i++) {
        check(twin(&many[i], &many_opts[i]), "decide_many equals its twin row by row");
    }

    const char *request = "{\"score\":\"How urgent?\",\"levels\":[\"low\",\"high\"],\"evidence\":\"now\"}";
    check(replies(tt, thinkthen_call(tt, request), "0.1"), "call scores");
    check(replies(tt, thinkthen_call_opts(tt, request, none, NULL), "0.1"), "call_opts equals call");

    const char *spec = "{\"version\":1,\"recognize\":{\"kinds\":{\"person\":\"A name.\"}}}";
    char *one = NULL;
    char *other = NULL;
    size_t one_len = 0;
    size_t other_len = 0;
    check(thinkthen_recognize(tt, spec, "Ada", 3, &one, &one_len) == 0, "recognize answers");
    check(thinkthen_recognize_opts(tt, spec, "Ada", 3, none, NULL, &other, &other_len) == 0,
          "recognize_opts answers");
    check(one_len == other_len && strcmp(one, other) == 0, "recognize equals its twin");
    thinkthen_free_string(one);
    thinkthen_free_string(other);

    const char *rules = "{\"version\":1,\"relate\":{\"relations\":[{\"name\":\"r\",\"source\":\"a\",\"target\":\"a\"}]}}";
    const char *records[2] = {"{\"name\":\"x\",\"kind\":\"a\"}", "{\"name\":\"y\",\"kind\":\"a\"}"};
    size_t record_lengths[2] = {strlen(records[0]), strlen(records[1])};
    check(thinkthen_relate(tt, rules, records, record_lengths, 2, &one, &one_len) == 0, "relate answers");
    check(thinkthen_relate_opts(tt, rules, records, record_lengths, 2, none, NULL, &other, &other_len) == 0,
          "relate_opts answers");
    check(one_len == other_len && strcmp(one, other) == 0, "relate equals its twin");
    thinkthen_free_string(one);
    thinkthen_free_string(other);

    /* The failure path: one code and one message for both spellings. */
    const char *choose = "{\"choose\":\"Pick.\",\"options\":[\"a\",\"b\"]}";
    check(thinkthen_decide(tt, choose, "x", 1, &plain) == 1 &&
              said(tt, 1, "decide takes a decide question"),
          "decide refuses a choose question");
    check(thinkthen_decide_opts(tt, choose, "x", 1, none, NULL, &plain) == 1 &&
              said(tt, 1, "decide takes a decide question"),
          "decide_opts refuses it alike");

    /* The budget's rules. */
    check(thinkthen_decide_opts(tt, q, "x", 1, 0, NULL, &plain) == THINKTHEN_EDEADLINE,
          "a spent budget is the deadline kind");
    check(thinkthen_decide_opts(tt, q, "x", 1, -2, NULL, &plain) == 1 &&
              said(tt, 1, "a deadline of -2 milliseconds is not -1, 0, or a positive budget of at most 4294967295 seconds"),
          "another negative budget is refused");
    check(thinkthen_decide_opts(tt, q, "x", 1, INT64_C(4294967295001), NULL, &plain) == 1 &&
              said(tt, 1, "a deadline of 4294967295001 milliseconds is not -1, 0, or a positive budget of at most 4294967295 seconds"),
          "a budget past the largest is refused");
    check(thinkthen_decide_opts(tt, q, "x", 1, INT64_C(4294967295000), NULL, &plain) == 0,
          "the largest budget answers");

    /* A fired token stops every call that carries it. */
    thinkthen_cancel_token *token = thinkthen_cancel_token_new();
    thinkthen_cancel(token);
    thinkthen_cancel(token);
    check(thinkthen_decide_opts(tt, q, "x", 1, none, token, &plain) == THINKTHEN_ECANCELLED &&
              said(tt, THINKTHEN_ECANCELLED, "the call was cancelled") &&
              thinkthen_error_retryable(tt) == 0,
          "a fired token cancels decide");
    check(thinkthen_call_opts(tt, request, none, token) == NULL &&
              said(tt, THINKTHEN_ECANCELLED, "the call was cancelled"),
          "a fired token cancels the JSON door");
    thinkthen_cancel_token_free(token);

    /* R2-26: the details flag reads its value. */
    check(replies(tt, thinkthen_call(tt, "{\"decide\":\"Q?\",\"evidence\":\"x\",\"details\":false}"), "true"),
          "details false answers the bare value");
    char *audit = thinkthen_call(tt, "{\"decide\":\"Q?\",\"evidence\":\"x\",\"details\":true}");
    const char *head = "{\"schema\":\"thinkthen.result/1\",\"value\":true,";
    check(audit != NULL && strncmp(audit, head, strlen(head)) == 0, "details true answers the audit line");
    if (audit != NULL && strncmp(audit, head, strlen(head)) != 0) {
        fprintf(stderr, "got %s\n", audit);
    }
    thinkthen_free_string(audit);
    check(thinkthen_call(tt, "{\"decide\":\"Q?\",\"evidence\":\"x\",\"details\":1}") == NULL &&
              said(tt, 1, "the details key takes true or false"),
          "details takes only true or false");

    thinkthen_engine_free(tt);
    return failed;
}
