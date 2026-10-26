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
    view.data = (uint8_t *)tt_utf8(VARDATA_ANY(value), VARSIZE_ANY_EXHDR(value), &length);
    view.len = length;
    return view;
}

void tt_question(TtOwner *owner, FunctionCallInfo fcinfo, const char *function)
{
    int index;
    int code;
    int word = argument(fcinfo, "question");
    int structured = argument(fcinfo, !strcmp(function, "annotate") ? "toolbox" : "rules");
    thinkthen_text wording = {NULL, 0};
    const char *list_names[] = {"choices", "labels", "levels", "kinds"};
    uint32 ids[] = {THINKTHEN_FUNCTION_DECIDE, THINKTHEN_FUNCTION_CHOOSE, THINKTHEN_FUNCTION_TAG,
        THINKTHEN_FUNCTION_SCORE, THINKTHEN_FUNCTION_FILTER, THINKTHEN_FUNCTION_RANK,
        THINKTHEN_FUNCTION_FIND, THINKTHEN_FUNCTION_RECOGNIZE, THINKTHEN_FUNCTION_RELATE, THINKTHEN_FUNCTION_ANNOTATE};
    const char *functions[] = {"decide", "choose", "tag", "score", "filter", "rank", "find", "recognize", "relate", "annotate"};
    for (index = 0; index < 10; index++) if (!strcmp(function, functions[index])) owner->function = ids[index];
    if (word >= 0) {
        if (!supplied(fcinfo, word)) {TT_BRIDGE(code = thinkthen_question_conversion_error(&owner->error)); tt_check(owner, code);}
        wording = utf8_text(PG_GETARG_TEXT_PP(word));
    }
    TT_BRIDGE(code = thinkthen_question_new(owner->function, wording.data, wording.len, &owner->question, &owner->error));
    tt_check(owner, code);
    for (index = 0; index < 4; index++) {
        int slot = argument(fcinfo, list_names[index]);
        ArrayType *array;
        Datum *values;
        bool *nulls;
        int count;
        int item;
        if (slot < 0) continue;
        if (!supplied(fcinfo, slot)) {TT_BRIDGE(code = thinkthen_question_conversion_error(&owner->error)); tt_check(owner, code);}
        array = PG_GETARG_ARRAYTYPE_P(slot);
        deconstruct_array(array, TEXTOID, -1, false, TYPALIGN_INT, &values, &nulls, &count);
        for (item = 0; item < count; item++) {
            thinkthen_text value;
            if (nulls[item]) {TT_BRIDGE(code = thinkthen_question_conversion_error(&owner->error)); tt_check(owner, code);}
            value = utf8_text(DatumGetTextPP(values[item]));
            if (index == 0) {TT_BRIDGE(code = thinkthen_question_add_choice(owner->question, value.data, value.len, &owner->error));}
            else if (index == 1) {TT_BRIDGE(code = thinkthen_question_add_label(owner->question, value.data, value.len, &owner->error));}
            else if (index == 2) {TT_BRIDGE(code = thinkthen_question_add_level(owner->question, value.data, value.len, &owner->error));}
            else {TT_BRIDGE(code = thinkthen_question_add_kind(owner->question, value.data, value.len, &owner->error));}
            tt_check(owner, code);
        }
    }
    if (structured >= 0) {
        StringInfoData encoded;
        char *json;
        size_t length;
        if (!supplied(fcinfo, structured)) {TT_BRIDGE(code = thinkthen_question_conversion_error(&owner->error)); tt_check(owner, code);}
        json = !strcmp(function, "annotate") ? JsonbToCString(NULL, &PG_GETARG_JSONB_P(structured)->root, 0)
            : TextDatumGetCString(DirectFunctionCall1(array_to_json, PG_GETARG_DATUM(structured)));
        initStringInfo(&encoded);
        appendStringInfo(&encoded, "{\"function\":\"%s\",\"%s\":%s}", function, !strcmp(function, "annotate") ? "questions" : "rules", json);
        json = tt_utf8(encoded.data, encoded.len, &length);
        TT_BRIDGE({thinkthen_question_free(owner->question); owner->question = NULL;
            code = thinkthen_sql_question_from_json((uint8_t *)json, length, &owner->question, &owner->error);});
        tt_check(owner, code);
    }
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
    index = argument(fcinfo, "evidence");
    if (supplied(fcinfo, index)) {
        thinkthen_text evidence = utf8_text(PG_GETARG_TEXT_PP(index));
        TT_BRIDGE(code = thinkthen_question_set_evidence(owner->question, evidence.data, evidence.len, &owner->error));
        tt_check(owner, code);
    }
    index = argument(fcinfo, "top");
    if (supplied(fcinfo, index)) {
        int64 top = PG_GETARG_INT64(index);
        if (top < 0) {TT_BRIDGE(code = thinkthen_question_conversion_error(&owner->error));}
        else {TT_BRIDGE(code = thinkthen_question_set_top(owner->question, (size_t)top, &owner->error));}
        tt_check(owner, code);
    }
}
