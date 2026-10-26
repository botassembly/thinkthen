#ifndef THINKTHEN_PG_INTERNAL_H
#define THINKTHEN_PG_INTERNAL_H

#include "postgres.h"
#include "fmgr.h"
#include "funcapi.h"
#include "executor/spi.h"
#include "nodes/params.h"
#include "utils/memutils.h"
#include <pthread.h>
#include <signal.h>
#include "thinkthen_sql.h"
#include "utils/hsearch.h"
#include "utils/jsonb.h"
typedef struct TtId { uint64 position; thinkthen_text value; MemoryContext memory; } TtId;

typedef struct TtOwner {
    thinkthen_sql_call *call;
    thinkthen_question *question;
    uint32 function;
    int record_column;
    int id_column;
    uint64 input_position;
    MemoryContext id_context;
    HTAB *ids;
    thinkthen_client *temporary_client;
    thinkthen_row row;
    thinkthen_error error;
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
int tt_sqlstate(int code);
thinkthen_client *tt_client(TtOwner *owner);
char *tt_utf8(const char *source, size_t length, size_t *converted_length);
text *tt_server_text(const char *source, size_t length);
ParamListInfo tt_parameters(Jsonb *parameters);
void tt_question(TtOwner *owner, FunctionCallInfo fcinfo, const char *function);
Datum tt_json_value(JsonbValue *value, Oid type, bool *is_null);
JsonbValue *tt_field(Jsonb *object, const char *name);
Jsonb *tt_json(const thinkthen_buffer *buffer);
Portal tt_source(TtOwner *owner, text *sql, Jsonb *parameters, const char *record_column, const char *id_column, uint32 *kind);
void tt_fetch(TtOwner *owner, uint32 kind, MemoryContext fetch_context);
void tt_emit(TtOwner *owner, ReturnSetInfo *result);

#endif
