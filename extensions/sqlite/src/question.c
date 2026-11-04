#include "internal.h"
#include <string.h>
#include <stdlib.h>

static const char *text(Cursor *cursor,int argument) {
    return cursor->arguments[argument] ? (const char*)sqlite3_value_text(cursor->arguments[argument]) : "";
}
static size_t length(Cursor *cursor,int argument) {
    return cursor->arguments[argument] ? (size_t)sqlite3_value_bytes(cursor->arguments[argument]) : 0;
}
static thinkthen_function function_id(const char *name) {
#define TT_FUNCTION(name, upper) if(!strcmp(name_text,#name))return THINKTHEN_FUNCTION_##upper
    const char *name_text=name;
    TT_FUNCTION(decide,DECIDE);TT_FUNCTION(choose,CHOOSE);TT_FUNCTION(tag,TAG);TT_FUNCTION(score,SCORE);
    TT_FUNCTION(filter,FILTER);TT_FUNCTION(rank,RANK);TT_FUNCTION(find,FIND);TT_FUNCTION(annotate,ANNOTATE);
    TT_FUNCTION(recognize,RECOGNIZE);TT_FUNCTION(relate,RELATE);
#undef TT_FUNCTION
    return 0;
}
/* Answer meanings and descriptions go through the shared question contract. A
 * usage mistake in the SQL argument itself sets *usage to the SQLite result. */
static int describe(Cursor *cursor, thinkthen_question **question, thinkthen_error *error, int *usage) {
    thinkthen_buffer built = {0};
    int code = thinkthen_question_to_json(*question, &built, error);
    if (code != THINKTHEN_OK) return code;
    json_t *definition = json_loadb((const char *)built.data, built.len, 0, NULL);
    thinkthen_buffer_free(&built);
    if (!json_is_object(definition)) { json_decref(definition); return thinkthen_question_conversion_error(error); }
    if (cursor->arguments[ARG_YES_MEANS])
        json_object_set_new(definition, "true", json_stringn(text(cursor, ARG_YES_MEANS), length(cursor, ARG_YES_MEANS)));
    if (cursor->arguments[ARG_NO_MEANS])
        json_object_set_new(definition, "false", json_stringn(text(cursor, ARG_NO_MEANS), length(cursor, ARG_NO_MEANS)));
    if (cursor->arguments[ARG_DESCRIPTIONS]) {
        static const char *const domains[] = {"choices", "labels", "levels"};
        const char *domain = NULL;
        json_t *names = NULL;
        for (size_t n = 0; n < 3 && !names; n++)
            if ((names = json_object_get(definition, domains[n]))) domain = domains[n];
        json_t *descriptions = json_loadb(text(cursor, ARG_DESCRIPTIONS), length(cursor, ARG_DESCRIPTIONS), 0, NULL);
        const char *key; json_t *value;
        if (!json_is_object(descriptions) || !json_is_array(names))
            *usage = usage_error(cursor, "descriptions must be a JSON object that maps each %s to its description",
                                 domain ? domain : "answer");
        else json_object_foreach(descriptions, key, value) {
            if (!json_is_string(value)) {
                *usage = usage_error(cursor, "descriptions gives '%s' a value that is not text", key);
                break;
            }
            size_t index; json_t *item; int found = 0;
            json_array_foreach(names, index, item) {
                json_t *name = json_is_object(item) ? json_object_get(item, "name") : item;
                if (json_is_string(name) && !strcmp(json_string_value(name), key)) {
                    json_array_set_new(names, index, json_pack("{s:s,s:O}", "name", key, "description", value));
                    found = 1;
                    break;
                }
            }
            if (!found) {
                *usage = usage_error(cursor, "descriptions names '%s', which is not one of the %s", key, domain);
                break;
            }
        }
        json_decref(descriptions);
        if (*usage) { json_decref(definition); return THINKTHEN_OK; }
    }
    char *encoded = json_dumps(definition, JSON_COMPACT);
    json_decref(definition);
    if (encoded == NULL) return thinkthen_question_conversion_error(error);
    thinkthen_question_free(*question);
    *question = NULL;
    code = thinkthen_sql_question_from_json(encoded, strlen(encoded), question, error);
    free(encoded);
    return code;
}

