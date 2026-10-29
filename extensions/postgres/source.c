#include "pg_internal.h"
#include "source_limits.h"
#include "catalog/pg_type_d.h"
#include "miscadmin.h"
#include "utils/builtins.h"
#include "utils/portal.h"
#include "utils/json.h"
#include "utils/lsyscache.h"
#include "nodes/plannodes.h"

ParamListInfo tt_parameters(Jsonb *parameters)
{
    Oid type = JSONBOID;
    Datum value = PointerGetDatum(parameters);
    ParamListInfo bindings;
    uint64 index;
    int result;
    if (!JB_ROOT_IS_ARRAY(parameters))
        ereport(ERROR, (errcode(ERRCODE_INVALID_PARAMETER_VALUE), errmsg("params must be a JSON array")));
    result = SPI_execute_with_args(
        "SELECT value OPERATOR(pg_catalog.#>>) '{}' AS value, pg_catalog.jsonb_typeof(value) AS type "
        "FROM pg_catalog.jsonb_array_elements($1) AS parameter(value)",
        1, &type, &value, NULL, true, 0);
    if (result != SPI_OK_SELECT)
        elog(ERROR, "cannot read source parameters: SPI result %d", result);
    bindings = makeParamList(SPI_processed);
    for (index = 0; index < SPI_processed; index++) {
        HeapTuple tuple = SPI_tuptable->vals[index];
        TupleDesc description = SPI_tuptable->tupdesc;
        char *kind = SPI_getvalue(tuple, description, 2);
        char *contents = SPI_getvalue(tuple, description, 1);
        ParamExternData *binding = &bindings->params[index];
        binding->pflags = PARAM_FLAG_CONST;
        binding->isnull = strcmp(kind, "null") == 0;
        if (strcmp(kind, "string") == 0 || binding->isnull) {
            binding->ptype = TEXTOID;
            if (!binding->isnull)
                binding->value = CStringGetTextDatum(contents);
        } else if (strcmp(kind, "boolean") == 0) {
            binding->ptype = BOOLOID;
            binding->value = BoolGetDatum(strcmp(contents, "true") == 0);
        } else if (strcmp(kind, "number") == 0) {
            binding->ptype = NUMERICOID;
            binding->value = DirectFunctionCall3(numeric_in, CStringGetDatum(contents),
                ObjectIdGetDatum(InvalidOid), Int32GetDatum(-1));
        } else {
            ereport(ERROR, (errcode(ERRCODE_INVALID_PARAMETER_VALUE),
                errmsg("source parameters must be JSON strings, numbers, booleans or null")));
        }
    }
    SPI_freetuptable(SPI_tuptable);
    return bindings;
}

Portal tt_source(TtOwner *owner, text *sql, Jsonb *parameters, const char *record_column, const char *id_column, uint32 *kind)
{
    SPIParseOpenOptions options = {0};
    Portal source;
    Oid column_type;
    options.params = tt_parameters(parameters);
    options.read_only = true;
    options.cursorOptions = CURSOR_OPT_NO_SCROLL;
    source = SPI_cursor_parse_open(NULL, text_to_cstring(sql), &options);
    owner->source = source;
    if (source->strategy != PORTAL_ONE_SELECT)
        ereport(ERROR, (errcode(ERRCODE_INVALID_PARAMETER_VALUE), errmsg("source must be one read-only SELECT")));
    {
        ListCell *statement;
        foreach(statement, source->stmts) {
            PlannedStmt *planned = lfirst_node(PlannedStmt, statement);
            if (planned->commandType != CMD_SELECT || planned->hasModifyingCTE)
                ereport(ERROR, (errcode(ERRCODE_INVALID_PARAMETER_VALUE), errmsg("source must be one read-only SELECT")));
        }
    }
    owner->record_column = SPI_fnumber(source->tupDesc, record_column);
    owner->id_column = id_column == NULL ? 0 : SPI_fnumber(source->tupDesc, id_column);
    if (owner->record_column <= 0 || owner->id_column < 0)
        ereport(ERROR, (errcode(ERRCODE_UNDEFINED_COLUMN), errmsg("selected source column does not exist")));
    column_type = TupleDescAttr(source->tupDesc, owner->record_column - 1)->atttypid;
    if (column_type != TEXTOID && column_type != JSONOID && column_type != JSONBOID)
        ereport(ERROR, (errcode(ERRCODE_DATATYPE_MISMATCH), errmsg("source record must have type text, json or jsonb")));
    *kind = column_type == TEXTOID ? THINKTHEN_RECORD_TEXT : THINKTHEN_RECORD_JSON;
    return source;
}

