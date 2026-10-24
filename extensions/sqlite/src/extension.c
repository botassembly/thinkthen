#include "internal.h"
#include <string.h>

SQLITE_EXTENSION_INIT1

static int connect_table(sqlite3 *db, void *aux, int argc, const char *const *argv,
                         sqlite3_vtab **output, char **error) {
    (void)argc; (void)argv; (void)error;
    int result = sqlite3_vtab_config(db, SQLITE_VTAB_DIRECTONLY);
    if (result != SQLITE_OK) return result;
    result = sqlite3_declare_vtab(db,
        "CREATE TABLE x(position INTEGER,status TEXT,answer TEXT,p REAL,error TEXT,detail TEXT,kept INTEGER,"
        "source_sql HIDDEN,question HIDDEN,options HIDDEN,parameters HIDDEN)");
    if (result != SQLITE_OK) return result;
    Table *table = sqlite3_malloc64(sizeof(*table));
    if (table == NULL) return SQLITE_NOMEM;
    memset(table, 0, sizeof(*table));
    table->db = db;
    table->registration = aux;
    *output = &table->base;
    return SQLITE_OK;
}

static int best_index(sqlite3_vtab *table, sqlite3_index_info *index) {
    int argument = 1;
    int mask = 0;
    for (int column = 0; column < 4; ++column) {
        int found = -1;
        int present = 0;
        for (int n = 0; n < index->nConstraint; ++n) {
            if (index->aConstraint[n].iColumn != column + 7 ||
                index->aConstraint[n].op != SQLITE_INDEX_CONSTRAINT_EQ) continue;
            present = 1;
            if (index->aConstraint[n].usable) { found = n; break; }
        }
        if (found < 0) {
            if (present) return SQLITE_CONSTRAINT;
            if (column >= 2) continue;
            sqlite3_free(table->zErrMsg);
            table->zErrMsg = sqlite3_mprintf("source_sql and question are required");
            return table->zErrMsg == NULL ? SQLITE_NOMEM : SQLITE_ERROR;
        }
        index->aConstraintUsage[found].argvIndex = argument++;
        index->aConstraintUsage[found].omit = 1;
        mask |= 1 << column;
    }
    index->idxNum = mask;
    index->estimatedCost = 1000000.0;
    index->estimatedRows = 1000;
    return SQLITE_OK;
}

static int disconnect_table(sqlite3_vtab *table) { sqlite3_free(table); return SQLITE_OK; }

static int open_cursor(sqlite3_vtab *table, sqlite3_vtab_cursor **output) {
    Cursor *cursor = sqlite3_malloc64(sizeof(*cursor));
    if (cursor == NULL) return SQLITE_NOMEM;
    memset(cursor, 0, sizeof(*cursor));
    cursor->base.pVtab = table;
    cursor->eof = 1;
    *output = &cursor->base;
    return SQLITE_OK;
}

static int close_cursor(sqlite3_vtab_cursor *base) {
    cursor_clear((Cursor *)base);
    sqlite3_free(base);
    return SQLITE_OK;
}

static int filter_cursor(sqlite3_vtab_cursor *base, int mask, const char *plan,
                         int argc, sqlite3_value **argv) {
    (void)plan; (void)argc;
    Cursor *cursor = (Cursor *)base;
    Table *table = (Table *)base->pVtab;
    cursor_clear(cursor);
    cursor->rowid = 0;
    int argument = 0;
    for (int n = 0; n < 4; ++n) {
        if (!(mask & (1 << n))) continue;
        if (sqlite3_value_type(argv[argument]) != SQLITE_TEXT)
            return cursor_error(cursor, SQLITE_MISMATCH, "table function arguments must be SQL TEXT");
        cursor->arguments[n] = sqlite3_value_dup(argv[argument++]);
        if (cursor->arguments[n] == NULL)
            return cursor_error(cursor, SQLITE_NOMEM, "out of memory");
    }
    int result = source_open(cursor);
    if (result != SQLITE_OK) return result;
    Connection *connection = table->registration->connection;
    char *message = NULL;
    if (connection->client == NULL) {
        result = connection_client(connection, "{}", 2, &message);
        if (result != SQLITE_OK) {
            result = cursor_error(cursor, result, message == NULL ? "out of memory" : message);
            sqlite3_free(message);
            return result;
        }
    }
    const char *question = (const char *)sqlite3_value_text(cursor->arguments[1]);
    const char *options = cursor->arguments[2] == NULL ? "{}" :
        (const char *)sqlite3_value_text(cursor->arguments[2]);
    if (question == NULL || options == NULL)
        return cursor_error(cursor, SQLITE_NOMEM, "out of memory");
    size_t options_len = cursor->arguments[2] == NULL ? 2 :
        (size_t)sqlite3_value_bytes(cursor->arguments[2]);
    const char *function = table->registration->function;
    thinkthen_buffer error = {0};
    result = thinkthen_sql_open(connection->client, (const uint8_t *)function, strlen(function),
        (const uint8_t *)question, (size_t)sqlite3_value_bytes(cursor->arguments[1]),
        (const uint8_t *)options, options_len, &cursor->call, &error);
    if (result != THINKTHEN_OK) {
        result = engine_error(&error, &message);
        result = cursor_error(cursor, result, message == NULL ? "out of memory" : message);
        sqlite3_free(message);
        return result;
    }
    return cursor_advance(cursor);
}

