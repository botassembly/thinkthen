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
