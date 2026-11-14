#include "internal.h"
#include <string.h>
#include <math.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>

int engine_error(thinkthen_error *error, char **diagnostic) {
    int result = error->code == THINKTHEN_CODE_LOCAL_CANCELLED ? SQLITE_INTERRUPT : SQLITE_ERROR;
    const char *code = thinkthen_code_name(error->code);
    *diagnostic = code == NULL
        ? sqlite3_mprintf("%s: %s", THINKTHEN_ABI_ERROR_CODE, THINKTHEN_ABI_ERROR_MESSAGE)
        : sqlite3_mprintf("%s: %.*s", code, (int)error->message.len, (const char *)error->message.data);
    thinkthen_error_free(error);
    return *diagnostic == NULL ? SQLITE_NOMEM : result;
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
    thinkthen_error error = { .struct_size = sizeof(error) };
    if (thinkthen_sql_client_open((const char *)settings, len, &client, &error) != THINKTHEN_OK)
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
    ++connection->generation;
    connection->settings = copy;
    connection->settings_len = len;
    return SQLITE_OK;
}

void cursor_clear(Cursor *cursor) {
    thinkthen_sql_call_free(cursor->call);
    cursor->call = NULL;
    if (cursor->source != NULL) sqlite3_finalize(cursor->source);
    cursor->source = NULL;
    if(!cursor->replaying)thinkthen_row_free(&cursor->row);
    else {memset(&cursor->row,0,sizeof(cursor->row));cursor->row.struct_size=sizeof(cursor->row);}
    json_decref(cursor->fields);
    cursor->fields = NULL;
    for (int i = 0; i < ARG_COUNT; ++i) {
        sqlite3_value_free(cursor->arguments[i]);
        cursor->arguments[i] = NULL;
    }
    json_decref(cursor->answer);cursor->answer=NULL;
    json_decref(cursor->error);cursor->error=NULL;
    sqlite3_finalize(cursor->read);cursor->read=NULL;
    sqlite3_finalize(cursor->projection);cursor->projection=NULL;
    cursor->projected_id=(thinkthen_buffer){0};
    sqlite3_finalize(cursor->save);cursor->save=NULL;
    sqlite3_finalize(cursor->input_save);cursor->input_save=NULL;
    sqlite3_finalize(cursor->input_read);cursor->input_read=NULL;
    cursor->input_count=0;
    sqlite3_close(cursor->replay);cursor->replay=NULL;
    cursor->complete=cursor->replaying=0;
    cursor->eof = 1;
}

int cursor_error(Cursor *cursor, int code, const char *message) {
    char *copy = sqlite3_mprintf("%s", message);
    cursor_clear(cursor);
    sqlite3_free(cursor->base.pVtab->zErrMsg);
    cursor->base.pVtab->zErrMsg = copy;
    return copy == NULL ? SQLITE_NOMEM : code;
}

/* The extension's own argument mistakes carry the shared usage.argument code. */
int usage_error(Cursor *cursor, const char *format, ...) {
    va_list arguments;
    va_start(arguments, format);
    char *detail = sqlite3_vmprintf(format, arguments);
    va_end(arguments);
    char *message = detail == NULL ? NULL : sqlite3_mprintf("usage.argument: %s", detail);
    sqlite3_free(detail);
    if (message == NULL) return cursor_error(cursor, SQLITE_NOMEM, "out of memory");
    int result = cursor_error(cursor, SQLITE_ERROR, message);
    sqlite3_free(message);
    return result;
}

static uint8_t interrupted(void *context) {
    return sqlite3_is_interrupted((sqlite3 *)context) != 0;
}

