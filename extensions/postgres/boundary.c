#include "pg_internal.h"
#include "mb/pg_wchar.h"
#include "utils/builtins.h"
#include "utils/guc.h"
#include <errno.h>
#include "lib/stringinfo.h"
#include "utils/json.h"
#include <limits.h>

static char *model;
static int backend;
static int batch;
static int retries;
static int backoff;
static int timeout;
#include "registry.h"
static char *client_settings;
static char *allowed_backends;
static char *client_policy;
static thinkthen_client *connection_client;

int tt_sqlstate(int code)
{
    const char digits[] = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    unsigned number = code;
    return MAKE_SQLSTATE('T', 'T', digits[(number / 1296) % 36], digits[(number / 36) % 36], digits[number % 36]);
}

PG_MODULE_MAGIC;
PGDLLEXPORT void _PG_init(void);

void _PG_init(void)
{
    if (!thinkthen_abi_compatible(THINKTHEN_ABI_VERSION, thinkthen_abi_version()))
        ereport(ERROR, (errcode(tt_sqlstate(THINKTHEN_CODE_USAGE_ABI)), errmsg("%s", THINKTHEN_ABI_ERROR_MESSAGE), errdetail_internal("%s", THINKTHEN_ABI_ERROR_CODE)));
    DefineCustomEnumVariable("thinkthen.backend", "Selected backend.", NULL, &backend,
        TT_DEFAULT_BACKEND, backend_options, PGC_USERSET, 0, NULL, NULL, NULL);
    DefineCustomStringVariable("thinkthen.model", "Selected model.", NULL, &model, "", PGC_USERSET, 0, NULL, NULL, NULL);
    DefineCustomIntVariable("thinkthen.batch", "Batch count; zero uses the engine default.", NULL, &batch, 0, INT_MIN, INT_MAX, PGC_USERSET, 0, NULL, NULL, NULL);
    DefineCustomIntVariable("thinkthen.max_retries", "Retry count.", NULL, &retries, TT_DEFAULT_MAX_RETRIES, INT_MIN, INT_MAX, PGC_USERSET, 0, NULL, NULL, NULL);
    DefineCustomIntVariable("thinkthen.initial_backoff_ms", "Initial retry backoff.", NULL, &backoff, TT_DEFAULT_INITIAL_BACKOFF_MS, INT_MIN, INT_MAX, PGC_USERSET, 0, NULL, NULL, NULL);
    DefineCustomIntVariable("thinkthen.attempt_timeout_ms", "Attempt timeout.", NULL, &timeout, TT_DEFAULT_ATTEMPT_TIMEOUT_MS, INT_MIN, INT_MAX, PGC_USERSET, 0, NULL, NULL, NULL);
    DefineCustomStringVariable("thinkthen.allowed_backends", "Allowed backend:model pairs, separated by commas.",
        NULL, &allowed_backends, NULL, PGC_SUSET, 0, NULL, NULL, NULL);
    MarkGUCPrefixReserved("thinkthen");
}

void tt_block_signals(sigset_t *saved)
{
    sigset_t blocked;
    int result;
    sigfillset(&blocked);
    result = pthread_sigmask(SIG_BLOCK, &blocked, saved);
    if (result != 0)
        ereport(ERROR, (errcode(ERRCODE_SYSTEM_ERROR),
            errmsg("cannot block engine thread signals: %s", strerror(result))));
}

void tt_restore_signals(const sigset_t *saved)
{
    int result = pthread_sigmask(SIG_SETMASK, saved, NULL);
    if (result != 0)
        ereport(ERROR, (errcode(ERRCODE_SYSTEM_ERROR),
            errmsg("cannot restore backend signals: %s", strerror(result))));
}

void tt_release(void *context)
{
    TtOwner *owner = context;
    TT_BRIDGE({
        thinkthen_question_free(owner->question);
        owner->question = NULL;
        thinkthen_sql_call_free(owner->call);
        owner->call = NULL;
        thinkthen_client_free(owner->temporary_client);
        owner->temporary_client = NULL;
        thinkthen_row_free(&owner->row);
        thinkthen_error_free(&owner->error);
    });
}

char *tt_utf8(const char *source, size_t length, size_t *converted_length)
{
    char *value = pg_server_to_any(source, length, PG_UTF8);
    *converted_length = value == source ? length : strlen(value);
    return value;
}

