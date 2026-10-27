#include "internal.h"
#include <string.h>
#include <stdlib.h>
#include "rules.h"

SQLITE_EXTENSION_INIT1

static const char *const argument_names[] = {"source_sql","question","choices","labels","levels","kinds",
    "rules","toolbox","threshold","low","high","top","evidence","params","record_column","id_column","format","name"};
static const char *const column_names[] = {"position INTEGER","status TEXT","answer","probability REAL",
    "error_code TEXT","error_message TEXT","error_http_status INTEGER","detail TEXT","id","source TEXT",
    "kept INTEGER","rank INTEGER","found_text TEXT","found_start INTEGER","found_end INTEGER"};

static int is_function(Table *table, const char *name) {
    return strcmp(table->registration->function, name) == 0;
}
static int exposed(Table *table, int argument) {
    if (argument == ARG_SOURCE || argument >= ARG_PARAMS) return 1;
    return tt_argument(table->registration->function, argument_names[argument]);
}
static int connect_table(sqlite3 *db, void *aux, int argc, const char *const *argv,
                         sqlite3_vtab **output, char **error) {
    (void)argc; (void)argv; (void)error;
    int result = sqlite3_vtab_config(db, SQLITE_VTAB_DIRECTONLY);
    if (result != SQLITE_OK) return result;
    Table *table = sqlite3_malloc64(sizeof(*table));
    if (!table) return SQLITE_NOMEM;
    memset(table,0,sizeof(*table)); table->db=db; table->registration=aux;
    sqlite3_str *schema=sqlite3_str_new(db);
    sqlite3_str_appendall(schema,"CREATE TABLE x(");
    for (int n=0;n<15;n++) {
        if ((n==COL_ANSWER && (is_function(table,"rank") || is_function(table,"find"))) ||
            (n==COL_KEPT && !is_function(table,"filter")) || (n==COL_RANK && !is_function(table,"rank")) ||
            (n>=COL_TEXT && !is_function(table,"find"))) continue;
        if(table->column_count) sqlite3_str_appendall(schema,",");
        sqlite3_str_appendall(schema,column_names[n]); table->columns[table->column_count++]=n;
    }
    for(int n=0;n<ARG_COUNT;n++) if(exposed(table,n)) {
        sqlite3_str_appendf(schema,",%s HIDDEN",argument_names[n]); table->arguments[table->argument_count++]=n;
    }
    sqlite3_str_appendall(schema,")");
    char *declaration=sqlite3_str_finish(schema);
    result=declaration ? sqlite3_declare_vtab(db,declaration) : SQLITE_NOMEM;
    sqlite3_free(declaration);
    if(result!=SQLITE_OK) {sqlite3_free(table);return result;}
    *output=&table->base; return SQLITE_OK;
}
static int best_index(sqlite3_vtab *base, sqlite3_index_info *index) {
    Table *table=(Table*)base;
    int argument=1,mask=0;
    for(int n=0;n<table->argument_count;n++) {
        int found=-1, key=table->arguments[n];
        for(int i=0;i<index->nConstraint;i++) {
            if(index->aConstraint[i].iColumn!=table->column_count+n) continue;
            if(!index->aConstraint[i].usable) return SQLITE_CONSTRAINT;
            if(index->aConstraint[i].op!=SQLITE_INDEX_CONSTRAINT_EQ || found!=-1) {
                base->zErrMsg=sqlite3_mprintf("argument %s requires one equality",argument_names[key]);return SQLITE_ERROR;
            }
            found=i;
        }
        if(found<0) {
            if(key<=ARG_TOOLBOX && key!=ARG_THRESHOLD) {
                base->zErrMsg=sqlite3_mprintf("%s is required",argument_names[key]);return SQLITE_ERROR;
            }
            continue;
        }
        index->aConstraintUsage[found].argvIndex=argument++;index->aConstraintUsage[found].omit=1;mask|=1<<key;
    }
    index->idxNum=mask;index->estimatedCost=1000000;index->estimatedRows=1000;return SQLITE_OK;
}
static int disconnect_table(sqlite3_vtab *table) {sqlite3_free(table);return SQLITE_OK;}
static int open_cursor(sqlite3_vtab *table, sqlite3_vtab_cursor **output) {
    Cursor *cursor=sqlite3_malloc64(sizeof(*cursor));if(!cursor)return SQLITE_NOMEM;
    memset(cursor,0,sizeof(*cursor));cursor->base.pVtab=table;cursor->eof=1;
    cursor->row.struct_size=sizeof(cursor->row);*output=&cursor->base;return SQLITE_OK;
}
static int close_cursor(sqlite3_vtab_cursor *base) {cursor_clear((Cursor*)base);sqlite3_free(base);return SQLITE_OK;}

