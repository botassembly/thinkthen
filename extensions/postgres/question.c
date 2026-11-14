#include "pg_internal.h"
#include "catalog/pg_type_d.h"
#include "lib/stringinfo.h"
#include "utils/array.h"
#include "utils/builtins.h"
#include "utils/json.h"
#include "funcapi.h"
#include "utils/syscache.h"
#include "catalog/pg_proc.h"
#include <math.h>

static int argument(FunctionCallInfo fcinfo, const char *name)
{
    Oid *types;
    char **names;
    char *modes;
    HeapTuple procedure = SearchSysCache1(PROCOID, ObjectIdGetDatum(fcinfo->flinfo->fn_oid));
    int count = get_func_arg_info(procedure, &types, &names, &modes);
    int index;
    for (index = 0; index < count && index < PG_NARGS(); index++)
        if (names != NULL && !strcmp(names[index], name)) {ReleaseSysCache(procedure); return index;}
    ReleaseSysCache(procedure);
    return -1;
}

static bool supplied(FunctionCallInfo fcinfo, int index)
{
    return index >= 0 && !PG_ARGISNULL(index);
}

static thinkthen_text utf8_text(text *value)
{
    thinkthen_text view;
    size_t length;
    view.data = tt_utf8(VARDATA_ANY(value), VARSIZE_ANY_EXHDR(value), &length);
    view.len = length;
    return view;
}

/* A required argument or list item that is SQL NULL keeps the engine's code. */
static void null_argument(TtOwner *owner, const char *name)
{
    int code;
    TT_BRIDGE(code = thinkthen_question_conversion_error(&owner->error));
    (void)code;
    ereport(ERROR, (errcode(ERRCODE_NULL_VALUE_NOT_ALLOWED), errmsg("%s cannot be SQL NULL", name),
        errdetail_internal("%s", thinkthen_code_name(owner->error.code))));
}