void tt_fetch(TtOwner *owner, uint32 kind, MemoryContext fetch_context)
{
    thinkthen_sql_input *records;
    uint64 index;
    uint64 count;
    int result;
    MemoryContext previous;
    SPI_cursor_fetch(owner->source, true, TT_SOURCE_CHUNK_RECORDS);
    count = SPI_processed;
    if (count == 0) {
        SPI_freetuptable(SPI_tuptable);
        TT_BRIDGE(thinkthen_sql_call_end_input(owner->call));
        return;
    }
    previous = MemoryContextSwitchTo(fetch_context);
    records = palloc0(sizeof(*records) * count);
    for (index = 0; index < count; index++) {
        bool is_null;
        Datum value = SPI_getbinval(SPI_tuptable->vals[index], SPI_tuptable->tupdesc, owner->record_column, &is_null);
        text *record;
        size_t length;
        CHECK_FOR_INTERRUPTS();
        owner->input_position++;
        if (owner->id_column > 0) {
            bool id_null;
            Oid type = TupleDescAttr(SPI_tuptable->tupdesc, owner->id_column - 1)->atttypid;
            Datum id_value = SPI_getbinval(SPI_tuptable->vals[index], SPI_tuptable->tupdesc, owner->id_column, &id_null);
            TtId *entry = hash_search(owner->ids, &owner->input_position, HASH_ENTER, NULL);
            char *encoded = "null";
            size_t id_length;
            if (!id_null) {
                Oid output;
                bool variable;
                getTypeOutputInfo(type, &output, &variable);
                encoded = OidOutputFunctionCall(output, id_value);
                if (type == BOOLOID) encoded = DatumGetBool(id_value) ? "true" : "false";
                else if (type != JSONOID && type != JSONBOID && type != NUMERICOID && type != INT2OID
                    && type != INT4OID && type != INT8OID && type != FLOAT4OID && type != FLOAT8OID) {
                    StringInfoData quoted;
                    initStringInfo(&quoted);
                    escape_json(&quoted, encoded);
                    encoded = quoted.data;
                }
            }
            encoded = tt_utf8(encoded, strlen(encoded), &id_length);
            entry->value.data = (uint8_t *)MemoryContextAlloc(owner->id_context, id_length);
            memcpy((void *)entry->value.data, encoded, id_length);
            entry->value.len = id_length;
        }
        if (is_null) {
            records[index].kind = THINKTHEN_RECORD_MISSING;
            continue;
        }
        if (TupleDescAttr(SPI_tuptable->tupdesc, owner->record_column - 1)->atttypid == JSONBOID)
            record = cstring_to_text(JsonbToCString(NULL, &DatumGetJsonbP(value)->root, 0));
        else record = DatumGetTextPP(value);
        records[index].kind = kind;
        records[index].data = (uint8_t *)tt_utf8(VARDATA_ANY(record), VARSIZE_ANY_EXHDR(record), &length);
        records[index].len = length;
    }
    TT_BRIDGE(result = thinkthen_sql_call_send_chunk_postgres(owner->call, records, count, &owner->error));
    MemoryContextSwitchTo(previous);
    tt_check(owner, result);
    SPI_freetuptable(SPI_tuptable);
    MemoryContextReset(fetch_context);
}