static int next_cursor(sqlite3_vtab_cursor *base) { return cursor_advance((Cursor *)base); }
static int eof_cursor(sqlite3_vtab_cursor *base) { return ((Cursor *)base)->eof; }
static int rowid_cursor(sqlite3_vtab_cursor *base, sqlite3_int64 *rowid) {
    *rowid = ((Cursor *)base)->rowid;
    return SQLITE_OK;
}

static const sqlite3_module module = {
    .iVersion = 1, .xConnect = connect_table, .xBestIndex = best_index,
    .xDisconnect = disconnect_table, .xOpen = open_cursor, .xClose = close_cursor,
    .xFilter = filter_cursor, .xNext = next_cursor, .xEof = eof_cursor,
    .xColumn = cursor_column, .xRowid = rowid_cursor
};

static void registration_free(void *pointer) {
    Registration *registration = pointer;
    connection_release(registration->connection);
    sqlite3_free(registration);
}

static void configure(sqlite3_context *context, int argc, sqlite3_value **argv) {
    (void)argc;
    if (sqlite3_value_type(argv[0]) != SQLITE_TEXT) {
        sqlite3_result_error(context, "configuration must be SQL TEXT", -1);
        sqlite3_result_error_code(context, SQLITE_MISMATCH);
        return;
    }
    const char *settings = (const char *)sqlite3_value_text(argv[0]);
    if (settings == NULL) { sqlite3_result_error_nomem(context); return; }
    char *message = NULL;
    int result = connection_client(sqlite3_user_data(context), settings,
        (size_t)sqlite3_value_bytes(argv[0]), &message);
    if (result == SQLITE_NOMEM) sqlite3_result_error_nomem(context);
    else if (result != SQLITE_OK) {
        sqlite3_result_error(context, message, -1);
        sqlite3_result_error_code(context, result);
    } else sqlite3_result_int(context, 1);
    sqlite3_free(message);
}

#ifdef _WIN32
__declspec(dllexport)
#else
__attribute__((visibility("default")))
#endif
int sqlite3_thinkthensqlite_init(sqlite3 *db, char **error, const sqlite3_api_routines *api) {
    SQLITE_EXTENSION_INIT2(api);
    if (sqlite3_libversion_number() < 3041000) {
        *error = sqlite3_mprintf("ThinkThen requires SQLite 3.41.0 or newer");
        return *error == NULL ? SQLITE_NOMEM : SQLITE_ERROR;
    }
    static const char *const functions[] = {
        "thinkthen_decide", "thinkthen_choose", "thinkthen_tag", "thinkthen_score", "thinkthen_filter",
        "thinkthen_rank", "thinkthen_find", "thinkthen_annotate", "thinkthen_recognize", "thinkthen_relate"
    };
    Connection *connection = sqlite3_malloc64(sizeof(*connection));
    if (connection == NULL) return SQLITE_NOMEM;
    memset(connection, 0, sizeof(*connection));
    connection->references = 1;
    size_t count = sizeof(functions) / sizeof(functions[0]);
    size_t installed = 0;
    int result = SQLITE_OK;
    for (; installed < count; ++installed) {
        Registration *registration = sqlite3_malloc64(sizeof(*registration));
        if (registration == NULL) { result = SQLITE_NOMEM; break; }
        registration->connection = connection;
        registration->function = functions[installed] + sizeof("thinkthen_") - 1;
        ++connection->references;
        result = sqlite3_create_module_v2(db, functions[installed], &module, registration, registration_free);
        if (result != SQLITE_OK) break;
    }
    if (result == SQLITE_OK) {
        ++connection->references;
        result = sqlite3_create_function_v2(db, "thinkthen_configure", 1,
            SQLITE_UTF8 | SQLITE_DIRECTONLY, connection, configure, NULL, NULL, connection_release);
    }
    if (result != SQLITE_OK) {
        *error = sqlite3_mprintf("%s", sqlite3_errmsg(db));
        for (size_t n = 0; n < installed; ++n) {
            sqlite3_create_module_v2(db, functions[n], NULL, NULL, NULL);
        }
        connection_release(connection);
        return *error == NULL ? SQLITE_NOMEM : result;
    }
    connection_release(connection);
    return SQLITE_OK_LOAD_PERMANENTLY;
}
