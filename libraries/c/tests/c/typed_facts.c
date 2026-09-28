/* Installed C ABI proof for the eight owned-facts typed entry points. */
#include <stdint.h>
#include <pthread.h>
#include <stdio.h>
#include <string.h>
#include <thinkthen.h>

static int failed;
static void check(int holds, const char *what) {
    if (!holds) { fprintf(stderr, "FAIL %s\n", what); failed = 1; }
}
static void facts(char *json, size_t len, int records, int sends) {
    char expected[40];
    snprintf(expected, sizeof expected, "\"records\":%d", records);
    check(json != NULL && strlen(json) == len, "owned facts length");
    check(json != NULL && strstr(json, expected) != NULL, "owned facts records");
    check(json != NULL && strstr(json, "\"requests_sent\":") != NULL, "owned facts sends");
    snprintf(expected, sizeof expected, "\"requests_sent\":%d", sends);
    check(json != NULL && strstr(json, expected) != NULL, "actual request count");
    if (records > 0) check(json != NULL && strstr(json, "\"model\":") != NULL,
                           "observed model stays in facts");
    check(json != NULL && strstr(json, "\"input_tokens\":") == NULL &&
          strstr(json, "\"output_tokens\":") == NULL,
          "unreported tokens stay absent");
    thinkthen_free_string(json);
}
static void result(char *json, size_t len) {
    check(json != NULL && strlen(json) == len, "owned result length");
    thinkthen_free_string(json);
}
struct worker {
    thinkthen_engine *engine;
    thinkthen_answer answer;
    char *facts;
    size_t length;
    int code;
};
static void *other_caller(void *opaque) {
    struct worker *one = opaque;
    one->code = thinkthen_decide_with_facts(one->engine, "Is this a complaint?",
                "one", 3, &one->answer, &one->facts, &one->length);
    return NULL;
}
int main(void) {
    thinkthen_engine *tt = thinkthen_engine_new_with("{\"batch\":1}");
    check(tt != NULL, "engine builds");
    if (tt == NULL) return 1;
    const char *q = "Is this a complaint?";
    if (getchar() == 'E') {
        thinkthen_answer unchanged = {47, 7.0};
        char *owned = (char *)(uintptr_t)17;
        size_t length = 83;
        check(thinkthen_decide_with_facts(tt, q, "failure", 7, &unchanged,
              &owned, &length) == THINKTHEN_EBACKEND, "started backend failure");
        check(unchanged.outcome == 47 && unchanged.probability == 7.0 &&
              owned == (char *)(uintptr_t)17 && length == 83, "failure keeps all outputs");
        const char *borrowed = thinkthen_error_facts_json(tt);
        check(borrowed != NULL && strstr(borrowed, "\"requests_sent\":1") != NULL,
              "started failure keeps final borrowed facts");
        check(thinkthen_decide_with_facts(tt, q, "x", 1, &unchanged, NULL,
              &length) == THINKTHEN_EUSAGE && thinkthen_error_facts_json(tt) == NULL,
              "later prestart failure replaces borrowed facts");
        thinkthen_engine_free(tt);
        return failed;
    }
    thinkthen_answer answer = {9, 9.0};
    char *f = NULL, *out = NULL;
    size_t fl = 0, ol = 0;
    check(thinkthen_decide_with_facts(tt, q, "one", 3, &answer, &f, &fl) == 0, "decide facts");
    check(answer.outcome == THINKTHEN_YES && answer.probability == 0.9, "decide result");
    thinkthen_answer legacy = {9, 9.0};
    check(thinkthen_decide(tt, q, "one", 3, &legacy) == 0 &&
          legacy.outcome == answer.outcome && legacy.probability == answer.probability,
          "legacy scalar keeps the same result");
    struct worker other = {.engine = tt};
    pthread_t thread;
    check(pthread_create(&thread, NULL, other_caller, &other) == 0, "second caller starts");
    check(pthread_join(thread, NULL) == 0, "second caller joins");
    check(other.code == THINKTHEN_OK && other.answer.outcome == THINKTHEN_YES,
          "second caller has its own result");
    facts(f, fl, 1, 1); f = NULL;
    facts(other.facts, other.length, 1, 0);
    check(thinkthen_decide_with_facts_opts(tt, q, "two", 3, THINKTHEN_NO_DEADLINE,
          NULL, &answer, &f, &fl) == 0, "decide opts facts");
    facts(f, fl, 1, 1); f = NULL;
    const char *texts[] = {"three", "four"};
    size_t lengths[] = {5, 4};
    thinkthen_answer rows[2] = {{9, 9}, {9, 9}};
    check(thinkthen_decide_many_with_facts(tt, q, texts, lengths, 2, rows, &f, &fl) == 0,
          "many facts");
    check(rows[0].outcome == THINKTHEN_YES && rows[1].outcome == THINKTHEN_YES,
          "many ordered rows");
    thinkthen_answer legacy_rows[2] = {{9, 9}, {9, 9}};
    check(thinkthen_decide_many(tt, q, texts, lengths, 2, legacy_rows) == 0 &&
          legacy_rows[0].outcome == rows[0].outcome &&
          legacy_rows[1].probability == rows[1].probability,
          "legacy bulk keeps ordered results");
    facts(f, fl, 2, 2); f = NULL;
    check(thinkthen_decide_many_with_facts_opts(tt, q, texts, lengths, 2,
          THINKTHEN_NO_DEADLINE, NULL, rows, &f, &fl) == 0, "many opts facts");
    facts(f, fl, 2, 0); f = NULL;
    const char *recognize = "{\"version\":1,\"recognize\":{\"kinds\":{\"person\":\"A name.\"}}}";
    check(thinkthen_recognize_with_facts(tt, recognize, "Ada", 3, &out, &ol, &f, &fl) == 0,
          "recognize facts");
    char *legacy_json = NULL;
    size_t legacy_len = 0;
    check(thinkthen_recognize(tt, recognize, "Ada", 3, &legacy_json, &legacy_len) == 0 &&
          legacy_len == ol && strcmp(legacy_json, out) == 0,
          "legacy recognition keeps the JSON result");
    thinkthen_free_string(legacy_json);
    result(out, ol); facts(f, fl, 1, 2); out = f = NULL;
    check(thinkthen_recognize_with_facts_opts(tt, recognize, "Bea", 3,
          THINKTHEN_NO_DEADLINE, NULL, &out, &ol, &f, &fl) == 0, "recognize opts facts");
    result(out, ol); facts(f, fl, 1, 2); out = f = NULL;
    const char *relate = "{\"version\":1,\"relate\":{\"relations\":[{\"name\":\"r\",\"source\":\"a\",\"target\":\"a\"}]}}";
    const char *entities[] = {"{\"name\":\"x\",\"kind\":\"a\"}", "{\"name\":\"y\",\"kind\":\"a\"}"};
    size_t entity_lengths[] = {strlen(entities[0]), strlen(entities[1])};
    check(thinkthen_relate_with_facts(tt, relate, entities, entity_lengths, 2,
          &out, &ol, &f, &fl) == 0, "relate facts");
    legacy_json = NULL;
    check(thinkthen_relate(tt, relate, entities, entity_lengths, 2,
          &legacy_json, &legacy_len) == 0 && legacy_len == ol &&
          strcmp(legacy_json, out) == 0, "legacy relation keeps the JSON result");
    thinkthen_free_string(legacy_json);
    result(out, ol); facts(f, fl, 1, 1); out = f = NULL;
    check(thinkthen_relate_with_facts_opts(tt, relate, entities, entity_lengths, 2,
          THINKTHEN_NO_DEADLINE, NULL, &out, &ol, &f, &fl) == 0, "relate opts facts");
    result(out, ol); facts(f, fl, 1, 0); out = f = NULL;

    /* Refusals never publish either output and never send. */
    answer.outcome = 47; f = (char *)(uintptr_t)17; fl = 83;
    check(thinkthen_decide_with_facts(tt, q, "x", 1, &answer, NULL, &fl) == THINKTHEN_EUSAGE,
          "null facts output");
    check(answer.outcome == 47 && fl == 83, "null leaves sentinels");
    check(thinkthen_decide_with_facts(tt, q, "x", SIZE_MAX, &answer, &f, &fl) == THINKTHEN_EUSAGE,
          "unrepresentable text length");
    check(answer.outcome == 47 && f == (char *)(uintptr_t)17 && fl == 83,
          "length refusal leaves sentinels");
    check(thinkthen_decide_many_with_facts(tt, q, texts, lengths, SIZE_MAX,
          rows, &f, &fl) == THINKTHEN_EUSAGE, "unrepresentable arrays");
    check(thinkthen_decide_with_facts(tt, "{bad", "x", 1, &answer,
          &f, &fl) == THINKTHEN_EUSAGE && answer.outcome == 47 &&
          f == (char *)(uintptr_t)17 && fl == 83,
          "malformed question leaves outputs unchanged");
    check(thinkthen_recognize_with_facts(tt, recognize, "x", 1,
          (char **)&fl, &fl, &f, &ol) == THINKTHEN_EUSAGE, "output alias");
    check(thinkthen_recognize_with_facts(tt, recognize, "x", 1,
          &out, &ol, &out, &fl) == THINKTHEN_EUSAGE, "result and facts pointer alias");
    check(thinkthen_recognize_with_facts(tt, recognize, "x", 1,
          &out, &ol, &f, &ol) == THINKTHEN_EUSAGE, "length slot alias");
    check(thinkthen_recognize_with_facts(tt, recognize, "x", 1,
          &out, (size_t *)&f, &f, &fl) == THINKTHEN_EUSAGE, "result length and facts alias");
    check(thinkthen_recognize_with_facts(tt, recognize, "x", 1,
          (char **)&fl, &ol, &f, &fl) == THINKTHEN_EUSAGE, "result and facts length alias");
    check(thinkthen_recognize_with_facts(tt, recognize, "x", 1,
          &out, &ol, (char **)&fl, &fl) == THINKTHEN_EUSAGE, "facts pointer and length alias");
    check(thinkthen_decide_with_facts(tt, q, "x", 1,
          (thinkthen_answer *)&f, &f, &fl) == THINKTHEN_EUSAGE, "typed result and facts alias");
    check(thinkthen_decide_with_facts(tt, q, "x", 1,
          (thinkthen_answer *)&fl, &f, &fl) == THINKTHEN_EUSAGE, "typed result and length alias");
    check(thinkthen_decide_with_facts(tt, q, "x", 1,
          &answer, (char **)&fl, &fl) == THINKTHEN_EUSAGE, "typed facts slots alias");
    check(thinkthen_decide_with_facts_opts(tt, q, "x", 1, 0, NULL,
          &answer, &f, &fl) == THINKTHEN_EDEADLINE, "spent budget");
    check(f == (char *)(uintptr_t)17 && fl == 83, "deadline leaves facts sentinels");
    check(thinkthen_decide_with_facts(tt, q, "x", 1, &answer, NULL, &fl) == THINKTHEN_EUSAGE &&
          thinkthen_error_facts_json(tt) == NULL, "prestart refusal has no facts");
    check(thinkthen_decide_many_with_facts(tt, q, NULL, NULL, 0,
          (thinkthen_answer *)&f, &f, &fl) == THINKTHEN_EUSAGE,
          "even a zero-count nonnull output cannot alias facts");
    check(thinkthen_decide_many_with_facts(tt, q, NULL, NULL, 0,
          NULL, &f, &fl) == 0, "zero-count many");
    facts(f, fl, 0, 0);
    thinkthen_engine_free(tt);
    return failed;
}
