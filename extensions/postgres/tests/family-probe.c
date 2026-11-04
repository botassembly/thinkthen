/* Compile the exact private mapper without initializing a second SQL module. */
#define _PG_init thinkthen_probe_initializer
#include "../boundary.c"
#undef _PG_init
#include "../source_limits.h"

PG_FUNCTION_INFO_V1(thinkthen_family_probe);
PGDLLEXPORT Datum thinkthen_family_probe(PG_FUNCTION_ARGS);

Datum thinkthen_family_probe(PG_FUNCTION_ARGS)
{
    ereport(ERROR, (errcode(tt_sqlstate(PG_GETARG_INT32(0))),
                   errmsg("installed adapter family probe")));
    PG_RETURN_VOID();
}

PG_FUNCTION_INFO_V1(thinkthen_source_chunk_limit);
PGDLLEXPORT Datum thinkthen_source_chunk_limit(PG_FUNCTION_ARGS);
Datum thinkthen_source_chunk_limit(PG_FUNCTION_ARGS)
{
    (void)fcinfo;
    PG_RETURN_INT32(TT_SOURCE_CHUNK_RECORDS);
}

/* Compare native argument translation with the engine's canonical admission. */
PG_FUNCTION_INFO_V1(thinkthen_question_probe);
PGDLLEXPORT Datum thinkthen_question_probe(PG_FUNCTION_ARGS);
Datum thinkthen_question_probe(PG_FUNCTION_ARGS)
{
    TtOwner owner = {0};
    text *question = PG_GETARG_TEXT_PP(0);
    size_t length;
    char *encoded = tt_utf8(VARDATA_ANY(question), VARSIZE_ANY_EXHDR(question), &length);
    int code;
    owner.error.struct_size = sizeof(owner.error);
    TT_BRIDGE(code = thinkthen_sql_question_from_json(encoded, length, &owner.question, &owner.error));
    PG_TRY();
    {
        tt_check(&owner, code);
    }
    PG_CATCH();
    {
        TT_BRIDGE(thinkthen_error_free(&owner.error));
        PG_RE_THROW();
    }
    PG_END_TRY();
    TT_BRIDGE(thinkthen_question_free(owner.question));
    TT_BRIDGE(thinkthen_error_free(&owner.error));
    PG_RETURN_VOID();
}
