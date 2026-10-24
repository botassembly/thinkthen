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
    size_t question_length;
    size_t options_length;
    char *question;
    char *options;
    int argument;
    for (argument = 0; argument < PG_NARGS(); argument++)
        if (PG_ARGISNULL(argument))
            ereport(ERROR, (errcode(ERRCODE_NULL_VALUE_NOT_ALLOWED), errmsg("ThinkThen arguments cannot be SQL NULL")));
    InitMaterializedSRF(fcinfo, MAT_SRF_USE_EXPECTED_DESC);
    invocation = AllocSetContextCreate(CurrentMemoryContext, "ThinkThen invocation", ALLOCSET_DEFAULT_SIZES);
    previous = MemoryContextSwitchTo(invocation);
    owner = palloc0(sizeof(*owner));
    owner->admitting = true;
    owner->cleanup.func = tt_release;
    owner->cleanup.arg = owner;
    MemoryContextRegisterResetCallback(invocation, &owner->cleanup);
    fetch_context = AllocSetContextCreate(invocation, "ThinkThen source chunk", ALLOCSET_DEFAULT_SIZES);
    row_context = AllocSetContextCreate(invocation, "ThinkThen output row", ALLOCSET_DEFAULT_SIZES);
    PG_TRY();
    {
        text *question_value = PG_GETARG_TEXT_PP(1);
        text *options_value = PG_GETARG_TEXT_PP(2);
        thinkthen_client *client;
        question = tt_utf8(VARDATA_ANY(question_value), VARSIZE_ANY_EXHDR(question_value), &question_length);
        options = tt_utf8(VARDATA_ANY(options_value), VARSIZE_ANY_EXHDR(options_value), &options_length);
        if (SPI_connect() != SPI_OK_CONNECT)
            elog(ERROR, "cannot open source SPI scope");
        owner->spi_connected = true;
        owner->source = tt_source(owner, PG_GETARG_TEXT_PP(0), PG_GETARG_TEXT_PP(3), &kind);
        client = tt_client(owner);
        TT_BRIDGE(code = thinkthen_sql_open(client, (uint8_t *)function, strlen(function),
            (uint8_t *)question, question_length, (uint8_t *)options, options_length,
            &owner->call, &owner->error));
        tt_check(owner, code);
        owner->admitting = false;
        for (;;) {
            CHECK_FOR_INTERRUPTS();
            TT_BRIDGE(code = thinkthen_sql_poll(owner->call, 20, interrupted, NULL, &owner->row, &owner->error));
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