static int advance_one(Cursor *cursor) {
    Table *table = (Table *)cursor->base.pVtab;
    if(!cursor->replaying)thinkthen_row_free(&cursor->row);
    json_decref(cursor->fields);
    cursor->fields = NULL;
    json_decref(cursor->answer);cursor->answer=NULL;
    json_decref(cursor->error);cursor->error=NULL;
    if(cursor->replaying) {
        int result=replay_next(cursor);
        if(result!=SQLITE_OK)return result;
        if(!cursor->eof)goto decoded;
        return result;
    }
    for (;;) {
        if (sqlite3_is_interrupted(table->db))
            return cursor_error(cursor, SQLITE_INTERRUPT, "interrupted");
        thinkthen_error error = { .struct_size = sizeof(error) };
        int result = thinkthen_sql_call_poll(cursor->call, 10, interrupted, table->db,
                                       &cursor->row, &error);
        if (sqlite3_is_interrupted(table->db)) {
            thinkthen_error_free(&error);
            return cursor_error(cursor, SQLITE_INTERRUPT, "interrupted");
        }
        if (result == THINKTHEN_PENDING) continue;
        if (result == THINKTHEN_NEEDS_INPUT) {
            int code = source_chunk(cursor);
            if (code != SQLITE_OK) return code;
            continue;
        }
        if (result < 0) {
            char *message = NULL;
            int code = engine_error(&error, &message);
            if (code == SQLITE_NOMEM) return cursor_error(cursor, code, "out of memory");
            code = cursor_error(cursor, code, message);
            sqlite3_free(message);
            return code;
        }
        if (result == THINKTHEN_END) {
            cursor->complete = 1;
            cursor->eof = 1;
            thinkthen_sql_call_free(cursor->call);
            cursor->call = NULL;
            return SQLITE_OK;
        }
        int saved=replay_save(cursor);if(saved!=SQLITE_OK)return saved;
        ++cursor->rowid;
        decoded:
        if (cursor->row.error_code && thinkthen_code_name(cursor->row.error_code) == NULL)
            return cursor_error(cursor, SQLITE_ERROR, THINKTHEN_ABI_ERROR_CODE ": " THINKTHEN_ABI_ERROR_MESSAGE);
        cursor->answer=json_incref(json_object_get(cursor->fields,"answer"));
        cursor->error=json_incref(json_object_get(cursor->fields,"error"));
        cursor->eof = 0;
        return SQLITE_OK;
    }
}

static int filtering(Cursor *cursor) {
    return strcmp(((Table *)cursor->base.pVtab)->registration->function, "filter") == 0;
}

/* The shared driver owns failure accounting. Only keep selection is host-specific. */
int cursor_advance(Cursor *cursor) {
    for (;;) {
        int result=advance_one(cursor);
        if(result!=SQLITE_OK || cursor->eof || !filtering(cursor))return result;
        if(cursor->row.status==THINKTHEN_STATUS_ANSWERED && json_is_true(cursor->answer))return SQLITE_OK;
    }
}

static void result_buffer(sqlite3_context *context, const thinkthen_buffer *value) {
    if (value->data == NULL) { sqlite3_result_null(context); return; }
    sqlite3_result_text64(context, (const char *)value->data, value->len,
                          SQLITE_TRANSIENT, SQLITE_UTF8);
}

/* JSON1 marks JSON text with this subtype, so json_object nests it unquoted. */
enum { JSON_SUBTYPE = 74 };

/* An id keeps its JSON type: a string as TEXT, an integer as INTEGER. Only an
 * integer outside SQLite's signed 64-bit range stays TEXT digits. */
static void result_id(sqlite3_context *context, const thinkthen_buffer *value) {
    if (value->data == NULL) { sqlite3_result_null(context); return; }
    json_t *id = json_loadb((const char *)value->data, value->len, JSON_DECODE_ANY | JSON_ALLOW_NUL, NULL);
    if (json_is_string(id))
        sqlite3_result_text64(context, json_string_value(id), json_string_length(id), SQLITE_TRANSIENT, SQLITE_UTF8);
    else if (json_is_integer(id)) sqlite3_result_int64(context, json_integer_value(id));
    else if (json_is_null(id)) sqlite3_result_null(context);
    else result_buffer(context, value);
    json_decref(id);
}

static void emit(sqlite3_str *out, json_t *value);

