#include "pg_internal.h"
#include "catalog/pg_type_d.h"
#include "miscadmin.h"
#include "utils/builtins.h"
#include "utils/portal.h"

ParamListInfo tt_parameters(text *parameters)
{
    Oid type = JSONOID;
    Datum value = PointerGetDatum(parameters);
    ParamListInfo bindings;
    uint64 index;
    int result = SPI_execute_with_args(
        "SELECT value OPERATOR(pg_catalog.#>>) '{}' AS value, pg_catalog.json_typeof(value) AS type "
        "FROM pg_catalog.json_array_elements($1) AS parameter(value)",
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

Portal tt_source(TtOwner *owner, text *sql, text *parameters, uint32 *kind)
{
    SPIParseOpenOptions options = {0};
    Portal source;
    Oid column_type;
    options.params = tt_parameters(parameters);
    options.read_only = true;
    options.cursorOptions = CURSOR_OPT_NO_SCROLL;
    source = SPI_cursor_parse_open(NULL, text_to_cstring(sql), &options);
    owner->source = source;
    if (source->strategy != PORTAL_ONE_SELECT || source->tupDesc->natts != 1 ||
        strcmp(NameStr(TupleDescAttr(source->tupDesc, 0)->attname), "record") != 0)
        ereport(ERROR, (errcode(ERRCODE_INVALID_PARAMETER_VALUE),
            errmsg("source must be one read-only SELECT with one column named record")));
    column_type = TupleDescAttr(source->tupDesc, 0)->atttypid;
    if (column_type != TEXTOID && column_type != JSONOID)
        ereport(ERROR, (errcode(ERRCODE_DATATYPE_MISMATCH),
            errmsg("source record must have type text or json")));
    *kind = column_type == TEXTOID ? THINKTHEN_SQL_TEXT : THINKTHEN_SQL_JSON;
    return source;
}

void tt_fetch(TtOwner *owner, uint32 kind, MemoryContext fetch_context)
{
    thinkthen_sql_record *records;
    uint64 index;
    uint64 count;
    int result;
    MemoryContext previous;
    SPI_cursor_fetch(owner->source, true, 128);
    count = SPI_processed;
    if (count == 0) {
        SPI_freetuptable(SPI_tuptable);
        TT_BRIDGE(thinkthen_sql_end_input(owner->call));
        return;
    }
    previous = MemoryContextSwitchTo(fetch_context);
    records = palloc0(sizeof(*records) * count);
    for (index = 0; index < count; index++) {
        bool is_null;
        Datum value = SPI_getbinval(SPI_tuptable->vals[index], SPI_tuptable->tupdesc, 1, &is_null);
        text *record;
        size_t length;
        CHECK_FOR_INTERRUPTS();
        if (is_null)
            ereport(ERROR, (errcode(ERRCODE_NULL_VALUE_NOT_ALLOWED), errmsg("source record cannot be SQL NULL")));
        record = DatumGetTextPP(value);
        records[index].kind = kind;
        records[index].data = (uint8_t *)tt_utf8(VARDATA_ANY(record), VARSIZE_ANY_EXHDR(record), &length);
        records[index].len = length;
    }
    TT_BRIDGE(result = thinkthen_sql_send_chunk(owner->call, records, count, &owner->error));
    MemoryContextSwitchTo(previous);
    tt_check(owner, result);
    SPI_freetuptable(SPI_tuptable);
    MemoryContextReset(fetch_context);
}
