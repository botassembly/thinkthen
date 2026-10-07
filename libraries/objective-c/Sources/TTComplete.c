#include "TTComplete.h"
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <errno.h>
#include <limits.h>

static bool text(const TTJSON *value, TTCText *out) {
    if (!value || value->type != TTJSONString) return false;
    *out = (TTCText){value->text,value->text_length};
    return true;
}
static bool number(const TTJSON *value, double *out) {
    if (!value || value->type != TTJSONNumber) return false;
    char *end = NULL;
    errno = 0;
    double n = strtod(value->text,&end);
    if (errno || end != value->text + value->text_length || !isfinite(n)) return false;
    *out = n;
    return true;
}
static bool probability(const TTJSON *value, double *out) {
    return number(value,out) && *out >= 0 && *out <= 1;
}
static bool word(TTCText value, const char *expected) {
    return value.length == strlen(expected) && !memcmp(value.data,expected,value.length);
}
bool ttc_atomic_read(const TTJSON *object, TTCAtomicAnswer *out, TTCProbability *storage, size_t capacity) {
    if (!object || object->type != TTJSONObject || !out) return false;
    TTCAtomicAnswer value = {0};
    TTCText kind;
    if (!text(tt_json_get(object,"kind"),&kind)) return false;
    if (word(kind,"yes_no")) {
        value.kind = TTCAtomicKindYesNo;
        value.probability.present = true;
        if (!probability(tt_json_get(object,"probability"),&value.probability.value)) return false;
        *out = value;
        return true;
    }
    if (word(kind,"choice")) value.kind = TTCAtomicKindChoice;
    else if (word(kind,"tag")) value.kind = TTCAtomicKindTag;
    else if (word(kind,"score")) value.kind = TTCAtomicKindScore;
    else if (word(kind,"find")) value.kind = TTCAtomicKindFind;
    else return false;
    if (value.kind == TTCAtomicKindChoice || value.kind == TTCAtomicKindFind) {
        value.pick.present = true;
        if (!text(tt_json_get(object,"pick"),&value.pick.value)) return false;
    }
    if (value.kind == TTCAtomicKindScore) {
        value.level.present = true;
        if (!text(tt_json_get(object,"level"),&value.level.value)) return false;
    }
    const TTJSON *confidence = tt_json_get(object,"confidence");
    if (confidence && value.kind != TTCAtomicKindTag) {
        value.confidence.present = true;
        if (!probability(confidence,&value.confidence.value)) return false;
    }
    const TTJSON *distribution = tt_json_get(object,"probabilities");
    if (!distribution || distribution->type != TTJSONObject || distribution->count > capacity ||
        distribution->count > PTRDIFF_MAX / sizeof(*storage) || (distribution->count && !storage)) return false;
    for (size_t i=0; i<distribution->count; ++i) {
        double p;
        if (!probability(distribution->children[i],&p)) return false;
    }
    /* Commit only after all validation: refusal changes neither output nor scratch. */
    for (size_t i=0; i<distribution->count; ++i) {
        storage[i].name = (TTCText){distribution->keys[i],distribution->key_lengths[i]};
        (void)probability(distribution->children[i],&storage[i].value);
    }
    value.probabilities = (TTCListProbability){storage,distribution->count};
    *out = value;
    return true;
}
static bool integer(const TTJSON *node, uint64_t *out) {
    if (!node || node->type != TTJSONNumber || !node->text_length) return false;
    uint64_t value = 0;
    for (size_t i=0; i<node->text_length; ++i) {
        unsigned char c = (unsigned char)node->text[i];
        if (c < '0' || c > '9' || value > (UINT64_MAX - (c-'0')) / 10) return false;
        value = value*10+(c-'0');
    }
    *out = value;
    return true;
}
static bool optional_integer(const TTJSON *object, const char *key, TTCOptionalulong *out) {
    const TTJSON *node = tt_json_get(object,key);
    if (!node) return true;
    out->present = true;
    return integer(node,&out->value);
}
static bool optional_text(const TTJSON *object, const char *key, TTCOptionalstring *out) {
    const TTJSON *node = tt_json_get(object,key);
    if (!node) return true;
    out->present = true;
    return text(node,&out->value);
}
bool ttc_facts_read(const TTJSON *object, TTCCallFacts *out) {
    if (!object || object->type != TTJSONObject || !out) return false;
    TTCCallFacts value = {0};
    TTCText id;
    if (!text(tt_json_get(object,"call_id"),&id) || !ttc_identity(id.data,id.length,value.callId.value) ||
        !integer(tt_json_get(object,"cache_answers"),&value.cacheAnswers) ||
        !integer(tt_json_get(object,"records"),&value.records) ||
        !integer(tt_json_get(object,"requests_sent"),&value.requestsSent) ||
        !number(tt_json_get(object,"seconds"),&value.seconds) || value.seconds < 0 ||
        !optional_integer(object,"input_tokens",&value.inputTokens) ||
        !optional_integer(object,"output_tokens",&value.outputTokens) ||
        !optional_integer(object,"command_ms",&value.commandMs) ||
        !optional_text(object,"model",&value.model) ||
        !optional_text(object,"estimated_cost_usd",&value.estimatedCostUsd)) return false;
    if (value.estimatedCostUsd.present) {
        TTCText cost = value.estimatedCostUsd.value;
        if (cost.length < 8 || cost.data[cost.length-7] != '.') return false;
        for (size_t i=0; i<cost.length; ++i) if (i != cost.length-7 && (cost.data[i]<'0' || cost.data[i]>'9')) return false;
    }
    *out = value;
    return true;
}