static void emit_string(sqlite3_str *out, const char *text, size_t length) {
    json_t *string = json_stringn_nocheck(text, length);
    char *encoded = string ? json_dumps(string, JSON_ENCODE_ANY) : NULL;
    if (encoded) sqlite3_str_appendall(out, encoded);
    else sqlite3_str_reset(out);
    free(encoded);
    json_decref(string);
}

static void emit_member(sqlite3_str *out, int *first, const char *key, json_t *value) {
    if (!*first) sqlite3_str_appendchar(out, 1, ',');
    *first = 0;
    emit_string(out, key, strlen(key));
    sqlite3_str_appendchar(out, 1, ':');
    emit(out, value);
}

/* Spans use the shared SQL fields text, kind, start_offset and end_offset,
 * because end is a SQL keyword. Other members keep the engine's order. */
static void emit_object(sqlite3_str *out, json_t *object) {
    json_t *start = json_object_get(object, "start"), *end = json_object_get(object, "end");
    int span = json_is_integer(start) && json_is_integer(end), first = 1;
    sqlite3_str_appendchar(out, 1, '{');
    if (span) {
        static const char *const leading[] = {"kind", "text"};
        for (size_t n = 0; n < 2; n++)
            if (json_object_get(object, leading[n])) emit_member(out, &first, leading[n], json_object_get(object, leading[n]));
        emit_member(out, &first, "start_offset", start);
        emit_member(out, &first, "end_offset", end);
    }
    const char *key; json_t *member;
    json_object_foreach(object, key, member) {
        if (span && (!strcmp(key, "text") || !strcmp(key, "kind") || !strcmp(key, "start") || !strcmp(key, "end"))) continue;
        emit_member(out, &first, key, member);
    }
    sqlite3_str_appendchar(out, 1, '}');
}

/* Re-encode with the shortest digits that round-trip, as the engine wrote them. */
static void emit_real(sqlite3_str *out, double number) {
    char digits[32];
    for (int precision = 15; precision <= 17; precision++) {
        snprintf(digits, sizeof(digits), "%.*g", precision, number);
        if (strtod(digits, NULL) == number) break;
    }
    sqlite3_str_appendall(out, digits);
    if (!strpbrk(digits, ".eEn")) sqlite3_str_appendall(out, ".0");
}

static void emit(sqlite3_str *out, json_t *value) {
    size_t index; json_t *item;
    switch (json_typeof(value)) {
    case JSON_OBJECT: emit_object(out, value); break;
    case JSON_ARRAY:
        sqlite3_str_appendchar(out, 1, '[');
        json_array_foreach(value, index, item) {
            if (index) sqlite3_str_appendchar(out, 1, ',');
            emit(out, item);
        }
        sqlite3_str_appendchar(out, 1, ']');
        break;
    case JSON_STRING: emit_string(out, json_string_value(value), json_string_length(value)); break;
    case JSON_INTEGER: sqlite3_str_appendf(out, "%lld", (long long)json_integer_value(value)); break;
    case JSON_REAL: emit_real(out, json_real_value(value)); break;
    case JSON_TRUE: sqlite3_str_appendall(out, "true"); break;
    case JSON_FALSE: sqlite3_str_appendall(out, "false"); break;
    case JSON_NULL: sqlite3_str_appendall(out, "null"); break;
    }
}

static void result_json(sqlite3_context *context, json_t *value) {
    sqlite3_str *out = sqlite3_str_new(sqlite3_context_db_handle(context));
    emit(out, value);
    int length = sqlite3_str_length(out);
    char *encoded = sqlite3_str_finish(out);
    if (encoded == NULL || length == 0) { sqlite3_free(encoded); sqlite3_result_error_nomem(context); return; }
    sqlite3_result_text(context, encoded, length, sqlite3_free);
    sqlite3_result_subtype(context, JSON_SUBTYPE);
}

