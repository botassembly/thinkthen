/* Controlled native replies for the public Ada facts boundary: facts pass
 * through as JSON text, and each native string is released once. */
#include "thinkthen.h"
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

static int selected;
static int calls;
static int frees;
void facts_boundary_select(int mode) { selected = mode; }
int facts_boundary_calls(void) { return calls; }
int facts_boundary_frees(void) { return frees; }
void thinkthen_free_string(char *text) { if (text) { frees++; free(text); } }

static char *owned(const char *text) {
    size_t length = strlen(text);
    char *copy = malloc(length + 1);
    if (copy) memcpy(copy, text, length + 1);
    return copy;
}
static int publish_facts(const char *report, char **facts, size_t *length) {
    char *copy = owned(report);
    if (!copy) return THINKTHEN_ELOCAL;
    *facts = copy; *length = strlen(report);
    return THINKTHEN_OK;
}
static const char good[] =
    "{\"records\":1,\"requests_sent\":1,\"cache_answers\":0,\"seconds\":0.125,\"model\":\"synthetic-model\"}";
static const char null_optional[] =
    "{\"records\":2,\"requests_sent\":1,\"cache_answers\":0,\"seconds\":0.125,\"input_tokens\":null}";
static const char unknown_member[] =
    "{\"records\":1,\"requests_sent\":1,\"cache_answers\":0,\"seconds\":0.125,\"surprise\":1}";

int thinkthen_decide_with_facts_opts(const thinkthen_engine *engine, const char *question,
    const char *text, size_t text_len, int64_t deadline, thinkthen_cancel_token *token,
    thinkthen_answer *out, char **facts, size_t *facts_len) {
    calls++;
    if (!engine || !question || strcmp(question, "Is it?") || !text ||
        text_len != 8 || memcmp(text, "evidence", 8) || deadline != -1 || token ||
        !out || !facts || !facts_len || selected != 1)
        return THINKTHEN_EUSAGE;
    int status = publish_facts(good, facts, facts_len);
    if (status == THINKTHEN_OK) { out->outcome = 1; out->probability = 0.9; }
    return status;
}
int thinkthen_decide_many_with_facts_opts(const thinkthen_engine *engine, const char *question,
    const char *const *texts, const size_t *lengths, size_t count, int64_t deadline,
    thinkthen_cancel_token *token, thinkthen_answer *out, char **facts, size_t *facts_len) {
    calls++;
    if (!engine || !question || strcmp(question, "Is it?") || !texts || !lengths ||
        count != 2 || lengths[0] != 3 || memcmp(texts[0], "one", 3) ||
        lengths[1] != 3 || memcmp(texts[1], "two", 3) || deadline != -1 || token ||
        !out || !facts || !facts_len || selected != 3) return THINKTHEN_EUSAGE;
    int status = publish_facts(null_optional, facts, facts_len);
    if (status == THINKTHEN_OK) {
        out[0].outcome = 1; out[0].probability = 0.9;
        out[1].outcome = 0; out[1].probability = 0.1;
    }
    return status;
}
/* A null report succeeds with no facts, which the Ada layer calls a defect. */
static int structured(const char *value, const char *report, char **out, size_t *out_len,
                      char **facts, size_t *facts_len) {
    char *result = owned(value), *details = report ? owned(report) : NULL;
    if (!result || (report && !details)) { free(result); free(details); return THINKTHEN_ELOCAL; }
    *out = result; *out_len = strlen(value);
    *facts = details; *facts_len = report ? strlen(report) : 0;
    return THINKTHEN_OK;
}
int thinkthen_recognize_with_facts_opts(const thinkthen_engine *engine, const char *spec,
    const char *text, size_t text_len, int64_t deadline, thinkthen_cancel_token *token,
    char **out, size_t *out_len, char **facts, size_t *facts_len) {
    calls++;
    if (!engine || !spec || !text || text_len != 4 || memcmp(text, "text", 4) ||
        deadline != -1 || token || !out || !out_len || !facts || !facts_len || selected != 4)
        return THINKTHEN_EUSAGE;
    return structured("{\"entities\":[]}", NULL, out, out_len, facts, facts_len);
}
int thinkthen_relate_with_facts_opts(const thinkthen_engine *engine, const char *spec,
    const char *const *texts, const size_t *lengths, size_t count, int64_t deadline,
    thinkthen_cancel_token *token, char **out, size_t *out_len, char **facts,
    size_t *facts_len) {
    calls++;
    if (!engine || !spec || !texts || !lengths || count != 2 ||
        deadline != -1 || token || !out || !out_len || !facts || !facts_len || selected != 5)
        return THINKTHEN_EUSAGE;
    return structured("{\"edges\":[]}", unknown_member, out, out_len, facts, facts_len);
}