void tt_question(TtOwner *owner, FunctionCallInfo fcinfo, const char *function)
{
    int index;
    int code;
    int word = argument(fcinfo, "question");
    int structured = argument(fcinfo, !strcmp(function, "annotate") ? "questions" : "rules");
    StringInfoData encoded;
    char *json;
    size_t length;
    const char *list_names[] = {"choices", "labels", "levels", "kinds"};
    thinkthen_function ids[] = {THINKTHEN_FUNCTION_DECIDE, THINKTHEN_FUNCTION_CHOOSE, THINKTHEN_FUNCTION_TAG,
        THINKTHEN_FUNCTION_SCORE, THINKTHEN_FUNCTION_FILTER, THINKTHEN_FUNCTION_RANK,
        THINKTHEN_FUNCTION_FIND, THINKTHEN_FUNCTION_RECOGNIZE, THINKTHEN_FUNCTION_RELATE, THINKTHEN_FUNCTION_ANNOTATE};
    const char *functions[] = {"decide", "choose", "tag", "score", "filter", "rank", "find", "recognize", "relate", "annotate"};
    for (index = 0; index < 10; index++) if (!strcmp(function, functions[index])) owner->function = ids[index];
    initStringInfo(&encoded);
    appendStringInfo(&encoded, "{\"function\":\"%s\"", function);
    if (word >= 0) {
        if (!supplied(fcinfo, word)) null_argument(owner, "question");
        appendStringInfoString(&encoded, ",\"question\":");
        escape_json(&encoded, text_to_cstring(PG_GETARG_TEXT_PP(word)));
    }
    for (index = 0; index < 4; index++) {
        int slot = argument(fcinfo, list_names[index]);
        int descriptions_slot = argument(fcinfo, "descriptions");
        Jsonb *descriptions = supplied(fcinfo, descriptions_slot) ? PG_GETARG_JSONB_P(descriptions_slot) : NULL;
        Datum *values;
        bool *nulls;
        int count;
        int item;
        bool unmatched = descriptions != NULL && !JB_ROOT_IS_OBJECT(descriptions);
        if (slot < 0) continue;
        if (!supplied(fcinfo, slot)) null_argument(owner, list_names[index]);
        deconstruct_array(PG_GETARG_ARRAYTYPE_P(slot), TEXTOID, -1, false, TYPALIGN_INT, &values, &nulls, &count);
        appendStringInfo(&encoded, ",\"%s\":[", list_names[index]);
        for (item = 0; item < count; item++) {
            char *value;
            JsonbValue *description;
            if (nulls[item]) null_argument(owner, list_names[index]);
            value = TextDatumGetCString(values[item]);
            description = descriptions != NULL && JB_ROOT_IS_OBJECT(descriptions) ? tt_field(descriptions, value) : NULL;
            if (item > 0) appendStringInfoChar(&encoded, ',');
            if (description != NULL) {
                appendStringInfoString(&encoded, "{\"name\":");
                escape_json(&encoded, value);
                appendStringInfoString(&encoded, ",\"description\":");
                appendStringInfoString(&encoded, JsonbToCString(NULL, &JsonbValueToJsonb(description)->root, 0));
                appendStringInfoChar(&encoded, '}');
            } else escape_json(&encoded, value);
        }
        appendStringInfoChar(&encoded, ']');
        if (descriptions != NULL && JB_ROOT_IS_OBJECT(descriptions)) {
            JsonbIterator *iterator = JsonbIteratorInit(&descriptions->root);
            JsonbIteratorToken token;
            JsonbValue key;
            while ((token = JsonbIteratorNext(&iterator, &key, true)) != WJB_DONE) {
                bool found = false;
                if (token != WJB_KEY) continue;
                for (item = 0; item < count; item++) {
                    text *label = DatumGetTextPP(values[item]);
                    if (VARSIZE_ANY_EXHDR(label) == (size_t)key.val.string.len &&
                        !memcmp(VARDATA_ANY(label), key.val.string.val, key.val.string.len)) found = true;
                }
                if (!found) unmatched = true;
            }
        }
        if (unmatched)
            appendStringInfo(&encoded, ",\"descriptions\":%s", JsonbToCString(NULL, &descriptions->root, 0));
    }
    if (structured >= 0) {
        if (!supplied(fcinfo, structured)) null_argument(owner, !strcmp(function, "annotate") ? "questions" : "rules");
        json = !strcmp(function, "annotate") ? JsonbToCString(NULL, &PG_GETARG_JSONB_P(structured)->root, 0)
            : TextDatumGetCString(DirectFunctionCall1(array_to_json, PG_GETARG_DATUM(structured)));
        appendStringInfo(&encoded, ",\"%s\":%s", !strcmp(function, "annotate") ? "questions" : "rules", json);
    }
    {
        const char *names[] = {"yes_means", "no_means"};
        const char *keys[] = {"yes_means", "no_means"};
        for (index = 0; index < 2; index++) {
            int slot = argument(fcinfo, names[index]);
            if (supplied(fcinfo, slot)) {
                appendStringInfo(&encoded, ",\"%s\":", keys[index]);
                escape_json(&encoded, text_to_cstring(PG_GETARG_TEXT_PP(slot)));
            }
        }
    }
    appendStringInfoChar(&encoded, '}');
    json = tt_utf8(encoded.data, encoded.len, &length);
    TT_BRIDGE(code = thinkthen_sql_question_from_json(json, length, &owner->question, &owner->error));
    tt_check(owner, code);
    {
        const char *names[] = {"threshold", "low", "high"};
        double numbers[3];
        const double *values[3] = {NULL, NULL, NULL};
        for (index = 0; index < 3; index++) {
            int slot = argument(fcinfo, names[index]);
            if (supplied(fcinfo, slot)) {numbers[index] = PG_GETARG_FLOAT8(slot); values[index] = &numbers[index];}
        }
        TT_BRIDGE(code = thinkthen_sql_question_set_threshold(owner->question, values[0], values[1], values[2], &owner->error));
        tt_check(owner, code);
    }
    index = argument(fcinfo, "field");
    if (supplied(fcinfo, index)) {
        thinkthen_text field = utf8_text(PG_GETARG_TEXT_PP(index));
        TT_BRIDGE(code = thinkthen_question_set_field(owner->question, field.data, field.len, &owner->error));
        tt_check(owner, code);
    }
    index = argument(fcinfo, "raise_if_failed");
    owner->raise_if_failed = supplied(fcinfo, index) && PG_GETARG_BOOL(index);
    index = argument(fcinfo, "top");
    if (supplied(fcinfo, index)) {
        int64 top = PG_GETARG_INT64(index);
        if (top < 0) {TT_BRIDGE(code = thinkthen_question_conversion_error(&owner->error));}
        else {TT_BRIDGE(code = thinkthen_question_set_top(owner->question, (size_t)top, &owner->error));}
        tt_check(owner, code);
    }
}
