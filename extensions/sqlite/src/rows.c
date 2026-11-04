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

void filter_reset(Cursor *cursor) {
    cursor->failed = cursor->first_failed_position = 0;
    sqlite3_free(cursor->first_failed_code);
    sqlite3_free(cursor->first_failed_message);
    cursor->first_failed_code = cursor->first_failed_message = NULL;
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
    cursor->keep_failures=0;
    filter_reset(cursor);
    for(size_t n=0;n<cursor->ids_count;n++)sqlite3_value_free(cursor->ids[n]);
    sqlite3_free(cursor->ids);cursor->ids=NULL;cursor->ids_count=cursor->ids_capacity=cursor->ids_start=0;cursor->ids_offset=0;
    sqlite3_finalize(cursor->read);cursor->read=NULL;
    sqlite3_finalize(cursor->save);cursor->save=NULL;
    sqlite3_finalize(cursor->id_save);cursor->id_save=NULL;
    sqlite3_finalize(cursor->id_read);cursor->id_read=NULL;
    cursor->spool_ids=0;cursor->spooled_ids=0;
    sqlite3_close(cursor->replay);cursor->replay=NULL;
    cursor->complete=cursor->replaying=cursor->draining=cursor->released=0;cursor->held=0;
    sqlite3_free(cursor->held_error);cursor->held_error=NULL;
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

/* Release the live row's native id once the row is emitted or spooled; rank's
 * ids live in the spool instead. */
static void release_id(Cursor *cursor) {
    if(cursor->row.position && cursor->row.function!=THINKTHEN_FUNCTION_RANK && cursor->row.position>cursor->ids_offset && cursor->row.position-cursor->ids_offset<=cursor->ids_count) {
        size_t previous=(size_t)(cursor->row.position-cursor->ids_offset)-1;
        sqlite3_value_free(cursor->ids[previous]);cursor->ids[previous]=NULL;cursor->ids_start=previous+1;
    }
}

/* Hold a backend failure until some record is answered or fails otherwise.
 * Returns 1 when the live row was held in the spool, 0 when it is emitted. */
static int hold_backend_failure(Cursor *cursor) {
    if(cursor->released)return 0;
    const char *family=cursor->row.status==THINKTHEN_STATUS_FAILED ? thinkthen_code_family(cursor->row.error_code) : NULL;
    if(family && !strcmp(family,"backend")) {
        if(cursor->held++==0) {
            json_t *error=cursor->row.error.data ? json_loadb((const char*)cursor->row.error.data,cursor->row.error.len,JSON_DECODE_ANY|JSON_ALLOW_NUL,NULL) : NULL;
            const char *text=json_string_value(json_object_get(error,"message"));
            cursor->held_error=sqlite3_mprintf("%s: %s",thinkthen_code_name(cursor->row.error_code),text?text:"");
            json_decref(error);
        }
        release_id(cursor);
        thinkthen_row_free(&cursor->row);
        return 1;
    }
    cursor->released=1;
    return 0;
}

/* Replay the held rows and the releasing row from the spool, in order. */
static int drain_held(Cursor *cursor) {
    release_id(cursor);
    thinkthen_row_free(&cursor->row);
    if(sqlite3_prepare_v2(cursor->replay,"SELECT * FROM rows WHERE rowid<=? ORDER BY rowid",-1,&cursor->read,NULL)!=SQLITE_OK
       || sqlite3_bind_int64(cursor->read,1,(sqlite3_int64)cursor->held+1)!=SQLITE_OK)
        return cursor_error(cursor,SQLITE_ERROR,"cannot read result spool");
    cursor->held=0;
    cursor->replaying=cursor->draining=1;
    return SQLITE_OK;
}

static int advance_one(Cursor *cursor) {
    Table *table = (Table *)cursor->base.pVtab;
    if(!cursor->replaying)release_id(cursor);
    if(!cursor->replaying)thinkthen_row_free(&cursor->row);
    json_decref(cursor->fields);
    cursor->fields = NULL;
    json_decref(cursor->answer);cursor->answer=NULL;
    json_decref(cursor->error);cursor->error=NULL;
    if(cursor->replaying) {
        int result=replay_next(cursor);
        if(result!=SQLITE_OK)return result;
        if(!cursor->eof)goto decoded;
        if(!cursor->draining)return result;
        /* The held rows are out; resume the live call. */
        sqlite3_finalize(cursor->read);cursor->read=NULL;
        cursor->replaying=cursor->draining=0;cursor->eof=0;
        memset(&cursor->row,0,sizeof(cursor->row));cursor->row.struct_size=sizeof(cursor->row);
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
            if (!cursor->released && cursor->held) {
                /* No record was answered and the backend failed every one. */
                char *message = cursor->held_error;
                cursor->held_error = NULL;
                result = cursor_error(cursor, message ? SQLITE_ERROR : SQLITE_NOMEM, message ? message : "out of memory");
                sqlite3_free(message);
                return result;
            }
            cursor->complete = 1;
            cursor->eof = 1;
            thinkthen_sql_call_free(cursor->call);
            cursor->call = NULL;
            return SQLITE_OK;
        }
        int saved=replay_save(cursor);if(saved!=SQLITE_OK)return saved;
        if (hold_backend_failure(cursor)) continue;
        if (cursor->held) {
            int drained = drain_held(cursor);
            if (drained != SQLITE_OK) return drained;
            int replayed = replay_next(cursor);
            if (replayed != SQLITE_OK) return replayed;
            goto decoded;
        }
        ++cursor->rowid;
        decoded:
        if (cursor->row.error_code && thinkthen_code_name(cursor->row.error_code) == NULL)
            return cursor_error(cursor, SQLITE_ERROR, THINKTHEN_ABI_ERROR_CODE ": " THINKTHEN_ABI_ERROR_MESSAGE);
        if(cursor->row.answer.data)cursor->answer=json_loadb((const char*)cursor->row.answer.data,cursor->row.answer.len,JSON_DECODE_ANY|JSON_ALLOW_NUL,NULL);
        if(cursor->row.error.data)cursor->error=json_loadb((const char*)cursor->row.error.data,cursor->row.error.len,JSON_DECODE_ANY|JSON_ALLOW_NUL,NULL);
        cursor->eof = 0;
        return SQLITE_OK;
    }
}

