#ifndef THINKTHEN_SQLITE_INTERNAL_H
#define THINKTHEN_SQLITE_INTERNAL_H

#include <sqlite3ext.h>
SQLITE_EXTENSION_INIT3
#include <jansson.h>
#include <thinkthen.h>

typedef struct {
    unsigned references;
    thinkthen_client *client;
    char *settings;
    size_t settings_len;
} Connection;

typedef struct {
    Connection *connection;
    const char *function;
} Registration;

typedef struct {
    sqlite3_vtab base;
    sqlite3 *db;
    Registration *registration;
} Table;

typedef struct {
    sqlite3_vtab_cursor base;
    sqlite3_stmt *source;
    thinkthen_sql_call *call;
    thinkthen_row row;
    json_t *fields;
    sqlite3_value *arguments[4];
    sqlite3_int64 rowid;
    int eof;
} Cursor;

void connection_release(void *pointer);
int connection_client(Connection *connection, const char *settings, size_t len,
                      char **diagnostic);
int engine_error(thinkthen_buffer *error, char **diagnostic);
void cursor_clear(Cursor *cursor);
int cursor_error(Cursor *cursor, int code, const char *message);
int source_open(Cursor *cursor);
int source_chunk(Cursor *cursor);
int cursor_advance(Cursor *cursor);
int cursor_column(sqlite3_vtab_cursor *base, sqlite3_context *context, int column);

#endif