text *tt_server_text(const char *source, size_t length)
{
    char *value = pg_any_to_server(source, length, PG_UTF8);
    text *result = cstring_to_text_with_len(value, value == source ? length : strlen(value));
    if (value != source)
        pfree(value);
    return result;
}

void tt_check(TtOwner *owner, int result)
{
    const char *code;
    text *converted;
    int state;
    if (result != THINKTHEN_ERROR) return;
    code = thinkthen_code_name(owner->error.code);
    if (code == NULL) {
        code = THINKTHEN_ABI_ERROR_CODE;
        ereport(ERROR, (errcode(tt_sqlstate(THINKTHEN_CODE_USAGE_ABI)), errmsg("%s", THINKTHEN_ABI_ERROR_MESSAGE), errdetail_internal("%s", code)));
    }
    state = tt_sqlstate(owner->error.code);
    if (owner->error.code == THINKTHEN_CODE_USAGE_THRESHOLD || owner->error.code == THINKTHEN_CODE_USAGE_QUESTION
        || owner->error.code == THINKTHEN_CODE_USAGE_NAME || owner->error.code == THINKTHEN_CODE_USAGE_POINTER
        || owner->error.code == THINKTHEN_CODE_USAGE_SETTINGS || owner->error.code == THINKTHEN_CODE_USAGE_BATCH
        || owner->error.code == THINKTHEN_CODE_USAGE_BACKEND || owner->error.code == THINKTHEN_CODE_USAGE_SQL_POLICY)
        state = ERRCODE_INVALID_PARAMETER_VALUE;
    if (owner->error.code == THINKTHEN_CODE_USAGE_SQL_SELECTION) state = ERRCODE_INSUFFICIENT_PRIVILEGE;
    if (owner->error.code == THINKTHEN_CODE_LOCAL_CANCELLED) state = ERRCODE_QUERY_CANCELED;
    converted = tt_server_text((const char *)owner->error.message.data, owner->error.message.len);
    ereport(ERROR, (errcode(state), errmsg_internal("%s", text_to_cstring(converted)), errdetail_internal("%s", code)));
}

thinkthen_client *tt_client(TtOwner *owner)
{
    StringInfoData settings_json;
    char *settings;
    char *encoded;
    char *snapshot;
    char *policy;
    char *policy_snapshot;
    size_t policy_length = 0;
    size_t length;
    int result;
    initStringInfo(&settings_json);
    appendStringInfoString(&settings_json, "{\"backend\":");
    escape_json(&settings_json, GetConfigOption("thinkthen.backend", false, false));
    if (model[0]) {appendStringInfoString(&settings_json, ",\"model\":"); escape_json(&settings_json, model);}
    if (batch) appendStringInfo(&settings_json, ",\"batch\":%d", batch);
    appendStringInfo(&settings_json, ",\"retry\":{\"max_retries\":%d,\"initial_backoff_ms\":%d,\"attempt_timeout_ms\":%d}}", retries, backoff, timeout);
    settings = settings_json.data;
    if (connection_client != NULL && strcmp(client_settings, settings) == 0
        && ((client_policy == NULL && allowed_backends == NULL)
            || (client_policy != NULL && allowed_backends != NULL
                && strcmp(client_policy, allowed_backends) == 0)))
        return connection_client;
    encoded = tt_utf8(settings, strlen(settings), &length);
    policy = allowed_backends == NULL ? NULL
        : tt_utf8(allowed_backends, strlen(allowed_backends), &policy_length);
    snapshot = MemoryContextStrdup(TopMemoryContext, settings);
    policy_snapshot = allowed_backends == NULL ? NULL
        : MemoryContextStrdup(TopMemoryContext, allowed_backends);
    TT_BRIDGE(result = thinkthen_sql_client_open_with_policy((uint8_t *)encoded, length,
        (uint8_t *)policy, policy_length,
        &owner->temporary_client, &owner->error));
    if (result == THINKTHEN_ERROR) {
        pfree(snapshot);
        if (policy_snapshot != NULL)
            pfree(policy_snapshot);
        tt_check(owner, result);
    }
    if (encoded != settings)
        pfree(encoded);
    if (policy != allowed_backends)
        pfree(policy);
    TT_BRIDGE({
        thinkthen_client_free(connection_client);
        connection_client = owner->temporary_client;
        owner->temporary_client = NULL;
    });
    if (client_settings != NULL)
        pfree(client_settings);
    client_settings = snapshot;
    if (client_policy != NULL)
        pfree(client_policy);
    client_policy = policy_snapshot;
    return connection_client;
}
