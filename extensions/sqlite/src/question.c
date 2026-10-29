#include "internal.h"
#include <string.h>
#include <stdlib.h>

static const char *text(Cursor *cursor,int argument) {
    return cursor->arguments[argument] ? (const char*)sqlite3_value_text(cursor->arguments[argument]) : "";
}
static size_t length(Cursor *cursor,int argument) {
    return cursor->arguments[argument] ? (size_t)sqlite3_value_bytes(cursor->arguments[argument]) : 0;
}
static uint32_t function_id(const char *name) {
#define TT_FUNCTION(name, upper) if(!strcmp(name_text,#name))return THINKTHEN_FUNCTION_##upper
    const char *name_text=name;
    TT_FUNCTION(decide,DECIDE);TT_FUNCTION(choose,CHOOSE);TT_FUNCTION(tag,TAG);TT_FUNCTION(score,SCORE);
    TT_FUNCTION(filter,FILTER);TT_FUNCTION(rank,RANK);TT_FUNCTION(find,FIND);TT_FUNCTION(annotate,ANNOTATE);
    TT_FUNCTION(recognize,RECOGNIZE);TT_FUNCTION(relate,RELATE);
#undef TT_FUNCTION
    return 0;
}
int question_open(Cursor *cursor) {
    Table *table=(Table*)cursor->base.pVtab;
    thinkthen_error error={.struct_size=sizeof(error)};
    thinkthen_question *question=NULL;
    uint32_t function=function_id(table->registration->function);
    int code=thinkthen_question_new(function,(const uint8_t*)text(cursor,ARG_QUESTION),length(cursor,ARG_QUESTION),&question,&error);
    for(int argument=ARG_CHOICES;code==THINKTHEN_OK && argument<=ARG_KINDS;argument++) {
        if(!cursor->arguments[argument])continue;
        json_t *list=json_loadb(text(cursor,argument),length(cursor,argument),JSON_DECODE_ANY|JSON_ALLOW_NUL,NULL);
        if(!json_is_array(list))code=thinkthen_question_conversion_error(&error);
        for(size_t n=0;code==THINKTHEN_OK && n<json_array_size(list);n++) {
            json_t *item=json_array_get(list,n);
            if(!json_is_string(item)){code=thinkthen_question_conversion_error(&error);break;}
            const uint8_t *data=(const uint8_t*)json_string_value(item);size_t len=json_string_length(item);
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
    if(code==THINKTHEN_OK && cursor->arguments[ARG_EVIDENCE])
        code=thinkthen_question_set_evidence(question,(const uint8_t*)text(cursor,ARG_EVIDENCE),length(cursor,ARG_EVIDENCE),&error);
    if(code==THINKTHEN_OK && cursor->arguments[ARG_TOP]) {
        sqlite3_int64 top=sqlite3_value_int64(cursor->arguments[ARG_TOP]);
        if(top<0)code=thinkthen_question_conversion_error(&error);
        else code=thinkthen_question_set_top(question,(size_t)top,&error);
    }
    /* Structured lists use the shared admitted contract, including unknown-field checks. */
    if(code==THINKTHEN_OK && (function==THINKTHEN_FUNCTION_ANNOTATE || function==THINKTHEN_FUNCTION_RELATE)) {
        int argument=function==THINKTHEN_FUNCTION_ANNOTATE?ARG_TOOLBOX:ARG_RULES;
        json_t *q=json_object();json_object_set_new(q,"function",json_string(table->registration->function));
        if(cursor->arguments[ARG_EVIDENCE])json_object_set_new(q,"evidence",json_stringn(text(cursor,ARG_EVIDENCE),length(cursor,ARG_EVIDENCE)));
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
        else {code=thinkthen_sql_question_from_json((const uint8_t*)encoded,(size_t)encoded_length,&question,&error);sqlite3_free(encoded);}
    }
    if(code==THINKTHEN_OK) {
        thinkthen_text name={(const uint8_t*)text(cursor,ARG_NAME),length(cursor,ARG_NAME)};
        code=thinkthen_sql_call_open_question(table->registration->connection->client,question,cursor->arguments[ARG_NAME]?&name:NULL,&cursor->call,&error);
    }
    thinkthen_question_free(question);
    if(code==THINKTHEN_OK)return SQLITE_OK;
    char *message=NULL;int result=engine_error(&error,&message);
    result=cursor_error(cursor,result,message?message:"out of memory");sqlite3_free(message);return result;
}
