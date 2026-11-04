#include "pg_internal.h"
#include "catalog/pg_type_d.h"
#include "utils/array.h"
#include "utils/builtins.h"
#include "utils/lsyscache.h"
#include "utils/typcache.h"
#include <math.h>

Jsonb *tt_json(const thinkthen_buffer *buffer)
{
    text *encoded;
    if (buffer->data == NULL) return NULL;
    encoded = tt_server_text((const char *)buffer->data, buffer->len);
    return DatumGetJsonbP(DirectFunctionCall1(jsonb_in, CStringGetDatum(text_to_cstring(encoded))));
}

JsonbValue *tt_field(Jsonb *object, const char *name)
{
    JsonbValue key;
    if (object == NULL) return NULL;
    key.type = jbvString;
    key.val.string.val = (char *)name;
    key.val.string.len = strlen(name);
    return findJsonbValueFromContainer(&object->root, JB_FOBJECT, &key);
}

Datum tt_json_value(JsonbValue *value, Oid type, bool *is_null)
{
    Oid element;
    Oid input;
    Oid parameter;
    char *contents;
    JsonbValue scalar;
    if (value != NULL && value->type == jbvBinary && JsonContainerIsScalar(value->val.binary.data)) {
        JsonbIterator *iterator = JsonbIteratorInit(value->val.binary.data);
        JsonbIteratorNext(&iterator, &scalar, true);
        JsonbIteratorNext(&iterator, &scalar, true);
        value = &scalar;
    }
    *is_null = value == NULL || value->type == jbvNull;
    if (*is_null) return (Datum)0;
    if (type == JSONBOID) return JsonbPGetDatum(JsonbValueToJsonb(value));
    element = get_element_type(type);
    if (OidIsValid(element)) {
        JsonbIterator *iterator;
        JsonbIteratorToken token;
        JsonbValue item;
        Datum *values;
        bool *nulls;
        int count = 0;
        int dimension;
        int lower = 1;
        int16 length;
        bool by_value;
        char alignment;
        Jsonb *array = JsonbValueToJsonb(value);
        if (!JB_ROOT_IS_ARRAY(array)) elog(ERROR, "invalid engine array representation");
        dimension = JB_ROOT_COUNT(array);
        values = palloc(sizeof(Datum) * Max(1, dimension));
        nulls = palloc(sizeof(bool) * Max(1, dimension));
        iterator = JsonbIteratorInit(&array->root);
        while ((token = JsonbIteratorNext(&iterator, &item, true)) != WJB_DONE)
            if (token == WJB_ELEM) {
                values[count] = tt_json_value(&item, element, &nulls[count]);
                count++;
            }
        get_typlenbyvalalign(element, &length, &by_value, &alignment);
        return PointerGetDatum(construct_md_array(values, nulls, 1, &dimension, &lower,
            element, length, by_value, alignment));
    }
    if (get_typtype(type) == TYPTYPE_COMPOSITE) {
        TupleDesc description = lookup_rowtype_tupdesc(type, -1);
        Datum *values = palloc0(sizeof(Datum) * description->natts);
        bool *nulls = palloc(sizeof(bool) * description->natts);
        Jsonb *object = JsonbValueToJsonb(value);
        HeapTuple tuple;
        int column;
        for (column = 0; column < description->natts; column++) {
            Form_pg_attribute attribute = TupleDescAttr(description, column);
            const char *name = NameStr(attribute->attname);
            /* SQL spans say start_offset and end_offset, since end is reserved. */
            JsonbValue *field = tt_field(object, !strcmp(name, "start_offset") ? "start"
                : !strcmp(name, "end_offset") ? "end" : name);
            if (field == NULL && !strcmp(name, "either")) {
                values[column] = BoolGetDatum(false);
                nulls[column] = false;
            } else values[column] = tt_json_value(field, attribute->atttypid, &nulls[column]);
        }
        tuple = heap_form_tuple(description, values, nulls);
        ReleaseTupleDesc(description);
        return HeapTupleGetDatum(tuple);
    }
    contents = value->type == jbvString ? pnstrdup(value->val.string.val, value->val.string.len)
        : JsonbToCString(NULL, &JsonbValueToJsonb(value)->root, 0);
    getTypeInputInfo(type, &input, &parameter);
    return OidInputFunctionCall(input, contents, parameter, -1);
}