int question_open(Cursor *cursor) {
    Table *table=(Table*)cursor->base.pVtab;
    thinkthen_error error={.struct_size=sizeof(error)};
    thinkthen_question *question=NULL;
    thinkthen_function function=function_id(table->registration->function);
    int code=thinkthen_question_new(function,(const char*)text(cursor,ARG_QUESTION),length(cursor,ARG_QUESTION),&question,&error);
    for(int argument=ARG_CHOICES;code==THINKTHEN_OK && argument<=ARG_KINDS;argument++) {
        if(!cursor->arguments[argument])continue;
        json_t *list=json_loadb(text(cursor,argument),length(cursor,argument),JSON_DECODE_ANY|JSON_ALLOW_NUL,NULL);
        int strings=json_is_array(list);
        for(size_t n=0;strings && n<json_array_size(list);n++)strings=json_is_string(json_array_get(list,n));
        if(!strings) {
            static const char *const names[]={"choices","labels","levels","kinds"};
            json_decref(list);thinkthen_question_free(question);
            return usage_error(cursor,"%s must be a JSON array of text, such as '[\"first\",\"second\"]'",names[argument-ARG_CHOICES]);
        }
        for(size_t n=0;code==THINKTHEN_OK && n<json_array_size(list);n++) {
            json_t *item=json_array_get(list,n);
            const char *data=json_string_value(item);size_t len=json_string_length(item);
            if(argument==ARG_CHOICES)code=thinkthen_question_add_choice(question,data,len,&error);
            else if(argument==ARG_LABELS)code=thinkthen_question_add_label(question,data,len,&error);
            else if(argument==ARG_LEVELS)code=thinkthen_question_add_level(question,data,len,&error);
            else code=thinkthen_question_add_kind(question,data,len,&error);
        }
        json_decref(list);
    }
    double cut=cursor->arguments[ARG_THRESHOLD]?sqlite3_value_double(cursor->arguments[ARG_THRESHOLD]):0;
    double low=cursor->arguments[ARG_LOW]?sqlite3_value_double(cursor->arguments[ARG_LOW]):0;
    double high=cursor->arguments[ARG_HIGH]?sqlite3_value_double(cursor->arguments[ARG_HIGH]):0;
    if(code==THINKTHEN_OK)code=thinkthen_sql_question_set_threshold(question,
        cursor->arguments[ARG_THRESHOLD]?&cut:NULL,cursor->arguments[ARG_LOW]?&low:NULL,cursor->arguments[ARG_HIGH]?&high:NULL,&error);
    if(code==THINKTHEN_OK && cursor->arguments[ARG_FIELD])
        code=thinkthen_question_set_field(question,(const char*)text(cursor,ARG_FIELD),length(cursor,ARG_FIELD),&error);
    if(code==THINKTHEN_OK && cursor->arguments[ARG_TOP]) {
        sqlite3_int64 top=sqlite3_value_int64(cursor->arguments[ARG_TOP]);
        if(top<0)code=thinkthen_question_conversion_error(&error);
        else code=thinkthen_question_set_top(question,(size_t)top,&error);
    }
    if(code==THINKTHEN_OK && (cursor->arguments[ARG_YES_MEANS] || cursor->arguments[ARG_NO_MEANS] || cursor->arguments[ARG_DESCRIPTIONS])) {
        int usage=0;
        code=describe(cursor,&question,&error,&usage);
        if(usage) {thinkthen_question_free(question);return usage;}
    }
    /* Structured lists use the shared admitted contract, including unknown-field checks. */
    if(code==THINKTHEN_OK && (function==THINKTHEN_FUNCTION_ANNOTATE || function==THINKTHEN_FUNCTION_RELATE)) {
        int argument=function==THINKTHEN_FUNCTION_ANNOTATE?ARG_TOOLBOX:ARG_RULES;
        json_t *q=json_object();json_object_set_new(q,"function",json_string(table->registration->function));
        if(cursor->arguments[ARG_FIELD])json_object_set_new(q,"field",json_stringn(text(cursor,ARG_FIELD),length(cursor,ARG_FIELD)));
        if(cursor->arguments[ARG_THRESHOLD])json_object_set_new(q,"threshold",json_real(sqlite3_value_double(cursor->arguments[ARG_THRESHOLD])));
        char *prefix=json_dumps(q,JSON_COMPACT);json_decref(q);
        sqlite3_str *builder=sqlite3_str_new(table->db);
        if(prefix) {
            sqlite3_str_append(builder,prefix,(int)strlen(prefix)-1);
            sqlite3_str_appendf(builder,",\"%s\":",function==THINKTHEN_FUNCTION_ANNOTATE?"questions":"rules");
            sqlite3_str_append(builder,text(cursor,argument),(int)length(cursor,argument));
            sqlite3_str_appendchar(builder,1,'}');
        }
        free(prefix);
        int encoded_length=sqlite3_str_length(builder);
        char *encoded=sqlite3_str_finish(builder);
        thinkthen_question_free(question);question=NULL;
        if(!encoded)code=thinkthen_question_conversion_error(&error);
        else {code=thinkthen_sql_question_from_json((const char*)encoded,(size_t)encoded_length,&question,&error);sqlite3_free(encoded);}
    }
    if(code==THINKTHEN_OK) {
        thinkthen_text name={(const char*)text(cursor,ARG_NAME),length(cursor,ARG_NAME)};
        code=thinkthen_sql_call_open_question(table->registration->connection->client,question,cursor->arguments[ARG_NAME]?&name:NULL,&cursor->call,&error);
    }
    thinkthen_question_free(question);
    if(code==THINKTHEN_OK)return SQLITE_OK;
    char *message=NULL;int result=engine_error(&error,&message);
    result=cursor_error(cursor,result,message?message:"out of memory");sqlite3_free(message);return result;
}