static int filter_cursor(sqlite3_vtab_cursor *base, int mask, const char *plan, int argc, sqlite3_value **argv) {
    (void)plan;(void)argc;
    Cursor *cursor=(Cursor*)base;Table *table=(Table*)base->pVtab;
    if(replay_same(cursor,mask,argv)) {
        while(!cursor->complete) {
            int result=cursor_advance(cursor);if(result!=SQLITE_OK)return result;
        }
    }
    if(replay_same(cursor,mask,argv)) {
        if(!cursor->replaying)thinkthen_row_free(&cursor->row);
        json_decref(cursor->answer);cursor->answer=NULL;
        json_decref(cursor->error);cursor->error=NULL;
        cursor->replaying=1;cursor->rowid=0;
        if(!cursor->read && sqlite3_prepare_v2(cursor->replay,"SELECT * FROM rows ORDER BY rowid",-1,&cursor->read,NULL)!=SQLITE_OK)
            return cursor_error(cursor,SQLITE_ERROR,"cannot rewind result spool");
        sqlite3_reset(cursor->read);
        return cursor_advance(cursor);
    }
    cursor_clear(cursor);cursor->rowid=0;
    cursor->mask=mask;cursor->changes=sqlite3_total_changes64(table->db);
    cursor->generation=table->registration->connection->generation;
    int opened=replay_open(cursor);if(opened!=SQLITE_OK)return opened;
    int index=0;
    for(int n=0;n<ARG_COUNT;n++) if(mask&(1<<n)) {
        int type=sqlite3_value_type(argv[index]);
        int numeric=n==ARG_THRESHOLD || n==ARG_LOW || n==ARG_HIGH;
        if((numeric && type!=SQLITE_FLOAT && type!=SQLITE_INTEGER) ||
           (n==ARG_TOP && type!=SQLITE_INTEGER) || (!numeric && n!=ARG_TOP && type!=SQLITE_TEXT))
            return cursor_error(cursor,SQLITE_MISMATCH,"argument has an incompatible SQL type");
        cursor->arguments[n]=sqlite3_value_dup(argv[index++]);
        if(!cursor->arguments[n])return cursor_error(cursor,SQLITE_NOMEM,"out of memory");
    }
    int result=source_open(cursor);if(result!=SQLITE_OK)return result;
    Connection *connection=table->registration->connection;char *message=NULL;
    if(!connection->client) {
        result=connection_client(connection,"{}",2,&message);
        if(result!=SQLITE_OK) {result=cursor_error(cursor,result,message?message:"out of memory");sqlite3_free(message);return result;}
    }
    cursor->generation=connection->generation;
    result=question_open(cursor);
    return result==SQLITE_OK ? cursor_advance(cursor) : result;
}
static int next_cursor(sqlite3_vtab_cursor *base) {return cursor_advance((Cursor*)base);}
static int eof_cursor(sqlite3_vtab_cursor *base) {return ((Cursor*)base)->eof;}
static int rowid_cursor(sqlite3_vtab_cursor *base,sqlite3_int64 *id) {*id=((Cursor*)base)->rowid;return SQLITE_OK;}
static const sqlite3_module module={.iVersion=1,.xConnect=connect_table,.xBestIndex=best_index,.xDisconnect=disconnect_table,
    .xOpen=open_cursor,.xClose=close_cursor,.xFilter=filter_cursor,.xNext=next_cursor,.xEof=eof_cursor,.xColumn=cursor_column,.xRowid=rowid_cursor};