static bool is_container(JsonbIteratorToken token)
{
    return token == WJB_BEGIN_ARRAY || token == WJB_BEGIN_OBJECT || token == WJB_END_ARRAY || token == WJB_END_OBJECT;
}

/* Copy one engine answer, renaming span keys as the typed span columns name them. */
static void push_answer(JsonbParseState **state, JsonbValue *value)
{
    JsonbIterator *iterator;
    JsonbIteratorToken token;
    JsonbValue item;
    if (value->type != jbvBinary) {
        pushJsonbValue(state, WJB_VALUE, value);
        return;
    }
    iterator = JsonbIteratorInit(value->val.binary.data);
    while ((token = JsonbIteratorNext(&iterator, &item, false)) != WJB_DONE) {
        if (token == WJB_KEY && item.val.string.len == 5 && !memcmp(item.val.string.val, "start", 5)) {
            item.val.string.val = (char *)"start_offset";
            item.val.string.len = 12;
        } else if (token == WJB_KEY && item.val.string.len == 3 && !memcmp(item.val.string.val, "end", 3)) {
            item.val.string.val = (char *)"end_offset";
            item.val.string.len = 10;
        }
        pushJsonbValue(state, token, is_container(token) ? NULL : &item);
    }
}

/* Annotate's answer maps each question to its bare answer, null unless answered. */
static Jsonb *bare_answers(Jsonb *outcomes)
{
    JsonbParseState *state = NULL;
    JsonbIterator *iterator = JsonbIteratorInit(&outcomes->root);
    JsonbIteratorToken token;
    JsonbValue item;
    JsonbValue missing = {.type = jbvNull};
    JsonbValue *result = NULL;
    while ((token = JsonbIteratorNext(&iterator, &item, true)) != WJB_DONE) {
        if (token == WJB_VALUE) {
            Jsonb *child = item.type == jbvBinary ? JsonbValueToJsonb(&item) : NULL;
            JsonbValue *status = tt_field(child, "status");
            JsonbValue *answer = tt_field(child, "answer");
            bool answered = status != NULL && status->type == jbvString && status->val.string.len == 8
                && !memcmp(status->val.string.val, "answered", 8);
            push_answer(&state, answered && answer != NULL ? answer : &missing);
        } else result = pushJsonbValue(&state, token, is_container(token) ? NULL : &item);
    }
    return JsonbValueToJsonb(result);
}

static JsonbValue *binary(JsonbValue *slot, Jsonb *value)
{
    if (value == NULL) return NULL;
    slot->type = jbvBinary;
    slot->val.binary.data = &value->root;
    slot->val.binary.len = VARSIZE(value) - VARHDRSZ;
    return slot;
}

/* Keep the first failure for the statement-level errors tt_finish raises. */
static void record_outcome(TtOwner *owner, const thinkthen_row *row, Jsonb *error)
{
    const char *family = thinkthen_code_family(row->error_code);
    JsonbValue *message = tt_field(error, "message");
    owner->rows++;
    if (row->status != THINKTHEN_STATUS_FAILED) return;
    owner->failed++;
    if (family != NULL && !strcmp(family, "backend")) owner->backend_failed++;
    if (owner->failed > 1) return;
    owner->first_failed_code = row->error_code;
    owner->first_failed_position = row->position;
    owner->first_failed_message = message != NULL && message->type == jbvString
        ? MemoryContextStrdup(owner->id_context, pnstrdup(message->val.string.val, message->val.string.len))
        : MemoryContextStrdup(owner->id_context, thinkthen_code_name(row->error_code));
}

void tt_finish(TtOwner *owner)
{
    char *message;
    /* A dead backend raises its error, as the command exits 4, not a page of failures. */
    if (owner->rows > 0 && owner->backend_failed == owner->rows)
        tt_raise(owner->first_failed_code, owner->first_failed_message, NULL);
    if (owner->function != THINKTHEN_FUNCTION_FILTER || owner->failed == 0 || owner->keep_failures)
        return;
    message = psprintf("%llu of %llu records failed; the first, at position %llu, failed with %s: %s",
        (unsigned long long)owner->failed, (unsigned long long)owner->rows,
        (unsigned long long)owner->first_failed_position,
        thinkthen_code_name(owner->first_failed_code), owner->first_failed_message);
    tt_raise(owner->first_failed_code, message, "Pass failures => 'row' to return the failed rows with the kept rows.");
}

