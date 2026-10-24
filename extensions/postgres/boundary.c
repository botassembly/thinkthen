#include "pg_internal.h"
#include "mb/pg_wchar.h"
#include "utils/builtins.h"
#include "utils/guc.h"
#include <errno.h>

static char *settings;
static char *client_settings;
static char *allowed_backends;
static char *client_policy;
static thinkthen_client *connection_client;

PG_MODULE_MAGIC;
PGDLLEXPORT void _PG_init(void);

void _PG_init(void)
{
    DefineCustomStringVariable("thinkthen.settings", "ThinkThen client settings JSON.",
        NULL, &settings, "{}", PGC_USERSET, 0, NULL, NULL, NULL);
    DefineCustomStringVariable("thinkthen.allowed_backends", "Allowed backend names and models JSON.",
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
        thinkthen_sql_free(owner->call);
        owner->call = NULL;
        thinkthen_client_free(owner->temporary_client);
        owner->temporary_client = NULL;
        thinkthen_row_free(&owner->row);
        thinkthen_buffer_free(&owner->error);
        thinkthen_buffer_free(&owner->columns);
    });
    tt_json_free(owner->projection);
    owner->projection = NULL;
    tt_json_free(owner->diagnostic);
    owner->diagnostic = NULL;
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
    const char *message;
    const TtJson *message_value;
    text *converted;
    int state;
    if (result != THINKTHEN_ERROR)
        return;
    owner->diagnostic = tt_json_parse((char *)owner->error.data, owner->error.len);
    if (owner->diagnostic == NULL)
        ereport(ERROR, (errcode(ERRCODE_OUT_OF_MEMORY), errmsg("cannot decode engine diagnostic")));
    code = tt_json_string(tt_json_get(owner->diagnostic, "code"));
    message_value = tt_json_get(owner->diagnostic, "message");
    message = tt_json_string(message_value);
    state = owner->admitting || strncmp(code, "usage.", 6) == 0
        ? ERRCODE_INVALID_PARAMETER_VALUE : ERRCODE_SYSTEM_ERROR;
    converted = tt_server_text(message, tt_json_length(message_value));
    ereport(ERROR, (errcode(state), errmsg_internal("%s: %s", code, text_to_cstring(converted))));
}

thinkthen_client *tt_client(TtOwner *owner)
{
    char *encoded;
    char *snapshot;
    char *policy;
    char *policy_snapshot;
    size_t policy_length = 0;
    size_t length;
    int result;
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
