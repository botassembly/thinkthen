#ifndef THINKTHEN_SQLITE_INTERNAL_H
#define THINKTHEN_SQLITE_INTERNAL_H

#include <sqlite3ext.h>
SQLITE_EXTENSION_INIT3
#include <jansson.h>
#include <thinkthen_sql.h>

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

enum { ARG_SOURCE, ARG_QUESTION, ARG_CHOICES, ARG_LABELS, ARG_LEVELS, ARG_KINDS,
    ARG_RULES, ARG_TOOLBOX, ARG_THRESHOLD, ARG_LOW, ARG_HIGH, ARG_TOP,
    ARG_EVIDENCE, ARG_PARAMS, ARG_RECORD, ARG_ID, ARG_FORMAT, ARG_NAME, ARG_COUNT };
enum { COL_POSITION, COL_STATUS, COL_ANSWER, COL_PROBABILITY, COL_CODE, COL_MESSAGE,
    COL_HTTP, COL_DETAIL, COL_ID, COL_SOURCE, COL_KEPT, COL_RANK, COL_TEXT, COL_START, COL_END };

typedef struct {
    sqlite3_vtab base;
    sqlite3 *db;
    Registration *registration;
    int columns[15], column_count;
    int arguments[ARG_COUNT], argument_count;
} Table;

typedef struct {
    sqlite3_vtab_cursor base;
    sqlite3_stmt *source;
    thinkthen_sql_call *call;
    thinkthen_row row;
    json_t *fields;
    sqlite3_value *arguments[ARG_COUNT];
    sqlite3_value **ids;
    size_t ids_count, ids_capacity, ids_start;
    uint64_t ids_offset;
    int record_column, id_column, json;
    json_t *answer, *error;
    sqlite3_int64 rowid;
    int eof;
} Cursor;

void connection_release(void *pointer);
int connection_client(Connection *connection, const char *settings, size_t len,
                      char **diagnostic);
int engine_error(thinkthen_error *error, char **diagnostic);
void cursor_clear(Cursor *cursor);
int cursor_error(Cursor *cursor, int code, const char *message);
int question_open(Cursor *cursor);
int source_open(Cursor *cursor);
int source_chunk(Cursor *cursor);
int cursor_advance(Cursor *cursor);
int cursor_column(sqlite3_vtab_cursor *base, sqlite3_context *context, int column);

#endif