void tt_emit(TtOwner *owner, ReturnSetInfo *result)
{
    thinkthen_row *row = &owner->row;
    Jsonb *answer;
    Jsonb *error;
    Jsonb *source;
    Jsonb *detail;
    Jsonb *id;
    int code;
    JsonbValue scalar;
    Datum *values = palloc0(sizeof(Datum) * result->setDesc->natts);
    bool *nulls = palloc(sizeof(bool) * result->setDesc->natts);
    const char *status;
    bool answered;
    bool annotate = owner->function == THINKTHEN_FUNCTION_ANNOTATE;
    int column;
    if (owner->ids != NULL) {
        TtId *selected = hash_search(owner->ids, &row->position, HASH_FIND, NULL);
        if (selected != NULL) {
            TT_BRIDGE(code = thinkthen_sql_row_postgres_id(row, &selected->value, &owner->error));
            tt_check(owner, code);
        }
    }
    answer = tt_json(&row->answer);
    error = tt_json(&row->error);
    source = tt_json(&row->source);
    detail = tt_json(&row->detail);
    id = tt_json(&row->id);
    /* The id conversion above may turn an answered row into a failed one. */
    answered = row->status == THINKTHEN_STATUS_ANSWERED;
    status = answered ? "answered" :
        row->status == THINKTHEN_STATUS_UNSURE ? "unsure" : row->status == THINKTHEN_STATUS_FAILED ? "failed" : NULL;
    if (status == NULL || row->function != owner->function
        || (row->status == THINKTHEN_STATUS_FAILED && thinkthen_code_name(row->error_code) == NULL))
        ereport(ERROR, (errcode(tt_sqlstate(THINKTHEN_CODE_USAGE_ABI)), errmsg("%s", THINKTHEN_ABI_ERROR_MESSAGE), errdetail_internal("%s", THINKTHEN_ABI_ERROR_CODE)));
    record_outcome(owner, row, error);
    /* Filter returns kept rows, and failed rows only when the caller asks. */
    if (owner->function != THINKTHEN_FUNCTION_FILTER || (answered && row->answer_bool)
        || (row->status == THINKTHEN_STATUS_FAILED && owner->keep_failures)) {
        for (column = 0; column < result->setDesc->natts; column++) {
            Form_pg_attribute attribute = TupleDescAttr(result->setDesc, column);
            const char *name = NameStr(attribute->attname);
            JsonbValue *value = NULL;
            nulls[column] = false;
            if (!strcmp(name, "position")) values[column] = Int64GetDatum(row->position);
            else if (!strcmp(name, "probability")) {
                nulls[column] = isnan(row->probability);
                values[column] = Float8GetDatum(row->probability);
            } else if (!strcmp(name, "id")) {nulls[column] = id == NULL; if (id != NULL) values[column] = JsonbPGetDatum(id);}
            else if (!strcmp(name, "answer") && owner->function == THINKTHEN_FUNCTION_RANK) {
                nulls[column] = !answered;
                values[column] = Int64GetDatum(row->rank);
            } else if (!strcmp(name, "answer") && annotate) {
                nulls[column] = !answered || answer == NULL;
                if (!nulls[column]) values[column] = JsonbPGetDatum(bare_answers(answer));
            } else {
                if (!strcmp(name, "status")) {scalar.type = jbvString; scalar.val.string.val = (char *)status; scalar.val.string.len = strlen(status); value = &scalar;}
                else if (!strcmp(name, "source")) value = binary(&scalar, source);
                else if (!strcmp(name, "detail")) value = binary(&scalar, annotate && answered ? answer : detail);
                else if (!strcmp(name, "error_code")) value = tt_field(error, "code");
                else if (!strcmp(name, "error_message")) value = tt_field(error, "message");
                else if (!strcmp(name, "error_http_status")) value = tt_field(error, "http_status");
                else value = binary(&scalar, answer);
                values[column] = tt_json_value(value, attribute->atttypid, &nulls[column]);
            }
        }
        tuplestore_putvalues(result->setResult, result->setDesc, values, nulls);
    }
    if (owner->ids != NULL) {
        TtId *selected = hash_search(owner->ids, &row->position, HASH_FIND, NULL);
        if (selected != NULL) pfree((void *)selected->value.data);
        hash_search(owner->ids, &row->position, HASH_REMOVE, NULL);
    }
    TT_BRIDGE(thinkthen_row_free(row));
}
