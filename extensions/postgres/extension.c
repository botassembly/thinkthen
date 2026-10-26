#include "pg_internal.h"
#include "miscadmin.h"
#include "utils/builtins.h"

static uint8_t interrupted(void *context)
{
    (void)context;
    return QueryCancelPending || ProcDiePending;
}

static Datum run(FunctionCallInfo fcinfo, const char *function)
{
    ReturnSetInfo *result = (ReturnSetInfo *)fcinfo->resultinfo;
    MemoryContext invocation;
    MemoryContext fetch_context;
    MemoryContext row_context;
    MemoryContext previous;
    TtOwner *owner;
    uint32 kind;
    int code;
    int last = PG_NARGS();
    if (PG_ARGISNULL(0) || PG_ARGISNULL(last - 3) || PG_ARGISNULL(last - 2))
        ereport(ERROR, (errcode(ERRCODE_INVALID_PARAMETER_VALUE), errmsg("source_sql, params and record_column cannot be SQL NULL")));
    InitMaterializedSRF(fcinfo, MAT_SRF_USE_EXPECTED_DESC);
    invocation = AllocSetContextCreate(CurrentMemoryContext, "ThinkThen invocation", ALLOCSET_DEFAULT_SIZES);
    previous = MemoryContextSwitchTo(invocation);
    owner = palloc0(sizeof(*owner));
    owner->admitting = true;
    owner->id_context = invocation;
    {
        HASHCTL control = {0};
        control.keysize = sizeof(uint64);
        control.entrysize = sizeof(TtId);
        control.hcxt = invocation;
        owner->ids = hash_create("ThinkThen selected ids", 128, &control, HASH_ELEM | HASH_BLOBS | HASH_CONTEXT);
    }
    owner->row.struct_size = sizeof(owner->row);
    owner->error.struct_size = sizeof(owner->error);
    owner->cleanup.func = tt_release;
    owner->cleanup.arg = owner;
    MemoryContextRegisterResetCallback(invocation, &owner->cleanup);
    fetch_context = AllocSetContextCreate(invocation, "ThinkThen source chunk", ALLOCSET_DEFAULT_SIZES);
    row_context = AllocSetContextCreate(invocation, "ThinkThen output row", ALLOCSET_DEFAULT_SIZES);
    PG_TRY();
    {
        thinkthen_client *client;
        thinkthen_text name = {NULL, 0};
        tt_question(owner, fcinfo, function);
        if (SPI_connect() != SPI_OK_CONNECT)
            elog(ERROR, "cannot open source SPI scope");
        owner->spi_connected = true;
        owner->source = tt_source(owner, PG_GETARG_TEXT_PP(0), PG_GETARG_JSONB_P(last - 3),
            NameStr(*PG_GETARG_NAME(last - 2)), PG_ARGISNULL(last - 1) ? NULL : NameStr(*PG_GETARG_NAME(last - 1)), &kind);
        client = tt_client(owner);
        if (!PG_ARGISNULL(last - 4)) {
            text *value = PG_GETARG_TEXT_PP(last - 4);
            size_t length;
            name.data = (uint8_t *)tt_utf8(VARDATA_ANY(value), VARSIZE_ANY_EXHDR(value), &length);
            name.len = length;
        }
        TT_BRIDGE(code = thinkthen_sql_call_open_question(client, owner->question,
            PG_ARGISNULL(last - 4) ? NULL : &name, &owner->call, &owner->error));
        tt_check(owner, code);
        owner->admitting = false;
        for (;;) {
            CHECK_FOR_INTERRUPTS();
            TT_BRIDGE(code = thinkthen_sql_call_poll_postgres(owner->call, 20, interrupted, NULL, &owner->row, &owner->error));
            CHECK_FOR_INTERRUPTS();
            tt_check(owner, code);
            if (code == THINKTHEN_END)
                break;
            if (code == THINKTHEN_NEEDS_INPUT)
                tt_fetch(owner, kind, fetch_context);
            else if (code == THINKTHEN_OK) {
                MemoryContext output_previous = MemoryContextSwitchTo(row_context);
                tt_emit(owner, result);
                MemoryContextSwitchTo(output_previous);
                MemoryContextReset(row_context);
            }
        }
        SPI_cursor_close(owner->source);
        owner->source = NULL;
        if (SPI_finish() != SPI_OK_FINISH)
            elog(ERROR, "cannot close source SPI scope");
        owner->spi_connected = false;
        tt_release(owner);
    }
    PG_CATCH();
    {
        MemoryContextSwitchTo(previous);
        tt_release(owner);
        if (owner->source != NULL) {
            SPI_cursor_close(owner->source);
            owner->source = NULL;
        }
        if (owner->spi_connected)
            (void)SPI_finish(); /* Preserve the original PostgreSQL error. */
        /* SPI_finish restores the invocation context saved by SPI_connect. */
        MemoryContextSwitchTo(previous);
        MemoryContextDelete(invocation);
        PG_RE_THROW();
    }
    PG_END_TRY();
    MemoryContextSwitchTo(previous);
    MemoryContextDelete(invocation);
    return (Datum)0;
}

#define TT_FUNCTION(name) \
    PG_FUNCTION_INFO_V1(thinkthen_##name); \
    PGDLLEXPORT Datum thinkthen_##name(PG_FUNCTION_ARGS); \
    Datum thinkthen_##name(PG_FUNCTION_ARGS) { return run(fcinfo, #name); }
TT_FUNCTION(decide)
TT_FUNCTION(choose)
TT_FUNCTION(tag)
TT_FUNCTION(score)
TT_FUNCTION(filter)
TT_FUNCTION(rank)
TT_FUNCTION(find)
TT_FUNCTION(annotate)
TT_FUNCTION(recognize)
TT_FUNCTION(relate)