static int filtering(Cursor *cursor) {
    return strcmp(((Table *)cursor->base.pVtab)->registration->function, "filter") == 0;
}

/* Filter returns answered-yes rows. Failed rows are returned with failures = 'row';
 * otherwise they are counted and one error follows the last kept row. */
int cursor_advance(Cursor *cursor) {
    for (;;) {
        int result = advance_one(cursor);
        if (result != SQLITE_OK || !filtering(cursor)) return result;
        if (cursor->eof) {
            if (cursor->failed == 0 || cursor->keep_failures) return SQLITE_OK;
            char *message = sqlite3_mprintf("%s: %llu of the filter's records failed, the first at position %llu (%s); "
                "add failures = 'row' to return failed rows", cursor->first_failed_code,
                (unsigned long long)cursor->failed, (unsigned long long)cursor->first_failed_position,
                cursor->first_failed_message);
            if (message == NULL) return cursor_error(cursor, SQLITE_NOMEM, "out of memory");
            result = cursor_error(cursor, SQLITE_ERROR, message);
            sqlite3_free(message);
            return result;
        }
        if (cursor->row.status == THINKTHEN_STATUS_ANSWERED && cursor->row.answer_bool) return SQLITE_OK;
        if (cursor->row.status != THINKTHEN_STATUS_FAILED) continue;
        if (cursor->keep_failures) return SQLITE_OK;
        if (cursor->failed++ == 0) {
            const char *code = json_string_value(json_object_get(cursor->error, "code"));
            const char *text = json_string_value(json_object_get(cursor->error, "message"));
            cursor->first_failed_code = sqlite3_mprintf("%s", code ? code : "");
            cursor->first_failed_message = sqlite3_mprintf("%s", text ? text : "");
            if (!cursor->first_failed_code || !cursor->first_failed_message)
                return cursor_error(cursor, SQLITE_NOMEM, "out of memory");
            cursor->first_failed_position = cursor->row.position;
        }
    }
}

static void result_buffer(sqlite3_context *context, const thinkthen_buffer *value) {
    if (value->data == NULL) { sqlite3_result_null(context); return; }
    sqlite3_result_text64(context, (const char *)value->data, value->len,
                          SQLITE_TRANSIENT, SQLITE_UTF8);
}

/* JSON1 marks JSON text with this subtype, so json_object nests it unquoted. */
enum { JSON_SUBTYPE = 74 };

static void result_json_buffer(sqlite3_context *context, const thinkthen_buffer *value) {
    result_buffer(context, value);
    if (value->data != NULL) sqlite3_result_subtype(context, JSON_SUBTYPE);
}

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
        static const char *const leading[] = {"text", "kind"};
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
        json_t *detail=is_annotate(cursor) ? json_incref(cursor->answer) : cursor->row.detail.data
            ? json_loadb((const char*)cursor->row.detail.data,cursor->row.detail.len,JSON_DECODE_ANY|JSON_ALLOW_NUL,NULL) : NULL;
        if(detail)result_json(context,detail);else sqlite3_result_null(context);
        json_decref(detail);
        break;
    }
    case COL_SOURCE:result_json_buffer(context,&cursor->row.source);break;
    case COL_ID: {
        if(cursor->replaying) {
            if(sqlite3_column_type(cursor->read,1)!=SQLITE_NULL)sqlite3_result_value(context,sqlite3_column_value(cursor->read,1));
            else result_id(context,&cursor->row.id);
            break;
        }
        sqlite3_value *id=row_id(cursor);
        if(id)sqlite3_result_value(context,id);
        else result_id(context,&cursor->row.id);
        break;
    }
    }
    return SQLITE_OK;
}
