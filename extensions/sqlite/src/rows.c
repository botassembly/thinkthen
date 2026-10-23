#include "internal.h"
#include <string.h>

int engine_error(thinkthen_buffer *error, char **diagnostic) {
    json_error_t failure;
    json_t *value = json_loadb((const char *)error->data, error->len, JSON_ALLOW_NUL, &failure);
    if (value == NULL) {
        thinkthen_buffer_free(error);
        return SQLITE_NOMEM;
    }
    const char *code = json_string_value(json_object_get(value, "code"));
    const char *message = json_string_value(json_object_get(value, "message"));
    *diagnostic = sqlite3_mprintf("%s: %s", code, message);
    json_decref(value);
    thinkthen_buffer_free(error);
    return *diagnostic == NULL ? SQLITE_NOMEM : SQLITE_ERROR;
}

void connection_release(void *pointer) {
    Connection *connection = pointer;
    if (--connection->references == 0) {
        thinkthen_client_free(connection->client);
        sqlite3_free(connection->settings);
        sqlite3_free(connection);
    }
}

int connection_client(Connection *connection, const char *settings, size_t len,
                      char **diagnostic) {
    if (connection->client != NULL && connection->settings_len == len &&
        memcmp(connection->settings, settings, len) == 0) return SQLITE_OK;
    thinkthen_client *client = NULL;
    thinkthen_buffer error = {0};
    if (thinkthen_sql_client_open((const uint8_t *)settings, len, &client, &error) != THINKTHEN_OK)
        return engine_error(&error, diagnostic);
    char *copy = sqlite3_malloc64(len + 1);
    if (copy == NULL) {
        thinkthen_client_free(client);
        return SQLITE_NOMEM;
    }
    memcpy(copy, settings, len);
    copy[len] = '\0';
    thinkthen_client_free(connection->client);
    sqlite3_free(connection->settings);
    connection->client = client;
    connection->settings = copy;
    connection->settings_len = len;
    return SQLITE_OK;
}

void cursor_clear(Cursor *cursor) {
    thinkthen_sql_free(cursor->call);
    cursor->call = NULL;
    if (cursor->source != NULL) sqlite3_finalize(cursor->source);
    cursor->source = NULL;
    thinkthen_row_free(&cursor->row);
    json_decref(cursor->fields);
    cursor->fields = NULL;
    for (int i = 0; i < 4; ++i) {
        sqlite3_value_free(cursor->arguments[i]);
        cursor->arguments[i] = NULL;
    }
    cursor->eof = 1;
}

int cursor_error(Cursor *cursor, int code, const char *message) {
    char *copy = sqlite3_mprintf("%s", message);
    cursor_clear(cursor);
    sqlite3_free(cursor->base.pVtab->zErrMsg);
    cursor->base.pVtab->zErrMsg = copy;
    return copy == NULL ? SQLITE_NOMEM : code;
}

static uint8_t interrupted(void *context) {
    return sqlite3_is_interrupted((sqlite3 *)context) != 0;
}

int cursor_advance(Cursor *cursor) {
    Table *table = (Table *)cursor->base.pVtab;
    thinkthen_row_free(&cursor->row);
    json_decref(cursor->fields);
    cursor->fields = NULL;
    for (;;) {
        if (sqlite3_is_interrupted(table->db))
            return cursor_error(cursor, SQLITE_INTERRUPT, "interrupted");
        thinkthen_buffer error = {0};
        int result = thinkthen_sql_poll(cursor->call, 10, interrupted, table->db,
                                       &cursor->row, &error);
        if (sqlite3_is_interrupted(table->db)) {
            thinkthen_buffer_free(&error);
            return cursor_error(cursor, SQLITE_INTERRUPT, "interrupted");
        }
        if (result == THINKTHEN_WAIT) continue;
        if (result == THINKTHEN_NEEDS_INPUT) {
            int code = source_chunk(cursor);
            if (code != SQLITE_OK) return code;
            continue;
        }
        if (result == THINKTHEN_ERROR) {
            char *message = NULL;
            int code = engine_error(&error, &message);
            if (code == SQLITE_NOMEM) return cursor_error(cursor, code, "out of memory");
            code = cursor_error(cursor, code, message);
            sqlite3_free(message);
            return code;
        }
        if (result == THINKTHEN_END) {
            cursor->eof = 1;
            thinkthen_sql_free(cursor->call);
            cursor->call = NULL;
            return SQLITE_OK;
        }
        thinkthen_buffer columns = {0};
        if (thinkthen_row_columns(&cursor->row, &columns, &error) != THINKTHEN_OK) {
            char *message = NULL;
            int code = engine_error(&error, &message);
            code = cursor_error(cursor, code, message == NULL ? "out of memory" : message);
            sqlite3_free(message);
            return code;
        }
        json_error_t failure;
        cursor->fields = json_loadb((const char *)columns.data, columns.len, JSON_ALLOW_NUL, &failure);
        thinkthen_buffer_free(&columns);
        if (cursor->fields == NULL)
            return cursor_error(cursor, SQLITE_NOMEM, "out of memory");
        cursor->eof = 0;
        ++cursor->rowid;
        return SQLITE_OK;
    }
}

static void result_raw_json(sqlite3_context *context, json_t *value) {
    if (value == NULL) { sqlite3_result_null(context); return; }
    sqlite3_result_text64(context, json_string_value(value), json_string_length(value),
                          SQLITE_TRANSIENT, SQLITE_UTF8);
}

int cursor_column(sqlite3_vtab_cursor *base, sqlite3_context *context, int column) {
    Cursor *cursor = (Cursor *)base;
    switch (column) {
    case 0: sqlite3_result_int64(context, (sqlite3_int64)cursor->row.position); break;
    case 1: {
        const char *status = json_string_value(json_object_get(cursor->fields, "status"));
        sqlite3_result_text(context, status, -1, SQLITE_TRANSIENT);
        break;
    }
    case 2: result_raw_json(context, json_object_get(cursor->fields, "answer")); break;
    case 3: {
        json_t *p = json_object_get(cursor->fields, "p");
        if (p == NULL) sqlite3_result_null(context);
        else sqlite3_result_double(context, json_number_value(p));
        break;
    }
    case 4: result_raw_json(context, json_object_get(cursor->fields, "error")); break;
    case 5:
        if (cursor->row.detail.data == NULL) sqlite3_result_null(context);
        else sqlite3_result_text64(context, (const char *)cursor->row.detail.data,
                                  cursor->row.detail.len, SQLITE_TRANSIENT, SQLITE_UTF8);
        break;
    case 6: sqlite3_result_int(context, cursor->row.kept); break;
    default:
        if (cursor->arguments[column - 7] != NULL)
            sqlite3_result_value(context, cursor->arguments[column - 7]);
        else sqlite3_result_text(context, column == 9 ? "{}" : "[]", -1, SQLITE_STATIC);
        break;
    }
    return SQLITE_OK;
}
