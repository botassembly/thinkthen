#ifndef THINKTHEN_SQLITE_INTERNAL_H
#define THINKTHEN_SQLITE_INTERNAL_H

#include <sqlite3ext.h>
SQLITE_EXTENSION_INIT3
#include <jansson.h>
#include <thinkthen_sql.h>

typedef struct {
    unsigned references;
    unsigned generation;
    thinkthen_client *client;
    char *settings;
    size_t settings_len;
} Connection;

typedef struct {
    Connection *connection;
    const char *function;
} Registration;

enum { ARG_SOURCE, ARG_QUESTION, ARG_CHOICES, ARG_LABELS, ARG_LEVELS, ARG_KINDS,
    ARG_RULES, ARG_QUESTIONS, ARG_THRESHOLD, ARG_LOW, ARG_HIGH, ARG_TOP,
    ARG_FIELD, ARG_PARAMS, ARG_RECORD, ARG_ID, ARG_FORMAT, ARG_NAME,
    ARG_YES_MEANS, ARG_NO_MEANS, ARG_DESCRIPTIONS, ARG_RAISE, ARG_COUNT };
/* The shared SQL row shape, in the order DuckDB and PostgreSQL also use. */
enum { COL_POSITION, COL_ID, COL_RECORD, COL_ANSWER, COL_PROBABILITY, COL_STATUS, COL_CODE, COL_MESSAGE,
    COL_HTTP, COL_SOURCE, COL_DETAIL, COL_COUNT };
extern const char *const tt_columns[COL_COUNT];

typedef struct {
    sqlite3_vtab base;
    sqlite3 *db;
    Registration *registration;
    int arguments[ARG_COUNT], argument_count;
} Table;

typedef struct {
    sqlite3_vtab_cursor base;
    sqlite3_stmt *source;
    sqlite3 *replay;
    sqlite3_stmt *save, *read, *projection;
    thinkthen_buffer projected_id;
    int complete, replaying, mask;
    sqlite3_int64 changes, data_version;
    unsigned generation;
    thinkthen_sql_call *call;
    thinkthen_row row;
    json_t *fields;
    sqlite3_value *arguments[ARG_COUNT];
    /* Exact SQLite input values wait in the cursor's disk-backed spool. */
    sqlite3_stmt *input_save, *input_read;
    sqlite3_int64 input_count;
    int record_column, id_column, json;
    json_t *answer, *error;
    sqlite3_int64 rowid;
    int eof;
} Cursor;

int replay_open(Cursor *cursor);
int replay_save(Cursor *cursor);
int replay_next(Cursor *cursor);
int row_projection(Cursor *cursor,const char *json,sqlite3_uint64 length);
int replay_same(Cursor *cursor,int mask,sqlite3_value **argv);
int replay_save_input(Cursor *cursor,sqlite3_value *id,sqlite3_value *record);
sqlite3_value *row_input(Cursor *cursor,int column);
sqlite3_value *row_id(Cursor *cursor);

void connection_release(void *pointer);
int connection_client(Connection *connection, const char *settings, size_t len,
                      char **diagnostic);
int engine_error(thinkthen_error *error, char **diagnostic);
int usage_error(Cursor *cursor, const char *format, ...);
void cursor_clear(Cursor *cursor);
int cursor_error(Cursor *cursor, int code, const char *message);
int question_open(Cursor *cursor);
int source_open(Cursor *cursor);
int source_chunk(Cursor *cursor);
int cursor_advance(Cursor *cursor);
int cursor_column(sqlite3_vtab_cursor *base, sqlite3_context *context, int column);

#endif