static void registration_free(void *pointer) {Registration *registration=pointer;connection_release(registration->connection);sqlite3_free(registration);}
static void set_setting(sqlite3_context *context,int argc,sqlite3_value **argv) {
    (void)argc;
    Connection *connection=sqlite3_user_data(context);
    const char *key=(const char*)sqlite3_value_text(argv[0]);
    int type=sqlite3_value_type(argv[1]);
    int setting=key ? tt_setting(key) : 0;
    int text=setting==1;
    int count=setting==2 || setting==4;
    if(sqlite3_value_type(argv[0])!=SQLITE_TEXT || (key && (size_t)sqlite3_value_bytes(argv[0])!=strlen(key)) || (!text && !count) || (text && type!=SQLITE_TEXT) || (count && type!=SQLITE_INTEGER)) {
        thinkthen_error error={.struct_size=sizeof(error)};char *message=NULL;
        thinkthen_settings_conversion_error(&error);engine_error(&error,&message);
        sqlite3_result_error(context,message?message:"out of memory",-1);sqlite3_free(message);return;
    }
    json_t *settings=connection->settings ? json_loadb(connection->settings,connection->settings_len,0,NULL):json_object();
    json_t *target=settings;
    if(setting==4) {
        target=json_object_get(settings,"retry");
        if(!target) {target=json_object();json_object_set_new(settings,"retry",target);}
    }
    json_t *value=text ? json_stringn((const char*)sqlite3_value_text(argv[1]),(size_t)sqlite3_value_bytes(argv[1])) : json_integer(sqlite3_value_int64(argv[1]));
    json_object_set_new(target,key,value);char *encoded=json_dumps(settings,JSON_COMPACT);json_decref(settings);
    if(!encoded) {sqlite3_result_error_nomem(context);return;}
    char *message=NULL;int result=connection_client(connection,encoded,strlen(encoded),&message);free(encoded);
    if(result==SQLITE_OK)sqlite3_result_int(context,1);
    else if(result==SQLITE_NOMEM)sqlite3_result_error_nomem(context);
    else sqlite3_result_error(context,message,-1);
    sqlite3_free(message);
}
#ifdef _WIN32
__declspec(dllexport)
#else
__attribute__((visibility("default")))
#endif
int sqlite3_thinkthensqlite_init(sqlite3 *db,char **error,const sqlite3_api_routines *api) {
    SQLITE_EXTENSION_INIT2(api);
    if(!thinkthen_abi_compatible(THINKTHEN_ABI_VERSION,thinkthen_abi_version())) {
        *error=sqlite3_mprintf("%s: %s",THINKTHEN_ABI_ERROR_CODE,THINKTHEN_ABI_ERROR_MESSAGE);return *error?SQLITE_ERROR:SQLITE_NOMEM;
    }
    if(sqlite3_libversion_number()<3041000) {*error=sqlite3_mprintf("ThinkThen requires SQLite 3.41.0 or newer");return *error?SQLITE_ERROR:SQLITE_NOMEM;}
    static const char *const modules[]={"thinkthen_decide","thinkthen_choose","thinkthen_tag","thinkthen_score","thinkthen_filter","thinkthen_rank","thinkthen_find","thinkthen_annotate","thinkthen_recognize","thinkthen_relate"};
    Connection *connection=sqlite3_malloc64(sizeof(*connection));if(!connection)return SQLITE_NOMEM;
    memset(connection,0,sizeof(*connection));connection->references=1;
    int result=SQLITE_OK;
    size_t registered=0;
    for(size_t n=0;n<sizeof(modules)/sizeof(modules[0]);n++) {
        Registration *registration=sqlite3_malloc64(sizeof(*registration));if(!registration){result=SQLITE_NOMEM;break;}
        registration->connection=connection;registration->function=modules[n]+sizeof("thinkthen_")-1;++connection->references;
        result=sqlite3_create_module_v2(db,modules[n],&module,registration,registration_free);if(result!=SQLITE_OK)break;
        ++registered;
    }
    if(result==SQLITE_OK) {++connection->references;result=sqlite3_create_function_v2(db,"thinkthen_set",2,SQLITE_UTF8|SQLITE_DIRECTONLY,connection,set_setting,NULL,NULL,connection_release);}
    /* SQLite unloads a failed extension without removing its registrations. */
    if(result!=SQLITE_OK) {
        while(registered) sqlite3_create_module_v2(db,modules[--registered],NULL,NULL,NULL);
    }
    connection_release(connection);return result==SQLITE_OK ? SQLITE_OK_LOAD_PERMANENTLY : result;
}
