#ifndef THINKTHEN_PG_INTERNAL_H
#define THINKTHEN_PG_INTERNAL_H

#include "postgres.h"
#include "fmgr.h"
#include "funcapi.h"
#include "executor/spi.h"
#include "nodes/params.h"
#include "utils/memutils.h"
#include "projection.h"
#include <pthread.h>
#include <signal.h>
#include "thinkthen.h"

typedef struct TtOwner {
    thinkthen_sql_call *call;
    thinkthen_client *temporary_client;
    thinkthen_row row;
    thinkthen_buffer error;
    thinkthen_buffer columns;
    TtJson *projection;
    TtJson *diagnostic;
    Portal source;
    bool spi_connected;
    bool admitting;
    MemoryContextCallback cleanup;
} TtOwner;

/* Store every returned owner before restoring signals. No PostgreSQL API or
 * throwing callback may run inside this boundary. New descendants inherit it. */
#define TT_BRIDGE(action) do { \
    sigset_t tt_saved; \
    tt_block_signals(&tt_saved); \
    action; \
    tt_restore_signals(&tt_saved); \
} while (0)

void tt_block_signals(sigset_t *saved);
void tt_restore_signals(const sigset_t *saved);
void tt_release(void *context);
void tt_check(TtOwner *owner, int result);
thinkthen_client *tt_client(TtOwner *owner);
char *tt_utf8(const char *source, size_t length, size_t *converted_length);
text *tt_server_text(const char *source, size_t length);
ParamListInfo tt_parameters(text *parameters);
Portal tt_source(TtOwner *owner, text *sql, text *parameters, uint32 *kind);
void tt_fetch(TtOwner *owner, uint32 kind, MemoryContext fetch_context);
void tt_emit(TtOwner *owner, ReturnSetInfo *result);

#endif