static void json_value(sqlite3_context *context,json_t *value) {
    if(!value || json_is_null(value))sqlite3_result_null(context);
    else if(json_is_string(value))sqlite3_result_text64(context,json_string_value(value),json_string_length(value),SQLITE_TRANSIENT,SQLITE_UTF8);
    else if(json_is_boolean(value))sqlite3_result_int(context,json_is_true(value));
    else if(json_is_integer(value))sqlite3_result_int64(context,json_integer_value(value));
    else if(json_is_real(value))sqlite3_result_double(context,json_real_value(value));
    else result_json(context,value);
}

/* Annotate's answer holds each question's bare answer, NULL unless answered. */
static void annotate_answer(sqlite3_context *context, json_t *outcomes) {
    json_t *answers = json_object();
    const char *name; json_t *outcome;
    json_object_foreach(outcomes, name, outcome) {
        json_t *status = json_object_get(outcome, "status"), *answer = json_object_get(outcome, "answer");
        int answered = json_is_string(status) && !strcmp(json_string_value(status), "answered") && answer;
        json_object_set(answers, name, answered ? answer : json_null());
    }
    result_json(context, answers);
    json_decref(answers);
}

static int is_annotate(Cursor *cursor) {
    return cursor->row.function == THINKTHEN_FUNCTION_ANNOTATE;
}

int cursor_column(sqlite3_vtab_cursor *base,sqlite3_context *context,int column) {
    Cursor *cursor=(Cursor*)base;
    if(column>=COL_COUNT) {
        Table *table=(Table*)base->pVtab;
        sqlite3_value *argument=cursor->arguments[table->arguments[column-COL_COUNT]];
        if(argument)sqlite3_result_value(context,argument);else sqlite3_result_null(context);
        return SQLITE_OK;
    }
    switch(column) {
    case COL_POSITION:sqlite3_result_int64(context,(sqlite3_int64)cursor->row.position);break;
    case COL_STATUS: {
        const char *status=cursor->row.status==THINKTHEN_STATUS_ANSWERED?"answered":cursor->row.status==THINKTHEN_STATUS_UNSURE?"unsure":cursor->row.status==THINKTHEN_STATUS_FAILED?"failed":NULL;
        if(!status)sqlite3_result_error(context,THINKTHEN_ABI_ERROR_CODE ": " THINKTHEN_ABI_ERROR_MESSAGE,-1);else sqlite3_result_text(context,status,-1,SQLITE_STATIC);break;
    }
    case COL_ANSWER:
        if(is_annotate(cursor) && json_is_object(cursor->answer))annotate_answer(context,cursor->answer);
        else json_value(context,cursor->answer);
        break;
    case COL_PROBABILITY:if(isnan(cursor->row.probability))sqlite3_result_null(context);else sqlite3_result_double(context,cursor->row.probability);break;
    case COL_CODE:json_value(context,json_object_get(cursor->error,"code"));break;
    case COL_MESSAGE:json_value(context,json_object_get(cursor->error,"message"));break;
    case COL_HTTP:json_value(context,json_object_get(cursor->error,"http_status"));break;
    case COL_DETAIL: {
        /* Annotate's detail holds each question's full outcome. */
        json_t *detail=is_annotate(cursor)?cursor->answer:json_object_get(cursor->fields,"detail");
        if(detail)result_json(context,detail);else sqlite3_result_null(context);
        break;
    }
    case COL_SOURCE:result_json(context,json_object_get(cursor->fields,"source"));break;
    case COL_RECORD: {
        sqlite3_value *record=cursor->replaying ? sqlite3_column_value(cursor->read,3) : row_input(cursor,1);
        if(record)sqlite3_result_value(context,record);else sqlite3_result_null(context);
        break;
    }
    case COL_ID: {
        if(cursor->replaying) {
            if(cursor->id_column>=0)sqlite3_result_value(context,sqlite3_column_value(cursor->read,1));
            else result_id(context,&cursor->projected_id);
            break;
        }
        sqlite3_value *id=row_id(cursor);
        if(cursor->id_column>=0){if(id)sqlite3_result_value(context,id);else sqlite3_result_null(context);}
        else result_id(context,&cursor->projected_id);
        break;
    }
    }
    return SQLITE_OK;
}
