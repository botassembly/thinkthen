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

/* PostgreSQL's own classes, so retry logic can tell our bug from a down service. */
int tt_sqlstate(int code)
{
    const char *family = thinkthen_code_family(code);
    if (code == THINKTHEN_CODE_USAGE_SQL_SELECTION) return ERRCODE_INSUFFICIENT_PRIVILEGE;
    if (code == THINKTHEN_CODE_LOCAL_CANCELLED) return ERRCODE_QUERY_CANCELED;
    if (code == THINKTHEN_CODE_BACKEND_TRANSPORT) return ERRCODE_SQLCLIENT_UNABLE_TO_ESTABLISH_SQLCONNECTION;
    if (family == NULL || strcmp(family, "usage") == 0) return ERRCODE_INVALID_PARAMETER_VALUE;
    if (strcmp(family, "record") == 0) return ERRCODE_DATA_EXCEPTION;
    if (strcmp(family, "internal") == 0) return ERRCODE_INTERNAL_ERROR;
    return ERRCODE_SYSTEM_ERROR;
}

/* The fix for errors whose remedy is a PostgreSQL setting or the server. */
static const char *hint(int code)
{
    switch (code) {
    case THINKTHEN_CODE_USAGE_SQL_SELECTION:
        return "An administrator allows a backend and model with thinkthen.allowed_backends, for example SET thinkthen.allowed_backends = 'stub:*'.";
    case THINKTHEN_CODE_USAGE_SQL_POLICY:
        return "List backend:model entries separated by commas, for example 'systemone:*,stub:*'.";
    case THINKTHEN_CODE_USAGE_KEY_MISSING:
        return "Set the backend's key in the database server's environment, then restart the server.";
    case THINKTHEN_CODE_BACKEND_TRANSPORT:
        return "Check that the database server can reach the backend.";
    default:
        return NULL;
    }
}

/* Raise an engine error; the message is already in the server encoding. */
void tt_raise(int code, const char *message, const char *extra_hint)
{
    const char *name = thinkthen_code_name(code);
    const char *fix = extra_hint != NULL ? extra_hint : hint(code);
    ereport(ERROR, (errcode(tt_sqlstate(code)), errmsg_internal("%s", message),
        errdetail_internal("%s", name == NULL ? THINKTHEN_ABI_ERROR_CODE : name),
        fix == NULL ? 0 : errhint("%s", fix)));
}

static bool tt_check_batch(int *value, void **extra, GucSource source)
{
    thinkthen_settings *settings = NULL;
    thinkthen_error error = {.struct_size = sizeof(error)};
    int code;
    const char *name;
    (void)extra;
    (void)source;
    /* Zero, the default SHOW reports, leaves the engine to choose. */
    if (*value == 0)
        return true;
    TT_BRIDGE({
        code = thinkthen_settings_new(&settings, &error);
        if (code == THINKTHEN_OK)
            code = thinkthen_settings_set_batch(settings, (uint64_t)*value, &error);
        thinkthen_settings_free(settings);
    });
    if (code == THINKTHEN_OK)
        return true;
    name = thinkthen_code_name(error.code);
    GUC_check_errcode(tt_sqlstate(error.code));
    GUC_check_errmsg("%.*s", (int)error.message.len, (const char *)error.message.data);
    GUC_check_errdetail("%s", name == NULL ? THINKTHEN_ABI_ERROR_CODE : name);
    TT_BRIDGE(thinkthen_error_free(&error));
    return false;
}

/* A malformed policy fails at SET, not at the first call that reads it. */
static bool tt_check_policy(char **value, void **extra, GucSource source)
{
    static const char probe[] = "{\"backend\":\"stub\"}";
    thinkthen_client *client = NULL;
    thinkthen_error error = {.struct_size = sizeof(error)};
    size_t length;
    char *policy;
    int code;
    (void)extra;
    (void)source;
    if (*value == NULL)
        return true;
    policy = tt_utf8(*value, strlen(*value), &length);
    TT_BRIDGE({
        code = thinkthen_sql_client_open_with_policy(probe, sizeof(probe) - 1,
            policy, length, &client, &error);
        thinkthen_client_free(client);
    });
    if (policy != *value)
        pfree(policy);
    if (code >= 0 || error.code != THINKTHEN_CODE_USAGE_SQL_POLICY) {
        TT_BRIDGE(thinkthen_error_free(&error));
        return true;
    }
    GUC_check_errcode(tt_sqlstate(error.code));
    GUC_check_errmsg("%.*s", (int)error.message.len, (const char *)error.message.data);
    GUC_check_errdetail("%s", thinkthen_code_name(error.code));
    GUC_check_errhint("%s", hint(error.code));
    TT_BRIDGE(thinkthen_error_free(&error));
    return false;
}

PG_MODULE_MAGIC;
PGDLLEXPORT void _PG_init(void);

void _PG_init(void)
{
    if (!thinkthen_abi_compatible(THINKTHEN_ABI_VERSION, thinkthen_abi_version()))
        ereport(ERROR, (errcode(tt_sqlstate(THINKTHEN_CODE_USAGE_ABI)), errmsg("%s", THINKTHEN_ABI_ERROR_MESSAGE), errdetail_internal("%s", THINKTHEN_ABI_ERROR_CODE)));
    TT_REGISTER_SETTINGS
    DefineCustomStringVariable("thinkthen.allowed_backends", "Allowed backend:model pairs, separated by commas.",
        NULL, &allowed_backends, NULL, PGC_SUSET, 0, tt_check_policy, NULL, NULL);
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
    if (result >= 0) return;
    if (thinkthen_code_name(owner->error.code) == NULL)
        ereport(ERROR, (errcode(tt_sqlstate(THINKTHEN_CODE_USAGE_ABI)), errmsg("%s", THINKTHEN_ABI_ERROR_MESSAGE), errdetail_internal("%s", THINKTHEN_ABI_ERROR_CODE)));
    tt_raise(owner->error.code, text_to_cstring(tt_server_text((const char *)owner->error.message.data,
        owner->error.message.len)), NULL);
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
    appendStringInfo(&settings_json, "{\"retry\":{\"max_retries\":%d,\"initial_backoff_ms\":%d,\"attempt_timeout_ms\":%d}", retries, backoff, timeout);
    if (backend) {
        appendStringInfoString(&settings_json, ",\"backend\":");
        escape_json(&settings_json, GetConfigOption("thinkthen.backend", false, false));
    }
    if (model[0]) {appendStringInfoString(&settings_json, ",\"model\":"); escape_json(&settings_json, model);}
    if (batch) appendStringInfo(&settings_json, ",\"batch\":%d", batch);
    appendStringInfoChar(&settings_json, '}');
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
    TT_BRIDGE(result = thinkthen_sql_client_open_with_policy((const char *)encoded, length,
        (const char *)policy, policy_length,
        &owner->temporary_client, &owner->error));
    if (result < 0) {
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
