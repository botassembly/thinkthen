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
            JsonbValue *field = tt_field(object, NameStr(attribute->attname));
            if (field == NULL && !strcmp(NameStr(attribute->attname), "either")) {
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
    status = row->status == THINKTHEN_STATUS_ANSWERED ? "answered" :
        row->status == THINKTHEN_STATUS_UNSURE ? "unsure" : row->status == THINKTHEN_STATUS_FAILED ? "failed" : NULL;
    if (status == NULL || row->function != owner->function
        || (row->status == THINKTHEN_STATUS_FAILED && thinkthen_code_name(row->error_code) == NULL))
        ereport(ERROR, (errcode(tt_sqlstate(THINKTHEN_CODE_USAGE_ABI)), errmsg("%s", THINKTHEN_ABI_ERROR_MESSAGE), errdetail_internal("%s", THINKTHEN_ABI_ERROR_CODE)));
    for (column = 0; column < result->setDesc->natts; column++) {
        Form_pg_attribute attribute = TupleDescAttr(result->setDesc, column);
        const char *name = NameStr(attribute->attname);
        JsonbValue *value = NULL;
        nulls[column] = false;
        if (!strcmp(name, "position")) values[column] = Int64GetDatum(row->position);
        else if (!strcmp(name, "probability")) {
            nulls[column] = isnan(row->probability);
            values[column] = Float8GetDatum(row->probability);
        } else if (!strcmp(name, "rank")) {nulls[column] = row->status != THINKTHEN_STATUS_ANSWERED; values[column] = Int64GetDatum(row->rank);}
        else if (!strcmp(name, "id")) {nulls[column] = id == NULL; if (id != NULL) values[column] = JsonbPGetDatum(id);}
        else if (!strcmp(name, "kept")) values[column] = BoolGetDatum(row->status == THINKTHEN_STATUS_ANSWERED && row->answer_bool);
        else {
            if (!strcmp(name, "status")) {scalar.type = jbvString; scalar.val.string.val = (char *)status; scalar.val.string.len = strlen(status); value = &scalar;}
            else if (!strcmp(name, "source")) {if (source) {scalar.type = jbvBinary; scalar.val.binary.data = &source->root; scalar.val.binary.len = VARSIZE(source) - VARHDRSZ; value = &scalar;}}
            else if (!strcmp(name, "detail")) {if (detail) {scalar.type = jbvBinary; scalar.val.binary.data = &detail->root; scalar.val.binary.len = VARSIZE(detail) - VARHDRSZ; value = &scalar;}}
            else if (!strcmp(name, "error_code")) value = tt_field(error, "code");
            else if (!strcmp(name, "error_message")) value = tt_field(error, "message");
            else if (!strcmp(name, "error_http_status")) value = tt_field(error, "http_status");
            else if (!strncmp(name, "found_", 6)) value = tt_field(answer, name + 6);
            else if (answer) {scalar.type = jbvBinary; scalar.val.binary.data = &answer->root; scalar.val.binary.len = VARSIZE(answer) - VARHDRSZ; value = &scalar;}
            values[column] = tt_json_value(value, attribute->atttypid, &nulls[column]);
        }
    }
    tuplestore_putvalues(result->setResult, result->setDesc, values, nulls);
    if (owner->ids != NULL) {
        TtId *selected = hash_search(owner->ids, &row->position, HASH_FIND, NULL);
        if (selected != NULL) pfree((void *)selected->value.data);
        hash_search(owner->ids, &row->position, HASH_REMOVE, NULL);
    }
    TT_BRIDGE(thinkthen_row_free(row));
}
